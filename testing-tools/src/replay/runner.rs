use std::{
    cmp::max,
    collections::{HashMap, HashSet, VecDeque},
    pin::Pin,
    sync::Arc,
    time::Duration
};

use alloy::{
    network::{Ethereum, EthereumWallet},
    node_bindings::{Anvil, AnvilInstance},
    primitives::Address,
    providers::{
        Identity, Provider, ProviderBuilder, RootProvider,
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller}
    },
    signers::local::PrivateKeySigner
};
use alloy_primitives::aliases::I24;
use alloy_rpc_types::{BlockId, BlockNumberOrTag, Filter, TransactionRequest};
use alloy_sol_types::SolEvent;
use angstrom::components::{StromHandles, initialize_strom_handles};
use angstrom_amm_quoter::{QuoterHandle, QuoterManager};
use angstrom_eth::{
    handle::Eth,
    manager::{EthDataCleanser, EthEvent}
};
use angstrom_network::{PoolManagerBuilder, StromNetworkHandle, pool_manager::PoolHandle};
use angstrom_rpc::{
    ConsensusApi, OrderApi,
    api::{ConsensusApiServer, OrderApiServer}
};
use angstrom_types::{
    block_sync::{BlockSyncProducer, GlobalBlockSync},
    consensus::ConsensusRoundName,
    contract_bindings::{
        angstrom::Angstrom::PoolKey,
        controller_v_1::ControllerV1::{self, PoolConfigured, PoolRemoved}
    },
    contract_payloads::angstrom::{AngstromPoolConfigStore, UniswapAngstromRegistry},
    pair_with_price::PairsWithPrice,
    primitive::{
        AngstromSigner, PoolId, UniswapPoolRegistry, init_with_chain_id, try_init_with_chain_id, *
    },
    sol_bindings::testnet::TestnetHub,
    submission::{ChainSubmitterHolder, SubmissionHandler},
    testnet::InitialTestnetState
};
use consensus::{AngstromValidator, ConsensusHandler, ConsensusManager, ManagerNetworkDeps};
use futures::{Future, Stream, StreamExt};
use jsonrpsee::server::ServerBuilder;
use matching_engine::{MatchingManager, manager::MatcherHandle};
use order_pool::{PoolConfig, order_storage::OrderStorage};
use reth_chainspec::Hardforks;
use reth_network::NetworkHandle;
use reth_provider::{
    BlockNumReader, BlockReader, CanonStateSubscriptions, ChainSpecProvider, HeaderProvider,
    ReceiptProvider, noop::NoopProvider
};
use reth_tasks::TaskExecutor;
use telemetry::{blocklog::BlockLog, init_telemetry};
use tokio::sync::mpsc::UnboundedSender;
use tracing::{Instrument, span};
use uniswap_v4::configure_uniswap_manager;
use validation::{
    common::{TokenPriceGenerator, WETH_ADDRESS},
    init_validation, init_validation_replay,
    order::state::pools::AngstromPoolsTracker,
    validator::ValidationClient
};

use super::fake_network::FakeNetwork;
use crate::{
    agents::AgentConfig,
    contracts::anvil::WalletProviderRpc,
    controllers::strom::initialize_strom_components_at_block,
    providers::{
        AnvilProvider, AnvilStateProvider, AnvilSubmissionProvider, TestnetBlockProvider,
        WalletProvider,
        utils::{StromContractInstance, async_to_sync}
    },
    types::{
        GlobalTestingConfig, SendingStromHandles, WithWalletProvider, config::TestingNodeConfig
    },
    validation::TestOrderValidator
};

/// Replay Runner
///
/// The Replay Runner allows us to take any snapshot of a block and replay it
/// in order to allow us to debug quickly.
pub struct ReplayRunner {
    anvil:          AnvilInstance,
    anvil_provider: AnvilProvider<WalletProvider>,
    fake_network:   FakeNetwork
}

impl ReplayRunner {
    pub async fn new(
        id: String,
        block_log: BlockLog,
        fork_url: String,
        rpc_port: u16,
        executor: TaskExecutor
    ) -> eyre::Result<Self> {
        let chain_id = block_log.constants.as_ref().unwrap().chain_id;
        try_init_with_chain_id(chain_id).unwrap();

        let endpoint = format!("/tmp/anvil-replay-{id}.ipc");

        let anvil = Anvil::new()
            .fork_block_number(block_log.blocknum())
            .chain_id(chain_id)
            .fork(fork_url)
            .arg("--ipc")
            .arg(endpoint.clone())
            // We not needed for the given block.
            .arg("--no-mining")
            .spawn();

        let node_addr = block_log.constants.as_ref().unwrap().node_address();
        let angstrom_signer = AngstromSigner::for_address(node_addr);
        let sk = angstrom_signer.clone().into_signer();

        let wallet = EthereumWallet::new(sk.clone());

        let rpc = alloy::providers::builder::<Ethereum>()
            .with_recommended_fillers()
            .wallet(wallet)
            .connect(&endpoint)
            .await
            .unwrap();

        tracing::info!("connected to anvil");

        let wallet_provider = WalletProvider::new_with_provider(rpc.clone(), sk);
        let state_provider = AnvilStateProvider::new(wallet_provider);
        let anvil_provider = AnvilProvider::new(state_provider, None, None);
        let reth_provider = NoopProvider::mainnet();

        let angstrom_address = *ANGSTROM_ADDRESS.get().unwrap();
        let controller = *CONTROLLER_V1_ADDRESS.get().unwrap();
        let deploy_block = *ANGSTROM_DEPLOYED_BLOCK.get().unwrap();
        let gas_token = *GAS_TOKEN_ADDRESS.get().unwrap();
        let pool_manager = *POOL_MANAGER_ADDRESS.get().unwrap();

        let mut strom_handles = initialize_strom_handles();
        let quoter_handle = QuoterHandle(strom_handles.quoter_tx.clone());

        // for rpc
        let pool = strom_handles.get_pool_handle();
        let executor_clone = executor.clone();
        let validation_client = ValidationClient(strom_handles.validator_tx.clone());
        let consensus_client = ConsensusHandler(strom_handles.consensus_tx_rpc.clone());

        let query_provider = Arc::new(anvil_provider.state_provider());

        let periphery_c = ControllerV1::new(*CONTROLLER_V1_ADDRESS.get().unwrap(), rpc.clone());
        let node_set = periphery_c
            .nodes()
            .call()
            .await
            .unwrap()
            .into_iter()
            .collect::<HashSet<_>>();

        let validation_client = ValidationClient(strom_handles.validator_tx);
        let matching_handle = MatchingManager::spawn(executor.clone(), validation_client.clone());
        let consensus_client = ConsensusHandler(strom_handles.consensus_tx_rpc.clone());

        let consensus_api = ConsensusApi::new(consensus_client.clone(), executor.clone());

        let amm_quoter = QuoterHandle(strom_handles.quoter_tx.clone());
        let order_api =
            OrderApi::new(pool.clone(), executor.clone(), validation_client.clone(), amm_quoter);

        // We set -1 as the start of the replay will be triggering new block transition.
        let block_number = BlockNumReader::best_block_number(&query_provider)? - 1;

        let global_block_sync = GlobalBlockSync::new(block_number);

        let pool_config_store = Arc::new(
            AngstromPoolConfigStore::load_from_chain(
                angstrom_address,
                BlockId::Number(BlockNumberOrTag::Latest),
                &rpc
            )
            .await
            .unwrap()
        );
        let pools = fetch_angstrom_pools(
            deploy_block as usize,
            block_number as usize,
            angstrom_address,
            controller,
            &rpc
        )
        .await;

        let uniswap_registry: UniswapPoolRegistry = pools.into();

        let sub = anvil_provider
            .state_provider()
            .subscribe_to_canonical_state();

        let eth_snap = block_log.eth_snapshot.as_ref().unwrap();

        let eth_handle = EthDataCleanser::spawn(
            angstrom_address,
            controller,
            sub,
            executor.clone(),
            strom_handles.eth_tx,
            strom_handles.eth_rx,
            eth_snap.angstrom_tokens.clone(),
            eth_snap.pool_store.clone(),
            global_block_sync.clone(),
            eth_snap.node_set.clone(),
            vec![]
        )
        .unwrap();

        tracing::debug!("spawned data cleaner");

        let network_stream = Box::pin(eth_handle.subscribe_network())
            as Pin<Box<dyn Stream<Item = EthEvent> + Send + Sync>>;

        let uniswap_pool_manager = configure_uniswap_manager(
            rpc.clone().into(),
            eth_handle.subscribe_cannon_state_notifications().await,
            uniswap_registry.clone(),
            block_number,
            global_block_sync.clone(),
            pool_manager,
            network_stream
        )
        .await;
        tracing::debug!("uniswap configured");

        let uniswap_pools = uniswap_pool_manager.pools();
        executor.spawn_critical(
            "uniswap",
            Box::pin(
                uniswap_pool_manager.instrument(span!(tracing::Level::ERROR, "pool manager",))
            )
        );

        let token_conversion =
            if let Some((prev_prices, base_wei)) = block_log.gas_price_snapshot.as_ref() {
                println!("Using snapshot");
                TokenPriceGenerator::from_snapshot(
                    uniswap_pools.clone(),
                    prev_prices.clone(),
                    gas_token,
                    *base_wei
                )
            } else {
                TokenPriceGenerator::new(
                    Arc::new(anvil_provider.rpc_provider()),
                    block_number,
                    uniswap_pools.clone(),
                    gas_token,
                    Some(1)
                )
                .await
                .expect("failed to start price generator")
            };

        let token_price_update_stream = anvil_provider.state_provider().canonical_state_stream();
        let token_price_update_stream = Box::pin(PairsWithPrice::into_price_update_stream(
            angstrom_address,
            token_price_update_stream,
            Arc::new(anvil_provider.rpc_provider())
        ));

        let user_account = block_log
            .validation_snapshot
            .as_ref()
            .unwrap()
            .state
            .clone();

        init_validation_replay(
            anvil_provider.state_provider(),
            block_number,
            angstrom_address,
            node_addr,
            token_price_update_stream,
            uniswap_pools.clone(),
            token_conversion,
            pool_config_store.clone(),
            strom_handles.validator_rx,
            |validator| validator.set_user_account(user_account)
        );

        let pool_config = PoolConfig {
            ids: uniswap_registry.pools().keys().cloned().collect::<Vec<_>>(),
            ..Default::default()
        };

        let pool_snapshot = block_log.order_pool_snapshot.as_ref().unwrap().clone();
        let order_storage = Arc::new(pool_snapshot.order_storage.clone());

        let fake_network = FakeNetwork::new(
            Some(strom_handles.pool_tx),
            Some(strom_handles.consensus_tx_op),
            strom_handles.eth_handle_rx.take().unwrap()
        );

        let network_handle = fake_network.handle.clone();

        let _pool_handle = PoolManagerBuilder::new(
            validation_client.clone(),
            Some(order_storage.clone()),
            network_handle.clone(),
            eth_handle.subscribe_network(),
            strom_handles.pool_rx,
            global_block_sync.clone()
        )
        .with_config(pool_config)
        .build_with_channels(
            executor.clone(),
            strom_handles.orderpool_tx,
            strom_handles.orderpool_rx,
            strom_handles.pool_manager_tx,
            block_number,
            |order_indexer| {
                // Order storage is set above.
                order_indexer.set_tracker(pool_snapshot.order_tracker);
            }
        );

        let server = ServerBuilder::default()
            .build(format!("0.0.0.0:{}", rpc_port))
            .await?;

        let addr = server.local_addr()?;

        executor.spawn_critical(
            "rpc",
            Box::pin(async move {
                let mut rpcs = order_api.into_rpc();
                rpcs.merge(consensus_api.into_rpc()).unwrap();
                let server_handle = server.start(rpcs);
                tracing::info!("rpc server started on: {}", addr);
                let _ = server_handle.stopped().await;
            })
        );

        let pool_registry =
            UniswapAngstromRegistry::new(uniswap_registry.clone(), pool_config_store.clone());

        let anvil_sub =
            AnvilSubmissionProvider { provider: anvil_provider.rpc_provider(), angstrom_address };

        let mev_boost_provider = SubmissionHandler {
            node_provider: Arc::new(anvil_provider.rpc_provider()),
            submitters:    vec![Box::new(ChainSubmitterHolder::new(
                anvil_sub,
                angstrom_signer.clone()
            ))]
        };

        tracing::debug!("created mev boost provider");
        let validators = node_set
            .into_iter()
            // use same weight for all validators
            .map(|addr| AngstromValidator::new(addr, 100))
            .collect::<Vec<_>>();

        let consensus = ConsensusManager::new(
            ManagerNetworkDeps::new(
                network_handle.clone(),
                eth_handle.subscribe_cannon_state_notifications().await,
                strom_handles.consensus_rx_op
            ),
            angstrom_signer,
            validators,
            order_storage.clone(),
            block_number,
            block_number,
            pool_registry,
            uniswap_pools.clone(),
            mev_boost_provider,
            matching_handle,
            global_block_sync.clone(),
            strom_handles.consensus_rx_rpc,
            None
        );
        executor.spawn_critical_with_graceful_shutdown_signal("consensus", move |grace| {
            consensus.run_till_shutdown(grace)
        });

        // spin up amm quoter
        let amm = QuoterManager::new(
            global_block_sync.clone(),
            order_storage.clone(),
            strom_handles.quoter_rx,
            uniswap_pools.clone(),
            rayon::ThreadPoolBuilder::default()
                .num_threads(2)
                .build()
                .expect("failed to build rayon thread pool"),
            Duration::from_millis(100)
        );

        executor.spawn_critical("amm quoting service", amm);

        tracing::info!("created consensus manager");
        global_block_sync.finalize_modules();

        Ok(Self { anvil, anvil_provider, fake_network })
    }
}

async fn fetch_angstrom_pools<P>(
    // the block angstrom was deployed at
    mut deploy_block: usize,
    end_block: usize,
    angstrom_address: Address,
    controller_address: Address,
    db: &P
) -> Vec<PoolKey>
where
    P: Provider
{
    let mut filters = vec![];

    loop {
        let this_end_block = std::cmp::min(deploy_block + 99_999, end_block);

        if this_end_block == deploy_block {
            break;
        }

        tracing::info!(?deploy_block, ?this_end_block);
        let filter = Filter::new()
            .from_block(deploy_block as u64)
            .to_block(this_end_block as u64)
            .address(controller_address);

        filters.push(filter);

        deploy_block = std::cmp::min(end_block, this_end_block);
    }

    let logs = futures::stream::iter(filters)
        .map(|filter| async move {
            db.get_logs(&filter)
                .await
                .unwrap()
                .into_iter()
                .collect::<Vec<_>>()
        })
        .buffered(10)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    tracing::info!(?logs);

    logs.into_iter()
        .fold(HashSet::new(), |mut set, log| {
            if let Ok(pool) = PoolConfigured::decode_log(&log.clone().into_inner()) {
                let pool_key = PoolKey {
                    currency0:   pool.asset0,
                    currency1:   pool.asset1,
                    fee:         pool.bundleFee,
                    tickSpacing: I24::try_from_be_slice(&{
                        let bytes = pool.tickSpacing.to_be_bytes();
                        let mut a = [0u8; 3];
                        a[1..3].copy_from_slice(&bytes);
                        a
                    })
                    .unwrap(),
                    hooks:       angstrom_address
                };

                set.insert(pool_key);
                return set;
            }

            if let Ok(pool) = PoolRemoved::decode_log(&log.clone().into_inner()) {
                let pool_key = PoolKey {
                    currency0:   pool.asset0,
                    currency1:   pool.asset1,
                    fee:         pool.feeInE6,
                    tickSpacing: pool.tickSpacing,
                    hooks:       angstrom_address
                };

                set.remove(&pool_key);
                return set;
            }
            set
        })
        .into_iter()
        .collect::<Vec<_>>()
}
