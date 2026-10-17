// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{exercise, Key, Scenario, Sink, TestResult, Workload};
use crate::change_feed_processor::workload::local_tests::workload_containers;
use std::sync::Arc;

#[tokio::test]
async fn backlog_drains_all_ranges_in_key_order() -> TestResult {
    let (source, leases) = workload_containers().await?;
    exercise(source, leases, Scenario::Drain).await
}

#[tokio::test]
async fn backlog_restart_replays_only_unacknowledged_progress() -> TestResult {
    let (source, leases) = workload_containers().await?;
    exercise(source, leases, Scenario::Restart).await
}

#[tokio::test]
async fn backlog_scale_out_steals_and_scale_down_recovers() -> TestResult {
    let (source, leases) = workload_containers().await?;
    exercise(source, leases, Scenario::Scale).await
}

fn workload() -> Arc<Workload> {
    Arc::new(Workload::from_keys(
        "ordering".into(),
        vec![
            Key {
                index: 0,
                range: 0,
                pk: "a".into(),
            },
            Key {
                index: 1,
                range: 1,
                pk: "b".into(),
            },
        ],
        2,
        2,
    ))
}

#[test]
fn ordering_accepts_transaction_permutations_and_known_replays() -> TestResult {
    let workload = workload();
    let (sink, _receiver) = Sink::new(workload.clone(), Vec::new());
    let mut batch = workload.documents(&workload.keys[0], 0);
    batch.reverse();
    sink.apply("worker", &batch)?;
    sink.apply("worker", &workload.documents(&workload.keys[0], 1))?;
    sink.apply("new-owner", &batch)?;
    let state = sink.state.lock().unwrap();
    assert_eq!(
        state
            .applied
            .iter()
            .map(|(_, doc)| (doc.group, doc.member))
            .collect::<Vec<_>>(),
        [(0, 2), (0, 1), (0, 0), (1, 0)]
    );
    assert_eq!(state.replays.len(), 3);
    Ok(())
}

#[test]
fn ordering_rejects_first_delivery_from_an_earlier_group() -> TestResult {
    let workload = workload();
    let (sink, _receiver) = Sink::new(workload.clone(), Vec::new());
    sink.apply("worker", &workload.documents(&workload.keys[0], 1))?;
    let error = sink
        .apply("worker", &workload.documents(&workload.keys[0], 0))
        .unwrap_err();
    assert!(error.to_string().contains("Out-of-order first delivery"));
    Ok(())
}

#[test]
fn ordering_does_not_compare_unrelated_partition_keys() -> TestResult {
    let workload = workload();
    let (sink, _receiver) = Sink::new(workload.clone(), Vec::new());
    sink.apply("worker", &workload.documents(&workload.keys[0], 1))?;
    sink.apply("worker", &workload.documents(&workload.keys[1], 0))?;
    sink.check()?;
    assert_eq!(sink.state.lock().unwrap().seen.len(), 4);
    Ok(())
}
