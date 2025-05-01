use angstrom_types::matching::SqrtPriceX96;
use reth_provider::test_utils::NoopProvider;
use reth_tasks::TaskExecutor;
use testing_tools::{
    controllers::enviroments::AngstromTestnet,
    types::{
        GlobalTestingConfig, actions::WithAction, checked_actions::WithCheckedAction,
        checks::WithCheck, config::DevnetConfig, initial_state::PartialConfigPoolKey
    }
};
use tracing::{debug, info};

use crate::cli::devnet::DevnetCli;

pub(crate) async fn run_devnet(executor: TaskExecutor, cli: DevnetCli) -> eyre::Result<()> {
    let config = cli.make_config()?;
    let initial_state = config.initial_state_config();

    let mut testnet =
        AngstromTestnet::spawn_devnet(NoopProvider::default(), config, executor.clone())
            .await?
            .as_state_machine();

    info!("deployed state machine");

    let token_gen = testnet
        .testnet
        .random_peer()
        .strom_validation(|v| v.underlying.token_price_generator());
    let pairs_to_pools = token_gen.pairs_to_pools();

    let new_pool_key =
        PartialConfigPoolKey::new(50, 60, 34028236692, SqrtPriceX96::at_tick(0).unwrap());

    testnet.advance_block();
    testnet.check_token_price_gen_has_pools(pairs_to_pools.clone());
    testnet.advance_block();
    testnet.deploy_new_pool(new_pool_key);
    testnet.check_token_price_gen_has_pools(pairs_to_pools.clone());

    testnet.run().await;

    Ok(())
}
