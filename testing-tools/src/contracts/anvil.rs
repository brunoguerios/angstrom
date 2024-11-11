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
use alloy_primitives::{Address, Bytes};
use tokio::sync::broadcast;

use crate::{
    anvil_state_provider::AnvilStateProvider,
    contracts::environment::{angstrom::AngstromEnv, uniswap::UniswapEnv},
    mocks::canon_state::AnvilConsensusCanonStateNotification,
    testnet_controllers::AngstromTestnetConfig
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
) -> eyre::Result<(Address, Bytes)> {
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

    let uni_env = UniswapEnv::with_anvil(provider.clone()).await?;
    let angstrom_env = AngstromEnv::new(uni_env).await?;

    tracing::debug!("deploying contracts to anvil");

    let state = provider.provider().anvil_dump_state().await?;

    Ok((angstrom_env.angstrom(), state))
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
