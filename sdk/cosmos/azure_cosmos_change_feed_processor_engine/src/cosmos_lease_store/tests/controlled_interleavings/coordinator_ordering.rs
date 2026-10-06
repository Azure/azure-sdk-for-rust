// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{fixture, Boundary, Event, Phase};
use crate::{
    ChangeFeedReadOptions, CheckpointError, CheckpointStore, LeaseRunOptions, LeaseRunOutcome,
    LeaseSession, OwnedLease,
};
use azure_data_cosmos_driver::models::{ChangeFeedStartFrom, ContinuationToken, FeedRange};
use futures::future::BoxFuture;
use std::{
    error::Error,
    num::NonZeroU32,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{sync::oneshot, time::timeout};

struct TrackedCheckpoint {
    session: LeaseSession,
    trace: Arc<Mutex<Vec<Event>>>,
    event: Event,
}
impl CheckpointStore for TrackedCheckpoint {
    fn persist<'a>(
        &'a mut self,
        lease: &'a OwnedLease,
        candidate: &'a ContinuationToken,
    ) -> BoxFuture<'a, Result<OwnedLease, CheckpointError>> {
        Box::pin(async move {
            let mut event = self.event.clone();
            event.phase = Phase::CheckpointStarted;
            self.trace.lock().unwrap().push(event.clone());
            let receipt = self.session.checkpoint(lease, candidate).await?;
            event.phase = Phase::CheckpointConfirmed;
            self.trace.lock().unwrap().push(event);
            Ok(receipt)
        })
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn callback_waits_while_renewal_confirms_then_checkpoint_uses_that_authoritative_revision(
) -> Result<(), Box<dyn Error>> {
    timeout(Duration::from_secs(10), async {
        let (f, service) = fixture().await?;
        let session = f.store_a.try_acquire("A").await?.unwrap();
        let initial = session.lease().await;
        let (entered, arrival) = oneshot::channel();
        let (resume, released) = oneshot::channel();
        let control = session.control().clone();
        let base = Event {
            worker: "A",
            lease_id: initial.id().to_owned(),
            generation: initial.epoch().get(),
            batch_id: 1,
            operation_id: 100,
            phase: Phase::CallbackEntered,
            if_match: String::new(),
        };
        let trace = service.events.clone();
        let mut checkpoint_store = TrackedCheckpoint {
            session: session.clone(),
            trace: trace.clone(),
            event: base.clone(),
        };
        let callback_event = base.clone();
        let task_trace = trace.clone();
        let mut exited = base.clone();
        exited.phase = Phase::TaskExited;
        let processing = tokio::spawn(async move {
            let mut entered = Some(entered);
            let mut released = Some(released);
            let report = f
                .worker_a
                .run_owned_lease(
                    initial,
                    ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning),
                    &control,
                    &LeaseRunOptions::new(NonZeroU32::new(1).unwrap()),
                    &mut checkpoint_store,
                    move |raw| {
                        let entered = entered.take().unwrap();
                        let released = released.take().unwrap();
                        let trace = trace.clone();
                        let mut event = callback_event.clone();
                        async move {
                            trace.lock().unwrap().push(event.clone());
                            entered.send(raw.continuation().clone()).unwrap();
                            released.await.unwrap();
                            event.phase = Phase::CallbackCompleted;
                            trace.lock().unwrap().push(event);
                            Ok(())
                        }
                    },
                )
                .await;
            task_trace.lock().unwrap().push(exited);
            report
        });
        let candidate = arrival.await?;
        let original = session.lease().await;
        let renewal = service.gate(Boundary::AfterCommit, 0, false);
        let renewal_id = renewal.operation_id;
        let writer = session.clone();
        let events = service.events.clone();
        let mut renewed_event = base.clone();
        renewed_event.phase = Phase::RenewConfirmed;
        renewed_event.operation_id = renewal_id;
        let renewing = tokio::spawn(async move {
            let lease = writer.renew().await;
            if lease.is_ok() {
                events.lock().unwrap().push(renewed_event);
            }
            lease
        });
        let committed = renewal.entered.await?;
        assert_eq!(committed.phase, Phase::Committed);
        assert_eq!(
            f.store_b.observe().await?.checkpoint(),
            original.checkpoint().as_str()
        );
        renewal.resume.send(()).unwrap();
        let confirmed = renewing.await??;
        assert_ne!(confirmed.revision(), original.revision());
        let checkpoint =
            service.checkpoint_gate(Boundary::BeforeCommit, 1, candidate.as_str(), false);
        resume.send(()).unwrap();
        let requested = checkpoint.entered.await?;
        let stored = f.store_b.observe().await?;
        assert_eq!(requested.if_match, confirmed.revision());
        assert_eq!(stored.revision(), confirmed.revision());
        assert_eq!(stored.checkpoint(), original.checkpoint().as_str());
        assert_eq!(stored.owner(), Some("A"));
        assert!(!processing.is_finished());
        checkpoint.resume.send(()).unwrap();
        let report = processing.await?.unwrap();
        assert_eq!(report.outcome(), LeaseRunOutcome::BatchLimitReached);
        assert_eq!(report.confirmed_batches(), 1);
        assert_eq!(f.store_b.observe().await?.checkpoint(), candidate.as_str());
        let trace = service.events.lock().unwrap().clone();
        let position = |phase| trace.iter().position(|event| event.phase == phase).unwrap();
        assert!(position(Phase::CallbackEntered) < position(Phase::RenewConfirmed));
        assert!(position(Phase::RenewConfirmed) < position(Phase::CallbackCompleted));
        assert!(position(Phase::CallbackCompleted) < position(Phase::CheckpointStarted));
        assert!(position(Phase::CheckpointStarted) < position(Phase::CheckpointConfirmed));
        assert!(position(Phase::CheckpointConfirmed) < position(Phase::TaskExited));
        session.release().await?;
        service.join_requests().await;
        Ok::<_, Box<dyn Error>>(())
    })
    .await??;
    Ok(())
}
