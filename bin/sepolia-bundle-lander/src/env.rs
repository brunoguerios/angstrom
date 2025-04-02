use std::sync::Arc;

use alloy::providers::Provider;
use angstrom_types::primitive::AngstromSigner;

pub struct BundleWashTraderEnv {
    pub keys:     Vec<AngstromSigner>,
    pub provider: Arc<Box<dyn Provider>>
}
pub async fn setup_env() {
    let querying_provider: Arc<_> = ProviderBuilder::<_, _, Ethereum>::default()
        .with_recommended_fillers()
        .layer(RethDbLayer::new(node.provider().clone()))
        // backup
        .connect(&url)
        .await
        .unwrap()
        .into();
}
