// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Issue #5483: race one resumed partition page, not the whole query. The
//! request observer alone cannot see when a send returns or is cancelled.
//! Use the mock factory seam to hold one *real emulator response* in transit.

use std::{
    collections::{BTreeMap, BTreeSet},
    future::pending,
    num::NonZeroU32,
    sync::{Arc, Mutex},
    time::Duration,
};

use async_trait::async_trait;
use azure_core::http::{headers::HeaderName, Request, Url};
use azure_data_cosmos_driver::{
    driver::CosmosDriver,
    in_memory_emulator::{
        ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, ReplicationConfig,
        VirtualAccountConfig, VirtualRegion,
    },
    models::{
        AccountReference, ContainerReference, CosmosOperation, FeedRange, ItemReference,
        MaxItemCountHint, PartitionKey, PartitionKeyDefinition,
    },
    options::{
        AvailabilityStrategy, DriverOptions, HedgeThreshold, HedgingStrategy, OperationOptions,
        OperationOptionsBuilder, PlanOptions, QueryPlanMode, Region,
    },
    test::{
        ConnectionPoolOptions, HttpClientConfig, HttpClientFactory, HttpRequest, HttpResponse,
        TransportClient, TransportError,
    },
};

use tokio::sync::Notify;

use super::{page_document_values, GATEWAY_URL, REQUEST_CHARGE, SESSION_TOKEN, SUBSTATUS};

const EAST: &str = "eastus.emulator.local";
const WEST: &str = "westus.emulator.local";
const ROWS: u32 = 32;
const PAGE_SIZE: u32 = 2;

// Backend continuation, not the client-facing merge continuation.
type Leaf = (String, Option<String>);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Alternate {
    #[default]
    Healthy,
    Split,
    Lag,
}

#[derive(Debug)]
struct Attempt {
    leaf: Leaf,
    host: String,
    session: String,
    body: Vec<u8>,
    active: bool,
    returned: bool,
    charge: f64,
}

#[derive(Default, Debug)]
struct Trace {
    stall: Option<Leaf>,
    alternate: Alternate,
    attempts: Vec<Attempt>,
    peak_leaves: usize,
    peak_legs: usize,
}

struct InFlight<'a> {
    trace: &'a Mutex<Trace>,
    index: usize,
}

impl Drop for InFlight<'_> {
    fn drop(&mut self) {
        self.trace
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .attempts[self.index]
            .active = false;
    }
}

#[derive(Debug)]
struct Gateway {
    emulator: InMemoryEmulatorHttpClient,
    trace: Mutex<Trace>,
    release_primary: Notify,
}

#[derive(Debug)]
struct Factory(Arc<Gateway>);

impl HttpClientFactory for Factory {
    fn build(
        &self,
        _: &ConnectionPoolOptions,
        _: HttpClientConfig,
    ) -> azure_data_cosmos_driver::error::Result<Arc<dyn TransportClient>> {
        Ok(self.0.clone())
    }
}

impl Gateway {
    fn enter_query(&self, request: &HttpRequest) -> Option<(InFlight<'_>, bool)> {
        let header = |name| {
            request
                .headers
                .get_optional_str(&HeaderName::from_static(name))
        };
        let is_true = |name| header(name).is_some_and(|v| v.eq_ignore_ascii_case("true"));
        if !is_true("x-ms-documentdb-isquery") || is_true("x-ms-cosmos-is-query-plan-request") {
            return None;
        }
        let range = header("x-ms-documentdb-partitionkeyrangeid").expect("physical query range");
        let leaf = (
            range.to_owned(),
            header("x-ms-continuation").map(str::to_owned),
        );
        let session = request
            .headers
            .get_optional_str(&SESSION_TOKEN)
            .expect("seeded session");
        assert_eq!(session.split_once(':').unwrap().0, range);
        assert!(
            !session.contains(','),
            "expected a single leaf session: {session}"
        );

        let host = request.url.host_str().unwrap();
        let mut trace = self.trace.lock().unwrap();
        // Arm the first resumed page. Its primary already has a real response
        // before West is dispatched; only its delivery to the driver is held.
        if trace.alternate != Alternate::Healthy && trace.stall.is_none() && leaf.1.is_some() {
            trace.stall = Some(leaf.clone());
        }
        let stall = host == EAST && trace.stall.as_ref() == Some(&leaf);
        if host == WEST {
            assert_eq!(trace.stall.as_ref(), Some(&leaf));
            assert!(
                trace
                    .attempts
                    .iter()
                    .any(|a| a.active && a.host == EAST && a.leaf == leaf),
                "alternate must race an outstanding primary for the same page"
            );
        }
        let index = trace.attempts.len();
        trace.attempts.push(Attempt {
            leaf,
            host: host.to_owned(),
            session: session.to_owned(),
            body: request.body.as_ref().unwrap().to_vec(),
            active: true,
            returned: false,
            charge: 0.0,
        });
        let active = trace.attempts.iter().filter(|a| a.active);
        let legs = active.clone().count();
        let leaves = active.map(|a| &a.leaf).collect::<BTreeSet<_>>().len();
        trace.peak_legs = trace.peak_legs.max(legs);
        trace.peak_leaves = trace.peak_leaves.max(leaves);
        Some((
            InFlight {
                trace: &self.trace,
                index,
            },
            stall,
        ))
    }
}

#[async_trait]
impl TransportClient for Gateway {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        let leg = self.enter_query(request);
        let mut core = Request::new(request.url.clone(), request.method);
        *core.headers_mut() = request.headers.clone();
        if let Some(body) = &request.body {
            core.set_body(body.to_vec());
        }
        let alternate = self.trace.lock().unwrap().alternate;
        let west = leg.is_some() && request.url.host_str() == Some(WEST);
        if west {
            let range = request
                .headers
                .get_optional_str(&HeaderName::from_static(
                    "x-ms-documentdb-partitionkeyrangeid",
                ))
                .unwrap();
            match alternate {
                Alternate::Split => {
                    let store = self.emulator.store();
                    let parent = range.parse().unwrap();
                    store.split_partition("db", "coll", parent, Duration::ZERO);
                    store.wait_for_split("db", "coll", parent).await;
                }
                Alternate::Lag => {
                    // Require a future vector version on this replica only. The
                    // emulator echoes it in the real ReadSessionNotAvailable reply.
                    core.headers_mut()
                        .insert(SESSION_TOKEN.clone(), format!("{range}:999#999"));
                }
                Alternate::Healthy => {}
            }
        }
        let raw = self
            .emulator
            .execute_request(&core)
            .await
            .unwrap()
            .try_into_raw_response()
            .await
            .unwrap();
        if let Some((guard, stall)) = leg {
            let expected = match (west, alternate) {
                (true, Alternate::Split) => 410,
                (true, Alternate::Lag) => 404,
                _ => 200,
            };
            assert_eq!(u16::from(raw.status()), expected);
            if expected != 200 {
                assert_eq!(raw.headers().get_optional_str(&SUBSTATUS), Some("1002"));
            }
            if expected == 404 {
                assert!(raw
                    .headers()
                    .get_optional_str(&SESSION_TOKEN)
                    .unwrap()
                    .contains(":999#999"));
                self.release_primary.notify_one();
            }
            let charge = raw
                .headers()
                .get_optional_str(&REQUEST_CHARGE)
                .unwrap()
                .parse()
                .unwrap();
            self.trace.lock().unwrap().attempts[guard.index].charge = charge;
            if stall {
                // Lag must leave the primary eligible; a split must cancel it.
                if alternate == Alternate::Lag {
                    self.release_primary.notified().await;
                } else {
                    pending::<()>().await;
                }
            }
            self.trace.lock().unwrap().attempts[guard.index].returned = true;
        }
        Ok(HttpResponse {
            status: u16::from(raw.status()),
            headers: raw.headers().clone(),
            body: raw.body().to_vec(),
        })
    }
}

async fn drain(
    driver: &CosmosDriver,
    container: &ContainerReference,
    ordered: bool,
    strategy: AvailabilityStrategy,
    scope: FeedRange,
) -> Vec<(u64, String)> {
    let sql = if ordered {
        "SELECT * FROM c ORDER BY c.rank ASC"
    } else {
        "SELECT * FROM c"
    };
    let query = CosmosOperation::query_items(container.clone(), Some(scope))
        .with_body(
            serde_json::to_vec(&serde_json::json!({"query": sql, "parameters": []})).unwrap(),
        )
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(PAGE_SIZE).unwrap()));
    let options = OperationOptionsBuilder::new()
        .with_availability_strategy(strategy)
        .build();
    let planning = PlanOptions::default().with_query_plan_mode(QueryPlanMode::GatewayOnly);
    let mut plan = driver
        .plan_operation(query, &options, None, &planning)
        .await
        .unwrap();
    let mut rows = Vec::new();
    let mut pages = 0;
    while let Some(response) = driver
        .execute_plan(&mut plan, Some(container.clone()), options.clone())
        .await
        .unwrap()
    {
        let docs = page_document_values(response);
        assert!(docs.len() <= PAGE_SIZE as usize);
        pages += 1;
        assert!(pages <= 2 * ROWS, "query made no progress");
        for doc in docs {
            rows.push((
                doc["rank"].as_u64().unwrap(),
                doc["id"].as_str().unwrap().to_owned(),
            ));
        }
    }
    assert!(pages > 1);
    if !ordered {
        rows.sort_unstable();
    }
    rows
}

async fn fixture() -> (
    Arc<Gateway>,
    Arc<CosmosDriver>,
    ContainerReference,
    Vec<String>,
) {
    let config = VirtualAccountConfig::new(vec![
        VirtualRegion::new("East US", Url::parse(GATEWAY_URL).unwrap()),
        VirtualRegion::new(
            "West US",
            Url::parse("https://westus.emulator.local").unwrap(),
        ),
    ])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session)
    .with_replication_config(ReplicationConfig::immediate());
    let gateway = Arc::new(Gateway {
        emulator: InMemoryEmulatorHttpClient::new(config),
        trace: Mutex::default(),
        release_primary: Notify::new(),
    });
    let store = gateway.emulator.store();
    store.create_database("db");
    store.create_container_with_config(
        "db",
        "coll",
        PartitionKeyDefinition::new(vec!["/pk".into()]),
        ContainerConfig::new()
            .with_partition_count(2)
            .build()
            .unwrap(),
    );
    let runtime = azure_data_cosmos_driver::CosmosDriverRuntime::builder()
        .with_mock_http_client_factory(Arc::new(Factory(gateway.clone())))
        .build()
        .await
        .unwrap();
    let account =
        AccountReference::with_account_key(Url::parse(GATEWAY_URL).unwrap(), "ZW11bGF0b3Ita2V5");
    let driver = runtime
        .create_driver(
            DriverOptions::builder(account)
                .with_preferred_regions(vec![Region::EAST_US, Region::WEST_US])
                .build(),
        )
        .await
        .unwrap();
    let container = driver
        .resolve_container("db", "coll", OperationOptions::default())
        .await
        .unwrap();

    // Write through the driver, so both range sessions are populated before
    // the first query. Require interleaved range ownership in rank order.
    let mut owners = Vec::new();
    for i in 0..ROWS {
        let pk = format!("pk-{i}");
        let id = format!("id-{i}");
        let item = ItemReference::from_name(&container, PartitionKey::from(pk.clone()), id.clone());
        let body = serde_json::json!({"id": id, "pk": pk, "rank": ROWS - 1 - i});
        let response = driver
            .execute_singleton_operation(
                CosmosOperation::create_item(item).with_body(serde_json::to_vec(&body).unwrap()),
                OperationOptions::default(),
            )
            .await
            .unwrap();
        let token = response.headers().session_token.as_ref().unwrap().as_str();
        owners.push(token.split_once(':').unwrap().0.to_owned());
    }
    assert!(owners.windows(2).filter(|pair| pair[0] != pair[1]).count() > 1);
    (gateway, driver, container, owners)
}

fn expected_rows(owners: &[String], range: Option<&str>) -> Vec<(u64, String)> {
    (0..ROWS)
        .rev()
        .filter(|&i| range.is_none_or(|r| r == owners[i as usize]))
        .map(|i| (u64::from(ROWS - 1 - i), format!("id-{i}")))
        .collect()
}

fn hedging() -> AvailabilityStrategy {
    let threshold = HedgeThreshold::new(Duration::from_millis(100)).unwrap();
    AvailabilityStrategy::Hedging(HedgingStrategy::new(threshold))
}

async fn exercise(ordered: bool) {
    let (gateway, driver, container, owners) = fixture().await;
    let expected = expected_rows(&owners, None);
    let rows = drain(
        &driver,
        &container,
        ordered,
        AvailabilityStrategy::Disabled,
        FeedRange::full(),
    )
    .await;
    assert_eq!(rows, expected);
    let control = std::mem::take(&mut *gateway.trace.lock().unwrap());
    let leaves: BTreeSet<_> = control.attempts.iter().map(|a| a.leaf.clone()).collect();
    assert_eq!(
        leaves.len(),
        control.attempts.len(),
        "baseline repeated a leaf page"
    );
    assert!(control
        .attempts
        .iter()
        .all(|a| a.host == EAST && a.returned && !a.active));
    let control_ru: f64 = control.attempts.iter().map(|a| a.charge).sum();
    let resumed: BTreeSet<_> = leaves
        .iter()
        .filter(|(_, c)| c.is_some())
        .map(|(r, _)| r.clone())
        .collect();
    assert_eq!(resumed, owners.into_iter().collect::<BTreeSet<_>>());
    assert_eq!(resumed.len(), 2, "both physical ranges must paginate");
    let target = leaves.iter().find(|(_, c)| c.is_some()).unwrap().clone();

    for slow in [false, true] {
        gateway.trace.lock().unwrap().stall = slow.then(|| target.clone());
        let rows = tokio::time::timeout(
            Duration::from_secs(10),
            drain(&driver, &container, ordered, hedging(), FeedRange::full()),
        )
        .await
        .expect("stalled leaf never obtained an alternate response");
        assert_eq!(rows, expected);
        let trace = std::mem::take(&mut *gateway.trace.lock().unwrap());
        assert_eq!(control.peak_leaves, 1);
        assert_eq!(
            trace.peak_leaves, 1,
            "partition pages must not race each other"
        );
        assert_eq!(trace.peak_legs, if slow { 2 } else { 1 });
        let mut returned = BTreeMap::new();
        for a in &trace.attempts {
            assert!(!a.active && a.charge > 0.0);
            if a.returned {
                assert!(returned.insert(a.leaf.clone(), a.host.as_str()).is_none());
            }
        }
        assert_eq!(trace.attempts.len(), leaves.len() + usize::from(slow));
        for leaf in &leaves {
            let host = if slow && leaf == &target { WEST } else { EAST };
            assert_eq!(returned.get(leaf), Some(&host), "wrong winner for {leaf:?}");
        }
        if slow {
            let loser = trace.attempts.iter().find(|a| !a.returned).unwrap();
            let winner = trace.attempts.iter().find(|a| a.host == WEST).unwrap();
            assert_eq!(loser.host, EAST);
            assert_eq!(loser.leaf, winner.leaf);
            assert_eq!(loser.session, winner.session);
            assert_eq!(loser.body, winner.body);
        }
        // These are synthetic *backend response* charges. The withheld East
        // response was evaluated too, even though its RU never reached the caller.
        let served_ru: f64 = trace.attempts.iter().map(|a| a.charge).sum();
        let returned_ru: f64 = trace
            .attempts
            .iter()
            .filter(|a| a.returned)
            .map(|a| a.charge)
            .sum();
        assert!((returned_ru - control_ru).abs() < 1e-6);
        assert_eq!(served_ru > returned_ru, slow);
    }
}

#[tokio::test]
async fn paged_unordered_query_hedges_only_the_slow_leaf() {
    exercise(false).await;
}

#[tokio::test]
async fn paged_order_by_hedges_only_the_slow_leaf() {
    exercise(true).await;
}

#[tokio::test]
async fn alternate_range_gone_replaces_the_resumed_parent_page() {
    let (gateway, driver, container, owners) = fixture().await;
    gateway.trace.lock().unwrap().alternate = Alternate::Split;
    let rows = tokio::time::timeout(
        Duration::from_secs(10),
        drain(&driver, &container, false, hedging(), FeedRange::full()),
    )
    .await
    .expect("West's 410 must cancel East and re-resolve the parent");
    assert_eq!(rows, expected_rows(&owners, None));

    let trace = gateway.trace.lock().unwrap();
    let west = trace.attempts.iter().position(|a| a.host == WEST).unwrap();
    let parent = &trace.attempts[west].leaf;
    assert!(trace.attempts[west].returned && parent.1.is_some());
    assert!(trace.attempts[..west]
        .iter()
        .any(|a| a.leaf == *parent && !a.active && !a.returned));
    let children =
        gateway
            .emulator
            .store()
            .child_partition_ids("db", "coll", &[parent.0.parse().unwrap()]);
    assert_eq!(children.len(), 2);
    let subsequent = &trace.attempts[west + 1..];
    assert!(subsequent.iter().all(|a| a.leaf.0 != parent.0));
    for child in children {
        let child = child.to_string();
        let first = subsequent
            .iter()
            .find(|a| a.leaf.0 == child)
            .expect("both children must be drained");
        assert_eq!(
            first.leaf.1, parent.1,
            "resume the failed page on each child"
        );
    }
}

#[tokio::test]
async fn single_range_plan_ignores_alternate_session_lag() {
    let (gateway, driver, container, owners) = fixture().await;
    let ranges = driver
        .resolve_all_partition_key_ranges(&container, false)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(ranges.len(), 2);
    let range = ranges
        .into_iter()
        .max_by_key(|r| owners.iter().filter(|id| **id == r.id).count())
        .unwrap();
    let scope = FeedRange::new(range.min_inclusive, range.max_exclusive).unwrap();
    gateway.trace.lock().unwrap().alternate = Alternate::Lag;
    let rows = tokio::time::timeout(
        Duration::from_secs(10),
        drain(&driver, &container, false, hedging(), scope),
    )
    .await
    .expect("West's 404/1002 must not preempt the healthy primary");
    assert_eq!(rows, expected_rows(&owners, Some(&range.id)));

    let trace = gateway.trace.lock().unwrap();
    assert!(trace
        .attempts
        .iter()
        .all(|a| a.leaf.0 == range.id && a.returned));
    let mut alternates = trace
        .attempts
        .iter()
        .enumerate()
        .filter(|(_, a)| a.host == WEST);
    let (index, west) = alternates.next().expect("resumed scoped query page hedged");
    assert!(alternates.next().is_none());
    let east = &trace.attempts[index - 1];
    assert_eq!((&east.leaf, &east.session), (&west.leaf, &west.session));
    let tail = &trace.attempts[index + 1..];
    assert!(
        !tail.is_empty(),
        "observe tracking on subsequent primary pages"
    );
    assert!(tail
        .iter()
        .all(|a| a.host == EAST && !a.session.contains(":999#")));
}
