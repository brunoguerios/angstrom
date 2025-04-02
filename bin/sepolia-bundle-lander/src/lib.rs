pub mod cli;
use eyre::Result;

#[inline]
pub fn run() -> eyre::Result<()> {
    reth::CliRunner::try_default_runtime()
        .unwrap()
        .run_command_until_exit(|ctx| cli::AngstromTestnetCli::run_all(ctx.task_executor))
}
