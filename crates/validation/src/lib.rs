pub mod bundle;
pub mod common;
pub mod order;
pub mod telemetry;
pub mod validator;

use std::{
    fmt::Debug,
    pin::Pin,
    sync::{Arc, atomic::AtomicU64}
};

use alloy::primitives::Address;
use angstrom_types::{
    contract_payloads::angstrom::AngstromPoolConfigStore, pair_with_price::PairsWithPrice,
    reth_db_wrapper::SetBlock
};
use bundle::BundleValidator;
use common::SharedTools;
use futures::Stream;
use tokio::sync::mpsc::UnboundedReceiver;
use validator::Validator;

pub use crate::order::state::db_state_utils::finders::{
    find_slot_offset_for_approval, find_slot_offset_for_balance
};
use crate::{
    common::{TokenPriceGenerator, key_split_threadpool::KeySplitThreadpool},
    order::{
        order_validator::OrderValidator,
        sim::SimValidation,
        state::{db_state_utils::FetchUtils, pools::AngstromPoolsTracker}
    },
    validator::{ValidationClient, ValidationRequest}
};
use amms::SyncedPools;

const MAX_VALIDATION_PER_ADDR: usize = 3;

#[allow(clippy::too_many_arguments)]
pub fn init_validation<
    DB: Unpin
        + Clone
        + 'static
        + SetBlock
        + reth_provider::BlockNumReader
        + revm::DatabaseRef
        + Send
        + Sync
>(
    db: DB,
    current_block: u64,
    angstrom_address: Address,
    node_address: Address,
    update_stream: Pin<Box<dyn Stream<Item = (u64, u128, Vec<PairsWithPrice>)> + Send + 'static>>,
    amm_pools: SyncedPools,
    price_generator: TokenPriceGenerator,
    pool_store: Arc<AngstromPoolConfigStore>,
    validator_rx: UnboundedReceiver<ValidationRequest>
) where
    <DB as revm::DatabaseRef>::Error: Send + Sync + Debug
{
    init_validation_replay(
        db,
        current_block,
        angstrom_address,
        node_address,
        update_stream,
        amm_pools,
        price_generator,
        pool_store,
        validator_rx,
        |_| {}
    );
}

#[allow(clippy::too_many_arguments)]
pub fn init_validation_replay<
    DB: Unpin
        + Clone
        + 'static
        + SetBlock
        + reth_provider::BlockNumReader
        + revm::DatabaseRef
        + Send
        + Sync
>(
    db: DB,
    current_block: u64,
    angstrom_address: Address,
    node_address: Address,
    update_stream: Pin<Box<dyn Stream<Item = (u64, u128, Vec<PairsWithPrice>)> + Send + 'static>>,
    amm_pools: SyncedPools,
    price_generator: TokenPriceGenerator,
    pool_store: Arc<AngstromPoolConfigStore>,
    validator_rx: UnboundedReceiver<ValidationRequest>,
    hook: impl FnOnce(&mut Validator<DB, AngstromPoolsTracker, FetchUtils<DB>>) + Send + 'static
) where
    <DB as revm::DatabaseRef>::Error: Send + Sync + Debug
{
    let current_block = Arc::new(AtomicU64::new(current_block));
    let revm_lru = Arc::new(db);
    let fetch = FetchUtils::new(angstrom_address, revm_lru.clone());

    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(8)
            .build()
            .unwrap();

        let handle = rt.handle().clone();
        let pools = AngstromPoolsTracker::new(angstrom_address, pool_store);
        // load storage slot state + pools
        let thread_pool = KeySplitThreadpool::new(handle, MAX_VALIDATION_PER_ADDR);
        let sim = SimValidation::new(revm_lru.clone(), angstrom_address, node_address);

        let order_validator =
            rt.block_on(OrderValidator::new(sim, current_block, pools, fetch, amm_pools));

        let bundle_validator =
            BundleValidator::new(revm_lru.clone(), angstrom_address, node_address);
        let shared_utils = SharedTools::new(price_generator, update_stream, thread_pool);

        rt.block_on(async {
            let mut validator = Validator::new(
                validator_rx,
                order_validator,
                bundle_validator,
                shared_utils,
                revm_lru
            );
            hook(&mut validator);
            validator.await
        })
    });
}
