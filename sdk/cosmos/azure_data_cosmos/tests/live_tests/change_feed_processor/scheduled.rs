// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{live_client, records, TestResult};
use azure_core::http::StatusCode;
use azure_data_cosmos::{
    feed::FeedRange,
    models::{ChangeFeedItem, ChangeFeedOperationType},
    options::{ChangeFeedMode, MaxItemCountHint, ReadFeedRangesOptions},
    ChangeFeedCheckpointPolicy, ChangeFeedProcessorBuilder, ChangeFeedProcessorContext,
    ContainerClient, CosmosError, CosmosStatus,
};
use futures::{future::BoxFuture, stream, FutureExt, StreamExt, TryStreamExt};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    io,
    num::NonZeroU32,
    sync::{Arc, Mutex, OnceLock},
    time::Duration,
};
use tokio::{sync::Notify, time::Instant};
use uuid::Uuid;

const PARTITIONS: usize = 10;
const WORKERS: usize = 5;
const LEASES_PER_WORKER: usize = PARTITIONS / WORKERS;
const DOCUMENTS_PER_PARTITION: usize = 10;
const DOCUMENTS: usize = PARTITIONS * DOCUMENTS_PER_PARTITION;
const LIVE_TICK: Duration = Duration::from_secs(60);
const CONTROL_WAIT: Duration = Duration::from_secs(120);
const POLL_DELAY: Duration = Duration::from_millis(50);

static ACTIVE: Mutex<Option<Arc<Collector>>> = Mutex::new(None);
static SERIAL: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
    Create,
    Replace,
    Delete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Event {
    operation: Operation,
    revision: Option<u32>,
}

#[derive(Clone, Copy, Debug)]
enum Action {
    Create(u32),
    Upsert(u32, Operation),
    Delete,
}

#[derive(Clone, Copy, Debug)]
enum Selection {
    All,
    Even,
    Odd,
}

impl Selection {
    fn contains(self, index: usize) -> bool {
        match self {
            Self::All => true,
            Self::Even => index.is_multiple_of(2),
            Self::Odd => !index.is_multiple_of(2),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Step {
    minute: u32,
    selection: Selection,
    action: Action,
}

const SCHEDULE: [Step; 10] = [
    Step {
        minute: 0,
        selection: Selection::All,
        action: Action::Create(1),
    },
    Step {
        minute: 1,
        selection: Selection::All,
        action: Action::Upsert(2, Operation::Replace),
    },
    Step {
        minute: 2,
        selection: Selection::All,
        action: Action::Upsert(3, Operation::Replace),
    },
    Step {
        minute: 3,
        selection: Selection::Even,
        action: Action::Delete,
    },
    Step {
        minute: 4,
        selection: Selection::Even,
        action: Action::Upsert(4, Operation::Create),
    },
    Step {
        minute: 5,
        selection: Selection::All,
        action: Action::Upsert(5, Operation::Replace),
    },
    Step {
        minute: 6,
        selection: Selection::Odd,
        action: Action::Delete,
    },
    Step {
        minute: 7,
        selection: Selection::Odd,
        action: Action::Upsert(6, Operation::Create),
    },
    Step {
        minute: 8,
        selection: Selection::All,
        action: Action::Upsert(7, Operation::Replace),
    },
    Step {
        minute: 9,
        selection: Selection::All,
        action: Action::Delete,
    },
];

fn expected(index: usize) -> [Event; 8] {
    let create = |revision| Event {
        operation: Operation::Create,
        revision: Some(revision),
    };
    let replace = |revision| Event {
        operation: Operation::Replace,
        revision: Some(revision),
    };
    let delete = Event {
        operation: Operation::Delete,
        revision: None,
    };
    if index.is_multiple_of(2) {
        [
            create(1),
            replace(2),
            replace(3),
            delete,
            create(4),
            replace(5),
            replace(7),
            delete,
        ]
    } else {
        [
            create(1),
            replace(2),
            replace(3),
            replace(5),
            delete,
            create(6),
            replace(7),
            delete,
        ]
    }
}

struct DocumentSpec {
    id: String,
    pk: String,
    partition: usize,
}

impl DocumentSpec {
    fn value(&self, revision: u32, partition_key_field: &str) -> Value {
        let mut value = json!({"id":self.id, "revision":revision});
        value[partition_key_field] = json!(self.pk);
        value
    }
}

struct Plan {
    name: String,
    ranges: Vec<FeedRange>,
    inventory: Vec<FeedRange>,
    partition_key_field: String,
    documents: Vec<DocumentSpec>,
}

async fn physical_ranges(source: &ContainerClient) -> TestResult<Vec<FeedRange>> {
    let mut ranges = source
        .read_feed_ranges(Some(
            ReadFeedRangesOptions::default().with_force_refresh(true),
        ))
        .await?;
    ranges.sort_by(|left, right| left.min_inclusive().cmp(right.min_inclusive()));
    if ranges.len() < PARTITIONS {
        return Err(format!(
            "Expected at least {PARTITIONS} physical partitions, found {}.",
            ranges.len()
        )
        .into());
    }
    Ok(ranges)
}

impl Plan {
    async fn create(
        source: &ContainerClient,
        name: String,
        inventory: Vec<FeedRange>,
        assigned: &[Value],
    ) -> TestResult<Self> {
        let properties = source.read(None).await?.into_model()?;
        let partition_key_field = if properties.partition_key.paths() == ["/pk"] {
            "pk"
        } else if properties.partition_key.paths() == ["/partition_key"] {
            "partition_key"
        } else {
            return Err(
                "The scheduled test requires a /pk or /partition_key source container.".into(),
            );
        }
        .to_owned();
        let mut ranges: Vec<FeedRange> = assigned
            .iter()
            .map(|record| serde_json::from_value(record["range"].clone()))
            .collect::<Result<_, _>>()?;
        ranges.sort_by(|left, right| left.min_inclusive().cmp(right.min_inclusive()));
        if ranges.len() != PARTITIONS || ranges.iter().any(|range| !inventory.contains(range)) {
            return Err("The five workers must own exactly ten physical ranges.".into());
        }
        let mut keys = vec![None; PARTITIONS];
        for candidate in 0..100_000 {
            let key = format!("{name}-key-{candidate}");
            let mapped = source
                .feed_range_from_partition_key(key.clone(), None)
                .await?;
            if mapped.len() != 1 {
                return Err(format!(
                    "Full partition key {key} resolved to {} ranges.",
                    mapped.len()
                )
                .into());
            }
            let Some(partition) = ranges.iter().position(|range| range == &mapped[0]) else {
                continue;
            };
            keys[partition].get_or_insert(key);
            if keys.iter().all(Option::is_some) {
                break;
            }
        }
        let keys = keys
            .into_iter()
            .enumerate()
            .map(|(partition, key)| {
                key.ok_or_else(|| format!("No key found for physical partition {partition}."))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let documents = (0..DOCUMENTS)
            .map(|index| {
                let partition = index / DOCUMENTS_PER_PARTITION;
                DocumentSpec {
                    id: format!("{name}-doc-{index:03}"),
                    pk: keys[partition].clone(),
                    partition,
                }
            })
            .collect();
        let plan = Self {
            name,
            ranges,
            inventory,
            partition_key_field,
            documents,
        };
        plan.check_topology(source).await?;
        Ok(plan)
    }

    async fn check_topology(&self, source: &ContainerClient) -> TestResult {
        if physical_ranges(source).await? != self.inventory {
            return Err(
                "Physical partition coverage changed during the fixed-schedule test.".into(),
            );
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Observed {
    event: Event,
    lsn: i64,
    elapsed: Duration,
    lease: String,
}

struct State {
    started: Option<Instant>,
    leases: BTreeMap<String, usize>,
    owners: BTreeMap<String, usize>,
    observed: Vec<Vec<Observed>>,
    identities: BTreeMap<(usize, i64), Event>,
    deliveries: u64,
    duplicates: u64,
    callbacks: u64,
    foreign_changes: u64,
    worker_changes: [usize; WORKERS],
    failure: Option<String>,
}

struct Collector {
    name: String,
    plan: OnceLock<Plan>,
    state: Mutex<State>,
    failed: Notify,
}

impl Collector {
    fn new(name: String) -> Self {
        Self {
            state: Mutex::new(State {
                started: None,
                leases: BTreeMap::new(),
                owners: BTreeMap::new(),
                observed: vec![Vec::new(); DOCUMENTS],
                identities: BTreeMap::new(),
                deliveries: 0,
                duplicates: 0,
                callbacks: 0,
                foreign_changes: 0,
                worker_changes: [0; WORKERS],
                failure: None,
            }),
            name,
            plan: OnceLock::new(),
            failed: Notify::new(),
        }
    }

    fn failure(&self, message: impl Into<String>) -> CosmosError {
        let message = message.into();
        self.state
            .lock()
            .unwrap()
            .failure
            .get_or_insert(message.clone());
        self.failed.notify_one();
        CosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::InternalServerError))
            .with_message(message)
            .build()
    }

    fn check(&self) -> TestResult {
        match &self.state.lock().unwrap().failure {
            Some(message) => Err(message.clone().into()),
            None => Ok(()),
        }
    }

    async fn wait_failure(&self) -> TestResult {
        loop {
            self.check()?;
            self.failed.notified().await;
        }
    }

    fn begin(&self, plan: Plan, records: &[Value]) -> TestResult<Instant> {
        if plan.name != self.name {
            return Err("Ingestion plan belongs to another workload.".into());
        }
        let mut leases = BTreeMap::new();
        let mut owners = BTreeMap::new();
        let mut partitions = BTreeSet::new();
        for record in records
            .iter()
            .filter(|record| record.get("range").is_some())
        {
            let id = record["id"].as_str().ok_or("Lease record has no id.")?;
            let range: FeedRange = serde_json::from_value(record["range"].clone())?;
            let partition = plan
                .ranges
                .iter()
                .position(|expected| expected == &range)
                .ok_or("Lease range does not match the physical partition inventory.")?;
            if !partitions.insert(partition) || leases.insert(id.to_owned(), partition).is_some() {
                return Err("Duplicate lease coverage in the ready workload.".into());
            }
            let owner = (0..WORKERS)
                .find(|index| record["owner"] == worker_name(&self.name, *index))
                .ok_or("Ready lease belongs to an unexpected worker.")?;
            owners.insert(id.to_owned(), owner);
            eprintln!("AVAD assignment: worker={owner} physical_range={range} lease={id}");
        }
        if partitions.len() != PARTITIONS {
            return Err("Not all ten physical partitions have a ready lease.".into());
        }
        self.plan
            .set(plan)
            .map_err(|_| "Ingestion plan was already initialized.")?;
        let started = Instant::now();
        let mut state = self.state.lock().unwrap();
        state.leases = leases;
        state.owners = owners;
        state.started = Some(started);
        Ok(started)
    }

    fn accept(
        &self,
        worker: usize,
        lease: &str,
        changes: &[ChangeFeedItem<Value>],
    ) -> Result<(), String> {
        let mut state = self.state.lock().unwrap();
        state.callbacks += 1;
        for change in changes {
            let id = change
                .current()
                .and_then(|item| item["id"].as_str())
                .or_else(|| change.metadata().and_then(|metadata| metadata.id()))
                .or_else(|| change.previous().and_then(|item| item["id"].as_str()))
                .ok_or("AVAD change has no document identity.")?;
            if !id.starts_with(&format!("{}-doc-", self.name)) {
                state.foreign_changes += 1;
                continue;
            }
            let started = state
                .started
                .ok_or("Test changes arrived before ingestion readiness.")?;
            let partition = *state
                .leases
                .get(lease)
                .ok_or("Unexpected callback lease.")?;
            if state.owners.get(lease) != Some(&worker) {
                return Err(format!(
                    "Worker {worker} delivered from a lease assigned to another worker."
                ));
            }
            let plan = self
                .plan
                .get()
                .ok_or("Ingestion plan was not initialized.")?;
            let metadata = change.metadata().ok_or("AVAD change has no metadata.")?;
            let index = plan
                .documents
                .iter()
                .position(|document| document.id == id)
                .ok_or_else(|| format!("Unexpected document {id}."))?;
            let document = &plan.documents[index];
            if document.partition != partition {
                return Err(format!(
                    "Document {id} was drained from the wrong physical range."
                ));
            }
            for identity in [
                change.current().and_then(|item| item["id"].as_str()),
                change.previous().and_then(|item| item["id"].as_str()),
                metadata.id(),
            ]
            .into_iter()
            .flatten()
            {
                if identity != id {
                    return Err(format!("Conflicting envelope identities for {id}."));
                }
            }
            let operation = match change.operation_type() {
                Some(ChangeFeedOperationType::Create) => Operation::Create,
                Some(ChangeFeedOperationType::Replace) => Operation::Replace,
                Some(ChangeFeedOperationType::Delete) => Operation::Delete,
                other => return Err(format!("Unexpected AVAD operation for {id}: {other:?}.")),
            };
            let revision = if operation == Operation::Delete {
                if let Some(pk) = metadata.partition_key() {
                    let mut expected = json!({});
                    expected[&plan.partition_key_field] = json!(document.pk);
                    if pk != &json!([document.pk]) && pk != &expected {
                        return Err(format!(
                            "Delete {id} contains an unexpected partition key: {pk}."
                        ));
                    }
                }
                None
            } else {
                let current = change
                    .current()
                    .ok_or("Non-delete change has no current image.")?;
                let revision = current["revision"]
                    .as_u64()
                    .and_then(|revision| u32::try_from(revision).ok());
                if current[&plan.partition_key_field].as_str() != Some(document.pk.as_str())
                    || revision.is_none()
                {
                    return Err(format!("Invalid current image for {id}: {current:?}."));
                }
                revision
            };
            let lsn = metadata.lsn().ok_or("AVAD change has no LSN.")?.value();
            let event = Event {
                operation,
                revision,
            };
            state.deliveries += 1;
            if let Some(previous) = state.identities.get(&(index, lsn)) {
                if previous != &event {
                    return Err(format!("Replay changed the event for {id} at LSN {lsn}."));
                }
                state.duplicates += 1;
                continue;
            }
            let position = state.observed[index].len();
            let sequence = expected(index);
            if sequence.get(position) != Some(&event) {
                return Err(format!(
                    "Unexpected event for {id} at position {position}: {event:?}; expected {:?}.",
                    sequence.get(position),
                ));
            }
            if state.observed[index]
                .last()
                .is_some_and(|previous| previous.lsn >= lsn)
            {
                return Err(format!(
                    "Non-increasing first-delivery LSN for {id}: {lsn}."
                ));
            }
            let elapsed = started.elapsed();
            state.identities.insert((index, lsn), event);
            state.worker_changes[worker] += 1;
            state.observed[index].push(Observed {
                event,
                lsn,
                elapsed,
                lease: lease.to_owned(),
            });
            eprintln!("AVAD drain: elapsed_ms={} worker={worker} partition={partition} id={id} event={event:?} lsn={lsn} lease={lease}", elapsed.as_millis());
        }
        Ok(())
    }

    fn verify(&self) -> TestResult<Summary> {
        self.check()?;
        let plan = self
            .plan
            .get()
            .ok_or("Ingestion plan was never initialized.")?;
        let state = self.state.lock().unwrap();
        let mut counts = [0; PARTITIONS];
        let mut operations = [0; 3];
        for (index, events) in state.observed.iter().enumerate() {
            let actual: Vec<_> = events.iter().map(|event| event.event).collect();
            if actual != expected(index) {
                let last = events
                    .last()
                    .map(|event| (&event.lease, event.elapsed, event.lsn));
                return Err(format!(
                    "Incomplete drain for {}: {actual:?}; expected {:?}; last observation {last:?}.",
                    plan.documents[index].id, expected(index),
                ).into());
            }
            counts[plan.documents[index].partition] += events.len();
            for event in events {
                operations[match event.event.operation {
                    Operation::Create => 0,
                    Operation::Replace => 1,
                    Operation::Delete => 2,
                }] += 1;
            }
        }
        if counts != [80; PARTITIONS]
            || operations != [200, 400, 200]
            || state.worker_changes != [160; WORKERS]
        {
            return Err(format!(
                "Unexpected drained totals: partitions={counts:?}, operations={operations:?}."
            )
            .into());
        }
        Ok(Summary {
            partitions: counts,
            operations,
            duplicates: state.duplicates,
            callbacks: state.callbacks,
            deliveries: state.deliveries,
            elapsed: state.started.ok_or("Collection never started.")?.elapsed(),
            workers: state.worker_changes,
            foreign_changes: state.foreign_changes,
        })
    }
}

#[derive(Debug)]
struct Summary {
    partitions: [usize; PARTITIONS],
    operations: [usize; 3],
    duplicates: u64,
    callbacks: u64,
    deliveries: u64,
    elapsed: Duration,
    workers: [usize; WORKERS],
    foreign_changes: u64,
}

async fn on_changes(
    worker: usize,
    context: ChangeFeedProcessorContext,
    changes: Vec<ChangeFeedItem<Value>>,
) -> azure_data_cosmos::Result<()> {
    let active = ACTIVE.lock().unwrap().clone().ok_or_else(|| {
        CosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::InternalServerError))
            .with_message("Scheduled-test collector is not active.")
            .build()
    })?;
    active
        .accept(worker, context.lease_token(), &changes)
        .map_err(|message| active.failure(message))
}

type Handler = fn(
    ChangeFeedProcessorContext,
    Vec<ChangeFeedItem<Value>>,
) -> BoxFuture<'static, azure_data_cosmos::Result<()>>;

fn handler<const WORKER: usize>(
    context: ChangeFeedProcessorContext,
    changes: Vec<ChangeFeedItem<Value>>,
) -> BoxFuture<'static, azure_data_cosmos::Result<()>> {
    on_changes(WORKER, context, changes).boxed()
}

fn worker_name(name: &str, worker: usize) -> String {
    format!("{name}-worker-{worker}")
}

async fn on_error(context: Option<ChangeFeedProcessorContext>, error: CosmosError) {
    let message = format!(
        "CFP failure for {:?}: {error}",
        context.map(|context| context.lease_token().to_owned())
    );
    eprintln!("{message}");
    if let Some(active) = ACTIVE.lock().unwrap().clone() {
        active.failure(message);
    }
}

async fn ready(leases: &ContainerClient, collector: &Collector) -> TestResult<Vec<Value>> {
    tokio::time::timeout(CONTROL_WAIT, async {
        loop {
            collector.check()?;
            let records = records(leases, &collector.name).await?;
            let ranges: Vec<_> = records
                .into_iter()
                .filter(|record| {
                    record.get("range").is_some() && record["owner"].as_str().is_some()
                })
                .collect();
            let owned = |worker| {
                ranges
                    .iter()
                    .filter(|record| record["owner"] == worker_name(&collector.name, worker))
                    .count()
            };
            if ranges.len() == PARTITIONS
                && (0..WORKERS).all(|worker| owned(worker) == LEASES_PER_WORKER)
                && ranges.iter().all(|record| {
                    record["continuation"]
                        .as_str()
                        .is_some_and(|token| !token.is_empty())
                })
            {
                return Ok(ranges);
            }
            tokio::time::sleep(POLL_DELAY).await;
        }
    })
    .await
    .map_err(|_| {
        io::Error::other("CFP did not pin Now on all ten ranges; check AVAD account eligibility.")
    })?
}

async fn write(
    source: &ContainerClient,
    document: &DocumentSpec,
    action: Action,
    partition_key_field: &str,
) -> TestResult {
    let (actual, expected) = match action {
        Action::Create(revision) => (
            source
                .create_item(
                    document.pk.clone(),
                    &document.id,
                    document.value(revision, partition_key_field),
                    None,
                )
                .await?
                .status(),
            StatusCode::Created,
        ),
        Action::Upsert(revision, operation) => (
            source
                .upsert_item(
                    document.pk.clone(),
                    &document.id,
                    document.value(revision, partition_key_field),
                    None,
                )
                .await?
                .status(),
            if operation == Operation::Create {
                StatusCode::Created
            } else {
                StatusCode::Ok
            },
        ),
        Action::Delete => (
            source
                .delete_item(document.pk.clone(), &document.id, None)
                .await?
                .status(),
            StatusCode::NoContent,
        ),
    };
    if actual != expected {
        return Err(format!(
            "Write {:?} for {} returned {actual}, expected {expected}.",
            action, document.id
        )
        .into());
    }
    Ok(())
}

async fn ingest(
    source: &ContainerClient,
    collector: &Collector,
    started: Instant,
    tick: Duration,
) -> TestResult {
    let plan = collector
        .plan
        .get()
        .ok_or("Ingestion plan was not initialized.")?;
    for step in SCHEDULE {
        tokio::time::sleep_until(started + tick * step.minute).await;
        collector.check()?;
        let deadline = started + tick * (step.minute + 1);
        let phase = async {
            plan.check_topology(source).await?;
            stream::iter(
                plan.documents
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| step.selection.contains(*index)),
            )
            .map(|(_, document)| write(source, document, step.action, &plan.partition_key_field))
            .buffer_unordered(PARTITIONS)
            .try_collect::<Vec<_>>()
            .await?;
            Ok::<_, Box<dyn std::error::Error>>(())
        };
        tokio::time::timeout_at(deadline, phase).await.map_err(|_| io::Error::other(format!(
            "Minute {} writes missed their fixed deadline; canceled requests may still complete. Run {} requires cleanup.",
            step.minute, collector.name,
        )))??;
        eprintln!(
            "AVAD ingestion: minute={} action={:?} elapsed_ms={}",
            step.minute,
            step.action,
            started.elapsed().as_millis()
        );
    }
    tokio::time::sleep_until(started + tick * 10).await;
    collector.verify()?;
    plan.check_topology(source).await?;
    Ok(())
}

async fn cleanup(
    source: &ContainerClient,
    leases: &ContainerClient,
    collector: &Collector,
) -> TestResult {
    let mut failures = Vec::new();
    for document in collector
        .plan
        .get()
        .into_iter()
        .flat_map(|plan| &plan.documents)
    {
        if let Err(error) = source
            .delete_item(document.pk.clone(), &document.id, None)
            .await
        {
            if !error.status().is_not_found() {
                failures.push(format!("item {}: {error}", document.id));
            }
        }
    }
    match records(leases, &collector.name).await {
        Ok(records) => {
            for record in records {
                let Some(id) = record["id"].as_str() else {
                    failures.push("lease record has no id".into());
                    continue;
                };
                if let Err(error) = leases.delete_item(id.to_owned(), id, None).await {
                    if !error.status().is_not_found() {
                        failures.push(format!("lease {id}: {error}"));
                    }
                }
            }
        }
        Err(error) => failures.push(format!("lease inventory: {error}")),
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("; ").into())
    }
}

async fn run_case(
    source: ContainerClient,
    leases: ContainerClient,
    tick: Duration,
) -> TestResult<Summary> {
    let _serial = SERIAL.lock().await;
    let inventory = physical_ranges(&source).await?;
    let name = format!("cfp-fixed-{}", Uuid::new_v4().simple());
    let collector = Arc::new(Collector::new(name));
    let handlers: [Handler; WORKERS] = [
        handler::<0>,
        handler::<1>,
        handler::<2>,
        handler::<3>,
        handler::<4>,
    ];
    let mut processors = Vec::new();
    for (worker, handler) in handlers.into_iter().enumerate() {
        processors.push(
            ChangeFeedProcessorBuilder::new(
                ChangeFeedMode::AllVersionsAndDeletes,
                source.clone(),
                leases.clone(),
                &collector.name,
                worker_name(&collector.name, worker),
            )
            .with_start_from_now()
            .with_checkpoint_policy(ChangeFeedCheckpointPolicy::EveryBatch)
            .with_max_concurrency(NonZeroU32::MIN)
            .with_min_scale_count(LEASES_PER_WORKER as u32)
            .with_max_scale_count(LEASES_PER_WORKER as u32)
            .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(10).unwrap()))
            .with_feed_poll_delay(POLL_DELAY)
            .handle_error(on_error)
            .handle_change_feed(handler)
            .build()?,
        );
    }
    *ACTIVE.lock().unwrap() = Some(collector.clone());
    eprintln!("AVAD fixed schedule: run={} workers={WORKERS} documents={DOCUMENTS} selected_partitions={PARTITIONS} total_partitions={} duration_secs={}", collector.name, inventory.len(), (tick * 10).as_secs());
    let result: TestResult = async {
        for processor in &processors {
            tokio::time::timeout(CONTROL_WAIT, processor.start()).await??;
        }
        let ready = ready(&leases, &collector).await?;
        let plan = Plan::create(&source, collector.name.clone(), inventory, &ready).await?;
        let started = collector.begin(plan, &ready)?;
        eprintln!("AVAD ready: five local workers each own two primed physical ranges; ingestion starts now.");
        tokio::select! {
            result = ingest(&source, &collector, started, tick) => result,
            failure = collector.wait_failure() => failure,
        }
    }
    .await;
    let mut failures = Vec::new();
    let succeeded = result.is_ok();
    let stopping = futures::future::join_all(processors.iter().map(|processor| async {
        if succeeded {
            tokio::time::timeout(CONTROL_WAIT, processor.stop()).await
        } else {
            tokio::time::timeout(CONTROL_WAIT, processor.abort()).await
        }
    }))
    .await;
    if let Err(error) = result {
        failures.push(error.to_string());
    }
    let mut stopped = true;
    for (worker, result) in stopping.into_iter().enumerate() {
        match result {
            Ok(Ok(())) => {}
            Ok(Err(error)) => {
                failures.push(format!("worker {worker} shutdown: {error}"));
                stopped = false;
            }
            Err(error) => {
                failures.push(format!("worker {worker} shutdown timeout: {error}"));
                stopped = false;
            }
        }
    }
    drop(processors);
    *ACTIVE.lock().unwrap() = None;
    let summary = collector.verify();
    if let Err(error) = &summary {
        failures.push(error.to_string());
    }
    if stopped {
        if let Err(error) =
            tokio::time::timeout(CONTROL_WAIT, cleanup(&source, &leases, &collector))
                .await
                .map_err(|error| error.to_string())
                .and_then(|result| result.map_err(|error| error.to_string()))
        {
            failures.push(format!("cleanup: {error}"));
        }
    } else {
        failures.push(format!(
            "Skipped cleanup because shutdown was not confirmed; inspect run {}.",
            collector.name
        ));
    }
    if !failures.is_empty() {
        return Err(failures.join("; ").into());
    }
    let summary = summary?;
    eprintln!("AVAD drain verified: partitions={:?} create_replace_delete={:?} callbacks={} deliveries={} identical_replays={} elapsed_ms={} workers={:?} foreign_changes={}",
        summary.partitions, summary.operations, summary.callbacks, summary.deliveries,
        summary.duplicates, summary.elapsed.as_millis(), summary.workers, summary.foreign_changes);
    Ok(summary)
}

#[tokio::test]
#[ignore = "Requires explicit live opt-in, an AVAD-enabled source with at least ten physical partitions, isolated lease storage, and ten minutes plus setup/cleanup."]
async fn cfp_live_avad_fixed_schedule_ten_minutes() -> TestResult {
    let (client, lease_database) = live_client().await?;
    let source_database =
        optional_environment("COSMOS_SCHEDULE_LIVE_SOURCE_DATABASE", &lease_database)?;
    let source_container = optional_environment("COSMOS_SCHEDULE_LIVE_SOURCE_CONTAINER", "source")?;
    let source = client
        .database_client(source_database)
        .container_client(source_container, None)
        .await?;
    let leases = client
        .database_client(lease_database)
        .container_client("leases", None)
        .await?;
    run_case(source, leases, LIVE_TICK).await?;
    Ok(())
}

fn optional_environment(name: &str, default: &str) -> TestResult<String> {
    match std::env::var(name) {
        Ok(value) => Ok(value),
        Err(std::env::VarError::NotPresent) => Ok(default.to_owned()),
        Err(error) => Err(error.into()),
    }
}

#[cfg(feature = "__internal_in_memory_emulator")]
#[path = "scheduled/local_tests.rs"]
mod local_tests;
