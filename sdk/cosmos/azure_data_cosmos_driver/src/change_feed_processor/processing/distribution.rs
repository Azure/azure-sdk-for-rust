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

#[derive(Default, PartialEq, Eq)]
enum DeliveryPhase {
    #[default]
    AwaitingDemand,
    Fetching,
    AwaitingAcknowledgment,
    Retired,
}

#[derive(Default)]
struct AcquisitionState {
    phase: DeliveryPhase,
    request: Option<oneshot::Sender<ReadDemand>>,
}

#[derive(Default)]
struct Delivery {
    state: Mutex<AcquisitionState>,
}

impl Delivery {
    fn state(&self) -> MutexGuard<'_, AcquisitionState> {
        self.state.lock().unwrap_or_else(|err| {
            tracing::error!("Recovering poisoned acquisition delivery state");
            err.into_inner()
        })
    }

    fn retire(&self) -> AcquisitionState {
        let mut state = self.state();
        std::mem::replace(
            &mut *state,
            AcquisitionState {
                phase: DeliveryPhase::Retired,
                ..Default::default()
            },
        )
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
                        let Some(request) = state.request.take() else {
                            continue;
                        };
                        let (response, received) = oneshot::channel();
                        state.phase = DeliveryPhase::Fetching;
                        if request.send(ReadDemand { response }).is_ok() {
                            return Poll::Ready(Some(received));
                        }
                        state.phase = DeliveryPhase::AwaitingDemand;
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
        let retired = {
            let mut scheduler = self.scheduler();
            scheduler.stopping = true;
            scheduler.forced |= force;
            std::mem::take(&mut scheduler.available)
        };
        for retired in retired {
            if let Some(delivery) = retired.upgrade() {
                drop(delivery.retire());
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
            if state.phase == DeliveryPhase::Retired {
                return Err(lost());
            }
            if state.request.is_some()
                || !matches!(
                    state.phase,
                    DeliveryPhase::AwaitingDemand | DeliveryPhase::Fetching
                )
            {
                return Err(error(
                    StatusCode::Conflict,
                    "Acquisition already has pending demand or an outstanding batch.",
                ));
            }
            let (request, received) = oneshot::channel();
            state.phase = DeliveryPhase::AwaitingDemand;
            state.request = Some(request);
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
        if scheduler.closed || state.phase == DeliveryPhase::Retired {
            return Err(lost());
        }
        if state.phase != DeliveryPhase::Fetching {
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
        state.phase = DeliveryPhase::AwaitingAcknowledgment;
        let sent = demand.response.send(batch);
        if sent.is_err() {
            state.phase = DeliveryPhase::Retired;
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
        match state.phase {
            DeliveryPhase::AwaitingAcknowledgment => {
                state.phase = DeliveryPhase::AwaitingDemand;
                Ok(())
            }
            DeliveryPhase::Retired => Err(lost()),
            _ => Err(error(
                StatusCode::Conflict,
                "Acquisition has no delivered batch to acknowledge.",
            )),
        }
    }

    fn retire(&self) {
        let pending = {
            let mut scheduler = self.distributor.scheduler();
            let delivery = Arc::downgrade(&self.delivery);
            scheduler.available.retain(|ready| !ready.ptr_eq(&delivery));
            self.delivery.retire()
        };
        drop(pending);
        self.distributor.waker.wake();
    }
}

impl Drop for AcquisitionDelivery {
    fn drop(&mut self) {
        self.retire();
    }
}

#[cfg(test)]
mod tests;
