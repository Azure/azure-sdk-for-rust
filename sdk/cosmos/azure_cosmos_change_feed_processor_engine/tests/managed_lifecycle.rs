// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Smoke/fault scenarios, not proofs of controlled protocol interleavings.

#[path = "managed_lifecycle/periodic_topology.rs"]
mod periodic_topology;

use azure_core::http::{Request, StatusCode};
use azure_cosmos_change_feed_processor_engine::{
    BootstrapStartPolicy, BootstrapStore, ChangeFeedMode, ChangeFeedReadOptions, InitialLease,
    LeaseOwnershipOptions, LeaseReleaseOutcome, LeaseRunOutcome, ManagedLeaseState,
    ManagedProcessor, ManagedProcessorOptions, ManagedProcessorState, ProcessorEngine,
    RawChangeHandler,
};
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
        AccountReference, ContainerReference, CosmosOperation, FeedRange, ItemReference,
        PartitionKey,
    },
    options::{DriverOptions, OperationOptions},
    CosmosDriver,
};
use std::{
    error::Error,
    num::NonZeroU32,
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    sync::{Notify, Semaphore},
    time::{sleep, timeout},
};

struct Fixture {
    engine: ProcessorEngine,
    lease_driver: Arc<CosmosDriver>,
    lease_container: ContainerReference,
    emulator: Arc<InMemoryEmulatorHttpClient>,
}

#[test]
fn managed_start_without_tokio_context_returns_error_before_publication(
) -> Result<(), Box<dyn Error>> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let f = runtime.block_on(fixture())?;
    let error = match futures::executor::block_on(f.engine.start_managed(
        f.lease_driver,
        f.lease_container,
        "unsupported-runtime",
        options(),
        Arc::new(|_| Box::pin(async { Ok(()) })),
    )) {
        Err(error) => error,
        Ok(_) => panic!("managed start outside Tokio must fail before task publication"),
    };
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);
    assert!(error.to_string().contains("active Tokio runtime"));
    Ok(())
}
async fn fixture() -> Result<Fixture, Box<dyn Error>> {
    fixture_with_observer(None).await
}
async fn fixture_with_observer(
    observer: Option<Arc<dyn RequestObserver>>,
) -> Result<Fixture, Box<dyn Error>> {
    fixture_with_faults(observer, Vec::new()).await
}
async fn fixture_with_faults(
    observer: Option<Arc<dyn RequestObserver>>,
    rules: Vec<Arc<FaultInjectionRule>>,
) -> Result<Fixture, Box<dyn Error>> {
    fixture_with_layout(observer, rules, 1).await
}
async fn fixture_with_layout(
    observer: Option<Arc<dyn RequestObserver>>,
    rules: Vec<Arc<FaultInjectionRule>>,
    source_partitions: u32,
) -> Result<Fixture, Box<dyn Error>> {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        "https://eastus.emulator.local".parse()?,
    )])?
    .with_consistency(ConsistencyLevel::Session);
    let mut emulator = InMemoryEmulatorHttpClient::new(config);
    if let Some(observer) = observer {
        emulator = emulator.with_request_observer(observer);
    }
    let emulator = Arc::new(emulator);
    emulator.store().create_database("db");
    for (name, path) in [("source", "/pk"), ("leases", "/workload")] {
        emulator.store().create_container_with_config(
            "db",
            name,
            serde_json::from_value(serde_json::json!({"paths":[path],"kind":"Hash","version":2}))?,
            ContainerConfig::new()
                .with_partition_count(if name == "source" {
                    source_partitions
                } else {
                    1
                })
                .build()?,
        );
    }
    let mut drivers = Vec::new();
    for _ in 0..2 {
        let runtime = emulator
            .runtime_builder_with_fault_rules(rules.clone())
            .build()
            .await?;
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
    let engine = ProcessorEngine::new(drivers[0].clone(), "db", "source").await?;
    let lease_container = drivers[1]
        .resolve_container("db", "leases", OperationOptions::default())
        .await?;
    Ok(Fixture {
        engine,
        lease_driver: drivers[1].clone(),
        lease_container,
        emulator,
    })
}
fn options() -> ManagedProcessorOptions {
    ManagedProcessorOptions::new("host")
        .with_startup_timeout(Duration::from_secs(10))
        .with_balance_interval(Duration::from_millis(20))
        .with_coverage_interval(Duration::from_secs(1))
        .with_poll_interval(Duration::from_millis(20))
        .with_recovery_backoff(Duration::from_millis(100))
}
async fn put(f: &Fixture, id: &str) -> Result<(), Box<dyn Error>> {
    put_with_key(f, id, "P").await
}
async fn put_with_key(f: &Fixture, id: &str, key: &str) -> Result<(), Box<dyn Error>> {
    f.engine
        .driver()
        .execute_singleton_operation(
            CosmosOperation::create_item(ItemReference::from_name(
                f.engine.container(),
                PartitionKey::from(key.to_owned()),
                id.to_owned(),
            ))
            .with_body(serde_json::to_vec(&serde_json::json!({"id":id,"pk":key}))?),
            OperationOptions::default(),
        )
        .await?;
    Ok(())
}
async fn wait_for(processor: &ManagedProcessor, predicate: impl Fn(&ManagedProcessor) -> bool) {
    timeout(Duration::from_secs(10), async {
        while !predicate(processor) {
            sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("managed lifecycle reached expected state");
}

#[tokio::test]
async fn continuous_polling_reuses_prepared_resources_and_stop_releases(
) -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    let driver = f.engine.driver().clone();
    let calls = Arc::new(AtomicU32::new(0));
    let handler: RawChangeHandler = {
        let calls = calls.clone();
        Arc::new(move |_| {
            let calls = calls.clone();
            Box::pin(async move {
                calls.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
        })
    };
    put(&f, "first").await?;
    let mut processor = f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            options(),
            handler,
        )
        .await?;
    wait_for(&processor, |p| {
        p.snapshot()
            .leases()
            .iter()
            .any(|l| l.last_callback().is_some() && l.last_feed() == Some(l.last_checkpoint()))
    })
    .await;
    assert!(Arc::ptr_eq(&driver, f.engine.driver()));
    assert!(!Arc::ptr_eq(&driver, &f.lease_driver));
    sleep(Duration::from_millis(150)).await;
    put(&f, "later").await?;
    wait_for(&processor, |_| calls.load(Ordering::SeqCst) == 2).await;
    let snapshot = processor.snapshot();
    assert_eq!(snapshot.state(), ManagedProcessorState::Running);
    assert_eq!(snapshot.leases().len(), 1);
    let report = processor.stop().await;
    assert!(report.is_clean());
    assert_eq!(report.leases().len(), 1);
    assert_eq!(
        report.leases()[0].outcome(),
        LeaseRunOutcome::StoppedDrained
    );
    assert!(matches!(
        report.leases()[0].release(),
        LeaseReleaseOutcome::Released
    ));
    let workload = BootstrapStore::load_existing_single_writer(
        f.lease_driver.clone(),
        f.lease_container.clone(),
        &f.engine.source_identity(),
        "group",
        options().mode(),
        options().start_policy(),
        options().ownership_policy().clone(),
    )
    .await?
    .unwrap();
    let ready = workload
        .ensure_initialized("inspector", Duration::from_secs(10))
        .await?;
    let store = workload.work_lease_store(&ready, report.leases()[0].id())?;
    assert_eq!(store.observe().await?.owner(), None);
    Ok(())
}

#[tokio::test]
async fn stop_drains_busy_callback_before_checkpoint_and_release() -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    put(&f, "event").await?;
    let entered = Arc::new(Notify::new());
    let completed = Arc::new(Notify::new());
    let handler: RawChangeHandler = {
        let entered = entered.clone();
        let completed = completed.clone();
        Arc::new(move |_| {
            let entered = entered.clone();
            let completed = completed.clone();
            Box::pin(async move {
                entered.notify_one();
                completed.notified().await;
                Ok(())
            })
        })
    };
    let mut processor = f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            options(),
            handler,
        )
        .await?;
    timeout(Duration::from_secs(10), entered.notified()).await?;
    let stopping = processor.stop();
    tokio::pin!(stopping);
    assert!(matches!(
        futures::poll!(stopping.as_mut()),
        std::task::Poll::Pending
    ));
    completed.notify_one();
    let report = timeout(Duration::from_secs(10), stopping).await?;
    assert!(report.is_clean());
    Ok(())
}

#[tokio::test]
async fn callback_panic_is_quarantined_without_killing_coordinator() -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    put(&f, "event").await?;
    let attempts = Arc::new(AtomicU32::new(0));
    let handler: RawChangeHandler = {
        let attempts = attempts.clone();
        Arc::new(move |_| {
            let attempts = attempts.clone();
            Box::pin(async move {
                if attempts.fetch_add(1, Ordering::SeqCst) == 0 {
                    panic!("application failure");
                }
                Ok(())
            })
        })
    };
    let mut processor = f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            options(),
            handler,
        )
        .await?;
    wait_for(&processor, |p| {
        p.snapshot().leases().iter().any(|l| {
            l.state() == ManagedLeaseState::Quarantined
                && l.last_callback().is_none()
                && l.last_error().is_some()
        })
    })
    .await;
    wait_for(&processor, |p| {
        p.snapshot()
            .leases()
            .iter()
            .any(|l| l.last_callback().is_some() && l.last_feed() == Some(l.last_checkpoint()))
    })
    .await;
    assert_eq!(attempts.load(Ordering::SeqCst), 2);
    assert_eq!(processor.snapshot().state(), ManagedProcessorState::Running);
    assert!(processor.stop().await.is_clean());
    Ok(())
}

#[tokio::test]
async fn startup_rejects_bounds_and_incompatible_lease_partitioning() -> Result<(), Box<dyn Error>>
{
    let f = fixture().await?;
    let handler: RawChangeHandler = Arc::new(|_| Box::pin(async { Ok(()) }));
    assert!(f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            options().with_host_capacity(NonZeroU32::new(33).unwrap()),
            handler.clone(),
        )
        .await
        .is_err());
    assert!(f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            options().with_callback_concurrency(NonZeroU32::new(33).unwrap()),
            handler.clone(),
        )
        .await
        .is_err());
    assert!(f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.engine.container().clone(),
            "group",
            options(),
            handler,
        )
        .await
        .is_err());
    let existing = BootstrapStore::load_existing_single_writer(
        f.lease_driver,
        f.lease_container,
        &f.engine.source_identity(),
        "group",
        options().mode(),
        options().start_policy(),
        options().ownership_policy().clone(),
    )
    .await?;
    assert!(existing.is_none());
    Ok(())
}

#[tokio::test]
async fn fresh_now_is_materialized_and_saved_plan_reused_on_restart() -> Result<(), Box<dyn Error>>
{
    let f = fixture().await?;
    put(&f, "old").await?;
    let calls = Arc::new(AtomicU32::new(0));
    let handler: RawChangeHandler = {
        let calls = calls.clone();
        Arc::new(move |_| {
            let calls = calls.clone();
            Box::pin(async move {
                calls.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
        })
    };
    let now = options().with_start_policy(BootstrapStartPolicy::Now);
    let mut processor = f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            now.clone(),
            handler.clone(),
        )
        .await?;
    wait_for(&processor, |p| !p.snapshot().leases().is_empty()).await;
    sleep(Duration::from_millis(100)).await;
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert!(processor.stop().await.is_clean());
    put(&f, "between-starts").await?;
    let mut processor = f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            now,
            handler,
        )
        .await?;
    wait_for(&processor, |_| calls.load(Ordering::SeqCst) == 1).await;
    assert!(processor.stop().await.is_clean());
    Ok(())
}

#[tokio::test]
async fn worker_task_failure_is_joined_and_reported_not_clean() -> Result<(), Box<dyn Error>> {
    #[derive(Debug)]
    struct PanicOnFeed(AtomicBool);
    impl RequestObserver for PanicOnFeed {
        fn on_request(&self, request: &Request) {
            if self.0.load(Ordering::SeqCst) && request.url().path().ends_with("/docs") {
                panic!("injected worker task failure");
            }
        }
    }
    let observer = Arc::new(PanicOnFeed(AtomicBool::new(false)));
    let f = fixture_with_observer(Some(observer.clone())).await?;
    put(&f, "event").await?;
    let mut processor = f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            options(),
            Arc::new(move |_| {
                let observer = observer.clone();
                Box::pin(async move {
                    observer.0.store(true, Ordering::SeqCst);
                    Ok(())
                })
            }),
        )
        .await?;
    wait_for(&processor, |p| {
        p.snapshot().state() == ManagedProcessorState::Failed
    })
    .await;
    let report = processor.stop().await;
    assert!(!report.is_clean());
    assert_eq!(report.errors().len(), 1);
    assert_eq!(report.leases().len(), 1);
    assert!(report.leases()[0].error().is_some());
    assert!(matches!(
        report.leases()[0].release(),
        LeaseReleaseOutcome::AuthorityLost
    ));
    Ok(())
}

fn one_failure(operation: FaultOperationType, status: StatusCode) -> Arc<FaultInjectionRule> {
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "managed-recovery-failure",
            FaultInjectionResultBuilder::new()
                .with_probability(1.0)
                .with_custom_response(
                    CustomResponseBuilder::new(status)
                        .with_body(
                            br#"{"code":"Injected","message":"managed recovery fault"}"#.to_vec(),
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
        .with_hit_limit(1)
        .build(),
    );
    rule.disable();
    rule
}

#[tokio::test]
async fn ambiguous_checkpoint_retries_candidate_without_rerunning_callback(
) -> Result<(), Box<dyn Error>> {
    let fault = one_failure(
        FaultOperationType::ReplaceItem,
        StatusCode::PreconditionFailed,
    );
    let f = fixture_with_faults(None, vec![fault.clone()]).await?;
    put(&f, "event").await?;
    let calls = Arc::new(AtomicU32::new(0));
    let handler: RawChangeHandler = {
        let fault = fault.clone();
        let calls = calls.clone();
        Arc::new(move |_| {
            let fault = fault.clone();
            let calls = calls.clone();
            Box::pin(async move {
                calls.fetch_add(1, Ordering::SeqCst);
                fault.enable();
                Ok(())
            })
        })
    };
    let mut processor = f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            options(),
            handler,
        )
        .await?;
    wait_for(&processor, |p| {
        fault.hit_count() == 1
            && p.snapshot()
                .leases()
                .iter()
                .any(|l| l.last_callback().is_some() && l.last_feed() == Some(l.last_checkpoint()))
    })
    .await;
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(processor.stop().await.is_clean());
    Ok(())
}

#[tokio::test]
async fn read_failure_reopens_from_confirmed_progress_and_retains_diagnostics(
) -> Result<(), Box<dyn Error>> {
    let fault = one_failure(FaultOperationType::ChangeFeedItem, StatusCode::BadRequest);
    let f = fixture_with_faults(None, vec![fault.clone()]).await?;
    put(&f, "first").await?;
    let calls = Arc::new(AtomicU32::new(0));
    let handler: RawChangeHandler = {
        let fault = fault.clone();
        let calls = calls.clone();
        Arc::new(move |_| {
            let fault = fault.clone();
            let calls = calls.clone();
            Box::pin(async move {
                calls.fetch_add(1, Ordering::SeqCst);
                fault.enable();
                Ok(())
            })
        })
    };
    let mut processor = f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            options(),
            handler,
        )
        .await?;
    wait_for(&processor, |p| {
        p.snapshot()
            .leases()
            .iter()
            .any(|l| l.state() == ManagedLeaseState::Recovering && l.last_error().is_some())
    })
    .await;
    let snapshot = processor.snapshot();
    let error = snapshot.leases()[0].last_error().unwrap();
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);
    assert!(error.diagnostics().is_some());
    put(&f, "later").await?;
    wait_for(&processor, |_| calls.load(Ordering::SeqCst) == 2).await;
    assert!(processor.stop().await.is_clean());
    Ok(())
}

#[tokio::test]
async fn drain_deadline_reports_unprocessed_candidate_without_checkpoint(
) -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    put(&f, "event").await?;
    let entered = Arc::new(Notify::new());
    let handler: RawChangeHandler = {
        let entered = entered.clone();
        Arc::new(move |_| {
            let entered = entered.clone();
            Box::pin(async move {
                entered.notify_one();
                std::future::pending::<()>().await;
                Ok(())
            })
        })
    };
    let mut processor = f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            options().with_drain_timeout(Duration::from_millis(20)),
            handler,
        )
        .await?;
    timeout(Duration::from_secs(10), entered.notified()).await?;
    let report = timeout(Duration::from_secs(2), processor.stop()).await?;
    assert!(!report.is_clean());
    assert_eq!(report.leases().len(), 1);
    let lease = &report.leases()[0];
    assert_eq!(lease.outcome(), LeaseRunOutcome::DrainTimedOut);
    assert!(matches!(lease.release(), LeaseReleaseOutcome::Released));
    let snapshot = lease.snapshot().unwrap();
    assert_eq!(snapshot.last_callback(), None);
    assert_ne!(snapshot.last_feed(), Some(snapshot.last_checkpoint()));
    Ok(())
}

#[tokio::test]
async fn dropping_processor_cancels_owned_callback_task() -> Result<(), Box<dyn Error>> {
    struct CancelSignal(Arc<Notify>);
    impl Drop for CancelSignal {
        fn drop(&mut self) {
            self.0.notify_one();
        }
    }
    let f = fixture().await?;
    put(&f, "event").await?;
    let entered = Arc::new(Notify::new());
    let cancelled = Arc::new(Notify::new());
    let handler: RawChangeHandler = {
        let entered = entered.clone();
        let cancelled = cancelled.clone();
        Arc::new(move |_| {
            let entered = entered.clone();
            let cancelled = cancelled.clone();
            Box::pin(async move {
                let _signal = CancelSignal(cancelled);
                entered.notify_one();
                std::future::pending::<()>().await;
                Ok(())
            })
        })
    };
    let processor = f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            options(),
            handler,
        )
        .await?;
    timeout(Duration::from_secs(10), entered.notified()).await?;
    drop(processor);
    timeout(Duration::from_secs(2), cancelled.notified()).await?;
    Ok(())
}

#[tokio::test]
async fn host_capacity_and_callback_concurrency_are_independent_bounds(
) -> Result<(), Box<dyn Error>> {
    let f = fixture_with_layout(None, Vec::new(), 2).await?;
    for index in 0..20 {
        put_with_key(&f, &format!("item-{index}"), &format!("key-{index}")).await?;
    }
    let gate = Arc::new(Semaphore::new(0));
    let active = Arc::new(AtomicU32::new(0));
    let maximum = Arc::new(AtomicU32::new(0));
    let handler: RawChangeHandler = {
        let gate = gate.clone();
        let active = active.clone();
        let maximum = maximum.clone();
        Arc::new(move |_| {
            let gate = gate.clone();
            let active = active.clone();
            let maximum = maximum.clone();
            Box::pin(async move {
                let current = active.fetch_add(1, Ordering::SeqCst) + 1;
                maximum.fetch_max(current, Ordering::SeqCst);
                let permit = gate.acquire().await.unwrap();
                permit.forget();
                active.fetch_sub(1, Ordering::SeqCst);
                Ok(())
            })
        })
    };
    let mut processor = f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            options()
                .with_host_capacity(NonZeroU32::new(2).unwrap())
                .with_callback_concurrency(NonZeroU32::new(1).unwrap()),
            handler,
        )
        .await?;
    wait_for(&processor, |p| {
        let snapshot = p.snapshot();
        snapshot.leases().len() == 2 && snapshot.leases().iter().all(|l| l.last_feed().is_some())
    })
    .await;
    assert_eq!(active.load(Ordering::SeqCst), 1);
    assert_eq!(maximum.load(Ordering::SeqCst), 1);
    gate.add_permits(2);
    wait_for(&processor, |p| {
        p.snapshot()
            .leases()
            .iter()
            .all(|l| l.last_callback().is_some() && l.last_feed() == Some(l.last_checkpoint()))
    })
    .await;
    let report = processor.stop().await;
    assert!(report.is_clean());
    assert_eq!(report.leases().len(), 2);
    assert_eq!(maximum.load(Ordering::SeqCst), 1);
    Ok(())
}

#[tokio::test]
async fn stop_does_not_deliver_a_callback_waiting_for_concurrency() -> Result<(), Box<dyn Error>> {
    let f = fixture_with_layout(None, Vec::new(), 2).await?;
    for index in 0..20 {
        put_with_key(&f, &format!("item-{index}"), &format!("key-{index}")).await?;
    }
    let finish = Arc::new(Notify::new());
    let calls = Arc::new(AtomicU32::new(0));
    let handler: RawChangeHandler = {
        let finish = finish.clone();
        let calls = calls.clone();
        Arc::new(move |_| {
            let finish = finish.clone();
            let calls = calls.clone();
            Box::pin(async move {
                calls.fetch_add(1, Ordering::SeqCst);
                finish.notified().await;
                Ok(())
            })
        })
    };
    let mut processor = f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            options(),
            handler,
        )
        .await?;
    wait_for(&processor, |p| {
        let snapshot = p.snapshot();
        snapshot.leases().len() == 2
            && snapshot.leases().iter().all(|l| l.last_feed().is_some())
            && calls.load(Ordering::SeqCst) == 1
    })
    .await;
    let stopping = processor.stop();
    tokio::pin!(stopping);
    assert!(matches!(
        futures::poll!(stopping.as_mut()),
        std::task::Poll::Pending
    ));
    finish.notify_one();
    let report = timeout(Duration::from_secs(2), stopping).await?;
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(!report.is_clean());
    assert_eq!(report.leases().len(), 2);
    assert_eq!(
        report
            .leases()
            .iter()
            .filter(|l| l.outcome() == LeaseRunOutcome::StoppedUnprocessed)
            .count(),
        1
    );
    assert_eq!(
        report
            .leases()
            .iter()
            .filter(|l| l.outcome() == LeaseRunOutcome::StoppedDrained)
            .count(),
        1
    );
    assert!(report
        .leases()
        .iter()
        .all(|l| matches!(l.release(), LeaseReleaseOutcome::Released)));
    Ok(())
}

#[tokio::test]
async fn bootstrap_failure_publishes_no_worker_or_callback() -> Result<(), Box<dyn Error>> {
    let fault = one_failure(FaultOperationType::CreateItem, StatusCode::BadRequest);
    let f = fixture_with_faults(None, vec![fault.clone()]).await?;
    put(&f, "event").await?;
    fault.enable();
    let calls = Arc::new(AtomicU32::new(0));
    let handler: RawChangeHandler = {
        let calls = calls.clone();
        Arc::new(move |_| {
            let calls = calls.clone();
            Box::pin(async move {
                calls.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
        })
    };
    let error = match f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            options(),
            handler,
        )
        .await
    {
        Err(error) => error,
        Ok(mut processor) => {
            processor.stop().await;
            panic!("bootstrap failure must reject startup");
        }
    };
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);
    assert!(error.diagnostics().is_some());
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert_eq!(fault.hit_count(), 1);
    Ok(())
}

#[tokio::test]
async fn confirmed_transfer_revokes_busy_callback_and_reports_ownership_loss(
) -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    put(&f, "event").await?;
    let configured = options().with_ownership_policy(LeaseOwnershipOptions::new(
        Duration::from_secs(2),
        Duration::from_millis(100),
        Duration::from_millis(400),
        Duration::from_millis(100),
    )?);
    let entered = Arc::new(Notify::new());
    let handler: RawChangeHandler = {
        let entered = entered.clone();
        Arc::new(move |_| {
            let entered = entered.clone();
            Box::pin(async move {
                entered.notify_one();
                std::future::pending::<()>().await;
                Ok(())
            })
        })
    };
    let mut processor = f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            configured.clone(),
            handler,
        )
        .await?;
    timeout(Duration::from_secs(10), entered.notified()).await?;
    let id = processor.snapshot().leases()[0].id().to_owned();
    let workload = BootstrapStore::load_existing_single_writer(
        f.lease_driver.clone(),
        f.lease_container.clone(),
        &f.engine.source_identity(),
        "group",
        configured.mode(),
        configured.start_policy(),
        configured.ownership_policy().clone(),
    )
    .await?
    .unwrap();
    let ready = workload
        .ensure_initialized("inspector", Duration::from_secs(10))
        .await?;
    let store = workload.work_lease_store(&ready, &id)?;
    let observation = store.observe().await?;
    let replacement = store.transfer(&observation, "replacement").await?;
    wait_for(&processor, |p| {
        p.snapshot().leases()[0].state() == ManagedLeaseState::OwnershipLost
    })
    .await;
    let report = processor.stop().await;
    assert!(!report.is_clean());
    assert_eq!(report.leases().len(), 1);
    assert_eq!(report.leases()[0].outcome(), LeaseRunOutcome::OwnershipLost);
    assert!(matches!(
        report.leases()[0].release(),
        LeaseReleaseOutcome::AuthorityLost
    ));
    assert_eq!(report.leases()[0].snapshot().unwrap().last_callback(), None);
    assert_eq!(store.observe().await?.owner(), Some("replacement"));
    replacement.release().await?;
    Ok(())
}

#[tokio::test]
async fn partially_anchored_now_scope_is_rejected_before_bootstrap() -> Result<(), Box<dyn Error>> {
    let f = fixture_with_layout(None, Vec::new(), 2).await?;
    let range = FeedRange::full();
    let mut reader = f
        .engine
        .open_reader(ChangeFeedReadOptions::new(
            range.clone(),
            BootstrapStartPolicy::Now,
        ))
        .await?;
    let page = reader.read_page().await?;
    let seed = InitialLease::new(range.clone(), page.continuation().clone())?;
    let error = match f
        .engine
        .bootstrap_plan(
            "partial-now",
            ChangeFeedMode::LatestVersion,
            BootstrapStartPolicy::Now,
            range,
            vec![seed],
        )
        .await
    {
        Err(error) => error,
        Ok(_) => panic!("one physical page must not anchor a multi-physical Now scope"),
    };
    assert_eq!(
        error.status(),
        azure_data_cosmos_driver::error::status_codes::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH
    );
    let existing = BootstrapStore::load_existing_single_writer(
        f.lease_driver,
        f.lease_container,
        &f.engine.source_identity(),
        "partial-now",
        ChangeFeedMode::LatestVersion,
        &BootstrapStartPolicy::Now,
        options().ownership_policy().clone(),
    )
    .await?;
    assert!(existing.is_none());
    Ok(())
}

#[tokio::test]
async fn cancelled_stop_retains_handles_and_later_stop_returns_cached_completion(
) -> Result<(), Box<dyn Error>> {
    let f = fixture().await?;
    put(&f, "event").await?;
    let entered = Arc::new(Notify::new());
    let finish = Arc::new(Notify::new());
    let handler: RawChangeHandler = {
        let entered = entered.clone();
        let finish = finish.clone();
        Arc::new(move |_| {
            let entered = entered.clone();
            let finish = finish.clone();
            Box::pin(async move {
                entered.notify_one();
                finish.notified().await;
                Ok(())
            })
        })
    };
    let mut processor = f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            options(),
            handler,
        )
        .await?;
    timeout(Duration::from_secs(10), entered.notified()).await?;
    assert!(!processor.is_complete());
    {
        let stopping = processor.stop();
        tokio::pin!(stopping);
        assert!(matches!(
            futures::poll!(stopping.as_mut()),
            std::task::Poll::Pending
        ));
    }
    assert_eq!(processor.state(), ManagedProcessorState::Stopping);
    assert!(!processor.is_complete());
    finish.notify_one();
    let report = timeout(Duration::from_secs(2), processor.stop()).await?;
    assert!(report.is_clean());
    assert_eq!(report.joined_workers(), 1);
    assert!(processor.is_complete());
    assert_eq!(processor.state(), ManagedProcessorState::Stopped);
    let repeated = processor.stop().await;
    assert!(Arc::ptr_eq(&report, &repeated));
    let mut restarted = f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            options(),
            Arc::new(|_| Box::pin(async { Ok(()) })),
        )
        .await?;
    wait_for(&restarted, |p| !p.snapshot().leases().is_empty()).await;
    assert!(restarted.stop().await.is_clean());
    assert!(restarted.is_complete());
    Ok(())
}

#[tokio::test]
async fn cancelling_start_before_readiness_never_spawns_a_callback() -> Result<(), Box<dyn Error>> {
    let fault = Arc::new(
        FaultInjectionRuleBuilder::new(
            "cancel-managed-start",
            FaultInjectionResultBuilder::new()
                .with_probability(1.0)
                .with_delay(Duration::from_secs(1))
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::CreateItem)
                .build(),
        )
        .with_hit_limit(1)
        .build(),
    );
    fault.disable();
    let f = fixture_with_faults(None, vec![fault.clone()]).await?;
    put(&f, "event").await?;
    fault.enable();
    let calls = Arc::new(AtomicU32::new(0));
    let handler: RawChangeHandler = {
        let calls = calls.clone();
        Arc::new(move |_| {
            let calls = calls.clone();
            Box::pin(async move {
                calls.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
        })
    };
    assert!(timeout(
        Duration::from_millis(30),
        f.engine.start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            options(),
            handler,
        )
    )
    .await
    .is_err());
    assert_eq!(fault.hit_count(), 1);
    sleep(Duration::from_millis(50)).await;
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert!(BootstrapStore::load_existing_single_writer(
        f.lease_driver,
        f.lease_container,
        &f.engine.source_identity(),
        "group",
        options().mode(),
        options().start_policy(),
        options().ownership_policy().clone(),
    )
    .await?
    .is_none());
    Ok(())
}

#[tokio::test]
async fn coordinator_panic_retains_workers_for_stop_to_join_and_release(
) -> Result<(), Box<dyn Error>> {
    #[derive(Debug, Default)]
    struct PanicOnRootRead {
        target: std::sync::Mutex<Option<String>>,
        armed: AtomicBool,
    }
    impl RequestObserver for PanicOnRootRead {
        fn on_request(&self, request: &Request) {
            let target = self.target.lock().unwrap().clone();
            if request.method() == azure_core::http::Method::Get
                && target.is_some_and(|target| request.url().path().ends_with(&target))
                && self.armed.swap(false, Ordering::SeqCst)
            {
                panic!("injected coordinator failure");
            }
        }
    }
    let observer = Arc::new(PanicOnRootRead::default());
    let f = fixture_with_observer(Some(observer.clone())).await?;
    put(&f, "event").await?;
    let entered = Arc::new(Notify::new());
    let finish = Arc::new(Notify::new());
    let handler: RawChangeHandler = {
        let entered = entered.clone();
        let finish = finish.clone();
        Arc::new(move |_| {
            let entered = entered.clone();
            let finish = finish.clone();
            Box::pin(async move {
                entered.notify_one();
                finish.notified().await;
                Ok(())
            })
        })
    };
    let configured = options().with_coverage_interval(Duration::from_millis(100));
    let mut processor = f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            configured.clone(),
            handler,
        )
        .await?;
    timeout(Duration::from_secs(10), entered.notified()).await?;
    let workload = BootstrapStore::load_existing_single_writer(
        f.lease_driver.clone(),
        f.lease_container.clone(),
        &f.engine.source_identity(),
        "group",
        configured.mode(),
        configured.start_policy(),
        configured.ownership_policy().clone(),
    )
    .await?
    .unwrap();
    let ready = workload
        .ensure_initialized("inspector", Duration::from_secs(10))
        .await?;
    *observer.target.lock().unwrap() = Some(format!("/docs/{}", ready.workload_id()));
    observer.armed.store(true, Ordering::SeqCst);
    wait_for(&processor, |p| p.state() == ManagedProcessorState::Failed).await;
    assert!(!processor.is_complete());
    {
        let stopping = processor.stop();
        tokio::pin!(stopping);
        assert!(matches!(
            futures::poll!(stopping.as_mut()),
            std::task::Poll::Pending
        ));
    }
    assert!(!processor.is_complete());
    finish.notify_one();
    let report = timeout(Duration::from_secs(2), processor.stop()).await?;
    assert!(!report.is_clean());
    assert_eq!(report.errors().len(), 1);
    assert_eq!(report.joined_workers(), 1);
    assert_eq!(report.leases().len(), 1);
    assert!(matches!(
        report.leases()[0].release(),
        LeaseReleaseOutcome::Released
    ));
    assert!(processor.is_complete());
    assert!(Arc::ptr_eq(&report, &processor.stop().await));
    Ok(())
}
