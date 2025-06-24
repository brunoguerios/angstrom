use angstrom_types::primitive::AngstromAddressConfig;
use clap::Parser;
use replay::{ReplayCli, init_tracing};
use reth_provider::noop::NoopProvider;
use telemetry::blocklog::BlockLog;
use testing_tools::{controllers::enviroments::AngstromTestnet, types::config::ReplayConfig};

fn main() {
    init_tracing(2);

    let cli = ReplayCli::parse();

    if cli.testnet_replay {
        // Setup the testnet addresses if we are doing a testnet run
        AngstromAddressConfig::INTERNAL_TESTNET.try_init();
    }
    let raw_log = read_log_from_stdin();
    let target_block = raw_log.blocknum().saturating_sub(71);
    tracing::info!(target_block, log_block = raw_log.blocknum(), "Loaded and registered log");
    let initial_state = cli.load_pool_keys().unwrap();
    let config = ReplayConfig::new(
        initial_state,
        cli.testnet_replay,
        cli.eth_fork_url.clone(),
        target_block
    );
    // let node_config = TestingNodeConfig::new(1, config.clone(), 10);
    reth::CliRunner::try_default_runtime()
        .unwrap()
        .run_command_until_exit(async move |ctx| -> eyre::Result<()> {
            let (net, state_rx) = AngstromTestnet::spawn_replay(
                NoopProvider::mainnet(),
                config,
                ctx.task_executor.clone(),
                raw_log.gas_price_snapshot().cloned()
            )
            .await?;
            net.replay_stuff(ctx.task_executor, raw_log, state_rx)
                .await?;
            Ok(())
        })
        .unwrap();
}
