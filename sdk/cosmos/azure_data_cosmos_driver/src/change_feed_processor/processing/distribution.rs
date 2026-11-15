// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    change_feed_processor::{
        bootstrap::error,
        processing::{lost, BatchReceipt, ChangeFeedBatch},
    },
    models::CosmosResponse,
    Result,
};
use azure_core::http::StatusCode;
use futures::{channel::oneshot, future::poll_fn, task::AtomicWaker};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, MutexGuard, Weak},
    task::Poll,
};

/// One acquisition's delivery phase; only a waiting phase retains a read-permission sender.
#[derive(Default)]
enum DeliveryState {
    #[default]
    Idle,
    WaitingForDemand(oneshot::Sender<ReadDemand>),
    Fetching,
    AwaitingAcknowledgment,
    Closed,
}

impl DeliveryState {
    fn take_read_request(&mut self) -> Option<oneshot::Sender<ReadDemand>> {
        match std::mem::replace(self, Self::Fetching) {
            Self::WaitingForDemand(request) => Some(request),
            previous => {
                *self = previous;
                None
            }
        }
    }
}

/// Acquisition-owned delivery state, independent of persisted lease ownership and checkpoints.
#[derive(Default)]
struct Delivery {
    state: Mutex<DeliveryState>,
}

impl Delivery {
    fn state(&self) -> MutexGuard<'_, DeliveryState> {
        self.state.lock().unwrap_or_else(|err| {
            tracing::error!("Recovering poisoned acquisition delivery state");
            err.into_inner()
        })
    }

    fn close(&self) -> DeliveryState {
        let mut state = self.state();
        std::mem::replace(&mut *state, DeliveryState::Closed)
    }
}

/// FIFO of eligible acquisitions and worker shutdown flags; never a queue of batch payloads.
#[derive(Default)]
struct Scheduler {
    available: VecDeque<Weak<Delivery>>,
    closed: bool,
    stopping: bool,
    forced: bool,
}

/// Matches pending batch receives to eligible acquisitions without reading the feed itself.
#[derive(Default)]
pub(super) struct ChangeFeedRequestDistributor {
    scheduler: Mutex<Scheduler>,
    // Serialize selection and its registered waker, not the subsequent network request.
    receiver: async_lock::Mutex<()>,
    waker: AtomicWaker,
}

/// Permission for one acquisition to read, with a direct response channel back to its receiver.
pub(super) struct ReadDemand {
    response: oneshot::Sender<ChangeFeedBatch>,
}

impl ReadDemand {
    pub(super) fn is_canceled(&self) -> bool {
        self.response.is_canceled()
    }
}

impl ChangeFeedRequestDistributor {
    fn scheduler(&self) -> MutexGuard<'_, Scheduler> {
        self.scheduler.lock().unwrap_or_else(|err| {
            tracing::error!("Recovering poisoned batch scheduler");
            err.into_inner()
        })
    }

    pub(super) fn acquire(
        self: &Arc<Self>,
        lease_token: String,
        incarnation: Option<String>,
    ) -> Result<AcquisitionDelivery> {
        // Create acquisition-owned state; it becomes eligible only when it requests demand.
        if incarnation.is_none() || self.scheduler().closed {
            return Err(lost());
        }
        Ok(AcquisitionDelivery {
            lease_token,
            delivery: Arc::new(Delivery::default()),
            distributor: self.clone(),
        })
    }

    pub(super) fn is_running(&self) -> bool {
        !self.scheduler().closed
    }

    pub(super) async fn read_next_change_feed_batch(&self) -> Option<ChangeFeedBatch> {
        // Repeat until a valid batch returns Some, shutdown returns None through `?`, or this future is dropped.
        loop {
            let received = {
                // Only queue selection is serialized, not the authorized reads.
                let _receiver = self.receiver.lock().await;
                poll_fn(|cx| {
                    // Register before inspecting the queue so newly eligible work can wake this receive.
                    self.waker.register(cx.waker());
                    let mut scheduler = self.scheduler();
                    if scheduler.closed || scheduler.stopping {
                        // Exit only this selection closure; the `?` below propagates None from read_next_change_feed_batch.
                        return Poll::Ready(None);
                    }
                    while let Some(available) = scheduler.available.pop_front() {
                        // Weak handles and stale entries must not retain or revive ended acquisitions.
                        let Some(delivery) = available.upgrade() else {
                            continue;
                        };
                        let mut state = delivery.state();
                        let Some(request) = state.take_read_request() else {
                            continue;
                        };
                        // Give the chosen acquisition permission and wait on a separate batch channel.
                        let (response, received) = oneshot::channel();
                        if request.send(ReadDemand { response }).is_ok() {
                            // Selection is complete, not the batch receive; return the reply receiver.
                            return Poll::Ready(Some(received));
                        }
                        // Its demand waiter disappeared before permission arrived; no read was authorized.
                        *state = DeliveryState::Idle;
                    }
                    // No eligible lease yet: suspend rather than issue a read or spin.
                    Poll::Pending
                })
                .await
            };
            // Equivalent to matching Some(receiver), or returning None from this function on shutdown.
            let received = received?;
            // Idle reads and ended acquisitions drop the sender; try another acquisition.
            if let Ok(batch) = received.await {
                let scheduler = self.scheduler();
                // Graceful stop may drain issued work; forced stop or a canceled receipt rejects it.
                if !scheduler.closed && !scheduler.forced && !batch.receipt.sender.is_canceled() {
                    return Some(batch);
                }
            }
            // No valid batch was returned: retry selection for this same pending SDK receive.
        }
    }

    pub(super) fn close(&self) {
        self.scheduler().closed = true;
        self.stop_delivery(true);
    }

    pub(super) fn stop_delivery(&self, force: bool) {
        let pending = {
            let mut scheduler = self.scheduler();
            scheduler.stopping = true;
            scheduler.forced |= force;
            // Remove queued permissions; dropping their senders releases acquisition waiters.
            std::mem::take(&mut scheduler.available)
        };
        for pending in pending {
            if let Some(delivery) = pending.upgrade() {
                drop(delivery.close());
            }
        }
        // Release a receiver waiting for eligibility, even when the queue was empty.
        self.waker.wake();
    }
}

/// Owns delivery state for one acquisition; scheduling handles cannot keep it alive.
pub(super) struct AcquisitionDelivery {
    lease_token: String,
    delivery: Arc<Delivery>,
    distributor: Arc<ChangeFeedRequestDistributor>,
}

impl AcquisitionDelivery {
    pub(super) async fn wait_for_demand(&self) -> Result<Option<ReadDemand>> {
        let received = {
            let mut scheduler = self.distributor.scheduler();
            if scheduler.stopping || scheduler.closed {
                return Ok(None);
            }
            let mut state = self.delivery.state();
            match &*state {
                DeliveryState::Closed => return Err(lost()),
                DeliveryState::Idle | DeliveryState::Fetching => {}
                _ => {
                    return Err(error(
                        StatusCode::Conflict,
                        "Acquisition already has pending demand or an outstanding batch.",
                    ))
                }
            }
            let (request, received) = oneshot::channel();
            *state = DeliveryState::WaitingForDemand(request);
            // Rejoin at the back for a fair eligible turn; the queue does not own this acquisition.
            scheduler
                .available
                .push_back(Arc::downgrade(&self.delivery));
            received
        };
        // Wake a pending receiver only after releasing scheduler and delivery locks.
        self.distributor.waker.wake();
        match received.await {
            Ok(demand) => Ok(Some(demand)),
            Err(_) if self.distributor.scheduler().stopping => Ok(None),
            Err(_) => Err(lost()),
        }
    }

    pub(super) fn deliver(
        &self,
        demand: ReadDemand,
        response: CosmosResponse,
        receipt: BatchReceipt,
    ) -> Result<bool> {
        let scheduler = self.distributor.scheduler();
        // Shutdown can race an issued read; do not start a new delivery after stopping begins.
        if scheduler.stopping && !scheduler.closed {
            return Ok(false);
        }
        let mut state = self.delivery.state();
        if scheduler.closed || matches!(*state, DeliveryState::Closed) {
            return Err(lost());
        }
        if !matches!(*state, DeliveryState::Fetching) {
            return Err(error(
                StatusCode::Conflict,
                "Acquisition already has an outstanding batch.",
            ));
        }
        let batch = ChangeFeedBatch {
            lease_token: self.lease_token.clone(),
            response,
            receipt,
        };
        // No further demand is allowed until this batch's receipt completes successfully.
        *state = DeliveryState::AwaitingAcknowledgment;
        let sent = demand.response.send(batch);
        if sent.is_err() {
            // The receiver canceled: abandon the acquisition instead of reassigning its payload.
            *state = DeliveryState::Closed;
        }
        drop(state);
        drop(scheduler);
        sent.map_err(|_| {
            error(
                StatusCode::Conflict,
                "Batch receive was canceled before delivery; abandoning the acquisition.",
            )
        })?;
        Ok(true)
    }

    pub(super) fn acknowledged(&self) -> Result<()> {
        let mut state = self.delivery.state();
        match &*state {
            DeliveryState::AwaitingAcknowledgment => {
                // Receipt completion permits another turn; persistence policy is handled by the poller.
                *state = DeliveryState::Idle;
                Ok(())
            }
            DeliveryState::Closed => Err(lost()),
            _ => Err(error(
                StatusCode::Conflict,
                "Acquisition has no delivered batch to acknowledge.",
            )),
        }
    }

    fn close(&self) {
        let pending = {
            let mut scheduler = self.distributor.scheduler();
            let delivery = Arc::downgrade(&self.delivery);
            // Remove only this acquisition's entries and close any outstanding demand channel.
            scheduler.available.retain(|ready| !ready.ptr_eq(&delivery));
            self.delivery.close()
        };
        drop(pending);
        self.distributor.waker.wake();
    }
}

impl Drop for AcquisitionDelivery {
    fn drop(&mut self) {
        self.close();
    }
}

#[cfg(test)]
mod tests;
