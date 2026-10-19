// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::CosmosDriver;
use crate::{
    driver::{
        cache::AccountProperties,
        transport::{
            cosmos_transport_client::{HttpRequest, HttpResponse, TransportClient, TransportError},
            http_client_factory::{HttpClientConfig, HttpClientFactory},
            rntbd::{
                tokens::{RntbdRequestToken, TokenValue},
                RntbdRequestFrame, RntbdResponse,
            },
        },
        CosmosDriverRuntimeBuilder,
    },
    models::{
        AccountEndpoint, AccountReference, ContainerProperties, ContainerReference,
        CosmosOperation, CosmosResponseHeaders, CosmosStatus, ItemReference, PartitionKey,
        SessionToken,
    },
    options::{
        AvailabilityStrategy, ConnectionPoolOptions, DriverOptions, HedgeThreshold,
        HedgingStrategy, OperationOptions, PartitionFailoverOptions, PartitionTopologyCacheMode,
        PlanOptions, Region,
    },
};
use async_trait::async_trait;
use azure_core::http::{headers::Headers, StatusCode};
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex, Weak,
    },
    time::Duration,
};
use url::Url;
use uuid::Uuid;

const COMPOSITE_TOKEN: &str = "0:1#100#1=10,1:1#200#1=20";
const SCOPED_TOKEN: &str = "0:1#100#1=10";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scenario {
    Success,
    ActivateBeforeRetry,
    ActivateBeforeHedge,
    SplitOnWrite,
    SplitOnRead,
    SplitOnConflict,
    SplitTopologyUnavailable,
}

#[derive(Debug)]
struct Observation {
    host: String,
    thin: bool,
    session: Option<String>,
}

#[derive(Debug)]
struct ScenarioClient {
    driver: Mutex<Weak<CosmosDriver>>,
    properties: Mutex<serde_json::Value>,
    scenario: Scenario,
    requests: Mutex<Vec<Observation>>,
    range_requests: AtomicUsize,
    item_requests: AtomicUsize,
    response_gate: Mutex<Option<futures::channel::oneshot::Receiver<()>>>,
}

fn account_properties(regions: usize, thin_east: bool, thin_west: bool) -> serde_json::Value {
    let locations = serde_json::json!([
        {"name":"East US","databaseAccountEndpoint":"https://test-east.documents.azure.com/"},
        {"name":"West US","databaseAccountEndpoint":"https://test-west.documents.azure.com/"}
    ]);
    let reads = locations.as_array().unwrap()[..regions].to_vec();
    let thin: Vec<_> = [(thin_east, "East US", "east"), (thin_west, "West US", "west")]
        .into_iter()
        .filter(|(enabled, _, _)| *enabled)
        .map(|(_, name, suffix)| serde_json::json!({
            "name": name,
            "databaseAccountEndpoint": format!("https://test-{suffix}-thin.documents.azure.com:444/")
        }))
        .collect();
    serde_json::json!({
        "_self": "", "id": "test", "_rid": "test", "media": "//media/",
        "addresses": "//addresses/", "_dbs": "//dbs/",
        "readableLocations": reads,
        "writableLocations": [locations[0].clone()],
        "thinClientReadableLocations": thin,
        "thinClientWritableLocations": [],
        "enableMultipleWriteLocations": false,
        "userConsistencyPolicy": {"defaultConsistencyLevel":"Session"},
        "queryEngineConfiguration":"{}"
    })
}

impl ScenarioClient {
    async fn publish(&self, properties: serde_json::Value) {
        *self.properties.lock().unwrap() = properties.clone();
        let properties: AccountProperties = serde_json::from_value(properties).unwrap();
        let driver = self.driver.lock().unwrap().upgrade().unwrap();
        let properties = driver
            .runtime
            .account_metadata_cache()
            .get_or_refresh_with(
                AccountEndpoint::from(driver.account()),
                |_| true,
                || async { properties },
            )
            .await
            .unwrap();
        driver
            .location_state_store
            .sync_account_properties(properties, driver.location_state_store.default_endpoint());
    }
}

#[async_trait]
impl TransportClient for ScenarioClient {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        let mut headers = Headers::new();
        if request.url.path() == "/" {
            return Ok(HttpResponse {
                status: 200,
                headers,
                body: serde_json::to_vec(&*self.properties.lock().unwrap()).unwrap(),
            });
        }
        if request.url.path().ends_with("/pkranges") {
            self.range_requests.fetch_add(1, Ordering::SeqCst);
            let split = self.item_requests.load(Ordering::SeqCst) > 0
                && matches!(
                    self.scenario,
                    Scenario::SplitOnWrite
                        | Scenario::SplitOnRead
                        | Scenario::SplitOnConflict
                        | Scenario::SplitTopologyUnavailable
                );
            if split && self.scenario == Scenario::SplitTopologyUnavailable {
                return Ok(HttpResponse {
                    status: 400,
                    headers,
                    body: br#"{"code":"BadRequest","message":"topology unavailable"}"#.to_vec(),
                });
            }
            let etag = if split { "split-ranges" } else { "ranges" };
            headers.insert("etag", etag);
            return Ok(
                if request
                    .headers
                    .get_optional_str(&"if-none-match".into())
                    .is_some_and(|value| value == etag)
                {
                    HttpResponse {
                        status: 304,
                        headers,
                        body: Vec::new(),
                    }
                } else {
                    HttpResponse {
                        status: 200,
                        headers,
                        body: if split {
                            br#"{"PartitionKeyRanges":[{"id":"1","minInclusive":"","maxExclusive":"80","parents":["0"]},{"id":"2","minInclusive":"80","maxExclusive":"FF","parents":["0"]}],"_count":2}"#.to_vec()
                        } else {
                            br#"{"PartitionKeyRanges":[{"id":"0","minInclusive":"","maxExclusive":"FF","parents":[]}],"_count":1}"#.to_vec()
                        },
                    }
                },
            );
        }
        if request.url.path() == "/probe" {
            return Ok(HttpResponse {
                status: 200,
                headers,
                body: Vec::new(),
            });
        }

        let host = request.url.host_str().unwrap().to_owned();
        let thin = host.contains("-thin");
        let frame = thin.then(|| RntbdRequestFrame::read(request.body.as_ref().unwrap()).unwrap());
        let session = match &frame {
            Some(frame) => frame.metadata.iter().find_map(|token| {
                if RntbdRequestToken::try_from(token.id.0) == Ok(RntbdRequestToken::SessionToken) {
                    match &token.value {
                        TokenValue::String(value) => Some(value.clone()),
                        other => panic!("unexpected session token encoding: {other:?}"),
                    }
                } else {
                    None
                }
            }),
            None => request
                .headers
                .get_optional_str(&"x-ms-session-token".into())
                .map(str::to_owned),
        };
        self.requests.lock().unwrap().push(Observation {
            host,
            thin,
            session,
        });
        let attempt = self.item_requests.fetch_add(1, Ordering::SeqCst);
        let activates = matches!(
            self.scenario,
            Scenario::ActivateBeforeRetry | Scenario::ActivateBeforeHedge
        );
        if attempt == 0 && activates {
            self.publish(account_properties(2, true, true)).await;
        }
        let split = matches!(
            self.scenario,
            Scenario::SplitOnWrite
                | Scenario::SplitOnRead
                | Scenario::SplitOnConflict
                | Scenario::SplitTopologyUnavailable
        );
        let status = if split {
            headers.insert("x-ms-documentdb-partitionkeyrangeid", "1");
            headers.insert("x-ms-session-token", "1:1#300#1=30");
            if attempt == 0 {
                if self.scenario == Scenario::SplitOnConflict {
                    409
                } else if self.scenario == Scenario::SplitOnRead {
                    200
                } else {
                    201
                }
            } else {
                200
            }
        } else if (attempt == 0 && activates)
            || (attempt == 1 && self.scenario == Scenario::ActivateBeforeHedge)
        {
            503
        } else {
            if attempt == 2 && self.scenario == Scenario::ActivateBeforeHedge {
                // Keep the primary pending until the hedge wins and cancels it.
                return futures::future::pending().await;
            }
            200
        };
        let body = if status == 200 {
            br#"{"id":"one","pk":"key"}"#.to_vec()
        } else {
            b"{}".to_vec()
        };
        let response_gate = self.response_gate.lock().unwrap().take();
        if let Some(gate) = response_gate {
            gate.await.expect("test releases the response");
        }
        if let Some(frame) = frame {
            Ok(thin_response(status, frame.activity_id, body))
        } else {
            Ok(HttpResponse {
                status,
                headers,
                body,
            })
        }
    }
}

fn thin_response(status: u16, activity_id: Uuid, body: Vec<u8>) -> HttpResponse {
    let response = RntbdResponse {
        status: CosmosStatus::new(StatusCode::from(status)),
        activity_id,
        body,
        continuation_token: None,
        etag: None,
        retry_after_ms: None,
        last_state_change_date_time: None,
        storage_max_resource_quota: None,
        storage_resource_quota_usage: None,
        schema_version: None,
        lsn: None,
        item_count: None,
        request_charge: None,
        backend_request_duration_ms: None,
        owner_full_name: None,
        owner_id: None,
        quorum_acked_lsn: None,
        current_write_quorum: None,
        current_replica_set_size: None,
        partition_key_range_id: None,
        xp_role: None,
        number_of_read_regions: None,
        item_lsn: None,
        global_committed_lsn: None,
        local_lsn: None,
        quorum_acked_local_lsn: None,
        item_local_lsn: None,
        transport_request_id: None,
        session_token: None,
        query_metrics: None,
        index_utilization: None,
        query_execution_info: None,
        pending_pk_delete: None,
        physical_partition_id: None,
        conflict_resolved_timestamp: None,
    };
    let mut body = Vec::new();
    response.write(&mut body).unwrap();
    HttpResponse {
        status: 200,
        headers: Headers::new(),
        body,
    }
}

#[derive(Debug)]
struct ScenarioFactory(Arc<ScenarioClient>);

impl HttpClientFactory for ScenarioFactory {
    fn build(
        &self,
        _: &ConnectionPoolOptions,
        _: HttpClientConfig,
    ) -> crate::error::Result<Arc<dyn TransportClient>> {
        Ok(self.0.clone())
    }
}

async fn setup(
    properties: serde_json::Value,
    scenario: Scenario,
) -> (Arc<CosmosDriver>, Arc<ScenarioClient>, CosmosOperation) {
    setup_with_topology_mode(properties, scenario, PartitionTopologyCacheMode::Lazy).await
}

async fn setup_with_topology_mode(
    properties: serde_json::Value,
    scenario: Scenario,
    topology_mode: PartitionTopologyCacheMode,
) -> (Arc<CosmosDriver>, Arc<ScenarioClient>, CosmosOperation) {
    let client = Arc::new(ScenarioClient {
        driver: Mutex::new(Weak::new()),
        properties: Mutex::new(properties.clone()),
        scenario,
        requests: Mutex::new(Vec::new()),
        range_requests: AtomicUsize::new(0),
        item_requests: AtomicUsize::new(0),
        response_gate: Mutex::new(None),
    });
    let runtime = CosmosDriverRuntimeBuilder::new()
        .with_http_client_factory(Arc::new(ScenarioFactory(client.clone())))
        .with_connection_pool(
            ConnectionPoolOptions::builder()
                .with_is_http2_allowed(true)
                .with_gateway_v2_disabled(false)
                .build()
                .unwrap(),
        )
        .build()
        .await
        .unwrap();
    let account = AccountReference::with_account_key(
        Url::parse("https://test.documents.azure.com/").unwrap(),
        "dGVzdA==",
    );
    let driver = Arc::new(
        CosmosDriver::new(
            runtime,
            DriverOptions::builder(account.clone())
                .with_preferred_regions(vec![Region::new("East US"), Region::new("West US")])
                .with_partition_failover_options(
                    PartitionFailoverOptions::builder()
                        .with_partition_topology_cache_mode(topology_mode)
                        .with_circuit_breaker_enabled(false)
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .unwrap(),
    );
    *client.driver.lock().unwrap() = Arc::downgrade(&driver);
    client.publish(properties).await;
    driver.initialized.store(true, Ordering::Release);
    let properties = ContainerProperties {
        id: "items".into(),
        partition_key: serde_json::from_str(r#"{"paths":["/pk"],"version":2}"#).unwrap(),
        system_properties: Default::default(),
    };
    let container = ContainerReference::new(
        account,
        "db",
        "-NY+AA==",
        "items",
        "-NY+AJoR+cc=",
        &properties,
    );
    driver
        .runtime
        .container_cache()
        .put(container.clone())
        .await;
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &container,
        PartitionKey::from("key"),
        "one".to_owned(),
    ));
    driver.session_manager.capture_session_token(
        &operation,
        &CosmosResponseHeaders {
            session_token: Some(SessionToken::new(
                if matches!(
                    scenario,
                    Scenario::SplitOnWrite
                        | Scenario::SplitOnRead
                        | Scenario::SplitOnConflict
                        | Scenario::SplitTopologyUnavailable
                ) {
                    "0:1#100#1=10"
                } else {
                    COMPOSITE_TOKEN
                },
            )),
            ..Default::default()
        },
    );
    (driver, client, operation)
}

fn options(hedging: bool) -> OperationOptions {
    OperationOptions {
        hedging_enabled: Some(hedging),
        max_failover_retry_count: Some(6),
        availability_strategy: Some(AvailabilityStrategy::Hedging(HedgingStrategy::new(
            HedgeThreshold::new(Duration::from_millis(1)).unwrap(),
        ))),
        ..Default::default()
    }
}

#[tokio::test]
async fn classic_session_progress_survives_an_overlapping_failed_topology_refresh() {
    let (driver, client, read) = setup_with_topology_mode(
        account_properties(1, false, false),
        Scenario::SplitOnWrite,
        PartitionTopologyCacheMode::Eager,
    )
    .await;
    let container = read.container().unwrap().clone();
    driver
        .resolve_all_partition_key_ranges(&container, false)
        .await
        .unwrap();
    let write = CosmosOperation::create_item(ItemReference::from_name(
        &container,
        PartitionKey::from("key"),
        "one",
    ))
    .with_body(br#"{"id":"one","pk":"key"}"#.to_vec());
    let (write_release, write_gate) = futures::channel::oneshot::channel();
    *client.response_gate.lock().unwrap() = Some(write_gate);
    let mut write = Box::pin(driver.execute_singleton_operation(write, options(false)));
    assert!(futures::poll!(write.as_mut()).is_pending());
    assert_eq!(client.item_requests.load(Ordering::SeqCst), 1);

    let (refresh_release, refresh_gate) = futures::channel::oneshot::channel();
    let refresh_gate = Mutex::new(Some(refresh_gate));
    let mut refresh = Box::pin(driver.pk_range_cache.try_lookup_result(
        &container,
        true,
        |_, _| {
            let gate = refresh_gate.lock().unwrap().take();
            async move {
                if let Some(gate) = gate {
                    gate.await.expect("test releases the refresh");
                }
                None::<crate::driver::cache::PkRangeFetchResult>
            }
        },
    ));
    assert!(futures::poll!(refresh.as_mut()).is_pending());
    assert!(refresh_gate.lock().unwrap().is_none());
    write_release.send(()).unwrap();
    assert_eq!(write.await.unwrap().status(), StatusCode::Created);
    refresh_release.send(()).unwrap();
    let old_snapshot = refresh.await.unwrap().unwrap();
    assert_eq!(old_snapshot.ranges()[0].id, "0");

    driver
        .execute_singleton_operation(read, options(false))
        .await
        .unwrap();
    let requests = client.requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    assert_eq!(
        requests[1].session.as_deref(),
        Some("0:1#100#1=10,1:1#300#1=30")
    );
}

#[tokio::test]
async fn classic_logical_reads_preserve_child_progress_after_stale_parent_routing() {
    for mode in [
        PartitionTopologyCacheMode::Eager,
        PartitionTopologyCacheMode::Lazy,
    ] {
        for hedging in [false, true] {
            for scenario in [
                Scenario::SplitOnWrite,
                Scenario::SplitOnRead,
                Scenario::SplitOnConflict,
                Scenario::SplitTopologyUnavailable,
            ] {
                let (driver, client, read) =
                    setup_with_topology_mode(account_properties(2, false, false), scenario, mode)
                        .await;
                let container = read.container().unwrap().clone();
                driver
                    .resolve_all_partition_key_ranges(&container, false)
                    .await
                    .unwrap();
                let options = options(hedging);
                let mut retained_plan = driver
                    .plan_operation(read.clone(), &options, None, &PlanOptions::default())
                    .await
                    .unwrap();
                let first = if scenario == Scenario::SplitOnRead {
                    read.clone()
                } else {
                    CosmosOperation::create_item(ItemReference::from_name(
                        &container,
                        PartitionKey::from("key"),
                        "one",
                    ))
                    .with_body(br#"{"id":"one","pk":"key"}"#.to_vec())
                };
                let result = driver
                    .execute_singleton_operation(first, options.clone())
                    .await;
                if scenario == Scenario::SplitOnConflict {
                    assert_eq!(
                        result.unwrap_err().status().status_code(),
                        StatusCode::Conflict
                    );
                } else {
                    assert_eq!(
                        result.unwrap().status(),
                        if scenario == Scenario::SplitOnRead {
                            StatusCode::Ok
                        } else {
                            StatusCode::Created
                        }
                    );
                }
                driver
                    .execute_plan(&mut retained_plan, Some(container.clone()), options.clone())
                    .await
                    .unwrap()
                    .unwrap();
                driver
                    .resolve_all_partition_key_ranges(&container, true)
                    .await
                    .unwrap();
                let range_requests = client.range_requests.load(Ordering::SeqCst);
                let response = driver
                    .execute_singleton_operation(read, options)
                    .await
                    .unwrap();
                assert_eq!(
                    response.diagnostics_ref().hedge_diagnostics().is_some(),
                    hedging
                );
                if scenario != Scenario::SplitTopologyUnavailable {
                    assert_eq!(client.range_requests.load(Ordering::SeqCst), range_requests);
                }
                let requests = client.requests.lock().unwrap();
                assert_eq!(
                    requests.len(),
                    3,
                    "{mode:?}, {scenario:?}, hedging={hedging}"
                );
                assert_eq!(
                    requests[0].session.as_deref(),
                    if scenario == Scenario::SplitOnRead {
                        Some("0:1#100#1=10")
                    } else {
                        None
                    }
                );
                let expected = if scenario != Scenario::SplitTopologyUnavailable {
                    "1:1#300#1=30"
                } else {
                    "0:1#100#1=10,1:1#300#1=30"
                };
                assert_eq!(
                    requests[1].session.as_deref(),
                    Some("0:1#100#1=10,1:1#300#1=30")
                );
                assert_eq!(requests[2].session.as_deref(), Some(expected));
                assert!(requests.iter().all(|request| !request.thin));
            }
        }
    }
}

#[tokio::test]
async fn activation_after_planning_resolves_topology_and_sends_scoped_token() {
    let (driver, client, operation) =
        setup(account_properties(1, false, false), Scenario::Success).await;
    let container = operation.container().cloned();
    let options = options(false);
    let mut plan = driver
        .plan_operation(operation, &options, None, &PlanOptions::default())
        .await
        .unwrap();
    assert_eq!(client.range_requests.load(Ordering::SeqCst), 0);
    client.publish(account_properties(2, true, true)).await;
    let response = driver
        .execute_plan(&mut plan, container, options)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(response.status(), StatusCode::Ok);
    assert_eq!(client.range_requests.load(Ordering::SeqCst), 2);
    let requests = client.requests.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert!(requests[0].thin);
    assert_eq!(requests[0].session.as_deref(), Some(SCOPED_TOKEN));
}

#[tokio::test]
async fn activation_between_retries_preserves_established_session_on_wire() {
    let (driver, client, operation) = setup(
        account_properties(1, false, false),
        Scenario::ActivateBeforeRetry,
    )
    .await;
    let response = driver
        .execute_singleton_operation(operation, options(false))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::Ok);
    let requests = client.requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    for request in requests.iter() {
        assert!(
            !request.thin,
            "unresolved logical retry must use classic gateway: {request:?}"
        );
        assert_eq!(request.session.as_deref(), Some(COMPOSITE_TOKEN));
    }
    assert_eq!(client.range_requests.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn activation_before_retry_hedge_preserves_session_on_both_legs() {
    let (driver, client, operation) = setup(
        account_properties(1, false, false),
        Scenario::ActivateBeforeHedge,
    )
    .await;
    let response = tokio::time::timeout(
        Duration::from_secs(5),
        driver.execute_singleton_operation(operation, options(true)),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(response.status(), StatusCode::Ok);
    let requests = client.requests.lock().unwrap();
    assert_eq!(requests.len(), 4);
    assert_ne!(
        requests[2].host, requests[3].host,
        "both hedge regions must dispatch"
    );
    for request in requests.iter() {
        assert!(
            !request.thin,
            "unresolved hedge must use classic gateway: {request:?}"
        );
        assert_eq!(request.session.as_deref(), Some(COMPOSITE_TOKEN));
    }
    assert_eq!(client.range_requests.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn excluded_thin_region_does_not_load_topology_or_drop_session() {
    let (driver, client, operation) =
        setup(account_properties(2, false, true), Scenario::Success).await;
    let mut options = options(false);
    options.excluded_regions = Some([Region::new("West US")].into_iter().collect());
    let response = driver
        .execute_singleton_operation(operation, options)
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::Ok);
    assert_eq!(client.range_requests.load(Ordering::SeqCst), 0);
    let requests = client.requests.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].host, "test-east.documents.azure.com");
    assert!(!requests[0].thin);
    assert_eq!(requests[0].session.as_deref(), Some(COMPOSITE_TOKEN));
}
