//! CLI definition and entrypoint to executable
pub mod components;
pub mod config;
use angstrom_rpc::OrderApi;
use clap::Parser;
use config::AngstromTestnetCli;
use reth::{builder::Node, tasks::TaskExecutor, CliRunner};
use reth_node_ethereum::{node::EthereumAddOns, EthereumNode};
use secp256k1::{Secp256k1, SecretKey};

pub fn run() -> eyre::Result<()> {
    CliRunner::default().run_command_until_exit(|ctx| execute(ctx.task_executor))
}

async fn execute(executor: TaskExecutor) -> eyre::Result<()> {
    let cli = AngstromTestnetCli::parse();
    executor.spawn_critical("metrics", cli.init_metrics());

    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let pub_key = secret_key.public_key(&Secp256k1::default());

    let mut network = init_network_builder(secret_key)?;
    let protocol_handle = network.build_protocol_handler();
    let channels = initialize_strom_handles();

    // for rpc
    let pool = channels.get_pool_handle();
    let executor_clone = executor.clone();
    let validation_client = ValidationClient(channels.validator_tx.clone());
    let NodeHandle { node, node_exit_future } = alloy::providers::builder()
        .with_types::<EthereumNode>()
        .with_components(
            EthereumNode::default()
                .components_builder()
                .network(AngstromNetworkBuilder::new(protocol_handle))
        )
        .with_add_ons::<EthereumAddOns>(Default::default())
        .extend_rpc_modules(move |rpc_context| {
            let order_api = OrderApi::new(pool.clone(), executor_clone, validation_client);
            rpc_context.modules.merge_configured(order_api.into_rpc())?;

            Ok(())
        })
        .launch()
        .await?;

    initialize_strom_components(
        args.angstrom_addr,
        args,
        secret_key,
        channels,
        network,
        node,
        &executor
    )
    .await;

    Ok(())
}
