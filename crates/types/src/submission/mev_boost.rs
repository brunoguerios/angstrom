use std::{
    fmt::Debug,
    sync::Arc,
    task::{Context, Poll}
};

use alloy::{
    eips::Encodable2718,
    hex,
    network::TransactionBuilder,
    primitives::keccak256,
    rpc::{
        client::{ClientBuilder, RpcClient},
        json_rpc::{RequestPacket, ResponsePacket},
        types::TransactionRequest
    },
    signers::{Signer, local::PrivateKeySigner},
    sol_types::SolCall,
    transports::{TransportError, TransportErrorKind, TransportFut}
};
use alloy_primitives::{Address, TxHash};
use itertools::Itertools;
use pade::PadeEncode;
use reth::rpc::types::mev::PrivateTransactionRequest;

use super::{AngstromBundle, AngstromSigner, ChainSubmitter, TxFeatureInfo, Url};

pub struct MevBoostSubmitter {
    clients:          Vec<RpcClient>,
    angstrom_address: Address
}

impl MevBoostSubmitter {
    pub fn new(urls: Vec<Url>, angstrom_address: Address) -> Self {
        let clients = urls
            .into_iter()
            .map(|url| {
                let transport = MevHttp::new_flashbots(url.clone());
                ClientBuilder::default().transport(transport, false)
            })
            .collect_vec();

        Self { clients, angstrom_address }
    }
}

impl ChainSubmitter for MevBoostSubmitter {
    fn angstrom_address(&self) -> Address {
        self.angstrom_address
    }

    fn submit<'a>(
        &'a self,
        signer: &'a AngstromSigner,
        bundle: Option<&'a AngstromBundle>,
        tx_features: &'a TxFeatureInfo
    ) -> std::pin::Pin<Box<dyn Future<Output = eyre::Result<TxHash>> + Send + 'a>> {
        Box::pin(async move {
            let Some(bundle) = bundle else { return Err(eyre::eyre!("no bundle was past in")) };
            let tx = self.build_and_sign_tx(signer, bundle, tx_features).await;
            let hash = *tx.tx_hash();

            let private_tx = PrivateTransactionRequest::new(&tx)
                .max_block_number(tx_features.target_block)
                .with_preferences(
                    reth::rpc::types::mev::PrivateTransactionPreferences::default().into_fast()
                );

            for client in &self.clients {
                client
                    .request::<(&PrivateTransactionRequest,), TxHash>(
                        "eth_sendPrivateTransaction",
                        (&private_tx,)
                    )
                    .await?;
            }

            Ok(hash)
        })
    }
}

/// A [`Signer`] wrapper to sign bundles.
#[derive(Clone)]
pub struct BundleSigner {
    /// The header name on which set the signature.
    pub header: String,
    /// The signer used to sign the bundle.
    pub signer: Arc<dyn Signer + Send + Sync>
}

impl BundleSigner {
    /// Creates a new [`BundleSigner`]
    pub fn new<S>(header: String, signer: S) -> Self
    where
        S: Signer + Send + Sync + 'static
    {
        Self { header, signer: Arc::new(signer) }
    }

    /// Creates a [`BundleSigner`] set up to add the Flashbots header.
    pub fn flashbots<S>(signer: S) -> Self
    where
        S: Signer + Send + Sync + 'static
    {
        Self { header: "X-Flashbots-Signature".to_string(), signer: Arc::new(signer) }
    }

    /// Returns the signer address.
    pub fn address(&self) -> Address {
        self.signer.address()
    }
}

impl Debug for BundleSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BundleSigner")
            .field("header", &self.header)
            .field("signer_address", &self.signer.address())
            .finish()
    }
}

#[derive(Clone)]
struct MevHttp {
    endpoint: Url,
    signer:   BundleSigner,
    http:     reqwest::Client
}

impl MevHttp {
    pub fn new_flashbots(endpoint: Url) -> Self {
        let bundle_signer = PrivateKeySigner::random();
        let signer = BundleSigner::flashbots(bundle_signer);
        Self { signer, endpoint, http: reqwest::Client::new() }
    }

    fn send_request(&self, req: RequestPacket) -> TransportFut<'static> {
        let this = self.clone();

        Box::pin(async move {
            let body = serde_json::to_vec(&req).map_err(TransportError::ser_err)?;

            let signature = this
                .signer
                .signer
                .sign_message(format!("{:?}", keccak256(&body)).as_bytes())
                .await
                .map_err(TransportErrorKind::custom)?;

            this.http
                .post(this.endpoint)
                .header(
                    &this.signer.header,
                    format!("{:?}:0x{}", this.signer.address(), hex::encode(signature.as_bytes()))
                )
                .body(body)
                .send()
                .await
                .map_err(TransportErrorKind::custom)?
                .json()
                .await
                .map_err(TransportErrorKind::custom)
        })
    }
}

use tower::Service;
impl Service<RequestPacket> for MevHttp {
    type Error = TransportError;
    type Future = TransportFut<'static>;
    type Response = ResponsePacket;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn call(&mut self, req: RequestPacket) -> Self::Future {
        self.send_request(req)
    }
}
