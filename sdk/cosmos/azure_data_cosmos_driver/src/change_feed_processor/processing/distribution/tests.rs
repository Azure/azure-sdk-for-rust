// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{BatchDistributor, LeaseSession};
use crate::{
    change_feed_processor::processing::{Acknowledgment, BatchReceipt, ChangeFeedBatch},
    diagnostics::DiagnosticsContextBuilder,
    models::{ActivityId, CosmosResponse, CosmosResponseHeaders, CosmosStatus},
    options::DiagnosticsOptions,
};
use azure_core::http::StatusCode;
use futures::{
    channel::oneshot,
    future,
    task::{waker_ref, ArcWake},
    FutureExt,
};
use std::{
    future::Future,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    task::{Context, Poll},
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
        permit: None,
        sender,
        cancelled: future::pending().boxed().shared(),
    };
    (response, receipt, received)
}

fn publish(session: &LeaseSession, value: &str) -> oneshot::Receiver<Acknowledgment> {
    let (response, receipt, received) = delivery(value);
    assert!(session.deliver(response, receipt).unwrap());
    received
}

fn next(distributor: &BatchDistributor) -> ChangeFeedBatch {
    distributor
        .next_batch()
        .now_or_never()
        .expect("ready session")
        .expect("open distributor")
}

fn acknowledge(batch: ChangeFeedBatch, mut received: oneshot::Receiver<Acknowledgment>) {
    assert!(received.try_recv().unwrap().is_none());
    let (_, _, receipt) = batch.into_parts();
    let mut saving = Box::pin(receipt.checkpoint());
    assert!(saving.as_mut().now_or_never().is_none());
    let acknowledgment = received
        .try_recv()
        .unwrap()
        .expect("acknowledgment submitted");
    acknowledgment.result.send(Ok(())).unwrap();
    saving
        .now_or_never()
        .expect("checkpoint result delivered")
        .unwrap();
}

#[test]
fn publication_wakes_waiter_and_waiting_cancellation_keeps_batch() {
    let distributor = Arc::new(BatchDistributor::default());
    let session = distributor
        .register("lease".into(), Some("first".into()))
        .unwrap();
    let wakes = Arc::new(Wakes::default());
    let waker = waker_ref(&wakes);
    let mut cx = Context::from_waker(&waker);
    let mut waiting = Box::pin(distributor.next_batch());
    assert!(waiting.as_mut().poll(&mut cx).is_pending());
    let received = publish(&session, "queued");
    assert_eq!(wakes.0.load(Ordering::SeqCst), 1);
    drop(waiting);
    let batch = next(&distributor);
    assert_eq!(batch.response.body_bytes(), b"queued");
    acknowledge(batch, received);
    assert!(distributor.next_batch().now_or_never().is_none());
}

#[test]
fn ready_sessions_take_turns_without_waiting_for_idle_sessions() {
    let distributor = Arc::new(BatchDistributor::default());
    let a = distributor.register("a".into(), Some("a1".into())).unwrap();
    let b = distributor.register("b".into(), Some("b1".into())).unwrap();
    let c = distributor.register("c".into(), Some("c1".into())).unwrap();
    let _idle = distributor
        .register("idle".into(), Some("i1".into()))
        .unwrap();
    let a1 = publish(&a, "a1");
    let b1 = publish(&b, "b1");
    let c1 = publish(&c, "c1");
    let batch = next(&distributor);
    assert_eq!(batch.lease_token, "a");
    acknowledge(batch, a1);
    let a2 = publish(&a, "a2");

    for (expected, received) in [("b", b1), ("c", c1), ("a", a2)] {
        let batch = next(&distributor);
        assert_eq!(batch.lease_token, expected);
        acknowledge(batch, received);
    }
    assert!(distributor.next_batch().now_or_never().is_none());
}

#[test]
fn full_mailbox_rejects_duplicate_publication_without_losing_first_batch() {
    let distributor = Arc::new(BatchDistributor::default());
    let a = distributor.register("a".into(), Some("a1".into())).unwrap();
    let b = distributor.register("b".into(), Some("b1".into())).unwrap();
    let first = publish(&a, "first");
    let (response, receipt, rejected) = delivery("second");
    assert_eq!(
        a.deliver(response, receipt)
            .unwrap_err()
            .status()
            .status_code(),
        StatusCode::Conflict,
    );
    assert!(rejected.now_or_never().unwrap().is_err());
    let other = publish(&b, "other");
    assert_eq!(distributor.registry().ready.len(), 2);
    let batch = next(&distributor);
    assert_eq!(batch.response.body_bytes(), b"first");
    acknowledge(batch, first);
    let batch = next(&distributor);
    assert_eq!(batch.response.body_bytes(), b"other");
    acknowledge(batch, other);
    assert!(distributor.next_batch().now_or_never().is_none());
}

#[test]
fn canceled_producer_is_skipped_without_blocking_another_session() {
    let distributor = Arc::new(BatchDistributor::default());
    let a = distributor.register("a".into(), Some("a1".into())).unwrap();
    let b = distributor.register("b".into(), Some("b1".into())).unwrap();
    drop(publish(&a, "canceled"));
    let received = publish(&b, "valid");
    let batch = next(&distributor);
    assert_eq!(batch.lease_token, "b");
    acknowledge(batch, received);
    assert!(distributor.next_batch().now_or_never().is_none());
}

#[test]
fn retirement_discards_only_its_mailbox_and_empty_registry_stays_open() {
    let distributor = Arc::new(BatchDistributor::default());
    assert!(distributor.next_batch().now_or_never().is_none());
    let a = distributor.register("a".into(), Some("a1".into())).unwrap();
    let b = distributor.register("b".into(), Some("b1".into())).unwrap();
    let retired = publish(&a, "retired");
    let retained = publish(&b, "retained");
    drop(a);
    assert!(retired.now_or_never().unwrap().is_err());
    let batch = next(&distributor);
    assert_eq!(batch.lease_token, "b");
    acknowledge(batch, retained);
    drop(b);
    assert!(distributor.registry().sessions.is_empty());
    assert!(distributor.registry().ready.is_empty());
    assert!(distributor.is_running());
    assert!(distributor.next_batch().now_or_never().is_none());

    let child = distributor
        .register("child".into(), Some("child1".into()))
        .unwrap();
    let received = publish(&child, "successor");
    let batch = next(&distributor);
    assert_eq!(batch.response.body_bytes(), b"successor");
    acknowledge(batch, received);
}

#[test]
fn stale_session_cannot_publish_to_or_retire_its_replacement() {
    for incarnation in ["second", "first"] {
        let distributor = Arc::new(BatchDistributor::default());
        let old = distributor
            .register("lease".into(), Some("first".into()))
            .unwrap();
        let retired = publish(&old, "old");
        distributor.retire(&old.key, &old.mailbox);
        assert!(retired.now_or_never().unwrap().is_err());
        let replacement = distributor
            .register("lease".into(), Some(incarnation.into()))
            .unwrap();
        let received = publish(&replacement, "replacement");
        let (response, receipt, rejected) = delivery("stale");
        assert!(old
            .deliver(response, receipt)
            .unwrap_err()
            .status()
            .is_precondition_failed());
        assert!(rejected.now_or_never().unwrap().is_err());
        drop(old);
        let batch = next(&distributor);
        assert_eq!(batch.response.body_bytes(), b"replacement");
        acknowledge(batch, received);
    }
}

#[test]
fn duplicate_registration_preserves_existing_session() {
    let distributor = Arc::new(BatchDistributor::default());
    let session = distributor
        .register("lease".into(), Some("first".into()))
        .unwrap();
    let received = publish(&session, "original");
    let error = distributor
        .register("lease".into(), Some("first".into()))
        .err()
        .unwrap();
    assert_eq!(error.status().status_code(), StatusCode::Conflict);
    assert!(distributor.register("other".into(), None).is_err());
    let batch = next(&distributor);
    assert_eq!(batch.response.body_bytes(), b"original");
    acknowledge(batch, received);
}

#[test]
fn graceful_shutdown_drains_ready_sessions_and_rejects_new_batches() {
    let distributor = Arc::new(BatchDistributor::default());
    let a = distributor.register("a".into(), Some("a1".into())).unwrap();
    let b = distributor.register("b".into(), Some("b1".into())).unwrap();
    let first = publish(&a, "first");
    let second = publish(&b, "second");
    distributor.stop_delivery(false);
    assert!(distributor.is_running());
    for (expected, received) in [("a", first), ("b", second)] {
        let batch = next(&distributor);
        assert_eq!(batch.lease_token, expected);
        acknowledge(batch, received);
    }
    let (response, receipt, rejected) = delivery("late");
    assert!(!a.deliver(response, receipt).unwrap());
    assert!(rejected.now_or_never().unwrap().is_err());
    assert!(matches!(
        distributor.next_batch().now_or_never(),
        Some(None)
    ));
    assert!(distributor.is_running());
    distributor.close();
    assert!(!distributor.is_running());
}

#[test]
fn force_during_drain_discards_pending_mailboxes_without_reopening() {
    let distributor = Arc::new(BatchDistributor::default());
    let session = distributor.register("a".into(), Some("a1".into())).unwrap();
    let queued = publish(&session, "pending");
    distributor.stop_delivery(false);
    distributor.stop_delivery(true);
    distributor.stop_delivery(false);
    assert!(queued.now_or_never().unwrap().is_err());
    assert!(matches!(
        distributor.next_batch().now_or_never(),
        Some(None)
    ));
    let (response, receipt, rejected) = delivery("late");
    assert!(!session.deliver(response, receipt).unwrap());
    assert!(rejected.now_or_never().unwrap().is_err());
}

#[test]
fn shutdown_wakes_waiter_and_rejects_further_sessions() {
    let distributor = Arc::new(BatchDistributor::default());
    let wakes = Arc::new(Wakes::default());
    let waker = waker_ref(&wakes);
    let mut cx = Context::from_waker(&waker);
    let mut waiting = Box::pin(distributor.next_batch());
    assert!(waiting.as_mut().poll(&mut cx).is_pending());
    distributor.close();
    assert_eq!(wakes.0.load(Ordering::SeqCst), 1);
    assert!(matches!(waiting.as_mut().poll(&mut cx), Poll::Ready(None)));
    assert!(!distributor.is_running());
    assert!(distributor
        .register("late".into(), Some("late1".into()))
        .is_err());
}

#[test]
fn shutdown_discards_queued_batches_and_rejects_late_publication() {
    let distributor = Arc::new(BatchDistributor::default());
    let session = distributor
        .register("lease".into(), Some("first".into()))
        .unwrap();
    let queued = publish(&session, "queued");
    distributor.close();
    assert!(queued.now_or_never().unwrap().is_err());
    assert!(matches!(
        distributor.next_batch().now_or_never(),
        Some(None)
    ));
    let (response, receipt, rejected) = delivery("late");
    assert!(session
        .deliver(response, receipt)
        .unwrap_err()
        .status()
        .is_precondition_failed());
    assert!(rejected.now_or_never().unwrap().is_err());
    drop(session);
    distributor.close();
    assert!(distributor.registry().sessions.is_empty());
    assert!(distributor.registry().ready.is_empty());
}
