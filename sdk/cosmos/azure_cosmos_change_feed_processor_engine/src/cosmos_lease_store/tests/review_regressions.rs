// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{fixture, fixture_with_rules, LeaseOwnershipOptions};
use crate::{ChangeFeedReadOptions, CheckpointError, LeaseRunOptions, LeaseRunOutcome};
use azure_core::http::StatusCode;
use azure_data_cosmos_driver::{
    fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionResultBuilder, FaultInjectionRule,
        FaultInjectionRuleBuilder, FaultOperationType,
    },
    models::{ChangeFeedStartFrom, ContinuationToken, FeedRange},
};
use std::{
    error::Error,
    num::NonZeroU32,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::time::{sleep, timeout};

fn delayed(operation: FaultOperationType) -> Arc<FaultInjectionRule> {
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "cancelled-standalone-write",
            FaultInjectionResultBuilder::new()
                .with_probability(1.0)
                .with_delay(Duration::from_secs(1))
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(operation)
                .build(),
        )
        .build(),
    );
    rule.disable();
    rule
}

async fn entered(rule: &FaultInjectionRule) -> Result<(), Box<dyn Error>> {
    timeout(Duration::from_secs(2), async {
        while rule.hit_count() == 0 {
            sleep(Duration::from_millis(1)).await;
        }
    })
    .await?;
    Ok(())
}

#[derive(Clone, Copy)]
enum StandaloneWrite {
    Renew,
    Checkpoint,
    Release,
}

async fn cancelled_write_stops_waiting_processor(
    write: StandaloneWrite,
) -> Result<(), Box<dyn Error>> {
    let feed_delay = delayed(FaultOperationType::ChangeFeedItem);
    let write_delay = delayed(FaultOperationType::ReplaceItem);
    let f = fixture_with_rules(vec![feed_delay.clone(), write_delay.clone()]).await?;
    let session = f.store_a.try_acquire("A").await?.unwrap();
    let lease = session.lease().await;
    let initial = lease.checkpoint().clone();
    let control = session.control().clone();
    let running_control = control.clone();
    let mut checkpoint_store = session.clone();
    let callbacks = Arc::new(AtomicU32::new(0));
    let callback_count = callbacks.clone();
    feed_delay.enable();
    let processing = tokio::spawn(async move {
        f.worker_a
            .run_owned_lease(
                lease,
                ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning),
                &running_control,
                &LeaseRunOptions::new(NonZeroU32::new(1).unwrap()),
                &mut checkpoint_store,
                |_| {
                    callback_count.fetch_add(1, Ordering::SeqCst);
                    async { Ok(()) }
                },
            )
            .await
    });
    entered(&feed_delay).await?;
    let expected = session.lease().await;
    let writer = session.clone();
    write_delay.enable();
    let writing = tokio::spawn(async move {
        match write {
            StandaloneWrite::Renew => {
                writer.renew().await.unwrap();
            }
            StandaloneWrite::Checkpoint => {
                writer
                    .checkpoint(&expected, expected.checkpoint())
                    .await
                    .unwrap();
            }
            StandaloneWrite::Release => {
                writer.release().await.unwrap();
            }
        }
    });
    entered(&write_delay).await?;
    writing.abort();
    assert!(writing.await.unwrap_err().is_cancelled());
    assert!(
        control.is_lost(),
        "cancellation must immediately signal lost authority"
    );
    let report = timeout(Duration::from_millis(100), processing)
        .await??
        .unwrap();
    assert_eq!(report.outcome(), LeaseRunOutcome::OwnershipLost);
    assert_eq!(report.confirmed_batches(), 0);
    assert_eq!(callbacks.load(Ordering::SeqCst), 0);
    assert_eq!(report.lease().checkpoint(), &initial);
    Ok(())
}

#[tokio::test]
async fn cancelling_standalone_renewal_notifies_an_active_processor() -> Result<(), Box<dyn Error>>
{
    cancelled_write_stops_waiting_processor(StandaloneWrite::Renew).await
}

#[tokio::test]
async fn cancelling_standalone_checkpoint_notifies_an_active_processor(
) -> Result<(), Box<dyn Error>> {
    cancelled_write_stops_waiting_processor(StandaloneWrite::Checkpoint).await
}

#[tokio::test]
async fn cancelling_standalone_release_notifies_an_active_processor() -> Result<(), Box<dyn Error>>
{
    cancelled_write_stops_waiting_processor(StandaloneWrite::Release).await
}

#[test]
fn renewal_headroom_includes_both_request_latencies() {
    let policy = |timeout_ms| {
        LeaseOwnershipOptions::new(
            Duration::from_millis(2000),
            Duration::from_millis(500),
            Duration::from_millis(200),
            Duration::from_millis(timeout_ms),
        )
    };
    assert!(policy(1000).is_err());
    assert!(policy(650).is_err(), "equality leaves no renewal headroom");
    assert!(policy(649).is_ok());
}

#[tokio::test]
async fn a_checkpoint_already_committed_before_412_requires_reconciliation(
) -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    let session = f.store_a.try_acquire("A").await?.unwrap();
    let expected = session.lease().await;
    let mut reader = f
        .worker_a
        .open_reader(
            ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning)
                .with_continuation(expected.checkpoint().clone()),
        )
        .await?;
    let candidate = reader.read_page().await?.continuation().clone();
    let observation = f.store_b.observe().await?;
    let mut committed = observation.record.clone();
    committed.checkpoint = candidate.as_str().to_owned();
    f.store_b
        .replace(&committed, observation.revision())
        .await?;
    let error = session.checkpoint(&expected, &candidate).await.unwrap_err();
    match error {
        CheckpointError::Ambiguous(error) => {
            assert_eq!(error.status().status_code(), StatusCode::PreconditionFailed)
        }
        other => {
            panic!("a previously committed candidate cannot be classified as rejected: {other:?}")
        }
    }
    let durable = f.store_b.observe().await?;
    assert_eq!(durable.owner(), Some(expected.owner()));
    assert_eq!(durable.generation(), expected.epoch().get());
    assert_eq!(durable.checkpoint(), candidate.as_str());
    Ok(())
}

#[tokio::test]
async fn rejected_local_checkpoint_validation_does_not_revoke_confirmed_authority(
) -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    let session = f.store_a.try_acquire("A").await?.unwrap();
    let expected = session.lease().await;
    let error = session
        .checkpoint(&expected, &ContinuationToken::from_string(String::new()))
        .await
        .unwrap_err();
    assert!(matches!(error, CheckpointError::Rejected(_)));
    assert!(!session.control().is_lost());
    assert_eq!(f.store_b.observe().await?.revision(), expected.revision());
    session.release().await?;
    Ok(())
}
