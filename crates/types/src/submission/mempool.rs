use alloy::{
    eips::Encodable2718,
    hex,
    network::TransactionBuilder,
    primitives::keccak256,
    providers::Provider,
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
use pade::PadeEncode;
use reth::rpc::builder::constants::gas_oracle::RPC_DEFAULT_GAS_CAP;

use super::{AngstromBundle, AngstromSigner, ChainSubmitter, EXTRA_GAS, TxFeatureInfo};
use crate::contract_bindings::angstrom::Angstrom;

/// handles submitting transaction to
pub struct MempoolSubmitter<P> {
    clients:          Vec<P>,
    angstrom_address: Address
}

impl<P> MempoolSubmitter<P> {
    pub fn new(clients: Vec<P>, angstrom_address: Address) -> Self {
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
    ) -> std::pin::Pin<Box<dyn Future<Output = eyre::Result<TxHash>> + Send + 'a>> {
        Box::pin(async move {
            let Some(bundle) = bundle else { return Err(eyre::eyre!("no bundle was past in")) };

            let tx = self.build_and_sign_tx(signer, bundle, tx_features).await;
            let encoded_tx = tx.encoded_2718();
            let tx_hash = *tx.tx_hash();

            for client in &self.clients {
                let _ = client.send_raw_transaction(&encoded_tx).await?;
            }

            Ok(tx_hash)
        })
    }
}
