// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    credentials::Secret,
    http::{StatusCode, Url},
};
use azure_data_cosmos::{
    models::ChangeFeedItem, options::Region, AccountReference, AllVersionsAndDeletes,
    ChangeFeedProcessorBuilder, ChangeFeedProcessorMode, ContainerClient, CosmosClientBuilder,
    FeedScope, LatestVersion, RoutingStrategy,
};
use futures::TryStreamExt;
use serde_json::Value;
use std::{
    error::Error,
    io,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

#[path = "change_feed_processor/backlog.rs"]
mod backlog;
#[path = "change_feed_processor/workload.rs"]
mod workload;

async fn records(leases: &ContainerClient, processor: &str) -> TestResult<Vec<Value>> {
    let query = leases
        .query_items::<Value>("SELECT * FROM c", FeedScope::full_container(), None)
        .await?;
    let mut records = Box::pin(query).try_collect::<Vec<_>>().await?;
    records.retain(|record| record["workload"]["processorName"] == processor);
    records.sort_by(|left, right| left["id"].as_str().cmp(&right["id"].as_str()));
    Ok(records)
}

async fn exercise_start<M: ChangeFeedProcessorMode, N: ChangeFeedProcessorMode>(
    source: &ContainerClient,
    leases: &ContainerClient,
    processor_name: &str,
    mode: M,
    incompatible: N,
    expected_mode: &str,
) -> TestResult {
    let calls = Arc::new(AtomicUsize::new(0));
    let callback_calls = Arc::clone(&calls);
    let processor = ChangeFeedProcessorBuilder::new(
        mode,
        source.clone(),
        leases.clone(),
        processor_name,
        "sdk-initializer",
    )
    .handle_change_feed(move |_, _: Vec<ChangeFeedItem<Value>>| {
        callback_calls.fetch_add(1, Ordering::SeqCst);
        async { Ok(()) }
    })
    .build()?;
    assert!(records(leases, processor_name).await?.is_empty());
    processor.start().await?;
    assert!(processor.is_started());
    processor.stop().await?;
    assert!(!processor.is_started());
    let stored = records(leases, processor_name).await?;
    for suffix in [".definition", ".plan", ".info"] {
        let matching: Vec<_> = stored
            .iter()
            .filter(|record| record["id"].as_str().unwrap().ends_with(suffix))
            .collect();
        assert_eq!(matching.len(), 1);
        assert_eq!(matching[0]["workload"]["mode"], expected_mode);
    }
    let initial_leases: Vec<_> = stored
        .iter()
        .filter(|record| record.get("range").is_some())
        .collect();
    assert_eq!(
        initial_leases.len(),
        source.read_feed_ranges(None).await?.len()
    );
    assert!(initial_leases
        .iter()
        .all(|lease| lease.get("owner").is_none()));
    processor.start().await?;
    processor.stop().await?;
    let before = records(leases, processor_name).await?;
    for previous in initial_leases {
        if let Some(continuation) = previous.get("continuation") {
            let current = before
                .iter()
                .find(|record| record["id"] == previous["id"])
                .unwrap();
            assert_eq!(&current["continuation"], continuation);
        }
    }

    let other = ChangeFeedProcessorBuilder::new(
        incompatible,
        source.clone(),
        leases.clone(),
        processor_name,
        "incompatible-worker",
    )
    .handle_change_feed(|_, _: Vec<ChangeFeedItem<Value>>| async { Ok(()) })
    .build()?;
    assert_eq!(
        other.start().await.unwrap_err().status().status_code(),
        StatusCode::Conflict
    );
    assert!(!other.is_started());
    assert_eq!(records(leases, processor_name).await?, before);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    Ok(())
}

async fn live_containers() -> TestResult<(ContainerClient, ContainerClient)> {
    if std::env::var("COSMOS_POLLING_ALLOW_LIVE")? != "true" {
        return Err(io::Error::other("Live polling requires separate explicit opt-in.").into());
    }
    if std::env::var("COSMOS_BOOTSTRAP_ALLOW_LIVE")? != "true" {
        return Err(io::Error::other("Live bootstrap tests require explicit opt-in.").into());
    }
    let database = std::env::var("COSMOS_BOOTSTRAP_LIVE_DATABASE")?;
    let endpoint: Url = std::env::var("COSMOS_BOOTSTRAP_LIVE_ENDPOINT")?.parse()?;
    if !database.starts_with("cfp-bootstrap-")
        || endpoint.scheme() != "https"
        || !endpoint
            .host_str()
            .is_some_and(|host| host.ends_with(".documents.azure.com"))
    {
        return Err(io::Error::other(
            "Live tests require an isolated test database and Azure Cosmos HTTPS endpoint.",
        )
        .into());
    }
    let client = CosmosClientBuilder::new()
        .build(
            AccountReference::with_authentication_key(
                endpoint.as_str().parse()?,
                Secret::new(std::env::var("COSMOS_BOOTSTRAP_LIVE_KEY")?),
            ),
            RoutingStrategy::ProximityTo(Region::CENTRAL_US),
        )
        .await?;
    let database = client.database_client(database);
    let source = database.container_client("source", None).await?;
    let leases = database.container_client("leases", None).await?;
    Ok((source, leases))
}

#[tokio::test]
#[ignore = "Requires explicit live polling opt-in and isolated pre-provisioned source/leases containers."]
async fn cfp_start_bootstraps_workloads() -> TestResult {
    let (source, leases) = live_containers().await?;
    exercise_start(
        &source,
        &leases,
        "sdk-latest",
        LatestVersion,
        AllVersionsAndDeletes,
        "latestVersion",
    )
    .await?;
    exercise_start(
        &source,
        &leases,
        "sdk-all-versions",
        AllVersionsAndDeletes,
        LatestVersion,
        "allVersionsAndDeletes",
    )
    .await?;
    Ok(())
}
