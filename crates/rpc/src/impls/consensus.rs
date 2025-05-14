use std::{collections::HashSet, pin::Pin};

use alloy_primitives::{Address, Bytes};
use consensus::{AngstromValidator, ConsensusHandle};
use jsonrpsee::{PendingSubscriptionSink, SubscriptionMessage, core::RpcResult};
use reth_tasks::TaskSpawner;

use crate::api::ConsensusApiServer;

pub struct ConsensusApi<Consensus, Spawner> {
    consensus:    Consensus,
    task_spawner: Spawner
}

impl<Consensus, Spawner> ConsensusApi<Consensus, Spawner> {
    pub fn new(consensus: Consensus, task_spawner: Spawner) -> Self {
        Self { consensus, task_spawner }
    }
}

#[async_trait::async_trait]
impl<Consensus, Spawner> ConsensusApiServer for ConsensusApi<Consensus, Spawner>
where
    Consensus: ConsensusHandle,
    Spawner: TaskSpawner + 'static
{
    async fn get_current_leader(&self) -> RpcResult<Address> {
        todo!()
    }

    #[method(name = "fetchConsensusState")]
    async fn fetch_consensus_state(&self) -> RpcResult<HashSet<AngstromValidator>> {
        todo!()
    }

    async fn subscribe_empty_block_attestations(
        &self,
        pending: PendingSubscriptionSink
    ) -> jsonrpsee::core::SubscriptionResult {
        let sink = pending.accept().await?;
        let mut subscription = self
            .pool
            .subscribe_orders()
            .map(move |update| update.map(|value| value.filter_out_order(&kind, &filter)));

        self.task_spawner.spawn(Box::pin(async move {
            while let Some(Ok(order)) = subscription.next().await {
                if sink.is_closed() {
                    break;
                }

                if let Some(result) = order {
                    match SubscriptionMessage::from_json(&result) {
                        Ok(message) => {
                            if sink.send(message).await.is_err() {
                                break;
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to serialize subscription message: {:?}", e);
                        }
                    }
                }
            }
        }));

        Ok(())
    }
}
