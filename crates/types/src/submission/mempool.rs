use alloy::{
    eips::{Encodable2718, eip1559::ETHEREUM_BLOCK_GAS_LIMIT_30M},
    network::TransactionBuilder,
    providers::{Provider, ProviderBuilder, RootProvider}
};
use alloy_primitives::{Address, TxHash};
use futures::stream::{StreamExt, iter};

use super::{
    AngstromBundle, AngstromSigner, ChainSubmitter, DEFAULT_SUBMISSION_CONCURRENCY, TxFeatureInfo,
    Url
};
use crate::{primitive::AngstromMetaSigner, submission::EXTRA_GAS_LIMIT};

/// handles submitting transaction to
pub struct MempoolSubmitter {
    clients:          Vec<(RootProvider, Url)>,
    angstrom_address: Address
}

impl MempoolSubmitter {
    pub fn new(clients: &[Url], angstrom_address: Address) -> Self {
        let clients = clients
            .iter()
            .map(|url| {
                (ProviderBuilder::<_, _, _>::default().connect_http(url.clone()), url.clone())
            })
            .collect::<Vec<_>>();
        Self { clients, angstrom_address }
    }
}

impl ChainSubmitter for MempoolSubmitter {
    fn angstrom_address(&self) -> Address {
        self.angstrom_address
    }

    fn submit<'a, S: AngstromMetaSigner>(
        &'a self,
        signer: &'a AngstromSigner<S>,
        bundle: Option<&'a AngstromBundle>,
        tx_features: &'a TxFeatureInfo
    ) -> std::pin::Pin<Box<dyn Future<Output = eyre::Result<Option<TxHash>>> + Send + 'a>> {
        Box::pin(async move {
            let bundle = bundle.ok_or_else(|| eyre::eyre!("no bundle was past in"))?;

            let client = self
                .clients
                .first()
                .ok_or(eyre::eyre!("no mempool clients found"))?
                .0
                .clone();

            let tx = self
                .build_and_sign_tx_with_gas(signer, bundle, tx_features, |tx| async move {
                    let gas = std::cmp::min(
                        client
                            .estimate_gas(tx.clone())
                            .await
                            .inspect_err(|e| {
                                tracing::error!(err=%e, "failed to query gas");
                            })
                            .unwrap_or(ETHEREUM_BLOCK_GAS_LIMIT_30M)
                            + EXTRA_GAS_LIMIT,
                        ETHEREUM_BLOCK_GAS_LIMIT_30M
                    );
                    tx.with_gas_limit(gas)
                })
                .await;

            let encoded_tx = tx.encoded_2718();
            let tx_hash = *tx.tx_hash();

            // Clone here is fine as its in a Arc
            let _: Vec<_> = iter(self.clients.clone())
                .map(async |(client, url)| {
                    client
                        .send_raw_transaction(&encoded_tx)
                        .await
                        .inspect_err(|e| {
                            tracing::warn!(url=?url, err=?e, "failed to send mempool tx");
                        })
                })
                .buffer_unordered(DEFAULT_SUBMISSION_CONCURRENCY)
                .collect::<Vec<_>>()
                .await;

            Ok(Some(tx_hash))
        })
    }
}
