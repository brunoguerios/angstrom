use std::{
    collections::VecDeque,
    ops::RangeToInclusive,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, RwLock
    }
};

use dashmap::DashMap;

/// The global block sync is a global syncing mechanism.
///
/// every module that interacts with blocks and or other modules that are block
/// sensitive are registered with the global block sync. The global block number
/// will only increase when all modules that are registered mark the global sync
/// as ready to increment. Unlike progressing. Any one module is able to abort
/// the process (aborts due to reorgs)
#[derive(Debug, Clone)]
pub struct GlobalBlockSync {
    /// state that we are waiting on all sign offs for
    pending_state:      Arc<RwLock<VecDeque<GlobalBlockState>>>,
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
            pending_state:      Arc::new(RwLock::new(VecDeque::with_capacity(2))),
            registered_modules: DashMap::default()
        }
    }

    pub fn new_block(&self, block_number: u64) {
        // add to pending state. this will trigger everyone to stop and start dealing
        // with new blocks
        self.pending_state
            .write()
            .unwrap()
            .push_back(GlobalBlockState::PendingProgression(block_number));
    }

    pub fn reorg(&self, reorg_range: RangeToInclusive<u64>) {
        self.pending_state
            .write()
            .unwrap()
            .push_back(GlobalBlockState::PendingReorg(reorg_range));
    }

    pub fn sign_off_reorg(&self, module: &'static str, block_number: u64) {
        // check to see if there is pending state
        if self.pending_state.read().unwrap().is_empty() {
            panic!("someone tried to sign off on a proposal that didn't exist");
        }
        // ensure the block number is cur_block + 1
        if self.current_block_number() != block_number {
            panic!("current_block_number != block_number, incorrectly delt with reorg");
        }

        let check = SignOffState::HandledReorg(block_number);

        self.registered_modules
            .entry(module)
            .and_modify(|sign_off_state| {
                *sign_off_state = check;
            });

        let mut transition = true;
        self.registered_modules.iter().for_each(|v| {
            transition &= *v.value() == check;
        });

        if transition {
            // was last sign off, pending state -> cur state
            let mut lock = self.pending_state.write().unwrap();
            let new_state = lock
                .pop_front()
                .expect("everyone signed off on a transition proposal that didn't exist");
            drop(lock);

            tracing::info!(handled_state=?new_state, "detected reorg has been handled successfully");

            // reset sign off state
            self.registered_modules.iter_mut().for_each(|mut v| {
                *v.value_mut() = Default::default();
            });

            self.block_number.fetch_add(1, Ordering::SeqCst);
        }
    }

    pub fn sign_off_on_block(&self, module: &'static str, block_number: u64) {
        // check to see if there is pending state
        if self.pending_state.read().unwrap().is_empty() {
            panic!("someone tried to sign off on a proposal that didn't exist");
        }
        // ensure the block number is cur_block + 1
        if self.current_block_number() + 1 != block_number {
            panic!("current_block_number + 1 != block_number");
        }

        let check = SignOffState::ReadyForNextBlock(block_number);

        self.registered_modules
            .entry(module)
            .and_modify(|sign_off_state| {
                *sign_off_state = check;
            });

        let mut transition = true;
        self.registered_modules.iter().for_each(|v| {
            transition &= *v.value() == check;
        });

        if transition {
            let mut lock = self.pending_state.write().unwrap();
            let new_state = lock
                .pop_front()
                .expect("everyone signed off on a transition proposal that didn't exist");
            drop(lock);

            tracing::info!(handled_state=?new_state, "new block has been handled successfully");

            // reset sign off state
            self.registered_modules.iter_mut().for_each(|mut v| {
                *v.value_mut() = Default::default();
            });

            self.block_number.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[inline(always)]
    pub fn can_operate(&self) -> bool {
        !self.has_proposal()
    }

    #[inline(always)]
    pub fn current_block_number(&self) -> u64 {
        self.block_number.load(Ordering::SeqCst)
    }

    #[inline(always)]
    pub fn has_proposal(&self) -> bool {
        !self.pending_state.read().unwrap().is_empty()
    }

    pub fn register(&self, module_name: &'static str) {
        if self
            .registered_modules
            .insert(module_name, SignOffState::default())
            .is_some()
        {
            panic!("module registered twice with global block sync");
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalBlockState {
    /// current block number processing
    Processing(u64),
    /// a block that we need to deal with the reorg for
    PendingReorg(RangeToInclusive<u64>),
    /// a new block that all modules need to index
    PendingProgression(u64)
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SignOffState {
    /// no sign off has occured yet
    #[default]
    Pending,
    /// module has registered that there is a new block and made sure it is up
    /// to date
    ReadyForNextBlock(u64),
    /// module has registered that there was a reorg and has appropritaly
    /// handled it and is ready to continue processing
    HandledReorg(u64)
}
