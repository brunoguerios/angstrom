use std::pin::Pin;

use angstrom_types::testnet::InitialTestnetState;
use futures::Future;
use reth_provider::test_utils::NoopProvider;
use reth_tasks::TaskExecutor;
use testing_tools::{
    agents::AgentConfig,
    controllers::enviroments::AngstromTestnet,
    order_generator::OrderGenerator,
    types::{
        actions::WithAction, checked_actions::WithCheckedAction, checks::WithCheck,
        config::DevnetConfig
    }
};
use tracing::{debug, info};

use crate::cli::e2e_orders::End2EndOrdersCli;

pub async fn run_e2e_orders(executor: TaskExecutor, cli: End2EndOrdersCli) -> eyre::Result<()> {
    let config = cli.testnet_config.make_config()?;
    let handle = executor.clone();

    let mut agents = Vec::new();
    agents.push(end_to_end_agent);

    // spawn testnet
    let testnet = AngstromTestnet::spawn_testnet(NoopProvider::default(), config, agents).await?;

    executor
        .spawn_critical_blocking("testnet", testnet.run_to_completion())
        .await?;
    Ok(())
}

fn end_to_end_agent<'a>(
    init_tesetnet_state: &'a InitialTestnetState,
    agent_config: &'a AgentConfig
) -> Pin<Box<dyn Future<Output = eyre::Result<()>> + Send + 'a>> {
    Box::pin(async {
        let generator = OrderGenerator::new(
            agent_config.uniswap_pools.clone(),
            agent_config.current_block,
            5..20,
            0.05..0.1
        );

        Ok(())
    }) as Pin<Box<dyn Future<Output = eyre::Result<()>> + Send + 'a>>
}
