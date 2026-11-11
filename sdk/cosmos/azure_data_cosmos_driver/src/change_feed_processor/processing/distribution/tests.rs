// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{AcquisitionDelivery, BatchDistributor, ReadDemand};
use crate::{
    change_feed_processor::processing::{Acknowledgment, BatchReceipt, ChangeFeedBatch},
    diagnostics::DiagnosticsContextBuilder,
    models::{ActivityId, CosmosResponse, CosmosResponseHeaders, CosmosStatus},
    options::DiagnosticsOptions,
    Result,
};
use azure_core::http::StatusCode;
use futures::{
    channel::oneshot,
    future::{self, BoxFuture},
    task::{waker_ref, ArcWake},
    FutureExt,
};
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    task::Context,
};

#[derive(Default)]
struct Wakes(AtomicUsize);

impl ArcWake for Wakes {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.0.fetch_add(1, Ordering::SeqCst);
    }
}

fn delivery(
    value: &str,
) -> (
    CosmosResponse,
    BatchReceipt,
    oneshot::Receiver<Acknowledgment>,
) {
    let diagnostics = DiagnosticsContextBuilder::new(
        ActivityId::new_uuid(),
        Arc::new(DiagnosticsOptions::default()),
    )
    .complete();
    let response = CosmosResponse::new(
        value.as_bytes().to_vec(),
        CosmosResponseHeaders::new(),
        CosmosStatus::new(StatusCode::Ok),
        Arc::new(diagnostics),
    );
    let (sender, received) = oneshot::channel();
    let receipt = BatchReceipt {
        sender,
        cancelled: future::pending().boxed().shared(),
    };
    (response, receipt, received)
}

fn acquire(distributor: &Arc<BatchDistributor>, lease: &str) -> AcquisitionDelivery {
    distributor
        .acquire(lease.into(), Some("incarnation".into()))
        .unwrap()
}

fn waiting(session: &AcquisitionDelivery) -> BoxFuture<'_, Result<Option<ReadDemand>>> {
    let mut waiting = session.wait_for_demand().boxed();
    assert!(waiting.as_mut().now_or_never().is_none());
    waiting
}

fn receive(distributor: &Arc<BatchDistributor>) -> BoxFuture<'static, Option<ChangeFeedBatch>> {
    let distributor = distributor.clone();
    async move { distributor.next_batch().await }.boxed()
}

fn authorize(
    session: &AcquisitionDelivery,
) -> (ReadDemand, BoxFuture<'static, Option<ChangeFeedBatch>>) {
    let waiting = waiting(session);
    let mut receiving = receive(&session.distributor);
    assert!(receiving.as_mut().now_or_never().is_none());
    let demand = waiting.now_or_never().unwrap().unwrap().unwrap();
    (demand, receiving)
}

fn publish(
    session: &AcquisitionDelivery,
    demand: ReadDemand,
    value: &str,
) -> oneshot::Receiver<Acknowledgment> {
    let (response, receipt, received) = delivery(value);
    assert!(session.deliver(demand, response, receipt).unwrap());
    received
}

fn acknowledge(
    session: &AcquisitionDelivery,
    batch: ChangeFeedBatch,
    mut received: oneshot::Receiver<Acknowledgment>,
) {
    assert!(received.try_recv().unwrap().is_none());
    let (_, _, receipt) = batch.into_parts();
    let mut saving = receipt.checkpoint().boxed();
    assert!(saving.as_mut().now_or_never().is_none());
    let acknowledgment = received
        .try_recv()
        .unwrap()
        .expect("acknowledgment submitted");
    session.acknowledged().unwrap();
    acknowledgment.result.send(Ok(())).unwrap();
    saving.now_or_never().unwrap().unwrap();
}

#[test]
fn acquisition_waits_for_demand_and_idle_reads_rotate() {
    let distributor = Arc::new(BatchDistributor::default());
    let a = acquire(&distributor, "a");
    let b = acquire(&distributor, "b");
    let mut a_waiting = waiting(&a);
    let mut b_waiting = waiting(&b);
    assert!(a_waiting.as_mut().now_or_never().is_none());
    assert!(b_waiting.as_mut().now_or_never().is_none());
    let mut receiving = receive(&distributor);
    assert!(receiving.as_mut().now_or_never().is_none());
    let demand = a_waiting.as_mut().now_or_never().unwrap().unwrap().unwrap();
    assert!(b_waiting.as_mut().now_or_never().is_none());
    drop(demand);
    let mut a_again = waiting(&a);
    assert!(receiving.as_mut().now_or_never().is_none());
    let demand = b_waiting.as_mut().now_or_never().unwrap().unwrap().unwrap();
    assert!(a_again.as_mut().now_or_never().is_none());
    let received = publish(&b, demand, "b");
    let batch = receiving.now_or_never().unwrap().unwrap();
    assert_eq!(batch.lease_token, "b");
    acknowledge(&b, batch, received);
    assert!(a_again.as_mut().now_or_never().is_none());
}

#[test]
fn concurrent_receives_authorize_only_the_requested_parallelism() {
    let distributor = Arc::new(BatchDistributor::default());
    let a = acquire(&distributor, "a");
    let b = acquire(&distributor, "b");
    let c = acquire(&distributor, "c");
    let a_waiting = waiting(&a);
    let b_waiting = waiting(&b);
    let mut c_waiting = waiting(&c);
    let mut first = receive(&distributor);
    let mut second = receive(&distributor);
    assert!(first.as_mut().now_or_never().is_none());
    assert!(second.as_mut().now_or_never().is_none());
    let a_demand = a_waiting.now_or_never().unwrap().unwrap().unwrap();
    let b_demand = b_waiting.now_or_never().unwrap().unwrap().unwrap();
    assert!(c_waiting.as_mut().now_or_never().is_none());
    let received = publish(&b, b_demand, "b");
    assert!(first.as_mut().now_or_never().is_none());
    let batch = second.now_or_never().unwrap().unwrap();
    assert_eq!(batch.lease_token, "b");
    acknowledge(&b, batch, received);
    assert!(c_waiting.as_mut().now_or_never().is_none());
    let received = publish(&a, a_demand, "a");
    acknowledge(&a, first.now_or_never().unwrap().unwrap(), received);
    assert!(c_waiting.as_mut().now_or_never().is_none());
}

#[test]
fn response_wakes_only_its_requester_and_cannot_be_taken_by_another_receive() {
    let distributor = Arc::new(BatchDistributor::default());
    let session = acquire(&distributor, "lease");
    let (demand, mut receiving) = authorize(&session);
    let wakes = Arc::new(Wakes::default());
    let waker = waker_ref(&wakes);
    let mut cx = Context::from_waker(&waker);
    assert!(receiving.as_mut().poll(&mut cx).is_pending());
    let received = publish(&session, demand, "requested");
    assert!(wakes.0.load(Ordering::SeqCst) > 0);
    assert!(receive(&distributor).now_or_never().is_none());
    let batch = receiving.now_or_never().unwrap().unwrap();
    assert_eq!(batch.response.body_bytes(), b"requested");
    acknowledge(&session, batch, received);
}

#[test]
fn cancellation_after_response_transfer_drops_the_unclaimed_batch() {
    let distributor = Arc::new(BatchDistributor::default());
    let session = acquire(&distributor, "lease");
    let (demand, receiving) = authorize(&session);
    let received = publish(&session, demand, "unclaimed");
    drop(receiving);
    assert!(received.now_or_never().unwrap().is_err());
    assert!(receive(&distributor).now_or_never().is_none());
}

#[test]
fn response_after_receive_cancellation_is_rejected_without_buffering() {
    let distributor = Arc::new(BatchDistributor::default());
    let session = acquire(&distributor, "lease");
    let (demand, receiving) = authorize(&session);
    drop(receiving);
    assert!(demand.is_canceled());
    let (response, receipt, received) = delivery("unclaimed");
    assert_eq!(
        session
            .deliver(demand, response, receipt)
            .unwrap_err()
            .status()
            .status_code(),
        StatusCode::Conflict
    );
    assert!(received.now_or_never().unwrap().is_err());
    assert!(session
        .acknowledged()
        .unwrap_err()
        .status()
        .is_precondition_failed());
    assert!(receive(&distributor).now_or_never().is_none());
}

#[test]
fn canceled_demand_is_visible_before_a_read_starts() {
    let distributor = Arc::new(BatchDistributor::default());
    let session = acquire(&distributor, "lease");
    let (demand, receiving) = authorize(&session);
    assert!(!demand.is_canceled());
    drop(receiving);
    assert!(demand.is_canceled());
    drop(demand);
    let mut waiting = waiting(&session);
    assert!(waiting.as_mut().now_or_never().is_none());
}

#[test]
fn delivery_requires_demand_and_the_previous_acknowledgment() {
    let distributor = Arc::new(BatchDistributor::default());
    let session = acquire(&distributor, "lease");
    let (sender, _receiver) = oneshot::channel();
    let (response, receipt, rejected) = delivery("unsolicited");
    assert_eq!(
        session
            .deliver(ReadDemand { response: sender }, response, receipt)
            .unwrap_err()
            .status()
            .status_code(),
        StatusCode::Conflict
    );
    assert!(rejected.now_or_never().unwrap().is_err());
    let (demand, receiving) = authorize(&session);
    let received = publish(&session, demand, "first");
    assert!(session.wait_for_demand().now_or_never().unwrap().is_err());
    let batch = receiving.now_or_never().unwrap().unwrap();
    assert!(session.wait_for_demand().now_or_never().unwrap().is_err());
    acknowledge(&session, batch, received);
    let mut waiting = waiting(&session);
    assert!(waiting.as_mut().now_or_never().is_none());
}

#[test]
fn canceled_producer_is_skipped_without_blocking_another_acquisition() {
    let distributor = Arc::new(BatchDistributor::default());
    let a = acquire(&distributor, "a");
    let (demand, mut receiving) = authorize(&a);
    drop(publish(&a, demand, "canceled"));
    let b = acquire(&distributor, "b");
    let b_waiting = waiting(&b);
    assert!(receiving.as_mut().now_or_never().is_none());
    let demand = b_waiting.now_or_never().unwrap().unwrap().unwrap();
    let received = publish(&b, demand, "valid");
    let batch = receiving.now_or_never().unwrap().unwrap();
    assert_eq!(batch.lease_token, "b");
    acknowledge(&b, batch, received);
}

#[test]
fn scheduler_does_not_retain_acquisition_state() {
    let distributor = Arc::new(BatchDistributor::default());
    let session = acquire(&distributor, "lease");
    let weak = Arc::downgrade(&session.delivery);
    drop(waiting(&session));
    drop(session);
    assert!(weak.upgrade().is_none());
    assert!(distributor.scheduler().available.is_empty());
    assert!(receive(&distributor).now_or_never().is_none());
    assert!(distributor.is_running());
}

#[test]
fn stale_demand_cannot_deliver_acknowledge_or_retire_a_replacement() {
    let distributor = Arc::new(BatchDistributor::default());
    let old = acquire(&distributor, "lease");
    let (stale, receiving) = authorize(&old);
    old.retire();
    let replacement = acquire(&distributor, "lease");
    let (demand, replacement_receive) = authorize(&replacement);
    let received = publish(&replacement, demand, "replacement");
    let (response, receipt, rejected) = delivery("stale");
    assert!(old
        .deliver(stale, response, receipt)
        .unwrap_err()
        .status()
        .is_precondition_failed());
    assert!(old
        .acknowledged()
        .unwrap_err()
        .status()
        .is_precondition_failed());
    assert!(rejected.now_or_never().unwrap().is_err());
    drop(old);
    assert!(receiving.now_or_never().is_none());
    let batch = replacement_receive.now_or_never().unwrap().unwrap();
    assert_eq!(batch.response.body_bytes(), b"replacement");
    acknowledge(&replacement, batch, received);
}

#[test]
fn graceful_shutdown_drains_transferred_batches_and_ends_demand_waits() {
    let distributor = Arc::new(BatchDistributor::default());
    let session = acquire(&distributor, "requested");
    let (demand, receiving) = authorize(&session);
    let received = publish(&session, demand, "requested");
    let late = acquire(&distributor, "late");
    let (demand, late_receiving) = authorize(&late);
    let idle = acquire(&distributor, "idle");
    let waiting = waiting(&idle);
    distributor.stop_delivery(false);
    assert!(waiting.now_or_never().unwrap().unwrap().is_none());
    acknowledge(
        &session,
        receiving.now_or_never().unwrap().unwrap(),
        received,
    );
    let (response, receipt, rejected) = delivery("late");
    assert!(!late.deliver(demand, response, receipt).unwrap());
    assert!(rejected.now_or_never().unwrap().is_err());
    assert!(late_receiving.now_or_never().unwrap().is_none());
    assert!(receive(&distributor).now_or_never().unwrap().is_none());
    assert!(session
        .wait_for_demand()
        .now_or_never()
        .unwrap()
        .unwrap()
        .is_none());
    assert!(distributor.is_running());
}

#[test]
fn force_during_drain_rejects_unclaimed_responses_without_reopening() {
    let distributor = Arc::new(BatchDistributor::default());
    let session = acquire(&distributor, "lease");
    let (demand, receiving) = authorize(&session);
    let unclaimed = publish(&session, demand, "unclaimed");
    distributor.stop_delivery(false);
    distributor.stop_delivery(true);
    distributor.stop_delivery(false);
    assert!(receiving.now_or_never().unwrap().is_none());
    assert!(unclaimed.now_or_never().unwrap().is_err());
}

#[test]
fn shutdown_wakes_waiters_and_rejects_late_acquisitions() {
    let distributor = Arc::new(BatchDistributor::default());
    let mut first = receive(&distributor);
    let mut second = receive(&distributor);
    assert!(first.as_mut().now_or_never().is_none());
    assert!(second.as_mut().now_or_never().is_none());
    distributor.close();
    assert!(first.now_or_never().unwrap().is_none());
    assert!(second.now_or_never().unwrap().is_none());
    assert!(!distributor.is_running());
    assert!(distributor
        .acquire("late".into(), Some("late".into()))
        .is_err());
    assert!(distributor.acquire("unowned".into(), None).is_err());
}
