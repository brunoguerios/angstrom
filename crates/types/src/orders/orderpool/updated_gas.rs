use crate::primitive::PoolId;

/// given the block, returns the updated
/// gas
#[derive(Debug, Clone, Copy)]
pub struct UpdatedGas {
    pub block_number:      u64,
    pub pool_id:           PoolId,
    pub gas_internal_book: u64,
    pub gas_external_book: u64,
    pub gas_internal_tob:  u64,
    pub gas_external_tob:  u64
}
