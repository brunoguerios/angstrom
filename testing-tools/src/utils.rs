use std::{path::Path, sync::Arc};

use reth_chainspec::MAINNET;
use reth_db::DatabaseEnv;
use reth_node_ethereum::EthereumNode;
use reth_node_types::NodeTypesWithDBAdapter;
use reth_provider::providers::{BlockchainProvider, ReadOnlyConfig};

pub type Provider = BlockchainProvider<NodeTypesWithDBAdapter<EthereumNode, Arc<DatabaseEnv>>>;

pub fn load_reth_db(db_path: &Path) -> Provider {
    let factory = EthereumNode::provider_factory_builder()
        .open_read_only(MAINNET.clone(), ReadOnlyConfig::from_datadir(db_path))
        .unwrap();
    BlockchainProvider::new(factory).unwrap()
}

pub fn workspace_dir() -> std::path::PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = std::path::Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}
