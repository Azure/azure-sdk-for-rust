// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    latest_case, CallbackBatch, Document, Event, Observations, Operation, Semantics, Start,
    TestResult, Workload,
};
use azure_core::{
    credentials::Secret,
    http::{headers::HeaderName, Request},
};
use azure_data_cosmos::{
    models::ChangeFeedItem, options::Region, AccountReference, ChangeFeedProcessorBuilder,
    ContainerClient, CosmosClientBuilder, CosmosRuntimeBuilder, LatestVersion, RoutingStrategy,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ContainerConfig, InMemoryEmulatorHttpClient, RequestObserver, VirtualAccountConfig,
    VirtualRegion,
};
use futures::channel::mpsc;
use std::{
    collections::BTreeSet,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use time::OffsetDateTime;

#[derive(Debug, Default)]
struct StartRequests {
    headers: Mutex<Vec<Option<String>>>,
}

impl RequestObserver for StartRequests {
    fn on_request(&self, request: &Request) {
        if request.url().path().ends_with("/docs")
            && request
                .headers()
                .get_optional_str(&HeaderName::from_static("a-im"))
                .is_some()
        {
            self.headers.lock().unwrap().push(
                request
                    .headers()
                    .get_optional_str(&HeaderName::from_static("if-modified-since"))
                    .map(str::to_owned),
            );
        }
    }
}

async fn containers(
    observer: Option<Arc<StartRequests>>,
) -> TestResult<(ContainerClient, ContainerClient)> {
    let endpoint = "https://eastus.emulator.local";
    let emulator =
        InMemoryEmulatorHttpClient::new(VirtualAccountConfig::new(vec![VirtualRegion::new(
            "East US",
            endpoint.parse()?,
        )])?);
    let emulator = Arc::new(match observer {
        Some(observer) => emulator.with_request_observer(observer),
        None => emulator,
    });
    let store = emulator.store();
    store.create_database("mixed-workload");
    store.create_container_with_config(
        "mixed-workload",
        "source",
        serde_json::from_value(serde_json::json!({"paths":["/pk"],"kind":"Hash","version":2}))?,
        ContainerConfig::default().with_partition_count(3),
    );
    let runtime = CosmosRuntimeBuilder::from(emulator.runtime_builder())
        .build()
        .await?;
    let client = CosmosClientBuilder::new()
        .with_runtime(runtime)
        .build(
            AccountReference::with_authentication_key(
                endpoint.parse()?,
                Secret::new("dGVzdGtleQ=="),
            ),
            RoutingStrategy::ProximityTo(Region::EAST_US),
        )
        .await?;
    let database = client.database_client("mixed-workload");
    database
        .create_container(
            serde_json::from_value(serde_json::json!({
                "id": "leases",
                "partitionKey": {"paths":["/id"],"kind":"Hash","version":2},
                "defaultTtl": -1
            }))?,
            None,
        )
        .await?;
    Ok((
        database.container_client("source", None).await?,
        database.container_client("leases", None).await?,
    ))
}

pub(in crate::change_feed_processor) async fn workload_containers(
) -> TestResult<(ContainerClient, ContainerClient)> {
    containers(None).await
}

#[tokio::test]
async fn mixed_latest_now_small_pages() -> TestResult {
    let (source, leases) = containers(None).await?;
    latest_case(source, leases, Start::Now, 1).await
}

#[tokio::test]
async fn mixed_latest_now_larger_pages() -> TestResult {
    let (source, leases) = containers(None).await?;
    latest_case(source, leases, Start::Now, 64).await
}

#[tokio::test]
async fn mixed_latest_beginning() -> TestResult {
    let (source, leases) = containers(None).await?;
    latest_case(source, leases, Start::Beginning, 2).await
}

#[tokio::test]
async fn processor_forwards_time_start_header() -> TestResult {
    let observer = Arc::new(StartRequests::default());
    let (source, leases) = containers(Some(observer.clone())).await?;
    let item = Document {
        id: "time-probe".into(),
        pk: Some("time-probe".into()),
        revision: Some(1),
    };
    source
        .create_item("time-probe", "time-probe", item, None)
        .await?;
    let calls = Arc::new(AtomicUsize::new(0));
    let received = calls.clone();
    let processor =
        ChangeFeedProcessorBuilder::new(LatestVersion, source, leases, "time-start", "worker")
            .with_start_time(OffsetDateTime::from_unix_timestamp(1_700_000_000)?)
            .handle_change_feed(move |_, changes: Vec<ChangeFeedItem<Document>>| {
                received.fetch_add(changes.len(), Ordering::SeqCst);
                async { Ok(()) }
            })
            .build()?;
    processor.start().await?;
    let result = tokio::time::timeout(Duration::from_secs(5), async {
        while calls.load(Ordering::SeqCst) == 0 {
            tokio::task::yield_now().await;
        }
    })
    .await;
    processor.stop().await?;
    result?;
    let headers = observer.headers.lock().unwrap();
    assert!(!headers.is_empty());
    assert!(
        headers
            .iter()
            .all(|value| value.as_deref() == Some("Tue, 14 Nov 2023 22:13:20 GMT")),
        "{headers:?}"
    );
    Ok(())
}

fn workload(semantics: Semantics) -> Workload {
    Workload {
        name: "test-run".into(),
        semantics,
    }
}

#[test]
fn delete_observation_uses_metadata_when_current_is_empty() -> TestResult {
    let workload = workload(Semantics::AllVersions);
    let change: ChangeFeedItem<Document> = serde_json::from_value(serde_json::json!({
        "current": {},
        "metadata": {
            "operationType":"delete",
            "id":"test-run-deleted",
            "partitionKey":["test-run"]
        }
    }))?;
    assert_eq!(
        workload.observe(&change)?,
        Some(Event {
            id: "test-run-deleted".into(),
            revision: None,
            operation: Operation::Delete
        }),
    );
    let latest = Workload {
        name: workload.name.clone(),
        semantics: Semantics::Latest,
    };
    assert!(latest
        .observe(&change)
        .unwrap_err()
        .to_string()
        .contains("unexpectedly delivered delete"));
    Ok(())
}

#[test]
fn full_fidelity_assertions_reject_missing_operation_and_wrong_partition() -> TestResult {
    let workload = workload(Semantics::AllVersions);
    let missing: ChangeFeedItem<Document> = serde_json::from_value(serde_json::json!({
        "current":{"id":"test-run-created","pk":"test-run","revision":1}
    }))?;
    assert!(workload
        .observe(&missing)
        .unwrap_err()
        .to_string()
        .contains("invalid operation metadata"));
    let wrong_partition: ChangeFeedItem<Document> = serde_json::from_value(serde_json::json!({
        "metadata":{"operationType":"delete","id":"test-run-deleted","partitionKey":["another-run"]}
    }))?;
    assert!(workload
        .observe(&wrong_partition)
        .unwrap_err()
        .to_string()
        .contains("unexpected partition key"));
    Ok(())
}

#[tokio::test]
async fn collector_accepts_replay_but_rejects_wrong_revision() -> TestResult {
    let workload = workload(Semantics::Latest);
    let expected = BTreeSet::from([workload.event("created", 1, Operation::Create)]);
    let change: ChangeFeedItem<Document> = serde_json::from_value(serde_json::json!({
        "current":{"id":"test-run-created","pk":"test-run","revision":1}
    }))?;
    let (sender, mut receiver) = mpsc::unbounded();
    sender.unbounded_send(CallbackBatch {
        lease: "lease".into(),
        changes: vec![change.clone(), change],
    })?;
    let mut observations = Observations {
        expected: expected.clone(),
        seen: BTreeSet::new(),
        fence_lease: None,
    };
    observations
        .collect_until(&workload, &mut receiver, &expected)
        .await?;
    assert_eq!(observations.seen, expected);

    let wrong: ChangeFeedItem<Document> = serde_json::from_value(serde_json::json!({
        "current":{"id":"test-run-created","pk":"test-run","revision":99}
    }))?;
    sender.unbounded_send(CallbackBatch {
        lease: "lease".into(),
        changes: vec![wrong],
    })?;
    observations.seen.clear();
    assert!(observations
        .collect_until(&workload, &mut receiver, &expected)
        .await
        .unwrap_err()
        .to_string()
        .contains("Unexpected callback change"));
    Ok(())
}
