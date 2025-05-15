use std::{
    collections::VecDeque,
    fmt::Debug,
    net::SocketAddr,
    ops::Deref,
    pin::Pin,
    time::{SystemTime, UNIX_EPOCH}
};

use alloy::rlp::{BytesMut, Encodable};
use angstrom_types::primitive::{AngstromSigner, PeerId};
use angstrom_utils::{GenericExt, PollFlatten};
use futures::{
    Stream, StreamExt,
    task::{Context, Poll}
};
use reth_eth_wire::multiplex::ProtocolConnection;
use reth_metrics::common::mpsc::MeteredPollSender;
use tokio::time::Duration;
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::sync::PollSender;

use super::{
    super::{
        handle::SessionCommand,
        ping::{PING, Ping}
    },
    StromSession, VerificationSidecar
};
use crate::{
    StatusBuilder, StromMessage, StromSessionHandle, StromSessionMessage,
    types::{
        message::StromProtocolMessage,
        status::{Status, StatusState}
    }
};
pub struct RegularProcessing {
    conn:               ProtocolConnection,
    remote_peer_id:     PeerId,
    to_session_manager: MeteredPollSender<StromSessionMessage>,
    shutdown:           bool,
    outbound_buffer:    VecDeque<StromSessionMessage>
}
