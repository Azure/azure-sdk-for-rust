// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{pk_range_retry_delay, CosmosDriver};
use crate::{
    driver::{
        pipeline::operation_pipeline::{OperationOverrides, RegionPin},
        routing::CosmosEndpoint,
        CosmosDriverRuntime,
    },
    models::{
        AccountEndpoint, AccountReference, ContainerProperties, ContainerReference, CosmosOperation,
    },
    options::{
        AvailabilityStrategy, ConnectionPoolOptions, DriverOptions, HedgeThreshold,
        HedgingStrategy, OperationOptions, PartitionFailoverOptions, PartitionTopologyCacheMode,
        Region, ThrottlingRetryOptions,
    },
    pipeline::transport::{
        cosmos_transport_client::{HttpRequest, HttpResponse, TransportClient, TransportError},
        http_client_factory::{HttpClientConfig, HttpClientFactory},
    },
};
use async_trait::async_trait;
use azure_core::http::headers::Headers;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};
use url::Url;

#[derive(Clone, Debug)]
struct PageAttempt {
    path: String,
    token: Option<String>,
    host: String,
    activity_id: String,
    started: Instant,
}

#[derive(Clone, Copy, Debug)]
struct ColdReply {
    status: u16,
    substatus: Option<u16>,
    delay: Duration,
    pending: bool,
}

#[derive(Debug)]
struct PageClient {
    attempts: Mutex<Vec<PageAttempt>>,
    remaining_throttles: Mutex<u32>,
    throttle_token: Mutex<Option<&'static str>>,
    fail_pinned: AtomicBool,
    hedge: AtomicBool,
    attempted: tokio::sync::Notify,
    hold_east_cold: AtomicBool,
    release_east_cold: tokio::sync::Notify,
    cold_replies: Mutex<Option<Vec<ColdReply>>>,
    capacity_throttle: AtomicBool,
    third_region: bool,
    hold_primary_until_hedge: AtomicBool,
    hedge_started: tokio::sync::Notify,
    page_throttles: Mutex<HashMap<String, u32>>,
    retry_after_ms: AtomicU64,
    active_requests: AtomicU64,
}

struct ActivePageRequest<'a>(&'a AtomicU64);

impl Drop for ActivePageRequest<'_> {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::SeqCst);
    }
}

#[async_trait]
impl TransportClient for PageClient {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        let mut headers = Headers::new();
        if request.url.path() == "/" {
            let mut locations = vec![
                serde_json::json!({"name":"East US","databaseAccountEndpoint":"https://test-east.documents.azure.com/"}),
                serde_json::json!({"name":"West US","databaseAccountEndpoint":"https://test-west.documents.azure.com/"}),
            ];
            if self.third_region {
                locations.push(serde_json::json!({"name":"Central US","databaseAccountEndpoint":"https://test-central.documents.azure.com/"}));
            }
            return Ok(HttpResponse {
                status: 200,
                headers,
                body: serde_json::to_vec(&serde_json::json!({
                    "_self": "", "id": "test", "_rid": "test", "media": "//media/",
                    "addresses": "//addresses/", "_dbs": "//dbs/",
                    "readableLocations": locations,
                    "writableLocations": [
                        {"name":"East US","databaseAccountEndpoint":"https://test-east.documents.azure.com/"}
                    ],
                    "enableMultipleWriteLocations": false,
                    "userConsistencyPolicy": {"defaultConsistencyLevel":"Session"},
                    "queryEngineConfiguration":"{}"
                })).unwrap(),
            });
        }
        if !request.url.path().ends_with("/pkranges") {
            headers.insert("x-ms-retry-after-ms", "0");
            return Ok(HttpResponse {
                status: 429,
                headers,
                body: b"{}".to_vec(),
            });
        }
        let token = request
            .headers
            .get_optional_str(&"if-none-match".into())
            .map(str::to_owned);
        let host = request.url.host_str().unwrap().to_owned();
        self.active_requests.fetch_add(1, Ordering::SeqCst);
        let _active = ActivePageRequest(&self.active_requests);
        self.attempts.lock().unwrap().push(PageAttempt {
            path: request.url.path().to_owned(),
            token: token.clone(),
            host: host.clone(),
            activity_id: request
                .headers
                .get_str(&"x-ms-activity-id".into())
                .unwrap()
                .to_owned(),
            started: Instant::now(),
        });
        self.attempted.notify_one();
        if token.is_none() && self.hold_primary_until_hedge.load(Ordering::SeqCst) {
            if host.contains("east") {
                self.hedge_started.notified().await;
            } else {
                self.hedge_started.notify_one();
            }
        }
        let cold_reply = token
            .is_none()
            .then(|| {
                self.cold_replies.lock().unwrap().as_ref().map(|replies| {
                    replies[if host.contains("central") {
                        2
                    } else {
                        usize::from(host.contains("west"))
                    }]
                })
            })
            .flatten();
        if let Some(reply) = cold_reply {
            if reply.pending {
                return futures::future::pending().await;
            }
            tokio::time::sleep(reply.delay).await;
            if reply.status != 200 {
                headers.insert(
                    "x-ms-retry-after-ms",
                    self.retry_after_ms.load(Ordering::SeqCst).to_string(),
                );
                if let Some(substatus) = reply.substatus {
                    headers.insert("x-ms-substatus", substatus.to_string());
                }
                return Ok(HttpResponse {
                    status: reply.status,
                    headers,
                    body: b"{}".to_vec(),
                });
            }
        }
        if self.hedge.load(Ordering::SeqCst) && token.is_none() && host.contains("east") {
            return futures::future::pending().await;
        }
        if token.is_none() && host.contains("east") && self.hold_east_cold.load(Ordering::SeqCst) {
            self.release_east_cold.notified().await;
        }
        let page_throttle = {
            let mut pages = self.page_throttles.lock().unwrap();
            pages
                .get_mut(token.as_deref().unwrap_or(""))
                .is_some_and(|remaining| {
                    if *remaining == 0 {
                        return false;
                    }
                    *remaining -= 1;
                    true
                })
        };
        let throttle = page_throttle || {
            let mut remaining = self.remaining_throttles.lock().unwrap();
            if token.as_deref() == *self.throttle_token.lock().unwrap() && *remaining > 0 {
                *remaining -= 1;
                true
            } else {
                false
            }
        };
        if throttle {
            headers.insert(
                "x-ms-retry-after-ms",
                self.retry_after_ms.load(Ordering::SeqCst).to_string(),
            );
            if self.capacity_throttle.load(Ordering::SeqCst) {
                headers.insert("x-ms-substatus", "3092");
            }
            return Ok(HttpResponse {
                status: 429,
                headers,
                body: b"{}".to_vec(),
            });
        }
        if token.is_some() && self.fail_pinned.swap(false, Ordering::SeqCst) {
            return Ok(HttpResponse {
                status: 400,
                headers,
                body: b"{}".to_vec(),
            });
        }
        let (status, etag, body) = match token.as_deref() {
            None => (
                200,
                "page-1",
                br#"{"PartitionKeyRanges":[{"id":"0","minInclusive":"","maxExclusive":"80"}]}"#
                    .to_vec(),
            ),
            Some("page-1") => (
                200,
                "page-2",
                br#"{"PartitionKeyRanges":[{"id":"1","minInclusive":"80","maxExclusive":"FF"}]}"#
                    .to_vec(),
            ),
            Some("page-2") => (304, "page-2", Vec::new()),
            other => panic!("unexpected continuation {other:?}"),
        };
        headers.insert("etag", etag);
        Ok(HttpResponse {
            status,
            headers,
            body,
        })
    }
}

#[derive(Debug)]
struct PageFactory(Arc<PageClient>);

impl HttpClientFactory for PageFactory {
    fn build(
        &self,
        _: &ConnectionPoolOptions,
        _: HttpClientConfig,
    ) -> crate::error::Result<Arc<dyn TransportClient>> {
        Ok(self.0.clone())
    }
}

async fn setup(
    hedge: bool,
    fail_pinned: bool,
) -> (Arc<CosmosDriver>, Arc<PageClient>, ContainerReference) {
    setup_with_metadata_timeout(hedge, fail_pinned, Duration::from_millis(100), false).await
}

async fn setup_with_metadata_timeout(
    hedge: bool,
    fail_pinned: bool,
    timeout: Duration,
    third_region: bool,
) -> (Arc<CosmosDriver>, Arc<PageClient>, ContainerReference) {
    setup_with_topology_mode(
        hedge,
        fail_pinned,
        timeout,
        third_region,
        PartitionTopologyCacheMode::Lazy,
    )
    .await
}

async fn setup_with_topology_mode(
    hedge: bool,
    fail_pinned: bool,
    timeout: Duration,
    third_region: bool,
    topology_mode: PartitionTopologyCacheMode,
) -> (Arc<CosmosDriver>, Arc<PageClient>, ContainerReference) {
    let client = Arc::new(PageClient {
        attempts: Mutex::new(Vec::new()),
        remaining_throttles: Mutex::new(0),
        throttle_token: Mutex::new(Some("page-1")),
        fail_pinned: AtomicBool::new(fail_pinned),
        hedge: AtomicBool::new(hedge),
        attempted: tokio::sync::Notify::new(),
        hold_east_cold: AtomicBool::new(false),
        release_east_cold: tokio::sync::Notify::new(),
        cold_replies: Mutex::new(None),
        capacity_throttle: AtomicBool::new(false),
        third_region,
        hold_primary_until_hedge: AtomicBool::new(false),
        hedge_started: tokio::sync::Notify::new(),
        page_throttles: Mutex::new(HashMap::new()),
        retry_after_ms: AtomicU64::new(0),
        active_requests: AtomicU64::new(0),
    });
    let runtime = CosmosDriverRuntime::builder()
        .with_connection_pool(
            ConnectionPoolOptions::builder()
                .with_max_metadata_request_timeout(timeout)
                .build()
                .unwrap(),
        )
        .with_http_client_factory(Arc::new(PageFactory(client.clone())))
        .build()
        .await
        .unwrap();
    let account = AccountReference::with_account_key(
        Url::parse("https://test.documents.azure.com/").unwrap(),
        "dGVzdA==",
    );
    let driver = runtime
        .create_driver(
            DriverOptions::builder(account.clone())
                .with_preferred_regions(if third_region {
                    vec![
                        Region::new("East US"),
                        Region::new("West US"),
                        Region::new("Central US"),
                    ]
                } else {
                    vec![Region::new("East US"), Region::new("West US")]
                })
                .with_partition_failover_options(
                    PartitionFailoverOptions::builder()
                        .with_partition_topology_cache_mode(topology_mode)
                        .with_circuit_breaker_enabled(false)
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .await
        .unwrap();
    let properties = ContainerProperties {
        id: "items".into(),
        partition_key: serde_json::from_str(r#"{"paths":["/pk"],"version":2}"#).unwrap(),
        system_properties: Default::default(),
    };
    let container =
        ContainerReference::new(account, "db", "db-rid", "items", "items-rid", &properties);
    (driver, client, container)
}

fn options(count: u32, wait: Duration, hedge: bool) -> OperationOptions {
    let mut options = OperationOptions {
        throttling_retry_options: Some(ThrottlingRetryOptions {
            max_retry_count: Some(count),
            max_retry_wait_time: Some(wait),
        }),
        availability_strategy: Some(AvailabilityStrategy::Disabled),
        ..Default::default()
    };
    if hedge {
        options.availability_strategy = Some(AvailabilityStrategy::Hedging(HedgingStrategy::new(
            HedgeThreshold::new(Duration::from_millis(1)).unwrap(),
        )));
    }
    options
}

#[tokio::test]
async fn later_page_keeps_ranges_continuation_region_and_request_id_across_caps() {
    for (count, wait) in [
        (0, Duration::ZERO),
        (2, Duration::from_millis(1)),
        (u32::MAX, Duration::MAX),
    ] {
        for hedge in [false, true] {
            let (driver, client, container) = setup(hedge, false).await;
            *client.remaining_throttles.lock().unwrap() = 25;
            let options = options(count, wait, hedge);
            let map = driver
                .pk_range_cache
                .try_lookup_result(
                    &container,
                    false,
                    driver.pk_range_page_fetcher(options.clone(), None),
                )
                .await
                .unwrap()
                .unwrap();
            assert_eq!(
                map.ranges()
                    .iter()
                    .map(|r| r.id.as_str())
                    .collect::<Vec<_>>(),
                ["0", "1"]
            );
            let attempts = client.attempts.lock().unwrap().clone();
            let later: Vec<_> = attempts
                .iter()
                .filter(|a| a.token.as_deref() == Some("page-1"))
                .collect();
            assert_eq!(later.len(), 26);
            assert!(later
                .iter()
                .all(|a| a.activity_id == later[0].activity_id && a.host == later[0].host));
            assert!(later[0].host.contains(if hedge { "west" } else { "east" }));
            assert_eq!(
                attempts.iter().filter(|a| a.token.is_none()).count(),
                if hedge { 2 } else { 1 }
            );
            let cold_id = &attempts
                .iter()
                .find(|a| a.token.is_none())
                .unwrap()
                .activity_id;
            assert_ne!(cold_id, &later[0].activity_id);

            client.attempts.lock().unwrap().clear();
            *client.remaining_throttles.lock().unwrap() = 15;
            *client.throttle_token.lock().unwrap() = Some("page-2");
            let refreshed = driver
                .pk_range_cache
                .try_lookup_result(
                    &container,
                    true,
                    driver.pk_range_page_fetcher(options.clone(), None),
                )
                .await
                .unwrap()
                .unwrap();
            assert_eq!(refreshed.ranges(), map.ranges());
            let attempts = client.attempts.lock().unwrap().clone();
            assert_eq!(attempts.len(), 16);
            assert!(attempts
                .iter()
                .all(|a| a.token.is_some() && a.host == later[0].host));
            assert!(attempts[..16]
                .iter()
                .all(|a| a.activity_id == attempts[0].activity_id));

            if count != u32::MAX {
                let unrelated = CosmosOperation::read_container(container.clone());
                let error = driver
                    .execute_operation_direct(&unrelated, OperationOverrides::default(), &options)
                    .await
                    .unwrap_err();
                assert!(error.status().is_throttled());
                assert_eq!(
                    error.diagnostics().unwrap().request_count(),
                    (if hedge { 3 } else { 1 }) * (count as usize + 1)
                );
            }
        }
    }
}

#[tokio::test]
async fn each_primary_or_hedged_batch_has_bounded_diagnostics_even_with_huge_caps() {
    for hedge in [false, true] {
        let (driver, client, container) = setup(hedge, false).await;
        *client.remaining_throttles.lock().unwrap() = u32::MAX;
        let operation = CosmosOperation::read_all_partition_key_ranges(container)
            .with_precondition(crate::models::Precondition::if_none_match("page-1"));
        let error = driver
            .execute_operation_direct(
                &operation,
                OperationOverrides::default(),
                &options(u32::MAX, Duration::MAX, hedge),
            )
            .await
            .unwrap_err();
        assert!(error.status().is_throttled());
        assert!(error.diagnostics().unwrap().request_count() <= 2);
        assert!(client.attempts.lock().unwrap().len() <= 2);
    }
}

#[tokio::test]
async fn cold_hedges_allow_a_healthy_partner_after_either_leg_throttles() {
    use crate::driver::pipeline::hedging_diagnostics::HedgeTerminalState;

    for (east_status, east_delay, west_status, west_delay, after_hedge) in [
        (429, 0, 200, 30, false),
        (429, 0, 200, 30, true),
        (200, 30, 429, 0, true),
        (200, 0, 429, 30, true),
        (429, 30, 200, 0, true),
        (429, 0, 200, 6_000, false),
    ] {
        let (driver, client, container) = setup_with_metadata_timeout(
            true,
            false,
            Duration::from_millis(west_delay.max(east_delay) + 100),
            false,
        )
        .await;
        client.hedge.store(false, Ordering::SeqCst);
        client
            .hold_primary_until_hedge
            .store(after_hedge, Ordering::SeqCst);
        *client.cold_replies.lock().unwrap() = Some(vec![
            ColdReply {
                status: east_status,
                substatus: None,
                delay: Duration::from_millis(east_delay),
                pending: false,
            },
            ColdReply {
                status: west_status,
                substatus: None,
                delay: Duration::from_millis(west_delay),
                pending: false,
            },
        ]);
        let deadline = Instant::now() + Duration::from_secs(30);
        let operation = CosmosOperation::read_all_partition_key_ranges(container)
            .with_absolute_deadline(Some(deadline));
        let response = driver
            .execute_operation_direct(
                &operation,
                OperationOverrides::default(),
                &options(0, Duration::ZERO, true),
            )
            .await
            .unwrap();
        let diagnostics = response.diagnostics();
        let expected = if east_status == 200 {
            HedgeTerminalState::PrimaryWonAfterHedge
        } else {
            HedgeTerminalState::AlternateWon
        };
        assert_eq!(
            diagnostics.hedge_diagnostics().unwrap().terminal_state(),
            expected,
            "east={east_status}/{east_delay}ms west={west_status}/{west_delay}ms"
        );
        assert!(diagnostics.request_count() <= 2);
        let attempts = client.attempts.lock().unwrap();
        assert_eq!(attempts.len(), 2);
        assert!(attempts.iter().all(|attempt| attempt.token.is_none()));
        assert_eq!(attempts[0].activity_id, attempts[1].activity_id);
    }
}

#[tokio::test]
async fn pending_hedge_partner_does_not_prevent_bounded_throttle_batches() {
    use crate::driver::pipeline::hedging_diagnostics::HedgeTerminalState;

    for pending_primary in [false, true] {
        let (driver, client, container) = setup(true, false).await;
        client.hedge.store(false, Ordering::SeqCst);
        *client.cold_replies.lock().unwrap() = Some(vec![
            ColdReply {
                status: 429,
                substatus: None,
                delay: Duration::ZERO,
                pending: pending_primary,
            },
            ColdReply {
                status: 429,
                substatus: None,
                delay: Duration::ZERO,
                pending: !pending_primary,
            },
        ]);
        let deadline = Some(Instant::now() + Duration::from_secs(30));
        let operation = CosmosOperation::read_all_partition_key_ranges(container)
            .with_absolute_deadline(deadline)
            .with_activity_id(crate::models::ActivityId::new_uuid());
        for _ in 0..2 {
            let error = driver
                .execute_operation_direct(
                    &operation,
                    OperationOverrides::default(),
                    &options(0, Duration::ZERO, true),
                )
                .await
                .unwrap_err();
            assert!(error.status().is_throttled());
            let diagnostics = error.diagnostics().unwrap();
            assert!(matches!(
                diagnostics.hedge_diagnostics().unwrap().terminal_state(),
                HedgeTerminalState::BothTransient {
                    deadline_elapsed: false
                }
            ));
            assert!(diagnostics.request_count() <= 2);
        }
        let attempts = client.attempts.lock().unwrap();
        assert_eq!(attempts.len(), 4);
        assert!(attempts.iter().all(
            |attempt| attempt.activity_id == attempts[0].activity_id && attempt.token.is_none()
        ));
    }
}

#[tokio::test]
async fn cold_capacity_throttle_fails_over_but_continuation_stays_pinned() {
    let (driver, client, container) = setup(false, false).await;
    *client.cold_replies.lock().unwrap() = Some(vec![
        ColdReply {
            status: 429,
            substatus: Some(3092),
            delay: Duration::ZERO,
            pending: false,
        },
        ColdReply {
            status: 200,
            substatus: None,
            delay: Duration::ZERO,
            pending: false,
        },
    ]);
    let operation = CosmosOperation::read_all_partition_key_ranges(container);
    driver
        .execute_operation_direct(
            &operation,
            OperationOverrides::default(),
            &options(0, Duration::ZERO, false),
        )
        .await
        .unwrap();
    {
        let attempts = client.attempts.lock().unwrap();
        assert_eq!(attempts.len(), 2);
        assert!(attempts[0].host.contains("east"));
        assert!(attempts[1].host.contains("west"));
    }
    client.attempts.lock().unwrap().clear();
    client.capacity_throttle.store(true, Ordering::SeqCst);
    *client.remaining_throttles.lock().unwrap() = u32::MAX;
    let operation =
        operation.with_precondition(crate::models::Precondition::if_none_match("page-1"));
    let pin = RegionPin {
        endpoint: Some(CosmosEndpoint::regional(
            Region::new("East US"),
            Url::parse("https://test-east.documents.azure.com").unwrap(),
        )),
    };
    let error = driver
        .execute_operation_direct(
            &operation,
            OperationOverrides {
                region_pin: Some(Box::new(pin)),
                ..Default::default()
            },
            &options(0, Duration::ZERO, false),
        )
        .await
        .unwrap_err();
    assert!(error.status().is_throttled());
    let attempts = client.attempts.lock().unwrap();
    assert_eq!(attempts.len(), 1);
    assert!(attempts[0].host.contains("east"));
    assert_eq!(attempts[0].token.as_deref(), Some("page-1"));
}

#[test]
fn retry_delay_is_positive_bounded_and_saturates() {
    for count in [0, 9, 64, i32::MAX as u32, u32::MAX] {
        for retry_after in [None, Some(0), Some(1), Some(u64::MAX)] {
            let delay = pk_range_retry_delay(count, retry_after);
            assert!(delay >= Duration::from_millis(5) && delay <= Duration::from_secs(5));
        }
    }
}

#[tokio::test]
async fn cold_throttle_sweep_reaches_third_region_and_bounds_huge_failover_caps() {
    for (hedge, substatus) in [(false, Some(3092)), (true, Some(3092)), (true, None)] {
        for third_status in [200, 429] {
            let (driver, client, container) =
                setup_with_metadata_timeout(hedge, false, Duration::from_millis(100), true).await;
            client.hedge.store(false, Ordering::SeqCst);
            *client.cold_replies.lock().unwrap() = Some(vec![
                ColdReply {
                    status: 429,
                    substatus,
                    delay: Duration::ZERO,
                    pending: false,
                },
                ColdReply {
                    status: 429,
                    substatus,
                    delay: Duration::ZERO,
                    pending: false,
                },
                ColdReply {
                    status: third_status,
                    substatus,
                    delay: Duration::ZERO,
                    pending: false,
                },
            ]);
            let operation = CosmosOperation::read_all_partition_key_ranges(container)
                .with_activity_id(crate::models::ActivityId::new_uuid());
            let mut options = options(u32::MAX, Duration::MAX, hedge);
            options.max_failover_retry_count = Some(u32::MAX);
            let result = driver
                .execute_operation_direct(&operation, OperationOverrides::default(), &options)
                .await;
            let diagnostics = if third_status == 200 {
                result.unwrap().diagnostics().clone()
            } else {
                let error = result.unwrap_err();
                assert!(error.status().is_throttled());
                error.diagnostics().unwrap().clone()
            };
            assert!(diagnostics.request_count() <= 4);
            let attempts = client.attempts.lock().unwrap();
            assert!(attempts.len() <= 4, "{attempts:?}");
            assert!(
                attempts
                    .iter()
                    .any(|attempt| attempt.host.contains("central")),
                "{attempts:?}"
            );
            assert!(attempts
                .iter()
                .all(|attempt| attempt.activity_id == attempts[0].activity_id
                    && attempt.token.is_none()));
        }
    }
}

#[tokio::test(start_paused = true)]
async fn cold_hedge_throttles_apply_completed_leg_effects_once_to_original_endpoints() {
    for (partner_status, partner_substatus) in [
        (429, Some(3092)),
        (200, None),
        (503, None),
        (429, Some(3200)),
    ] {
        for primary_first in [false, true] {
            let (driver, client, container) =
                setup_with_metadata_timeout(false, false, Duration::from_millis(100), true).await;
            client
                .hold_primary_until_hedge
                .store(true, Ordering::SeqCst);
            let capacity = ColdReply {
                status: 429,
                substatus: Some(3092),
                delay: Duration::ZERO,
                pending: false,
            };
            let partner = ColdReply {
                status: partner_status,
                substatus: partner_substatus,
                delay: Duration::from_millis(30),
                pending: false,
            };
            *client.cold_replies.lock().unwrap() = Some(vec![
                if primary_first { capacity } else { partner },
                if primary_first { partner } else { capacity },
                ColdReply {
                    status: 200,
                    substatus: None,
                    delay: Duration::ZERO,
                    pending: false,
                },
            ]);
            let initial_version = driver.location_state_store.version_for_tests();
            let operation = CosmosOperation::read_all_partition_key_ranges(container);
            let result = driver
                .execute_operation_direct(
                    &operation,
                    OperationOverrides::default(),
                    &options(0, Duration::ZERO, true),
                )
                .await;
            let diagnostics = if partner_substatus == Some(3200) {
                let error = result.unwrap_err();
                assert_eq!(
                    error.status().sub_status().unwrap().value(),
                    if primary_first { 3092 } else { 3200 },
                    "the primary metadata response must remain authoritative"
                );
                error.diagnostics().unwrap().clone()
            } else {
                result.unwrap().diagnostics().clone()
            };
            let both_unavailable = partner_status == 503 || partner_substatus == Some(3092);
            let expected_marks = if both_unavailable { 2 } else { 1 };
            let snapshot = driver.location_state_store.snapshot();
            assert_eq!(
                snapshot.account.unavailable_endpoints.len(),
                expected_marks,
                "every completed capacity/503 leg must mark only its own endpoint"
            );
            assert_eq!(
                driver.location_state_store.version_for_tests() - initial_version,
                expected_marks as u64,
                "each leg's endpoint effect must be applied exactly once"
            );
            for endpoint in snapshot.account.unavailable_endpoints.keys() {
                let host = endpoint.host_str().unwrap();
                assert!(
                    host.contains(if primary_first { "east" } else { "west" })
                        || (both_unavailable
                            && host.contains(if primary_first { "west" } else { "east" })),
                    "effect was attributed to the wrong endpoint"
                );
            }
            let attempts = client.attempts.lock().unwrap();
            let expected_attempts = if both_unavailable { 3 } else { 2 };
            assert_eq!(attempts.len(), expected_attempts);
            assert_eq!(diagnostics.request_count(), expected_attempts);
            assert_eq!(
                attempts
                    .iter()
                    .any(|attempt| attempt.host.contains("central")),
                both_unavailable,
                "remaining failover budget must reach the healthy third region"
            );
        }
    }
}

#[tokio::test]
async fn mixed_hedge_throttles_preserve_primary_result_without_regional_sweep() {
    for substatus in [3200, 3210, 3214] {
        for primary_first in [false, true] {
            let (driver, client, container) =
                setup_with_metadata_timeout(true, false, Duration::from_millis(100), true).await;
            client.hedge.store(false, Ordering::SeqCst);
            client
                .hold_primary_until_hedge
                .store(true, Ordering::SeqCst);
            *client.cold_replies.lock().unwrap() = Some(vec![
                ColdReply {
                    status: 429,
                    substatus: None,
                    delay: Duration::from_millis(if primary_first { 0 } else { 30 }),
                    pending: false,
                },
                ColdReply {
                    status: 429,
                    substatus: Some(substatus),
                    delay: Duration::from_millis(if primary_first { 30 } else { 0 }),
                    pending: false,
                },
                ColdReply {
                    status: 200,
                    substatus: None,
                    delay: Duration::ZERO,
                    pending: false,
                },
            ]);
            let operation = CosmosOperation::read_all_partition_key_ranges(container)
                .with_activity_id(crate::models::ActivityId::new_uuid());
            let error = driver
                .execute_operation_direct(
                    &operation,
                    OperationOverrides::default(),
                    &options(0, Duration::ZERO, true),
                )
                .await
                .unwrap_err();
            assert!(error.status().is_throttled(), "{error:?}");
            assert!(error
                .status()
                .sub_status()
                .is_none_or(|status| status.value() == 0));
            assert_eq!(error.diagnostics().unwrap().request_count(), 2);
            let attempts = client.attempts.lock().unwrap();
            assert_eq!(attempts.len(), 2);
            assert!(attempts.iter().all(|attempt| attempt.token.is_none()
                && !attempt.host.contains("central")
                && attempt.activity_id == attempts[0].activity_id));
        }
    }
}

#[tokio::test]
async fn final_ru_budget_throttles_do_not_trigger_a_regional_sweep() {
    for substatus in [3200, 3210, 3214] {
        let (driver, client, container) = setup(true, false).await;
        client.hedge.store(false, Ordering::SeqCst);
        *client.cold_replies.lock().unwrap() = Some(vec![
            ColdReply {
                status: 429,
                substatus: Some(substatus),
                delay: Duration::ZERO,
                pending: false,
            },
            ColdReply {
                status: 200,
                substatus: None,
                delay: Duration::ZERO,
                pending: false,
            },
        ]);
        let operation = CosmosOperation::read_all_partition_key_ranges(container);
        let error = driver
            .execute_operation_direct(
                &operation,
                OperationOverrides::default(),
                &options(0, Duration::ZERO, true),
            )
            .await
            .unwrap_err();
        assert_eq!(error.status().sub_status().unwrap().value(), substatus);
        assert_eq!(client.attempts.lock().unwrap().len(), 1);
    }
}

#[tokio::test]
async fn cold_page_retries_keep_request_identity_on_primary_and_hedged_paths() {
    for hedge in [false, true] {
        let (driver, client, container) = setup(hedge, false).await;
        *client.throttle_token.lock().unwrap() = None;
        *client.remaining_throttles.lock().unwrap() = 12;
        let map = driver
            .pk_range_cache
            .try_lookup_result(
                &container,
                false,
                driver.pk_range_page_fetcher(options(0, Duration::ZERO, hedge), None),
            )
            .await
            .unwrap()
            .unwrap();
        assert_eq!(map.ranges().len(), 2);
        let attempts = client.attempts.lock().unwrap();
        let cold: Vec<_> = attempts.iter().filter(|a| a.token.is_none()).collect();
        assert_eq!(cold.len(), if hedge { 26 } else { 13 });
        assert!(cold.iter().all(|a| a.activity_id == cold[0].activity_id));
        let later: Vec<_> = attempts.iter().filter(|a| a.token.is_some()).collect();
        assert_eq!(later.len(), 2);
        assert!(later
            .iter()
            .all(|a| a.host.contains(if hedge { "west" } else { "east" })));
    }
}

#[tokio::test]
async fn non_throttle_pinned_failure_still_restarts_cold() {
    let (driver, client, container) = setup(false, false).await;
    let original = driver
        .resolve_all_partition_key_ranges(&container, false)
        .await
        .unwrap()
        .unwrap();
    client.attempts.lock().unwrap().clear();
    client.fail_pinned.store(true, Ordering::SeqCst);
    let refreshed = driver
        .resolve_all_partition_key_ranges(&container, true)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(refreshed, original);
    let attempts = client.attempts.lock().unwrap();
    assert_eq!(attempts.len(), 4);
    assert_eq!(attempts[0].token.as_deref(), Some("page-2"));
    assert!(attempts[1].token.is_none());
}

async fn wait_for_page_attempts(client: &PageClient, token: Option<&str>, count: usize) {
    tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            if client
                .attempts
                .lock()
                .unwrap()
                .iter()
                .filter(|attempt| attempt.token.as_deref() == token)
                .count()
                >= count
            {
                break;
            }
            client.attempted.notified().await;
        }
    })
    .await
    .expect("expected page attempts");
}

#[tokio::test]
async fn shared_eager_fill_survives_its_first_waiter_and_keeps_each_throttled_page() {
    for timeout in [false, true] {
        let (driver, client, container) = setup_with_topology_mode(
            false,
            false,
            Duration::from_millis(100),
            false,
            PartitionTopologyCacheMode::Eager,
        )
        .await;
        client.retry_after_ms.store(20, Ordering::SeqCst);
        client.page_throttles.lock().unwrap().extend([
            ("page-1".to_owned(), u32::MAX),
            ("page-2".to_owned(), u32::MAX),
        ]);
        let mut first_options = options(0, Duration::ZERO, false);
        first_options.end_to_end_latency_policy = Some(Duration::from_millis(60).into());
        let mut first = Box::pin(driver.prime_partition_topology(&container, first_options));
        tokio::select! {
            result = &mut first => panic!("eager fill completed before throttled page: {result:?}"),
            () = wait_for_page_attempts(&client, Some("page-1"), 2) => {}
        }
        if timeout {
            assert!(first.await.unwrap_err().status().is_timeout());
        } else {
            drop(first);
        }
        assert!(driver
            .pk_range_cache
            .cached_routing_map(&container)
            .await
            .unwrap()
            .is_none());

        // A later transaction/session-topology caller must join the same fill.
        let mut second = Box::pin(driver.load_session_token_routing_map(
            &container,
            options(0, Duration::ZERO, false),
            Some(Instant::now() + Duration::from_secs(2)),
        ));
        assert!(futures::poll!(&mut second).is_pending());
        *client
            .page_throttles
            .lock()
            .unwrap()
            .get_mut("page-1")
            .unwrap() = 0;
        tokio::select! {
            result = &mut second => panic!("published before the final page completed: {result:?}"),
            () = wait_for_page_attempts(&client, Some("page-2"), 2) => {}
        }
        assert!(driver
            .pk_range_cache
            .cached_routing_map(&container)
            .await
            .unwrap()
            .is_none());
        *client
            .page_throttles
            .lock()
            .unwrap()
            .get_mut("page-2")
            .unwrap() = 0;
        let map = second.await.unwrap().unwrap();
        assert_eq!(
            map.ranges()
                .iter()
                .map(|range| range.id.as_str())
                .collect::<Vec<_>>(),
            ["0", "1"]
        );
        let attempts = client.attempts.lock().unwrap();
        assert_eq!(
            attempts
                .iter()
                .filter(|attempt| attempt.token.is_none())
                .count(),
            1
        );
        for token in ["page-1", "page-2"] {
            let page: Vec<_> = attempts
                .iter()
                .filter(|attempt| attempt.token.as_deref() == Some(token))
                .collect();
            assert!(page.len() >= 3);
            assert!(page
                .iter()
                .all(|attempt| attempt.activity_id == page[0].activity_id
                    && attempt.host == page[0].host));
            assert!(page.windows(2).all(|pair| {
                pair[1].started.duration_since(pair[0].started) >= Duration::from_millis(20)
            }));
        }
    }
}

#[tokio::test]
async fn unobserved_completed_fill_does_not_swallow_a_later_forced_refresh() {
    let (driver, client, container) = setup(false, false).await;
    *client.remaining_throttles.lock().unwrap() = u32::MAX;
    let mut waiter = Box::pin(driver.load_session_token_routing_map(
        &container,
        options(0, Duration::ZERO, false),
        None,
    ));
    tokio::select! {
        result = &mut waiter => panic!("fill unexpectedly completed: {result:?}"),
        () = wait_for_page_attempts(&client, Some("page-1"), 1) => {}
    }
    drop(waiter);
    *client.remaining_throttles.lock().unwrap() = 0;
    let published = tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            if let Some(map) = driver
                .pk_range_cache
                .cached_routing_map(&container)
                .await
                .unwrap()
            {
                break map;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    client.attempts.lock().unwrap().clear();
    let refreshed = driver
        .lookup_partition_topology(&container, true, OperationOptions::default())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(refreshed.ranges(), published.ranges());
    let attempts = client.attempts.lock().unwrap();
    assert_eq!(attempts.len(), 1);
    assert_eq!(attempts[0].token.as_deref(), Some("page-2"));
}

#[tokio::test]
async fn shared_refresh_outlives_deadline_without_replacing_the_previous_map_early() {
    let (driver, client, container) = setup(false, false).await;
    let previous = driver
        .load_session_token_routing_map(&container, OperationOptions::default(), None)
        .await
        .unwrap()
        .unwrap();
    client.attempts.lock().unwrap().clear();
    *client.throttle_token.lock().unwrap() = Some("page-2");
    *client.remaining_throttles.lock().unwrap() = u32::MAX;
    let mut first_options = options(0, Duration::ZERO, false);
    first_options.end_to_end_latency_policy = Some(Duration::from_millis(50).into());
    assert!(driver
        .lookup_partition_topology(&container, true, first_options)
        .await
        .unwrap_err()
        .status()
        .is_timeout());
    assert!(Arc::ptr_eq(
        &previous,
        &driver
            .pk_range_cache
            .cached_routing_map(&container)
            .await
            .unwrap()
            .unwrap()
    ));
    let mut second = Box::pin(driver.load_session_token_routing_map(
        &container,
        OperationOptions::default(),
        None,
    ));
    assert!(futures::poll!(&mut second).is_pending());
    *client.remaining_throttles.lock().unwrap() = 0;
    let refreshed = tokio::time::timeout(Duration::from_secs(2), second)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(refreshed.ranges(), previous.ranges());
    assert!(!Arc::ptr_eq(&refreshed, &previous));
    let attempts = client.attempts.lock().unwrap();
    assert!(attempts.len() > 1);
    assert!(attempts
        .iter()
        .all(|attempt| attempt.token.as_deref() == Some("page-2")
            && attempt.activity_id == attempts[0].activity_id));
}

#[tokio::test]
async fn dropping_the_driver_reclaims_shared_fills_even_during_a_transport_attempt() {
    for in_transport in [false, true] {
        let (driver, client, container) = setup(false, false).await;
        if in_transport {
            *client.cold_replies.lock().unwrap() = Some(vec![
                ColdReply {
                    status: 403,
                    substatus: None,
                    delay: Duration::from_millis(100),
                    pending: false,
                };
                2
            ]);
        }
        *client.remaining_throttles.lock().unwrap() = u32::MAX;
        let mut operation_options = options(0, Duration::ZERO, false);
        operation_options.max_failover_retry_count = Some(0);
        let mut waiter =
            Box::pin(driver.load_session_token_routing_map(&container, operation_options, None));
        tokio::select! {
            result = &mut waiter => panic!("fill unexpectedly completed: {result:?}"),
            () = wait_for_page_attempts(
                &client,
                if in_transport { None } else { Some("page-1") },
                1,
            ) => {}
        }
        drop(waiter);
        let weak = Arc::downgrade(&driver);
        drop(driver);
        tokio::time::timeout(Duration::from_secs(2), async {
            while weak.upgrade().is_some() {
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("a bounded metadata attempt must release the last driver reference");
        let attempts = client.attempts.lock().unwrap().len();
        tokio::time::sleep(Duration::from_millis(50)).await;
        assert_eq!(client.attempts.lock().unwrap().len(), attempts);
    }
}

#[tokio::test(start_paused = true)]
async fn dropping_last_driver_stops_staggered_container_fills() {
    let (driver, client, first) = setup(false, false).await;
    let properties = ContainerProperties {
        id: "other-items".into(),
        partition_key: serde_json::from_str(r#"{"paths":["/pk"],"version":2}"#).unwrap(),
        system_properties: Default::default(),
    };
    let second = ContainerReference::new(
        driver.account().clone(),
        "db",
        "db-rid",
        "other-items",
        "other-items-rid",
        &properties,
    );
    *client.cold_replies.lock().unwrap() = Some(vec![
        ColdReply {
            status: 429,
            substatus: None,
            delay: Duration::from_millis(100),
            pending: false,
        };
        2
    ]);
    client.retry_after_ms.store(20, Ordering::SeqCst);
    let mut operation_options = options(0, Duration::ZERO, false);
    operation_options.max_failover_retry_count = Some(0);
    let mut first_waiter =
        Box::pin(driver.load_session_token_routing_map(&first, operation_options.clone(), None));
    assert!(futures::poll!(&mut first_waiter).is_pending());
    wait_for_page_attempts(&client, None, 1).await;
    tokio::time::advance(Duration::from_millis(60)).await;
    let mut second_waiter =
        Box::pin(driver.load_session_token_routing_map(&second, operation_options, None));
    assert!(futures::poll!(&mut second_waiter).is_pending());
    wait_for_page_attempts(&client, None, 2).await;
    drop(first_waiter);
    drop(second_waiter);

    let last_owner = Arc::clone(&driver);
    let weak = Arc::downgrade(&driver);
    drop(driver);
    for _ in 0..15 {
        if client.attempts.lock().unwrap().len() >= 6 {
            break;
        }
        tokio::time::advance(Duration::from_millis(20)).await;
        tokio::task::yield_now().await;
    }
    let attempts = client.attempts.lock().unwrap().clone();
    assert!(
        attempts.len() >= 6,
        "both fills must keep retrying while owned"
    );
    let mut by_container = HashMap::new();
    for attempt in &attempts {
        *by_container.entry(&attempt.path).or_insert(0) += 1;
    }
    assert_eq!(by_container.len(), 2);
    assert!(by_container.values().all(|count| *count >= 2));
    assert_eq!(client.active_requests.load(Ordering::SeqCst), 2);
    drop(last_owner);

    for _ in 0..50 {
        tokio::time::advance(Duration::from_millis(20)).await;
        tokio::task::yield_now().await;
    }
    assert!(
        weak.upgrade().is_none(),
        "overlapping fills must not keep the driver alive for one another"
    );
    assert_eq!(client.active_requests.load(Ordering::SeqCst), 0);
    assert_eq!(client.attempts.lock().unwrap().len(), attempts.len());
}

#[tokio::test]
async fn shared_fill_non_throttle_failures_terminate_and_allow_a_later_retry() {
    for status in [403, 503] {
        let (driver, client, container) = setup(false, false).await;
        *client.cold_replies.lock().unwrap() = Some(vec![
            ColdReply {
                status,
                substatus: None,
                delay: Duration::ZERO,
                pending: false,
            };
            2
        ]);
        let mut operation_options = options(0, Duration::ZERO, false);
        operation_options.max_failover_retry_count = Some(1);
        let error = tokio::time::timeout(
            Duration::from_secs(2),
            driver.load_session_token_routing_map(&container, operation_options, None),
        )
        .await
        .expect("non-throttle retries must remain bounded")
        .unwrap_err();
        assert_eq!(u16::from(error.status().status_code()), status);
        assert!(client.attempts.lock().unwrap().len() <= 2);
        assert!(driver
            .pk_range_cache
            .cached_routing_map(&container)
            .await
            .unwrap()
            .is_none());
        *client.cold_replies.lock().unwrap() = None;
        assert_eq!(
            driver
                .load_session_token_routing_map(&container, OperationOptions::default(), None)
                .await
                .unwrap()
                .unwrap()
                .ranges()
                .len(),
            2
        );
    }
}

#[tokio::test]
async fn caller_deadline_stops_waiting_without_partial_publication() {
    let (driver, client, container) = setup(false, false).await;
    *client.remaining_throttles.lock().unwrap() = u32::MAX;
    let deadline = Instant::now() + Duration::from_millis(50);
    let result = driver
        .pk_range_cache
        .try_lookup_result_with_deadline(
            &container,
            false,
            driver.pk_range_page_fetcher(options(0, Duration::ZERO, false), Some(deadline)),
            Some(deadline),
        )
        .await;
    assert!(result.unwrap_err().status().is_timeout());
    assert!(driver
        .pk_range_cache
        .cached_routing_map(&container)
        .await
        .unwrap()
        .is_none());
    *client.remaining_throttles.lock().unwrap() = 0;
    assert_eq!(
        driver
            .resolve_all_partition_key_ranges(&container, false)
            .await
            .unwrap()
            .unwrap()
            .len(),
        2
    );
}

#[tokio::test]
async fn unfinished_cold_recovery_preserves_the_cached_continuations_region() {
    for timeout in [false, true] {
        let (driver, client, container) = setup(false, false).await;
        let original = driver
            .pk_range_cache
            .try_lookup_result(
                &container,
                false,
                driver.pk_range_page_fetcher(options(0, Duration::ZERO, false), None),
            )
            .await
            .unwrap()
            .unwrap();
        client.attempts.lock().unwrap().clear();
        client.fail_pinned.store(true, Ordering::SeqCst);
        client.hedge.store(true, Ordering::SeqCst);
        *client.remaining_throttles.lock().unwrap() = u32::MAX;
        let deadline = timeout.then(|| Instant::now() + Duration::from_secs(10));
        let fetch = driver.pk_range_page_fetcher(options(0, Duration::ZERO, true), None);
        let mut refresh = Box::pin(
            driver
                .pk_range_cache
                .try_lookup_result_with_deadline(&container, true, &fetch, deadline),
        );
        if timeout {
            assert!(refresh.await.unwrap_err().status().is_timeout());
        } else {
            loop {
                tokio::select! {
                    result = &mut refresh => panic!("refresh unexpectedly completed: {result:?}"),
                    () = client.attempted.notified() => {
                        if client.attempts.lock().unwrap().iter().any(|attempt| {
                            attempt.token.as_deref() == Some("page-1") && attempt.host.contains("west")
                        }) {
                            break;
                        }

                    }
                }
            }
            drop(refresh);
        }
        let cached = driver
            .pk_range_cache
            .cached_routing_map(&container)
            .await
            .unwrap()
            .unwrap();
        assert!(Arc::ptr_eq(&cached, &original));
        assert_eq!(
            cached.change_feed_next_if_none_match.as_deref(),
            Some("page-2")
        );
        {
            let attempts = client.attempts.lock().unwrap();
            assert!(
                attempts.iter().any(|attempt| {
                    attempt.token.as_deref() == Some("page-1") && attempt.host.contains("west")
                }),
                "cold recovery did not reach its second page: {attempts:?}"
            );
        }
        *client.remaining_throttles.lock().unwrap() = 0;
        client.attempts.lock().unwrap().clear();
        driver
            .pk_range_cache
            .try_lookup_result(&container, true, &fetch)
            .await
            .unwrap()
            .unwrap();
        let attempts = client.attempts.lock().unwrap().clone();
        assert_eq!(attempts.len(), 1);
        assert_eq!(attempts[0].token.as_deref(), Some("page-2"));
        assert!(attempts[0].host.contains("east"));

        client.fail_pinned.store(true, Ordering::SeqCst);
        driver
            .pk_range_cache
            .try_lookup_result(
                &container,
                true,
                driver.pk_range_page_fetcher(options(0, Duration::ZERO, true), None),
            )
            .await
            .unwrap()
            .unwrap();
        client.attempts.lock().unwrap().clear();
        driver
            .pk_range_cache
            .try_lookup_result(
                &container,
                true,
                driver.pk_range_page_fetcher(options(0, Duration::ZERO, true), None),
            )
            .await
            .unwrap()
            .unwrap();
        let attempts = client.attempts.lock().unwrap();
        assert_eq!(attempts.len(), 1);
        assert_eq!(attempts[0].token.as_deref(), Some("page-2"));
        assert!(attempts[0].host.contains("west"));
    }
}

#[tokio::test]
async fn cold_replacement_without_response_endpoint_does_not_inherit_the_cached_pin() {
    for cancel in [false, true] {
        let (driver, client, container) = setup(false, false).await;
        let original = driver
            .pk_range_cache
            .try_lookup_result(
                &container,
                false,
                driver.pk_range_page_fetcher(options(0, Duration::ZERO, false), None),
            )
            .await
            .unwrap()
            .unwrap();
        client.attempts.lock().unwrap().clear();
        client.fail_pinned.store(true, Ordering::SeqCst);
        client.hold_east_cold.store(true, Ordering::SeqCst);
        let fetch = driver.pk_range_page_fetcher(options(0, Duration::ZERO, false), None);
        let mut refresh = Box::pin(
            driver
                .pk_range_cache
                .try_lookup_result(&container, true, &fetch),
        );
        loop {
            tokio::select! {
                result = &mut refresh => panic!("refresh unexpectedly completed: {result:?}"),
                () = client.attempted.notified() => {
                    if client.attempts.lock().unwrap().iter().any(|attempt| attempt.token.is_none()) {
                        break;
                    }
                }
            }
        }

        // Remove the in-flight response's region before response_endpoint resolves it.
        let account_endpoint = AccountEndpoint::from(driver.account());
        let mut properties = (*driver
            .runtime
            .account_metadata_cache()
            .get(&account_endpoint)
            .await
            .unwrap())
        .clone();
        properties
            .readable_locations
            .retain(|location| location.name == Region::new("West US"));
        let properties = driver
            .runtime
            .account_metadata_cache()
            .get_or_refresh_with(account_endpoint, |_| true, || async { properties })
            .await
            .unwrap();
        driver
            .location_state_store
            .sync_account_properties(properties, driver.location_state_store.default_endpoint());
        if cancel {
            *client.remaining_throttles.lock().unwrap() = u32::MAX;
        }
        client.release_east_cold.notify_one();
        if cancel {
            loop {
                tokio::select! {
                    result = &mut refresh => panic!("refresh unexpectedly completed: {result:?}"),
                    () = client.attempted.notified() => {
                        if client.attempts.lock().unwrap().iter().any(|attempt| {
                            attempt.token.as_deref() == Some("page-1")
                        }) {
                            break;
                        }
                    }
                }
            }
            drop(refresh);
        } else {
            let replacement = refresh.await.unwrap().unwrap();
            assert!(!Arc::ptr_eq(&original, &replacement));
        }
        {
            let pins = driver.pk_range_region_pins.lock().unwrap();
            let pin = pins.get(&container).unwrap();
            assert!(
                pin.staged.is_none(),
                "unknown serving region must not inherit staged affinity"
            );
            let (cached, endpoint) = pin.cached.as_ref().unwrap();
            assert!(cached.ptr_eq(&Arc::downgrade(&original)));
            assert!(endpoint.url().host_str().unwrap().contains("east"));
        }
        let cached = driver
            .pk_range_cache
            .cached_routing_map(&container)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(Arc::ptr_eq(&cached, &original), cancel);
        *client.remaining_throttles.lock().unwrap() = 0;
        client.attempts.lock().unwrap().clear();
        driver
            .pk_range_cache
            .try_lookup_result(&container, true, &fetch)
            .await
            .unwrap()
            .unwrap();
        let attempts = client.attempts.lock().unwrap();
        assert_eq!(attempts.len(), 1);
        assert_eq!(attempts[0].token.as_deref(), Some("page-2"));
        assert!(
            attempts[0]
                .host
                .contains(if cancel { "east" } else { "west" }),
            "only cancellation should retain the previous map's east pin: {attempts:?}"
        );
    }
}

#[tokio::test]
async fn reused_fetcher_rebinds_after_another_drain_publishes_a_different_region() {
    let (driver, client, container) = setup(false, false).await;
    let fetch = driver.pk_range_page_fetcher(options(0, Duration::ZERO, true), None);
    driver
        .pk_range_cache
        .try_lookup_result(&container, false, &fetch)
        .await
        .unwrap()
        .unwrap();
    client.fail_pinned.store(true, Ordering::SeqCst);
    client.hedge.store(true, Ordering::SeqCst);
    driver
        .pk_range_cache
        .try_lookup_result(
            &container,
            true,
            driver.pk_range_page_fetcher(options(0, Duration::ZERO, true), None),
        )
        .await
        .unwrap()
        .unwrap();
    client.attempts.lock().unwrap().clear();
    driver
        .pk_range_cache
        .try_lookup_result(&container, true, &fetch)
        .await
        .unwrap()
        .unwrap();
    let attempts = client.attempts.lock().unwrap();
    assert_eq!(attempts.len(), 1);
    assert_eq!(attempts[0].token.as_deref(), Some("page-2"));
    assert!(attempts[0].host.contains("west"));
}

#[tokio::test]
async fn invalidated_drain_cannot_overwrite_the_new_maps_region_pin() {
    let (driver, client, container) = setup(false, false).await;
    client.hold_east_cold.store(true, Ordering::SeqCst);
    let mut old = Box::pin(driver.pk_range_cache.try_lookup_result(
        &container,
        false,
        driver.pk_range_page_fetcher(options(0, Duration::ZERO, false), None),
    ));
    tokio::select! {
        result = &mut old => panic!("old drain unexpectedly completed: {result:?}"),
        () = client.attempted.notified() => {}
    }
    driver.pk_range_cache.invalidate(&container).await;
    client.hedge.store(true, Ordering::SeqCst);
    let current = driver
        .pk_range_cache
        .try_lookup_result(
            &container,
            false,
            driver.pk_range_page_fetcher(options(0, Duration::ZERO, true), None),
        )
        .await
        .unwrap()
        .unwrap();
    client.release_east_cold.notify_one();
    old.await.unwrap().unwrap();
    assert!(Arc::ptr_eq(
        &current,
        &driver
            .pk_range_cache
            .cached_routing_map(&container)
            .await
            .unwrap()
            .unwrap()
    ));
    client.attempts.lock().unwrap().clear();
    driver
        .pk_range_cache
        .try_lookup_result(
            &container,
            true,
            driver.pk_range_page_fetcher(options(0, Duration::ZERO, true), None),
        )
        .await
        .unwrap()
        .unwrap();
    let attempts = client.attempts.lock().unwrap();
    assert_eq!(attempts.len(), 1);
    assert_eq!(attempts[0].token.as_deref(), Some("page-2"));
    assert!(attempts[0].host.contains("west"));
}
