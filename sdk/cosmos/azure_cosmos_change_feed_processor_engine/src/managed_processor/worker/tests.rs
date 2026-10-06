// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    run_worker, Arc, Duration, LeaseControl, ManagedProcessorOptions, RawChangeHandler, Semaphore,
};
use crate::managed_processor::{
    ManagedLeaseSnapshot, ManagedLeaseState, ManagedProcessorState, SharedState,
};
use crate::{
    ChangeFeedReadOptions, CosmosLeaseStore, LeaseOwnershipOptions, LeaseReleaseOutcome,
    LeaseRunOutcome, ProcessorEngine,
};
use azure_core::http::StatusCode;
use azure_data_cosmos_driver::{
    fault_injection::{
        CustomResponseBuilder, FaultInjectionConditionBuilder, FaultInjectionResultBuilder,
        FaultInjectionRuleBuilder, FaultOperationType,
    },
    in_memory_emulator::{
        ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
    },
    models::{
        AccountReference, ChangeFeedStartFrom, CosmosOperation, FeedRange, ItemReference,
        PartitionKey,
    },
    options::{DriverOptions, OperationOptions},
};
use std::{
    collections::BTreeMap,
    error::Error,
    sync::{
        atomic::{AtomicU32, Ordering},
        Mutex,
    },
};
use tokio::{sync::mpsc, time::timeout};

#[tokio::test]
async fn per_worker_stop_survives_ambiguous_checkpoint_and_replacement_session(
) -> Result<(), Box<dyn Error>> {
    let fault = Arc::new(
        FaultInjectionRuleBuilder::new(
            "worker-checkpoint-response-rejected",
            FaultInjectionResultBuilder::new()
                .with_probability(1.0)
                .with_custom_response(
                    CustomResponseBuilder::new(StatusCode::PreconditionFailed)
                        .with_body(
                            br#"{"code":"PreconditionFailed","message":"checkpoint ambiguity"}"#
                                .to_vec(),
                        )
                        .build(),
                )
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::ReplaceItem)
                .build(),
        )
        .with_hit_limit(1)
        .build(),
    );
    fault.disable();
    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(VirtualAccountConfig::new(
        vec![VirtualRegion::new(
            "East US",
            "https://eastus.emulator.local".parse()?,
        )],
    )?));
    emulator.store().create_database("db");
    for name in ["source", "leases"] {
        emulator.store().create_container_with_config(
            "db",
            name,
            serde_json::from_value(serde_json::json!({"paths":["/pk"],"kind":"Hash","version":2}))?,
            ContainerConfig::new().with_partition_count(1).build()?,
        );
    }
    let runtime = emulator
        .runtime_builder_with_fault_rules(vec![fault.clone()])
        .build()
        .await?;
    let driver = runtime
        .create_driver(
            DriverOptions::builder(AccountReference::with_master_key(
                "https://eastus.emulator.local".parse()?,
                "dGVzdGtleQ==",
            ))
            .build(),
        )
        .await?;
    let engine = ProcessorEngine::new(driver.clone(), "db", "source").await?;
    let checkpoint = engine
        .open_reader(ChangeFeedReadOptions::new(
            FeedRange::full(),
            ChangeFeedStartFrom::Beginning,
        ))
        .await?
        .to_continuation_token()?;
    driver
        .execute_singleton_operation(
            CosmosOperation::create_item(ItemReference::from_name(
                engine.container(),
                PartitionKey::from("P"),
                "event",
            ))
            .with_body(br#"{"id":"event","pk":"P"}"#.to_vec()),
            OperationOptions::default(),
        )
        .await?;
    let container = driver
        .resolve_container("db", "leases", OperationOptions::default())
        .await?;
    driver.execute_singleton_operation(CosmosOperation::create_item(ItemReference::from_name(
        &container, PartitionKey::from("P"), "lease",
    )).with_body(serde_json::to_vec(&serde_json::json!({
        "id":"lease","version":1,"ownership":{"owner":null,"generation":0},
        "range":FeedRange::full(),"checkpoint":checkpoint.as_str(),"lease_duration_ms":60000,"pk":"P",
    }))?), OperationOptions::default()).await?;
    let policy = LeaseOwnershipOptions::new(
        Duration::from_secs(60),
        Duration::from_secs(10),
        Duration::from_secs(10),
        Duration::from_secs(5),
    )?;
    let store = CosmosLeaseStore::from_resolved_single_writer(
        driver,
        container,
        PartitionKey::from("P"),
        "lease",
        policy,
    )?;
    let session = store.try_acquire("worker").await?.unwrap();
    let shared = Arc::new(Mutex::new(SharedState {
        state: ManagedProcessorState::Running,
        last_error: None,
        leases: BTreeMap::from([(
            "lease".into(),
            ManagedLeaseSnapshot {
                id: "lease".into(),
                state: ManagedLeaseState::Running,
                last_feed: None,
                last_callback: None,
                last_checkpoint: checkpoint.clone(),
                last_error: None,
            },
        )]),
    }));
    let stop = LeaseControl::default();
    let global = LeaseControl::default();
    let calls = Arc::new(AtomicU32::new(0));
    let handler: RawChangeHandler = {
        let stop = stop.clone();
        let calls = calls.clone();
        let fault = fault.clone();
        Arc::new(move |_| {
            let stop = stop.clone();
            let calls = calls.clone();
            let fault = fault.clone();
            Box::pin(async move {
                calls.fetch_add(1, Ordering::SeqCst);
                fault.enable();
                stop.stop();
                Ok(())
            })
        })
    };
    let (events, _received) = mpsc::channel(32);
    let report = timeout(
        Duration::from_secs(5),
        Box::pin(run_worker(
            engine,
            session,
            ManagedProcessorOptions::new("worker"),
            handler,
            Arc::new(Semaphore::new(1)),
            shared,
            global.clone(),
            stop,
            events,
        )),
    )
    .await?;
    assert_eq!(fault.hit_count(), 1);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(report.outcome(), LeaseRunOutcome::StoppedDrained);
    assert!(matches!(report.release(), LeaseReleaseOutcome::Released));
    assert_eq!(store.observe().await?.owner(), None);
    assert_ne!(store.observe().await?.checkpoint(), checkpoint.as_str());
    assert!(!global.is_lost());
    assert!(!super::stop_requested(&global));
    Ok(())
}
