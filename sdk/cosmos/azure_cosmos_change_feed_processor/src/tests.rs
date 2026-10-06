// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{ChangeFeedProcessor, ProcessorEngine, ReadChangesOptions, Url};
use std::{error::Error, num::NonZeroU32, sync::Arc};

use azure_core::http::StatusCode;
use azure_data_cosmos_driver::{
    in_memory_emulator::{
        ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig,
        VirtualRegion,
    },
    models::{AccountReference, CosmosOperation, ItemReference, PartitionKey},
    options::{DriverOptions, OperationOptions},
};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct Document {
    id: String,
    value: u64,
}

pub(super) async fn setup(partition_count: u32) -> Result<ChangeFeedProcessor, Box<dyn Error>> {
    Ok(setup_with_driver(partition_count).await?.0)
}

pub(super) async fn setup_with_driver(
    partition_count: u32,
) -> Result<
    (
        ChangeFeedProcessor,
        Arc<azure_data_cosmos_driver::CosmosDriver>,
    ),
    Box<dyn Error>,
> {
    let endpoint: Url = "https://eastus.emulator.local".parse()?;
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new("East US", endpoint.clone())])?
        .with_consistency(ConsistencyLevel::Session);
    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    let store = emulator.store();
    store.create_database("feed-db");
    store.create_container_with_config(
        "feed-db",
        "feed",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"], "kind": "Hash", "version": 2
        }))?,
        ContainerConfig::new()
            .with_partition_count(partition_count)
            .build()?,
    );
    let runtime = emulator.runtime_builder().build().await?;
    let driver = runtime
        .create_driver(
            DriverOptions::builder(AccountReference::with_master_key(endpoint, "dGVzdGtleQ=="))
                .build(),
        )
        .await?;
    let container = driver
        .resolve_container("feed-db", "feed", OperationOptions::default())
        .await?;
    for value in 0..3 {
        let id = format!("item-{value}");
        let item = ItemReference::from_name(&container, PartitionKey::from("pk"), id.clone());
        driver
            .execute_singleton_operation(
                CosmosOperation::create_item(item).with_body(serde_json::to_vec(
                    &serde_json::json!({"id": id, "pk": "pk", "value": value}),
                )?),
                OperationOptions::default(),
            )
            .await?;
    }
    Ok((
        ChangeFeedProcessor {
            engine: ProcessorEngine::new(driver.clone(), "feed-db", "feed").await?,
            prepared: None,
        },
        driver,
    ))
}

#[tokio::test]
async fn reads_typed_changes_through_engine_and_driver() -> Result<(), Box<dyn Error>> {
    let processor = setup(1).await?;
    let page = processor
        .read_page::<Document>(&ReadChangesOptions::default())
        .await?;
    let mut actual: Vec<_> = page
        .items()
        .iter()
        .map(|item| {
            (
                item.current().unwrap().id.as_str(),
                item.current().unwrap().value,
            )
        })
        .collect();
    actual.sort_unstable();
    assert_eq!(actual, vec![("item-0", 0), ("item-1", 1), ("item-2", 2)]);
    assert_eq!(page.status(), StatusCode::Ok);
    assert!(page.request_charge().is_some_and(|charge| charge > 0.0));
    assert!(page.activity_id().is_some_and(|id| !id.is_empty()));
    assert!(!page.continuation().is_empty());
    Ok(())
}

#[tokio::test]
async fn resumes_without_duplicates_and_returns_idle_page() -> Result<(), Box<dyn Error>> {
    let processor = setup(1).await?;
    let mut options =
        ReadChangesOptions::default().with_max_item_count(NonZeroU32::new(1).unwrap());
    let mut actual = Vec::new();
    let mut idle = false;
    for _ in 0..8 {
        let page = processor.read_page::<Document>(&options).await?;
        assert!(!page.continuation().is_empty());
        options = options.with_continuation(page.continuation());
        actual.extend(
            page.items()
                .iter()
                .map(|item| item.current().unwrap().id.clone()),
        );
        if page.status() == StatusCode::NotModified {
            assert!(page.items().is_empty());
            idle = true;
            break;
        }
    }
    actual.sort_unstable();
    assert_eq!(actual, vec!["item-0", "item-1", "item-2"]);
    assert!(
        idle,
        "feed must return an idle page after consuming the seeded changes"
    );
    let still_idle = processor.read_page::<Document>(&options).await?;
    assert_eq!(still_idle.status(), StatusCode::NotModified);
    assert!(still_idle.items().is_empty());
    Ok(())
}

#[tokio::test]
async fn invalid_token_is_an_error_not_an_empty_page() -> Result<(), Box<dyn Error>> {
    let processor = setup(1).await?;
    let options = ReadChangesOptions::default().with_continuation("c1.not-a-valid-token");
    let result = processor.read_page::<Document>(&options).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn decoding_failure_does_not_skip_changes() -> Result<(), Box<dyn Error>> {
    #[derive(Deserialize)]
    struct IncompatibleDocument {
        #[serde(rename = "value")]
        _value: bool,
    }
    let processor = setup(1).await?;
    let options = ReadChangesOptions::default();
    let error = processor
        .read_page::<IncompatibleDocument>(&options)
        .await
        .err()
        .ok_or("decoding must fail")?;
    assert!(error.source().is_some());
    let page = processor.read_page::<Document>(&options).await?;
    let mut actual: Vec<_> = page
        .items()
        .iter()
        .map(|item| item.current().unwrap().id.as_str())
        .collect();
    actual.sort_unstable();
    assert_eq!(actual, vec!["item-0", "item-1", "item-2"]);
    Ok(())
}

#[tokio::test]
async fn idle_partition_does_not_end_cross_partition_feed() -> Result<(), Box<dyn Error>> {
    let processor = setup(4).await?;
    let mut options = ReadChangesOptions::default();
    let first = processor.read_page::<Document>(&options).await?;
    assert_eq!(first.status(), StatusCode::NotModified);
    assert!(first.items().is_empty());
    options = options.with_continuation(first.continuation());
    let mut actual = Vec::new();
    for _ in 0..8 {
        let page = processor.read_page::<Document>(&options).await?;
        actual.extend(
            page.items()
                .iter()
                .map(|item| item.current().unwrap().id.clone()),
        );
        options = options.with_continuation(page.continuation());
        if actual.len() >= 3 {
            break;
        }
    }
    actual.sort_unstable();
    assert_eq!(actual, vec!["item-0", "item-1", "item-2"]);
    Ok(())
}
