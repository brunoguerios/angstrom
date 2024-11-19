use alloy::primitives::{keccak256, BlockNumber};
use bytes::Bytes;
use reth_network_peers::PeerId;
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

    fn payload(&self) -> Bytes {
        let mut buf = vec![];
        buf.extend(bincode::serialize(&self.block_height).unwrap());
        buf.extend(*self.source);
        buf.extend(bincode::serialize(&self.pre_proposals).unwrap());

        Bytes::from_iter(buf)
    }
}
