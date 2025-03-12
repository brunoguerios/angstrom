use alloy::primitives::{Address, B256};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::primitive::PoolId;

#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
pub enum OrderValidationError {
    #[error(transparent)]
    StateError(#[from] UserAccountVerificationError),
    #[error("the input or output generates a invalid tob swap")]
    InvalidToBSwap,
    #[error("min qty on partial < max gas amount")]
    InvalidPartialOrder,
    #[error("invalid signature")]
    InvalidSignature,
    #[error("invalid pool")]
    InvalidPool,
    #[error("not enough gas t0")]
    NotEnoughGas,
    #[error("duplicate order")]
    DuplicateOrder,
    #[error("Not Valid At Block")]
    InvalidOrderAtBlock,
    #[error("unknown")]
    Unknown(String)
}

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum UserAccountVerificationError {
    #[error("tried to verify for block {} where current is {}", requested, current)]
    BlockMissMatch { requested: u64, current: u64, pool_info: UserOrderPoolInfo },
    #[error("order hash has been cancelled {0:?}")]
    OrderIsCancelled(B256),
    #[error("Nonce exists for a current order hash: {0:?}")]
    DuplicateNonce(B256),
    #[error("block for flash order is not for next block. next_block: {0}, requested_block: {1}.")]
    BadBlock(u64, u64),
    #[error("currently hooks are not supported. this field should be empty bytes")]
    NonEmptyHook,
    #[error("could not fetch nonce '{1}' for user '{0}'")]
    CouldNotFetch(Address, u64)
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserOrderPoolInfo {
    // token in for pool
    pub token:   Address,
    pub is_bid:  bool,
    pub pool_id: PoolId
}
