// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    bootstrap, namespace, ChangeFeedFactoryMode, ChangeFeedProcessorBootstrapCoalescingKey,
    ChangeFeedProcessorBootstrapContext, ChangeFeedProcessorBootstrapLockRecord,
    ChangeFeedProcessorBootstrapProcessingLease, ChangeFeedProcessorBootstrapRangePlan,
    ChangeFeedProcessorBootstrapRecord, LOCK_TTL_SECONDS,
};
use crate::{
    in_memory_emulator::{
        ContainerConfig, InMemoryEmulatorHttpClient, RequestObserver, VirtualAccountConfig,
        VirtualRegion, WriteMode,
    },
    models::{AccountReference, ContainerReference, ContinuationToken, FeedRange},
    options::{DriverOptions, OperationOptions},
    CosmosDriver, Result,
};
use azure_core::http::{headers::HeaderName, Method, Request, StatusCode};
use futures::{channel::oneshot, future::join_all, poll, FutureExt};
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

const ENDPOINT: &str = "https://eastus.emulator.local";

#[derive(Debug, Default)]
struct Requests {
    writes: AtomicUsize,
    lease_reads: AtomicUsize,
    plan_reads: AtomicUsize,
    change_feed_reads: AtomicUsize,
}

impl RequestObserver for Requests {
    fn on_request(&self, request: &Request) {
        if matches!(
            request.method(),
            Method::Post | Method::Put | Method::Delete
        ) {
            self.writes.fetch_add(1, Ordering::SeqCst);
        }
        if request.method() == Method::Get {
            let path = request.url().path();
            if path.contains(".lease.") {
                self.lease_reads.fetch_add(1, Ordering::SeqCst);
            }
            if path.ends_with(".plan") {
                self.plan_reads.fetch_add(1, Ordering::SeqCst);
            }
        }
        if request.url().path().ends_with("/docs")
            && request
                .headers()
                .get_optional_str(&HeaderName::from_static("a-im"))
                .is_some()
        {
            self.change_feed_reads.fetch_add(1, Ordering::SeqCst);
        }
    }
}

struct Fixture {
    emulator: Arc<InMemoryEmulatorHttpClient>,
    driver: Arc<CosmosDriver>,
    source: ContainerReference,
    leases: ContainerReference,
    requests: Arc<Requests>,
}

impl Fixture {
    async fn new(ttl: Option<i32>, partition_key: &str) -> Result<Self> {
        let requests = Arc::new(Requests::default());
        let emulator = Arc::new(
            InMemoryEmulatorHttpClient::new(VirtualAccountConfig::new(vec![VirtualRegion::new(
                "East US",
                ENDPOINT.parse()?,
            )])?)
            .with_request_observer(requests.clone()),
        );
        let store = emulator.store();
        store.create_database("bootstrap");
        store.create_container_with_config(
            "bootstrap",
            "source",
            serde_json::from_value(serde_json::json!({"paths":["/pk"],"kind":"Hash","version":2}))?,
            ContainerConfig::default().with_partition_count(3),
        );
        let mut properties = serde_json::Map::new();
        if let Some(ttl) = ttl {
            properties.insert("defaultTtl".into(), ttl.into());
        }
        store.create_container_with_properties_internal(
            "bootstrap",
            "leases",
            serde_json::from_value(
                serde_json::json!({"paths":[partition_key],"kind":"Hash","version":2}),
            )?,
            ContainerConfig::default(),
            properties,
        );
        let driver = Self::new_driver(&emulator).await?;
        let source = driver
            .resolve_container("bootstrap", "source", OperationOptions::default())
            .await?;
        let leases = driver
            .resolve_container("bootstrap", "leases", OperationOptions::default())
            .await?;
        Ok(Self {
            emulator,
            driver,
            source,
            leases,
            requests,
        })
    }

    async fn new_driver(emulator: &Arc<InMemoryEmulatorHttpClient>) -> Result<Arc<CosmosDriver>> {
        let runtime = emulator.runtime_builder().build().await?;
        runtime
            .create_driver(
                DriverOptions::builder(AccountReference::with_account_key(
                    ENDPOINT.parse()?,
                    "dGVzdGtleQ==",
                ))
                .build(),
            )
            .await
    }

    fn context(
        &self,
        mode: ChangeFeedFactoryMode,
    ) -> Result<ChangeFeedProcessorBootstrapContext<'_>> {
        let mut context = ChangeFeedProcessorBootstrapContext::new(
            &self.driver,
            &self.source,
            &self.driver,
            &self.leases,
            mode,
            "orders",
            "worker",
        )?;
        context.store.deadline = Instant::now() + Duration::from_secs(10);
        context.retry_delay = Duration::from_millis(5);
        Ok(context)
    }
}

async fn plan(
    context: &ChangeFeedProcessorBootstrapContext<'_>,
) -> Result<ChangeFeedProcessorBootstrapRangePlan> {
    Ok(context
        .store
        .read::<ChangeFeedProcessorBootstrapRangePlan>(&context.record("plan").id)
        .await?
        .unwrap()
        .value)
}

fn lease_id(context: &ChangeFeedProcessorBootstrapContext<'_>, range: &FeedRange) -> String {
    context
        .record(&format!(
            "lease.{}-{}",
            range.min_inclusive(),
            range.max_exclusive()
        ))
        .id
}

#[tokio::test]
async fn concurrent_workers_create_unowned_leases_without_reading_changes() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let mut workers = Vec::new();
    for _ in 0..4 {
        workers.push(Fixture::new_driver(&fixture.emulator).await?);
    }
    let worker_names: Vec<_> = (0..workers.len())
        .map(|index| format!("worker-{index}"))
        .collect();
    let results = tokio::time::timeout(
        Duration::from_secs(10),
        join_all(workers.iter().zip(&worker_names).map(|(driver, name)| {
            bootstrap(
                driver,
                &fixture.source,
                driver,
                &fixture.leases,
                ChangeFeedFactoryMode::Incremental,
                "orders",
                name,
            )
        })),
    )
    .await
    .expect("concurrent workers must complete bootstrap");
    for result in results {
        result?;
    }
    let context = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    assert!(context.ready().await?);
    let plan = plan(&context).await?;
    assert_eq!(plan.ranges.len(), 3);
    for range in plan.ranges {
        let item = context
            .store
            .read::<serde_json::Value>(&lease_id(&context, &range))
            .await?
            .unwrap();
        assert_eq!(item.value["ttl"], -1);
        assert!(item.value.get("continuation").is_none());
        assert!(item.value.get("owner").is_none());
        let lease: ChangeFeedProcessorBootstrapProcessingLease =
            serde_json::from_value(item.value)?;
        assert_eq!(lease.range, range);
    }
    assert!(context
        .store
        .read::<ChangeFeedProcessorBootstrapLockRecord>(&context.record("lock").id)
        .await?
        .is_none());
    assert_eq!(fixture.requests.change_feed_reads.load(Ordering::SeqCst), 0);
    let writes = fixture.requests.writes.load(Ordering::SeqCst);
    context.run().await?;
    assert_eq!(fixture.requests.writes.load(Ordering::SeqCst), writes);
    Ok(())
}

#[tokio::test]
async fn exactly_one_concurrent_claim_succeeds() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let context = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    context.ensure_record(&context.record("definition")).await?;
    let claims = join_all((0..8).map(|_| context.try_claim())).await;
    let mut winners = Vec::new();
    for claim in claims {
        if let Some(claim) = claim? {
            winners.push(claim);
        }
    }
    assert_eq!(winners.len(), 1);
    let stored = context
        .store
        .read::<ChangeFeedProcessorBootstrapLockRecord>(&context.record("lock").id)
        .await?
        .unwrap();
    assert_eq!(stored.value.record.ttl, LOCK_TTL_SECONDS);
    assert_eq!(stored.etag, winners[0].etag);
    Ok(())
}

#[tokio::test]
async fn competing_modes_cannot_initialize_the_same_workload() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let latest = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    let all_versions = fixture.context(ChangeFeedFactoryMode::FullFidelity)?;
    let (first, second) = futures::join!(latest.run(), all_versions.run());
    let (winner, loser, context) = if first.is_ok() {
        (first, second, &latest)
    } else {
        (second, first, &all_versions)
    };
    winner?;
    assert_eq!(
        loser.unwrap_err().status().status_code(),
        StatusCode::Conflict
    );
    let definition = context
        .store
        .read::<ChangeFeedProcessorBootstrapRecord>(&context.record("definition").id)
        .await?
        .unwrap();
    assert_eq!(definition.value.workload.mode, context.identity.mode);
    assert!(context.ready().await?);
    Ok(())
}

#[tokio::test]
async fn incompatible_mode_is_rejected_before_and_after_completion() -> Result<()> {
    for (original, incoming) in [
        (
            ChangeFeedFactoryMode::Incremental,
            ChangeFeedFactoryMode::FullFidelity,
        ),
        (
            ChangeFeedFactoryMode::FullFidelity,
            ChangeFeedFactoryMode::Incremental,
        ),
    ] {
        let fixture = Fixture::new(Some(-1), "/id").await?;
        let context = fixture.context(original)?;
        let incoming = fixture.context(incoming)?;
        context.ensure_record(&context.record("definition")).await?;
        for complete in [false, true] {
            if complete {
                context.run().await?;
            }
            let writes = fixture.requests.writes.load(Ordering::SeqCst);
            let error = incoming.run().await.unwrap_err();
            assert_eq!(error.status().status_code(), StatusCode::Conflict);
            assert_eq!(fixture.requests.writes.load(Ordering::SeqCst), writes);
        }
    }
    Ok(())
}

#[tokio::test]
async fn stored_schema_one_modes_remain_compatible() -> Result<()> {
    for (mode, stored_mode) in [
        (ChangeFeedFactoryMode::Incremental, "latestVersion"),
        (ChangeFeedFactoryMode::FullFidelity, "allVersionsAndDeletes"),
    ] {
        let fixture = Fixture::new(Some(-1), "/id").await?;
        let context = fixture.context(mode)?;
        for suffix in ["definition", "info"] {
            let id = context.record(suffix).id;
            let stored = serde_json::json!({
                "id": id,
                "workload": {
                    "version": 1,
                    "account": ENDPOINT,
                    "containerRid": fixture.source.rid(),
                    "processorName": "orders",
                    "mode": stored_mode
                },
                "ttl": -1
            });
            assert!(context.store.create(&id, &stored).await?);
        }
        let writes = fixture.requests.writes.load(Ordering::SeqCst);
        context.run().await?;
        assert!(context.ready().await?);
        assert_eq!(fixture.requests.writes.load(Ordering::SeqCst), writes);
    }
    Ok(())
}

#[tokio::test]
async fn recovery_creates_only_missing_leases_and_preserves_progress() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let context = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    context.ensure_record(&context.record("definition")).await?;
    let claim = context.try_claim().await?.unwrap();
    let plan = context.plan(&claim).await?.unwrap();
    let id = lease_id(&context, &plan.ranges[0]);
    let lease = ChangeFeedProcessorBootstrapProcessingLease {
        record: ChangeFeedProcessorBootstrapRecord {
            id: id.clone(),
            workload: context.identity.clone(),
            ttl: -1,
        },
        range: plan.ranges[0].clone(),
        owner: Some("existing-owner".into()),
        continuation: Some(ContinuationToken::from_string("saved-position".into())),
    };
    assert!(context.store.create(&id, &lease).await?);
    let before = context.store.read::<serde_json::Value>(&id).await?.unwrap();
    assert!(!context.ready().await?);
    // The in-memory emulator has no TTL sweeper; simulate service expiry of only the lock.
    context
        .store
        .release(&context.record("lock").id, claim.etag)
        .await?;
    context.run().await?;
    let after = context.store.read::<serde_json::Value>(&id).await?.unwrap();
    assert_eq!(after.etag, before.etag);
    assert_eq!(after.value, before.value);
    for range in &plan.ranges {
        assert!(context
            .store
            .read::<ChangeFeedProcessorBootstrapProcessingLease>(&lease_id(&context, range))
            .await?
            .is_some());
    }
    assert!(context.ready().await?);
    Ok(())
}

#[tokio::test]
async fn stale_release_cannot_delete_replacement_claim() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let context = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    let original = context.try_claim().await?.unwrap();
    context
        .store
        .release(&context.record("lock").id, original.etag.clone())
        .await?;
    let replacement = context.try_claim().await?.unwrap();
    context
        .store
        .release(&context.record("lock").id, original.etag)
        .await?;
    let current = context
        .store
        .read::<ChangeFeedProcessorBootstrapLockRecord>(&context.record("lock").id)
        .await?
        .unwrap();
    assert_eq!(current.etag, replacement.etag);
    Ok(())
}

#[tokio::test]
async fn waiting_worker_has_a_bounded_timeout() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let context = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    context.ensure_record(&context.record("definition")).await?;
    let claim = context.try_claim().await?.unwrap();
    let mut waiting = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    waiting.store.deadline = Instant::now() + Duration::from_millis(50);
    let result = tokio::time::timeout(Duration::from_secs(2), waiting.run())
        .await
        .expect("bootstrap must respect its own timeout");
    assert_eq!(
        result.unwrap_err().status().status_code(),
        StatusCode::RequestTimeout
    );
    let current = context
        .store
        .read::<ChangeFeedProcessorBootstrapLockRecord>(&context.record("lock").id)
        .await?
        .unwrap();
    assert_eq!(current.etag, claim.etag);
    assert!(!context.ready().await?);
    assert!(context
        .store
        .read::<ChangeFeedProcessorBootstrapRangePlan>(&context.record("plan").id)
        .await?
        .is_none());
    Ok(())
}

#[tokio::test]
async fn waiter_takes_over_after_abandoned_claim_disappears() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let context = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    context.ensure_record(&context.record("definition")).await?;
    let claim = context.try_claim().await?.unwrap();
    assert!(context.try_claim().await?.is_none());
    let (result, expiry) = futures::join!(context.run(), async {
        tokio::time::sleep(Duration::from_millis(30)).await;
        context
            .store
            .release(&context.record("lock").id, claim.etag)
            .await
    });
    expiry?;
    result?;
    assert!(context.ready().await?);
    Ok(())
}

#[tokio::test]
async fn completed_workload_does_not_read_or_recreate_leases() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let context = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    context.run().await?;
    let plan = plan(&context).await?;
    let preserved_id = lease_id(&context, &plan.ranges[0]);
    let preserved = context
        .store
        .read::<serde_json::Value>(&preserved_id)
        .await?
        .unwrap();
    let removed_id = lease_id(&context, &plan.ranges[1]);
    let removed = context
        .store
        .read::<ChangeFeedProcessorBootstrapProcessingLease>(&removed_id)
        .await?
        .unwrap();
    context.store.release(&removed_id, removed.etag).await?;
    let writes = fixture.requests.writes.load(Ordering::SeqCst);
    let lease_reads = fixture.requests.lease_reads.load(Ordering::SeqCst);
    let plan_reads = fixture.requests.plan_reads.load(Ordering::SeqCst);
    context.run().await?;
    assert_eq!(fixture.requests.writes.load(Ordering::SeqCst), writes);
    assert_eq!(
        fixture.requests.lease_reads.load(Ordering::SeqCst),
        lease_reads
    );
    assert_eq!(
        fixture.requests.plan_reads.load(Ordering::SeqCst),
        plan_reads
    );
    assert!(context
        .store
        .read::<ChangeFeedProcessorBootstrapProcessingLease>(&removed_id)
        .await?
        .is_none());
    let after = context
        .store
        .read::<serde_json::Value>(&preserved_id)
        .await?
        .unwrap();
    assert_eq!(after.etag, preserved.etag);
    assert_eq!(after.value, preserved.value);
    Ok(())
}

#[tokio::test]
async fn invalid_lease_container_configuration_fails_before_writing() -> Result<()> {
    for (ttl, partition_key) in [(None, "/id"), (Some(60), "/id"), (Some(-1), "/pk")] {
        let fixture = Fixture::new(ttl, partition_key).await?;
        let writes = fixture.requests.writes.load(Ordering::SeqCst);
        let error = fixture
            .context(ChangeFeedFactoryMode::Incremental)?
            .run()
            .await
            .unwrap_err();
        assert_eq!(error.status().status_code(), StatusCode::BadRequest);
        assert_eq!(fixture.requests.writes.load(Ordering::SeqCst), writes);
    }
    Ok(())
}

#[tokio::test]
async fn conflicting_existing_range_is_not_overwritten_or_marked_ready() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let context = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    context.ensure_record(&context.record("definition")).await?;
    let claim = context.try_claim().await?.unwrap();
    let plan = context.plan(&claim).await?.unwrap();
    let id = lease_id(&context, &plan.ranges[0]);
    let lease = ChangeFeedProcessorBootstrapProcessingLease {
        record: ChangeFeedProcessorBootstrapRecord {
            id: id.clone(),
            workload: context.identity.clone(),
            ttl: -1,
        },
        range: FeedRange::full(),
        owner: Some("owner".into()),
        continuation: Some(ContinuationToken::from_string("keep-me".into())),
    };
    assert!(context.store.create(&id, &lease).await?);
    context
        .store
        .release(&context.record("lock").id, claim.etag)
        .await?;
    let before = context.store.read::<serde_json::Value>(&id).await?.unwrap();
    let error = context.run().await.unwrap_err();
    assert_eq!(error.status().status_code(), StatusCode::Conflict);
    assert!(!context.ready().await?);
    let after = context.store.read::<serde_json::Value>(&id).await?.unwrap();
    assert_eq!(after.value, before.value);
    assert_eq!(after.etag, before.etag);
    Ok(())
}

#[tokio::test]
async fn names_and_source_rids_isolate_workloads_but_modes_do_not() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let context = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    let mut different = context.identity.clone();
    different.mode = ChangeFeedFactoryMode::FullFidelity;
    assert_eq!(namespace(&different)?, context.namespace);
    different.container_rid.push('x');
    assert_ne!(namespace(&different)?, context.namespace);
    different = context.identity.clone();
    different.processor_name = "other-orders".into();
    assert_ne!(namespace(&different)?, context.namespace);
    different = context.identity.clone();
    different.account = "https://another-account.documents.azure.com".into();
    assert_ne!(namespace(&different)?, context.namespace);
    Ok(())
}

#[tokio::test]
async fn local_invalid_configuration_does_not_issue_requests() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let writes = fixture.requests.writes.load(Ordering::SeqCst);
    for (processor_name, instance_name) in [(" ", "worker"), ("orders", "")] {
        assert_eq!(
            bootstrap(
                &fixture.driver,
                &fixture.source,
                &fixture.driver,
                &fixture.leases,
                ChangeFeedFactoryMode::Incremental,
                processor_name,
                instance_name,
            )
            .await
            .unwrap_err()
            .status()
            .status_code(),
            StatusCode::BadRequest
        );
    }
    assert_eq!(fixture.requests.writes.load(Ordering::SeqCst), writes);
    Ok(())
}

#[tokio::test]
async fn completed_workload_does_not_require_its_initial_plan() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let context = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    context.run().await?;
    let id = context.record("plan").id;
    let existing = context
        .store
        .read::<ChangeFeedProcessorBootstrapRangePlan>(&id)
        .await?
        .unwrap();
    context.store.release(&id, existing.etag).await?;
    let claim = context.try_claim().await?.unwrap();
    let writes = fixture.requests.writes.load(Ordering::SeqCst);
    assert!(context.plan(&claim).await?.is_none());
    let lease_reads = fixture.requests.lease_reads.load(Ordering::SeqCst);
    let plan_reads = fixture.requests.plan_reads.load(Ordering::SeqCst);

    context.initialize(&claim).await?;
    context.run().await?;

    assert_eq!(fixture.requests.writes.load(Ordering::SeqCst), writes);
    assert_eq!(
        fixture.requests.lease_reads.load(Ordering::SeqCst),
        lease_reads
    );
    assert_eq!(
        fixture.requests.plan_reads.load(Ordering::SeqCst),
        plan_reads
    );
    assert!(context
        .store
        .read::<ChangeFeedProcessorBootstrapRangePlan>(&id)
        .await?
        .is_none());
    context
        .store
        .release(&context.record("lock").id, claim.etag)
        .await?;
    Ok(())
}

#[tokio::test]
async fn superseded_claim_does_not_create_leases_or_mark_completion() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let context = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    context.ensure_record(&context.record("definition")).await?;
    let original = context.try_claim().await?.unwrap();
    let plan = context.plan(&original).await?.unwrap();
    context
        .store
        .release(&context.record("lock").id, original.etag.clone())
        .await?;
    let replacement = context.try_claim().await?.unwrap();
    assert_eq!(
        context
            .initialize(&original)
            .await
            .unwrap_err()
            .status()
            .status_code(),
        StatusCode::PreconditionFailed,
    );
    assert!(!context.ready().await?);
    for range in &plan.ranges {
        assert!(context
            .store
            .read::<ChangeFeedProcessorBootstrapProcessingLease>(&lease_id(&context, range))
            .await?
            .is_none());
    }
    assert_eq!(
        context
            .store
            .read::<ChangeFeedProcessorBootstrapLockRecord>(&context.record("lock").id)
            .await?
            .unwrap()
            .etag,
        replacement.etag,
    );
    Ok(())
}

#[tokio::test]
async fn multi_write_lease_account_can_bootstrap() -> Result<()> {
    for mode in [
        ChangeFeedFactoryMode::Incremental,
        ChangeFeedFactoryMode::FullFidelity,
    ] {
        let fixture = Fixture::new(Some(-1), "/id").await?;
        fixture
            .emulator
            .store()
            .config()
            .set_write_mode(WriteMode::Multi);
        let lease_driver = Fixture::new_driver(&fixture.emulator).await?;
        let (_, multi_write, _, _) = lease_driver
            .cached_account_configuration_for_testing()
            .await
            .unwrap();
        assert!(multi_write);
        bootstrap(
            &fixture.driver,
            &fixture.source,
            &lease_driver,
            &fixture.leases,
            mode,
            "orders",
            "worker",
        )
        .await?;
        let context = ChangeFeedProcessorBootstrapContext::new(
            &fixture.driver,
            &fixture.source,
            &lease_driver,
            &fixture.leases,
            mode,
            "orders",
            "worker",
        )?;
        assert!(context.ready().await?);
        let plan = plan(&context).await?;
        assert_eq!(plan.ranges.len(), 3);
        for range in &plan.ranges {
            let lease = context
                .store
                .read::<ChangeFeedProcessorBootstrapProcessingLease>(&lease_id(&context, range))
                .await?
                .unwrap();
            assert_eq!(lease.value.record.workload, context.identity);
            assert_eq!(&lease.value.range, range);
            assert!(lease.value.owner.is_none());
            assert!(lease.value.continuation.is_none());
        }
        assert_eq!(fixture.requests.change_feed_reads.load(Ordering::SeqCst), 0);
    }
    Ok(())
}

#[tokio::test]
async fn waiting_worker_exits_on_completion_without_acquiring_the_lock() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let initializer = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    initializer
        .ensure_record(&initializer.record("definition"))
        .await?;
    let claim = initializer.try_claim().await?.unwrap();
    let mut waiting = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    waiting.store.deadline = Instant::now() + Duration::from_secs(1);
    waiting.instance_name = "waiting-worker".into();
    assert!(waiting.try_claim().await?.is_none());

    initializer.initialize(&claim).await?;
    waiting.run().await?;

    assert!(waiting.ready().await?);
    let still_held = waiting
        .store
        .read::<ChangeFeedProcessorBootstrapLockRecord>(&waiting.record("lock").id)
        .await?
        .unwrap();
    assert_eq!(still_held.etag, claim.etag);
    Ok(())
}

#[tokio::test]
async fn same_driver_workers_re_register_after_claim_contention() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let context = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    context.ensure_record(&context.record("definition")).await?;
    let claim = context.try_claim().await?.unwrap();
    let workers = ["first", "second", "third"];
    let mut waiters: Vec<_> = workers
        .iter()
        .map(|worker| {
            Box::pin(bootstrap(
                &fixture.driver,
                &fixture.source,
                &fixture.driver,
                &fixture.leases,
                ChangeFeedFactoryMode::Incremental,
                "orders",
                worker,
            ))
        })
        .collect();
    for waiter in &mut waiters {
        assert!(poll!(waiter.as_mut()).is_pending());
    }
    assert_eq!(fixture.driver.bootstrap_coalescer().active_count(), 0);
    context
        .store
        .release(&context.record("lock").id, claim.etag)
        .await?;
    for result in tokio::time::timeout(Duration::from_secs(10), join_all(waiters))
        .await
        .expect("joined workers must complete")
    {
        result?;
    }
    assert!(context.ready().await?);
    assert_eq!(fixture.driver.bootstrap_coalescer().active_count(), 0);
    assert_eq!(fixture.requests.change_feed_reads.load(Ordering::SeqCst), 0);
    Ok(())
}

#[tokio::test]
async fn incompatible_mode_does_not_join_a_successful_flight() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let context = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    context.ensure_record(&context.record("definition")).await?;
    let claim = context.try_claim().await?.unwrap();
    let mut first = Box::pin(bootstrap(
        &fixture.driver,
        &fixture.source,
        &fixture.driver,
        &fixture.leases,
        ChangeFeedFactoryMode::Incremental,
        "orders",
        "first",
    ));
    assert!(poll!(first.as_mut()).is_pending());
    let error = tokio::time::timeout(
        Duration::from_secs(2),
        bootstrap(
            &fixture.driver,
            &fixture.source,
            &fixture.driver,
            &fixture.leases,
            ChangeFeedFactoryMode::FullFidelity,
            "orders",
            "other-mode",
        ),
    )
    .await
    .expect("an incompatible mode must not wait on another mode's flight")
    .unwrap_err();
    assert_eq!(error.status().status_code(), StatusCode::Conflict);
    context
        .store
        .release(&context.record("lock").id, claim.etag)
        .await?;
    tokio::time::timeout(Duration::from_secs(10), first)
        .await
        .expect("original bootstrap must complete")?;
    assert_eq!(fixture.driver.bootstrap_coalescer().active_count(), 0);
    Ok(())
}

#[tokio::test]
async fn completed_flight_does_not_replace_the_durable_marker() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    for worker in ["first", "later"] {
        bootstrap(
            &fixture.driver,
            &fixture.source,
            &fixture.driver,
            &fixture.leases,
            ChangeFeedFactoryMode::Incremental,
            "orders",
            worker,
        )
        .await?;
        assert_eq!(fixture.driver.bootstrap_coalescer().active_count(), 0);
        let context = fixture.context(ChangeFeedFactoryMode::Incremental)?;
        assert!(context.ready().await?);
        let plan = plan(&context).await?;
        for range in &plan.ranges {
            assert!(context
                .store
                .read::<ChangeFeedProcessorBootstrapProcessingLease>(&lease_id(&context, range))
                .await?
                .is_some());
        }
        if worker == "first" {
            let marker_id = context.record("info").id;
            let marker = context
                .store
                .read::<ChangeFeedProcessorBootstrapRecord>(&marker_id)
                .await?
                .unwrap();
            context.store.release(&marker_id, marker.etag).await?;
            let id = lease_id(&context, &plan.ranges[0]);
            let lease = context
                .store
                .read::<ChangeFeedProcessorBootstrapProcessingLease>(&id)
                .await?
                .unwrap();
            context.store.release(&id, lease.etag).await?;
        }
    }
    Ok(())
}

#[tokio::test]
async fn cancelling_waiters_between_attempts_does_not_retain_the_driver() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let weak = Arc::downgrade(&fixture.driver);
    let context = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    context.ensure_record(&context.record("definition")).await?;
    let claim = context.try_claim().await?.unwrap();
    let mut waiter = Box::pin(bootstrap(
        &fixture.driver,
        &fixture.source,
        &fixture.driver,
        &fixture.leases,
        ChangeFeedFactoryMode::Incremental,
        "orders",
        "cancelled",
    ));
    assert!(poll!(waiter.as_mut()).is_pending());
    assert_eq!(fixture.driver.bootstrap_coalescer().active_count(), 0);
    drop(waiter);
    assert_eq!(fixture.driver.bootstrap_coalescer().active_count(), 0);
    context
        .store
        .release(&context.record("lock").id, claim.etag)
        .await?;
    drop(context);
    drop(fixture);
    assert!(weak.upgrade().is_none());
    Ok(())
}

#[tokio::test]
async fn unwrapped_driver_preserves_borrowed_bootstrap_contract() -> Result<()> {
    let Fixture {
        driver,
        source,
        leases,
        ..
    } = Fixture::new(Some(-1), "/id").await?;
    let driver = Arc::try_unwrap(driver).expect("fixture has a single driver owner");
    assert!(driver.retain_for_bootstrap().is_none());
    bootstrap(
        &driver,
        &source,
        &driver,
        &leases,
        ChangeFeedFactoryMode::Incremental,
        "orders",
        "worker",
    )
    .await?;
    assert!(
        ChangeFeedProcessorBootstrapContext::new(
            &driver,
            &source,
            &driver,
            &leases,
            ChangeFeedFactoryMode::Incremental,
            "orders",
            "worker",
        )?
        .ready()
        .await?
    );
    Ok(())
}

#[tokio::test]
async fn joined_workers_re_register_after_partial_initialization_panics() -> Result<()> {
    let fixture = Arc::new(Fixture::new(Some(-1), "/id").await?);
    let context = fixture.context(ChangeFeedFactoryMode::Incremental)?;
    let key = ChangeFeedProcessorBootstrapCoalescingKey {
        namespace: context.namespace.clone(),
        lease_container: fixture.leases.rid().to_owned(),
        lease_driver: Arc::as_ptr(&fixture.driver) as usize,
        mode: std::mem::discriminant(&ChangeFeedFactoryMode::Incremental),
    };
    let (prepared_tx, prepared_rx) = oneshot::channel();
    let (crash_tx, crash_rx) = oneshot::channel();
    let retained = fixture.clone();
    let failed_attempt = fixture.driver.bootstrap_coalescer().start_or_join(
        key,
        context.store.deadline,
        move || {
            async move {
                let context = retained.context(ChangeFeedFactoryMode::Incremental)?;
                context.ensure_record(&context.record("definition")).await?;
                let claim = context.try_claim().await?.unwrap();
                context
                    .initialize_and_release(
                        &claim,
                        Box::pin(async {
                            let plan = context.plan(&claim).await?.unwrap();
                            let partial = ChangeFeedProcessorBootstrapRangePlan {
                                record: plan.record.clone(),
                                ranges: vec![plan.ranges[0].clone()],
                            };
                            context.create_leases(&partial, &claim).await?;
                            let id = lease_id(&context, &plan.ranges[0]);
                            let before =
                                context.store.read::<serde_json::Value>(&id).await?.unwrap();
                            assert!(prepared_tx.send((id, before)).is_ok());
                            crash_rx.await.unwrap();
                            panic!("initializer failed after persisting its first lease");
                        }),
                    )
                    .await?;
                Ok(true)
            }
            .boxed()
        },
    );
    let mut first = Box::pin(bootstrap(
        &fixture.driver,
        &fixture.source,
        &fixture.driver,
        &fixture.leases,
        ChangeFeedFactoryMode::Incremental,
        "orders",
        "first",
    ));
    assert!(poll!(first.as_mut()).is_pending());
    let (id, before) = prepared_rx.await.unwrap();
    let mut follower = Box::pin(bootstrap(
        &fixture.driver,
        &fixture.source,
        &fixture.driver,
        &fixture.leases,
        ChangeFeedFactoryMode::Incremental,
        "orders",
        "follower",
    ));
    assert!(poll!(follower.as_mut()).is_pending());
    assert_eq!(fixture.driver.bootstrap_coalescer().active_count(), 1);
    crash_tx.send(()).unwrap();
    assert_eq!(
        failed_attempt.await.unwrap_err().status().status_code(),
        StatusCode::InternalServerError
    );
    assert_eq!(fixture.driver.bootstrap_coalescer().active_count(), 0);
    assert!(context
        .store
        .read::<ChangeFeedProcessorBootstrapLockRecord>(&context.record("lock").id)
        .await?
        .is_none());
    assert!(!context.ready().await?);
    let late = Box::pin(bootstrap(
        &fixture.driver,
        &fixture.source,
        &fixture.driver,
        &fixture.leases,
        ChangeFeedFactoryMode::Incremental,
        "orders",
        "late",
    ));
    for result in tokio::time::timeout(Duration::from_secs(5), join_all([first, follower, late]))
        .await
        .expect("all workers must recover within their original calls")
    {
        result?;
    }
    let after = context.store.read::<serde_json::Value>(&id).await?.unwrap();
    assert_eq!(after.etag, before.etag);
    assert_eq!(after.value, before.value);
    let plan = plan(&context).await?;
    assert_eq!(plan.ranges.len(), 3);
    for range in &plan.ranges {
        assert!(context
            .store
            .read::<ChangeFeedProcessorBootstrapProcessingLease>(&lease_id(&context, range))
            .await?
            .is_some());
    }
    assert!(context.ready().await?);
    assert_eq!(fixture.driver.bootstrap_coalescer().active_count(), 0);
    assert_eq!(fixture.requests.change_feed_reads.load(Ordering::SeqCst), 0);
    Ok(())
}
