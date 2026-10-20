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
//! * the `ChangeFeedStartFrom::Beginning` and `ChangeFeedStartFrom::PointInTime`
//!   rejections for AllVersionsAndDeletes (gated by the emulator, mirroring the
//!   service), and
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

use std::{error::Error, sync::Arc};

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

async fn setup_client() -> Result<(CosmosClient, Arc<InMemoryEmulatorHttpClient>), Box<dyn Error>> {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        azure_core::http::Url::parse(EMULATOR_GATEWAY_URL)?,
    )])?
    .with_consistency(ConsistencyLevel::Session);

    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
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

    Ok((client, emulator))
}

async fn setup() -> Result<ContainerClient, Box<dyn Error>> {
    let (client, _) = setup_client().await?;
    Ok(client
        .database_client(DB_NAME)
        .container_client(CONTAINER_NAME, None)
        .await?)
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

    let mut pages = Box::pin(container.query_change_feed::<TestItem>(
        FeedScope::partition(PARTITION_KEY),
        ChangeFeedStartFrom::Beginning,
        None,
    ))
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

    let options = ChangeFeedOptions::default().with_mode(ChangeFeedMode::AllVersionsAndDeletes);
    let mut pages = Box::pin(container.query_change_feed::<TestItem>(
        FeedScope::partition(PARTITION_KEY),
        ChangeFeedStartFrom::Now,
        Some(options),
    ))
    .await
    .unwrap();

    let initial = pages
        .next()
        .await
        .expect("the change feed should yield an initial page")
        .expect("the initial page should not be an error");
    assert!(initial.items().is_empty());

    insert_items(&container, &items).await;

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

/// AllVersionsAndDeletes rejects `ChangeFeedStartFrom::Beginning`. The client
/// no longer validates this itself; the request is issued and the emulator
/// (standing in for the service) returns `400 Bad Request` on the first poll.
#[tokio::test]
async fn all_versions_and_deletes_rejects_beginning() {
    let container = setup().await.unwrap();

    let options = ChangeFeedOptions::default().with_mode(ChangeFeedMode::AllVersionsAndDeletes);
    let mut pages = Box::pin(container.query_change_feed::<TestItem>(
        FeedScope::partition(PARTITION_KEY),
        ChangeFeedStartFrom::Beginning,
        Some(options),
    ))
    .await
    .unwrap();

    // A `Beginning` start sends no `If-None-Match`, which the emulator rejects
    // for a full-fidelity read with a 400 BadRequest on the first page poll.
    let err = pages
        .next()
        .await
        .expect("the change feed should yield a page")
        .expect_err("AllVersionsAndDeletes must reject ChangeFeedStartFrom::Beginning");

    assert_eq!(
        u16::from(err.status().status_code()),
        400,
        "the rejection must be a 400 BadRequest, got {err:?}"
    );
}

/// AllVersionsAndDeletes rejects `ChangeFeedStartFrom::PointInTime`. As with
/// `Beginning`, the client defers to the emulator (service stand-in), which
/// returns `400 Bad Request` on the first poll.
#[tokio::test]
async fn all_versions_and_deletes_rejects_point_in_time() {
    use time::OffsetDateTime;

    let container = setup().await.unwrap();

    let options = ChangeFeedOptions::default().with_mode(ChangeFeedMode::AllVersionsAndDeletes);
    let mut pages = Box::pin(container.query_change_feed::<TestItem>(
        FeedScope::partition(PARTITION_KEY),
        ChangeFeedStartFrom::PointInTime(OffsetDateTime::now_utc()),
        Some(options),
    ))
    .await
    .unwrap();

    // A `PointInTime` start sends `If-Modified-Since`, which the emulator
    // rejects for a full-fidelity read with a 400 BadRequest on the first poll.
    let err = pages
        .next()
        .await
        .expect("the change feed should yield a page")
        .expect_err("AllVersionsAndDeletes must reject ChangeFeedStartFrom::PointInTime");

    assert_eq!(
        u16::from(err.status().status_code()),
        400,
        "the rejection must be a 400 BadRequest, got {err:?}"
    );
}

#[cfg(test)]
mod processor_lifecycle_tests {
    use super::{
        setup, setup_client, TestItem, CONTAINER_NAME, DB_NAME, EMULATOR_GATEWAY_URL, PARTITION_KEY,
    };
    use azure_core::http::StatusCode;
    use azure_data_cosmos::{
        models::ChangeFeedItem, options::MaxItemCountHint, AllVersionsAndDeletes,
        ChangeFeedProcessor, ChangeFeedProcessorBuilder, ChangeFeedProcessorContext,
        ChangeFeedProcessorMode, ContainerClient, FeedScope, LatestVersion,
    };
    use azure_data_cosmos_driver::{
        models::{AccountReference as DriverAccountReference, CosmosOperation, DatabaseReference},
        options::{DriverOptions, OperationOptions},
    };
    use futures::TryStreamExt;
    use serde_json::Value;
    use std::{
        error::Error,
        num::NonZeroU32,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
        time::Duration,
    };

    async fn setup_processor() -> Result<(ContainerClient, ContainerClient), Box<dyn Error>> {
        let (client, emulator) = setup_client().await?;
        let runtime = emulator.runtime_builder().build().await?;
        let driver = runtime
            .create_driver(
                DriverOptions::builder(DriverAccountReference::with_account_key(
                    EMULATOR_GATEWAY_URL.parse()?,
                    "dGVzdGtleQ==",
                ))
                .build(),
            )
            .await?;
        driver
            .execute_singleton_operation(
                CosmosOperation::create_container(DatabaseReference::from_name(
                    driver.account().clone(),
                    DB_NAME,
                ))
                .with_body(serde_json::to_vec(&serde_json::json!({
                    "id": "processor-leases",
                    "partitionKey": {"paths": ["/id"], "kind": "Hash", "version": 2},
                    "defaultTtl": -1
                }))?),
                OperationOptions::default(),
            )
            .await?;
        let database = client.database_client(DB_NAME);
        Ok((
            database.container_client(CONTAINER_NAME, None).await?,
            database.container_client("processor-leases", None).await?,
        ))
    }

    async fn records(leases: &ContainerClient) -> Result<Vec<Value>, Box<dyn Error>> {
        let query = leases
            .query_items::<Value>("SELECT * FROM c", FeedScope::full_container(), None)
            .await?;
        let mut records = Box::pin(query).try_collect::<Vec<_>>().await?;
        records.sort_by(|left, right| left["id"].as_str().cmp(&right["id"].as_str()));
        Ok(records)
    }

    async fn assert_bootstrap_and_processing<M: ChangeFeedProcessorMode>(
        mode: M,
        expected_mode: &str,
    ) -> Result<(), Box<dyn Error>> {
        let (container, leases) = setup_processor().await?;
        let expected_range_count = container.read_feed_ranges(None).await?.len();
        let calls = Arc::new(AtomicUsize::new(0));
        let callback_calls = Arc::clone(&calls);
        let (release, released) = futures::channel::oneshot::channel::<()>();
        let released = Arc::new(async_lock::Mutex::new(Some(released)));
        let observed = Arc::new(std::sync::Mutex::new(Vec::new()));
        let callback_observed = observed.clone();
        let processor: ChangeFeedProcessor<M, TestItem, _> = ChangeFeedProcessorBuilder::new(
            mode,
            container.clone(),
            leases.clone(),
            "processor",
            "worker-01",
        )
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(200).unwrap()))
        .with_feed_poll_delay(Duration::from_millis(5))
        .with_min_scale_count(1)
        .with_max_scale_count(4)
        .with_lease_verification_enabled_on_restart(true)
        .handle_change_feed(
            move |context: ChangeFeedProcessorContext, items: Vec<ChangeFeedItem<TestItem>>| {
                let released = released.clone();
                let observed = callback_observed.clone();
                let calls = callback_calls.clone();
                async move {
                    observed.lock().unwrap().push((
                        context.lease_token().to_owned(),
                        items
                            .iter()
                            .map(|item| item.current().unwrap().id.clone())
                            .collect::<Vec<_>>(),
                    ));
                    calls.fetch_add(1, Ordering::SeqCst);
                    if let Some(released) = released.lock().await.take() {
                        released.await.unwrap();
                    }
                    Ok(())
                }
            },
        )
        .with_max_item_count(MaxItemCountHint::ServerDecides)
        .with_feed_poll_delay(Duration::from_millis(5))
        .with_min_scale_count(0)
        .with_max_scale_count(0)
        .with_lease_verification_enabled_on_restart(false)
        .build()?;

        assert!(records(&leases).await?.is_empty());
        assert!(!processor.is_started());
        let error = processor
            .current_state()
            .await
            .expect_err("monitoring must not report an empty snapshot");
        assert_eq!(error.status().status_code(), StatusCode::NotImplemented);
        assert!(error.response().is_none());
        assert!(error
            .to_string()
            .contains("Change feed processor monitoring is not implemented."));
        assert!(!processor.is_started());
        assert_eq!(
            Arc::strong_count(&calls),
            2,
            "processor must retain the callback"
        );
        processor.stop().await?;
        processor.start().await?;
        assert!(processor.is_started());
        assert_eq!(
            processor.start().await.unwrap_err().status().status_code(),
            StatusCode::Conflict
        );
        let before = tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                let current = records(&leases).await.unwrap();
                let range_leases: Vec<_> = current
                    .iter()
                    .filter(|r| r.get("range").is_some())
                    .collect();
                if range_leases.len() == expected_range_count
                    && range_leases.iter().all(|r| r.get("continuation").is_some())
                {
                    break current;
                }
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await?;
        for lease in before.iter().filter(|r| r.get("range").is_some()) {
            assert_eq!(lease["workload"]["mode"], expected_mode);
        }
        for suffix in [".definition", ".plan", ".info"] {
            assert_eq!(
                before
                    .iter()
                    .filter(|r| r["id"].as_str().unwrap().ends_with(suffix))
                    .count(),
                1
            );
        }
        container
            .create_item(
                PARTITION_KEY,
                "processed",
                TestItem::new("processed", 42),
                None,
            )
            .await?;
        tokio::time::timeout(Duration::from_secs(5), async {
            while calls.load(Ordering::SeqCst) == 0 {
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await?;
        let (id, items) = observed.lock().unwrap()[0].clone();
        assert_eq!(items, vec!["processed"]);
        let previous = before.iter().find(|r| r["id"] == id).unwrap()["continuation"].clone();
        assert_eq!(
            records(&leases)
                .await?
                .iter()
                .find(|r| r["id"] == id)
                .unwrap()["continuation"],
            previous
        );
        release.send(()).unwrap();
        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                if records(&leases)
                    .await
                    .unwrap()
                    .iter()
                    .find(|r| r["id"] == id)
                    .unwrap()["continuation"]
                    != previous
                {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await?;
        processor.stop().await?;
        processor.stop().await?;
        assert!(!processor.is_started());
        processor.start().await?;
        tokio::time::sleep(Duration::from_millis(40)).await;
        processor.stop().await?;
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        drop(processor);
        assert_eq!(
            Arc::strong_count(&calls),
            1,
            "dropping the processor must release the callback"
        );
        Ok(())
    }

    #[tokio::test]
    async fn latest_version_bootstraps_and_checkpoints_after_callback() -> Result<(), Box<dyn Error>>
    {
        assert_bootstrap_and_processing(LatestVersion, "latestVersion").await
    }

    #[tokio::test]
    async fn all_versions_and_deletes_bootstraps_and_checkpoints_after_callback(
    ) -> Result<(), Box<dyn Error>> {
        assert_bootstrap_and_processing(AllVersionsAndDeletes, "allVersionsAndDeletes").await
    }

    async fn assert_mode_conflict<M: ChangeFeedProcessorMode, N: ChangeFeedProcessorMode>(
        initial: M,
        incompatible: N,
    ) -> Result<(), Box<dyn Error>> {
        let (source, leases) = setup_processor().await?;
        let first = ChangeFeedProcessorBuilder::new(
            initial,
            source.clone(),
            leases.clone(),
            "orders",
            "first",
        )
        .handle_change_feed(|_, _: Vec<ChangeFeedItem<TestItem>>| async { Ok(()) })
        .build()?;
        first.start().await?;
        first.stop().await?;
        let before = records(&leases).await?;
        let other = ChangeFeedProcessorBuilder::new(
            incompatible,
            source,
            leases.clone(),
            "orders",
            "other",
        )
        .handle_change_feed(|_, _: Vec<ChangeFeedItem<TestItem>>| async { Ok(()) })
        .build()?;
        assert_eq!(
            other.start().await.unwrap_err().status().status_code(),
            StatusCode::Conflict
        );
        assert!(!other.is_started());
        assert_eq!(records(&leases).await?, before);
        Ok(())
    }

    #[tokio::test]
    async fn start_propagates_mode_conflicts_in_both_directions() -> Result<(), Box<dyn Error>> {
        assert_mode_conflict(LatestVersion, AllVersionsAndDeletes).await?;
        assert_mode_conflict(AllVersionsAndDeletes, LatestVersion).await
    }

    #[tokio::test]
    async fn start_propagates_invalid_lease_container() -> Result<(), Box<dyn Error>> {
        let source = setup().await?;
        let processor = ChangeFeedProcessorBuilder::new(
            LatestVersion,
            source.clone(),
            source,
            "orders",
            "worker",
        )
        .handle_change_feed(|_, _: Vec<ChangeFeedItem<TestItem>>| async { Ok(()) })
        .build()?;
        let error = processor.start().await.unwrap_err();
        assert_eq!(error.status().status_code(), StatusCode::BadRequest);
        assert!(error.to_string().contains("defaultTtl"));
        assert!(!processor.is_started());
        Ok(())
    }

    #[tokio::test]
    async fn start_forwards_restart_verification() -> Result<(), Box<dyn Error>> {
        let (source, leases) = setup_processor().await?;
        let first = ChangeFeedProcessorBuilder::new(
            LatestVersion,
            source.clone(),
            leases.clone(),
            "orders",
            "first",
        )
        .handle_change_feed(|_, _: Vec<ChangeFeedItem<TestItem>>| async { Ok(()) })
        .build()?;
        first.start().await?;
        first.stop().await?;
        let before = records(&leases).await?;
        let id = before
            .iter()
            .find(|record| record.get("range").is_some())
            .unwrap()["id"]
            .as_str()
            .unwrap()
            .to_owned();
        leases.delete_item(id.clone(), &id, None).await?;
        let without = records(&leases).await?;
        first.start().await?;
        first.stop().await?;
        assert_eq!(records(&leases).await?, without);
        let verifying = ChangeFeedProcessorBuilder::new(
            LatestVersion,
            source,
            leases.clone(),
            "orders",
            "verifier",
        )
        .with_lease_verification_enabled_on_restart(true)
        .handle_change_feed(|_, _: Vec<ChangeFeedItem<TestItem>>| async { Ok(()) })
        .build()?;
        verifying.start().await?;
        verifying.stop().await?;
        let mut after = records(&leases).await?;
        assert_eq!(after.len(), before.len());
        let restored = after.iter().find(|record| record["id"] == id).unwrap();
        assert!(restored.get("continuation").is_none());
        assert!(restored.get("owner").is_none());
        after.retain(|record| record["id"] != id);
        assert_eq!(after, without);
        Ok(())
    }

    #[tokio::test]
    async fn callback_failure_replays_without_committing_progress() -> Result<(), Box<dyn Error>> {
        let (source, leases) = setup_processor().await?;
        source
            .create_item(PARTITION_KEY, "replay", TestItem::new("replay", 1), None)
            .await?;
        let calls = Arc::new(AtomicUsize::new(0));
        let callback_calls = calls.clone();
        let lease_token = Arc::new(std::sync::Mutex::new(String::new()));
        let callback_lease = lease_token.clone();
        let processor = ChangeFeedProcessorBuilder::new(
            LatestVersion,
            source,
            leases.clone(),
            "orders",
            "worker",
        )
        .with_start_from_beginning()
        .handle_change_feed(
            move |context: ChangeFeedProcessorContext, items: Vec<ChangeFeedItem<TestItem>>| {
                assert_eq!(items.len(), 1);
                assert_eq!(items[0].current().unwrap().id, "replay");
                *callback_lease.lock().unwrap() = context.lease_token().to_owned();
                let fail = callback_calls.fetch_add(1, Ordering::SeqCst) == 0;
                async move {
                    if fail {
                        Err(azure_data_cosmos::CosmosError::builder()
                            .with_status(azure_data_cosmos::CosmosStatus::new(
                                StatusCode::ServiceUnavailable,
                            ))
                            .with_message("Application callback failed.")
                            .build())
                    } else {
                        Ok(())
                    }
                }
            },
        )
        .build()?;
        processor.start().await?;
        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                let all = records(&leases).await.unwrap();
                let id = lease_token.lock().unwrap().clone();
                if calls.load(Ordering::SeqCst) == 1
                    && all
                        .iter()
                        .any(|r| r["id"] == id && r.get("owner").is_none())
                {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await?;
        processor.stop().await?;
        let id = lease_token.lock().unwrap().clone();
        assert!(records(&leases)
            .await?
            .iter()
            .find(|r| r["id"] == id)
            .unwrap()
            .get("continuation")
            .is_none());
        processor.start().await?;
        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                if calls.load(Ordering::SeqCst) == 2
                    && records(&leases)
                        .await
                        .unwrap()
                        .iter()
                        .any(|r| r["id"] == id && r.get("continuation").is_some())
                {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await?;
        processor.stop().await?;
        assert_eq!(calls.load(Ordering::SeqCst), 2);
        Ok(())
    }

    #[tokio::test]
    async fn stop_cancels_pending_callback_without_checkpoint() -> Result<(), Box<dyn Error>> {
        let (source, leases) = setup_processor().await?;
        source
            .create_item(PARTITION_KEY, "pending", TestItem::new("pending", 1), None)
            .await?;
        let calls = Arc::new(AtomicUsize::new(0));
        let callback_calls = calls.clone();
        let lease_token = Arc::new(std::sync::Mutex::new(String::new()));
        let callback_lease = lease_token.clone();
        let processor = ChangeFeedProcessorBuilder::new(
            LatestVersion,
            source,
            leases.clone(),
            "orders",
            "worker",
        )
        .with_start_from_beginning()
        .handle_change_feed(
            move |context: ChangeFeedProcessorContext, _: Vec<ChangeFeedItem<TestItem>>| {
                *callback_lease.lock().unwrap() = context.lease_token().to_owned();
                callback_calls.fetch_add(1, Ordering::SeqCst);
                futures::future::pending::<azure_data_cosmos::Result<()>>()
            },
        )
        .build()?;
        processor.start().await?;
        tokio::time::timeout(Duration::from_secs(5), async {
            while calls.load(Ordering::SeqCst) == 0 {
                tokio::task::yield_now().await;
            }
        })
        .await?;
        tokio::time::timeout(Duration::from_secs(5), processor.stop()).await??;
        assert!(!processor.is_started());
        let stored = records(&leases).await?;
        let id = lease_token.lock().unwrap().clone();
        assert!(stored
            .iter()
            .find(|r| r["id"] == id)
            .unwrap()
            .get("continuation")
            .is_none());
        assert!(stored
            .iter()
            .filter(|r| r.get("range").is_some())
            .all(|r| r.get("owner").is_none()));
        Ok(())
    }

    #[tokio::test]
    async fn start_time_reads_existing_changes() -> Result<(), Box<dyn Error>> {
        let (source, leases) = setup_processor().await?;
        source
            .create_item(
                PARTITION_KEY,
                "after-time",
                TestItem::new("after-time", 1),
                None,
            )
            .await?;
        let calls = Arc::new(AtomicUsize::new(0));
        let callback_calls = calls.clone();
        let processor = ChangeFeedProcessorBuilder::new(
            LatestVersion,
            source,
            leases.clone(),
            "orders",
            "worker",
        )
        .with_start_time(time::OffsetDateTime::from_unix_timestamp(1_700_000_000)?)
        .handle_change_feed(move |_, items: Vec<ChangeFeedItem<TestItem>>| {
            assert_eq!(
                items
                    .iter()
                    .map(|item| item.current().unwrap().id.as_str())
                    .collect::<Vec<_>>(),
                vec!["after-time"]
            );
            callback_calls.fetch_add(1, Ordering::SeqCst);
            async { Ok(()) }
        })
        .build()?;
        processor.start().await?;
        tokio::time::timeout(Duration::from_secs(5), async {
            while calls.load(Ordering::SeqCst) == 0 {
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await?;
        processor.stop().await?;
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        Ok(())
    }
}
