// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    num::NonZeroU32,
    sync::{Arc, Mutex},
    time::Duration,
};

use azure_core::http::{headers::HeaderName, Method, Request, StatusCode, Url};
use azure_data_cosmos::{
    feed::{ChangeFeedPageIterator, ContinuationToken, FeedPage, FeedRange},
    models::ChangeFeedItem,
    options::{
        ChangeFeedOptions, ChangeFeedStartFrom, MaxItemCountHint, OperationOptions,
        PartitionFailoverOptions, PartitionTopologyCacheMode, Region,
    },
    AccountEndpoint, AccountReference, ContainerClient, CosmosClientBuilder, CosmosRuntimeBuilder,
    FeedScope, RoutingStrategy,
};
use azure_data_cosmos_driver::{
    in_memory_emulator::{
        ConsistencyLevel, ContainerConfig, Epk, InMemoryEmulatorHttpClient, RequestObserver,
        VirtualAccountConfig, VirtualRegion,
    },
    models::{PartitionKey, PartitionKeyDefinition},
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

const ENDPOINT: &str = "https://eastus.emulator.local";
const DATABASE: &str = "known-partition-db";
const CONTAINER: &str = "known-partition-coll";

#[derive(Clone, Debug)]
struct ObservedRequest {
    path: String,
    change_feed: bool,
    partition_id: Option<String>,
    min: Option<String>,
    max: Option<String>,
    continuation: Option<String>,
    session: Option<String>,
    intended_rid: Option<String>,
}

#[derive(Debug, Default)]
struct Observer(Mutex<Vec<ObservedRequest>>);

impl RequestObserver for Observer {
    fn on_request(&self, request: &Request) {
        let header = |name| {
            request
                .headers()
                .get_optional_str(&HeaderName::from_static(name))
                .map(str::to_owned)
        };
        self.0.lock().unwrap().push(ObservedRequest {
            path: request.url().path().to_owned(),
            change_feed: header("a-im").is_some(),
            partition_id: header("x-ms-documentdb-partitionkeyrangeid"),
            min: header("x-ms-start-epk"),
            max: header("x-ms-end-epk"),
            continuation: header("if-none-match"),
            session: header("x-ms-session-token"),
            intended_rid: header("x-ms-cosmos-intended-collection-rid"),
        });
    }
}

impl Observer {
    fn range_requests(&self) -> usize {
        self.0
            .lock()
            .unwrap()
            .iter()
            .filter(|request| request.path.ends_with("/pkranges"))
            .count()
    }

    fn feeds(&self) -> Vec<ObservedRequest> {
        self.0
            .lock()
            .unwrap()
            .iter()
            .filter(|request| request.change_feed && request.path.ends_with("/docs"))
            .cloned()
            .collect()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Item {
    id: String,
    pk: String,
}

struct Harness {
    emulator: Arc<InMemoryEmulatorHttpClient>,
    observer: Arc<Observer>,
    container: ContainerClient,
}

fn partition_definition() -> PartitionKeyDefinition {
    serde_json::from_value(serde_json::json!({
        "paths": ["/pk"], "kind": "Hash", "version": 2
    }))
    .unwrap()
}

impl Harness {
    async fn new(partition_count: u32) -> Self {
        let observer = Arc::new(Observer::default());
        let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
            "East US",
            Url::parse(ENDPOINT).unwrap(),
        )])
        .unwrap()
        .with_consistency(ConsistencyLevel::Session);
        let emulator = Arc::new(
            InMemoryEmulatorHttpClient::new(config).with_request_observer(observer.clone()),
        );
        emulator.store().create_database(DATABASE);
        emulator.store().create_container_with_config(
            DATABASE,
            CONTAINER,
            partition_definition(),
            ContainerConfig::new()
                .with_partition_count(partition_count)
                .with_partition_key_range_page_size(1)
                .build()
                .unwrap(),
        );
        let client = CosmosClientBuilder::new()
            .with_runtime(
                CosmosRuntimeBuilder::from(emulator.runtime_builder())
                    .build()
                    .await
                    .unwrap(),
            )
            .with_partition_failover_options(
                PartitionFailoverOptions::builder()
                    .with_partition_topology_cache_mode(PartitionTopologyCacheMode::Lazy)
                    .build()
                    .unwrap(),
            )
            .build(
                AccountReference::with_authentication_key(
                    ENDPOINT.parse::<AccountEndpoint>().unwrap(),
                    azure_core::credentials::Secret::new("dGVzdGtleQ=="),
                ),
                RoutingStrategy::ProximityTo(Region::EAST_US),
            )
            .await
            .unwrap();
        let container = client
            .database_client(DATABASE)
            .container_client(CONTAINER, None)
            .await
            .unwrap();
        assert_eq!(observer.range_requests(), 0);
        Self {
            emulator,
            observer,
            container,
        }
    }

    async fn seed(&self, items: &[Item]) -> String {
        let mut session = String::new();
        for item in items {
            let mut request = Request::new(
                Url::parse(&format!("{ENDPOINT}/dbs/{DATABASE}/colls/{CONTAINER}/docs")).unwrap(),
                Method::Post,
            );
            request.headers_mut().insert(
                HeaderName::from_static("x-ms-documentdb-partitionkey"),
                serde_json::to_string(&[&item.pk]).unwrap(),
            );
            request.set_body(serde_json::to_vec(item).unwrap());
            let response = self.emulator.execute_request(&request).await.unwrap();
            assert_eq!(response.status(), StatusCode::Created);
            session = response
                .headers()
                .get_str(&HeaderName::from_static("x-ms-session-token"))
                .unwrap()
                .to_owned();
        }
        session
    }

    async fn feed(
        &self,
        range: FeedRange,
        options: ChangeFeedOptions,
    ) -> ChangeFeedPageIterator<ChangeFeedItem<Item>> {
        self.container
            .query_change_feed(
                FeedScope::range(range),
                ChangeFeedStartFrom::Beginning,
                Some(options),
            )
            .await
            .unwrap()
    }

    async fn recreate(&self) {
        let request = Request::new(
            Url::parse(&format!("{ENDPOINT}/dbs/{DATABASE}/colls/{CONTAINER}")).unwrap(),
            Method::Delete,
        );
        assert_eq!(
            self.emulator
                .execute_request(&request)
                .await
                .unwrap()
                .status(),
            StatusCode::NoContent
        );
        self.emulator
            .store()
            .create_container(DATABASE, CONTAINER, partition_definition());
    }
}

fn fixtures() -> (Vec<Item>, Vec<String>) {
    let definition = partition_definition();
    let mut fixtures: Vec<_> = (0..8)
        .map(|i| {
            let item = Item {
                id: format!("item-{i}"),
                pk: format!("key-{i}"),
            };
            let range = FeedRange::for_partition(PartitionKey::from(item.pk.clone()), &definition);
            (range.min_inclusive().clone(), item)
        })
        .collect();
    fixtures.sort_by(|left, right| left.0.cmp(&right.0));
    let bounds = fixtures.iter().map(|(epk, _)| epk.to_hex()).collect();
    let items = fixtures.into_iter().map(|(_, item)| item).collect();
    (items, bounds)
}

fn range(min: &str, max: &str) -> FeedRange {
    FeedRange::new(min.try_into().unwrap(), max.try_into().unwrap()).unwrap()
}

fn options(id: &str) -> ChangeFeedOptions {
    ChangeFeedOptions::default()
        .with_partition_key_range_id(id)
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(2).unwrap()))
}

fn current(page: FeedPage<ChangeFeedItem<Item>>) -> Vec<Item> {
    page.into_items()
        .into_iter()
        .map(|item| item.current().unwrap().clone())
        .collect()
}

async fn remaining(
    feed: &mut ChangeFeedPageIterator<ChangeFeedItem<Item>>,
    idle_pages: u32,
) -> Vec<Item> {
    let mut items = Vec::new();
    let mut idle = 0;
    for _ in 0..32 {
        let page = feed.next().await.unwrap().unwrap();
        if page.items().is_empty() {
            idle += 1;
            if idle == idle_pages {
                return items;
            }
        } else {
            idle = 0;
            items.extend(current(page));
        }
    }
    panic!("change feed did not become idle within 32 pages");
}

fn assert_items(mut actual: Vec<Item>, expected: &[Item]) {
    let mut expected = expected.to_vec();
    actual.sort_by(|left, right| left.id.cmp(&right.id));
    expected.sort_by(|left, right| left.id.cmp(&right.id));
    assert_eq!(actual, expected);
}

#[tokio::test]
async fn lazy_hint_preserves_epk_bounds_pagination_and_session_without_topology() {
    let harness = Harness::new(1).await;
    let (items, bounds) = fixtures();
    harness.seed(&items).await;
    let scope = range(&bounds[1], &bounds[7]);
    let mut feed = harness.feed(scope.clone(), options("0")).await;
    let first = feed.next().await.unwrap().unwrap();
    let first_etag = first.headers().etag().unwrap().to_string();
    let first_session = first.headers().session_token().unwrap().as_str().to_owned();
    let mut actual = current(first);
    assert_eq!(actual, items[1..3]);
    actual.extend(current(feed.next().await.unwrap().unwrap()));
    assert_eq!(actual, items[1..5]);
    let token = feed.to_continuation_token().unwrap();
    let token = ContinuationToken::from_string(token.as_str().to_owned());
    drop(feed);

    let mut resumed = harness
        .feed(scope, options("0").with_continuation_token(token))
        .await;
    actual.extend(remaining(&mut resumed, 1).await);
    assert_eq!(actual, items[1..7]);
    assert_eq!(harness.observer.range_requests(), 0);
    let requests = harness.observer.feeds();
    assert_eq!(requests.len(), 4);
    assert_eq!(requests[0].continuation, None);
    assert_eq!(requests[0].session, None);
    assert_eq!(
        requests[1].continuation.as_deref(),
        Some(first_etag.as_str())
    );
    assert_eq!(requests[1].session.as_deref(), Some(first_session.as_str()));
    for request in requests {
        assert_eq!(request.partition_id.as_deref(), Some("0"));
        assert_eq!(request.min.as_deref(), Some(bounds[1].as_str()));
        assert_eq!(request.max.as_deref(), Some(bounds[7].as_str()));
        assert!(request.intended_rid.is_some());
    }
}

#[tokio::test]
async fn lazy_feed_without_hint_still_loads_all_topology_pages() {
    let harness = Harness::new(4).await;
    let mut feed = harness
        .feed(FeedRange::full(), ChangeFeedOptions::default())
        .await;
    assert!(feed.next().await.unwrap().unwrap().items().is_empty());
    // Four topology pages plus the terminal conditional read returning 304.
    assert_eq!(harness.observer.range_requests(), 5);
}

#[tokio::test]
async fn explicit_session_progress_is_preserved_without_topology() {
    let harness = Harness::new(1).await;
    let (items, _) = fixtures();
    let session = harness.seed(&items).await;
    let mut feed = harness
        .feed(
            FeedRange::full(),
            options("0").with_session_token(session.clone()),
        )
        .await;
    assert_eq!(current(feed.next().await.unwrap().unwrap()), items[..2]);
    assert_eq!(harness.observer.range_requests(), 0);
    let requests = harness.observer.feeds();
    assert_eq!(requests[0].session.as_deref(), Some(session.as_str()));
    assert_eq!(requests[0].min.as_deref(), Some(""));
    assert_eq!(requests[0].max.as_deref(), Some("FF"));
}

#[tokio::test]
async fn unavailable_explicit_session_is_not_silently_dropped() {
    let harness = Harness::new(1).await;
    let session = "0:1#999999#1=999999";
    let mut operation = OperationOptions::default();
    operation.max_session_retry_count = Some(0);
    let mut feed = harness
        .feed(
            FeedRange::full(),
            options("0")
                .with_session_token(session)
                .with_operation_options(operation),
        )
        .await;
    assert!(feed.next().await.unwrap().is_err());
    let requests = harness.observer.feeds();
    assert!(!requests.is_empty());
    for request in requests {
        assert_eq!(request.session.as_deref(), Some(session));
    }
}

#[tokio::test]
async fn stale_partition_hint_refreshes_topology_without_widening_scope() {
    let harness = Harness::new(1).await;
    let (items, bounds) = fixtures();
    harness.seed(&items).await;
    let mut feed = harness
        .feed(range(&bounds[1], &bounds[7]), options("999"))
        .await;
    assert_items(remaining(&mut feed, 1).await, &items[1..7]);
    assert!(harness.observer.range_requests() > 0);
    let requests = harness.observer.feeds();
    assert_eq!(requests[0].partition_id.as_deref(), Some("999"));
    for request in &requests[1..] {
        assert_eq!(request.partition_id.as_deref(), Some("0"));
        assert_eq!(request.min.as_deref(), Some(bounds[1].as_str()));
        assert_eq!(request.max.as_deref(), Some(bounds[7].as_str()));
    }
}

#[tokio::test]
async fn split_after_checkpoint_preserves_progress_and_owned_epk_slice() {
    let harness = Harness::new(1).await;
    let (items, bounds) = fixtures();
    harness.seed(&items).await;
    let scope = range(&bounds[1], &bounds[7]);
    let mut feed = harness.feed(scope.clone(), options("0")).await;
    let mut actual = current(feed.next().await.unwrap().unwrap());
    let token = feed.to_continuation_token().unwrap();
    drop(feed);
    let children = harness
        .emulator
        .store()
        .split_partition_and_wait(
            DATABASE,
            CONTAINER,
            0,
            Epk::try_from(bounds[4].as_str()).unwrap(),
            Duration::ZERO,
        )
        .await
        .unwrap();
    let mut resumed = harness
        .feed(scope.clone(), options("0").with_continuation_token(token))
        .await;
    actual.extend(remaining(&mut resumed, 2).await);
    assert_items(actual, &items[1..7]);
    assert!(harness.observer.range_requests() > 0);
    let requests = harness.observer.feeds();
    assert_eq!(requests[1].partition_id.as_deref(), Some("0"));
    assert!(requests[1].continuation.is_some());
    let child_ids: Vec<_> = children.iter().map(ToString::to_string).collect();
    for request in &requests[2..] {
        assert!(child_ids.contains(request.partition_id.as_ref().unwrap()));
        let requested = range(request.min.as_ref().unwrap(), request.max.as_ref().unwrap());
        assert!(requested.is_subset_of(&scope));
        assert!(request.continuation.is_some());
    }
}

#[tokio::test]
async fn merge_after_checkpoint_does_not_read_the_other_parent_slice() {
    let harness = Harness::new(1).await;
    let (items, bounds) = fixtures();
    harness.seed(&items).await;
    let children = harness
        .emulator
        .store()
        .split_partition_and_wait(
            DATABASE,
            CONTAINER,
            0,
            Epk::try_from(bounds[4].as_str()).unwrap(),
            Duration::ZERO,
        )
        .await
        .unwrap();
    let scope = range(&bounds[1], &bounds[4]);
    let hint = children[0].to_string();
    let mut feed = harness.feed(scope.clone(), options(&hint)).await;
    let mut actual = current(feed.next().await.unwrap().unwrap());
    let token = feed.to_continuation_token().unwrap();
    drop(feed);
    assert_eq!(harness.observer.range_requests(), 0);
    let merged = harness
        .emulator
        .store()
        .merge_partitions_and_wait(
            DATABASE,
            CONTAINER,
            children[0],
            children[1],
            Duration::ZERO,
        )
        .await
        .unwrap();
    let mut resumed = harness
        .feed(scope, options(&hint).with_continuation_token(token))
        .await;
    actual.extend(remaining(&mut resumed, 1).await);
    assert_items(actual, &items[1..4]);
    assert!(harness.observer.range_requests() > 0);
    let requests = harness.observer.feeds();
    assert_eq!(requests[1].partition_id.as_deref(), Some(hint.as_str()));
    let merged = merged.to_string();
    for request in &requests[2..] {
        assert_eq!(request.partition_id.as_deref(), Some(merged.as_str()));
        assert_eq!(request.min.as_deref(), Some(bounds[1].as_str()));
        assert_eq!(request.max.as_deref(), Some(bounds[4].as_str()));
        assert!(request.continuation.is_some());
    }
}

#[tokio::test]
async fn hinted_feed_does_not_follow_a_recreated_container() {
    let harness = Harness::new(1).await;
    let (items, bounds) = fixtures();
    harness.seed(&items).await;
    let mut feed = harness
        .feed(range(&bounds[1], &bounds[7]), options("0"))
        .await;
    assert_eq!(current(feed.next().await.unwrap().unwrap()), items[1..3]);
    let old_rid = harness.observer.feeds()[0].intended_rid.clone();
    harness.recreate().await;
    harness.seed(&items).await;
    let error = feed.next().await.unwrap().unwrap_err();
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);
    assert_eq!(
        error.status().sub_status(),
        Some(azure_data_cosmos_driver::error::status_codes::substatus::COLLECTION_RID_MISMATCH)
    );
    for request in harness.observer.feeds() {
        assert_eq!(request.intended_rid, old_rid);
        assert_eq!(request.partition_id.as_deref(), Some("0"));
    }
}

#[tokio::test]
async fn fresh_full_range_hint_does_not_follow_a_recreated_container() {
    let harness = Harness::new(1).await;
    let mut feed = harness.feed(FeedRange::full(), options("0")).await;
    assert!(harness.observer.feeds().is_empty());
    harness.recreate().await;
    let (items, _) = fixtures();
    harness.seed(&items).await;

    let error = feed.next().await.unwrap().unwrap_err();
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);
    assert_eq!(
        error.status().sub_status(),
        Some(azure_data_cosmos_driver::error::status_codes::substatus::COLLECTION_RID_MISMATCH)
    );
    let requests = harness.observer.feeds();
    assert_eq!(
        requests.len(),
        1,
        "the hint must never reach the replacement RID"
    );
    assert_eq!(requests[0].partition_id.as_deref(), Some("0"));
    assert_eq!(requests[0].min.as_deref(), Some(""));
    assert_eq!(requests[0].max.as_deref(), Some("FF"));
    assert!(requests[0].intended_rid.is_some());
}

#[tokio::test]
async fn invalid_hint_or_scope_is_rejected_before_data_requests() {
    let harness = Harness::new(1).await;
    let logical = FeedRange::for_partition(PartitionKey::from("key"), &partition_definition());
    let hpk: PartitionKeyDefinition = serde_json::from_value(serde_json::json!({
        "paths": ["/tenant", "/user"], "kind": "MultiHash", "version": 2
    }))
    .unwrap();
    let prefix = FeedRange::for_partition(PartitionKey::from("tenant"), &hpk);
    let mut cases: Vec<_> = ["", " ", "0 1", "0\n", "\t0", "0\u{7f}"]
        .into_iter()
        .map(|id| (FeedScope::range(FeedRange::full()), id))
        .collect();
    cases.extend([
        (FeedScope::partition("key"), "0"),
        (FeedScope::range(logical), "0"),
        (FeedScope::range(prefix), "0"),
        (FeedScope::range(range("20", "20")), "0"),
    ]);
    for (scope, id) in cases {
        let result = harness
            .container
            .query_change_feed::<Item>(scope, ChangeFeedStartFrom::Beginning, Some(options(id)))
            .await;
        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("invalid hint or scope was accepted: {id:?}"),
        };
        assert_eq!(error.status().status_code(), StatusCode::BadRequest);
    }
    assert!(harness.observer.feeds().is_empty());
    assert_eq!(harness.observer.range_requests(), 0);
}
