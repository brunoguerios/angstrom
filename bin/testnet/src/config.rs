use std::{net::IpAddr, path::PathBuf};

use alloy_primitives::{Address, Bytes};
use angstrom_metrics::{initialize_prometheus_metrics, METRICS_ENABLED};
use angstrom_types::contract_bindings::angstrom::Angstrom::PoolKey;
use eyre::Context;
use serde::Deserialize;

#[derive(Debug, Clone, Default, clap::Parser)]
pub struct AngstromDevnetCli {
    #[clap(long)]
    pub mev_guard:    bool,
    #[clap(long)]
    pub node_config:  PathBuf,
    /// enables the metrics
    #[clap(long, default_value = "false", global = true)]
    pub metrics:      bool,
    /// spawns the prometheus metrics exporter at the specified port
    /// Default: 6969
    #[clap(long, default_value = "6969", global = true)]
    pub metrics_port: u16
}

impl AngstromDevnetCli {
    pub async fn init_metrics(self) {
        if self.metrics
            && initialize_prometheus_metrics(self.metrics_port)
                .await
                .inspect_err(|e| eprintln!("failed to start metrics endpoint - {:?}", e))
                .is_ok()
        {
            {
                METRICS_ENABLED.set(true).unwrap();
            }
        } else {
            METRICS_ENABLED.set(false).unwrap();
        }
    }

    pub(crate) fn load_config(&self) -> eyre::Result<FullTestnetNodeConfig> {
        FullTestnetNodeConfig::load_from_config(&self.node_config)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct FullTestnetNodeConfig {
    pub(crate) nodes: Vec<TestnetNodeConfig>,
    pub(crate) pools: Vec<PoolKey>
}

impl FullTestnetNodeConfig {
    fn load_from_config(config_path: &PathBuf) -> eyre::Result<Self> {
        if !config_path.exists() {
            return Err(eyre::eyre!("Config file does not exist at {:?}", config_path))
        }

        let toml_content = std::fs::read_to_string(&config_path)
            .wrap_err_with(|| format!("Could not read config file {:?}", config_path))?;

        let node_config: Self = toml::from_str(&toml_content)
            .wrap_err_with(|| format!("Could not deserialize config file {:?}", config_path))?;

        Ok(node_config)
    }

    pub(crate) fn my_node_config(&self) -> TestnetNodeConfig {
        todo!()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct TestnetNodeConfig {
    pub(crate) node_id:    usize,
    pub(crate) address:    Address,
    pub(crate) ip:         IpAddr,
    pub(crate) is_leader:  bool,
    pub(crate) secret_key: Bytes
}
