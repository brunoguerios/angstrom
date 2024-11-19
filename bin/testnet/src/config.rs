use std::{net::IpAddr, path::PathBuf, str::FromStr};

use alloy::signers::local::PrivateKeySigner;
use alloy_primitives::{Address, Bytes};
use angstrom_metrics::{initialize_prometheus_metrics, METRICS_ENABLED};
use angstrom_types::contract_bindings::angstrom::Angstrom::PoolKey;
use enr::k256::ecdsa::SigningKey;
use eyre::Context;
use secp256k1::SecretKey;
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

    pub fn load_config(&self) -> eyre::Result<FullTestnetNodeConfig> {
        FullTestnetNodeConfig::load_from_config(&self.node_config)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FullTestnetNodeConfig {
    pub nodes:  Vec<TestnetNodeConfig>,
    pub leader: LeaderNodeConfig,
    pub pools:  Vec<PoolKey>
}

impl FullTestnetNodeConfig {
    fn load_from_config(config_path: &PathBuf) -> eyre::Result<Self> {
        FullTestnetNodeConfigInner::load_from_config(config_path)?.try_into()
    }

    pub fn my_node_config(&self) -> eyre::Result<TestnetNodeConfig> {
        let my_ip = local_ip_address::local_ip()?;

        self.nodes
            .iter()
            .find(|node| node.ip == my_ip)
            .cloned()
            .ok_or(eyre::eyre!("no node found for IP: {my_ip:?}"))
    }

    pub fn leader_node_config(&self) -> eyre::Result<TestnetNodeConfig> {
        self.nodes
            .iter()
            .find(|node| node.is_leader)
            .cloned()
            .ok_or(eyre::eyre!("no leader node found"))
    }
}

impl TryFrom<FullTestnetNodeConfigInner> for FullTestnetNodeConfig {
    type Error = eyre::ErrReport;

    fn try_from(value: FullTestnetNodeConfigInner) -> Result<Self, Self::Error> {
        Ok(FullTestnetNodeConfig {
            nodes:  value
                .nodes
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            pools:  value.pools,
            leader: value.leader
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TestnetNodeConfig {
    pub node_id:     usize,
    pub address:     Address,
    pub ip:          IpAddr,
    pub is_leader:   bool,
    pub signing_key: PrivateKeySigner,
    pub secret_key:  SecretKey
}

impl TryFrom<TestnetNodeConfigInner> for TestnetNodeConfig {
    type Error = eyre::ErrReport;

    fn try_from(value: TestnetNodeConfigInner) -> Result<Self, Self::Error> {
        let ip = IpAddr::from_str(&value.ip)?;
        let signing_key = PrivateKeySigner::from_signing_key(SigningKey::from_slice(
            &Bytes::from_str(&value.signing_key)?.0.to_vec()
        )?);
        let address = signing_key.address();
        let secret_key = SecretKey::from_slice(&Bytes::from_str(&value.secret_key)?.0.to_vec())?;

        Ok(TestnetNodeConfig {
            node_id: value.node_id,
            address,
            ip,
            is_leader: value.is_leader,
            signing_key,
            secret_key
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
struct FullTestnetNodeConfigInner {
    nodes:  Vec<TestnetNodeConfigInner>,
    pools:  Vec<PoolKey>,
    leader: LeaderNodeConfig
}

impl FullTestnetNodeConfigInner {
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
}

#[derive(Debug, Clone, Deserialize)]
struct TestnetNodeConfigInner {
    node_id:     usize,
    ip:          String,
    is_leader:   bool,
    signing_key: String,
    secret_key:  String
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct LeaderNodeConfig {
    ip_or_domain: String,
    port:         u64
}

impl LeaderNodeConfig {
    pub(crate) fn ws_url(&self) -> String {
        format!("ws://{}:{}", self.ip_or_domain, self.port)
    }
}
