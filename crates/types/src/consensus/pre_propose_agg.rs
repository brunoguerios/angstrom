use alloy::primitives::{keccak256, BlockNumber};
use bytes::Bytes;
use reth_network_peers::PeerId;
use secp256k1::SecretKey;
use serde::{Deserialize, Serialize};

use crate::{consensus::PreProposal, primitive::Signature};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PreProposalAggregation {
    pub block_height:  BlockNumber,
    pub source:        PeerId,
    pub pre_proposals: Vec<PreProposal>,
    pub signature:     Signature
}

impl PreProposalAggregation {
    pub fn new(
        block_height: BlockNumber,
        sk: &SecretKey,
        source: PeerId,
        pre_proposals: Vec<PreProposal>
    ) -> Self {
        let payload = Self::serialize_payload(&block_height, &pre_proposals);
        let signature = Self::sign_payload(sk, payload);
        Self { block_height, source, pre_proposals, signature }
    }

    fn sign_payload(sk: &SecretKey, payload: Vec<u8>) -> Signature {
        let hash = keccak256(payload);
        let sig = reth_primitives::sign_message(sk.secret_bytes().into(), hash).unwrap();
        Signature(sig)
    }

    fn serialize_payload(block_height: &BlockNumber, pre_proposals: &[PreProposal]) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(bincode::serialize(block_height).unwrap());
        buf.extend(bincode::serialize(pre_proposals).unwrap());
        buf
    }

    fn payload(&self) -> Bytes {
        Bytes::from(Self::serialize_payload(&self.block_height, &self.pre_proposals))
    }

    pub fn is_valid(&self, block_height: &BlockNumber) -> bool {
        if !self
            .pre_proposals
            .iter()
            .all(|prop| prop.is_valid(block_height))
        {
            return false
        }
        let hash = keccak256(self.payload());
        let Ok(source) = self.signature.recover_signer_full_public_key(hash) else {
            return false;
        };
        source == self.source
    }
}
