use clap::Parser;

use super::testnet::TestnetCli;

#[derive(Debug, Clone, Parser)]
pub struct CompareEnginesCli {
    #[clap(flatten)]
    pub testnet_config: TestnetCli
}
