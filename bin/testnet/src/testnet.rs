use reth_provider::test_utils::NoopProvider;
use reth_tasks::TaskExecutor;
use testing_tools::{controllers::enviroments::AngstromTestnet, types::config::TestnetConfig};

use crate::cli::testnet::TestnetCli;

pub(crate) async fn run_testnet(executor: TaskExecutor, cli: TestnetCli) -> eyre::Result<()> {
    let config = cli.make_config()?;

    let testnet = AngstromTestnet::spawn_testnet(NoopProvider::default(), config).await?;

    testnet.run_to_completion(executor).await;
    Ok(())
}
