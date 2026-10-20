// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    balancing, fixture_with_emulator, insert, leases, next, options, start,
    ChangeFeedProcessorMode, Context, Lease, Update,
};
use crate::Result;
use std::{sync::Arc, time::Duration};

async fn fixture() -> Result<Arc<Context>> {
    Ok(
        fixture_with_emulator(ChangeFeedProcessorMode::LatestVersion, 1)
            .await?
            .0,
    )
}

async fn record(context: &Context, id: &str) -> Lease {
    context
        .store()
        .read::<Lease>(id)
        .await
        .unwrap()
        .unwrap()
        .value
}

#[tokio::test]
async fn idle_stop_releases_leases_and_is_repeatable() -> Result<()> {
    let context = fixture().await?;
    let processor = start(&context, options("first")).await?;
    processor.stop().await?;
    processor.stop().await?;
    assert!(!processor.is_running());
    assert!(processor.next_batch().await.is_none());
    assert!(context
        .list()
        .await?
        .iter()
        .all(|lease| lease.base.owner.is_none()));
    Ok(())
}

#[tokio::test]
async fn drain_renews_then_checkpoints_before_release_without_polling_again() -> Result<()> {
    let context = fixture().await?;
    insert(&context, "before").await?;
    let processor = start(&context, options("first")).await?;
    let (id, _, receipt) = next(&processor).await.into_parts();
    let before = record(&context, &id).await;
    let mut stopping = Box::pin(processor.stop());
    assert!(futures::poll!(stopping.as_mut()).is_pending());
    assert!(processor.next_batch().await.is_none());
    insert(&context, "after").await?;
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let current = record(&context, &id).await;
            assert_eq!(current.base.owner, before.base.owner);
            assert_eq!(current.incarnation, before.incarnation);
            assert_eq!(current.base.continuation, before.base.continuation);
            if current.renewed_at != before.renewed_at {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert!(futures::poll!(stopping.as_mut()).is_pending());
    receipt.checkpoint().await?;
    stopping.await?;
    let saved = record(&context, &id).await;
    assert!(saved.base.owner.is_none());
    assert!(saved.base.continuation.is_some());
    let restarted = start(&context, options("replacement")).await?;
    let (_, response, receipt) = next(&restarted).await.into_parts();
    let body: serde_json::Value = response.into_body().into_single()?;
    let documents = body["Documents"].as_array().unwrap();
    assert_eq!(documents.len(), 1);
    assert_eq!(documents[0]["current"]["id"], "after");
    receipt.checkpoint().await?;
    restarted.stop().await?;
    Ok(())
}

#[tokio::test]
async fn cancelled_wait_and_concurrent_stops_share_cleanup() -> Result<()> {
    let context = fixture().await?;
    insert(&context, "one").await?;
    let processor = start(&context, options("first")).await?;
    let (id, _, receipt) = next(&processor).await.into_parts();
    let mut stopping = Box::pin(processor.stop());
    assert!(futures::poll!(stopping.as_mut()).is_pending());
    drop(stopping);
    assert!(processor.is_running());
    receipt.checkpoint().await?;
    let (first, second) = futures::join!(processor.stop(), processor.stop());
    first?;
    second?;
    assert!(record(&context, &id).await.base.owner.is_none());
    assert!(record(&context, &id).await.base.continuation.is_some());
    Ok(())
}

#[tokio::test]
async fn abandoned_callback_reports_incomplete_drain_and_replays() -> Result<()> {
    let context = fixture().await?;
    insert(&context, "one").await?;
    let processor = start(&context, options("first")).await?;
    let (id, _, receipt) = next(&processor).await.into_parts();
    let mut stopping = Box::pin(processor.stop());
    assert!(futures::poll!(stopping.as_mut()).is_pending());
    drop(receipt);
    assert!(stopping.await.is_err());
    assert!(processor.stop().await.is_err());
    let saved = record(&context, &id).await;
    assert!(saved.base.owner.is_none());
    assert!(saved.base.continuation.is_none());
    let replacement = start(&context, options("replacement")).await?;
    let (replayed, _, receipt) = next(&replacement).await.into_parts();
    assert_eq!(replayed, id);
    receipt.checkpoint().await?;
    replacement.stop().await?;
    Ok(())
}

#[tokio::test]
async fn force_interrupts_drain_without_saving_pending_progress() -> Result<()> {
    let context = fixture().await?;
    insert(&context, "one").await?;
    let processor = start(&context, options("first")).await?;
    let (id, _, receipt) = next(&processor).await.into_parts();
    let mut stopping = Box::pin(processor.stop());
    assert!(futures::poll!(stopping.as_mut()).is_pending());
    processor.abort().await?;
    assert!(stopping.await.is_err());
    assert!(receipt.checkpoint().await.is_err());
    let saved = record(&context, &id).await;
    assert!(saved.base.owner.is_none());
    assert!(saved.base.continuation.is_none());
    Ok(())
}

#[tokio::test]
async fn ownership_loss_during_drain_never_clears_a_replacement_owner() -> Result<()> {
    let context = fixture().await?;
    insert(&context, "one").await?;
    let processor = start(&context, options("first")).await?;
    let (id, _, receipt) = next(&processor).await.into_parts();
    let observed = record(&context, &id).await;
    let mut stopping = Box::pin(processor.stop());
    assert!(futures::poll!(stopping.as_mut()).is_pending());
    let mut replacement = context.acquire(&observed).await?;
    assert_ne!(replacement.value.incarnation, observed.incarnation);
    tokio::time::timeout(Duration::from_secs(5), receipt.cancelled())
        .await
        .unwrap();
    assert!(receipt.checkpoint().await.is_err());
    assert!(stopping.await.is_err());
    let saved = record(&context, &id).await;
    assert_eq!(saved.incarnation, replacement.value.incarnation);
    assert_eq!(saved.base.owner, replacement.value.base.owner);
    assert_eq!(saved.base.continuation, observed.base.continuation);
    context.update(&mut replacement, Update::Release).await?;
    Ok(())
}

#[tokio::test]
async fn last_handle_drop_cancels_without_acknowledging() -> Result<()> {
    let context = fixture().await?;
    insert(&context, "one").await?;
    let processor = start(&context, options("first")).await?;
    let (id, _, receipt) = next(&processor).await.into_parts();
    drop(processor);
    tokio::time::timeout(Duration::from_secs(5), receipt.cancelled())
        .await
        .unwrap();
    assert!(receipt.checkpoint().await.is_err());
    tokio::time::timeout(Duration::from_secs(5), async {
        while record(&context, &id).await.base.owner.is_some() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert!(record(&context, &id).await.base.continuation.is_none());
    Ok(())
}

#[test]
fn expiration_boundary_and_maintenance_are_independent_of_poll_delay() {
    let all = leases(&[Some("first")]);
    let options = options("replacement");
    let expires = 1000 + i64::try_from(options.expiration.as_millis()).unwrap();
    assert!(balancing::select(&all, &options, expires).is_empty());
    assert_eq!(balancing::select(&all, &options, expires + 1), vec![0]);
    let busy = options.clone().with_poll_delay(Duration::ZERO);
    let idle = options.with_poll_delay(Duration::from_secs(3600));
    assert_eq!(busy.renew_interval(), idle.renew_interval());
    assert_eq!(busy.acquire_interval(), idle.acquire_interval());
    assert_eq!(busy.renew_interval() * 3, busy.expiration);
}

#[cfg(feature = "fault_injection")]
mod faults {
    use super::super::super::{run, signal, SharedState};
    use super::{
        fixture_with_emulator, insert, next, record, start, ChangeFeedProcessorMode, Context,
    };
    use crate::{
        fault_injection::{
            CustomResponseBuilder, FaultInjectionConditionBuilder, FaultInjectionErrorType,
            FaultInjectionResultBuilder, FaultInjectionRule, FaultInjectionRuleBuilder,
            FaultOperationType,
        },
        options::DriverOptions,
        Result,
    };
    use azure_core::http::StatusCode;
    use futures::task::AtomicWaker;
    use std::{
        sync::{atomic::AtomicBool, Arc},
        time::Duration,
    };

    async fn fixture(
        hit_limit: Option<u32>,
        after_service: bool,
    ) -> Result<(Arc<Context>, Arc<FaultInjectionRule>)> {
        let (mut context, emulator) =
            fixture_with_emulator(ChangeFeedProcessorMode::LatestVersion, 1).await?;
        let result = if after_service {
            FaultInjectionResultBuilder::new()
                .with_error(FaultInjectionErrorType::ResponseTimeoutAfterService)
        } else {
            FaultInjectionResultBuilder::new()
                .with_custom_response(CustomResponseBuilder::new(StatusCode::BadRequest).build())
        }
        .build();
        let rule = FaultInjectionRuleBuilder::new("lease-write-failure", result).with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::ReplaceItem)
                .build(),
        );
        let rule = Arc::new(match hit_limit {
            Some(limit) => rule.with_hit_limit(limit).build(),
            None => rule.build(),
        });
        rule.disable();
        let lease_driver = emulator
            .runtime_builder()
            .build()
            .await?
            .create_driver(
                DriverOptions::builder(context.source_driver.account().clone())
                    .with_fault_injection_rules(vec![rule.clone()])?
                    .build(),
            )
            .await?;
        let owned = Arc::get_mut(&mut context).expect("unshared fixture");
        owned.lease_driver = lease_driver;
        owned.options.expiration = Duration::from_secs(60);
        Ok((context, rule))
    }

    #[tokio::test]
    async fn failed_checkpoint_is_not_a_successful_drain() -> Result<()> {
        let (context, rule) = fixture(Some(1), false).await?;
        insert(&context, "one").await?;
        let processor = start(&context, context.options.clone()).await?;
        let (id, _, receipt) = next(&processor).await.into_parts();
        let mut stopping = Box::pin(processor.stop());
        assert!(futures::poll!(stopping.as_mut()).is_pending());
        rule.enable();
        assert_eq!(
            receipt
                .checkpoint()
                .await
                .unwrap_err()
                .status()
                .status_code(),
            StatusCode::BadRequest
        );
        assert!(stopping.await.is_err());
        assert_eq!(rule.hit_count(), 1);
        let saved = record(&context, &id).await;
        assert!(saved.base.continuation.is_none());
        assert!(saved.base.owner.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn failed_release_is_reported_and_keeps_expirable_ownership() -> Result<()> {
        let (context, rule) = fixture(None, false).await?;
        insert(&context, "one").await?;
        let processor = start(&context, context.options.clone()).await?;
        let (id, _, receipt) = next(&processor).await.into_parts();
        let before = record(&context, &id).await;
        let mut stopping = Box::pin(processor.stop());
        assert!(futures::poll!(stopping.as_mut()).is_pending());
        rule.enable();
        assert!(processor.abort().await.is_err());
        assert!(stopping.await.is_err());
        assert!(receipt.checkpoint().await.is_err());
        assert_eq!(rule.hit_count(), 2);
        let saved = record(&context, &id).await;
        assert_eq!(saved.base.owner, before.base.owner);
        assert_eq!(saved.incarnation, before.incarnation);
        assert_eq!(saved.base.continuation, before.base.continuation);
        Ok(())
    }

    #[tokio::test]
    async fn stop_retries_a_release_that_failed_before_shutdown() -> Result<()> {
        let (context, rule) = fixture(Some(1), false).await?;
        insert(&context, "one").await?;
        let processor = start(&context, context.options.clone()).await?;
        let (id, _, receipt) = next(&processor).await.into_parts();
        rule.enable();
        drop(receipt);
        tokio::time::timeout(Duration::from_secs(5), async {
            while !processor
                .owner
                .state
                .unreleased
                .lock()
                .unwrap()
                .values()
                .any(|lease| lease.base.record.id == id)
            {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        assert_eq!(rule.hit_count(), 1);
        assert_eq!(
            record(&context, &id).await.base.owner.as_deref(),
            Some("first")
        );
        processor.stop().await?;
        assert!(record(&context, &id).await.base.owner.is_none());
        assert!(record(&context, &id).await.base.continuation.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn lost_acquisition_response_does_not_leave_an_untracked_owner() -> Result<()> {
        let (context, rule) = fixture(Some(1), true).await?;
        let lease = context.list().await?.remove(0);
        let id = lease.base.record.id.clone();
        let state = Arc::new(SharedState {
            ready: Default::default(),
            waker: AtomicWaker::new(),
            running: AtomicBool::new(true),
            stopping: AtomicBool::new(false),
            forced: AtomicBool::new(false),
            failure: Default::default(),
            unreleased: Default::default(),
        });
        rule.enable();
        assert!(context.acquire_tracked(&lease, Some(&state)).await.is_err());
        assert_eq!(rule.hit_count(), 1);
        assert_eq!(
            record(&context, &id).await.base.owner.as_deref(),
            Some("first")
        );
        assert_eq!(state.unreleased.lock().unwrap().len(), 1);
        let (stop, stopping) = signal();
        let (_force, forced) = signal();
        state.close(false);
        stop.send(()).unwrap();
        assert!(!run(context.clone(), state, stopping, forced).await?);
        assert!(record(&context, &id).await.base.owner.is_none());
        assert!(record(&context, &id).await.base.continuation.is_none());
        Ok(())
    }
}
