use std::collections::HashMap;

use alloy::{
    dyn_abi::Eip712Domain,
    primitives::{aliases::U24, Address},
    sol,
    sol_types::eip712_domain
};

use crate::contract_bindings::angstrom::Angstrom::PoolKey;

sol! {
#![sol(all_derives = true)]
ERC20,
"src/primitive/contract/ERC20.json"}

pub use ERC20::*;

use crate::primitive::PoolId;

pub const TESTNET_ANGSTROM_ADDRESS: Address =
    alloy::primitives::address!("293954613283cC7B82BfE9676D3cc0fb0A58fAa0");

// The `eip712_domain` macro lets you easily define an EIP-712 domain
// object :)
pub const ANGSTROM_DOMAIN: Eip712Domain = eip712_domain!(
    name: "Angstrom",
    version: "v1",
    chain_id: 1,
    verifying_contract: TESTNET_ANGSTROM_ADDRESS,
);

#[derive(Default, Clone)]
pub struct UniswapPoolRegistry {
    pools:              HashMap<PoolId, PoolKey>,
    pub conversion_map: HashMap<PoolId, PoolId>
}
impl UniswapPoolRegistry {
    pub fn get(&self, pool_id: &PoolId) -> Option<&PoolKey> {
        self.pools.get(pool_id)
    }

    pub fn pools(&self) -> HashMap<PoolId, PoolKey> {
        self.pools.clone()
    }
}
impl From<Vec<PoolKey>> for UniswapPoolRegistry {
    fn from(pools: Vec<PoolKey>) -> Self {
        let pubmap = pools
            .iter()
            .map(|pool_key| {
                let pool_id = PoolId::from(pool_key.clone());
                (pool_id, pool_key.clone())
            })
            .collect();

        let priv_map = pools
            .into_iter()
            .map(|mut pool_key| {
                let pool_id_pub = PoolId::from(pool_key.clone());
                pool_key.fee = U24::from(0x800000);
                let pool_id_priv = PoolId::from(pool_key.clone());
                (pool_id_pub, pool_id_priv)
            })
            .collect();
        Self { pools: pubmap, conversion_map: priv_map }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use alloy::signers::{local::LocalSigner, SignerSync};
    use alloy_primitives::{address, bytes, Bytes, PrimitiveSignature, U256};
    use pade::{PadeDecode, PadeEncode};

    use super::*;
    use crate::sol_bindings::rpc_orders::{ExactFlashOrder, OmitOrderMeta};

    #[test]
    fn test_this() {
        let private_key = "7bbd27422e113bc608150c92768febbf9c4d6e6cea369e121f459cb2e3a07c08";
        let signer = LocalSigner::from_str(private_key).unwrap();

        let order = ExactFlashOrder {
            ref_id:               0,
            exact_in:             true,
            amount:               1000000000000000000,
            max_extra_fee_asset0: 1000000000000000000,
            min_price:            U256::from(36481077109493002240u128),
            use_internal:         false,
            asset_in:             address!("0x3d85e7b30be9fd7a4bad709d6ed2d130579f9a2e"),
            asset_out:            address!("0xd550015f84142abecd5e82c8f296083df3c9a80d"),
            recipient:            address!("0xa7f1aeb6e43443c683865fdb9e15dd01386c955b"),
            hook_data:            Default::default(),
            valid_for_block:      21787256,
            meta:                 Default::default()
        };

        let hash = order.no_meta_eip712_signing_hash(&ANGSTROM_DOMAIN);
        let sig: Bytes = signer.sign_hash_sync(&hash).unwrap().pade_encode().into();
        println!("{sig:?}\n");

        let decoded_sig: Bytes = PrimitiveSignature::pade_decode(&mut &**bytes!("0x01d960219e5211b30a82004ae8405e1f9e3e9e981a4de77a7ffc4e58830efe4dcb6bae0c7f0c005e6e1193b0086bae6d5729a2d4c9db2ab926df912076a6db717f"), None).unwrap().pade_encode().into();
        println!("{decoded_sig:?}");

        assert_eq!(decoded_sig, sig);
    }
}
