// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod controlled_interleavings;
mod generated_ownership;
mod managed_recovery;
mod review_regressions;

use super::{
    CosmosLeaseStore, LeaseIdentity, LeaseOwnershipOptions, LeaseRecord, LeaseReleaseOutcome,
};
use crate::{ChangeFeedReadOptions, LeaseRunOptions, LeaseRunOutcome, ProcessorEngine};
use azure_core::http::StatusCode;
use azure_data_cosmos_driver::{
    in_memory_emulator::{
        ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig,
        VirtualRegion,
    },
    models::{
        AccountReference, ChangeFeedStartFrom, CosmosOperation, FeedRange, ItemReference,
        PartitionKey,
    },
    options::{DriverOptions, OperationOptions},
    CosmosDriver,
};
use std::{collections::BTreeMap, error::Error, num::NonZeroU32, sync::Arc, time::Duration};
use tokio::{
    sync::oneshot,
    time::{sleep, timeout},
};

struct Fixture {
    worker_a: ProcessorEngine,
    store_a: CosmosLeaseStore,
    store_b: CosmosLeaseStore,
    driver: Arc<CosmosDriver>,
}

fn policy() -> LeaseOwnershipOptions {
    LeaseOwnershipOptions::new(
        Duration::from_secs(2),
        Duration::from_millis(200),
        Duration::from_millis(500),
        Duration::from_millis(300),
    )
    .unwrap()
}

async fn fixture() -> Result<Fixture, Box<dyn Error>> {
    fixture_with_rules(vec![]).await
}

async fn fixture_with_rules(
    rules: Vec<Arc<azure_data_cosmos_driver::fault_injection::FaultInjectionRule>>,
) -> Result<Fixture, Box<dyn Error>> {
    let endpoint = "https://eastus.emulator.local".parse()?;
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new("East US", endpoint)])?
        .with_consistency(ConsistencyLevel::Session);
    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    emulator.store().create_database("db");
    for container in ["source", "leases"] {
        emulator.store().create_container_with_config(
            "db",
            container,
            serde_json::from_value(serde_json::json!({"paths":["/pk"],"kind":"Hash","version":2}))?,
            ContainerConfig::new().with_partition_count(1).build()?,
        );
    }
    let mut drivers = Vec::new();
    // Independent runtimes, clients, and session caches; only the service is shared.
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

    finish_fixture(drivers).await
}

#[tokio::test]
async fn uncertain_renewal_revokes_busy_handler_and_allows_eventual_takeover(
) -> Result<(), Box<dyn Error>> {
    use azure_data_cosmos_driver::fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionResultBuilder, FaultInjectionRuleBuilder,
        FaultOperationType,
    };
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "delayed-lease-write",
            FaultInjectionResultBuilder::new()
                .with_delay(Duration::from_secs(1))
                .with_probability(1.0)
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::ReplaceItem)
                .build(),
        )
        .build(),
    );
    rule.disable();
    let f = fixture_with_rules(vec![rule.clone()]).await?;
    let session = f.store_a.try_acquire("A").await?.unwrap();
    let old = session.lease().await;
    let observation = f.store_b.observe().await?;
    let (entered_tx, entered_rx) = oneshot::channel();
    let (late_tx, late_rx) = oneshot::channel();
    let running = session.clone();
    let task = tokio::spawn(async move {
        let mut entered = Some(entered_tx);
        let mut late = Some(late_rx);
        f.worker_a
            .run_with_lease_session(
                &running,
                ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning),
                &LeaseRunOptions::new(NonZeroU32::new(1).unwrap()),
                |_| {
                    let entered = entered.take().unwrap();
                    let late = late.take().unwrap();
                    async move {
                        entered.send(()).unwrap();
                        late.await.unwrap();
                        Ok(())
                    }
                },
            )
            .await
    });
    timeout(Duration::from_secs(5), entered_rx).await??;
    rule.enable();
    let result = timeout(Duration::from_secs(5), task).await??;
    assert!(rule.hit_count() > 0);
    assert!(result.maintenance_error().is_some());
    assert_eq!(
        result.processing().as_ref().unwrap().outcome(),
        LeaseRunOutcome::OwnershipLost
    );
    assert_eq!(result.processing().as_ref().unwrap().confirmed_batches(), 0);
    assert!(late_tx.send(()).is_err());
    assert!(matches!(
        result.release(),
        LeaseReleaseOutcome::AuthorityLost
    ));
    assert!(session.checkpoint(&old, old.checkpoint()).await.is_err());
    rule.disable();
    sleep(policy().duration()).await;
    let replacement = f.store_b.acquire(&observation, "B").await?.unwrap();
    assert_eq!(replacement.lease().await.checkpoint(), old.checkpoint());
    replacement.release().await?;
    Ok(())
}

#[tokio::test]
async fn watchdog_revokes_authority_even_when_revision_lock_is_blocked(
) -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    let session = f.store_a.try_acquire("A").await?.unwrap();
    let running = session.clone();
    let (entered_tx, entered_rx) = oneshot::channel();
    let (late_tx, late_rx) = oneshot::channel();
    let task = tokio::spawn(async move {
        let mut entered = Some(entered_tx);
        let mut late = Some(late_rx);
        f.worker_a
            .run_with_lease_session(
                &running,
                ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning),
                &LeaseRunOptions::new(NonZeroU32::new(1).unwrap()),
                |_| {
                    let entered = entered.take().unwrap();
                    let late = late.take().unwrap();
                    async move {
                        entered.send(()).unwrap();
                        late.await.unwrap();
                        Ok(())
                    }
                },
            )
            .await
    });
    timeout(Duration::from_secs(5), entered_rx).await??;
    let lock = session.state.lock().await;
    sleep(policy().duration()).await;
    assert!(
        session.control().is_lost(),
        "watchdog must not wait on the revision lock"
    );
    assert!(late_tx.send(()).is_err());
    drop(lock);
    let result = timeout(Duration::from_secs(5), task).await??;
    assert_eq!(
        result.processing().as_ref().unwrap().outcome(),
        LeaseRunOutcome::OwnershipLost
    );
    assert!(matches!(
        result.release(),
        LeaseReleaseOutcome::AuthorityLost
    ));
    Ok(())
}
async fn finish_fixture(drivers: Vec<Arc<CosmosDriver>>) -> Result<Fixture, Box<dyn Error>> {
    let worker_a = ProcessorEngine::new(drivers[0].clone(), "db", "source").await?;
    let reader = worker_a
        .open_reader(ChangeFeedReadOptions::new(
            FeedRange::full(),
            ChangeFeedStartFrom::Beginning,
        ))
        .await?;
    let checkpoint = reader.to_continuation_token()?.as_str().to_owned();
    let source = drivers[0]
        .resolve_container("db", "source", OperationOptions::default())
        .await?;
    drivers[0]
        .execute_singleton_operation(
            CosmosOperation::create_item(ItemReference::from_name(
                &source,
                PartitionKey::from("P"),
                "event",
            ))
            .with_body(br#"{"id":"event","pk":"P","value":1}"#.to_vec()),
            OperationOptions::default(),
        )
        .await?;
    let lease_container = drivers[0]
        .resolve_container("db", "leases", OperationOptions::default())
        .await?;
    let mut extra = BTreeMap::new();
    extra.insert("pk".into(), serde_json::json!("P"));
    extra.insert(
        "applicationMarker".into(),
        serde_json::json!({"preserve":true}),
    );
    let record = LeaseRecord {
        id: "lease".into(),
        version: 1,
        ownership: LeaseIdentity {
            owner: None,
            generation: 0,
        },
        range: FeedRange::full(),
        checkpoint,
        lease_duration_ms: 2000,
        extra,
    };
    drivers[0]
        .execute_singleton_operation(
            CosmosOperation::create_item(ItemReference::from_name(
                &lease_container,
                PartitionKey::from("P"),
                "lease",
            ))
            .with_body(serde_json::to_vec(&record)?),
            OperationOptions::default(),
        )
        .await?;
    let store_a = CosmosLeaseStore::new_single_writer(
        drivers[0].clone(),
        "db",
        "leases",
        PartitionKey::from("P"),
        "lease",
        policy(),
    )
    .await?;
    let store_b = CosmosLeaseStore::new_single_writer(
        drivers[1].clone(),
        "db",
        "leases",
        PartitionKey::from("P"),
        "lease",
        policy(),
    )
    .await?;
    Ok(Fixture {
        worker_a,
        store_a,
        store_b,
        driver: drivers[0].clone(),
    })
}

#[tokio::test]
async fn independent_workers_compete_take_over_and_reject_stale_authority(
) -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    let a = f.store_a.observe().await?;
    let b = f.store_b.observe().await?;
    assert_eq!(a.revision(), b.revision());
    let (a, b) = tokio::join!(f.store_a.acquire(&a, "A"), f.store_b.acquire(&b, "B"));
    let (winner, loser) = match (a, b) {
        (Ok(Some(session)), Err(error)) => {
            assert_eq!(error.status().status_code(), StatusCode::PreconditionFailed);
            (session, &f.store_b)
        }
        (Err(error), Ok(Some(session))) => {
            assert_eq!(error.status().status_code(), StatusCode::PreconditionFailed);
            (session, &f.store_a)
        }
        _ => panic!("exactly one ETag acquisition must succeed"),
    };
    let old = winner.lease().await;
    assert!(loser.try_acquire("next").await?.is_none());
    let observed = loser.observe().await?;
    sleep(policy().duration() + Duration::from_millis(50)).await;
    let replacement = loser
        .acquire(&observed, "next")
        .await?
        .ok_or("unchanged expired lease should be eligible")?;
    let current = replacement.lease().await;
    assert_eq!(current.epoch().get(), old.epoch().get() + 1);
    assert_eq!(current.checkpoint(), old.checkpoint());
    assert!(winner.renew().await.is_err());
    assert!(winner.checkpoint(&old, old.checkpoint()).await.is_err());
    assert!(winner.release().await.is_err());
    let observed = loser.observe().await?;
    assert_eq!(observed.owner(), Some("next"));
    assert_eq!(observed.generation(), current.epoch().get());
    replacement.release().await?;
    let released = loser.observe().await?;
    assert!(released.owner().is_none());
    assert_eq!(released.checkpoint(), old.checkpoint().as_str());
    let fresh = loser
        .try_acquire("third")
        .await?
        .ok_or("released lease should acquire immediately")?;
    assert_eq!(fresh.lease().await.epoch().get(), current.epoch().get() + 1);
    fresh.release().await?;
    Ok(())
}

#[tokio::test]
async fn busy_handler_is_renewed_and_checkpoint_uses_latest_revision() -> Result<(), Box<dyn Error>>
{
    let f = fixture().await?;
    let session = f
        .store_a
        .try_acquire("A")
        .await?
        .ok_or("initial acquisition")?;
    let initial = session.lease().await;
    let contender = f.store_b.observe().await?;
    let (entered_tx, entered_rx) = oneshot::channel();
    let (ack_tx, ack_rx) = oneshot::channel();
    let running = session.clone();
    let task = tokio::spawn(async move {
        let mut entered_tx = Some(entered_tx);
        let mut ack_rx = Some(ack_rx);
        f.worker_a
            .run_with_lease_session(
                &running,
                ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning),
                &LeaseRunOptions::new(NonZeroU32::new(1).unwrap()),
                |_| {
                    let entered_tx = entered_tx.take().unwrap();
                    let ack_rx = ack_rx.take().unwrap();
                    async move {
                        entered_tx.send(()).unwrap();
                        ack_rx.await.unwrap();
                        Ok(())
                    }
                },
            )
            .await
    });
    timeout(Duration::from_secs(5), entered_rx).await??;
    sleep(Duration::from_millis(2300)).await;
    let renewed = session.lease().await;
    assert_ne!(renewed.revision(), initial.revision());
    assert_eq!(renewed.checkpoint(), initial.checkpoint());
    let takeover = f.store_b.acquire(&contender, "B").await;
    assert!(
        takeover.is_err(),
        "renewals must fence an aged stale observation"
    );
    ack_tx.send(()).unwrap();
    let result = timeout(Duration::from_secs(5), task).await??;
    let report = result
        .processing()
        .as_ref()
        .map_err(|error| error.to_string())?;
    assert_eq!(report.outcome(), LeaseRunOutcome::BatchLimitReached);
    assert_eq!(report.confirmed_batches(), 1);
    assert_ne!(report.lease().checkpoint(), initial.checkpoint());
    assert!(result.maintenance_error().is_none());
    assert!(matches!(result.release(), LeaseReleaseOutcome::Released));
    let released = f.store_b.observe().await?;
    assert!(released.owner().is_none());
    assert_eq!(released.checkpoint(), report.lease().checkpoint().as_str());
    assert_eq!(
        released.record.extra["applicationMarker"],
        serde_json::json!({"preserve":true})
    );
    Ok(())
}

#[tokio::test]
async fn concurrent_renewal_and_checkpoint_preserve_each_others_fields(
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
    let (renewed, persisted) =
        tokio::join!(session.renew(), session.checkpoint(&expected, &candidate));
    renewed?;
    persisted.map_err(|error| format!("{error:?}"))?;
    let authoritative = session.lease().await;
    assert_eq!(authoritative.checkpoint(), &candidate);
    assert_eq!(authoritative.owner(), expected.owner());
    assert_eq!(authoritative.epoch(), expected.epoch());
    assert_eq!(f.store_b.observe().await?.checkpoint(), candidate.as_str());
    session.release().await?;
    Ok(())
}

#[tokio::test]
async fn stale_generation_is_rejected_by_cosmos_etag_not_just_local_time(
) -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    let obsolete = f.store_a.try_acquire("A").await?.unwrap();
    let old = obsolete.lease().await;
    // Controlled external authority replacement while the old local timer is still valid.
    let observation = f.store_b.observe().await?;
    let mut replacement = observation.record.clone();
    replacement.ownership.owner = Some("B".into());
    replacement.ownership.generation += 1;
    f.store_b
        .replace(&replacement, observation.revision())
        .await?;
    let error = obsolete
        .checkpoint(&old, old.checkpoint())
        .await
        .unwrap_err();
    match error {
        crate::CheckpointError::Ambiguous(error) => {
            assert_eq!(error.status().status_code(), StatusCode::PreconditionFailed)
        }
        _ => panic!("stale revision must fail conditionally"),
    }
    assert!(obsolete.release().await.is_err());
    let stored = f.store_b.observe().await?;
    assert_eq!(stored.owner(), Some("B"));
    assert_eq!(stored.generation(), old.epoch().get() + 1);
    assert_eq!(stored.checkpoint(), old.checkpoint().as_str());
    Ok(())
}

#[tokio::test]
async fn release_is_conditional_against_a_replacement_owner() -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    let old = f.store_a.try_acquire("A").await?.unwrap();
    let observation = f.store_b.observe().await?;
    let mut replacement = observation.record.clone();
    replacement.ownership.owner = Some("B".into());
    replacement.ownership.generation += 1;
    f.store_b
        .replace(&replacement, observation.revision())
        .await?;
    let error = old.release().await.unwrap_err();
    assert_eq!(error.status().status_code(), StatusCode::PreconditionFailed);
    assert_eq!(f.store_b.observe().await?.owner(), Some("B"));
    Ok(())
}

#[tokio::test]
async fn renewal_conflict_cancels_handler_and_rejects_late_completion() -> Result<(), Box<dyn Error>>
{
    let f = fixture().await?;
    let session = f.store_a.try_acquire("A").await?.unwrap();
    let (entered_tx, entered_rx) = oneshot::channel();
    let (late_tx, late_rx) = oneshot::channel();
    let running = session.clone();
    let task = tokio::spawn(async move {
        let mut entered = Some(entered_tx);
        let mut late = Some(late_rx);
        f.worker_a
            .run_with_lease_session(
                &running,
                ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning),
                &LeaseRunOptions::new(NonZeroU32::new(1).unwrap()),
                |_| {
                    let entered = entered.take().unwrap();
                    let late = late.take().unwrap();
                    async move {
                        entered.send(()).unwrap();
                        late.await.unwrap();
                        Ok(())
                    }
                },
            )
            .await
    });
    timeout(Duration::from_secs(5), entered_rx).await??;
    let observation = f.store_b.observe().await?;
    let mut replacement = observation.record.clone();
    replacement.ownership.owner = Some("B".into());
    replacement.ownership.generation += 1;
    f.store_b
        .replace(&replacement, observation.revision())
        .await?;
    let result = timeout(Duration::from_secs(5), task).await??;
    let report = result
        .processing()
        .as_ref()
        .map_err(|error| error.to_string())?;
    assert_eq!(report.outcome(), LeaseRunOutcome::OwnershipLost);
    assert_eq!(report.confirmed_batches(), 0);
    assert!(report.candidate().is_some());
    assert!(late_tx.send(()).is_err());
    assert!(result.maintenance_error().is_some());
    assert!(matches!(
        result.release(),
        LeaseReleaseOutcome::AuthorityLost
    ));
    assert_eq!(f.store_b.observe().await?.owner(), Some("B"));
    Ok(())
}

#[tokio::test]
async fn observation_binding_duration_and_missing_lease_fail_explicitly(
) -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    let observation = f.store_a.observe().await?;
    assert!(f.store_b.acquire(&observation, "B").await.is_err());
    let different = LeaseOwnershipOptions::new(
        Duration::from_secs(3),
        Duration::from_millis(200),
        Duration::from_millis(500),
        Duration::from_millis(300),
    )?;
    let mismatch = CosmosLeaseStore::new_single_writer(
        f.driver.clone(),
        "db",
        "leases",
        PartitionKey::from("P"),
        "lease",
        different,
    )
    .await?;
    assert!(mismatch.observe().await.is_err());
    let missing = CosmosLeaseStore::new_single_writer(
        f.driver,
        "db",
        "leases",
        PartitionKey::from("P"),
        "missing",
        policy(),
    )
    .await?;
    assert_eq!(
        missing
            .observe()
            .await
            .err()
            .unwrap()
            .status()
            .status_code(),
        StatusCode::NotFound
    );
    Ok(())
}

#[tokio::test]
async fn stop_drains_and_releases_without_replaying_or_reading_another_batch(
) -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    let session = f.store_a.try_acquire("A").await?.unwrap();
    let (entered_tx, entered_rx) = oneshot::channel();
    let (finish_tx, finish_rx) = oneshot::channel();
    let running = session.clone();
    let task = tokio::spawn(async move {
        let mut entered = Some(entered_tx);
        let mut finish = Some(finish_rx);
        f.worker_a
            .run_with_lease_session(
                &running,
                ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning),
                &LeaseRunOptions::new(NonZeroU32::new(2).unwrap()),
                |_| {
                    let entered = entered.take().expect("no second batch after stop");
                    let finish = finish.take().expect("handler is invoked once");
                    async move {
                        entered.send(()).unwrap();
                        finish.await.unwrap();
                        Ok(())
                    }
                },
            )
            .await
    });
    timeout(Duration::from_secs(5), entered_rx).await??;
    session.control().stop();
    finish_tx.send(()).unwrap();
    let result = timeout(Duration::from_secs(5), task).await??;
    let report = result.processing().as_ref().unwrap();
    assert_eq!(report.outcome(), LeaseRunOutcome::StoppedDrained);
    assert_eq!(report.confirmed_batches(), 1);
    assert!(matches!(result.release(), LeaseReleaseOutcome::Released));
    let next = f
        .store_b
        .try_acquire("B")
        .await?
        .ok_or("released lease must be reusable")?;
    next.release().await?;
    Ok(())
}

#[tokio::test]
async fn duplicate_run_is_rejected_without_releasing_the_original_session(
) -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    let session = f.store_a.try_acquire("A").await?.unwrap();
    let another_engine = ProcessorEngine::new(f.driver.clone(), "db", "source").await?;
    let running = session.clone();
    let (entered_tx, entered_rx) = oneshot::channel();
    let (finish_tx, finish_rx) = oneshot::channel();
    let task = tokio::spawn(async move {
        let mut entered = Some(entered_tx);
        let mut finish = Some(finish_rx);
        f.worker_a
            .run_with_lease_session(
                &running,
                ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning),
                &LeaseRunOptions::new(NonZeroU32::new(1).unwrap()),
                |_| {
                    let entered = entered.take().unwrap();
                    let finish = finish.take().unwrap();
                    async move {
                        entered.send(()).unwrap();
                        finish.await.unwrap();
                        Ok(())
                    }
                },
            )
            .await
    });
    timeout(Duration::from_secs(5), entered_rx).await??;
    let duplicate = another_engine
        .run_with_lease_session(
            &session,
            ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning),
            &LeaseRunOptions::new(NonZeroU32::new(1).unwrap()),
            |_| async {
                panic!("duplicate must not deliver");
            },
        )
        .await;
    assert!(duplicate.processing().is_err());
    assert!(matches!(
        duplicate.release(),
        LeaseReleaseOutcome::NotAttempted
    ));
    assert!(!session.control().is_lost());
    assert_eq!(f.store_b.observe().await?.owner(), Some("A"));
    finish_tx.send(()).unwrap();
    assert!(matches!(
        timeout(Duration::from_secs(5), task).await??.release(),
        LeaseReleaseOutcome::Released
    ));
    Ok(())
}

#[tokio::test]
async fn cancelling_run_revokes_session_and_leaves_no_renewal_task() -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    let session = f.store_a.try_acquire("A").await?.unwrap();
    let initial = session.lease().await;
    let running = session.clone();
    let (entered_tx, entered_rx) = oneshot::channel();
    let (_finish_tx, finish_rx) = oneshot::channel::<()>();
    let task = tokio::spawn(async move {
        let mut entered = Some(entered_tx);
        let mut finish = Some(finish_rx);
        f.worker_a
            .run_with_lease_session(
                &running,
                ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning),
                &LeaseRunOptions::new(NonZeroU32::new(1).unwrap()),
                |_| {
                    let entered = entered.take().unwrap();
                    let finish = finish.take().unwrap();
                    async move {
                        entered.send(()).unwrap();
                        finish.await.unwrap();
                        Ok(())
                    }
                },
            )
            .await
    });
    timeout(Duration::from_secs(5), entered_rx).await??;
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    assert!(session.control().is_lost());
    let observation = f.store_b.observe().await?;
    sleep(policy().duration() + Duration::from_millis(100)).await;
    let next = f.store_b.acquire(&observation, "B").await?.unwrap();
    assert_eq!(next.lease().await.checkpoint(), initial.checkpoint());
    next.release().await?;
    Ok(())
}

#[tokio::test]
async fn callback_panic_releases_authority_without_persisting_pending_progress(
) -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    let session = f.store_a.try_acquire("A").await?.unwrap();
    let initial = session.lease().await;
    let result = f
        .worker_a
        .run_with_lease_session(
            &session,
            ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning),
            &LeaseRunOptions::new(NonZeroU32::new(2).unwrap()),
            |_| async {
                tokio::task::yield_now().await;
                panic!("application callback failed while awaiting completion");
            },
        )
        .await;
    let error = result.processing().as_ref().unwrap_err();
    assert_eq!(error.report().phase(), Some(crate::LeaseRunPhase::Handling));
    assert_eq!(error.report().confirmed_batches(), 0);
    assert!(error.report().candidate().is_some());
    assert!(error.error().diagnostics().is_some());
    assert!(matches!(result.release(), LeaseReleaseOutcome::Released));
    let stored = f.store_b.observe().await?;
    assert!(stored.owner().is_none());
    assert_eq!(stored.checkpoint(), initial.checkpoint().as_str());
    Ok(())
}
