use std::sync::{
    atomic::{AtomicU64, AtomicU8, Ordering},
    Arc, Mutex
};

use dashmap::DashMap;

/// The global block sync is a global syncing mechanism. every module
/// that interacts with blocks and or other modules that are block sensitive
/// are registered with the global block sync. The global block number will only
/// increase when all modules that are registered mark the global sync as ready
/// to increment. Unlike progressing. Any one module is able to abort the
/// process (aborts due to reorgs)
#[derive(Debug, Clone)]
pub struct GlobalBlockSync {
    cur_state:          GlobalBlockState,
    /// state that we are waiting on all sign offs for
    pending_state:      Arc<Mutex<Option<GlobalBlockState>>>,
    /// the block number
    block_number:       Arc<AtomicU64>,
    /// the modules with there current sign off state for the transition of
    /// pending state -> cur state
    registered_modules: DashMap<&'static str, SignOffState>
}

impl GlobalBlockSync {
    pub fn new(block_number: u64) -> Self {
        Self {
            block_number:       Arc::new(AtomicU64::new(block_number)),
            cur_state:          GlobalBlockState::Processing,
            pending_state:      Arc::new(Mutex::new(None)),
            registered_modules: DashMap::default()
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum GlobalBlockState {
    Processing,
    AbortedReorg,
    #[default]
    PendingProgression
}

#[derive(Debug, Clone, Copy, Default)]
pub enum SignOffState {
    #[default]
    Pending,
    ReadyForNextBlock(u64),
    HandledReorg
}
