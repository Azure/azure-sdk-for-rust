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
use azure_data_cosmos::{
    models::{ContainerProperties, ThroughputProperties},
    options::{CreateContainerOptions, ItemReadOptions, MaxItemCountHint, QueryOptions, Region},
    AccountEndpoint, AccountReference, ContainerClient, CosmosClient, CosmosClientBuilder,
    CosmosRuntimeBuilder, FeedScope, Query, RoutingStrategy, TransactionalBatch,
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
            })
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
                container_properties(),
                Some(
                    CreateContainerOptions::default()
                        .with_throughput(ThroughputProperties::manual(400)),
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
        self.container.delete(None).await.unwrap();
        self.client
            .database_client(DATABASE_NAME)
            .create_container(
                container_properties(),
                Some(
                    CreateContainerOptions::default()
                        .with_throughput(ThroughputProperties::manual(400)),
                ),
            )
            .await
            .unwrap();
        self.observer.clear();
    }

    async fn seed_raw(&self, item: &TestItem) {
        let mut request = Request::new(
            Url::parse(&format!(
                "{GATEWAY_URL}/dbs/{DATABASE_NAME}/colls/{CONTAINER_NAME}/docs"
            ))
            .unwrap(),
            Method::Post,
        );
        request.headers_mut().insert(
            PARTITION_KEY.clone(),
            serde_json::to_string(&[&item.pk]).unwrap(),
        );
        request.set_body(serde_json::to_vec(item).unwrap());
        let response = self.emulator.execute_request(&request).await.unwrap();
        assert_eq!(response.status(), StatusCode::Created);
        self.observer.clear();
    }
}

fn container_properties() -> ContainerProperties {
    ContainerProperties::new(CONTAINER_NAME.to_owned(), "/pk".into())
}

fn item(id: &str, value: i64) -> TestItem {
    TestItem {
        id: id.to_owned(),
        pk: "pk1".to_owned(),
        value,
    }
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
    harness.seed_raw(&item("query", 3)).await;
    harness.seed_raw(&item("query-2", 4)).await;
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
    harness.seed_raw(&item("replacement-query", 4)).await;
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

#[cfg(feature = "preview_patch")]
#[tokio::test]
async fn patch_restarts_after_container_recreation() {
    use azure_data_cosmos::models::{PatchInstructions, PatchOperation};

    let harness = Harness::new().await;
    harness.recreate().await;
    harness.seed_raw(&item("patch", 1)).await;

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
