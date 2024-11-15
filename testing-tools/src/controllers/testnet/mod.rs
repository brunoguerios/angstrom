use std::{
    collections::{HashMap, HashSet},
    future::Future
};

use alloy::providers::Provider;
use angstrom::components::initialize_strom_handles;
use angstrom_network::{
    manager::StromConsensusEvent, NetworkOrderEvent, StromMessage, StromNetworkManager
};
use angstrom_types::{sol_bindings::grouped_orders::AllOrders, testnet::InitialTestnetState};
use consensus::AngstromValidator;
use futures::{FutureExt, StreamExt, TryFutureExt};
use rand::Rng;
use reth_chainspec::Hardforks;
use reth_metrics::common::mpsc::{
    metered_unbounded_channel, UnboundedMeteredReceiver, UnboundedMeteredSender
};
use reth_network_peers::pk2id;
use reth_provider::{BlockReader, ChainSpecProvider, HeaderProvider};
use secp256k1::{PublicKey, SecretKey};
use tracing::{instrument, span, Instrument, Level};

use super::utils::generate_node_keys;
use crate::{
    anvil_state_provider::{AnvilTestnetIntializer, TestnetBlockProvider},
    controllers::strom::TestnetNode,
    network::TestnetNodeNetwork
};

pub struct AngstromTestnet<C> {
    block_provider: TestnetBlockProvider,
    node:           TestnetNode<C>,
    _initializer:   AnvilTestnetIntializer,
    config:         ()
}
