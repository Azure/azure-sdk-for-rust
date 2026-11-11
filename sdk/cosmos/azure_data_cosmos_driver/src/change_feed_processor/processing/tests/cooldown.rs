// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    emulator, fixture_with_runtime, start,
    topology::{keys, publish},
    ChangeFeedFactoryMode, CheckpointPolicy, Context, Lease,
};
use crate::{
    change_feed_processor::processing::retry::ReadRetry,
    fault_injection::{
        CustomResponseBuilder, FaultInjectionConditionBuilder, FaultInjectionResultBuilder,
        FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
    },
    models::MaxItemCountHint,
    options::{OperationOptions, PlanOptions, ThrottlingRetryOptions},
    Result,
};
use azure_core::http::StatusCode;
use std::{
    collections::BTreeMap,
    num::NonZeroU32,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

fn throttle(delay_ms: u64, hits: u32) -> Arc<FaultInjectionRule> {
    Arc::new(
        FaultInjectionRuleBuilder::new(
            "read-throttle",
            FaultInjectionResultBuilder::new()
                .with_custom_response(
                    CustomResponseBuilder::new(StatusCode::TooManyRequests)
                        .with_header("x-ms-retry-after-ms", delay_ms.to_string())
                        .with_body(br#"{"message":"throttled"}"#.to_vec())
                        .build(),
                )
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::ChangeFeedItem)
                .build(),
        )
        .with_hit_limit(hits)
        .build(),
    )
}

async fn fixture(
    rule: Arc<FaultInjectionRule>,
    options: OperationOptions,
    partitions: u32,
) -> Result<Arc<Context>> {
    let emulator = emulator()?;
    let runtime = emulator
        .runtime_builder_with_fault_rules(vec![rule])
        .with_default_operation_options(options);
    let mut context = fixture_with_runtime(
        &emulator,
        runtime,
        ChangeFeedFactoryMode::Incremental,
        partitions,
    )
    .await?;
    Arc::get_mut(&mut context).unwrap().options.page_size =
        MaxItemCountHint::Limit(NonZeroU32::MIN);
    let ranges = context
        .list()
        .await?
        .into_iter()
        .map(|l| l.base.range)
        .collect::<Vec<_>>();
    publish(&context, &keys(&ranges), 0..1, &mut BTreeMap::new()).await?;
    Ok(context)
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
async fn throttled_lease_renews_while_single_receive_serves_another_and_stop_interrupts_cooldown(
) -> Result<()> {
    let rule = throttle(10_000, 1);
    let context = fixture(rule.clone(), OperationOptions::default(), 2).await?;
    let processor = start(&context, context.options.clone()).await?;
    let batch = tokio::time::timeout(Duration::from_secs(1), processor.next_batch())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(rule.hit_count(), 1);
    let (healthy, _, receipt) = batch.into_parts();
    let leases = context.list().await?;
    let slow = leases.iter().find(|l| l.base.record.id != healthy).unwrap();
    let id = slow.base.record.id.clone();
    let renewed = slow.renewed_at;
    assert!(slow.base.owner.is_some());
    assert!(slow.base.continuation.is_none());
    tokio::time::timeout(Duration::from_secs(1), async {
        while record(&context, &id).await.renewed_at == renewed {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert_eq!(rule.hit_count(), 1);
    assert!(record(&context, &id).await.base.continuation.is_none());
    let mut stopping = Box::pin(processor.stop());
    assert!(futures::poll!(stopping.as_mut()).is_pending());
    receipt.checkpoint().await?;
    tokio::time::timeout(Duration::from_secs(1), stopping)
        .await
        .unwrap()?;
    assert!(record(&context, &id).await.base.owner.is_none());
    assert!(record(&context, &id).await.base.continuation.is_none());
    Ok(())
}

#[tokio::test]
async fn exhausted_budget_finishes_final_cooldown_before_reporting_and_releasing() -> Result<()> {
    let rule = throttle(80, u32::MAX);
    let options = OperationOptions {
        throttling_retry_options: Some(ThrottlingRetryOptions {
            max_retry_count: Some(2),
            max_retry_wait_time: Some(Duration::from_secs(10)),
        }),
        ..Default::default()
    };
    let context = fixture(rule.clone(), options, 1).await?;
    let reported = Arc::new(Mutex::new(Vec::new()));
    let reports = reported.clone();
    let processor = start(
        &context,
        context
            .options
            .clone()
            .with_error_observer(move |lease, error| {
                reports
                    .lock()
                    .unwrap()
                    .push((lease, error.status(), Instant::now()));
            }),
    )
    .await?;
    let started = Instant::now();
    let mut receiving = Box::pin(processor.next_batch());
    let (id, status, ended) = tokio::time::timeout(Duration::from_secs(3), async {
        tokio::select! {
            _ = &mut receiving => panic!("a throttled page must not be delivered"),
            reported = async {
                loop {
                    if let Some(report) = reported.lock().unwrap().first().cloned() { break report; }
                    tokio::task::yield_now().await;
                }
            } => reported,
        }
    }).await.unwrap();
    assert!(status.is_throttled());
    assert_eq!(rule.hit_count(), 3);
    assert!(ended.duration_since(started) >= Duration::from_millis(240));
    processor.stop().await?;
    assert!(receiving.await.is_none());
    let lease = record(&context, &id.unwrap()).await;
    assert!(lease.base.owner.is_none());
    assert!(lease.base.continuation.is_none());
    Ok(())
}

#[tokio::test]
async fn cooldown_keeps_prior_success_checkpoint_deadline_active() -> Result<()> {
    let rule = throttle(10_000, 1);
    rule.disable();
    let context = fixture(rule.clone(), OperationOptions::default(), 1).await?;
    let options = context
        .options
        .clone()
        .with_checkpoint_policy(CheckpointPolicy::Coalesced {
            max_interval: Duration::from_millis(50),
        });
    let processor = start(&context, options).await?;
    let (id, _, receipt) = processor.next_batch().await.unwrap().into_parts();
    receipt.acknowledge().await?;
    rule.enable();
    let mut receiving = Box::pin(processor.next_batch());
    tokio::time::timeout(Duration::from_secs(1), async {
        tokio::select! {
            _ = &mut receiving => panic!("cooldown must not deliver the next page"),
            () = async {
                loop {
                    if rule.hit_count() == 1 && record(&context, &id).await.base.continuation.is_some() {
                        break;
                    }
                    tokio::task::yield_now().await;
                }
            } => {},
        }
    }).await.unwrap();
    let saved = record(&context, &id).await.base.continuation;
    drop(receiving);
    tokio::time::timeout(Duration::from_secs(1), processor.stop())
        .await
        .unwrap()?;
    assert_eq!(record(&context, &id).await.base.continuation, saved);
    assert_eq!(rule.hit_count(), 1);
    Ok(())
}

#[tokio::test]
async fn read_deadline_yields_demand_and_preserves_failed_acquisitions_progress() -> Result<()> {
    for configured in [None, Some(Duration::from_secs(1))] {
        let rule = Arc::new(
            FaultInjectionRuleBuilder::new(
                "slow-read",
                FaultInjectionResultBuilder::new()
                    .with_delay(Duration::from_secs(10))
                    .build(),
            )
            .with_condition(
                FaultInjectionConditionBuilder::new()
                    .with_operation_type(FaultOperationType::ChangeFeedItem)
                    .build(),
            )
            .with_hit_limit(1)
            .build(),
        );
        let context = fixture(
            rule.clone(),
            OperationOptions {
                end_to_end_latency_policy: configured.map(Into::into),
                ..Default::default()
            },
            2,
        )
        .await?;
        let (_, _, timeout) = ReadRetry::for_driver(&context.source_driver);
        assert_eq!(timeout, configured.unwrap_or(Duration::from_secs(5)));
        let reported = Arc::new(Mutex::new(Vec::new()));
        let reports = reported.clone();
        let processor = start(
            &context,
            context
                .options
                .clone()
                .with_error_observer(move |lease, error| {
                    reports.lock().unwrap().push((lease, error.status()));
                }),
        )
        .await?;
        let batch = tokio::time::timeout(timeout + Duration::from_secs(2), processor.next_batch())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(rule.hit_count(), 1);
        let (healthy, _, receipt) = batch.into_parts();
        let (slow, status) = reported
            .lock()
            .unwrap()
            .first()
            .cloned()
            .expect("timed-out acquisition reported");
        let slow = slow.unwrap();
        assert_ne!(healthy, slow);
        assert_eq!(status.status_code(), StatusCode::RequestTimeout);
        assert!(record(&context, &slow).await.base.continuation.is_none());
        let mut stopping = Box::pin(processor.stop());
        assert!(futures::poll!(stopping.as_mut()).is_pending());
        receipt.checkpoint().await?;
        tokio::time::timeout(Duration::from_secs(1), stopping)
            .await
            .unwrap()?;
    }
    Ok(())
}

#[tokio::test]
async fn processor_throttle_override_does_not_change_ordinary_feed_retries() -> Result<()> {
    let rule = throttle(10, 1);
    let context = fixture(rule.clone(), OperationOptions::default(), 1).await?;
    let lease = context.list().await?.remove(0);
    let operation = context.operation(&lease.base.range);
    let mut plan = context
        .source_driver
        .plan_operation(
            operation,
            &OperationOptions::default(),
            None,
            &PlanOptions::default(),
        )
        .await?;
    let response = context
        .source_driver
        .execute_plan(
            &mut plan,
            Some(context.source.clone()),
            OperationOptions::default(),
        )
        .await?
        .unwrap();
    assert!(response.status().status_code().is_success());
    assert_eq!(rule.hit_count(), 1);
    Ok(())
}
