use std::sync::Arc;

use alloy::{eips::Encodable2718, providers::Provider};
use alloy_primitives::{Address, TxHash};
use futures::{
    TryStreamExt,
    stream::{StreamExt, iter}
};

use super::{
    AngstromBundle, AngstromSigner, ChainSubmitter, DEFAULT_SUBMISSION_CONCURRENCY, TxFeatureInfo
};

/// handles submitting transaction to
pub struct MempoolSubmitter<P> {
    clients:          Vec<Arc<P>>,
    angstrom_address: Address
}

impl<P> MempoolSubmitter<P> {
    pub fn new(clients: Vec<Arc<P>>, angstrom_address: Address) -> Self {
        Self { clients, angstrom_address }
    }
}

impl<P> ChainSubmitter for MempoolSubmitter<P>
where
    P: Provider + Unpin + 'static
{
    fn angstrom_address(&self) -> Address {
        self.angstrom_address
    }

    fn submit<'a>(
        &'a self,
        signer: &'a AngstromSigner,
        bundle: Option<&'a AngstromBundle>,
        tx_features: &'a TxFeatureInfo
    ) -> std::pin::Pin<Box<dyn Future<Output = eyre::Result<Option<TxHash>>> + Send + 'a>> {
        Box::pin(async move {
            let Some(bundle) = bundle else { return Err(eyre::eyre!("no bundle was past in")) };

            let tx = self.build_and_sign_tx(signer, bundle, tx_features).await;
            let encoded_tx = tx.encoded_2718();
            let tx_hash = *tx.tx_hash();

            // Clone here is fine as its in a Arc
            let _: Vec<_> = iter(self.clients.clone())
                .map(async |client| client.send_raw_transaction(&encoded_tx).await)
                .buffer_unordered(DEFAULT_SUBMISSION_CONCURRENCY)
                .try_collect()
                .await?;

            Ok(Some(tx_hash))
        })
    }
}
