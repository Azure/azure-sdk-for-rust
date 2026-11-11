// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    expected, physical_ranges, run_case, Action, Collector, DocumentSpec, Event, FeedRange,
    Operation, Plan, TestResult, DOCUMENTS, DOCUMENTS_PER_PARTITION, LIVE_TICK, PARTITIONS,
    SCHEDULE, WORKERS,
};
use crate::change_feed_processor::workload::local_tests::partitioned_workload_containers;
use azure_data_cosmos::{
    models::{ChangeFeedItem, EffectivePartitionKey},
    FeedScope,
};
use futures::TryStreamExt;
use serde_json::{json, Value};
use std::{collections::BTreeMap, time::Duration};
use tokio::time::Instant;

fn collector() -> TestResult<Collector> {
    let ranges = (0..PARTITIONS)
        .map(|partition| {
            FeedRange::new(
                EffectivePartitionKey::try_from(format!("{partition:02X}").as_str())?,
                EffectivePartitionKey::try_from(format!("{:02X}", partition + 1).as_str())?,
            )
        })
        .collect::<azure_data_cosmos::Result<Vec<_>>>()?;
    let documents = (0..DOCUMENTS)
        .map(|index| DocumentSpec {
            id: format!("oracle-doc-{index}"),
            pk: format!("key-{}", index / DOCUMENTS_PER_PARTITION),
            partition: index / DOCUMENTS_PER_PARTITION,
        })
        .collect();
    let collector = Collector::new("oracle".into());
    collector
        .plan
        .set(Plan {
            name: "oracle".into(),
            inventory: ranges.clone(),
            ranges,
            documents,
            partition_key_field: "pk".into(),
        })
        .map_err(|_| "Plan already initialized.")?;
    {
        let mut state = collector.state.lock().unwrap();
        state.started = Some(Instant::now());
        state.leases = (0..PARTITIONS)
            .map(|partition| (format!("lease-{partition}"), partition))
            .collect();
        state.owners = (0..PARTITIONS)
            .map(|partition| (format!("lease-{partition}"), partition % WORKERS))
            .collect();
    }
    Ok(collector)
}

fn envelope(index: usize, event: Event, lsn: i64, object_partition_key: bool) -> Value {
    let id = format!("oracle-doc-{index}");
    let pk = format!("key-{}", index / DOCUMENTS_PER_PARTITION);
    let operation = match event.operation {
        Operation::Create => "create",
        Operation::Replace => "replace",
        Operation::Delete => "delete",
    };
    json!({
        "current": if event.operation == Operation::Delete {
            Value::Null
        } else {
            json!({"id":id, "pk":pk, "revision":event.revision})
        },
        "metadata": {
            "id": id, "lsn": lsn, "operationType": operation,
            "partitionKey": if object_partition_key { json!({"pk":pk}) } else { json!([pk]) },
        },
    })
}

fn accept(collector: &Collector, index: usize, event: Event, lsn: i64) -> TestResult {
    let change: ChangeFeedItem<Value> =
        serde_json::from_value(envelope(index, event, lsn, index.is_multiple_of(2)))?;
    collector.accept(
        (index / DOCUMENTS_PER_PARTITION) % WORKERS,
        &format!("lease-{}", index / DOCUMENTS_PER_PARTITION),
        &[change],
    )?;
    Ok(())
}

#[test]
fn schedule_matches_independent_version_sequences_and_exact_ten_minute_budget() {
    assert_eq!(LIVE_TICK * 10, Duration::from_secs(600));
    assert_eq!(
        SCHEDULE.map(|step| step.minute),
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    );
    assert_eq!(
        LIVE_TICK * SCHEDULE.last().unwrap().minute,
        Duration::from_secs(540)
    );
    let mut stored = BTreeMap::new();
    let mut events = vec![Vec::new(); DOCUMENTS];
    let mut counts = [0; 3];
    for step in SCHEDULE {
        for (index, events) in events.iter_mut().enumerate() {
            if !step.selection.contains(index) {
                continue;
            }
            let event = match step.action {
                Action::Create(revision) => {
                    assert!(stored.insert(index, revision).is_none());
                    Event {
                        operation: Operation::Create,
                        revision: Some(revision),
                    }
                }
                Action::Upsert(revision, expected_operation) => {
                    let operation = if stored.insert(index, revision).is_some() {
                        Operation::Replace
                    } else {
                        Operation::Create
                    };
                    assert_eq!(operation, expected_operation);
                    Event {
                        operation,
                        revision: Some(revision),
                    }
                }
                Action::Delete => {
                    assert!(stored.remove(&index).is_some());
                    Event {
                        operation: Operation::Delete,
                        revision: None,
                    }
                }
            };
            counts[match event.operation {
                Operation::Create => 0,
                Operation::Replace => 1,
                Operation::Delete => 2,
            }] += 1;
            events.push(event);
        }
    }
    assert_eq!(counts, [200, 400, 200]);
    assert!(stored.is_empty());
    for (index, actual) in events.iter().enumerate() {
        assert_eq!(actual, &expected(index));
    }
}

#[test]
fn oracle_accepts_all_versions_deletes_and_identical_replay_without_losing_counts() -> TestResult {
    let collector = collector()?;
    for ordinal in 0..8 {
        for index in 0..DOCUMENTS {
            accept(
                &collector,
                index,
                expected(index)[ordinal],
                ordinal as i64 + 1,
            )?;
        }
    }
    accept(&collector, 0, expected(0)[0], 1)?;
    let summary = collector.verify()?;
    assert_eq!(summary.partitions, [80; PARTITIONS]);
    assert_eq!(summary.operations, [200, 400, 200]);
    assert_eq!(summary.duplicates, 1);
    Ok(())
}

#[test]
fn oracle_rejects_missing_extra_reordered_and_wrong_revision_events() -> TestResult {
    let missing = collector()?;
    assert!(missing.verify().is_err());
    accept(&missing, 0, expected(0)[0], 1)?;
    assert!(accept(&missing, 0, expected(0)[2], 2).is_err());

    let extra = collector()?;
    for (ordinal, event) in expected(0).into_iter().enumerate() {
        accept(&extra, 0, event, ordinal as i64 + 1)?;
    }
    assert!(accept(&extra, 0, expected(0)[0], 9).is_err());

    let reordered = collector()?;
    accept(&reordered, 0, expected(0)[0], 5)?;
    assert!(accept(&reordered, 0, expected(0)[1], 4).is_err());

    let replay = collector()?;
    accept(&replay, 0, expected(0)[0], 1)?;
    assert!(accept(
        &replay,
        0,
        Event {
            operation: Operation::Create,
            revision: Some(999)
        },
        1
    )
    .is_err());
    Ok(())
}

#[test]
fn oracle_rejects_wrong_partition_and_incomplete_metadata() -> TestResult {
    let collector = collector()?;
    let change = serde_json::from_value(envelope(0, expected(0)[0], 1, false))?;
    assert!(collector.accept(1, "lease-1", &[change]).is_err());
    for field in ["operationType", "lsn"] {
        let mut value = envelope(0, expected(0)[0], 1, false);
        value["metadata"].as_object_mut().unwrap().remove(field);
        let change = serde_json::from_value(value)?;
        assert!(collector.accept(0, "lease-0", &[change]).is_err());
    }
    let mut value = envelope(0, expected(0)[0], 1, false);
    value["current"]["pk"] = json!("wrong");
    let change = serde_json::from_value(value)?;
    assert!(collector.accept(0, "lease-0", &[change]).is_err());
    Ok(())
}

#[test]
fn oracle_rejects_wrong_delete_partition_key_in_both_supported_shapes() -> TestResult {
    for object in [false, true] {
        let collector = collector()?;
        for ordinal in 0..3 {
            accept(&collector, 0, expected(0)[ordinal], ordinal as i64 + 1)?;
        }
        let mut value = envelope(0, expected(0)[3], 4, object);
        value["metadata"]["partitionKey"] = if object {
            json!({"pk":"wrong"})
        } else {
            json!(["wrong"])
        };
        let change = serde_json::from_value(value)?;
        assert!(collector.accept(0, "lease-0", &[change]).is_err());
    }
    Ok(())
}

#[tokio::test]
#[ignore = "Requires retained AVAD version/delete history and LSNs, which the current in-memory emulator does not implement."]
async fn cfp_in_memory_avad_fixed_schedule_ten_minutes() -> TestResult {
    let (source, leases) = partitioned_workload_containers(PARTITIONS as u32).await?;
    let summary = run_case(source.clone(), leases.clone(), LIVE_TICK).await?;
    assert_eq!(summary.operations, [200, 400, 200]);
    assert_eq!(summary.partitions, [80; PARTITIONS]);
    assert!(summary.elapsed >= Duration::from_secs(600));
    for container in [source, leases] {
        let query = container
            .query_items::<Value>("SELECT * FROM c", FeedScope::full_container(), None)
            .await?;
        assert!(Box::pin(query).try_collect::<Vec<_>>().await?.is_empty());
    }
    Ok(())
}

#[tokio::test]
async fn preflight_rejects_wrong_partition_count_without_starting_a_processor() -> TestResult {
    let (source, leases) = partitioned_workload_containers(3).await?;
    let error = run_case(source, leases, Duration::from_secs(1))
        .await
        .unwrap_err();
    assert!(error
        .to_string()
        .contains("at least 10 physical partitions"));
    Ok(())
}

#[tokio::test]
async fn planning_preserves_existing_source_documents() -> TestResult {
    let (source, leases) = partitioned_workload_containers(PARTITIONS as u32).await?;
    source
        .create_item(
            "existing",
            "existing",
            json!({"id":"existing","pk":"existing"}),
            None,
        )
        .await?;
    let inventory = physical_ranges(&source).await?;
    let assigned = inventory
        .iter()
        .map(|range| json!({"range":range}))
        .collect::<Vec<_>>();
    let plan = Plan::create(&source, "isolated-plan".into(), inventory, &assigned).await?;
    assert_eq!(plan.documents.len(), DOCUMENTS);
    assert!(plan
        .documents
        .iter()
        .all(|document| document.id != "existing"));
    let query = leases
        .query_items::<Value>("SELECT * FROM c", FeedScope::full_container(), None)
        .await?;
    assert!(Box::pin(query).try_collect::<Vec<_>>().await?.is_empty());
    let stored: Value = source
        .read_item("existing", "existing", None)
        .await?
        .into_model()?;
    assert_eq!(stored["id"], "existing");
    Ok(())
}
