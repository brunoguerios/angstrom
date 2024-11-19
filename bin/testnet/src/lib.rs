//! CLI definition and entrypoint to executable
pub mod components;
pub mod config;

use clap::Parser;
use config::AngstromDevnetCli;
use reth::{tasks::TaskExecutor, CliRunner};
use secp256k1::{Secp256k1, SecretKey};
use validation::validator::ValidationClient;

use crate::components::{init_network_builder, initialize_strom_handles};

pub fn run() -> eyre::Result<()> {
    CliRunner::default().run_command_until_exit(|ctx| execute(ctx.task_executor))
}

async fn execute(executor: TaskExecutor) -> eyre::Result<()> {
    let cli = AngstromDevnetCli::parse();
    executor.spawn_critical("metrics", cli.clone().init_metrics());

    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let pub_key = secret_key.public_key(&Secp256k1::default());

    let mut network = init_network_builder(secret_key)?;
    let protocol_handle = network.build_protocol_handler();
    let channels = initialize_strom_handles();

    // for rpc
    let pool = channels.get_pool_handle();
    let executor_clone = executor.clone();
    let validation_client = ValidationClient(channels.validator_tx.clone());

    // let config = TestnetConfig::new(
    //     7,
    //     1,
    //     "ws://localhost:8545",
    //     Address::random(),
    //     pub_key,
    //     secret_key,
    //     secret_key,
    //     vec![],
    //     angstrom_address,
    //     Some(TestnetLeaderConfig::new(20000000, "ws://35.245.117.24:8546"))
    // );

    Ok(())
}
