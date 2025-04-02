use alloy::primitives::Address;
use reth::tasks::TaskExecutor;
use url::Url;

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
    pub angstrom_address:     Address
}

/// the way that the bundle lander works is by more or less wash trading back
/// and forth on the sepolia testnet
impl BundleLander {
    pub async fn run(self, executor: TaskExecutor) -> eyre::Result<()> {
        Ok(())
    }
}
