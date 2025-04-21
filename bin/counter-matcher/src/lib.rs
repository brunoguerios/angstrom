use std::collections::HashSet;

use angstrom_rpc::{
    api::OrderApiClient,
    types::{OrderSubscriptionFilter, OrderSubscriptionKind}
};
use angstrom_types::primitive::{ANGSTROM_DOMAIN, TESTNET_ANGSTROM_ADDRESS};
use clap::Parser;
use futures::TryStreamExt;
use jsonrpsee::{
    http_client::HttpClientBuilder,
    ws_client::{PingConfig, WsClientBuilder}
};
use reth::tasks::TaskExecutor;
use sepolia_bundle_lander::{
    cli::{BundleLander, JsonPKs},
    env::BundleWashTraderEnv
};
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

pub mod accounting;
pub mod order_manager;

#[inline]
pub fn run() -> eyre::Result<()> {
    let args = BundleLander::parse();
    reth::CliRunner::try_default_runtime()
        .unwrap()
        .run_command_until_exit(|ctx| start(args, ctx.task_executor))
}

pub async fn start(cfg: BundleLander, executor: TaskExecutor) -> eyre::Result<()> {
    init_tracing();
    let domain = ANGSTROM_DOMAIN;
    tracing::info!(?domain);

    let keys: JsonPKs = serde_json::from_str(&std::fs::read_to_string(&cfg.testing_private_keys)?)?;
    let env = BundleWashTraderEnv::init(&cfg, keys).await?;

    let ws = WsClientBuilder::new()
        .enable_ws_ping(PingConfig::default())
        .build(cfg.node_endpoint.as_str())
        .await
        .expect("node endpoint must be WS");

    let mut filters = HashSet::new();
    let mut subscriptions = HashSet::new();

    filters.insert(OrderSubscriptionFilter::None);
    subscriptions.insert(OrderSubscriptionKind::NewOrders);
    subscriptions.insert(OrderSubscriptionKind::FilledOrders);
    subscriptions.insert(OrderSubscriptionKind::CancelledOrders);
    subscriptions.insert(OrderSubscriptionKind::ExpiredOrders);

    let sub = ws
        .subscribe_orders(subscriptions, filters)
        .await
        .unwrap()
        .into_stream();

    Ok(())
}

fn init_tracing() {
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
