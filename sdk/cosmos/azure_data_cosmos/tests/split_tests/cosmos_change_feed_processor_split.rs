// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Real-account CFP recovery using the same throughput-driven split as pull-reader tests.
//! The source and lease containers belong to the harness's uniquely named test database.

use super::{
    cosmos_query_split::force_split_and_wait,
    framework::{test_client::TEST_MODE_ENV_VAR, TestClient, TestOptions},
};
use azure_core::http::StatusCode;
use azure_data_cosmos::{
    feed::FeedRange,
    models::{ChangeFeedItem, ContainerProperties, ThroughputProperties, TimeToLive},
    options::{CreateContainerOptions, MaxItemCountHint, ReadFeedRangesOptions, Region},
    ChangeFeedProcessor, ChangeFeedProcessorBuilder, ChangeFeedProcessorContext, ContainerClient,
    CosmosError, CosmosStatus, FeedScope, LatestVersion,
};
use futures::{
    channel::{mpsc, oneshot},
    future::BoxFuture,
    FutureExt, StreamExt, TryStreamExt,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    io,
    num::NonZeroU32,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

const NAME: &str = "cfp-online-split";
const KEYS: u32 = 32;
const BASELINE_GROUPS: u32 = 4;
const CONTROL_WAIT: Duration = Duration::from_secs(180);
const INTERVAL: Duration = Duration::from_millis(100);
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
type Handler = Box<
    dyn Fn(
            ChangeFeedProcessorContext,
            Vec<ChangeFeedItem<Item>>,
        ) -> BoxFuture<'static, azure_data_cosmos::Result<()>>
        + Send
        + Sync,
>;
type Processor = ChangeFeedProcessor<LatestVersion, Item, Handler>;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Item {
    id: String,
    pk: String,
    sequence: u32,
    fence: bool,
}

fn item(key: u32, sequence: u32, fence: bool) -> Item {
    Item {
        id: format!("key-{key}-{sequence}"),
        pk: format!("key-{key}"),
        sequence,
        fence,
    }
}

#[derive(Default)]
struct Observed {
    items: BTreeMap<String, Item>,
    next: BTreeMap<String, u32>,
    deliveries: BTreeSet<(String, String)>,
    replays: usize,
    callbacks: usize,
    error: Option<String>,
}

impl Observed {
    fn accept(&mut self, worker: &str, lease: &str, items: &[Item]) -> Result<(), String> {
        for document in items {
            let key = document
                .pk
                .strip_prefix("key-")
                .and_then(|key| key.parse::<u32>().ok())
                .filter(|key| *key < KEYS)
                .ok_or_else(|| format!("Unexpected partition key: {}.", document.pk))?;
            if *document != item(key, document.sequence, document.fence) {
                return Err(format!("Unexpected document: {document:?}."));
            }
            if let Some(previous) = self.items.get(&document.id) {
                if previous != document {
                    return Err(format!("Replay changed document {}.", document.id));
                }
                self.replays += 1;
                continue;
            }
            let next = self.next.entry(document.pk.clone()).or_default();
            if document.sequence != *next {
                return Err(format!(
                    "Out-of-order first delivery for {}: expected {}, received {}.",
                    document.pk, next, document.sequence
                ));
            }
            *next += 1;
            self.items.insert(document.id.clone(), document.clone());
        }
        self.deliveries
            .insert((worker.to_owned(), lease.to_owned()));
        self.callbacks += 1;
        Ok(())
    }
}

struct Gate {
    worker: String,
    lease: String,
    fences: Vec<String>,
    release: oneshot::Sender<()>,
}

struct Sink {
    state: Mutex<Observed>,
    gates: mpsc::UnboundedSender<Gate>,
}

impl Sink {
    fn new() -> (Arc<Self>, mpsc::UnboundedReceiver<Gate>) {
        let (gates, receiver) = mpsc::unbounded();
        (
            Arc::new(Self {
                state: Mutex::new(Observed::default()),
                gates,
            }),
            receiver,
        )
    }

    fn failure(&self, message: impl Into<String>) -> CosmosError {
        let message = message.into();
        self.state.lock().unwrap().error = Some(message.clone());
        CosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::BadRequest))
            .with_message(message)
            .build()
    }

    fn check(&self) -> TestResult {
        match &self.state.lock().unwrap().error {
            Some(message) => Err(message.clone().into()),
            None => Ok(()),
        }
    }

    async fn callback(
        self: Arc<Self>,
        worker: String,
        context: ChangeFeedProcessorContext,
        changes: Vec<ChangeFeedItem<Item>>,
    ) -> azure_data_cosmos::Result<()> {
        let items: Vec<_> = changes
            .into_iter()
            .map(|change| {
                change
                    .current()
                    .cloned()
                    .ok_or_else(|| self.failure("Latest-version event has no current item."))
            })
            .collect::<azure_data_cosmos::Result<_>>()?;
        let lease = context.lease_token().to_owned();
        let recorded = self.state.lock().unwrap().accept(&worker, &lease, &items);
        if let Err(message) = recorded {
            return Err(self.failure(message));
        }
        let fences: Vec<_> = items
            .iter()
            .filter(|item| item.fence)
            .map(|item| item.pk.clone())
            .collect();
        if !fences.is_empty() {
            let (release, wait) = oneshot::channel();
            self.gates
                .unbounded_send(Gate {
                    worker,
                    lease,
                    fences,
                    release,
                })
                .map_err(|_| self.failure("Fence receiver closed."))?;
            wait.await
                .map_err(|_| self.failure("Fence callback was abandoned."))?;
        }
        Ok(())
    }
}

fn processor(
    source: &ContainerClient,
    leases: &ContainerClient,
    sink: &Arc<Sink>,
    worker: &str,
) -> TestResult<Processor> {
    let worker = worker.to_owned();
    let name = worker.clone();
    let sink = sink.clone();
    let handler: Handler = Box::new(move |context, changes| {
        sink.clone()
            .callback(worker.clone(), context, changes)
            .boxed()
    });
    Ok(
        ChangeFeedProcessorBuilder::new(LatestVersion, source.clone(), leases.clone(), NAME, name)
            .with_start_from_beginning()
            .with_max_scale_count(1)
            .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(4).unwrap()))
            .with_feed_poll_delay(INTERVAL)
            .handle_change_feed(handler)
            .build()?,
    )
}

fn ensure(condition: bool, message: &str) -> TestResult {
    if condition {
        Ok(())
    } else {
        Err(io::Error::other(message).into())
    }
}

async fn wait_for<T>(
    sink: &Sink,
    description: &str,
    mut condition: impl AsyncFnMut() -> TestResult<Option<T>>,
) -> TestResult<T> {
    tokio::time::timeout(CONTROL_WAIT, async {
        loop {
            sink.check()?;
            if let Some(value) = condition().await? {
                return Ok(value);
            }
            tokio::time::sleep(INTERVAL).await;
        }
    })
    .await
    .map_err(|_| io::Error::other(format!("Timed out waiting for {description}.")))?
}

async fn records(leases: &ContainerClient) -> TestResult<Vec<Value>> {
    let items = leases
        .query_items::<Value>("SELECT * FROM c", FeedScope::full_container(), None)
        .await?;
    let records = Box::pin(items).try_collect::<Vec<_>>().await?;
    Ok(records
        .into_iter()
        .filter(|record| {
            record["workload"]["processorName"] == NAME && record.get("range").is_some()
        })
        .collect())
}

fn active(record: &Value) -> bool {
    record.get("children").is_none_or(Value::is_null)
}

async fn progress(leases: &ContainerClient, id: &str) -> TestResult<Value> {
    Ok(leases
        .read_item(id.to_owned(), id, None)
        .await?
        .into_model()?)
}

async fn publish_group(source: &ContainerClient, sequence: u32, fence: bool) -> TestResult {
    for key in 0..KEYS {
        let item = item(key, sequence, fence);
        source
            .create_item(item.pk.clone(), &item.id.clone(), item, None)
            .await?;
    }
    Ok(())
}

async fn exercise(source: &ContainerClient, leases: &ContainerClient) -> TestResult {
    let before = source.read_feed_ranges(None).await?;
    ensure(
        before.len() == 1,
        "The bounded 13,000 RU/s split test requires a fresh single-range source.",
    )?;
    let (sink, mut gates) = Sink::new();
    let workers = [
        processor(source, leases, &sink, "host-a")?,
        processor(source, leases, &sink, "host-b")?,
    ];
    let result = async {
        for worker in &workers { worker.start().await?; }
        let initial = wait_for(&sink, "initial lease checkpoint", async || {
            let all = records(leases).await?;
            Ok((all.len() == 1 && all[0]["owner"].is_string() && all[0]["continuation"].is_string()).then_some(all))
        }).await?;
        let parent_id = initial[0]["id"].as_str().ok_or("Parent lease has no ID.")?.to_owned();
        for sequence in 0..BASELINE_GROUPS { publish_group(source, sequence, false).await?; }
        wait_for(&sink, "baseline delivery", async || {
            Ok((sink.state.lock().unwrap().items.len() == (KEYS * BASELINE_GROUPS) as usize).then_some(()))
        }).await?;
        println!("Both workers running; baseline drained: {} items over {KEYS} logical keys.", KEYS * BASELINE_GROUPS);

        let stop = AtomicBool::new(false);
        let split = async {
            let result = force_split_and_wait(source, before.len()).await;
            stop.store(true, Ordering::Release);
            result
        };
        let writer = async {
            let mut sequence = BASELINE_GROUPS;
            while !stop.load(Ordering::Acquire) {
                ensure(sequence < BASELINE_GROUPS + 1024, "Exceeded the bounded split-time write budget.")?;
                sink.check()?;
                publish_group(source, sequence, false).await?;
                sequence += 1;
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            Ok::<_, Box<dyn Error>>(sequence)
        };
        let (split, written) = futures::join!(split, writer);
        let after_count = split?;
        let next_sequence = written?;
        ensure(after_count == 2, "Expected two physical children for the two capped workers.")?;
        println!("Real topology grew {} -> {after_count}; {} items written during the split.", before.len(), (next_sequence - BASELINE_GROUPS) * KEYS);
        let physical = source.read_feed_ranges(Some(ReadFeedRangesOptions::default().with_force_refresh(true))).await?;
        let all = wait_for(&sink, "child lease ownership on both running workers", async || {
            let all = records(leases).await?;
            let active: Vec<_> = all.iter().filter(|record| active(record)).collect();
            let owners: BTreeSet<_> = active.iter().filter_map(|record| record["owner"].as_str()).collect();
            let ready = all.len() == 3 && active.len() == 2
                && owners == BTreeSet::from(["host-a", "host-b"])
                && active.iter().all(|record| record["continuation"].is_string());
            Ok(ready.then_some(all))
        }).await?;
        let parent = all.iter().find(|record| record["id"] == parent_id).ok_or("Retired parent is missing.")?;
        ensure(parent["owner"].is_null() && parent["continuation"].is_string(), "Parent was not retired with durable progress.")?;
        ensure(parent["workload"] == initial[0]["workload"], "Split changed workload identity.")?;
        let mut children: Vec<FeedRange> = serde_json::from_value(parent["children"].clone())?;
        children.sort_by(|left, right| left.min_inclusive().cmp(right.min_inclusive()));
        ensure(children == physical, "Retired parent children do not exactly cover the new physical topology.")?;
        let mut lease_ranges: Vec<FeedRange> = all.iter().filter(|record| active(record))
            .map(|record| serde_json::from_value(record["range"].clone())).collect::<Result<_, _>>()?;
        lease_ranges.sort_by(|left, right| left.min_inclusive().cmp(right.min_inclusive()));
        ensure(lease_ranges == physical, "Active child lease coverage differs from physical topology.")?;
        let child_ids: BTreeSet<_> = all.iter().filter(|record| active(record))
            .map(|record| record["id"].as_str().ok_or("Child lease has no ID.").map(str::to_owned))
            .collect::<Result<_, _>>()?;
        ensure(workers.iter().all(Processor::is_started), "A processor stopped during split recovery.")?;

        let mut covered = BTreeSet::new();
        for key in 0..KEYS {
            let ranges = source.feed_range_from_partition_key(format!("key-{key}"), None).await?;
            ensure(ranges.len() == 1, "A full partition key did not map to one physical range.")?;
            covered.insert(physical.iter().position(|range| *range == ranges[0]).ok_or("Unexpected post-split key range.")?);
        }
        ensure(covered.len() == physical.len(), "Workload keys did not exercise every physical child.")?;
        for sequence in next_sequence..next_sequence + 4 { publish_group(source, sequence, false).await?; }
        let final_sequence = next_sequence + 4;
        publish_group(source, final_sequence, true).await?;

        let mut durable = BTreeSet::new();
        while durable.len() < KEYS as usize {
            let gate = tokio::time::timeout(CONTROL_WAIT, gates.next()).await
                .map_err(|_| io::Error::other("Timed out waiting for final callback fences."))?
                .ok_or("Fence stream ended.")?;
            sink.check()?;
            ensure(child_ids.contains(&gate.lease), "Post-split fence was delivered on the retired parent.")?;
            let before = progress(leases, &gate.lease).await?;
            ensure(before["owner"].as_str() == Some(&gate.worker) && before["continuation"].is_string(), "Fence callback does not own a checkpointed child lease.")?;
            tokio::time::sleep(INTERVAL).await;
            let held = progress(leases, &gate.lease).await?;
            ensure(held["continuation"] == before["continuation"] && held["incarnation"] == before["incarnation"], "Checkpoint advanced while the callback was held.")?;
            gate.release.send(()).map_err(|_| io::Error::other("Fence callback was canceled."))?;
            wait_for(&sink, "acknowledged child checkpoint", async || {
                let after = progress(leases, &gate.lease).await?;
                ensure(after["owner"] == before["owner"] && after["incarnation"] == before["incarnation"], "Child ownership changed during final drain.")?;
                Ok((after["continuation"] != before["continuation"]).then_some(()))
            }).await?;
            durable.extend(gate.fences);
        }
        sink.check()?;
        ensure(workers.iter().all(Processor::is_started), "A processor stopped before the final drain.")?;
        let expected: BTreeMap<_, _> = (0..KEYS).flat_map(|key| (0..=final_sequence).map(move |sequence| {
            let item = item(key, sequence, sequence == final_sequence);
            (item.id.clone(), item)
        })).collect();
        let state = sink.state.lock().unwrap();
        ensure(state.items == expected, "Complete expected event coverage was not delivered.")?;
        let delivered_children: BTreeSet<_> = state.deliveries.iter().filter(|(_, lease)| child_ids.contains(lease)).map(|(_, lease)| lease.clone()).collect();
        ensure(delivered_children == child_ids, "Not every child lease delivered a callback.")?;
        println!("PASS: {} exact expected items; {} callbacks; {} known replays; {} durable per-key fences; two original workers stayed running, one child lease each; per-key first-delivery order preserved.", expected.len(), state.callbacks, state.replays, durable.len());
        Ok::<_, Box<dyn Error>>(())
    }.await;
    let mut outcome = result;
    for worker in &workers {
        let cleanup = if outcome.is_ok() {
            worker.stop().await
        } else {
            worker.abort().await
        };
        if let Err(error) = cleanup {
            eprintln!("Processor cleanup failed: {error}");
            if outcome.is_ok() {
                outcome = Err(error.into());
            }
        }
    }
    outcome
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "split"),
    ignore = "Requires a real account and temporarily provisions 13,000 RU/s."
)]
async fn processor_recovers_online_after_real_split() -> TestResult {
    ensure(std::env::var(TEST_MODE_ENV_VAR).as_deref() == Ok("required"), "Set AZURE_COSMOS_TEST_MODE=required so missing credentials cannot silently skip this live test.")?;
    TestClient::run_with_unique_db(
        async |run, database| {
            println!("Isolated CFP split-test database: {}", run.db_name());
            let source = run
                .create_container(
                    database,
                    ContainerProperties::new("cfp-split-source", "/pk".into()),
                    Some(
                        CreateContainerOptions::default()
                            .with_throughput(ThroughputProperties::manual(1000)),
                    ),
                )
                .await?;
            let mut properties = ContainerProperties::new("cfp-split-leases", "/id".into());
            properties.default_ttl = TimeToLive::NoDefault;
            let leases = run
                .create_container(
                    database,
                    properties,
                    Some(
                        CreateContainerOptions::default()
                            .with_throughput(ThroughputProperties::manual(400)),
                    ),
                )
                .await?;
            exercise(&source, &leases).await
        },
        Some(
            TestOptions::new()
                .with_client_application_region(Region::CENTRAL_US)
                .with_timeout(Duration::from_secs(40 * 60)),
        ),
    )
    .await
}

#[test]
fn ordering_accepts_known_replay_but_rejects_missing_or_reordered_first_delivery() {
    let mut state = Observed::default();
    state.accept("a", "parent", &[item(0, 0, false)]).unwrap();
    state
        .accept(
            "b",
            "child",
            &[item(0, 0, false), item(1, 0, false), item(0, 1, false)],
        )
        .unwrap();
    assert_eq!(state.replays, 1);
    assert!(state.accept("b", "child", &[item(0, 3, false)]).is_err());
    let mut changed = item(0, 0, false);
    changed.fence = true;
    assert!(state.accept("b", "child", &[changed]).is_err());
}
