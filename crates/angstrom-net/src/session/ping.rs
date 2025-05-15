use std::{
    task::Poll,
    time::{Duration, SystemTime}
};

use futures::{Stream, StreamExt};
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

pub const PING: [u8; 4] = [0x4D, 0x45, 0x4F, 0x57];

/// Simple structure for tracking ping messages.
/// we really dont care about acking the pings here
/// as we already have a angstrom handshake to
/// even start recieving messages  
pub struct Ping {
    last_received_ping:     SystemTime,
    /// the amount of time between when our pings are send
    ping_interval:          IntervalStream,
    /// the max amount of time before we should disconnect
    max_recv_ping_interval: Duration
}

impl Ping {
    pub fn new(breach_timeout: Duration) -> Self {
        let send_interval = breach_timeout * 2 / 3;
        let ping_interval = IntervalStream::new(interval(send_interval));
        Self {
            last_received_ping: SystemTime::now(),
            ping_interval,
            max_recv_ping_interval: breach_timeout
        }
    }

    pub fn got_ping(&mut self) {
        self.last_received_ping = SystemTime::now();
    }
}

impl Stream for Ping {
    type Item = [u8; 4];

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Option<Self::Item>> {
        while let Poll::Ready(Some(_)) = self.ping_interval.poll_next_unpin(cx) {
            return Poll::Ready(Some(PING));
        }

        if self
            .last_received_ping
            .duration_since(SystemTime::now())
            .unwrap()
            > self.max_recv_ping_interval
        {
            return Poll::Ready(None);
        }

        Poll::Pending
    }
}
