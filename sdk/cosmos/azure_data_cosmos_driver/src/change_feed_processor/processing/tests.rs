// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    balancing, bootstrap, namespace, now_millis, ChangeFeedBatch, ChangeFeedFactoryMode, Context,
    Identity, Lease, ProcessingLease, Processor, ProcessorOptions, Update, SCHEMA_VERSION,
};
#[cfg(feature = "fault_injection")]
use crate::fault_injection::{
    CustomResponseBuilder, FaultInjectionConditionBuilder, FaultInjectionResultBuilder,
    FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
};
use crate::{
    change_feed_processor::bootstrap::{
        ChangeFeedProcessorBootstrapRangePlan as RangePlan,
        ChangeFeedProcessorBootstrapRecord as Record,
    },
    driver::CosmosDriverRuntimeBuilder,
    in_memory_emulator::{
        ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
    },
    models::{
        AccountReference, ChangeFeedStartFrom, ContinuationToken, CosmosOperation, FeedRange,
        ItemReference, MaxItemCountHint, PartitionKey,
    },
    options::{DriverOptions, OperationOptions},
    Result,
};
use azure_core::http::StatusCode;
use std::{num::NonZeroU32, sync::Arc, time::Duration};

mod topology;

const ENDPOINT: &str = "https://eastus.emulator.local";

fn options(worker: &str) -> ProcessorOptions {
    let mut options = ProcessorOptions::new(ChangeFeedFactoryMode::Incremental, "workload", worker)
        .with_start(ChangeFeedStartFrom::Beginning);
    options.acquire_interval = Duration::from_millis(20);
    options.poll_delay = Duration::from_millis(10);
    options
}

async fn fixture() -> Result<Arc<Context>> {
    Ok(fixture_with_emulator(ChangeFeedFactoryMode::Incremental, 3)
        .await?
        .0)
}

async fn fixture_with_emulator(
    mode: ChangeFeedFactoryMode,
    partitions: u32,
) -> Result<(Arc<Context>, Arc<InMemoryEmulatorHttpClient>)> {
    let emulator = emulator()?;
    let context =
        fixture_with_runtime(&emulator, emulator.runtime_builder(), mode, partitions).await?;
    Ok((context, emulator))
}

fn emulator() -> Result<Arc<InMemoryEmulatorHttpClient>> {
    Ok(Arc::new(InMemoryEmulatorHttpClient::new(
        VirtualAccountConfig::new(vec![VirtualRegion::new("East US", ENDPOINT.parse()?)])?,
    )))
}

async fn fixture_with_runtime(
    emulator: &InMemoryEmulatorHttpClient,
    runtime: CosmosDriverRuntimeBuilder,
    mode: ChangeFeedFactoryMode,
    partitions: u32,
) -> Result<Arc<Context>> {
    let store = emulator.store();
    store.create_database("polling");
    let pk =
        serde_json::from_value(serde_json::json!({"paths":["/pk"],"kind":"Hash","version":2}))?;
    store.create_container_with_config(
        "polling",
        "source",
        pk,
        ContainerConfig::default().with_partition_count(partitions),
    );
    store.create_container_with_properties_internal(
        "polling",
        "leases",
        serde_json::from_value(serde_json::json!({"paths":["/id"],"kind":"Hash","version":2}))?,
        ContainerConfig::default(),
        serde_json::from_value(serde_json::json!({"defaultTtl":-1}))?,
    );
    let driver = runtime
        .build()
        .await?
        .create_driver(
            DriverOptions::builder(AccountReference::with_account_key(
                ENDPOINT.parse()?,
                "dGVzdGtleQ==",
            ))
            .build(),
        )
        .await?;
    let source = driver
        .resolve_container("polling", "source", OperationOptions::default())
        .await?;
    let leases = driver
        .resolve_container("polling", "leases", OperationOptions::default())
        .await?;
    let mut options = options("first");
    options.mode = mode;
    if mode == ChangeFeedFactoryMode::FullFidelity {
        options.start = ChangeFeedStartFrom::Now;
    }
    bootstrap(
        &driver,
        &source,
        &driver,
        &leases,
        options.mode,
        &options.processor_name,
        &options.instance_name,
    )
    .await?;
    let identity = Identity {
        version: SCHEMA_VERSION,
        account: source.account().endpoint().origin().ascii_serialization(),
        container_rid: source.rid().into(),
        processor_name: "workload".into(),
        mode,
    };
    Ok(Arc::new(Context {
        source_driver: driver.clone(),
        source,
        lease_driver: driver,
        leases,
        namespace: namespace(&identity)?,
        identity,
        options,
    }))
}

#[cfg(feature = "fault_injection")]
async fn fault_fixture(
    operation: FaultOperationType,
    status: StatusCode,
    hit_limit: u32,
) -> Result<(Arc<Context>, Arc<FaultInjectionRule>)> {
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "lease-mutation",
            FaultInjectionResultBuilder::new()
                .with_custom_response(
                    CustomResponseBuilder::new(status)
                        .with_body(br#"{"message":"injected lease failure"}"#.to_vec())
                        .build(),
                )
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(operation)
                .build(),
        )
        .with_hit_limit(hit_limit)
        .build(),
    );
    rule.disable();
    let emulator = emulator()?;
    let context = fixture_with_runtime(
        &emulator,
        emulator.runtime_builder_with_fault_rules(vec![rule.clone()]),
        ChangeFeedFactoryMode::Incremental,
        3,
    )
    .await?;
    Ok((context, rule))
}

async fn start(context: &Context, options: ProcessorOptions) -> Result<Processor> {
    Processor::start(
        context.source_driver.clone(),
        context.source.clone(),
        context.lease_driver.clone(),
        context.leases.clone(),
        options,
    )
    .await
}

async fn insert(context: &Context, id: &str) -> Result<()> {
    context
        .source_driver
        .execute_singleton_operation(
            CosmosOperation::create_item(ItemReference::from_name(
                &context.source,
                PartitionKey::from("same"),
                id.to_owned(),
            ))
            .with_body(serde_json::to_vec(
                &serde_json::json!({"id":id,"pk":"same"}),
            )?),
            OperationOptions::default(),
        )
        .await?;
    Ok(())
}

async fn next(processor: &Processor) -> ChangeFeedBatch {
    tokio::time::timeout(Duration::from_secs(5), processor.next_batch())
        .await
        .expect("batch timeout")
        .expect("processor stopped")
}

#[tokio::test]
async fn concurrent_restart_verification_preserves_existing_progress() -> Result<()> {
    let context = fixture().await?;
    let before = context.list().await?;
    let kept_id = &before[0].base.record.id;
    let missing_id = &before[1].base.record.id;
    let store = context.store();
    let mut kept = store.read::<Lease>(kept_id).await?.unwrap();
    kept.value.base.owner = Some("existing-worker".into());
    kept.value.base.continuation = Some(ContinuationToken::from_string("saved-position".into()));
    let kept = store.replace(kept_id, &kept.value, kept.etag).await?;
    store
        .execute(CosmosOperation::delete_item(ItemReference::from_name(
            &context.leases,
            PartitionKey::from(missing_id.clone()),
            missing_id.clone(),
        )))
        .await?;
    assert!(context.list().await.err().unwrap().status().is_not_found());
    let (left, right) = futures::join!(context.read_leases(true), context.read_leases(true));
    for leases in [left?, right?] {
        assert_eq!(leases.len(), before.len());
        assert_eq!(
            serde_json::to_value(
                leases
                    .iter()
                    .find(|lease| &lease.base.record.id == kept_id)
                    .unwrap()
            )?,
            serde_json::to_value(&kept.value)?,
        );
        assert_eq!(
            serde_json::to_value(
                leases
                    .iter()
                    .find(|lease| &lease.base.record.id == missing_id)
                    .unwrap()
            )?,
            serde_json::to_value(&before[1])?,
        );
    }
    Ok(())
}

#[tokio::test]
async fn restart_verification_rejects_incomplete_range_plan() -> Result<()> {
    let context = fixture().await?;
    let id = format!("{}.plan", context.namespace);
    let store = context.store();
    let mut plan = store.read::<RangePlan>(&id).await?.unwrap();
    plan.value.ranges.pop();
    store.replace(&id, &plan.value, plan.etag).await?;
    assert!(context
        .read_leases(true)
        .await
        .err()
        .unwrap()
        .status()
        .is_conflict());
    Ok(())
}

fn leases(owners: &[Option<&str>]) -> Vec<Lease> {
    owners
        .iter()
        .enumerate()
        .map(|(index, owner)| Lease {
            base: ProcessingLease {
                record: Record {
                    id: index.to_string(),
                    workload: Identity {
                        version: 1,
                        account: "account".into(),
                        container_rid: "rid".into(),
                        processor_name: "workload".into(),
                        mode: ChangeFeedFactoryMode::Incremental,
                    },
                    ttl: -1,
                },
                range: FeedRange::full(),
                owner: owner.map(str::to_owned),
                continuation: None,
            },
            incarnation: Some(index.to_string()),
            renewed_at: Some(1000),
            children: None,
        })
        .collect()
}

#[test]
fn equal_balancing_scale_out_down_and_limits() {
    let all = leases(&[Some("a"), Some("a"), Some("a"), Some("a")]);
    let selected = balancing::select(&all, &options("b"), 1000);
    assert_eq!(selected.len(), 1);
    assert_eq!(all[selected[0]].base.owner.as_deref(), Some("a"));
    let balanced = leases(&[Some("a"), Some("a"), Some("b"), Some("b")]);
    assert!(balancing::select(&balanced, &options("a"), 1000).is_empty());
    let mut expired = balanced;
    expired[2].renewed_at = Some(-100_000);
    expired[3].renewed_at = Some(-100_000);
    assert_eq!(balancing::select(&expired, &options("a"), 1000).len(), 2);
    assert!(balancing::select(&expired, &options("a").with_lease_counts(0, 2), 1000).is_empty());
    assert_eq!(
        balancing::select(&leases(&[None, None, None]), &options("a"), 1000).len(),
        3
    );
}

#[test]
fn donor_threshold_does_not_oscillate_uneven_distribution() {
    let all = leases(&[Some("a"), Some("a"), Some("a"), Some("b"), Some("b")]);
    assert!(balancing::select(&all, &options("b"), 1000).is_empty());
    assert_eq!(balancing::select(&all, &options("c"), 1000).len(), 1);
}

#[tokio::test]
async fn workers_converge_on_scale_out_and_reclaim_on_scale_down() -> Result<()> {
    let context = fixture().await?;
    let mut first_options = options("first");
    first_options.renew_interval = Duration::from_millis(20);
    let first = start(&context, first_options).await?;
    let wait_counts = |first_count, second_count| {
        let context = context.clone();
        async move {
            tokio::time::timeout(Duration::from_secs(5), async {
                loop {
                    let leases = context.list().await.unwrap();
                    let first = leases
                        .iter()
                        .filter(|lease| lease.base.owner.as_deref() == Some("first"))
                        .count();
                    let second = leases
                        .iter()
                        .filter(|lease| lease.base.owner.as_deref() == Some("second"))
                        .count();
                    if (first, second) == (first_count, second_count) {
                        break;
                    }
                    tokio::time::sleep(Duration::from_millis(5)).await;
                }
            })
            .await
            .unwrap();
        }
    };
    wait_counts(3, 0).await;
    let mut second_options = options("second");
    second_options.renew_interval = Duration::from_millis(20);
    let second = start(&context, second_options).await?;
    wait_counts(2, 1).await;
    second.stop().await;
    wait_counts(3, 0).await;
    first.stop().await;
    Ok(())
}

#[tokio::test]
async fn stealing_fences_old_renewal_and_checkpoint() -> Result<()> {
    let context = fixture().await?;
    let lease = context.list().await?.remove(0);
    let mut first = context.acquire(&lease).await?;
    let observed = first.value.clone();
    let mut second_context = Context {
        source_driver: context.source_driver.clone(),
        source: context.source.clone(),
        lease_driver: context.lease_driver.clone(),
        leases: context.leases.clone(),
        identity: context.identity.clone(),
        namespace: context.namespace.clone(),
        options: options("second"),
    };
    let second = second_context.acquire(&observed).await?;
    let checkpoint = ContinuationToken::from_string("stale-checkpoint".into());
    let original = serde_json::to_value(&first.value)?;
    let original_etag = first.etag.clone();
    for update in [
        Update::Renew,
        Update::Checkpoint(&checkpoint),
        Update::Release,
    ] {
        assert!(context
            .update(&mut first, update)
            .await
            .unwrap_err()
            .status()
            .is_precondition_failed());
    }
    let stored = context
        .store()
        .read::<Lease>(&observed.base.record.id)
        .await?
        .unwrap();
    assert_eq!(stored.etag, second.etag);
    assert_eq!(
        serde_json::to_value(&stored.value)?,
        serde_json::to_value(&second.value)?
    );
    // Reacquiring under the old instance name must not revive the old receipt.
    second_context.options.instance_name = "first".into();
    let third = second_context.acquire(&second.value).await?;
    assert_ne!(first.value.incarnation, third.value.incarnation);
    for update in [
        Update::Renew,
        Update::Checkpoint(&checkpoint),
        Update::Release,
    ] {
        assert!(context
            .update(&mut first, update)
            .await
            .unwrap_err()
            .status()
            .is_precondition_failed());
    }
    assert_eq!(first.etag, original_etag);
    assert_eq!(serde_json::to_value(&first.value)?, original);
    let stored = context
        .store()
        .read::<Lease>(&observed.base.record.id)
        .await?
        .unwrap();
    assert_eq!(stored.etag, third.etag);
    assert_eq!(
        serde_json::to_value(&stored.value)?,
        serde_json::to_value(&third.value)?
    );
    Ok(())
}

#[tokio::test]
async fn lease_mutations_preserve_fresh_progress() -> Result<()> {
    let context = fixture().await?;
    let observed = context.list().await?.remove(0);
    let id = &observed.base.record.id;
    let store = context.store();
    let mut stored = store.read::<Lease>(id).await?.unwrap();
    stored.value.base.continuation = Some(ContinuationToken::from_string("saved".into()));
    let saved = store.replace(id, &stored.value, stored.etag).await?;
    let mut owned = context.acquire(&observed).await?;
    assert_ne!(owned.etag, saved.etag);
    assert_eq!(
        owned.value.base.continuation.as_ref().unwrap().as_str(),
        "saved"
    );

    let mut concurrent = owned.value.clone();
    concurrent.base.continuation = Some(ContinuationToken::from_string("newer".into()));
    let concurrent = store.replace(id, &concurrent, owned.etag.clone()).await?;
    context.update(&mut owned, Update::Renew).await?;
    assert_ne!(owned.etag, concurrent.etag);
    let mut expected = concurrent.value;
    expected.renewed_at = owned.value.renewed_at;
    assert_eq!(
        serde_json::to_value(&owned.value)?,
        serde_json::to_value(&expected)?
    );

    let checkpoint = ContinuationToken::from_string("processed".into());
    context
        .update(&mut owned, Update::Checkpoint(&checkpoint))
        .await?;
    expected.base.continuation = Some(checkpoint);
    expected.renewed_at = owned.value.renewed_at;
    assert_eq!(
        serde_json::to_value(&owned.value)?,
        serde_json::to_value(&expected)?
    );

    context.update(&mut owned, Update::Release).await?;
    expected.base.owner = None;
    expected.renewed_at = owned.value.renewed_at;
    assert_eq!(
        serde_json::to_value(&owned.value)?,
        serde_json::to_value(&expected)?
    );
    let persisted = store.read::<Lease>(id).await?.unwrap();
    assert_eq!(persisted.etag, owned.etag);
    assert_eq!(
        serde_json::to_value(&persisted.value)?,
        serde_json::to_value(&expected)?
    );
    Ok(())
}

#[tokio::test]
async fn lease_mutations_reject_incompatible_records() -> Result<()> {
    for invalid_ttl in [false, true] {
        let context = fixture().await?;
        let observed = context.list().await?.remove(0);
        let mut owned = context.acquire(&observed).await?;
        let original = serde_json::to_value(&owned.value)?;
        let original_etag = owned.etag.clone();
        let mut invalid = owned.value.clone();
        if invalid_ttl {
            invalid.base.record.ttl = 30;
        } else {
            invalid.base.record.workload.processor_name = "another-workload".into();
        }
        let stored = context
            .store()
            .replace(&invalid.base.record.id, &invalid, owned.etag.clone())
            .await?;
        assert_eq!(
            context
                .acquire(&owned.value)
                .await
                .err()
                .unwrap()
                .status()
                .status_code(),
            StatusCode::Conflict
        );
        let checkpoint = ContinuationToken::from_string("rejected".into());
        for update in [
            Update::Renew,
            Update::Checkpoint(&checkpoint),
            Update::Release,
        ] {
            assert_eq!(
                context
                    .update(&mut owned, update)
                    .await
                    .unwrap_err()
                    .status()
                    .status_code(),
                StatusCode::Conflict
            );
        }
        assert_eq!(owned.etag, original_etag);
        assert_eq!(serde_json::to_value(&owned.value)?, original);
        let persisted = context
            .store()
            .read::<Lease>(&invalid.base.record.id)
            .await?
            .unwrap();
        assert_eq!(persisted.etag, stored.etag);
        assert_eq!(
            serde_json::to_value(&persisted.value)?,
            serde_json::to_value(&invalid)?
        );
    }
    Ok(())
}

#[cfg(feature = "fault_injection")]
#[tokio::test]
async fn acquisition_does_not_retry_write_conflicts() -> Result<()> {
    let (context, rule) = fault_fixture(
        FaultOperationType::ReplaceItem,
        StatusCode::PreconditionFailed,
        10,
    )
    .await?;
    let observed = context.list().await?.remove(0);
    let before = context
        .store()
        .read::<Lease>(&observed.base.record.id)
        .await?
        .unwrap();
    rule.enable();
    let error = context.acquire(&observed).await.err().unwrap();
    rule.disable();
    assert_eq!(error.status().status_code(), StatusCode::PreconditionFailed);
    assert!(error.to_string().contains("injected lease failure"));
    assert_eq!(rule.hit_count(), 1);
    let after = context
        .store()
        .read::<Lease>(&observed.base.record.id)
        .await?
        .unwrap();
    assert_eq!(after.etag, before.etag);
    assert_eq!(
        serde_json::to_value(&after.value)?,
        serde_json::to_value(&before.value)?
    );
    Ok(())
}

#[cfg(feature = "fault_injection")]
#[tokio::test]
async fn owned_mutations_retry_only_conditional_write_conflicts() -> Result<()> {
    let checkpoint = ContinuationToken::from_string("processed".into());
    for (operation, status, limit, hits, succeeds) in [
        (
            FaultOperationType::ReplaceItem,
            StatusCode::PreconditionFailed,
            4,
            4,
            true,
        ),
        (
            FaultOperationType::ReplaceItem,
            StatusCode::PreconditionFailed,
            10,
            5,
            false,
        ),
        (
            FaultOperationType::ReadItem,
            StatusCode::PreconditionFailed,
            10,
            1,
            false,
        ),
        (
            FaultOperationType::ReplaceItem,
            StatusCode::BadRequest,
            10,
            1,
            false,
        ),
    ] {
        for update in [
            Update::Renew,
            Update::Checkpoint(&checkpoint),
            Update::Release,
        ] {
            let (context, rule) = fault_fixture(operation, status, limit).await?;
            let observed = context.list().await?.remove(0);
            let mut owned = context.acquire(&observed).await?;
            let before = serde_json::to_value(&owned.value)?;
            let before_etag = owned.etag.clone();
            let mut expected = owned.value.clone();
            match update {
                Update::Checkpoint(token) => expected.base.continuation = Some(token.clone()),
                Update::Release => expected.base.owner = None,
                Update::Renew => {}
                Update::Acquire | Update::Split(_) => unreachable!(),
            }
            rule.enable();
            let result = context.update(&mut owned, update).await;
            rule.disable();
            assert_eq!(rule.hit_count(), hits);
            if succeeds {
                result?;
                assert_ne!(owned.etag, before_etag);
                expected.renewed_at = owned.value.renewed_at;
                assert_eq!(
                    serde_json::to_value(&owned.value)?,
                    serde_json::to_value(&expected)?
                );
            } else {
                let error = result.unwrap_err();
                assert_eq!(error.status().status_code(), status);
                if hits == 5 {
                    assert!(error
                        .to_string()
                        .contains("Lease update conflicts exhausted the retry budget."));
                } else {
                    assert!(error.to_string().contains("injected lease failure"));
                }
                assert_eq!(owned.etag, before_etag);
                assert_eq!(serde_json::to_value(&owned.value)?, before);
            }
            let persisted = context
                .store()
                .read::<Lease>(&owned.value.base.record.id)
                .await?
                .unwrap();
            assert_eq!(persisted.etag, owned.etag);
            assert_eq!(
                serde_json::to_value(&persisted.value)?,
                serde_json::to_value(&owned.value)?
            );
        }
    }
    Ok(())
}

#[tokio::test]
async fn callback_receipt_gates_next_page_and_saved_progress() -> Result<()> {
    let context = fixture().await?;
    insert(&context, "one").await?;
    insert(&context, "two").await?;
    let opts =
        options("first").with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap()));
    let processor = start(&context, opts).await?;
    let (id, _, receipt) = next(&processor).await.into_parts();
    let stored = context.store().read::<Lease>(&id).await?.unwrap();
    assert!(stored.value.base.continuation.is_none());
    assert!(
        tokio::time::timeout(Duration::from_millis(60), processor.next_batch())
            .await
            .is_err()
    );
    receipt.checkpoint().await?;
    let saved = context
        .store()
        .read::<Lease>(&id)
        .await?
        .unwrap()
        .value
        .base
        .continuation
        .unwrap();
    let (second_id, _, second_receipt) = next(&processor).await.into_parts();
    assert_eq!(id, second_id);
    assert_eq!(
        context
            .store()
            .read::<Lease>(&id)
            .await?
            .unwrap()
            .value
            .base
            .continuation
            .unwrap()
            .as_str(),
        saved.as_str()
    );
    drop(second_receipt);
    processor.stop().await;
    assert!(!processor.is_running());
    let restarted = start(
        &context,
        options("first").with_start(ChangeFeedStartFrom::Now),
    )
    .await?;
    let (resumed_id, _, resumed_receipt) = next(&restarted).await.into_parts();
    assert_eq!(resumed_id, id);
    resumed_receipt.checkpoint().await?;
    restarted.stop().await;
    Ok(())
}

#[tokio::test]
async fn watchdog_cancels_receipt_without_saving_progress() -> Result<()> {
    let context = fixture().await?;
    insert(&context, "one").await?;
    let mut opts = options("first");
    opts.renew_interval = Duration::from_millis(10);
    opts.acquire_interval = Duration::from_secs(2);
    let processor = start(&context, opts).await?;
    let (id, _, receipt) = next(&processor).await.into_parts();
    tokio::time::timeout(Duration::from_secs(2), receipt.cancelled())
        .await
        .unwrap();
    assert!(receipt
        .checkpoint()
        .await
        .unwrap_err()
        .status()
        .is_precondition_failed());
    assert!(context
        .store()
        .read::<Lease>(&id)
        .await?
        .unwrap()
        .value
        .base
        .continuation
        .is_none());
    processor.stop().await;
    Ok(())
}

#[tokio::test]
async fn now_start_pins_idle_position_before_new_changes() -> Result<()> {
    let context = fixture().await?;
    insert(&context, "before").await?;
    let processor = start(
        &context,
        options("first").with_start(ChangeFeedStartFrom::Now),
    )
    .await?;
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let all = context.list().await.unwrap();
            if all.iter().all(|lease| lease.base.continuation.is_some()) {
                break;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(30), processor.next_batch())
            .await
            .is_err()
    );
    insert(&context, "after").await?;
    let (_, response, receipt) = next(&processor).await.into_parts();
    let envelope: serde_json::Value = response.into_body().into_single()?;
    let items = envelope["Documents"]
        .as_array()
        .expect("change feed envelope");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["current"]["id"], "after");
    receipt.checkpoint().await?;
    processor.stop().await;
    assert!(context
        .list()
        .await?
        .iter()
        .all(|lease| lease.base.owner.is_none()));
    assert!(now_millis() > 0);
    Ok(())
}
