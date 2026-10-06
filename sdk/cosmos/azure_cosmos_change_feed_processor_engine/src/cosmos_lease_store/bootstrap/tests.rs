// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    coverage, BootstrapPhase, BootstrapPlan, BootstrapStartPolicy, BootstrapStore, InitialLease,
    StoredMode, INITIALIZATION_GENERATION,
};
use crate::cosmos_lease_store::{LeaseIdentity, LeaseRecord};
use crate::{ChangeFeedMode, ChangeFeedReadOptions, LeaseOwnershipOptions, ProcessorEngine};
use azure_core::http::{headers::HeaderName, Request};
use azure_data_cosmos_driver::{
    fault_injection::{
        CustomResponseBuilder, FaultInjectionConditionBuilder, FaultInjectionResultBuilder,
        FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
    },
    in_memory_emulator::{
        ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, RequestObserver,
        VirtualAccountConfig, VirtualRegion,
    },
    models::{
        AccountReference, CosmosOperation, FeedRange, ItemReference, PartitionKey, Precondition,
    },
    options::DriverOptions,
    CosmosDriver,
};
use serde_json::json;
use std::{
    collections::BTreeMap,
    error::Error,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::time::sleep;

#[derive(Debug)]
struct ArmAfterBatch {
    count: AtomicU32,
    after: u32,
    rules: Vec<Arc<FaultInjectionRule>>,
}
impl RequestObserver for ArmAfterBatch {
    fn on_request(&self, request: &Request) {
        if request
            .headers()
            .get_optional_str(&HeaderName::from_static("x-ms-cosmos-is-batch-request"))
            == Some("True")
            && self.count.fetch_add(1, Ordering::SeqCst) + 1 == self.after
        {
            for rule in &self.rules {
                rule.enable();
            }
        }
    }
}

pub(super) struct Fixture {
    pub a: BootstrapStore,
    pub b: BootstrapStore,
    pub driver: Arc<CosmosDriver>,
    pub source: ProcessorEngine,
}
fn range(min: &str, max: &str) -> FeedRange {
    FeedRange::new(min.try_into().unwrap(), max.try_into().unwrap()).unwrap()
}
pub(super) fn policy() -> LeaseOwnershipOptions {
    LeaseOwnershipOptions::new(
        Duration::from_secs(2),
        Duration::from_millis(200),
        Duration::from_millis(500),
        Duration::from_millis(300),
    )
    .unwrap()
}
pub(super) fn error_rule(id: &str, operation: FaultOperationType) -> Arc<FaultInjectionRule> {
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            id,
            FaultInjectionResultBuilder::new()
                .with_probability(1.0)
                .with_custom_response(
                    CustomResponseBuilder::new(azure_core::http::StatusCode::BadRequest)
                        .with_body(
                            br#"{"code":"BadRequest","message":"injected-bootstrap-failure"}"#
                                .to_vec(),
                        )
                        .build(),
                )
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

pub(super) async fn fixture(
    rules: Vec<Arc<FaultInjectionRule>>,
    after: u32,
) -> Result<Fixture, Box<dyn Error>> {
    fixture_with_observer(rules, after, None).await
}

pub(super) async fn fixture_with_observer(
    rules: Vec<Arc<FaultInjectionRule>>,
    after: u32,
    observer: Option<Arc<dyn RequestObserver>>,
) -> Result<Fixture, Box<dyn Error>> {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        "https://eastus.emulator.local".parse()?,
    )])?
    .with_consistency(ConsistencyLevel::Session);
    let mut emulator = InMemoryEmulatorHttpClient::new(config);
    if let Some(observer) = observer {
        emulator = emulator.with_request_observer(observer);
    } else if !rules.is_empty() {
        emulator = emulator.with_request_observer(Arc::new(ArmAfterBatch {
            count: AtomicU32::new(0),
            after,
            rules: rules.clone(),
        }));
    }
    let emulator = Arc::new(emulator);
    emulator.store().create_database("db");
    for (container, pk) in [("source", "/pk"), ("leases", "/workload")] {
        emulator.store().create_container_with_config(
            "db",
            container,
            serde_json::from_value(json!({"paths":[pk],"kind":"Hash","version":2}))?,
            ContainerConfig::new().with_partition_count(1).build()?,
        );
    }
    let mut drivers = Vec::new();
    for index in 0..2 {
        let runtime = if index == 0 && !rules.is_empty() {
            emulator
                .runtime_builder_with_fault_rules(rules.clone())
                .build()
                .await?
        } else {
            emulator.runtime_builder().build().await?
        };
        drivers.push(
            runtime
                .create_driver(
                    DriverOptions::builder(AccountReference::with_master_key(
                        "https://eastus.emulator.local".parse()?,
                        "dGVzdGtleQ==",
                    ))
                    .build(),
                )
                .await?,
        );
    }
    let source = ProcessorEngine::new(drivers[0].clone(), "db", "source").await?;
    let mut seeds = Vec::new();
    for range in [range("", "80"), range("80", "FF")] {
        let reader = source
            .open_reader(ChangeFeedReadOptions::new(
                range.clone(),
                BootstrapStartPolicy::Beginning,
            ))
            .await?;
        seeds.push(InitialLease::new(range, reader.to_continuation_token()?)?);
    }
    let plan = source
        .bootstrap_plan(
            "group",
            ChangeFeedMode::LatestVersion,
            BootstrapStartPolicy::Beginning,
            FeedRange::full(),
            seeds,
        )
        .await?;
    let a = BootstrapStore::new_single_writer(
        drivers[0].clone(),
        "db",
        "leases",
        plan.clone(),
        policy(),
    )
    .await?;
    let b = BootstrapStore::new_single_writer(drivers[1].clone(), "db", "leases", plan, policy())
        .await?;
    Ok(Fixture {
        a,
        b,
        driver: drivers[0].clone(),
        source,
    })
}

#[tokio::test]
async fn concurrent_initializers_create_complete_workload_once_then_both_join(
) -> Result<(), Box<dyn Error>> {
    let f = fixture(vec![], 0).await?;
    let (a, b) = tokio::join!(
        f.a.ensure_initialized("initializer-A", Duration::from_secs(10)),
        f.b.ensure_initialized("initializer-B", Duration::from_secs(10)),
    );
    let a = a?;
    let b = b?;
    assert_eq!(a.workload_id(), b.workload_id());
    assert_eq!(a.lease_ids().len(), 2);
    assert_eq!(b.lease_ids().len(), 2);
    let records = f.a.discover().await?;
    assert_eq!(records.len(), 2);
    assert!(coverage(a.plan(), &records, a.workload_id())?.is_empty());
    let first = f.a.work_lease_store(&a, &a.lease_ids()[0])?;
    let second = f.b.work_lease_store(&b, &b.lease_ids()[1])?;
    let first = first.try_acquire("worker-A").await?.unwrap();
    let second = second.try_acquire("worker-B").await?.unwrap();
    first.release().await?;
    second.release().await?;
    Ok(())
}

#[tokio::test]
async fn partial_creation_failure_never_marks_ready_and_restart_preserves_progress(
) -> Result<(), Box<dyn Error>> {
    let fail = error_rule("fail-second-create", FaultOperationType::BatchItem);
    let f = fixture(vec![fail.clone()], 2).await?;
    assert!(f
        .a
        .ensure_initialized("A", Duration::from_secs(10))
        .await
        .is_err());
    let observed = f.b.store.observe().await?;
    assert_eq!(
        f.b.metadata(&observed.record)?.phase,
        BootstrapPhase::Initializing
    );
    let records = f.b.discover().await?;
    assert_eq!(records.len(), 1);
    let mut advanced = records[0].clone();
    advanced.checkpoint = "existing-durable-progress".into();
    let etag = advanced.extra["_etag"].as_str().unwrap().to_owned();
    f.b.store
        .execute(
            CosmosOperation::replace_item(ItemReference::from_name(
                &f.b.store.container,
                f.b.store.partition_key.clone(),
                advanced.id.clone(),
            ))
            .with_precondition(Precondition::if_match(etag))
            .with_body(serde_json::to_vec(&advanced)?),
        )
        .await?;
    fail.disable();
    sleep(policy().duration()).await;
    let ready = f.b.ensure_initialized("B", Duration::from_secs(10)).await?;
    let records = f.b.discover().await?;
    assert_eq!(records.len(), 2);
    let kept = records
        .iter()
        .find(|lease| lease.id == advanced.id)
        .unwrap();
    assert_eq!(kept.checkpoint, "existing-durable-progress");
    let other = records
        .iter()
        .find(|lease| lease.id != advanced.id)
        .unwrap();
    assert_eq!(
        other.checkpoint,
        ready.plan().initial_leases()[1].checkpoint()
    );
    Ok(())
}

#[tokio::test]
async fn ready_marker_is_reverified_and_missing_lease_is_repaired() -> Result<(), Box<dyn Error>> {
    let f = fixture(vec![], 0).await?;
    let ready = f.a.ensure_initialized("A", Duration::from_secs(10)).await?;
    let records = f.a.discover().await?;
    let deleted = &records[1];
    f.a.store
        .execute(
            CosmosOperation::delete_item(ItemReference::from_name(
                &f.a.store.container,
                f.a.store.partition_key.clone(),
                deleted.id.clone(),
            ))
            .with_precondition(Precondition::if_match(
                deleted.extra["_etag"].as_str().unwrap().to_owned(),
            )),
        )
        .await?;
    let repaired = f.b.ensure_initialized("B", Duration::from_secs(10)).await?;
    assert_eq!(repaired.lease_ids().len(), 2);
    let now = f.b.discover().await?;
    assert!(coverage(ready.plan(), &now, ready.workload_id())?.is_empty());
    assert_eq!(
        now.iter()
            .find(|lease| lease.id == records[0].id)
            .unwrap()
            .checkpoint,
        records[0].checkpoint
    );
    Ok(())
}

#[tokio::test]
async fn parent_coverage_prevents_fresh_child_creation() -> Result<(), Box<dyn Error>> {
    let f = fixture(vec![], 0).await?;
    f.a.create_if_absent().await?;
    let reader = f
        .source
        .open_reader(ChangeFeedReadOptions::new(
            FeedRange::full(),
            BootstrapStartPolicy::Beginning,
        ))
        .await?;
    let original = reader.to_continuation_token()?;
    let parent = LeaseRecord {
        id: "existing-parent".into(),
        version: 1,
        ownership: LeaseIdentity {
            owner: None,
            generation: 4,
        },
        range: FeedRange::full(),
        checkpoint: original.as_str().to_owned(),
        lease_duration_ms: 2000,
        extra: BTreeMap::from([
            ("workload".into(), json!(f.a.store.id)),
            (
                "initializationGeneration".into(),
                json!(INITIALIZATION_GENERATION),
            ),
        ]),
    };
    f.a.store
        .execute(
            CosmosOperation::create_item(ItemReference::from_name(
                &f.a.store.container,
                f.a.store.partition_key.clone(),
                parent.id.clone(),
            ))
            .with_body(serde_json::to_vec(&parent)?),
        )
        .await?;
    let ready = f.b.ensure_initialized("B", Duration::from_secs(10)).await?;
    assert_eq!(ready.lease_ids(), &["existing-parent"]);
    let stored = f.b.discover().await?;
    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].range, FeedRange::full());
    assert_eq!(stored[0].checkpoint, original.as_str());
    Ok(())
}

#[tokio::test]
async fn lost_initializer_cannot_create_work_or_publish_ready() -> Result<(), Box<dyn Error>> {
    let f = fixture(vec![], 0).await?;
    f.a.create_if_absent().await?;
    let old = f.a.store.try_acquire("old").await?.unwrap();
    let observation = f.b.store.observe().await?;
    sleep(policy().duration() + Duration::from_millis(20)).await;
    let replacement = f.b.store.acquire(&observation, "new").await?.unwrap();
    let mut metadata = f.a.metadata(&observation.record)?;
    metadata.phase = BootstrapPhase::Ready;
    metadata.committed_generation = Some(1);
    assert!(old.bootstrap_write(&metadata, None, &[]).await.is_err());
    assert_eq!(
        f.b.metadata(&f.b.store.observe().await?.record)?.phase,
        BootstrapPhase::Initializing
    );
    replacement.release().await?;
    Ok(())
}

#[tokio::test]
async fn processing_acquisition_rejects_staged_work_until_readiness_commit(
) -> Result<(), Box<dyn Error>> {
    let f = fixture(vec![], 0).await?;
    let ready = f.a.ensure_initialized("A", Duration::from_secs(10)).await?;
    let work = f.b.work_lease_store(&ready, &ready.lease_ids()[0])?;
    let repairer = f.a.store.try_acquire("repairer").await?.unwrap();
    let mut metadata = f.a.metadata(&f.a.store.observe().await?.record)?;
    metadata.phase = BootstrapPhase::Initializing;
    metadata.committed_generation = None;
    repairer.bootstrap_write(&metadata, None, &[]).await?;
    assert!(work.try_acquire("worker").await.is_err());
    let records = f.a.discover().await?;
    metadata.phase = BootstrapPhase::Ready;
    metadata.committed_generation = Some(1);
    repairer.bootstrap_write(&metadata, None, &records).await?;
    repairer.release().await?;
    let worker = work.try_acquire("worker").await?.unwrap();
    worker.release().await?;
    Ok(())
}

#[tokio::test]
async fn incompatible_configuration_and_wait_deadlines_fail_explicitly(
) -> Result<(), Box<dyn Error>> {
    let f = fixture(vec![], 0).await?;
    f.a.create_if_absent().await?;
    let incumbent = f.a.store.try_acquire("incumbent").await?.unwrap();
    assert!(f
        .b
        .ensure_initialized("waiter", Duration::from_millis(50))
        .await
        .is_err());
    let mut changed = f.a.plan.clone();
    changed.mode = StoredMode::AllVersionsAndDeletes;
    let incompatible =
        BootstrapStore::new_single_writer(f.driver, "db", "leases", changed, policy()).await?;
    assert!(incompatible
        .ensure_initialized("other", Duration::from_secs(1))
        .await
        .is_err());
    incumbent.release().await?;
    Ok(())
}

#[test]
fn completeness_is_expected_coverage_not_merely_valid_supplied_leases() {
    let seed = |bounds| {
        InitialLease::new(
            bounds,
            azure_data_cosmos_driver::models::ContinuationToken::from_string("position".into()),
        )
        .unwrap()
    };
    assert!(BootstrapPlan::new(
        "source",
        "group",
        ChangeFeedMode::LatestVersion,
        BootstrapStartPolicy::Beginning,
        FeedRange::full(),
        vec![seed(range("", "80"))]
    )
    .is_err());
    assert!(BootstrapPlan::new(
        "source",
        "group",
        ChangeFeedMode::LatestVersion,
        BootstrapStartPolicy::Beginning,
        FeedRange::full(),
        vec![seed(range("", "C0")), seed(range("80", "FF"))]
    )
    .is_err());
    let plan = BootstrapPlan::new(
        "source",
        "group",
        ChangeFeedMode::LatestVersion,
        BootstrapStartPolicy::Beginning,
        FeedRange::full(),
        vec![seed(range("", "80")), seed(range("80", "FF"))],
    )
    .unwrap();
    assert_eq!(coverage(&plan, &[], "workload").unwrap(), vec![0, 1]);
}

#[tokio::test]
async fn discovery_failure_and_cleanup_failure_preserve_error_and_forbid_ready(
) -> Result<(), Box<dyn Error>> {
    let query = error_rule("discovery-failed", FaultOperationType::QueryItem);
    let release = error_rule("cleanup-failed", FaultOperationType::ReplaceItem);
    // The first initializer batch succeeds, then both discovery and cleanup fail.
    let f = fixture(vec![query.clone(), release.clone()], 1).await?;
    let error =
        f.a.ensure_initialized("A", Duration::from_secs(10))
            .await
            .err()
            .ok_or("discovery must fail")?;
    assert!(error.to_string().contains("bootstrap cleanup also failed"));
    assert!(error.diagnostics().is_some());
    assert!(query.hit_count() > 0);
    assert!(release.hit_count() > 0);
    let record = f.b.store.observe().await?;
    assert_eq!(
        f.b.metadata(&record.record)?.phase,
        BootstrapPhase::Initializing
    );
    assert_eq!(f.b.discover().await?.len(), 0);
    query.disable();
    release.disable();
    sleep(policy().duration()).await;
    assert_eq!(
        f.b.ensure_initialized("B", Duration::from_secs(10))
            .await?
            .lease_ids()
            .len(),
        2
    );
    Ok(())
}

#[tokio::test]
async fn stale_etag_fences_both_child_creation_and_readiness_publication(
) -> Result<(), Box<dyn Error>> {
    let f = fixture(vec![], 0).await?;
    f.a.create_if_absent().await?;
    let old = f.a.store.try_acquire("A").await?.unwrap();
    let observed = f.b.store.observe().await?;
    let mut replacement = observed.record.clone();
    replacement.ownership.owner = Some("B".into());
    replacement.ownership.generation += 1;
    f.b.store.replace(&replacement, observed.revision()).await?;
    let metadata = f.a.metadata(&observed.record)?;
    let child = LeaseRecord {
        id: format!("{}.lease.0", f.a.store.id),
        version: 1,
        ownership: LeaseIdentity {
            owner: None,
            generation: 0,
        },
        range: metadata.plan.seeds[0].range.clone(),
        checkpoint: metadata.plan.seeds[0].checkpoint.clone(),
        lease_duration_ms: 2000,
        extra: BTreeMap::from([
            ("workload".into(), json!(f.a.store.id)),
            ("initializationGeneration".into(), json!(1)),
        ]),
    };
    let error = old
        .bootstrap_write(&metadata, Some(child), &[])
        .await
        .unwrap_err();
    assert_eq!(
        error.status().status_code(),
        azure_core::http::StatusCode::PreconditionFailed
    );
    assert!(
        f.b.discover().await?.is_empty(),
        "transaction must not create stale-generation work"
    );
    let stored = f.b.store.observe().await?;
    assert_eq!(stored.owner(), Some("B"));
    assert_eq!(
        f.b.metadata(&stored.record)?.phase,
        BootstrapPhase::Initializing
    );
    Ok(())
}

#[tokio::test]
async fn store_without_readiness_gate_cannot_acquire_a_bootstrap_lease(
) -> Result<(), Box<dyn Error>> {
    let f = fixture(vec![], 0).await?;
    let ready = f.a.ensure_initialized("A", Duration::from_secs(10)).await?;
    let plain = crate::CosmosLeaseStore::new_single_writer(
        f.driver,
        "db",
        "leases",
        PartitionKey::from(ready.workload_id().to_owned()),
        ready.lease_ids()[0].clone(),
        policy(),
    )
    .await?;
    assert!(plain.try_acquire("bypass").await.is_err());
    let gated = f.b.work_lease_store(&ready, &ready.lease_ids()[0])?;
    let worker = gated.try_acquire("worker").await?.unwrap();
    worker.release().await?;
    Ok(())
}

#[tokio::test]
async fn restart_adopts_persisted_positions_not_new_caller_tokens() -> Result<(), Box<dyn Error>> {
    let f = fixture(vec![], 0).await?;
    let original = f.a.ensure_initialized("A", Duration::from_secs(10)).await?;
    let mut new_plan = f.a.plan.clone();
    for seed in &mut new_plan.seeds {
        seed.checkpoint = "new-caller-position-must-not-replace-original".into();
    }
    let later =
        BootstrapStore::new_single_writer(f.driver, "db", "leases", new_plan, policy()).await?;
    let ready = later
        .ensure_initialized("later", Duration::from_secs(10))
        .await?;
    assert!(ready.plan().initial_leases() == original.plan().initial_leases());
    let records = f.b.discover().await?;
    for (index, seed) in original.plan().initial_leases().iter().enumerate() {
        let id = format!("{}.lease.{index}", ready.workload_id());
        assert_eq!(
            records
                .iter()
                .find(|record| record.id == id)
                .unwrap()
                .checkpoint,
            seed.checkpoint()
        );
    }
    Ok(())
}

#[test]
fn partial_parent_coverage_and_unrecognized_generations_are_not_reseeded() {
    let seed = |bounds| {
        InitialLease::new(
            bounds,
            azure_data_cosmos_driver::models::ContinuationToken::from_string("initial".into()),
        )
        .unwrap()
    };
    let plan = BootstrapPlan::new(
        "source",
        "group",
        ChangeFeedMode::LatestVersion,
        BootstrapStartPolicy::Beginning,
        FeedRange::full(),
        vec![seed(range("", "80")), seed(range("80", "FF"))],
    )
    .unwrap();
    let parent = LeaseRecord {
        id: "parent".into(),
        version: 1,
        ownership: LeaseIdentity {
            owner: None,
            generation: 2,
        },
        range: range("", "40"),
        checkpoint: "existing-progress".into(),
        lease_duration_ms: 2000,
        extra: BTreeMap::from([
            ("workload".into(), json!("workload")),
            ("initializationGeneration".into(), json!(1)),
        ]),
    };
    assert!(coverage(&plan, std::slice::from_ref(&parent), "workload").is_err());
    let mut unknown = parent;
    unknown.range = FeedRange::full();
    unknown
        .extra
        .insert("initializationGeneration".into(), json!(2));
    assert!(coverage(&plan, &[unknown], "workload").is_err());
}
