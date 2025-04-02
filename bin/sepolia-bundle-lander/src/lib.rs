pub mod cli;
pub mod env;
use clap::Parser;

#[inline]
pub fn run() -> eyre::Result<()> {
    let args = cli::BundleLander::parse();
    reth::CliRunner::try_default_runtime()
        .unwrap()
        .run_command_until_exit(|ctx| args.run(ctx.task_executor))
}
