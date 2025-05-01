use std::collections::HashMap;

use alloy_primitives::Address;
use angstrom_types::{matching::SqrtPriceX96, primitive::PoolId};
use itertools::Itertools;
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
use validation::common::TokenPriceGenerator;

use crate::cli::devnet::DevnetCli;

pub(crate) async fn run_devnet(executor: TaskExecutor, cli: DevnetCli) -> eyre::Result<()> {
    let config = cli.make_config()?;
    let initial_state = config.initial_state_config();

    let mut testnet =
        AngstromTestnet::spawn_devnet(NoopProvider::default(), config, executor.clone()).await?;

    info!("deployed state machine");

    let peer = testnet.random_peer();
    peer.start_network_and_consensus_and_validation();

    //

    let token_gen = peer.strom_validation(|v| v.underlying.token_price_generator());
    let mut pairs_to_pools = token_gen.pairs_to_pools();
    checked_pair_to_pool(pairs_to_pools.clone(), token_gen.clone());

    let new_pool_key =
        PartialConfigPoolKey::new(50, 60, 34028236692, SqrtPriceX96::at_tick(0).unwrap());

    peer.deploy_new_pool(new_pool_key).await?;
    peer.state_provider().mine_block().await?;

    checked_pair_to_pool(pairs_to_pools.clone(), token_gen.clone());

    // testnet.advance_block();
    // testnet.deploy_new_pool(new_pool_key);
    // testnet.advance_block();
    // testnet.check_token_price_gen_has_pools(pairs_to_pools.clone());
    // // testnet.advance_block();
    // // pairs_to_pools.in
    // testnet.check_token_price_gen_has_pools(pairs_to_pools.clone());

    // testnet.run().await;

    Ok(())
}

fn checked_pair_to_pool(
    checked_pair_to_pool: HashMap<(Address, Address), PoolId>,
    token_gen: TokenPriceGenerator
) {
    let pairs_to_pools = token_gen.pairs_to_pools();
    let binding = token_gen.prev_prices();
    let prev_prices = binding.keys().sorted().collect::<Vec<_>>();

    let checked_pair_to_pool_ids = checked_pair_to_pool.values().sorted().collect::<Vec<_>>();

    assert_eq!(checked_pair_to_pool, pairs_to_pools);

    assert_eq!(prev_prices, checked_pair_to_pool_ids);
}
