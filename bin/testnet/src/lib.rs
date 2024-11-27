//! CLI definition and entrypoint to executable
#![allow(unused)]
pub mod cli;
pub(crate) mod utils;

use reth::{tasks::TaskExecutor, CliRunner};

pub fn run() -> eyre::Result<()> {
    CliRunner::default().run_command_until_exit(|ctx| execute(ctx.task_executor))
}

async fn execute(_executor: TaskExecutor) -> eyre::Result<()> {
    // let cli = AngstromTestnetCli::parse();
    // executor.spawn_critical("metrics", cli.clone().init_metrics());

    // let testnet_config = cli.load_config()?;
    // let my_node_config = testnet_config.my_node_config()?;

    // let pub_key = my_node_config.secret_key.public_key(&Secp256k1::default());
    // let initial_validators = testnet_config.initial_validators();

    // let iam_leader = my_node_config.is_leader;

    // let config = TestnetConfig::new(3, Vec::new(), "ws://35.245.117.24:8546");

    // let testnet =
    //     AngstromTestnet::spawn_testnet(NoopProvider::default(), config,
    // initial_validators).await?;

    // executor
    //     .spawn_critical_blocking("testnet", testnet.run())
    //     .await?;

    Ok(())
}
