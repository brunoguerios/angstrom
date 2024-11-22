mod anvil_cleanser;
mod rpc_provider;

use alloy_primitives::Address;
pub use rpc_provider::*;
mod anvil_provider;
mod state_provider;
pub use anvil_cleanser::*;
pub use anvil_provider::*;
pub use state_provider::*;
mod block_provider;
pub mod utils;
pub use block_provider::*;
mod initializer;
use alloy::pubsub::PubSubFrontend;
pub use initializer::*;
use utils::WalletProviderRpc;

use crate::contracts::environment::TestAnvilEnvironment;

#[derive(Debug, Clone)]
pub struct WalletProvider {
    provider:                  WalletProviderRpc,
    pub controller_address:    Address,
    pub controller_secret_key: alloy::signers::local::PrivateKeySigner
}

impl WalletProvider {
    pub fn new(
        provider: WalletProviderRpc,
        controller_address: Address,
        controller_secret_key: alloy::signers::local::PrivateKeySigner
    ) -> Self {
        Self { provider, controller_address, controller_secret_key }
    }

    pub fn provider_ref(&self) -> &WalletProviderRpc {
        &self.provider
    }

    pub fn provider(&self) -> WalletProviderRpc {
        self.provider.clone()
    }
}

impl TestAnvilEnvironment for WalletProvider {
    type P = WalletProviderRpc;
    type T = PubSubFrontend;

    fn provider(&self) -> &WalletProviderRpc {
        &self.provider
    }

    fn controller(&self) -> Address {
        self.controller_address
    }
}
