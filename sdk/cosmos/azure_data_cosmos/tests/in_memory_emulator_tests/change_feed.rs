// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! In-memory emulator end-to-end tests for the change feed pull API, covering
//! both [`ChangeFeedMode::LatestVersion`] and
//! [`ChangeFeedMode::AllVersionsAndDeletes`].
//!
//! These drive the full SDK pipeline (`CosmosClient` → `ContainerClient` →
//! driver → in-memory emulator) so they exercise, end-to-end:
//!
//! * `A-IM` header emission and propagation (`Incremental Feed` vs
//!   `Full-Fidelity Feed`),
//! * `query_change_feed` mode dispatch,
//! * the `ChangeFeedStartFrom::Beginning` rejection for AllVersionsAndDeletes,
//!   and
//! * decoding per mode: callers bind the plain document type `T` and read
//!   `ChangeFeedItem<T>` envelopes in both modes.
//!
//! ## What the in-memory emulator models (and what it does not)
//!
//! The in-memory store only retains the *latest* state of each document — it
//! keeps no change log — so it cannot replay historical versions, deletes, or
//! pre-images. For a full-fidelity read it therefore synthesizes a minimal
//! `create` envelope per current document. That is enough to validate the AVAD
//! code path (headers, dispatch, envelope deserialization) deterministically.
//! Delete / `previous`-image envelopes remain covered by the model unit tests
//! in `src/models/change_feed_item.rs`, and true replay is a documented
//! follow-up against a full-fidelity-capable emulator or live account.
//!
//! Because the emulator returns the current state on every poll (no incremental
//! filtering, no 304), these tests poll a **single page** rather than draining
//! to an empty-page streak (which would loop forever collecting duplicates).

use std::error::Error;

use azure_data_cosmos::options::{ChangeFeedMode, ChangeFeedOptions, ChangeFeedStartFrom, Region};
use azure_data_cosmos::{
    AccountEndpoint, AccountReference, ContainerClient, CosmosClient, CosmosClientBuilder,
    CosmosRuntimeBuilder, FeedScope, RoutingStrategy,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

const EMULATOR_GATEWAY_URL: &str = "https://eastus.emulator.local";
const DB_NAME: &str = "changefeed-db";
const CONTAINER_NAME: &str = "changefeed-coll";
const PK_PATH: &str = "/pk";
const PARTITION_KEY: &str = "pk1";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TestItem {
    id: String,
    pk: String,
    value: i64,
}

impl TestItem {
    fn new(id: &str, value: i64) -> Self {
        Self {
            id: id.to_string(),
            pk: PARTITION_KEY.to_string(),
            value,
        }
    }
}

/// Builds an emulator-backed [`CosmosClient`], provisions a single-partition
/// container, and returns a ready-to-use [`ContainerClient`].
async fn setup() -> Result<ContainerClient, Box<dyn Error>> {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        azure_core::http::Url::parse(EMULATOR_GATEWAY_URL)?,
    )])?
    .with_consistency(ConsistencyLevel::Session);

    let emulator = std::sync::Arc::new(InMemoryEmulatorHttpClient::new(config));
    let store = emulator.store();

    store.create_database(DB_NAME);
    store.create_container(
        DB_NAME,
        CONTAINER_NAME,
        serde_json::from_value(serde_json::json!({
            "paths": [PK_PATH],
            "kind": "Hash",
            "version": 2
        }))?,
    );

    let account = AccountReference::with_authentication_key(
        EMULATOR_GATEWAY_URL.parse::<AccountEndpoint>()?,
        azure_core::credentials::Secret::new("dGVzdGtleQ=="),
    );

    let client: CosmosClient = CosmosClientBuilder::new()
        .with_runtime(
            CosmosRuntimeBuilder::from(emulator.runtime_builder())
                .build()
                .await?,
        )
        .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await?;

    let container = client
        .database_client(DB_NAME)
        .container_client(CONTAINER_NAME)
        .await?;

    Ok(container)
}

/// Inserts the given items under [`PARTITION_KEY`].
async fn insert_items(container: &ContainerClient, items: &[TestItem]) {
    for item in items {
        container
            .create_item(PARTITION_KEY, &item.id, item, None)
            .await
            .expect("create_item should succeed against the in-memory emulator");
    }
}

/// LatestVersion reads against the in-memory emulator yield envelopes whose
/// `current` carries the full document.
#[tokio::test]
async fn latest_version_returns_current_documents() {
    let container = setup().await.unwrap();
    let items = vec![
        TestItem::new("item-1", 1),
        TestItem::new("item-2", 2),
        TestItem::new("item-3", 3),
    ];
    insert_items(&container, &items).await;

    let mut pages = container
        .query_change_feed::<TestItem>(
            FeedScope::partition(PARTITION_KEY),
            ChangeFeedStartFrom::Beginning,
            None,
        )
        .await
        .unwrap();

    let page = pages
        .next()
        .await
        .expect("the change feed should yield a page")
        .expect("the page should not be an error");

    let mut returned: Vec<TestItem> = page
        .items()
        .iter()
        .map(|envelope| {
            envelope
                .current()
                .cloned()
                .expect("LatestVersion envelopes should carry current documents")
        })
        .collect();
    returned.sort_by(|a, b| a.id.cmp(&b.id));
    assert_eq!(returned, items);
}

/// AllVersionsAndDeletes reads yield full-fidelity envelopes: the whole
/// document is preserved under `current` and the change metadata is populated.
#[tokio::test]
async fn all_versions_and_deletes_returns_envelopes() {
    let container = setup().await.unwrap();
    let items = vec![TestItem::new("item-1", 10), TestItem::new("item-2", 20)];
    insert_items(&container, &items).await;

    let options = ChangeFeedOptions::default().with_mode(ChangeFeedMode::AllVersionsAndDeletes);
    let mut pages = container
        .query_change_feed::<TestItem>(
            FeedScope::partition(PARTITION_KEY),
            ChangeFeedStartFrom::Now,
            Some(options),
        )
        .await
        .unwrap();

    let page = pages
        .next()
        .await
        .expect("the change feed should yield a page")
        .expect("the page should not be an error");

    let mut envelopes = page.items().to_vec();
    assert_eq!(envelopes.len(), items.len());

    // Each envelope must preserve the whole document under `current` and carry
    // change metadata. The in-memory emulator synthesizes `create` envelopes.
    envelopes.sort_by(|a, b| {
        a.current()
            .map(|d| d.id.clone())
            .cmp(&b.current().map(|d| d.id.clone()))
    });
    for (envelope, expected) in envelopes.iter().zip(items.iter()) {
        use azure_data_cosmos::models::ChangeFeedOperationType;

        assert_eq!(
            envelope.operation_type(),
            Some(ChangeFeedOperationType::Create)
        );
        assert_eq!(envelope.current(), Some(expected));
        assert!(
            envelope.previous().is_none(),
            "the in-memory emulator does not synthesize pre-images"
        );
    }
}

/// AllVersionsAndDeletes rejects `ChangeFeedStartFrom::Beginning` with a
/// client-side error before issuing any request.
#[tokio::test]
async fn all_versions_and_deletes_rejects_beginning() {
    let container = setup().await.unwrap();

    let options = ChangeFeedOptions::default().with_mode(ChangeFeedMode::AllVersionsAndDeletes);
    let err = match container
        .query_change_feed::<TestItem>(
            FeedScope::partition(PARTITION_KEY),
            ChangeFeedStartFrom::Beginning,
            Some(options),
        )
        .await
    {
        Ok(_) => panic!("AllVersionsAndDeletes must reject ChangeFeedStartFrom::Beginning"),
        Err(err) => err,
    };

    // The rejection is a client-side validation error raised before any request
    // is issued, so it must carry a 400 BadRequest status.
    assert_eq!(
        u16::from(err.status().status_code()),
        400,
        "the rejection must be a client-side 400 BadRequest, got {err:?}"
    );
}
