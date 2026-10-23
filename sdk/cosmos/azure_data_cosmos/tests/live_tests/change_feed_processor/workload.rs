// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{live_containers, records, TestResult};
use azure_core::http::StatusCode;
use azure_data_cosmos::{
    models::{ChangeFeedItem, ChangeFeedOperationType},
    options::{ChangeFeedMode, ChangeFeedOptions, ChangeFeedStartFrom, MaxItemCountHint},
    AllVersionsAndDeletes, ChangeFeedProcessorBuilder, ChangeFeedProcessorContext,
    ChangeFeedProcessorMode, ContainerClient, CosmosError, CosmosStatus, FeedScope, LatestVersion,
    TransactionalBatch,
};
use futures::{
    channel::{mpsc, oneshot},
    StreamExt,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::BTreeSet, io, num::NonZeroU32, sync::Arc, time::Duration};
use time::OffsetDateTime;
use uuid::Uuid;

const WAIT: Duration = Duration::from_secs(120);
const INTERVAL: Duration = Duration::from_millis(100);
const ITEM_NAMES: [&str; 10] = [
    "before-start",
    "after-time",
    "clock",
    "created",
    "upserted",
    "deleted",
    "batch-deleted",
    "batch-created",
    "batch-upserted",
    "fence",
];

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Document {
    id: String,
    pk: Option<String>,
    revision: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Semantics {
    Latest,
    AllVersions,
}

#[derive(Clone, Copy)]
enum Start {
    Now,
    Beginning,
    AtTime,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Operation {
    Version,
    Create,
    Replace,
    Delete,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Event {
    id: String,
    revision: Option<u32>,
    operation: Operation,
}

struct Workload {
    name: String,
    semantics: Semantics,
}

impl Workload {
    fn new(semantics: Semantics) -> Self {
        Self {
            name: format!("cfp-mixed-{}", Uuid::new_v4().simple()),
            semantics,
        }
    }

    fn id(&self, name: &str) -> String {
        format!("{}-{name}", self.name)
    }

    fn document(&self, name: &str, revision: u32) -> Document {
        Document {
            id: self.id(name),
            pk: Some(self.name.clone()),
            revision: Some(revision),
        }
    }

    fn event(&self, name: &str, revision: u32, operation: Operation) -> Event {
        Event {
            id: self.id(name),
            revision: (operation != Operation::Delete).then_some(revision),
            operation: if self.semantics == Semantics::Latest {
                Operation::Version
            } else {
                operation
            },
        }
    }

    fn expected(&self, initial: &[Event]) -> BTreeSet<Event> {
        let mut events: BTreeSet<_> = initial.iter().cloned().collect();
        for name in [
            "created",
            "upserted",
            "deleted",
            "batch-deleted",
            "batch-created",
            "batch-upserted",
            "fence",
        ] {
            events.insert(self.event(name, 1, Operation::Create));
        }
        for name in ["created", "upserted"] {
            events.insert(self.event(name, 2, Operation::Replace));
        }
        events.insert(self.event("created", 3, Operation::Replace));
        if self.semantics == Semantics::AllVersions {
            for name in ["deleted", "batch-deleted"] {
                events.insert(self.event(name, 0, Operation::Delete));
            }
        }
        events
    }

    fn observe(&self, change: &ChangeFeedItem<Document>) -> TestResult<Option<Event>> {
        let id = item_id(change).ok_or("Change feed envelope has no item identity.")?;
        if !id.starts_with(&format!("{}-", self.name)) {
            return Ok(None);
        }
        let operation = match (self.semantics, change.operation_type()) {
            (Semantics::Latest, Some(ChangeFeedOperationType::Delete)) => {
                return Err(
                    format!("Latest-version feed unexpectedly delivered delete {id}.").into(),
                );
            }
            (Semantics::Latest, _) => Operation::Version,
            (Semantics::AllVersions, Some(ChangeFeedOperationType::Create)) => Operation::Create,
            (Semantics::AllVersions, Some(ChangeFeedOperationType::Replace)) => Operation::Replace,
            (Semantics::AllVersions, Some(ChangeFeedOperationType::Delete)) => Operation::Delete,
            (Semantics::AllVersions, other) => {
                return Err(format!(
                    "Full-fidelity change {id} has invalid operation metadata: {other:?}."
                )
                .into());
            }
        };
        let revision = if operation == Operation::Delete {
            if let Some(pk) = change
                .metadata()
                .and_then(|metadata| metadata.partition_key())
            {
                if pk != &serde_json::json!([self.name]) {
                    return Err(
                        format!("Delete {id} has an unexpected partition key: {pk}.").into(),
                    );
                }
            }
            None
        } else {
            let current = change
                .current()
                .ok_or("Non-delete change has no current document.")?;
            if current.pk.as_deref() != Some(self.name.as_str()) || current.revision.is_none() {
                return Err(format!("Invalid workload document in callback: {current:?}.").into());
            }
            current.revision
        };
        Ok(Some(Event {
            id: id.to_owned(),
            revision,
            operation,
        }))
    }

    async fn cleanup(&self, source: &ContainerClient, leases: &ContainerClient) -> TestResult {
        let mut failures = Vec::new();
        for name in ITEM_NAMES {
            if let Err(error) = source
                .delete_item(self.name.clone(), &self.id(name), None)
                .await
            {
                if !error.status().is_not_found() {
                    failures.push(format!("source item {name}: {error}"));
                }
            }
        }
        match records(leases, &self.name).await {
            Ok(records) => {
                for record in records {
                    let Some(id) = record["id"].as_str() else {
                        failures.push("lease metadata has no id".into());
                        continue;
                    };
                    if let Err(error) = leases.delete_item(id.to_owned(), id, None).await {
                        if !error.status().is_not_found() {
                            failures.push(format!("lease metadata {id}: {error}"));
                        }
                    }
                }
            }
            Err(error) => failures.push(format!("listing lease metadata: {error}")),
        }
        if failures.is_empty() {
            Ok(())
        } else {
            Err(format!("Cleanup of {} failed: {}", self.name, failures.join("; ")).into())
        }
    }
}

fn item_id(change: &ChangeFeedItem<Document>) -> Option<&str> {
    change
        .current()
        .map(|doc| doc.id.as_str())
        .or_else(|| change.metadata().and_then(|metadata| metadata.id()))
        .or_else(|| change.previous().map(|doc| doc.id.as_str()))
}

fn combine(result: TestResult, cleanup: TestResult) -> TestResult {
    match (result, cleanup) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(error), Ok(())) | (Ok(()), Err(error)) => Err(error),
        (Err(error), Err(cleanup)) => {
            Err(format!("{error}; cleanup also failed: {cleanup}").into())
        }
    }
}

async fn validate_source(source: &ContainerClient) -> TestResult {
    let properties = source.read(None).await?.into_model()?;
    if properties.partition_key.paths() != ["/pk"] {
        return Err(
            "Mixed workload tests require the source container to be partitioned by /pk.".into(),
        );
    }
    Ok(())
}

async fn service_timestamp(
    source: &ContainerClient,
    workload: &Workload,
    name: &str,
) -> TestResult<i64> {
    let item: Value = source
        .read_item(workload.name.clone(), &workload.id(name), None)
        .await?
        .into_model()?;
    item["_ts"]
        .as_i64()
        .ok_or_else(|| format!("Source item {name} has no service timestamp.").into())
}

async fn seed_source(
    source: &ContainerClient,
    workload: &Workload,
    start: Start,
) -> TestResult<(Option<OffsetDateTime>, Vec<Event>)> {
    source
        .create_item(
            workload.name.clone(),
            &workload.id("before-start"),
            workload.document("before-start", 1),
            None,
        )
        .await?;
    match start {
        Start::Now => Ok((None, Vec::new())),
        Start::Beginning => Ok((
            None,
            vec![workload.event("before-start", 1, Operation::Create)],
        )),
        Start::AtTime => {
            let boundary = service_timestamp(source, workload, "before-start").await? + 1;
            tokio::time::timeout(WAIT, async {
                loop {
                    source
                        .upsert_item(
                            workload.name.clone(),
                            &workload.id("clock"),
                            workload.document("clock", 1),
                            None,
                        )
                        .await?;
                    if service_timestamp(source, workload, "clock").await? > boundary {
                        break;
                    }
                    tokio::time::sleep(INTERVAL).await;
                }
                Ok::<_, Box<dyn std::error::Error>>(())
            })
            .await
            .map_err(|_| {
                io::Error::other("Service clock did not advance beyond the start-time boundary.")
            })??;
            source
                .delete_item(workload.name.clone(), &workload.id("clock"), None)
                .await?;
            source
                .create_item(
                    workload.name.clone(),
                    &workload.id("after-time"),
                    workload.document("after-time", 1),
                    None,
                )
                .await?;
            if service_timestamp(source, workload, "after-time").await? <= boundary {
                return Err(
                    "After-time seed did not fall strictly after the service-time boundary.".into(),
                );
            }
            Ok((
                Some(OffsetDateTime::from_unix_timestamp(boundary)?),
                vec![workload.event("after-time", 1, Operation::Create)],
            ))
        }
    }
}

struct CallbackBatch {
    lease: String,
    changes: Vec<ChangeFeedItem<Document>>,
}

struct Observations {
    expected: BTreeSet<Event>,
    seen: BTreeSet<Event>,
    fence_lease: Option<String>,
}

impl Observations {
    async fn collect_until(
        &mut self,
        workload: &Workload,
        receiver: &mut mpsc::UnboundedReceiver<CallbackBatch>,
        target: &BTreeSet<Event>,
    ) -> TestResult {
        let result = tokio::time::timeout(WAIT, async {
            while !target.is_subset(&self.seen) {
                let batch = receiver
                    .next()
                    .await
                    .ok_or("Processor callback stream closed before expected changes arrived.")?;
                for change in batch.changes {
                    if let Some(event) = workload.observe(&change)? {
                        if !self.expected.contains(&event) {
                            return Err(format!(
                                "Unexpected callback change: {event:?}; expected: {:?}",
                                self.expected
                            )
                            .into());
                        }
                        if event.id == workload.id("fence") {
                            self.fence_lease = Some(batch.lease.clone());
                        }
                        self.seen.insert(event);
                    }
                }
            }
            Ok::<_, Box<dyn std::error::Error>>(())
        })
        .await;
        match result {
            Ok(result) => result,
            Err(_) => Err(format!(
                "Timed out waiting for callback changes in {}. Missing: {:?}; observed: {:?}.",
                workload.name,
                target.difference(&self.seen).collect::<Vec<_>>(),
                self.seen,
            )
            .into()),
        }
    }
}

async fn wait_ready(leases: &ContainerClient, workload: &Workload) -> TestResult {
    tokio::time::timeout(WAIT, async {
        loop {
            let records = records(leases, &workload.name).await?;
            let planned = records.iter().find(|record| record["id"].as_str().is_some_and(|id| id.ends_with(".plan")))
                .and_then(|record| record["ranges"].as_array()).map(Vec::len);
            let ranges: Vec<_> = records.iter().filter(|record| record.get("range").is_some()).collect();
            if planned.is_some_and(|count| count > 0 && count == ranges.len())
                && ranges.iter().all(|record| record["owner"] == workload.name && record["continuation"].as_str().is_some_and(|token| !token.is_empty())) {
                return Ok::<_, Box<dyn std::error::Error>>(());
            }
            tokio::time::sleep(INTERVAL).await;
        }
    }).await.map_err(|_| io::Error::other(format!(
        "Processor {} did not acquire and save initial progress for every planned range. Check feed-mode eligibility and service errors.",
        workload.name,
    )))?
}

async fn checkpoint(leases: &ContainerClient, lease: &str) -> TestResult<String> {
    let record: Value = leases
        .read_item(lease.to_owned(), lease, None)
        .await?
        .into_model()?;
    record["continuation"]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| format!("Lease {lease} has no durable continuation.").into())
}

fn callback_error(message: &'static str) -> CosmosError {
    CosmosError::builder()
        .with_status(CosmosStatus::new(StatusCode::ServiceUnavailable))
        .with_message(message)
        .build()
}

async fn exercise<M: ChangeFeedProcessorMode>(
    source: &ContainerClient,
    leases: &ContainerClient,
    workload: &Workload,
    builder: ChangeFeedProcessorBuilder<M>,
    page_size: u32,
    initial: &[Event],
) -> TestResult {
    let (sender, mut receiver) = mpsc::unbounded();
    let (release, wait) = oneshot::channel::<()>();
    let gate = Arc::new(async_lock::Mutex::new(Some(wait)));
    let prefix = format!("{}-", workload.name);
    let fence_id = workload.id("fence");
    let processor = builder
        .with_max_item_count(MaxItemCountHint::Limit(
            NonZeroU32::new(page_size).ok_or("Page hint must be positive.")?,
        ))
        .with_feed_poll_delay(Duration::from_millis(100))
        .handle_change_feed(
            move |context: ChangeFeedProcessorContext,
                  mut changes: Vec<ChangeFeedItem<Document>>| {
                let sender = sender.clone();
                let gate = gate.clone();
                changes.retain(|change| item_id(change).is_none_or(|id| id.starts_with(&prefix)));
                let contains_fence = changes
                    .iter()
                    .any(|change| item_id(change) == Some(fence_id.as_str()));
                async move {
                    if !changes.is_empty() {
                        sender
                            .unbounded_send(CallbackBatch {
                                lease: context.lease_token().to_owned(),
                                changes,
                            })
                            .map_err(|_| {
                                callback_error("Workload observer stopped receiving changes.")
                            })?;
                    }
                    if contains_fence {
                        let wait = gate.lock().await.take();
                        if let Some(wait) = wait {
                            wait.await.map_err(|_| {
                                callback_error("Workload checkpoint gate was abandoned.")
                            })?;
                        }
                    }
                    Ok(())
                }
            },
        )
        .build()?;
    let result = async {
        processor.start().await?;
        wait_ready(leases, workload).await?;
        let expected = workload.expected(initial);
        let mut observations = Observations {
            expected: expected.clone(),
            seen: BTreeSet::new(),
            fence_lease: None,
        };
        let mut phase: BTreeSet<_> = initial.iter().cloned().collect();
        for name in ["created", "deleted", "batch-deleted"] {
            source
                .create_item(
                    workload.name.clone(),
                    &workload.id(name),
                    workload.document(name, 1),
                    None,
                )
                .await?;
            phase.insert(workload.event(name, 1, Operation::Create));
        }
        let inserted = source
            .upsert_item(
                workload.name.clone(),
                &workload.id("upserted"),
                workload.document("upserted", 1),
                None,
            )
            .await?;
        if inserted.status() != StatusCode::Created {
            return Err("Upsert of a new workload item did not create it.".into());
        }
        phase.insert(workload.event("upserted", 1, Operation::Create));
        observations
            .collect_until(workload, &mut receiver, &phase)
            .await?;

        // Observe each revision before overwriting it; latest-version feeds may coalesce updates.
        for name in ["created", "upserted"] {
            let replaced = source
                .upsert_item(
                    workload.name.clone(),
                    &workload.id(name),
                    workload.document(name, 2),
                    None,
                )
                .await?;
            if replaced.status() != StatusCode::Ok {
                return Err(format!("Upsert of existing item {name} did not replace it.").into());
            }
            phase.insert(workload.event(name, 2, Operation::Replace));
        }
        observations
            .collect_until(workload, &mut receiver, &phase)
            .await?;

        source
            .delete_item(workload.name.clone(), &workload.id("deleted"), None)
            .await?;
        let batch = TransactionalBatch::new(workload.name.clone())
            .create_item(workload.document("batch-created", 1))?
            .upsert_item(workload.document("created", 3), None)?
            .upsert_item(workload.document("batch-upserted", 1), None)?
            .delete_item(workload.id("batch-deleted"), None);
        let response = source.execute_transactional_batch(batch, None).await?;
        if response.status() != StatusCode::Ok {
            return Err(format!("Transactional workload failed: {}.", response.status()).into());
        }
        let statuses = response
            .into_model()?
            .results()
            .iter()
            .map(|result| result.status_code())
            .collect::<Vec<_>>();
        if statuses != [201, 200, 201, 204] {
            return Err(
                format!("Unexpected transactional operation statuses: {statuses:?}.").into(),
            );
        }
        source
            .create_item(
                workload.name.clone(),
                &workload.id("fence"),
                workload.document("fence", 1),
                None,
            )
            .await?;
        observations
            .collect_until(workload, &mut receiver, &expected)
            .await?;
        if observations.seen != expected {
            return Err(format!(
                "Callback event set differs: {:?}; expected {expected:?}.",
                observations.seen
            )
            .into());
        }
        let lease = observations
            .fence_lease
            .as_deref()
            .ok_or("No callback contained the final same-partition fence.")?;
        let held = checkpoint(leases, lease).await?;
        tokio::time::sleep(Duration::from_millis(250)).await;
        if checkpoint(leases, lease).await? != held {
            return Err("Lease checkpoint advanced while the final callback was blocked.".into());
        }
        release.send(()).map_err(|_| {
            io::Error::other("Final callback stopped before its checkpoint gate was released.")
        })?;
        tokio::time::timeout(WAIT, async {
            while checkpoint(leases, lease).await? == held {
                tokio::time::sleep(INTERVAL).await;
            }
            Ok::<_, Box<dyn std::error::Error>>(())
        })
        .await
        .map_err(|_| {
            io::Error::other("Successful final callback was not followed by a durable checkpoint.")
        })??;
        verify_source(source, workload).await
    }
    .await;
    combine(result, processor.stop().await.map_err(Into::into))
}

async fn verify_source(source: &ContainerClient, workload: &Workload) -> TestResult {
    for (name, revision) in [
        ("created", 3),
        ("upserted", 2),
        ("batch-created", 1),
        ("batch-upserted", 1),
        ("fence", 1),
    ] {
        let actual: Document = source
            .read_item(workload.name.clone(), &workload.id(name), None)
            .await?
            .into_model()?;
        if actual != workload.document(name, revision) {
            return Err(format!("Unexpected persisted item {name}: {actual:?}.").into());
        }
    }
    for name in ["deleted", "batch-deleted"] {
        match source
            .read_item(workload.name.clone(), &workload.id(name), None)
            .await
        {
            Err(error) if error.status().is_not_found() => {}
            Err(error) => return Err(error.into()),
            Ok(_) => return Err(format!("Deleted item {name} is still readable.").into()),
        }
    }
    Ok(())
}

async fn latest_case(
    source: ContainerClient,
    leases: ContainerClient,
    start: Start,
    page_size: u32,
) -> TestResult {
    validate_source(&source).await?;
    let workload = Workload::new(Semantics::Latest);
    let result = async {
        let (time, initial) = seed_source(&source, &workload, start).await?;
        let builder = ChangeFeedProcessorBuilder::new(
            LatestVersion,
            source.clone(),
            leases.clone(),
            &workload.name,
            &workload.name,
        );
        let builder = match start {
            Start::Now => builder.with_start_from_now(),
            Start::Beginning => builder.with_start_from_beginning(),
            Start::AtTime => {
                builder.with_start_time(time.ok_or("Start-time fixture is missing its boundary.")?)
            }
        };
        exercise(&source, &leases, &workload, builder, page_size, &initial).await
    }
    .await;
    combine(result, workload.cleanup(&source, &leases).await)
}

async fn all_versions_case(
    source: ContainerClient,
    leases: ContainerClient,
    page_size: u32,
) -> TestResult {
    validate_source(&source).await?;
    let workload = Workload::new(Semantics::AllVersions);
    let result = async {
        let (_, initial) = seed_source(&source, &workload, Start::Now).await?;
        let mut pages = source
            .query_change_feed::<Document>(
                FeedScope::partition(workload.name.clone()),
                ChangeFeedStartFrom::Now,
                Some(ChangeFeedOptions::default().with_mode(ChangeFeedMode::AllVersionsAndDeletes)),
            )
            .await?;
        let page = tokio::time::timeout(WAIT, pages.next())
            .await?
            .ok_or("Full-fidelity eligibility read unexpectedly ended.")??;
        if !page.items().is_empty() {
            return Err(
                "A fresh full-fidelity Now read unexpectedly returned historical items.".into(),
            );
        }
        let builder = ChangeFeedProcessorBuilder::new(
            AllVersionsAndDeletes,
            source.clone(),
            leases.clone(),
            &workload.name,
            &workload.name,
        );
        exercise(&source, &leases, &workload, builder, page_size, &initial).await
    }
    .await;
    combine(result, workload.cleanup(&source, &leases).await)
}

#[tokio::test]
#[ignore = "Requires explicit live opt-in; source /pk, leases /id with defaultTtl -1."]
async fn cfp_live_mixed_latest_now_small_pages() -> TestResult {
    let (source, leases) = live_containers().await?;
    latest_case(source, leases, Start::Now, 1).await
}

#[tokio::test]
#[ignore = "Requires explicit live opt-in; source /pk, leases /id with defaultTtl -1."]
async fn cfp_live_mixed_latest_now_larger_pages() -> TestResult {
    let (source, leases) = live_containers().await?;
    latest_case(source, leases, Start::Now, 64).await
}

#[tokio::test]
#[ignore = "Requires explicit live opt-in; source /pk, leases /id with defaultTtl -1."]
async fn cfp_live_mixed_latest_beginning() -> TestResult {
    let (source, leases) = live_containers().await?;
    latest_case(source, leases, Start::Beginning, 2).await
}

#[tokio::test]
#[ignore = "Requires explicit live opt-in; source /pk, leases /id with defaultTtl -1."]
async fn cfp_live_mixed_latest_start_time() -> TestResult {
    let (source, leases) = live_containers().await?;
    latest_case(source, leases, Start::AtTime, 2).await
}

#[tokio::test]
#[ignore = "Requires explicit live opt-in and an all-versions-and-deletes-enabled /pk source."]
async fn cfp_live_mixed_all_versions_now_small_pages() -> TestResult {
    let (source, leases) = live_containers().await?;
    all_versions_case(source, leases, 1).await
}

#[tokio::test]
#[ignore = "Requires explicit live opt-in and an all-versions-and-deletes-enabled /pk source."]
async fn cfp_live_mixed_all_versions_now_larger_pages() -> TestResult {
    let (source, leases) = live_containers().await?;
    all_versions_case(source, leases, 64).await
}

#[cfg(feature = "__internal_in_memory_emulator")]
#[path = "workload/local_tests.rs"]
pub(super) mod local_tests;
