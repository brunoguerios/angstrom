use alloy_primitives::TxHash;

use super::{AngstromBundle, AngstromSigner, ChainSubmitter, TxFeatureInfo};

pub struct AngstromSubmitter {}

impl ChainSubmitter for AngstromSubmitter {
    fn submit<'a>(
        &'a self,
        signer: &'a AngstromSigner,
        bundle: Option<&'a AngstromBundle>,
        tx_features: &'a TxFeatureInfo
    ) -> std::pin::Pin<Box<dyn Future<Output = eyre::Result<TxHash>> + Send + 'a>> {
        Box::pin(async move { todo!() })
    }
}
