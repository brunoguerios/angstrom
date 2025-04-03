use std::{pin::pin, time::Duration};

use alloy::{primitives::Address, providers::Provider};
use angstrom_types::primitive::ANGSTROM_DOMAIN;
use futures::StreamExt;
use jsonrpsee::http_client::HttpClientBuilder;
use reth::tasks::TaskExecutor;
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};
use url::Url;

use crate::{env::BundleWashTraderEnv, intent_builder::PoolIntentBundler};

#[derive(Debug, Clone, clap::Parser)]
pub struct BundleLander {
    /// angstrom endpoint
    #[clap(short, long)]
    pub node_endpoint:        Url,
    /// keys to trade with
    #[clap(short, long)]
    pub testing_private_keys: Vec<String>,
    /// address of angstrom
    #[clap(short, long)]
    pub angstrom_address:     Address,
    #[clap(short, long)]
    pub pool_manager_address: Address
}

/// the way that the bundle lander works is by more or less wash trading back
/// and forth on the sepolia testnet
impl BundleLander {
    pub async fn run(self, executor: TaskExecutor) -> eyre::Result<()> {
        init_tracing();
        let domain = ANGSTROM_DOMAIN;
        tracing::info!(?domain);

        let env = BundleWashTraderEnv::init(&self).await?;

        executor
            .spawn_critical("order placer", async move {
                let BundleWashTraderEnv { keys, provider, pools } = env;

                let subscription = provider
                    .clone()
                    .watch_blocks()
                    .await
                    .unwrap()
                    .with_poll_interval(Duration::from_millis(50))
                    .into_stream()
                    .flat_map(futures::stream::iter)
                    .then(|hash| {
                        let provider_c = provider.clone();
                        async move {
                            provider_c
                                .get_block_by_hash(hash)
                                .await
                                .map(|block_o| block_o.map(|block| block.header.number))
                        }
                    });

                let http_client = HttpClientBuilder::new().build(self.node_endpoint).unwrap();

                let mut pinned = pin!(subscription);
                let Some(Ok(Some(current_block))) = pinned.next().await else {
                    tracing::error!("couldn't fetch next block");
                    return;
                };
                let mut processors = pools
                    .into_iter()
                    .map(|pool| {
                        PoolIntentBundler::new(
                            pool,
                            current_block,
                            &keys,
                            provider.clone(),
                            &http_client
                        )
                    })
                    .collect::<Vec<_>>();

                loop {
                    let new_block = pinned.next().await;
                    match new_block {
                        Some(Ok(Some(block))) => {
                            futures::stream::iter(&mut processors)
                                .for_each(|processor| async {
                                    processor
                                        .new_block(block)
                                        .await
                                        .expect("failed to process new block in pool");
                                    processor
                                        .submit_new_orders_to_angstrom()
                                        .await
                                        .expect("failed to send angstrom orders");
                                })
                                .await;
                        }
                        _ => {
                            tracing::error!("failed to get new block number");
                            break;
                        }
                    }
                }
            })
            .await?;

        Ok(())
    }
}

pub fn init_tracing() {
    let level = Level::INFO;

    let envfilter = filter::EnvFilter::builder().try_from_env().ok();
    let format = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .with_target(true);

    if let Some(f) = envfilter {
        let _ = tracing_subscriber::registry()
            .with(format)
            .with(f)
            .try_init();
    } else {
        let filter = filter::Targets::new()
            .with_target("sepolia_bundle_lander", level)
            .with_target("testnet", level)
            .with_target("devnet", level)
            .with_target("angstrom_rpc", level)
            .with_target("angstrom", level)
            .with_target("testing_tools", level)
            .with_target("angstrom_eth", level)
            .with_target("matching_engine", level)
            .with_target("uniswap_v4", level)
            .with_target("consensus", level)
            .with_target("validation", level)
            .with_target("order_pool", level);
        let _ = tracing_subscriber::registry()
            .with(format)
            .with(filter)
            .try_init();
    }
}
