use reth_provider::test_utils::NoopProvider;
use reth_tasks::TaskExecutor;
use testing_tools::{
    controllers::enviroments::AngstromTestnet,
    types::{
        actions::WithAction, checked_actions::WithCheckedAction, checks::WithCheck,
        config::DevnetConfig
    }
};
use tracing::{debug, info};

use crate::cli::e2e_orders::End2EndOrdersCli;
pub async fn run_e2e_orders(executor: TaskExecutor, cli: End2EndOrdersCli) -> eyre::Result<()> {}
