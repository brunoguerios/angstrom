use std::sync::{
    atomic::{AtomicU64, AtomicU8, Ordering},
    Arc, Mutex, RwLock
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
    cur_state:          Arc<RwLock<GlobalBlockState>>,
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
            cur_state:          Arc::new(RwLock::new(GlobalBlockState::Processing)),
            pending_state:      Arc::new(Mutex::new(None)),
            registered_modules: DashMap::default()
        }
    }

    pub fn sign_off_reorg(&self, module: &'static str, block_number: u64) {
        // check to see if there is pending state
        if self.pending_state.lock().unwrap().is_none() {
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
            // if self.rem_sign_offs.fetch_sub(1, Ordering::SeqCst) == 1 {
            let mut lock = self.pending_state.lock().unwrap();
            let new_state = lock
                .take()
                .expect("everyone signed off on a transition proposal that didn't exist");
            drop(lock);

            *self.cur_state.write().unwrap() = new_state;

            // reset sign off state
            self.registered_modules.iter_mut().for_each(|mut v| {
                *v.value_mut() = Default::default();
            });

            self.block_number.fetch_add(1, Ordering::SeqCst);
        }
    }

    pub fn sign_off_on_block(&self, module: &'static str, block_number: u64) {
        // check to see if there is pending state
        if self.pending_state.lock().unwrap().is_none() {
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
            // was last sign off, pending state -> cur state
            // if self.rem_sign_offs.fetch_sub(1, Ordering::SeqCst) == 1 {
            let mut lock = self.pending_state.lock().unwrap();
            let new_state = lock
                .take()
                .expect("everyone signed off on a transition proposal that didn't exist");
            drop(lock);

            *self.cur_state.write().unwrap() = new_state;

            // reset sign off state
            self.registered_modules.iter_mut().for_each(|mut v| {
                *v.value_mut() = Default::default();
            });

            self.block_number.fetch_add(1, Ordering::SeqCst);
        }
    }

    pub fn can_operate(&self) -> bool {
        matches!(*self.cur_state.read().unwrap(), GlobalBlockState::Processing)
    }

    pub fn current_block_number(&self) -> u64 {
        self.block_number.load(Ordering::SeqCst)
    }

    pub fn has_proposal(&self) -> bool {
        self.pending_state.lock().unwrap().is_some()
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

#[derive(Debug, Clone, Copy, Default)]
pub enum GlobalBlockState {
    Processing,
    PendingReorg,
    #[default]
    PendingProgression
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SignOffState {
    #[default]
    Pending,
    ReadyForNextBlock(u64),
    HandledReorg(u64)
}
