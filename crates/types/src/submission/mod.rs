pub mod angstrom;
pub mod mempool;
pub mod mev_boost;

use std::{borrow::Cow, pin::Pin, sync::Arc};

use alloy::{
    providers::Provider,
    rpc::{
        client::RpcClient,
        json_rpc::{RpcRecv, RpcSend}
    }
};
use alloy_primitives::Bytes;
use reqwest::Url;

use crate::{contract_payloads::angstrom::AngstromBundle, primitive::AngstromSigner};

/// a chain submitter is a trait that deals with submitting a bundle to the
/// different configured endpoints.
pub trait ChainSubmitter<P, R>: Send + Sync + 'static
where
    P: RpcSend,
    R: RpcRecv
{
    fn client(&self) -> RpcClient;
    fn rpc_method(&self) -> Cow<'static, str>;
    fn submit<'a>(
        &'a self,
        signer: &'a AngstromSigner,
        bundle: Option<&'a AngstromBundle>
    ) -> Pin<Box<dyn Future<Output = eyre::Result<R>> + Send + 'a>> {
        Box::pin(async move {
            let payload = self.encode(signer, bundle);
            let client = self.client();
            client
                .request::<(P,), R>(self.rpc_method(), (payload,))
                .await
                .map_err(Into::into)
        })
    }
    fn encode(&self, signer: &AngstromSigner, bundle: Option<&AngstromBundle>) -> P;
}

pub struct SubmissionHandler<P, S>
where
    P: Provider + 'static,
    S: ChainSubmitter<Box<dyn RpcSend>, Box<dyn RpcRecv>>
{
    node_provider: Arc<P>,
    submitters:    Vec<S>
}

impl<P, S> SubmissionHandler<P, S>
where
    P: Provider + 'static,
    S: ChainSubmitter<Box<dyn RpcSend>, Box<dyn RpcRecv>>
{
    pub async fn submit_tx(&mut self, signer: &AngstromSigner, bundle: Option<AngstromBundle>) {}
}
