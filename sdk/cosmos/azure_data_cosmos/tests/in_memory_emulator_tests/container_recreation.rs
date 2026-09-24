// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    num::NonZeroU32,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

use azure_core::http::{headers::HeaderName, Context, Method, Request, StatusCode, Url};
use azure_data_cosmos::diagnostics::{DiagnosticsContext, DiagnosticsHandler};
use azure_data_cosmos::feed::FeedRange;
use azure_data_cosmos::{
    models::{ContainerProperties, PartitionKeyDefinition, ThroughputProperties},
    options::{CreateContainerOptions, ItemReadOptions, MaxItemCountHint, QueryOptions, Region},
    AccountEndpoint, AccountReference, ContainerClient, CosmosClient, CosmosClientBuilder,
    CosmosRuntimeBuilder, FeedScope, PartitionKey, Query, RoutingStrategy, SubStatusCode,
    TransactionalBatch,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, RequestObserver, VirtualAccountConfig,
    VirtualRegion,
};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};

const GATEWAY_URL: &str = "https://eastus.emulator.local";
const ACCOUNT_KEY: &str = "dGVzdGtleQ==";
const DATABASE_NAME: &str = "recreation-db";
const CONTAINER_NAME: &str = "recreation-coll";

static INTENDED_COLLECTION_RID: HeaderName =
    HeaderName::from_static("x-ms-cosmos-intended-collection-rid");
static SESSION_TOKEN: HeaderName = HeaderName::from_static("x-ms-session-token");
static PARTITION_KEY: HeaderName = HeaderName::from_static("x-ms-documentdb-partitionkey");

#[derive(Clone, Debug)]
struct RequestSnapshot {
    method: Method,
    path: String,
    intended_rid: Option<String>,
    session_token: Option<String>,
    partition_key: Option<String>,
    is_query: bool,
    is_batch: bool,
}

#[derive(Debug, Default)]
struct RecordingObserver {
    requests: Mutex<Vec<RequestSnapshot>>,
}

#[derive(Default)]
struct RecordingDiagnosticsHandler {
    completions: AtomicUsize,
    last_request_count: AtomicUsize,
}

impl DiagnosticsHandler for RecordingDiagnosticsHandler {
    fn handle(&self, diagnostics: &DiagnosticsContext, _context: &Context<'_>) {
        self.completions.fetch_add(1, Ordering::SeqCst);
        self.last_request_count
            .store(diagnostics.request_count(), Ordering::SeqCst);
    }
}

impl RecordingObserver {
    fn clear(&self) {
        self.requests
            .lock()
            .expect("request observer mutex poisoned")
            .clear();
    }

    fn item_creates(&self) -> Vec<RequestSnapshot> {
        self.requests
            .lock()
            .expect("request observer mutex poisoned")
            .iter()
            .filter(|request| {
                request.method == Method::Post
                    && request.path == format!("/dbs/{DATABASE_NAME}/colls/{CONTAINER_NAME}/docs")
                    && !request.is_query
                    && !request.is_batch
            })
            .cloned()
            .collect()
    }

    fn query_requests(&self) -> Vec<RequestSnapshot> {
        self.requests
            .lock()
            .expect("request observer mutex poisoned")
            .iter()
            .filter(|request| request.is_query)
            .cloned()
            .collect()
    }

    fn batch_requests(&self) -> Vec<RequestSnapshot> {
        self.requests
            .lock()
            .expect("request observer mutex poisoned")
            .iter()
            .filter(|request| request.is_batch)
            .cloned()
            .collect()
    }
}

impl RequestObserver for RecordingObserver {
    fn on_request(&self, request: &Request) {
        self.requests
            .lock()
            .expect("request observer mutex poisoned")
            .push(RequestSnapshot {
                method: request.method(),
                path: request.url().path().to_owned(),
                intended_rid: request
                    .headers()
                    .get_optional_str(&INTENDED_COLLECTION_RID)
                    .map(str::to_owned),
                session_token: request
                    .headers()
                    .get_optional_str(&SESSION_TOKEN)
                    .map(str::to_owned),
                partition_key: request
                    .headers()
                    .get_optional_str(&PARTITION_KEY)
                    .map(str::to_owned),
                is_query: request
                    .headers()
                    .get_optional_str(&HeaderName::from_static("x-ms-documentdb-isquery"))
                    .is_some(),
                is_batch: request
                    .headers()
                    .get_optional_str(&HeaderName::from_static("x-ms-cosmos-is-batch-request"))
                    .is_some(),
            });
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct TestItem {
    id: String,
    pk: String,
    value: i64,
}

struct Harness {
    emulator: Arc<InMemoryEmulatorHttpClient>,
    observer: Arc<RecordingObserver>,
    diagnostics_handler: Arc<RecordingDiagnosticsHandler>,
    client: CosmosClient,
    container: ContainerClient,
}

impl Harness {
    async fn new() -> Self {
        Self::new_with_container("/pk".into(), 400).await
    }

    async fn new_with_container(partition_key: PartitionKeyDefinition, throughput: u64) -> Self {
        let observer = Arc::new(RecordingObserver::default());
        let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
            "East US",
            Url::parse(GATEWAY_URL).unwrap(),
        )])
        .unwrap()
        .with_consistency(ConsistencyLevel::Session);
        let emulator = Arc::new(
            InMemoryEmulatorHttpClient::new(config).with_request_observer(observer.clone()),
        );
        let account = AccountReference::with_authentication_key(
            GATEWAY_URL.parse::<AccountEndpoint>().unwrap(),
            azure_core::credentials::Secret::new(ACCOUNT_KEY),
        );
        let diagnostics_handler = Arc::new(RecordingDiagnosticsHandler::default());
        let client = CosmosClientBuilder::new()
            .with_runtime(
                CosmosRuntimeBuilder::from(emulator.runtime_builder())
                    .build()
                    .await
                    .unwrap(),
            )
            .with_diagnostics_handler(diagnostics_handler.clone())
            .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
            .await
            .unwrap();
        client.create_database(DATABASE_NAME, None).await.unwrap();
        let database = client.database_client(DATABASE_NAME);
        database
            .create_container(
                container_properties(partition_key),
                Some(
                    CreateContainerOptions::default()
                        .with_throughput(ThroughputProperties::manual(throughput)),
                ),
            )
            .await
            .unwrap();
        let container = database
            .container_client(CONTAINER_NAME, None)
            .await
            .unwrap();

        Self {
            emulator,
            observer,
            diagnostics_handler,
            client,
            container,
        }
    }

    async fn recreate(&self) {
        self.recreate_with("/pk".into(), 400).await;
    }

    async fn recreate_with(
        &self,
        partition_key: PartitionKeyDefinition,
        throughput: u64,
    ) -> ContainerProperties {
        self.container.delete(None).await.unwrap();
        let replacement = self
            .client
            .database_client(DATABASE_NAME)
            .create_container(
                container_properties(partition_key),
                Some(
                    CreateContainerOptions::default()
                        .with_throughput(ThroughputProperties::manual(throughput)),
                ),
            )
            .await
            .unwrap()
            .into_model()
            .unwrap();
        self.observer.clear();
        replacement
    }

    async fn seed_raw<T>(&self, partition_key: &str, item: &T)
    where
        T: Serialize,
    {
        let mut request = Request::new(
            Url::parse(&format!(
                "{GATEWAY_URL}/dbs/{DATABASE_NAME}/colls/{CONTAINER_NAME}/docs"
            ))
            .unwrap(),
            Method::Post,
        );
        request.headers_mut().insert(
            PARTITION_KEY.clone(),
            serde_json::to_string(&[partition_key]).unwrap(),
        );
        request.set_body(serde_json::to_vec(item).unwrap());
        let response = self.emulator.execute_request(&request).await.unwrap();
        assert_eq!(response.status(), StatusCode::Created);
        self.observer.clear();
    }
}

fn container_properties(partition_key: PartitionKeyDefinition) -> ContainerProperties {
    ContainerProperties::new(CONTAINER_NAME.to_owned(), partition_key)
}

fn item(id: &str, value: i64) -> TestItem {
    TestItem {
        id: id.to_owned(),
        pk: "pk1".to_owned(),
        value,
    }
}

fn item_for_path(id: &str, partition_key_path: &str, value: i64) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        (partition_key_path): "pk1",
        "value": value,
    })
}

fn intended_rid_transitions(requests: &[RequestSnapshot]) -> Vec<&str> {
    requests
        .iter()
        .filter_map(|request| request.intended_rid.as_deref())
        .fold(Vec::new(), |mut transitions, rid| {
            if transitions.last().copied() != Some(rid) {
                transitions.push(rid);
            }
            transitions
        })
}

#[tokio::test]
async fn long_lived_client_recovers_across_supported_operations() {
    let harness = Harness::new().await;

    let old_response = harness
        .container
        .create_item("pk1", "old", &item("old", 0), None)
        .await
        .unwrap();
    let old_session = old_response
        .headers()
        .session_token()
        .expect("create response has a session token")
        .clone();

    harness.recreate().await;
    let explicit_token_error = harness
        .container
        .read_item(
            "pk1",
            "old",
            Some(ItemReadOptions::default().with_session_token(old_session.clone())),
        )
        .await
        .expect_err("an explicit token must not cross container recreation");
    assert_eq!(
        explicit_token_error.status().status_code(),
        StatusCode::BadRequest
    );
    assert_eq!(
        explicit_token_error
            .status()
            .sub_status()
            .map(|status| status.value()),
        Some(1024)
    );

    let created = harness
        .container
        .create_item("pk1", "point", &item("point", 1), None)
        .await
        .unwrap();
    assert_eq!(created.status(), StatusCode::Created);
    let attempts = harness.observer.item_creates();
    assert_eq!(attempts.len(), 2, "the stale write must retry exactly once");
    assert_ne!(attempts[0].intended_rid, attempts[1].intended_rid);
    assert!(
        attempts[1].session_token.is_none(),
        "the replacement generation must not receive the old session token"
    );
    let point: TestItem = harness
        .container
        .read_item("pk1", "point", None)
        .await
        .unwrap()
        .into_model()
        .unwrap();
    assert_eq!(point, item("point", 1));
    let warmed_cache_token_error = harness
        .container
        .read_item(
            "pk1",
            "point",
            Some(ItemReadOptions::default().with_session_token(old_session)),
        )
        .await
        .expect_err("a warmed name cache must not bypass explicit-token safety");
    assert_eq!(
        warmed_cache_token_error.status().status_code(),
        StatusCode::BadRequest
    );

    harness.recreate().await;
    let batch = TransactionalBatch::new("pk1")
        .create_item(item("batch", 2))
        .unwrap()
        .read_item("batch", None);
    let batch = harness
        .container
        .execute_transactional_batch(batch, None)
        .await
        .unwrap()
        .into_model()
        .unwrap();
    assert_eq!(
        batch
            .results()
            .iter()
            .map(|result| result.status_code())
            .collect::<Vec<_>>(),
        vec![201, 200]
    );

    harness.recreate().await;
    harness.seed_raw("pk1", &item("query", 3)).await;
    harness.seed_raw("pk1", &item("query-2", 4)).await;
    let queried: Vec<TestItem> = Box::pin(
        harness.container.query_items(
            Query::from("SELECT * FROM c"),
            FeedScope::partition("pk1"),
            Some(
                QueryOptions::default()
                    .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap())),
            ),
        ),
    )
    .await
    .unwrap()
    .try_collect()
    .await
    .unwrap();
    assert_eq!(queried, vec![item("query", 3), item("query-2", 4)]);

    let mut pages = harness
        .container
        .query_items::<TestItem>(
            Query::from("SELECT * FROM c"),
            FeedScope::full_container(),
            None,
        )
        .await
        .unwrap()
        .into_pages();
    pages.next().await.expect("query returns one page").unwrap();
    let continuation = pages.to_continuation_token().unwrap();

    harness.recreate().await;
    harness.seed_raw("pk1", &item("replacement-query", 4)).await;
    let replacement: TestItem = harness
        .container
        .read_item("pk1", "replacement-query", None)
        .await
        .unwrap()
        .into_model()
        .unwrap();
    assert_eq!(replacement, item("replacement-query", 4));
    let resume_result = harness
        .container
        .query_items::<TestItem>(
            Query::from("SELECT * FROM c"),
            FeedScope::full_container(),
            Some(QueryOptions::default().with_continuation_token(continuation)),
        )
        .await;
    let resume_error = match resume_result {
        Ok(_) => panic!("a continuation token must not cross container recreation"),
        Err(error) => error,
    };
    assert_eq!(resume_error.status().status_code(), StatusCode::BadRequest);

    let completions_before_throughput = harness
        .diagnostics_handler
        .completions
        .load(Ordering::SeqCst);
    let throughput = harness
        .container
        .read_throughput(None)
        .await
        .unwrap()
        .expect("replacement container has dedicated throughput");
    assert_eq!(throughput.throughput(), Some(400));
    assert_eq!(
        harness
            .diagnostics_handler
            .completions
            .load(Ordering::SeqCst),
        completions_before_throughput + 1,
        "throughput recreation recovery must dispatch one completion"
    );
    assert_eq!(
        harness
            .diagnostics_handler
            .last_request_count
            .load(Ordering::SeqCst),
        2,
        "the completion must retain both offer-query requests"
    );
}

#[tokio::test]
async fn changed_definition_recovers_supported_operations_once() {
    let harness = Harness::new().await;

    let replacement = harness.recreate_with("/replacementPk".into(), 600).await;
    let replacement_rid = replacement
        .system_properties
        .resource_id
        .as_deref()
        .expect("replacement container has a RID");
    let point = item_for_path("point-new-definition", "replacementPk", 1);
    let response = harness
        .container
        .create_item("pk1", "point-new-definition", &point, None)
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::Created);
    let attempts = harness.observer.item_creates();
    assert_eq!(attempts.len(), 2, "the stale write must retry exactly once");
    assert_ne!(attempts[0].intended_rid, attempts[1].intended_rid);
    assert_eq!(attempts[1].intended_rid.as_deref(), Some(replacement_rid));
    assert_eq!(attempts[1].partition_key.as_deref(), Some(r#"["pk1"]"#));
    assert!(attempts[1].session_token.is_none());
    let read: serde_json::Value = harness
        .container
        .read_item("pk1", "point-new-definition", None)
        .await
        .unwrap()
        .into_model()
        .unwrap();
    assert_eq!(read["id"], point["id"]);
    assert_eq!(read["replacementPk"], point["replacementPk"]);
    assert_eq!(read["value"], point["value"]);

    let batch_replacement = harness.recreate_with("/batchPk".into(), 700).await;
    let batch_rid = batch_replacement
        .system_properties
        .resource_id
        .as_deref()
        .expect("replacement container has a RID");
    let batch_item = item_for_path("batch-new-definition", "batchPk", 2);
    let batch = TransactionalBatch::new("pk1")
        .create_item(&batch_item)
        .unwrap()
        .read_item("batch-new-definition", None);
    let batch = harness
        .container
        .execute_transactional_batch(batch, None)
        .await
        .unwrap()
        .into_model()
        .unwrap();
    assert_eq!(
        batch
            .results()
            .iter()
            .map(|result| result.status_code())
            .collect::<Vec<_>>(),
        vec![201, 200]
    );
    let batch_attempts = harness.observer.batch_requests();
    assert_eq!(
        intended_rid_transitions(&batch_attempts).last().copied(),
        Some(batch_rid)
    );
    assert!(
        intended_rid_transitions(&batch_attempts).len() <= 2,
        "batch recovery must cross at most one RID boundary"
    );

    let query_replacement = harness.recreate_with("/queryPk".into(), 800).await;
    let query_rid = query_replacement
        .system_properties
        .resource_id
        .as_deref()
        .expect("replacement container has a RID");
    harness
        .seed_raw(
            "pk1",
            &item_for_path("query-new-definition-1", "queryPk", 3),
        )
        .await;
    harness
        .seed_raw(
            "pk1",
            &item_for_path("query-new-definition-2", "queryPk", 4),
        )
        .await;
    let queried: Vec<serde_json::Value> = Box::pin(harness.container.query_items(
        Query::from("SELECT * FROM c"),
        FeedScope::partition("pk1"),
        None,
    ))
    .await
    .unwrap()
    .try_collect()
    .await
    .unwrap();
    let mut queried_ids = queried
        .iter()
        .filter_map(|item| item["id"].as_str())
        .collect::<Vec<_>>();
    queried_ids.sort_unstable();
    assert_eq!(
        queried_ids,
        vec!["query-new-definition-1", "query-new-definition-2"]
    );
    let query_attempts = harness.observer.query_requests();
    let query_transitions = intended_rid_transitions(&query_attempts);
    assert_eq!(query_transitions.last().copied(), Some(query_rid));
    assert!(
        query_transitions.len() <= 2,
        "query recovery must cross at most one RID boundary"
    );

    let database = harness.client.database_client(DATABASE_NAME);
    let throughput_client = database
        .container_client(CONTAINER_NAME, None)
        .await
        .unwrap();
    harness.recreate_with("/throughputPk".into(), 900).await;
    let throughput = throughput_client
        .read_throughput(None)
        .await
        .unwrap()
        .expect("replacement container has dedicated throughput");
    assert_eq!(throughput.throughput(), Some(900));
    assert_eq!(
        harness
            .diagnostics_handler
            .last_request_count
            .load(Ordering::SeqCst),
        2,
        "throughput recovery must retain the stale and replacement offer queries"
    );

    let replace_client = database
        .container_client(CONTAINER_NAME, None)
        .await
        .unwrap();
    harness
        .recreate_with("/replaceThroughputPk".into(), 1000)
        .await;
    let replaced = replace_client
        .begin_replace_throughput(ThroughputProperties::manual(1100), None)
        .await
        .unwrap()
        .await
        .unwrap()
        .into_model()
        .unwrap();
    assert_eq!(replaced.throughput(), Some(1100));
}

#[tokio::test]
async fn incompatible_partition_key_shape_does_not_reach_replacement() {
    let harness = Harness::new_with_container(("/tenant", "/user").into(), 400).await;
    harness.recreate_with("/tenant".into(), 500).await;

    let body = serde_json::json!({
        "id": "incompatible-shape",
        "tenant": "tenant-a",
    });
    let error = harness
        .container
        .create_item(
            PartitionKey::from(("tenant-a", "user-a")),
            "incompatible-shape",
            body,
            None,
        )
        .await
        .expect_err("the old two-component key must not reach the replacement");
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);
    assert_eq!(
        error.status().sub_status(),
        Some(SubStatusCode::CLIENT_PARTITION_KEY_TOO_MANY_COMPONENTS)
    );
    let attempts = harness.observer.item_creates();
    assert_eq!(
        attempts.len(),
        1,
        "only the stale-generation request may reach the transport"
    );
}

#[tokio::test]
async fn stale_epk_range_does_not_cross_container_recreation() {
    let harness = Harness::new_with_container("/pk".into(), 11000).await;
    let old_ranges = harness.container.read_feed_ranges(None).await.unwrap();
    assert!(old_ranges.len() >= 2);
    let stale_range: FeedRange = old_ranges[0].clone();

    harness.recreate_with("/replacementPk".into(), 12000).await;
    harness
        .seed_raw(
            "pk1",
            &item_for_path("replacement-range-item", "replacementPk", 1),
        )
        .await;

    let result = harness
        .container
        .query_items::<serde_json::Value>(
            Query::from("SELECT * FROM c"),
            FeedScope::range(stale_range),
            None,
        )
        .await;
    let error = match result {
        Err(error) => error,
        Ok(pager) => Box::pin(pager)
            .try_collect::<Vec<_>>()
            .await
            .expect_err("a stale EPK range must not return replacement data"),
    };
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);
    assert_eq!(
        error.status().sub_status(),
        Some(SubStatusCode::COLLECTION_RID_MISMATCH)
    );
    let query_requests = harness.observer.query_requests();
    let query_transitions = intended_rid_transitions(&query_requests);
    assert_eq!(
        query_transitions.len(),
        1,
        "the stale EPK range must not be retried against a replacement RID"
    );
}

#[cfg(feature = "preview_patch")]
#[tokio::test]
async fn patch_restarts_after_container_recreation() {
    use azure_data_cosmos::models::{PatchInstructions, PatchOperation};

    let harness = Harness::new().await;
    harness.recreate().await;
    harness.seed_raw("pk1", &item("patch", 1)).await;

    let response = harness
        .container
        .patch_item(
            "pk1",
            "patch",
            PatchInstructions::new()
                .with_operation(PatchOperation::replace("/value", serde_json::json!(2))),
            None,
        )
        .await
        .unwrap();
    let patched: TestItem = response.into_model().unwrap();
    assert_eq!(patched, item("patch", 2));
}
