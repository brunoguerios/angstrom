//! CLI definition and entrypoint to executable
pub mod components;
pub mod config;
pub(crate) mod utils;

use clap::Parser;
use config::AngstromDevnetCli;
use reth::{tasks::TaskExecutor, CliRunner};
use secp256k1::Secp256k1;
use testing_tools::controllers::testnet::{TestnetConfig, TestnetLeaderConfig};

pub fn run() -> eyre::Result<()> {
    CliRunner::default().run_command_until_exit(|ctx| execute(ctx.task_executor))
}

async fn execute(executor: TaskExecutor) -> eyre::Result<()> {
    let cli = AngstromDevnetCli::parse();
    executor.spawn_critical("metrics", cli.clone().init_metrics());

    let testnet_nodes_config = cli.load_config()?;
    let my_node_config = testnet_nodes_config.my_node_config()?;
    let leader_node_config = testnet_nodes_config.leader_node_config()?;

    let pub_key = my_node_config.secret_key.public_key(&Secp256k1::default());

    let config = TestnetConfig::new(
        7,
        1,
        "ws://localhost:8545",
        my_node_config.address,
        pub_key,
        my_node_config.signing_key,
        my_node_config.secret_key,
        testnet_nodes_config.pools,
        angstrom_address,
        Some(TestnetLeaderConfig::new(testnet_nodes_config.leader.ws_url()))
    );

    Ok(())
}
