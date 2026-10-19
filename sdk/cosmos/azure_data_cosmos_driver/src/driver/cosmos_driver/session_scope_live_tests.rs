// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Opt-in real-transport regression; requires a disposable, pre-provisioned container named
//! `items`, partitioned on `/pk`, with at least two physical ranges.
//! Set COSMOS_SESSION_LIVE_ENDPOINT, COSMOS_SESSION_LIVE_KEY, and COSMOS_SESSION_LIVE_DATABASE.

use super::CosmosDriver;
use crate::{
    diagnostics::TransportKind,
    driver::{
        transport::{
            cosmos_transport_client::{HttpRequest, HttpResponse, TransportClient, TransportError},
            http_client_factory::{DefaultHttpClientFactory, HttpClientConfig, HttpClientFactory},
        },
        CosmosDriverRuntimeBuilder,
    },
    models::{
        AccountReference, ContainerReference, CosmosOperation, CosmosResponse, ItemReference,
        PartitionKey,
    },
    options::{
        AvailabilityStrategy, ConnectionPoolOptions, ContentResponseOnWrite, DriverOptions,
        OperationOptions, PartitionFailoverOptions, PartitionTopologyCacheMode,
        ReadConsistencyStrategy,
    },
};
use async_trait::async_trait;
use std::{
    collections::BTreeMap,
    fmt,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use url::Url;

#[derive(Debug)]
struct Observation {
    transport: Option<TransportKind>,
    token_present: bool,
    range_ids: Vec<String>,
    explicit_equal: Option<bool>,
    status: Option<u16>,
}

#[derive(Default)]
struct Recorder {
    observations: Mutex<Vec<Observation>>,
    expected_token: Mutex<Option<String>>,
    topology_requests: AtomicUsize,
}

// Never expose the explicit token, even when a test assertion fails.
impl fmt::Debug for Recorder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Recorder").finish_non_exhaustive()
    }
}

#[derive(Debug)]
struct ObservingFactory(Arc<Recorder>);

impl HttpClientFactory for ObservingFactory {
    fn build(
        &self,
        pool: &ConnectionPoolOptions,
        config: HttpClientConfig,
    ) -> crate::error::Result<Arc<dyn TransportClient>> {
        Ok(Arc::new(ObservingClient {
            inner: DefaultHttpClientFactory::new().build(pool, config)?,
            transport: config.transport_kind,
            recorder: self.0.clone(),
        }))
    }
}

#[derive(Debug)]
struct ObservingClient {
    inner: Arc<dyn TransportClient>,
    transport: Option<TransportKind>,
    recorder: Arc<Recorder>,
}

fn range_ids(token: &str) -> Vec<String> {
    token
        .split(',')
        .map(|segment| {
            let (id, _) = segment.split_once(':').expect("range-prefixed token");
            assert!(id.bytes().all(|byte| byte.is_ascii_digit()));
            id.to_owned()
        })
        .collect()
}

#[async_trait]
impl TransportClient for ObservingClient {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        if request.url.path().ends_with("/pkranges") {
            self.recorder
                .topology_requests
                .fetch_add(1, Ordering::SeqCst);
        }
        let observation = if request.url.path().contains("/docs") {
            let token = request
                .headers
                .get_optional_str(&"x-ms-session-token".into());
            Some(Observation {
                transport: self.transport,
                token_present: token.is_some(),
                range_ids: token.map(range_ids).unwrap_or_default(),
                explicit_equal: self
                    .recorder
                    .expected_token
                    .lock()
                    .unwrap()
                    .as_ref()
                    .map(|expected| token == Some(expected.as_str())),
                status: None,
            })
        } else {
            None
        };
        let response = self.inner.send(request).await;
        if let Some(mut observation) = observation {
            observation.status = response.as_ref().ok().map(|response| response.status);
            self.recorder.observations.lock().unwrap().push(observation);
        }
        response
    }
}

async fn client(
    account: AccountReference,
    mode: PartitionTopologyCacheMode,
) -> (Arc<CosmosDriver>, Arc<Recorder>) {
    let recorder = Arc::new(Recorder::default());
    let runtime = CosmosDriverRuntimeBuilder::new()
        .with_mock_http_client_factory(Arc::new(ObservingFactory(recorder.clone())))
        .with_connection_pool(
            ConnectionPoolOptions::builder()
                .with_gateway_v2_disabled(true)
                .build()
                .unwrap(),
        )
        .build()
        .await
        .unwrap_or_else(|_| panic!("live runtime initialization failed"));
    let driver = runtime
        .create_driver(
            DriverOptions::builder(account)
                .with_operation_options(OperationOptions {
                    read_consistency_strategy: Some(ReadConsistencyStrategy::Session),
                    session_capturing_disabled: Some(false),
                    availability_strategy: Some(AvailabilityStrategy::Disabled),
                    ..Default::default()
                })
                .with_partition_failover_options(
                    PartitionFailoverOptions::builder()
                        .with_partition_topology_cache_mode(mode)
                        .with_circuit_breaker_enabled(false)
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .await
        .unwrap_or_else(|_| panic!("live driver initialization failed"));
    (driver, recorder)
}

async fn execute(
    driver: &CosmosDriver,
    operation: CosmosOperation,
    options: OperationOptions,
) -> CosmosResponse {
    driver
        .execute_operation(operation, options)
        .await
        .unwrap_or_else(|error| panic!("live operation failed: status={:?}", error.status()))
        .expect("live operation must return a response")
}

fn item(container: &ContainerReference, key: &str, label: &str) -> ItemReference {
    ItemReference::from_name(
        container,
        PartitionKey::from(key.to_owned()),
        format!("{label}-{key}"),
    )
}

async fn seed(
    driver: &CosmosDriver,
    container: &ContainerReference,
    keys: &BTreeMap<String, String>,
    label: &str,
    disabled: bool,
) {
    for key in keys.values() {
        for version in [1, 2] {
            let response = execute(
                driver,
                CosmosOperation::upsert_item(item(container, key, label)).with_body(
                    serde_json::to_vec(&serde_json::json!({
                        "id": format!("{label}-{key}"), "pk": key, "version": version
                    }))
                    .unwrap(),
                ),
                OperationOptions {
                    session_capturing_disabled: Some(disabled),
                    content_response_on_write: Some(ContentResponseOnWrite::Enabled),
                    ..Default::default()
                },
            )
            .await;
            let body: serde_json::Value = response.into_body().into_single().unwrap();
            assert_eq!(body["version"], version);
        }
    }
}

fn check_observations(
    recorder: &Recorder,
    scenario: &str,
    expected_ids: &[String],
    explicit: bool,
) {
    let observations = std::mem::take(&mut *recorder.observations.lock().unwrap());
    assert!(!observations.is_empty(), "no outgoing data-plane request");
    for observation in observations {
        println!("scenario={scenario} {observation:?}");
        assert_eq!(observation.transport, Some(TransportKind::Gateway));
        assert_eq!(observation.status, Some(200));
        assert_eq!(observation.token_present, !expected_ids.is_empty());
        let mut actual = observation.range_ids;
        actual.sort();
        let mut expected = expected_ids.to_vec();
        expected.sort();
        assert_eq!(actual, expected, "outgoing session range IDs: {scenario}");
        if explicit {
            assert!(observation.explicit_equal == Some(true));
        }
    }
}

async fn read(driver: &CosmosDriver, operation: CosmosOperation, options: OperationOptions) {
    let response = execute(driver, operation, options).await;
    let body: serde_json::Value = response.into_body().into_single().unwrap();
    assert_eq!(body["version"], 2, "read-your-writes version");
}

async fn scenarios(account: AccountReference, database: &str) {
    let (eager, eager_recorder) = client(account.clone(), PartitionTopologyCacheMode::Eager).await;
    let container = eager
        .resolve_container(database, "items", OperationOptions::default())
        .await
        .unwrap_or_else(|_| panic!("live container unavailable"));
    let ranges = eager
        .resolve_all_partition_key_ranges(&container, true)
        .await
        .unwrap()
        .expect("live topology unavailable");
    println!("physical_range_count={}", ranges.len());
    assert!(
        (2..=8).contains(&ranges.len()),
        "requires 2 to 8 real ranges"
    );
    let mut keys = BTreeMap::new();
    for index in 0..256 {
        let key = format!("scope-{index}");
        let resolved = eager
            .resolve_partition_key_ranges_for_key(
                &container,
                &PartitionKey::from(key.clone()),
                false,
            )
            .await
            .unwrap()
            .expect("key routing unavailable");
        assert_eq!(resolved.len(), 1);
        keys.entry(resolved[0].id.clone()).or_insert(key);
        if keys.len() == ranges.len() {
            break;
        }
    }
    assert_eq!(
        keys.len(),
        ranges.len(),
        "bounded key search must cover all ranges"
    );
    let all_ids: Vec<_> = keys.keys().cloned().collect();
    seed(&eager, &container, &keys, "eager", false).await;
    eager_recorder.observations.lock().unwrap().clear();
    for (range, key) in &keys {
        read(
            &eager,
            CosmosOperation::read_item(item(&container, key, "eager")),
            OperationOptions::default(),
        )
        .await;
        check_observations(
            &eager_recorder,
            "eager-scoped",
            std::slice::from_ref(range),
            false,
        );
    }

    let (lazy, lazy_recorder) = client(account.clone(), PartitionTopologyCacheMode::Lazy).await;
    let lazy_container = lazy
        .resolve_container(database, "items", OperationOptions::default())
        .await
        .unwrap_or_else(|_| panic!("live lazy container unavailable"));
    seed(&lazy, &lazy_container, &keys, "lazy", false).await;
    assert_eq!(lazy_recorder.topology_requests.load(Ordering::SeqCst), 0);
    lazy_recorder.observations.lock().unwrap().clear();
    for key in keys.values() {
        read(
            &lazy,
            CosmosOperation::read_item(item(&lazy_container, key, "lazy")),
            OperationOptions::default(),
        )
        .await;
        check_observations(&lazy_recorder, "lazy-cold-composite", &all_ids, false);
    }
    assert_eq!(lazy_recorder.topology_requests.load(Ordering::SeqCst), 0);
    let warm_ranges = lazy
        .resolve_all_partition_key_ranges(&lazy_container, true)
        .await
        .unwrap()
        .expect("explicit topology warm-up failed");
    assert_eq!(warm_ranges.len(), ranges.len());
    assert!(lazy_recorder.topology_requests.load(Ordering::SeqCst) > 0);
    for (range, key) in &keys {
        read(
            &lazy,
            CosmosOperation::read_item(item(&lazy_container, key, "lazy")),
            OperationOptions::default(),
        )
        .await;
        check_observations(
            &lazy_recorder,
            "lazy-warm-scoped",
            std::slice::from_ref(range),
            false,
        );
    }
    let key = keys.values().next().unwrap();
    let operation = CosmosOperation::read_item(item(&lazy_container, key, "lazy"));
    let explicit = lazy
        .session_manager
        .resolve_session_token(&operation, None, None)
        .expect("real response tokens must be cached");
    assert_eq!(range_ids(explicit.as_str()).len(), ranges.len());
    *lazy_recorder.expected_token.lock().unwrap() = Some(explicit.as_str().to_owned());
    for disabled in [false, true] {
        read(
            &lazy,
            operation.clone().with_session_token(explicit.clone()),
            OperationOptions {
                session_capturing_disabled: Some(disabled),
                ..Default::default()
            },
        )
        .await;
        check_observations(
            &lazy_recorder,
            if disabled {
                "explicit-disabled"
            } else {
                "explicit-enabled"
            },
            &all_ids,
            true,
        );
    }
    *lazy_recorder.expected_token.lock().unwrap() = None;
    read(
        &lazy,
        operation.clone(),
        OperationOptions {
            read_consistency_strategy: Some(ReadConsistencyStrategy::Eventual),
            ..Default::default()
        },
    )
    .await;
    check_observations(&lazy_recorder, "eventual-no-automatic-token", &[], false);
    read(
        &lazy,
        operation,
        OperationOptions {
            session_capturing_disabled: Some(true),
            ..Default::default()
        },
    )
    .await;
    check_observations(&lazy_recorder, "disabled-with-cached-state", &[], false);

    let (disabled, recorder) = client(account, PartitionTopologyCacheMode::Eager).await;
    let disabled_container = disabled
        .resolve_container(database, "items", OperationOptions::default())
        .await
        .unwrap_or_else(|_| panic!("live disabled container unavailable"));
    seed(&disabled, &disabled_container, &keys, "disabled", true).await;
    let operation = CosmosOperation::read_item(item(&disabled_container, key, "disabled"));
    assert!(disabled
        .session_manager
        .resolve_session_token(&operation, None, None)
        .is_none());
    recorder.observations.lock().unwrap().clear();
    read(
        &disabled,
        operation.clone(),
        OperationOptions {
            session_capturing_disabled: Some(true),
            ..Default::default()
        },
    )
    .await;
    check_observations(&recorder, "disabled-no-token-no-capture", &[], false);
    assert!(disabled
        .session_manager
        .resolve_session_token(&operation, None, None)
        .is_none());
    read(&disabled, operation.clone(), OperationOptions::default()).await;
    check_observations(&recorder, "reenabled-first-read-no-token", &[], false);
    assert!(disabled
        .session_manager
        .resolve_session_token(&operation, None, None)
        .is_some());
    println!("all_classic_gateway_session_scenarios_passed=true");
}

#[tokio::test]
#[ignore = "requires explicitly provisioned live Cosmos DB resources and credentials"]
async fn classic_gateway_session_scope_live() {
    let endpoint = std::env::var("COSMOS_SESSION_LIVE_ENDPOINT").expect("live endpoint required");
    let key = std::env::var("COSMOS_SESSION_LIVE_KEY").expect("live key required");
    let database = std::env::var("COSMOS_SESSION_LIVE_DATABASE").expect("live database required");
    assert!(database.starts_with("test-db-session-scope-"));
    let endpoint = Url::parse(&endpoint).expect("valid endpoint required");
    let account = AccountReference::with_account_key(endpoint, key);
    tokio::time::timeout(Duration::from_secs(180), scenarios(account, &database))
        .await
        .expect("bounded live regression timed out");
}
