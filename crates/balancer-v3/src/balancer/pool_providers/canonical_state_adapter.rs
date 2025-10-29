use std::sync::Arc;

use futures_util::stream::BoxStream;
use reth_provider::{CanonStateNotification, CanonStateNotifications};
use tokio::sync::broadcast;

use super::{PoolManagerBlocks, PoolManagerProvider};

pub struct CanonicalStateAdapter<P> {
    canon_state_notifications: broadcast::Receiver<CanonStateNotification>,
    _provider: Arc<P>,
    _tip: u64
}

impl<P: Send + Sync + 'static> CanonicalStateAdapter<P> {
    pub fn new(inner: CanonStateNotifications, provider: Arc<P>, tip: u64) -> Self {
        Self { canon_state_notifications: inner, _provider: provider, _tip: tip }
    }
}

impl<P: Send + Sync + 'static> Clone for CanonicalStateAdapter<P> {
    fn clone(&self) -> Self {
        Self {
            canon_state_notifications: self.canon_state_notifications.resubscribe(),
            _provider: self._provider.clone(),
            _tip: self._tip
        }
    }
}

impl<P: Send + Sync + 'static> PoolManagerProvider for CanonicalStateAdapter<P> {
    fn subscribe_blocks(&self) -> BoxStream<'static, Option<PoolManagerBlocks>> {
        let mut sub = self.canon_state_notifications.resubscribe();
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
}
