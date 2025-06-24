use angstrom_types::primitive::AngstromAddressConfig;
use clap::Parser;
use replay::{ReplayCli, init_tracing};
use reth_provider::noop::NoopProvider;
use telemetry::{blocklog::BlockLog, outputs::s3::S3Storage};
use testing_tools::{controllers::enviroments::AngstromTestnet, types::config::ReplayConfig};

fn main() -> eyre::Result<()> {
    init_tracing(3);
    reth::CliRunner::try_default_runtime()
        .unwrap()
        .run_command_until_exit(async move |ctx| {
            let cli = ReplayCli::parse();
            let aws = S3Storage::new().await.unwrap();
            let snapshot = aws.retrieve_snapshot(&cli.id, cli.is_error).await?;

            Ok(())
        })
}
