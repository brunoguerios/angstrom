use alloy::primitives::U256;
use angstrom_types::primitive::PoolId;
use futures::Stream;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Slot0Update {
    /// there will be 120 updates per block or per 100ms
    pub seq_id:        u8,
    /// incase of block lag on node
    pub current_block: u64,
    /// basic identifier
    pub pool_id:       PoolId,

    pub sqrt_price_x96: U256,
    pub tick:           i32
}

pub trait AngstromBookQuoter: Stream<Item = Slot0Update> {
    /// will configure this stream to receieve updates of the given pool
    fn subscribe_to_updates(&self, pool_id: PoolId);
    /// will configure this stream to stop updates of the given pool
    fn unsubscribe_to_updates(&self, pool_id: PoolId);
}

/// to enable re-usability,
pub struct QuoterManager {}
