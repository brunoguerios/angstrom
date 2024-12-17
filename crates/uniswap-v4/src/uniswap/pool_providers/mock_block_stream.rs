use std::{marker::PhantomData, sync::Arc};

use alloy::{network::Network, providers::Provider, rpc::types::Filter, transports::Transport};
use alloy_primitives::Log;
use futures_util::StreamExt;

use super::PoolMangerBlocks;
use crate::uniswap::{pool_manager::PoolManagerError, pool_providers::PoolManagerProvider};

#[derive(Debug, Clone)]
pub struct MockBlockStream<P, T, N> {
    inner:      Arc<P>,
    from_block: u64,
    to_block:   u64,
    _phantom:   PhantomData<(T, N)>
}

impl<P, T, N> MockBlockStream<P, T, N>
where
    P: Provider<T, N> + 'static,
    T: Transport + Clone,
    N: Network
{
    pub fn new(inner: Arc<P>, from_block: u64, to_block: u64) -> Self {
        Self { inner, from_block, to_block, _phantom: PhantomData }
    }
}

impl<P, T, N> PoolManagerProvider for MockBlockStream<P, T, N>
where
    P: Provider<T, N> + Unpin + 'static,
    T: Transport + Clone + Unpin,
    N: Network + Unpin
{
    fn subscribe_blocks(&self) -> futures::stream::BoxStream<Option<PoolMangerBlocks>> {
        futures::stream::iter(self.from_block..=self.to_block)
            .then(|block| async move {
                // yield to sym async call
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                Some(PoolMangerBlocks::NewBlock(block))
            })
            .boxed()
    }

    fn get_logs(&self, filter: &Filter) -> Result<Vec<Log>, PoolManagerError> {
        let handle = tokio::runtime::Handle::try_current().expect("No tokio runtime found");
        let alloy_logs = tokio::task::block_in_place(|| {
            handle.block_on(async {
                self.inner
                    .get_logs(filter)
                    .await
                    .map_err(PoolManagerError::from)
            })
        })?;

        let reth_logs = alloy_logs
            .iter()
            .map(|alloy_log| Log {
                address: alloy_log.address(),
                data:    alloy_log.data().clone()
            })
            .collect();

        Ok(reth_logs)
    }
}
