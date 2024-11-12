use alloy_primitives::{
    aliases::{I24, U24},
    Address, Bytes
};
use angstrom_types::contract_bindings::angstrom::Angstrom::PoolKey;

#[derive(Debug, Clone)]
pub struct InitialTestnetState {
    pub angstrom_addr: Address,
    pub state:         Bytes,
    pub pool_keys:     Vec<PoolKey>
}

impl InitialTestnetState {
    pub fn new(angstrom_addr: Address, state: Bytes) -> Self {
        Self { angstrom_addr, state, pool_keys: Vec::new() }
    }

    pub fn add_key(
        &mut self,
        currency0: Address,
        currency1: Address,
        hooks: Address,
        fee: Option<u32>,
        tick_spacing: Option<i32>
    ) {
        let key = PoolKey {
            currency0,
            currency1,
            fee: U24::from(fee.unwrap_or_default()),
            tickSpacing: I24::try_from(tick_spacing.unwrap_or_default()).unwrap(),
            hooks
        };

        self.pool_keys.push(key);
    }
}
