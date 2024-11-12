use alloy::{
    network::{Ethereum, EthereumWallet},
    node_bindings::{Anvil, AnvilInstance},
    providers::{
        builder,
        ext::AnvilApi,
        fillers::{ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller},
        Identity, IpcConnect, RootProvider
    },
    pubsub::PubSubFrontend,
    signers::local::PrivateKeySigner,
    transports::http::{Client, Http}
};
use alloy_primitives::{
    aliases::{I24, U24},
    Address, Bytes
};
use angstrom_types::contract_bindings::angstrom::Angstrom::{AngstromInstance, PoolKey};
use tokio::sync::broadcast;

use crate::{
    anvil_state_provider::AnvilStateProvider,
    contracts::{
        deploy::tokens::mint_token_pair,
        environment::{angstrom::AngstromEnv, uniswap::UniswapEnv, TestAnvilEnvironment},
        DebugTransaction
    },
    mocks::canon_state::AnvilConsensusCanonStateNotification,
    testnet_controllers::AngstromTestnetConfig,
    types::initial_state::InitialTestnetState
};

pub type AnvilWalletRpc = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<
                GasFiller,
                JoinFill<
                    alloy::providers::fillers::BlobGasFiller,
                    JoinFill<NonceFiller, ChainIdFiller>
                >
            >
        >,
        WalletFiller<EthereumWallet>
    >,
    RootProvider<PubSubFrontend>,
    PubSubFrontend,
    Ethereum
>;

pub type LocalAnvilRpc = alloy::providers::fillers::FillProvider<
    alloy::providers::fillers::JoinFill<
        alloy::providers::fillers::JoinFill<
            alloy::providers::Identity,
            alloy::providers::fillers::JoinFill<
                alloy::providers::fillers::GasFiller,
                alloy::providers::fillers::JoinFill<
                    alloy::providers::fillers::BlobGasFiller,
                    alloy::providers::fillers::JoinFill<
                        alloy::providers::fillers::NonceFiller,
                        alloy::providers::fillers::ChainIdFiller
                    >
                >
            >
        >,
        alloy::providers::fillers::WalletFiller<EthereumWallet>
    >,
    RootProvider<Http<Client>>,
    Http<Client>,
    Ethereum
>;

pub(crate) async fn angstrom_address_with_state(
    config: AngstromTestnetConfig
) -> eyre::Result<InitialTestnetState> {
    let mut anvil_builder = Anvil::new()
        .block_time(2)
        .chain_id(1)
        .arg("--ipc")
        .arg(format!("/tmp/anvil_temp_deploy.ipc"))
        .arg("--code-size-limit")
        .arg("393216")
        .arg("--disable-block-gas-limit");

    if let Some(fork_block) = config.fork_block_number() {
        anvil_builder = anvil_builder
            .arg("--fork-block-number")
            .arg(format!("{}", fork_block));
    }

    let anvil = anvil_builder.try_spawn()?;

    let endpoint = format!("/tmp/anvil_temp_deploy.ipc");
    tracing::info!(?endpoint);
    let ipc = alloy::providers::IpcConnect::new(endpoint.to_string());
    let sk: PrivateKeySigner = anvil.keys()[7].clone().into();
    let controller = anvil.addresses()[7];

    let wallet = EthereumWallet::new(sk);
    let rpc = builder::<Ethereum>()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_ipc(ipc)
        .await?;

    let (tx, _) = broadcast::channel(100);
    let provider = AnvilStateProvider {
        provider:       rpc,
        controller:     anvil.addresses()[7],
        canon_state_tx: tx,
        canon_state:    AnvilConsensusCanonStateNotification::new()
    };

    let (token0, token1) = mint_token_pair(&provider.provider).await;

    let uni_env = UniswapEnv::with_anvil(provider.clone()).await?;
    let angstrom_env = AngstromEnv::new(uni_env).await?;
    let angstrom = AngstromInstance::new(angstrom_env.angstrom(), angstrom_env.provider());

    let pool = PoolKey {
        currency0:   token0,
        currency1:   token1,
        fee:         U24::ZERO,
        tickSpacing: I24::unchecked_from(10),
        hooks:       Address::default()
    };

    angstrom
        .configurePool(
            pool.currency0,
            pool.currency1,
            pool.tickSpacing.try_into().unwrap(),
            pool.fee
        )
        .from(controller)
        .run_safe()
        .await
        .unwrap();

    tracing::debug!("deploying contracts to anvil");

    let state_bytes = provider.provider().anvil_dump_state().await?;
    let mut inital_state = InitialTestnetState::new(angstrom_env.angstrom(), state_bytes);

    Ok(inital_state)
}

pub async fn spawn_anvil(anvil_key: usize) -> eyre::Result<(AnvilInstance, AnvilWalletRpc)> {
    let anvil = Anvil::new()
        .chain_id(1)
        .arg("--ipc")
        .arg("--code-size-limit")
        .arg("393216")
        .arg("--disable-block-gas-limit")
        .try_spawn()?;

    let endpoint = "/tmp/anvil.ipc";
    tracing::info!(?endpoint);
    let ipc = IpcConnect::new(endpoint.to_string());
    let sk: PrivateKeySigner = anvil.keys()[anvil_key].clone().into();

    let wallet = EthereumWallet::new(sk);
    let rpc = builder::<Ethereum>()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_ipc(ipc)
        .await?;

    tracing::info!("connected to anvil");

    Ok((anvil, rpc))
}
