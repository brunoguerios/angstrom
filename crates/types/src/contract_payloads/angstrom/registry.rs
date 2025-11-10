//! Pool configuration registry types

use std::{collections::HashMap, ops::Deref, sync::Arc};

use alloy::{
    eips::BlockId,
    network::Network,
    primitives::{Address, U256, keccak256},
    providers::Provider,
    sol_types::SolValue
};
use base64::{Engine, prelude::BASE64_STANDARD};
use dashmap::DashMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    contract_bindings::angstrom::Angstrom::PoolKey,
    contract_payloads::{CONFIG_STORE_SLOT, POOL_CONFIG_STORE_ENTRY_SIZE},
    primitive::{PoolId, UniswapPoolRegistry}
};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct AngstromPoolPartialKey([u8; 27]);

impl AngstromPoolPartialKey {
    pub fn new(key: [u8; 27]) -> Self {
        Self(key)
    }
}

impl Deref for AngstromPoolPartialKey {
    type Target = [u8; 27];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for AngstromPoolPartialKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BASE64_STANDARD.encode(self.0))
    }
}

impl std::str::FromStr for AngstromPoolPartialKey {
    type Err = base64::DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = BASE64_STANDARD.decode(s)?;
        if bytes.len() != 27 {
            return Err(base64::DecodeError::InvalidLength(bytes.len()));
        }
        let mut arr = [0u8; 27];
        arr.copy_from_slice(&bytes);
        Ok(Self(arr))
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct AngPoolConfigEntry {
    pub pool_partial_key: AngstromPoolPartialKey,
    pub tick_spacing:     u16,
    pub fee_in_e6:        u32,
    pub store_index:      usize
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AngstromPoolConfigStore {
    #[serde(
        serialize_with = "serialize_dashmap_with_display_keys",
        deserialize_with = "deserialize_dashmap_with_display_keys"
    )]
    entries: DashMap<AngstromPoolPartialKey, AngPoolConfigEntry>
}

impl From<DashMap<AngstromPoolPartialKey, AngPoolConfigEntry>> for AngstromPoolConfigStore {
    fn from(value: DashMap<AngstromPoolPartialKey, AngPoolConfigEntry>) -> Self {
        Self { entries: value }
    }
}

fn serialize_dashmap_with_display_keys<S>(
    dashmap: &DashMap<AngstromPoolPartialKey, AngPoolConfigEntry>,
    serializer: S
) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let hashmap: HashMap<String, AngPoolConfigEntry> = dashmap
        .iter()
        .map(|entry| (entry.key().to_string(), *entry.value()))
        .collect();
    hashmap.serialize(serializer)
}

fn deserialize_dashmap_with_display_keys<'de, D>(
    deserializer: D
) -> Result<DashMap<AngstromPoolPartialKey, AngPoolConfigEntry>, D::Error>
where
    D: Deserializer<'de>
{
    let hashmap: HashMap<String, AngPoolConfigEntry> = HashMap::deserialize(deserializer)?;
    let dashmap = DashMap::new();
    for (key_str, value) in hashmap {
        let key = key_str
            .parse::<AngstromPoolPartialKey>()
            .map_err(serde::de::Error::custom)?;
        dashmap.insert(key, value);
    }
    Ok(dashmap)
}

impl AngstromPoolConfigStore {
    pub async fn load_from_chain<N, P>(
        angstrom_contract: Address,
        block_id: BlockId,
        provider: &P
    ) -> Result<AngstromPoolConfigStore, String>
    where
        N: Network,
        P: Provider<N>
    {
        // offset of 6 bytes
        let value = provider
            .get_storage_at(angstrom_contract, U256::from(CONFIG_STORE_SLOT))
            .block_id(block_id)
            .await
            .map_err(|e| format!("Error getting storage: {e}"))?;

        let value_bytes: [u8; 32] = value.to_be_bytes();
        tracing::debug!("storage slot of poolkey storage {:?}", value_bytes);
        let config_store_address =
            Address::from(<[u8; 20]>::try_from(&value_bytes[4..24]).unwrap());
        tracing::info!(?config_store_address);

        let code = provider
            .get_code_at(config_store_address)
            .block_id(block_id)
            .await
            .map_err(|e| format!("Error getting code: {e}"))?;

        tracing::info!(len=?code.len(), "bytecode: {:x}", code);

        AngstromPoolConfigStore::try_from(code.as_ref())
            .map_err(|e| format!("Failed to deserialize code into AngstromPoolConfigStore: {e}"))
    }

    pub fn length(&self) -> usize {
        self.entries.len()
    }

    pub fn remove_pair(&self, asset0: Address, asset1: Address) {
        let key = Self::derive_store_key(asset0, asset1);

        let Some((_, entry)) = self.entries.remove(&key) else { return };
        let index = entry.store_index;

        // if we have any indexes that are GT the index we remove, we subtract 1 from it
        self.entries.iter_mut().for_each(|mut f| {
            let v = f.value_mut();
            if v.store_index > index {
                v.store_index -= 1;
            }
        })
    }

    pub fn new_pool(&self, asset0: Address, asset1: Address, pool: AngPoolConfigEntry) {
        let key = Self::derive_store_key(asset0, asset1);

        self.entries.insert(key, pool);
    }

    pub fn derive_store_key(asset0: Address, asset1: Address) -> AngstromPoolPartialKey {
        let hash = keccak256((asset0, asset1).abi_encode());
        let mut store_key = [0u8; 27];
        store_key.copy_from_slice(&hash[5..32]);
        AngstromPoolPartialKey(store_key)
    }

    pub fn get_entry(&self, asset0: Address, asset1: Address) -> Option<AngPoolConfigEntry> {
        let store_key = Self::derive_store_key(asset0, asset1);
        self.entries.get(&store_key).map(|i| *i)
    }

    pub fn all_entries(&self) -> &DashMap<AngstromPoolPartialKey, AngPoolConfigEntry> {
        &self.entries
    }
}

impl TryFrom<&[u8]> for AngstromPoolConfigStore {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Ok(Self::default());
        }

        if value.first() != Some(&0) {
            return Err("Invalid encoded entries: must start with a safety byte of 0".to_string());
        }
        tracing::info!(bytecode_len=?value.len());
        let adjusted_entries = &value[1..];
        if adjusted_entries.len() % POOL_CONFIG_STORE_ENTRY_SIZE != 0 {
            tracing::info!(bytecode_len=?adjusted_entries.len(), ?POOL_CONFIG_STORE_ENTRY_SIZE);
            return Err(
                "Invalid encoded entries: incorrect length after removing safety byte".to_string()
            );
        }
        let entries = adjusted_entries
            .chunks(POOL_CONFIG_STORE_ENTRY_SIZE)
            .enumerate()
            .map(|(index, chunk)| {
                let pool_partial_key =
                    AngstromPoolPartialKey(<[u8; 27]>::try_from(&chunk[0..27]).unwrap());
                let tick_spacing = u16::from_be_bytes([chunk[27], chunk[28]]);
                let fee_in_e6 = u32::from_be_bytes([0, chunk[29], chunk[30], chunk[31]]);
                (
                    pool_partial_key,
                    AngPoolConfigEntry {
                        pool_partial_key,
                        tick_spacing,
                        fee_in_e6,
                        store_index: index
                    }
                )
            })
            .collect();

        Ok(AngstromPoolConfigStore { entries })
    }
}

#[derive(Default, Clone)]
pub struct UniswapAngstromRegistry {
    uniswap_pools:         UniswapPoolRegistry,
    angstrom_config_store: Arc<AngstromPoolConfigStore>
}

impl UniswapAngstromRegistry {
    pub fn new(
        uniswap_pools: UniswapPoolRegistry,
        angstrom_config_store: Arc<AngstromPoolConfigStore>
    ) -> Self {
        UniswapAngstromRegistry { uniswap_pools, angstrom_config_store }
    }

    pub fn get_uni_pool(&self, pool_id: &PoolId) -> Option<PoolKey> {
        self.uniswap_pools.get(pool_id).cloned()
    }

    pub fn get_ang_entry(&self, pool_id: &PoolId) -> Option<AngPoolConfigEntry> {
        let uni_entry = self.get_uni_pool(pool_id)?;
        self.angstrom_config_store
            .get_entry(uni_entry.currency0, uni_entry.currency1)
    }
}
