use std::path::PathBuf;

use alloy::signers::local::PrivateKeySigner;
use alloy_primitives::Address;
use angstrom_metrics::initialize_prometheus_metrics;
use angstrom_types::primitive::AngstromSigner;
use eyre::Context;
use hsm_signer::{Pkcs11Signer, Pkcs11SignerConfig};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Default, clap::Args)]
pub struct AngstromConfig {
    #[clap(long)]
    pub node_config:               PathBuf,
    /// enables the metrics
    #[clap(long, default_value = "false", global = true)]
    pub metrics_enabled:           bool,
    /// spawns the prometheus metrics exporter at the specified port
    /// Default: 6969
    #[clap(long, default_value = "6969", global = true)]
    pub metrics_port:              u16,
    #[clap(short, long)]
    pub mev_boost_endpoints:       Vec<Url>,
    /// needed to properly setup the node as we need some chain state before
    /// starting the internal reth node
    #[clap(short, long, default_value = "https://eth.drpc.org")]
    pub boot_node:                 String,
    #[clap(short, long)]
    pub normal_nodes:              Vec<Url>,
    #[clap(short, long)]
    pub angstrom_submission_nodes: Vec<Url>,
    #[clap(flatten)]
    pub key_config:                KeyConfig
}

impl AngstromConfig {
    pub fn get_local_signer(&self) -> eyre::Result<Option<AngstromSigner<PrivateKeySigner>>> {
        self.key_config
            .local_secret_key_location
            .as_ref()
            .map(|sk_path| {
                let exists = sk_path.try_exists();

                match exists {
                    Ok(true) => {
                        let contents = std::fs::read_to_string(sk_path)?;
                        Ok(AngstromSigner::new(contents.trim().parse::<PrivateKeySigner>()?))
                    }
                    _ => Err(eyre::eyre!("no secret_key was found at {:?}", sk_path))
                }
            })
            .transpose()
    }

    pub fn get_hsm_signer(&self) -> eyre::Result<Option<AngstromSigner<Pkcs11Signer>>> {
        Ok((self.key_config.hsm_enabled)
            .then(|| {
                Pkcs11Signer::new(
                    Pkcs11SignerConfig::from_env_with_defaults(
                        self.key_config.hsm_public_key_label.as_ref().unwrap(),
                        self.key_config.hsm_private_key_label.as_ref().unwrap(),
                        self.key_config.pkcs11_lib_path.clone().into(),
                        None
                    ),
                    Some(1u64)
                )
                .map(AngstromSigner::new)
            })
            .transpose()?)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct NodeConfig {
    pub angstrom_address:      Address,
    pub periphery_address:     Address,
    pub pool_manager_address:  Address,
    pub gas_token_address:     Address,
    pub angstrom_deploy_block: u64
}

impl NodeConfig {
    pub fn load_from_config(config: Option<PathBuf>) -> Result<Self, eyre::Report> {
        let config_path = config.ok_or_else(|| eyre::eyre!("Config path not provided"))?;

        if !config_path.exists() {
            return Err(eyre::eyre!("Config file does not exist at {:?}", config_path));
        }

        let toml_content = std::fs::read_to_string(&config_path)
            .wrap_err_with(|| format!("Could not read config file {:?}", config_path))?;

        let node_config: NodeConfig = toml::from_str(&toml_content)
            .wrap_err_with(|| format!("Could not deserialize config file {:?}", config_path))?;

        Ok(node_config)
    }
}

pub async fn init_metrics(metrics_port: u16) {
    let _ = initialize_prometheus_metrics(metrics_port)
        .await
        .inspect_err(|e| eprintln!("failed to start metrics endpoint - {:?}", e));
}

#[derive(Debug, Clone, Default, clap::Args)]
pub struct KeyConfig {
    #[clap(long, conflicts_with = "hsm_enabled")]
    pub local_secret_key_location: Option<PathBuf>,
    #[clap(short, long, conflicts_with = "local_secret_key_location")]
    pub hsm_enabled:               bool,
    #[clap(long, requires = "hsm_enabled")]
    pub hsm_public_key_label:      Option<String>,
    #[clap(long, requires = "hsm_enabled")]
    pub hsm_private_key_label:     Option<String>,
    #[clap(
        long,
        requires = "hsm_enabled",
        default_value = "/opt/cloudhsm/lib/libcloudhsm_pkcs11.so"
    )]
    pub pkcs11_lib_path:           String
}
