// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    bootstrap, namespace, BootstrapContext, BootstrapOptions, ChangeFeedProcessorMode, Identity,
    LockRecord, ProcessingLease, RangePlan, Record, Store, LOCK_TTL_SECONDS, SCHEMA_VERSION,
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
use futures::future::join_all;
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

    fn context<'a>(&'a self, options: &'a BootstrapOptions) -> Result<BootstrapContext<'a>> {
        let identity = Identity {
            version: SCHEMA_VERSION,
            account: self
                .source
                .account()
                .endpoint()
                .origin()
                .ascii_serialization(),
            container_rid: self.source.rid().to_owned(),
            processor_name: options.processor_name.clone(),
            mode: options.mode,
        };
        let namespace = namespace(&identity)?;
        Ok(BootstrapContext {
            source_driver: &self.driver,
            source: &self.source,
            store: Store {
                driver: &self.driver,
                container: &self.leases,
                deadline: Instant::now() + Duration::from_secs(30),
            },
            identity,
            namespace,
            options,
        })
    }

    async fn bootstrap(&self, options: &BootstrapOptions) -> Result<()> {
        bootstrap(
            &self.driver,
            &self.source,
            &self.driver,
            &self.leases,
            options,
        )
        .await
    }
}

fn options(mode: ChangeFeedProcessorMode) -> BootstrapOptions {
    let mut options =
        BootstrapOptions::new(mode, "orders", "worker").with_timeout(Duration::from_secs(10));
    options.retry_delay = Duration::from_millis(5);
    options
}

async fn plan(context: &BootstrapContext<'_>) -> Result<RangePlan> {
    Ok(context
        .store
        .read::<RangePlan>(&context.record("plan").id)
        .await?
        .unwrap()
        .value)
}

fn lease_id(context: &BootstrapContext<'_>, range: &FeedRange) -> String {
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
    let configurations: Vec<_> = (0..workers.len())
        .map(|index| {
            let mut value = options(ChangeFeedProcessorMode::LatestVersion);
            value.instance_name = format!("worker-{index}");
            value
        })
        .collect();
    let results = join_all(workers.iter().zip(&configurations).map(|(driver, config)| {
        bootstrap(driver, &fixture.source, driver, &fixture.leases, config)
    }))
    .await;
    for result in results {
        result?;
    }
    let context = fixture.context(&configurations[0])?;
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
        let lease: ProcessingLease = serde_json::from_value(item.value)?;
        assert_eq!(lease.range, range);
    }
    assert!(context
        .store
        .read::<LockRecord>(&context.record("lock").id)
        .await?
        .is_none());
    assert_eq!(fixture.requests.change_feed_reads.load(Ordering::SeqCst), 0);
    let writes = fixture.requests.writes.load(Ordering::SeqCst);
    fixture.bootstrap(&configurations[0]).await?;
    assert_eq!(fixture.requests.writes.load(Ordering::SeqCst), writes);
    Ok(())
}

#[tokio::test]
async fn exactly_one_concurrent_claim_succeeds() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let config = options(ChangeFeedProcessorMode::LatestVersion);
    let context = fixture.context(&config)?;
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
        .read::<LockRecord>(&context.record("lock").id)
        .await?
        .unwrap();
    assert_eq!(stored.value.record.ttl, LOCK_TTL_SECONDS);
    assert_eq!(stored.etag, winners[0].etag);
    Ok(())
}

#[tokio::test]
async fn competing_modes_cannot_initialize_the_same_workload() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let latest = options(ChangeFeedProcessorMode::LatestVersion);
    let all_versions = options(ChangeFeedProcessorMode::AllVersionsAndDeletes);
    let (first, second) =
        futures::join!(fixture.bootstrap(&latest), fixture.bootstrap(&all_versions));
    let (winner, loser, winner_options) = if first.is_ok() {
        (first, second, &latest)
    } else {
        (second, first, &all_versions)
    };
    winner?;
    assert_eq!(
        loser.unwrap_err().status().status_code(),
        StatusCode::Conflict
    );
    let context = fixture.context(winner_options)?;
    let definition = context
        .store
        .read::<Record>(&context.record("definition").id)
        .await?
        .unwrap();
    assert_eq!(definition.value.workload.mode, winner_options.mode);
    assert!(context.ready().await?);
    Ok(())
}

#[tokio::test]
async fn incompatible_mode_is_rejected_before_and_after_completion() -> Result<()> {
    for (original, incoming) in [
        (
            ChangeFeedProcessorMode::LatestVersion,
            ChangeFeedProcessorMode::AllVersionsAndDeletes,
        ),
        (
            ChangeFeedProcessorMode::AllVersionsAndDeletes,
            ChangeFeedProcessorMode::LatestVersion,
        ),
    ] {
        let fixture = Fixture::new(Some(-1), "/id").await?;
        let original = options(original);
        let incoming = options(incoming);
        let context = fixture.context(&original)?;
        context.ensure_record(&context.record("definition")).await?;
        for complete in [false, true] {
            if complete {
                fixture.bootstrap(&original).await?;
            }
            let writes = fixture.requests.writes.load(Ordering::SeqCst);
            let error = fixture.bootstrap(&incoming).await.unwrap_err();
            assert_eq!(error.status().status_code(), StatusCode::Conflict);
            assert_eq!(fixture.requests.writes.load(Ordering::SeqCst), writes);
        }
    }
    Ok(())
}

#[tokio::test]
async fn recovery_creates_only_missing_leases_and_preserves_progress() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let config = options(ChangeFeedProcessorMode::LatestVersion);
    let context = fixture.context(&config)?;
    context.ensure_record(&context.record("definition")).await?;
    let claim = context.try_claim().await?.unwrap();
    let plan = context.plan(&claim).await?;
    let id = lease_id(&context, &plan.ranges[0]);
    let lease = ProcessingLease {
        record: Record {
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
    // The in-memory emulator has no TTL sweeper; simulate service expiry of only the lock.
    context
        .store
        .release(&context.record("lock").id, claim.etag)
        .await?;
    fixture.bootstrap(&config).await?;
    let after = context.store.read::<serde_json::Value>(&id).await?.unwrap();
    assert_eq!(after.etag, before.etag);
    assert_eq!(after.value, before.value);
    for range in &plan.ranges {
        assert!(context
            .store
            .read::<ProcessingLease>(&lease_id(&context, range))
            .await?
            .is_some());
    }
    assert!(context.ready().await?);
    Ok(())
}

#[tokio::test]
async fn stale_release_cannot_delete_replacement_claim() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let config = options(ChangeFeedProcessorMode::LatestVersion);
    let context = fixture.context(&config)?;
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
        .read::<LockRecord>(&context.record("lock").id)
        .await?
        .unwrap();
    assert_eq!(current.etag, replacement.etag);
    Ok(())
}

#[tokio::test]
async fn waiting_worker_has_a_bounded_timeout() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let config = options(ChangeFeedProcessorMode::LatestVersion);
    let context = fixture.context(&config)?;
    context.ensure_record(&context.record("definition")).await?;
    let claim = context.try_claim().await?.unwrap();
    let waiting = config.clone().with_timeout(Duration::from_millis(50));
    let result = tokio::time::timeout(Duration::from_secs(2), fixture.bootstrap(&waiting))
        .await
        .expect("bootstrap must respect its own timeout");
    assert_eq!(
        result.unwrap_err().status().status_code(),
        StatusCode::RequestTimeout
    );
    let current = context
        .store
        .read::<LockRecord>(&context.record("lock").id)
        .await?
        .unwrap();
    assert_eq!(current.etag, claim.etag);
    assert!(!context.ready().await?);
    assert!(context
        .store
        .read::<RangePlan>(&context.record("plan").id)
        .await?
        .is_none());
    Ok(())
}

#[tokio::test]
async fn waiter_takes_over_after_abandoned_claim_disappears() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let config = options(ChangeFeedProcessorMode::LatestVersion);
    let context = fixture.context(&config)?;
    context.ensure_record(&context.record("definition")).await?;
    let claim = context.try_claim().await?.unwrap();
    assert!(context.try_claim().await?.is_none());
    let (result, expiry) = futures::join!(fixture.bootstrap(&config), async {
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
async fn restart_verification_preserves_existing_leases() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let config = options(ChangeFeedProcessorMode::LatestVersion);
    fixture.bootstrap(&config).await?;
    let context = fixture.context(&config)?;
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
        .read::<ProcessingLease>(&removed_id)
        .await?
        .unwrap();
    context.store.release(&removed_id, removed.etag).await?;
    fixture.bootstrap(&config).await?;
    assert!(context
        .store
        .read::<ProcessingLease>(&removed_id)
        .await?
        .is_none());
    fixture
        .bootstrap(
            &config
                .clone()
                .with_lease_verification_enabled_on_restart(true),
        )
        .await?;
    assert!(context
        .store
        .read::<ProcessingLease>(&removed_id)
        .await?
        .is_some());
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
            .bootstrap(&options(ChangeFeedProcessorMode::LatestVersion))
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
    let config = options(ChangeFeedProcessorMode::LatestVersion);
    let context = fixture.context(&config)?;
    context.ensure_record(&context.record("definition")).await?;
    let claim = context.try_claim().await?.unwrap();
    let plan = context.plan(&claim).await?;
    let id = lease_id(&context, &plan.ranges[0]);
    let lease = ProcessingLease {
        record: Record {
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
    let error = fixture.bootstrap(&config).await.unwrap_err();
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
    let original = options(ChangeFeedProcessorMode::LatestVersion);
    let context = fixture.context(&original)?;
    let mut different = context.identity.clone();
    different.mode = ChangeFeedProcessorMode::AllVersionsAndDeletes;
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
    for config in [
        BootstrapOptions::new(ChangeFeedProcessorMode::LatestVersion, " ", "worker"),
        BootstrapOptions::new(ChangeFeedProcessorMode::LatestVersion, "orders", ""),
        options(ChangeFeedProcessorMode::LatestVersion).with_timeout(Duration::ZERO),
    ] {
        assert_eq!(
            fixture
                .bootstrap(&config)
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
async fn missing_completed_range_plan_is_not_reconstructed() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let config = options(ChangeFeedProcessorMode::LatestVersion);
    fixture.bootstrap(&config).await?;
    let context = fixture.context(&config)?;
    let id = context.record("plan").id;
    let existing = context.store.read::<RangePlan>(&id).await?.unwrap();
    context.store.release(&id, existing.etag).await?;
    let error = fixture
        .bootstrap(&config.with_lease_verification_enabled_on_restart(true))
        .await
        .unwrap_err();
    assert_eq!(error.status().status_code(), StatusCode::Conflict);
    Ok(())
}

#[tokio::test]
async fn superseded_claim_does_not_create_leases_or_mark_completion() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let config = options(ChangeFeedProcessorMode::LatestVersion);
    let context = fixture.context(&config)?;
    context.ensure_record(&context.record("definition")).await?;
    let original = context.try_claim().await?.unwrap();
    let plan = context.plan(&original).await?;
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
            .read::<ProcessingLease>(&lease_id(&context, range))
            .await?
            .is_none());
    }
    assert_eq!(
        context
            .store
            .read::<LockRecord>(&context.record("lock").id)
            .await?
            .unwrap()
            .etag,
        replacement.etag,
    );
    Ok(())
}

#[tokio::test]
async fn multi_write_lease_account_is_rejected_before_writing() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    fixture
        .emulator
        .store()
        .config()
        .set_write_mode(WriteMode::Multi);
    let lease_driver = Fixture::new_driver(&fixture.emulator).await?;
    let writes = fixture.requests.writes.load(Ordering::SeqCst);
    let error = bootstrap(
        &fixture.driver,
        &fixture.source,
        &lease_driver,
        &fixture.leases,
        &options(ChangeFeedProcessorMode::LatestVersion),
    )
    .await
    .unwrap_err();
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);
    assert_eq!(fixture.requests.writes.load(Ordering::SeqCst), writes);
    Ok(())
}

#[tokio::test]
async fn waiting_worker_exits_on_completion_without_acquiring_the_lock() -> Result<()> {
    let fixture = Fixture::new(Some(-1), "/id").await?;
    let initializer_options = options(ChangeFeedProcessorMode::LatestVersion);
    let initializer = fixture.context(&initializer_options)?;
    initializer
        .ensure_record(&initializer.record("definition"))
        .await?;
    let claim = initializer.try_claim().await?.unwrap();
    let mut waiting_options = initializer_options
        .clone()
        .with_timeout(Duration::from_secs(1));
    waiting_options.instance_name = "waiting-worker".into();
    let waiting = fixture.context(&waiting_options)?;
    assert!(waiting.try_claim().await?.is_none());

    initializer.initialize(&claim).await?;
    fixture.bootstrap(&waiting_options).await?;

    assert!(waiting.ready().await?);
    let still_held = waiting
        .store
        .read::<LockRecord>(&waiting.record("lock").id)
        .await?
        .unwrap();
    assert_eq!(still_held.etag, claim.etag);
    Ok(())
}
