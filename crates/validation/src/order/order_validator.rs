use std::{
    fmt::Debug,
    pin::Pin,
    sync::{Arc, atomic::AtomicU64}
};

use alloy::primitives::{Address, B256, BlockNumber};
use angstrom_metrics::validation::ValidationMetrics;
use angstrom_types::sol_bindings::{RawPoolOrder, grouped_orders::AllOrders};
use futures::Future;
use rand::random;
use tokio::runtime::Handle;

use super::{
    OrderValidationRequest,
    sim::SimValidation,
    state::{
        StateValidation, account::user::UserAddress, db_state_utils::StateFetchUtils,
        pools::PoolsTracker
    }
};
use crate::{
    common::{TokenPriceGenerator, key_split_threadpool::KeySplitThreadpool},
    order::{OrderValidation, state::account::UserAccountProcessor},
    telemetry::ValidationSnapshot
};
use amms::SyncedPools;

pub struct OrderValidator<DB, Pools, Fetch> {
    sim:                     SimValidation<DB>,
    pub(crate) state:        StateValidation<Pools, Fetch>,
    pub(crate) block_number: Arc<AtomicU64>
}

impl<DB, Pools, Fetch> OrderValidator<DB, Pools, Fetch>
where
    DB: Unpin + Clone + 'static + revm::DatabaseRef + reth_provider::BlockNumReader + Sync + Send,
    <DB as revm::DatabaseRef>::Error: Send + Sync + Debug,
    Pools: PoolsTracker + Send + Sync + 'static,
    Fetch: StateFetchUtils + Send + Sync + 'static
{
    pub async fn new(
        sim: SimValidation<DB>,
        block_number: Arc<AtomicU64>,
        pools: Pools,
        fetch: Fetch,
        amm_pools: SyncedPools
    ) -> Self {
        let state = StateValidation::new(UserAccountProcessor::new(fetch), pools, amm_pools);

        Self { state, sim, block_number }
    }

    pub fn fetch_nonce(&self, addr: Address) -> u64 {
        loop {
            let nonce = random();
            let Ok(is_valid) = self
                .state
                .user_account_tracker
                .fetch_utils
                .is_valid_nonce(addr, nonce)
            else {
                panic!("db failure");
            };

            if is_valid {
                return nonce;
            }
        }
    }

    pub fn cancel_order(&self, user: Address, hash: B256) {
        self.state.cancel_order(user, hash);
    }

    pub fn on_new_block(
        &mut self,
        block_number: BlockNumber,
        completed_orders: Vec<B256>,
        address_changes: Vec<Address>
    ) {
        self.block_number
            .store(block_number, std::sync::atomic::Ordering::Relaxed);
        // when this occurs, we know there are currently no pending orders and thus we
        // can snapshot them.
        let state = self.state.user_account_tracker.user_accounts.deep_clone();
        telemetry_recorder::telemetry_event!(ValidationSnapshot::from((block_number, state)));

        self.state.new_block(completed_orders, address_changes);
    }

    /// only checks state
    pub fn validate_order(
        &mut self,
        order: OrderValidationRequest,
        token_conversion: TokenPriceGenerator,
        thread_pool: &mut KeySplitThreadpool<
            UserAddress,
            Pin<Box<dyn Future<Output = ()> + Send + Sync>>,
            Handle
        >,
        metrics: ValidationMetrics
    ) {
        let block_number = self.block_number.load(std::sync::atomic::Ordering::Relaxed);
        let order_validation: OrderValidation = order.into();
        let user = order_validation.user();
        let cloned_state = self.state.clone();
        let cloned_sim = self.sim.clone();

        thread_pool.add_new_task(
            user,
            Box::pin(async move {
                match order_validation {
                    OrderValidation::Limit(tx, order, loc) => {
                        metrics
                            .new_order(false, || async {
                                let mut results = cloned_state
                                    .handle_orders(
                                        order,
                                        block_number,
                                        metrics.clone(),
                                        loc.is_revalidating(),
                                        async |_, _| Ok((0u128, 0u128))
                                    )
                                    .await;

                                results.add_gas_cost_or_invalidate(
                                    &cloned_sim,
                                    &token_conversion,
                                    true,
                                    block_number
                                );

                                let _ = tx.send(results);
                            })
                            .await;
                    }
                    OrderValidation::Searcher(tx, order, _) => {
                        metrics
                            .new_order(true, || async {
                                let AllOrders::TOB(order) = order else { panic!() };
                                let (mut t0, mut t1) = (order.token_in(), order.token_out());

                                // ensure we are in order as it does matter here
                                if t0 > t1 {
                                    std::mem::swap(&mut t0, &mut t1);
                                }

                                let conversion_rate = token_conversion
                                    .conversion_rate_of_pair(t0, t1)
                                    .unwrap_or_default();

                                let mut results = cloned_state
                                    .handle_tob_order(
                                        order,
                                        block_number,
                                        conversion_rate,
                                        metrics.clone()
                                    )
                                    .await;
                                results.add_gas_cost_or_invalidate(
                                    &cloned_sim,
                                    &token_conversion,
                                    false,
                                    block_number
                                );
                                let _ = tx.send(results);
                            })
                            .await;
                    }
                }
            })
        );
    }
}
