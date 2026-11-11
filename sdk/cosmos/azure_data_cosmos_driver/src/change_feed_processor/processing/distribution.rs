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

#[derive(Default)]
enum DeliveryState {
    #[default]
    Idle,
    WaitingForDemand(oneshot::Sender<ReadDemand>),
    Fetching,
    AwaitingAcknowledgment,
    Closed,
}

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

#[derive(Default)]
struct Scheduler {
    available: VecDeque<Weak<Delivery>>,
    closed: bool,
    stopping: bool,
    forced: bool,
}

#[derive(Default)]
pub(super) struct BatchDistributor {
    scheduler: Mutex<Scheduler>,
    receiver: async_lock::Mutex<()>,
    waker: AtomicWaker,
}

pub(super) struct ReadDemand {
    response: oneshot::Sender<ChangeFeedBatch>,
}

impl ReadDemand {
    pub(super) fn is_canceled(&self) -> bool {
        self.response.is_canceled()
    }
}

impl BatchDistributor {
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

    pub(super) async fn next_batch(&self) -> Option<ChangeFeedBatch> {
        loop {
            let received = {
                // Only queue selection is serialized, not the authorized reads.
                let _receiver = self.receiver.lock().await;
                poll_fn(|cx| {
                    self.waker.register(cx.waker());
                    let mut scheduler = self.scheduler();
                    if scheduler.closed || scheduler.stopping {
                        return Poll::Ready(None);
                    }
                    while let Some(available) = scheduler.available.pop_front() {
                        let Some(delivery) = available.upgrade() else {
                            continue;
                        };
                        let mut state = delivery.state();
                        let request = match std::mem::replace(&mut *state, DeliveryState::Fetching)
                        {
                            DeliveryState::WaitingForDemand(request) => request,
                            previous => {
                                *state = previous;
                                continue;
                            }
                        };
                        let (response, received) = oneshot::channel();
                        if request.send(ReadDemand { response }).is_ok() {
                            return Poll::Ready(Some(received));
                        }
                        *state = DeliveryState::Idle;
                    }
                    Poll::Pending
                })
                .await
            };
            let received = received?;
            // Idle reads and ended acquisitions drop the sender; try another acquisition.
            if let Ok(batch) = received.await {
                let scheduler = self.scheduler();
                if !scheduler.closed && !scheduler.forced && !batch.receipt.sender.is_canceled() {
                    return Some(batch);
                }
            }
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
            std::mem::take(&mut scheduler.available)
        };
        for pending in pending {
            if let Some(delivery) = pending.upgrade() {
                drop(delivery.close());
            }
        }
        self.waker.wake();
    }
}

/// Owns delivery state for one acquisition; scheduling handles cannot keep it alive.
pub(super) struct AcquisitionDelivery {
    lease_token: String,
    delivery: Arc<Delivery>,
    distributor: Arc<BatchDistributor>,
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
            scheduler
                .available
                .push_back(Arc::downgrade(&self.delivery));
            received
        };
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
        *state = DeliveryState::AwaitingAcknowledgment;
        let sent = demand.response.send(batch);
        if sent.is_err() {
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
