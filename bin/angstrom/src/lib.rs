//! Angstrom binary executable.
//!
//! ## Feature Flags

use std::{collections::HashSet, path::PathBuf, str::FromStr, sync::Arc};

use alloy::{
    providers::{ProviderBuilder, network::Ethereum},
    signers::local::PrivateKeySigner
};
use alloy_chains::NamedChain;
use angstrom_amm_quoter::QuoterHandle;
use angstrom_metrics::METRICS_ENABLED;
use angstrom_network::AngstromNetworkBuilder;
use angstrom_rpc::{
    ConsensusApi, OrderApi,
    api::{ConsensusApiServer, OrderApiServer}
};
use angstrom_types::{
    contract_bindings::controller_v_1::ControllerV1,
    primitive::{
        ANGSTROM_DOMAIN, AngstromAddressBuilder, AngstromSigner, ETH_ANGSTROM_RPC, ETH_DEFAULT_RPC,
        ETH_MEV_RPC, SEPOLIA_DEFAULT_RPC, SEPOLIA_MEV_RPC
    }
};
use clap::Parser;
use cli::AngstromConfig;
use consensus::ConsensusHandler;
use parking_lot::RwLock;
use reth::{
    chainspec::{EthChainSpec, EthereumChainSpecParser},
    cli::Cli
};
use reth_node_builder::{Node, NodeHandle};
use reth_node_ethereum::node::{EthereumAddOns, EthereumNode};
use url::Url;
use validation::validator::ValidationClient;

use crate::{
    cli::NodeConfig,
    components::{init_network_builder, initialize_strom_components, initialize_strom_handles}
};

pub mod cli;
pub mod components;

/// Convenience function for parsing CLI options, set up logging and run the
/// chosen command.
#[inline]
pub fn run() -> eyre::Result<()> {
    Cli::<EthereumChainSpecParser, AngstromConfig>::parse().run(|builder, mut args| async move {
        let executor = builder.task_executor().clone();
        let chain = builder.config().chain.chain().named().unwrap();

        let node_config = NodeConfig::load_from_config(Some(args.node_config.clone())).unwrap();

        let address_builder = AngstromAddressBuilder::default()
            .with_angstrom_address(node_config.angstrom_address)
            .with_controller(node_config.periphery_address)
            .with_pool_manager(node_config.pool_manager_address)
            .with_deploy_block(node_config.angstrom_deploy_block)
            .build();

        match chain {
            NamedChain::Sepolia => {
                if args.mev_boost_endpoints.is_empty() {
                    args.mev_boost_endpoints = SEPOLIA_MEV_RPC
                        .into_iter()
                        .map(|url| Url::from_str(url).unwrap())
                        .collect();
                }
                if args.normal_nodes.is_empty() {
                    args.normal_nodes = SEPOLIA_DEFAULT_RPC
                        .into_iter()
                        .map(|url| Url::from_str(url).unwrap())
                        .collect();
                }

                address_builder.init_with_chain_fallback(NamedChain::Sepolia as u64);
            }
            NamedChain::Mainnet => {
                if args.mev_boost_endpoints.is_empty() {
                    args.mev_boost_endpoints = ETH_MEV_RPC
                        .into_iter()
                        .map(|url| Url::from_str(url).unwrap())
                        .collect();
                }
                if args.normal_nodes.is_empty() {
                    args.normal_nodes = ETH_DEFAULT_RPC
                        .into_iter()
                        .map(|url| Url::from_str(url).unwrap())
                        .collect();
                }
                if args.angstrom_submission_nodes.is_empty() {
                    args.angstrom_submission_nodes = ETH_ANGSTROM_RPC
                        .into_iter()
                        .map(|url| Url::from_str(url).unwrap())
                        .collect();
                }

                address_builder.init_with_chain_fallback(NamedChain::Mainnet as u64);
            }
            chain => panic!("we do not support chain {chain}")
        }

        if args.metrics_enabled {
            executor.spawn_critical("metrics", crate::cli::init_metrics(args.metrics_port));
            METRICS_ENABLED.set(true).unwrap();
        } else {
            METRICS_ENABLED.set(false).unwrap();
        }

        tracing::info!(domain=?ANGSTROM_DOMAIN);

        let secret_key = get_secret_key(&args.secret_key_location)?;

        let mut channels = initialize_strom_handles();
        let quoter_handle = QuoterHandle(channels.quoter_tx.clone());

        // for rpc
        let pool = channels.get_pool_handle();
        let executor_clone = executor.clone();
        let validation_client = ValidationClient(channels.validator_tx.clone());
        let consensus_client = ConsensusHandler(channels.consensus_tx_rpc.clone());

        // get provider and node set for startup, we need this so when reth startup
        // happens, we directly can connect to the nodes.

        let startup_provider = ProviderBuilder::<_, _, Ethereum>::default()
            .with_recommended_fillers()
            .connect(&args.boot_node)
            .await
            .unwrap();

        let periphery_c = ControllerV1::new(node_config.periphery_address, startup_provider);
        let node_set = periphery_c
            .nodes()
            .call()
            .await
            .unwrap()
            .into_iter()
            .collect::<HashSet<_>>();

        let mut network = init_network_builder(
            secret_key.clone(),
            channels.eth_handle_rx.take().unwrap(),
            Arc::new(RwLock::new(node_set.clone()))
        )?;

        let protocol_handle = network.build_protocol_handler();

        let NodeHandle { node, node_exit_future } = builder
            .with_types::<EthereumNode>()
            .with_components(
                EthereumNode::default()
                    .components_builder()
                    .network(AngstromNetworkBuilder::new(protocol_handle))
            )
            .with_add_ons::<EthereumAddOns<_>>(Default::default())
            .extend_rpc_modules(move |rpc_context| {
                let order_api = OrderApi::new(
                    pool.clone(),
                    executor_clone.clone(),
                    validation_client,
                    quoter_handle
                );
                let consensus = ConsensusApi::new(consensus_client, executor_clone);
                rpc_context.modules.merge_configured(order_api.into_rpc())?;
                rpc_context.modules.merge_configured(consensus.into_rpc())?;

                Ok(())
            })
            .launch()
            .await?;
        network = network.with_reth(node.network.clone());

        initialize_strom_components(
            args,
            secret_key,
            channels,
            network,
            &node,
            executor,
            node_exit_future,
            node_set,
            node_config
        )
        .await
    })
}

fn get_secret_key(sk_path: &PathBuf) -> eyre::Result<AngstromSigner> {
    let exists = sk_path.try_exists();

    match exists {
        Ok(true) => {
            let contents = std::fs::read_to_string(sk_path)?;
            Ok(AngstromSigner::new(contents.trim().parse::<PrivateKeySigner>()?))
        }
        _ => Err(eyre::eyre!("no secret_key was found at {:?}", sk_path))
    }
}
