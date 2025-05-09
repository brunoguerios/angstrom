use std::fmt::Debug;

use alloy::{
    eips::Encodable2718,
    network::TransactionBuilder,
    primitives::Bytes,
    rpc::client::RpcClient,
    signers::{Signer, SignerSync},
    sol_types::{SolCall, SolStruct}
};
use alloy_primitives::{Address, TxHash};
use pade::PadeEncode;
use serde::{Deserialize, Serialize};

use super::{AngstromBundle, AngstromSigner, ChainSubmitter, TxFeatureInfo};
use crate::{
    contract_bindings::angstrom::Angstrom::unlockWithEmptyAttestationCall,
    primitive::ANGSTROM_DOMAIN, sol_bindings::rpc_orders::AttestAngstromBlockEmpty
};

pub struct AngstromSubmitter {
    clients:          Vec<RpcClient>,
    angstrom_address: Address
}

impl ChainSubmitter for AngstromSubmitter {
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
            let payload = if let Some(bundle) = bundle {
                let tx = self.build_tx(signer, bundle, tx_features);
                let gas = tx.max_priority_fee_per_gas.unwrap();
                // TODO: manipulate gas before signing based of off defined rebate spec.

                let signed_tx = tx.build(signer).await.unwrap();
                let tx_payload = Bytes::from(signed_tx.encoded_2718());
                AngstromIntegrationSubmission {
                    tx: tx_payload,
                    unlock_data: Bytes::new(),
                    max_priority_fee_per_gas: gas
                }
            } else {
                let attestation =
                    AttestAngstromBlockEmpty { block_number: tx_features.target_block };

                let hash = attestation.eip712_signing_hash(&ANGSTROM_DOMAIN);
                // we pade encode here as we expect v, r, s which is not the standard currently
                let sig = signer.sign_hash_sync(&hash)?.pade_encode();
                let signer = signer.address();

                let unlock_data = Bytes::from(
                    unlockWithEmptyAttestationCall::new((signer, sig.into())).abi_encode()
                );
                AngstromIntegrationSubmission {
                    tx: Bytes::new(),
                    unlock_data,
                    max_priority_fee_per_gas: 0
                }
            };

            let mut tx_hash = TxHash::default();
            for client in &self.clients {
                if let Some(this_tx_hash) = client
                    .request::<(&AngstromIntegrationSubmission,), Option<TxHash>>(
                        "angstrom_submitBundle",
                        (&payload,)
                    )
                    .await?
                {
                    tx_hash = this_tx_hash;
                }
            }

            Ok(tx_hash)
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AngstromIntegrationSubmission {
    pub tx: Bytes,
    pub unlock_data: Bytes,
    pub max_priority_fee_per_gas: u128
}
