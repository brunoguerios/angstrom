use std::{
    future::Future,
    sync::Arc,
    task::{Poll, Waker},
    time::Duration
};

use crate::rounds::OrderStorage;

/// How soon we send our pre-proposal
const DEFAULT_DURATION: Duration = Duration::from_secs(7);
/// The frequency we adjust our duration estimate.
const CHECK_INTERVAL: Duration = Duration::from_millis(200);
/// How close we want to be to the creation of the ethereum block
const TARGET_SUBMISSION_TIME_REM: Duration = Duration::from_millis(800);

/// When we should trigger to build our pre-proposals
/// this is very important for maximizing how long we can
/// wait till we start the next block. This helps us maximize
/// the amount of orders we clear while making sure that we
/// never miss a possible slot.
#[derive(Debug, Clone)]
pub struct PreProposalWaitTrigger {
    /// the base wait duration that we scale down based on orders.
    wait_duration: Duration,
    /// to track our scaling
    order_storage: Arc<OrderStorage>,
    /// Waker
    waker:         Option<Waker>
}

impl PreProposalWaitTrigger {
    pub fn new(order_storage: Arc<OrderStorage>) -> Self {
        Self { wait_duration: DEFAULT_DURATION, order_storage, waker: None }
    }

    pub fn update_for_new_round(&mut self, info: Option<LastRoundInfo>) -> Self {
        if let Some(info) = info {
            self.update_wait_duration_base(info);
        }
        self.waker = None;

        self.clone()
    }

    fn update_wait_duration_base(&mut self, info: LastRoundInfo) {}

    pub fn get_wait_time(&self) -> Duration {
        self.wait_duration
    }
}

impl Future for PreProposalWaitTrigger {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Self::Output> {
        Poll::Pending
    }
}

/// Details on how to adjust our duration,
/// Given that angstroms matching engine is linear time.
/// we scale the timeout estimation linearly.
#[derive(Debug)]
pub struct LastRoundInfo {
    /// the time from the trigger to the submission of the bundle to relays
    /// when we are the leader of the round
    pub time_to_complete: Duration,
    /// the total amount of orders we were running through the matching engine
    pub total_orders:     u16
}
