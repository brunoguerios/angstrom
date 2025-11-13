use std::sync::Arc;

use alloy::{providers::Provider, rpc::types::eth::Filter};
use alloy_primitives::Log;
use futures_util::stream::BoxStream;
use reth_provider::{CanonStateNotification, CanonStateNotifications};
use tokio::sync::broadcast;

use super::{PoolManagerBlocks, PoolManagerProvider};
use crate::balancer::pool_manager::PoolManagerError;

pub struct CanonicalStateAdapter<P> {
    canon_state_notifications: broadcast::Receiver<CanonStateNotification>,
    provider: Arc<P>,
    _tip: u64
}

impl<P: Send + Sync + 'static> CanonicalStateAdapter<P> {
    pub fn new(inner: CanonStateNotifications, provider: Arc<P>, tip: u64) -> Self {
        Self { canon_state_notifications: inner, provider, _tip: tip }
    }
}

impl<P: Send + Sync + 'static> Clone for CanonicalStateAdapter<P> {
    fn clone(&self) -> Self {
        Self {
            canon_state_notifications: self.canon_state_notifications.resubscribe(),
            provider: self.provider.clone(),
            _tip: self._tip
        }
    }
}

impl<P> PoolManagerProvider for CanonicalStateAdapter<P>
where
    P: Provider + Send + Sync + 'static
{
    fn subscribe_blocks(self) -> BoxStream<'static, Option<PoolManagerBlocks>> {
        let mut sub = self.canon_state_notifications;
        Box::pin(async_stream::stream! {
            while let Ok(notification) = sub.recv().await {
                match notification {
                    CanonStateNotification::Commit { new } => {
                        yield Some(PoolManagerBlocks::NewBlock(new.tip().number))
                    }
                    CanonStateNotification::Reorg { new, old } => {
                        let tip = new.tip().number;
                        let rng = old.tip().number..=tip;
                        yield Some(PoolManagerBlocks::Reorg(tip, rng));
                    }
                }
            }
        })
    }

    fn get_logs(&self, _filter: &Filter) -> Result<Vec<Log>, PoolManagerError> {
        // TODO: Implement log fetching when needed for event decoding
        Ok(Vec::new())
    }

    fn provider(&self) -> Arc<impl Provider> {
        self.provider.clone()
    }
}
