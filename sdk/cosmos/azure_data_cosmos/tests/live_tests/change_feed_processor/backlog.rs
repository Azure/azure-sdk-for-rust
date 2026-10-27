// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{live_containers, records, TestResult};
use azure_core::http::StatusCode;
use azure_data_cosmos::{
    models::ChangeFeedItem, options::MaxItemCountHint, ChangeFeedProcessor,
    ChangeFeedProcessorBuilder, ChangeFeedProcessorContext, ContainerClient, CosmosError,
    CosmosStatus, FeedScope, LatestVersion, TransactionalBatch,
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
    io,
    num::NonZeroU32,
    sync::{Arc, Mutex},
    time::Duration,
};
use uuid::Uuid;

const GROUPS: u32 = 32;
const CONTROL_WAIT: Duration = Duration::from_secs(120);
const DRAIN_WAIT: Duration = Duration::from_secs(240);
const INTERVAL: Duration = Duration::from_millis(100);

type Handler = Box<
    dyn Fn(
            ChangeFeedProcessorContext,
            Vec<ChangeFeedItem<Value>>,
        ) -> BoxFuture<'static, azure_data_cosmos::Result<()>>
        + Send
        + Sync,
>;
type Processor = ChangeFeedProcessor<LatestVersion, Value, Handler>;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Document {
    id: String,
    pk: String,
    run: String,
    group: u32,
    member: u32,
    fence: bool,
}

#[derive(Clone)]
struct Key {
    index: usize,
    range: usize,
    pk: String,
}

struct Workload {
    name: String,
    keys: Vec<Key>,
    range_count: usize,
    groups: u32,
    expected: BTreeMap<String, Document>,
}

impl Workload {
    async fn discover(source: &ContainerClient, groups: u32) -> TestResult<Self> {
        let properties = source.read(None).await?.into_model()?;
        if properties.partition_key.paths() != ["/pk"] {
            return Err("Backlog tests require a /pk source container.".into());
        }
        let ranges = source.read_feed_ranges(None).await?;
        if !(2..=8).contains(&ranges.len()) {
            return Err(format!("Backlog tests require 2..=8 actual feed ranges; found {}. Provision a dedicated partitioned test source.", ranges.len()).into());
        }
        let name = format!("cfp-backlog-{}", Uuid::new_v4().simple());
        let mut keys_by_range = vec![Vec::new(); ranges.len()];
        for candidate in 0..4096 {
            let pk = format!("{name}-key-{candidate}");
            let mapped = source
                .feed_range_from_partition_key(pk.clone(), None)
                .await?;
            if let [range] = mapped.as_slice() {
                let index = ranges
                    .iter()
                    .position(|expected| expected == range)
                    .ok_or("Partition topology changed while selecting workload keys.")?;
                if keys_by_range[index].len() < 2 {
                    keys_by_range[index].push(pk);
                }
            } else {
                return Err("A full workload partition key did not map to one feed range.".into());
            }
            if keys_by_range.iter().all(|keys| keys.len() == 2) {
                break;
            }
        }
        if keys_by_range.iter().any(|keys| keys.len() != 2) {
            return Err("Could not select two logical partition keys in every feed range.".into());
        }
        let keys = keys_by_range
            .into_iter()
            .enumerate()
            .flat_map(|(range, keys)| keys.into_iter().map(move |pk| (range, pk)))
            .enumerate()
            .map(|(index, (range, pk))| Key { index, range, pk })
            .collect();
        Ok(Self::from_keys(name, keys, ranges.len(), groups))
    }

    fn from_keys(name: String, keys: Vec<Key>, range_count: usize, groups: u32) -> Self {
        let mut workload = Self {
            name,
            keys,
            range_count,
            groups,
            expected: BTreeMap::new(),
        };
        for key in &workload.keys {
            for group in 0..groups {
                for doc in workload.documents(key, group) {
                    workload.expected.insert(doc.id.clone(), doc);
                }
            }
            let fence = workload.fence(key);
            workload.expected.insert(fence.id.clone(), fence);
        }
        workload
    }

    fn documents(&self, key: &Key, group: u32) -> Vec<Document> {
        let count = if group.is_multiple_of(4) { 3 } else { 1 };
        (0..count)
            .map(|member| Document {
                id: format!("{}-{}-{group}-{member}", self.name, key.index),
                pk: key.pk.clone(),
                run: self.name.clone(),
                group,
                member,
                fence: false,
            })
            .collect()
    }

    fn fence(&self, key: &Key) -> Document {
        Document {
            id: format!("{}-fence-{}", self.name, key.index),
            pk: key.pk.clone(),
            run: self.name.clone(),
            group: self.groups,
            member: 0,
            fence: true,
        }
    }

    async fn publish(&self, source: &ContainerClient, start: u32, end: u32) -> TestResult {
        futures::stream::iter(&self.keys)
            .map(|key| async move {
                for group in start..end {
                    let docs = self.documents(key, group);
                    if docs.len() > 1 {
                        let mut batch = TransactionalBatch::new(key.pk.clone());
                        for doc in &docs {
                            batch = batch.create_item(doc)?;
                        }
                        let response = source.execute_transactional_batch(batch, None).await?;
                        let status = response.status();
                        let statuses = response
                            .into_model()?
                            .results()
                            .iter()
                            .map(|r| r.status_code())
                            .collect::<Vec<_>>();
                        if status != StatusCode::Ok || statuses != [201, 201, 201] {
                            return Err(
                                format!("Backlog batch failed: {status}; {statuses:?}.").into()
                            );
                        }
                    } else {
                        let doc = &docs[0];
                        let response = if group.is_multiple_of(2) {
                            source
                                .create_item(key.pk.clone(), &doc.id, doc, None)
                                .await?
                        } else {
                            source
                                .upsert_item(key.pk.clone(), &doc.id, doc, None)
                                .await?
                        };
                        if response.status() != StatusCode::Created {
                            return Err(
                                format!("Append-only event {} was not created.", doc.id).into()
                            );
                        }
                    }
                }
                Ok::<_, Box<dyn std::error::Error>>(())
            })
            .buffer_unordered(4)
            .try_collect::<Vec<_>>()
            .await?;
        Ok(())
    }

    async fn publish_fences(&self, source: &ContainerClient) -> TestResult {
        for key in &self.keys {
            let doc = self.fence(key);
            source
                .create_item(key.pk.clone(), &doc.id, &doc, None)
                .await?;
        }
        Ok(())
    }

    async fn cleanup(&self, source: &ContainerClient, leases: &ContainerClient) -> TestResult {
        let mut failures = Vec::new();
        for key in &self.keys {
            let result = async {
                let items = source
                    .query_items::<Document>(
                        "SELECT * FROM c",
                        FeedScope::partition(key.pk.clone()),
                        None,
                    )
                    .await?
                    .try_collect::<Vec<_>>()
                    .await?;
                if items.iter().any(|doc| doc.run != self.name) {
                    return Err("Cleanup encountered an item outside this workload.".into());
                }
                for chunk in items.chunks(100) {
                    let mut batch = TransactionalBatch::new(key.pk.clone());
                    for doc in chunk {
                        batch = batch.delete_item(doc.id.clone(), None);
                    }
                    let response = source.execute_transactional_batch(batch, None).await?;
                    let status = response.status();
                    let body = response.into_model()?;
                    if status != StatusCode::Ok
                        || body
                            .results()
                            .iter()
                            .any(|result| result.status_code() != 204)
                    {
                        return Err(format!("Source cleanup failed: {status}.").into());
                    }
                }
                Ok::<_, Box<dyn std::error::Error>>(())
            }
            .await;
            if let Err(error) = result {
                failures.push(error.to_string());
            }
        }
        match records(leases, &self.name).await {
            Ok(items) => {
                for item in items {
                    match item["id"].as_str() {
                        Some(id) => {
                            if let Err(error) = leases.delete_item(id.to_owned(), id, None).await {
                                if !error.status().is_not_found() {
                                    failures.push(error.to_string());
                                }
                            }
                        }
                        None => failures.push("Lease metadata lacks an id.".into()),
                    }
                }
            }
            Err(error) => failures.push(error.to_string()),
        }
        if failures.is_empty() {
            Ok(())
        } else {
            Err(format!("Cleanup {}: {}", self.name, failures.join("; ")).into())
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Stage {
    BeforeApply,
    AfterApply,
}

struct HoldRule {
    worker: String,
    key: String,
    group: u32,
    stage: Stage,
    fired: bool,
}

struct Gate {
    worker: String,
    lease: String,
    items: Vec<String>,
    fences: Vec<String>,
    release: oneshot::Sender<()>,
}

#[derive(Default)]
struct Observed {
    arrivals: Vec<(String, String, Vec<String>)>,
    applied: Vec<(String, Document)>,
    seen: BTreeSet<String>,
    last_group: BTreeMap<String, u32>,
    replays: Vec<(String, String)>,
    active: BTreeSet<(String, String)>,
    completed_batches: usize,
    canceled_batches: usize,
    error: Option<String>,
}

struct Sink {
    workload: Arc<Workload>,
    state: Mutex<Observed>,
    rules: Mutex<Vec<HoldRule>>,
    gates: mpsc::UnboundedSender<Gate>,
}

struct CallbackGuard {
    sink: Arc<Sink>,
    identity: (String, String),
    completed: bool,
}

impl Drop for CallbackGuard {
    fn drop(&mut self) {
        let mut state = self.sink.state.lock().unwrap();
        state.active.remove(&self.identity);
        if self.completed {
            state.completed_batches += 1;
        } else {
            state.canceled_batches += 1;
        }
    }
}

impl Sink {
    fn new(
        workload: Arc<Workload>,
        rules: Vec<HoldRule>,
    ) -> (Arc<Self>, mpsc::UnboundedReceiver<Gate>) {
        let (gates, receiver) = mpsc::unbounded();
        (
            Arc::new(Self {
                workload,
                state: Mutex::new(Observed::default()),
                rules: Mutex::new(rules),
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

    fn apply(&self, worker: &str, docs: &[Document]) -> azure_data_cosmos::Result<()> {
        let mut state = self.state.lock().unwrap();
        for doc in docs {
            if self.workload.expected.get(&doc.id) != Some(doc) {
                drop(state);
                return Err(self.failure(format!("Unexpected backlog event: {doc:?}.")));
            }
            if state.seen.contains(&doc.id) {
                state.replays.push((worker.to_owned(), doc.id.clone()));
                continue;
            }
            if state
                .last_group
                .get(&doc.pk)
                .is_some_and(|group| *group > doc.group)
            {
                let previous = state.last_group[&doc.pk];
                drop(state);
                return Err(self.failure(format!(
                    "Out-of-order first delivery for {}: group {} after {previous}; id {}.",
                    doc.pk, doc.group, doc.id
                )));
            }
            state.last_group.insert(doc.pk.clone(), doc.group);
            state.seen.insert(doc.id.clone());
            state.applied.push((worker.to_owned(), doc.clone()));
        }
        Ok(())
    }

    async fn hold(
        &self,
        worker: &str,
        lease: &str,
        docs: &[Document],
        fence: bool,
    ) -> azure_data_cosmos::Result<()> {
        let (release, wait) = oneshot::channel();
        self.gates
            .unbounded_send(Gate {
                worker: worker.to_owned(),
                lease: lease.to_owned(),
                items: docs.iter().map(|doc| doc.id.clone()).collect(),
                fences: if fence {
                    docs.iter()
                        .filter(|doc| doc.fence)
                        .map(|doc| doc.id.clone())
                        .collect()
                } else {
                    Vec::new()
                },
                release,
            })
            .map_err(|_| self.failure("Backlog gate receiver closed."))?;
        wait.await
            .map_err(|_| self.failure("Backlog callback gate abandoned."))?;
        Ok(())
    }

    async fn callback(
        self: Arc<Self>,
        worker: String,
        context: ChangeFeedProcessorContext,
        changes: Vec<ChangeFeedItem<Value>>,
    ) -> azure_data_cosmos::Result<()> {
        let mut docs = Vec::new();
        for change in changes {
            if let Some(value) = change
                .current()
                .filter(|value| value["run"] == self.workload.name)
            {
                docs.push(
                    serde_json::from_value::<Document>(value.clone()).map_err(|error| {
                        self.failure(format!("Invalid backlog document: {error}"))
                    })?,
                );
            }
        }
        if docs.is_empty() {
            return Ok(());
        }
        let lease = context.lease_token().to_owned();
        let identity = (worker.clone(), lease.clone());
        {
            let mut state = self.state.lock().unwrap();
            if !state.active.insert(identity.clone()) {
                drop(state);
                return Err(self.failure(format!(
                    "Overlapping callbacks within worker/lease {identity:?}."
                )));
            }
            state.arrivals.push((
                worker.clone(),
                lease.clone(),
                docs.iter().map(|doc| doc.id.clone()).collect(),
            ));
        }
        let mut guard = CallbackGuard {
            sink: self.clone(),
            identity,
            completed: false,
        };
        let stage = {
            let mut rules = self.rules.lock().unwrap();
            rules
                .iter_mut()
                .find(|rule| {
                    !rule.fired
                        && rule.worker == worker
                        && docs
                            .iter()
                            .any(|doc| !doc.fence && doc.pk == rule.key && doc.group >= rule.group)
                })
                .map(|rule| {
                    rule.fired = true;
                    rule.stage
                })
        };
        if stage == Some(Stage::BeforeApply) {
            self.hold(&worker, &lease, &docs, false).await?;
        }
        if docs.iter().any(|doc| doc.fence) {
            self.hold(&worker, &lease, &docs, true).await?;
        }
        tokio::time::sleep(Duration::from_millis(40)).await;
        self.apply(&worker, &docs)?;
        if stage == Some(Stage::AfterApply) {
            self.hold(&worker, &lease, &docs, false).await?;
        }
        guard.completed = true;
        Ok(())
    }

    fn check(&self) -> TestResult {
        match &self.state.lock().unwrap().error {
            Some(error) => Err(error.clone().into()),
            None => Ok(()),
        }
    }
}

fn processor(
    source: &ContainerClient,
    leases: &ContainerClient,
    sink: &Arc<Sink>,
    worker: &str,
    hint: u32,
    resume_with_now: bool,
) -> TestResult<Processor> {
    let name = worker.to_owned();
    let handler_sink = sink.clone();
    let handler: Handler = Box::new(move |context, changes| {
        handler_sink
            .clone()
            .callback(name.clone(), context, changes)
            .boxed()
    });
    let builder = ChangeFeedProcessorBuilder::new(
        LatestVersion,
        source.clone(),
        leases.clone(),
        &sink.workload.name,
        worker,
    );
    let builder = if resume_with_now {
        builder.with_start_from_now()
    } else {
        builder.with_start_from_beginning()
    };
    Ok(builder
        .with_max_item_count(MaxItemCountHint::Limit(
            NonZeroU32::new(hint).ok_or("Invalid page hint.")?,
        ))
        .with_feed_poll_delay(INTERVAL)
        .handle_change_feed(handler)
        .build()?)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Progress {
    owner: Option<String>,
    incarnation: Option<String>,
    continuation: Option<String>,
    renewed_at: Option<i64>,
}

async fn progress(leases: &ContainerClient, id: &str) -> TestResult<Progress> {
    let item: Value = leases
        .read_item(id.to_owned(), id, None)
        .await?
        .into_model()?;
    Ok(Progress {
        owner: item["owner"].as_str().map(str::to_owned),
        incarnation: item["incarnation"].as_str().map(str::to_owned),
        continuation: item["continuation"].as_str().map(str::to_owned),
        renewed_at: item["renewed_at"].as_i64(),
    })
}

async fn gate(receiver: &mut mpsc::UnboundedReceiver<Gate>, sink: &Sink) -> TestResult<Gate> {
    tokio::time::timeout(CONTROL_WAIT, async {
        loop {
            sink.check()?;
            if let Ok(result) = tokio::time::timeout(INTERVAL, receiver.next()).await {
                return result.ok_or_else(|| "Callback gate stream ended.".into());
            }
        }
    })
    .await
    .map_err(|_| io::Error::other("Timed out waiting for the expected callback gate."))?
}

async fn wait_owned(
    leases: &ContainerClient,
    sink: &Sink,
    workers: &BTreeSet<String>,
    all_saved: bool,
) -> TestResult {
    let mut last = Vec::new();
    let result = tokio::time::timeout(CONTROL_WAIT, async {
        loop {
            sink.check()?;
            last = records(leases, &sink.workload.name)
                .await?
                .into_iter()
                .filter(|record| record.get("range").is_some())
                .collect();
            let owners: BTreeSet<_> = last
                .iter()
                .filter_map(|record| record["owner"].as_str().map(str::to_owned))
                .collect();
            if last.len() == sink.workload.range_count
                && last.iter().all(|record| {
                    record["owner"]
                        .as_str()
                        .is_some_and(|owner| workers.contains(owner))
                        && (!all_saved || record["continuation"].is_string())
                })
                && &owners == workers
            {
                return Ok::<_, Box<dyn std::error::Error>>(());
            }
            tokio::time::sleep(INTERVAL).await;
        }
    })
    .await;
    result.map_err(|_| {
        io::Error::other(format!(
            "Ownership did not converge to {workers:?}; leases: {last:?}."
        ))
    })?
}

async fn wait_canceled(gate: &Gate, sink: &Sink) -> TestResult {
    tokio::time::timeout(CONTROL_WAIT, async {
        while !gate.release.is_canceled() {
            sink.check()?;
            tokio::time::sleep(INTERVAL).await;
        }
        Ok::<_, Box<dyn std::error::Error>>(())
    })
    .await
    .map_err(|_| io::Error::other("Lost/stopped lease did not cancel its blocked callback."))?
}

async fn drain(
    leases: &ContainerClient,
    sink: &Sink,
    receiver: &mut mpsc::UnboundedReceiver<Gate>,
) -> TestResult {
    let expected_fences: BTreeSet<_> = sink
        .workload
        .keys
        .iter()
        .map(|key| sink.workload.fence(key).id)
        .collect();
    let mut durable = BTreeSet::new();
    let result = tokio::time::timeout(DRAIN_WAIT, async {
        while durable != expected_fences {
            let gate = gate(receiver, sink).await?;
            if gate.fences.is_empty() { return Err("Unexpected data hold while draining final fences.".into()); }
            let before = progress(leases, &gate.lease).await?;
            if before.owner.as_deref() != Some(gate.worker.as_str()) || before.continuation.is_none() {
                return Err(format!("Fence callback does not own a previously saved lease: {before:?}.").into());
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
            let held = progress(leases, &gate.lease).await?;
            if held.continuation != before.continuation || held.incarnation != before.incarnation {
                return Err("Fence progress changed before callback completion.".into());
            }
            gate.release.send(()).map_err(|_| io::Error::other("Fence callback was canceled before release."))?;
            tokio::time::timeout(CONTROL_WAIT, async {
                loop {
                    sink.check()?;
                    let after = progress(leases, &gate.lease).await?;
                    if after.owner != before.owner || after.incarnation != before.incarnation {
                        return Err("Ownership changed during final stable-worker drain.".into());
                    }
                    if after.continuation != before.continuation {
                        return Ok::<_, Box<dyn std::error::Error>>(());
                    }
                    tokio::time::sleep(INTERVAL).await;
                }
            }).await.map_err(|_| io::Error::other("Fence callback was not followed by a checkpoint."))??;
            durable.extend(gate.fences);
        }
        sink.check()?;
        let state = sink.state.lock().unwrap();
        let expected: BTreeSet<_> = sink.workload.expected.keys().cloned().collect();
        if state.seen != expected {
            return Err(format!("Incomplete backlog: missing {:?}.", expected.difference(&state.seen).collect::<Vec<_>>()).into());
        }
        eprintln!("Verified {} ranges, {} logical keys, {} distinct events, {} completed callbacks, {} replay deliveries, {} durable fences; per-key transaction-group ordering preserved.",
            sink.workload.range_count, sink.workload.keys.len(), expected.len(), state.completed_batches, state.replays.len(), durable.len());
        Ok::<_, Box<dyn std::error::Error>>(())
    }).await;
    result.map_err(|_| {
        io::Error::other(format!(
            "Timed out draining backlog; saved {} of {} fences.",
            durable.len(),
            expected_fences.len()
        ))
    })?
}

#[derive(Clone, Copy, Debug)]
enum Scenario {
    Drain,
    Restart,
    Scale,
}

async fn exercise(
    source: ContainerClient,
    leases: ContainerClient,
    scenario: Scenario,
) -> TestResult {
    let groups = if matches!(scenario, Scenario::Scale) {
        GROUPS * 2
    } else {
        GROUPS
    };
    let workload = Arc::new(Workload::discover(&source, groups).await?);
    let a = format!("{}-a", workload.name);
    let b = format!("{}-b", workload.name);
    let key = workload.keys[0].pk.clone();
    let rules = match scenario {
        Scenario::Drain => Vec::new(),
        Scenario::Restart => vec![HoldRule {
            worker: a.clone(),
            key,
            group: 4,
            stage: Stage::AfterApply,
            fired: false,
        }],
        Scenario::Scale => vec![
            HoldRule {
                worker: a.clone(),
                key: key.clone(),
                group: 0,
                stage: Stage::BeforeApply,
                fired: false,
            },
            HoldRule {
                worker: b.clone(),
                key,
                group: GROUPS,
                stage: Stage::BeforeApply,
                fired: false,
            },
        ],
    };
    let (sink, mut receiver) = Sink::new(workload.clone(), rules);
    let hint = match scenario {
        Scenario::Drain => 1,
        Scenario::Restart => 2,
        Scenario::Scale => 4,
    };
    let first = processor(&source, &leases, &sink, &a, hint, false)?;
    let second = processor(
        &source,
        &leases,
        &sink,
        &b,
        hint,
        matches!(scenario, Scenario::Restart),
    )?;
    let result = async {
        workload.publish(&source, 0, GROUPS).await?;
        first.start().await?;
        match scenario {
            Scenario::Drain => {}
            Scenario::Restart => {
                let held = gate(&mut receiver, &sink).await?;
                if held.worker != a || !held.fences.is_empty() { return Err("Unexpected restart hold.".into()); }
                wait_owned(&leases, &sink, &BTreeSet::from([a.clone()]), true).await?;
                let before = progress(&leases, &held.lease).await?;
                first.stop().await?;
                wait_canceled(&held, &sink).await?;
                if progress(&leases, &held.lease).await?.continuation != before.continuation {
                    return Err("Stopping advanced an unfinished callback's checkpoint.".into());
                }
                second.start().await?;
                workload.publish_fences(&source).await?;
                drain(&leases, &sink, &mut receiver).await?;
                let state = sink.state.lock().unwrap();
                for id in held.items {
                    if !state.replays.contains(&(b.clone(), id.clone())) {
                        return Err(format!("Restart skipped unacknowledged applied event {id}.").into());
                    }
                }
                return Ok(());
            }
            Scenario::Scale => {
                let held = gate(&mut receiver, &sink).await?;
                if held.worker != a || !held.fences.is_empty() { return Err("Unexpected scale-out hold.".into()); }
                wait_owned(&leases, &sink, &BTreeSet::from([a.clone()]), false).await?;
                let before = progress(&leases, &held.lease).await?;
                tokio::time::timeout(CONTROL_WAIT, async {
                    loop {
                        let current = progress(&leases, &held.lease).await?;
                        if current.incarnation != before.incarnation || current.continuation != before.continuation {
                            return Err("Blocked callback lost ownership or advanced its checkpoint before stealing.".into());
                        }
                        if current.renewed_at > before.renewed_at { return Ok::<_, Box<dyn std::error::Error>>(()); }
                        tokio::time::sleep(INTERVAL).await;
                    }
                }).await.map_err(|_| io::Error::other("Lease renewal stalled behind a blocked callback."))??;
                if !sink.state.lock().unwrap().applied.iter().any(|(worker, doc)| worker == &a
                    && workload.keys.iter().any(|key| key.range != 0 && key.pk == doc.pk)) {
                    return Err("Other ranges did not progress while one callback was blocked.".into());
                }
                second.start().await?;
                wait_owned(&leases, &sink, &BTreeSet::from([a.clone(), b.clone()]), false).await?;
                let stolen = progress(&leases, &held.lease).await?;
                if stolen.owner.as_deref() != Some(b.as_str()) || stolen.incarnation == before.incarnation {
                    return Err("Scale-out did not steal the blocked lease into a new acquisition.".into());
                }
                wait_canceled(&held, &sink).await?;
                workload.publish(&source, GROUPS, groups).await?;
                let departing = gate(&mut receiver, &sink).await?;
                if departing.worker != b || !departing.fences.is_empty() { return Err("Expected a blocked tail callback on the departing worker.".into()); }
                second.stop().await?;
                wait_canceled(&departing, &sink).await?;
                wait_owned(&leases, &sink, &BTreeSet::from([a.clone()]), true).await?;
            }
        }
        workload.publish_fences(&source).await?;
        drain(&leases, &sink, &mut receiver).await
    }.await;
    let mut failures = Vec::new();
    if let Err(error) = result {
        failures.push(error.to_string());
    }
    for worker in [&first, &second] {
        if let Err(error) = worker.stop().await {
            failures.push(format!("Stop failed: {error}"));
        }
    }
    if let Err(error) = workload.cleanup(&source, &leases).await {
        failures.push(error.to_string());
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{scenario:?} workload {}: {}",
            workload.name,
            failures.join("; ")
        )
        .into())
    }
}

#[tokio::test]
#[ignore = "Requires live opt-in and a /pk source spanning at least two actual feed ranges."]
async fn cfp_live_backlog_drain_and_order() -> TestResult {
    let (source, leases) = live_containers().await?;
    exercise(source, leases, Scenario::Drain).await
}

#[tokio::test]
#[ignore = "Requires live opt-in and a /pk source spanning at least two actual feed ranges."]
async fn cfp_live_backlog_restart_and_replay() -> TestResult {
    let (source, leases) = live_containers().await?;
    exercise(source, leases, Scenario::Restart).await
}

#[tokio::test]
#[ignore = "Requires live opt-in and a /pk source spanning at least two actual feed ranges."]
async fn cfp_live_backlog_scale_out_and_down() -> TestResult {
    let (source, leases) = live_containers().await?;
    exercise(source, leases, Scenario::Scale).await
}

#[cfg(feature = "__internal_in_memory_emulator")]
#[path = "backlog/local_tests.rs"]
mod local_tests;
