use secp256k1::{PublicKey, SecretKey};

use super::TestingConfig;

#[derive(Debug, Clone)]
pub struct TestnetNodeConfig<C: TestingConfig> {
    pub node_id:        u64,
    pub testnet_config: C,
    pub pub_key:        PublicKey,
    pub secret_key:     SecretKey,
    pub voting_power:   u64
}

impl<C: TestingConfig> TestnetNodeConfig<C> {
    pub fn new(
        node_id: u64,
        testnet_config: C,
        pub_key: PublicKey,
        secret_key: SecretKey,
        voting_power: u64
    ) -> Self {
        Self { node_id, testnet_config, pub_key, secret_key, voting_power }
    }

    pub fn is_leader(&self) -> bool {
        self.node_id == 0
    }
}
