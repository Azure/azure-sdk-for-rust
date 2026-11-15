// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{ChangeFeedProcessorBuilder, ChangeFeedProcessorContext, ChangeFeedStartFrom};
use crate::{
    diagnostics::DiagnosticsContext,
    feed::{ContinuationToken, FeedScope},
    models::{ChangeFeedItem, TimeToLive},
    options::{ChangeFeedMode, MaxItemCountHint, Region},
    AccountReference, ContainerClient, CosmosClientBuilder, CosmosRuntimeBuilder, RoutingStrategy,
};
use azure_core::{
    credentials::Secret,
    http::{StatusCode, Url},
};
use azure_data_cosmos_driver::{
    driver::CosmosDriverRuntimeBuilder,
    in_memory_emulator::{
        ContainerConfig, InMemoryEmulatorHttpClient, RequestObserver, VirtualAccountConfig,
        VirtualRegion,
    },
    models::ActivityId,
};
use futures::TryStreamExt;
use serde_json::Value;
use std::{error::Error, num::NonZeroU32, sync::Arc, time::Duration};
use time::OffsetDateTime;

#[cfg(feature = "fault_injection")]
mod cooldown;
mod pollers;

async fn on_changes(
    _context: ChangeFeedProcessorContext,
    _changes: Vec<ChangeFeedItem<String>>,
) -> crate::Result<()> {
    Ok(())
}

async fn containers() -> Result<(ContainerClient, ContainerClient), Box<dyn Error>> {
    containers_with_partitions(1).await
}

async fn containers_with_partitions(
    partitions: u32,
) -> Result<(ContainerClient, ContainerClient), Box<dyn Error>> {
    containers_with_observer(partitions, None).await
}

async fn containers_with_observer(
    partitions: u32,
    observer: Option<Arc<dyn RequestObserver>>,
) -> Result<(ContainerClient, ContainerClient), Box<dyn Error>> {
    containers_with_runtime(partitions, observer, |emulator| emulator.runtime_builder()).await
}

async fn containers_with_runtime(
    partitions: u32,
    observer: Option<Arc<dyn RequestObserver>>,
    runtime: impl FnOnce(&Arc<InMemoryEmulatorHttpClient>) -> CosmosDriverRuntimeBuilder,
) -> Result<(ContainerClient, ContainerClient), Box<dyn Error>> {
    let endpoint = "https://eastus.emulator.local";
    let mut emulator =
        InMemoryEmulatorHttpClient::new(VirtualAccountConfig::new(vec![VirtualRegion::new(
            "East US",
            Url::parse(endpoint)?,
        )])?);
    if let Some(observer) = observer {
        emulator = emulator.with_request_observer(observer);
    }
    let emulator = Arc::new(emulator);
    let store = emulator.store();
    store.create_database("processor-tests");
    for name in ["monitored", "leases"] {
        store.create_container_with_config(
            "processor-tests",
            name,
            serde_json::from_value(serde_json::json!({
                "paths": ["/id"],
                "kind": "Hash",
                "version": 2
            }))?,
            ContainerConfig::default().with_partition_count(if name == "monitored" {
                partitions
            } else {
                1
            }),
        );
    }
    let client = CosmosClientBuilder::new()
        .with_runtime(
            CosmosRuntimeBuilder::from(runtime(&emulator))
                .build()
                .await?,
        )
        .build(
            AccountReference::with_authentication_key(
                endpoint.parse()?,
                Secret::new("dGVzdGtleQ=="),
            ),
            RoutingStrategy::ProximityTo(Region::EAST_US),
        )
        .await?;
    let database = client.database_client("processor-tests");
    let leases = database.container_client("leases", None).await?;
    let properties = leases
        .read(None)
        .await?
        .into_model()?
        .with_default_ttl(TimeToLive::NoDefault);
    leases.replace(properties, None).await?;
    Ok((database.container_client("monitored", None).await?, leases))
}

fn assert_defaults<T, Fut>(builder: &ChangeFeedProcessorBuilder<T, Fut>) {
    assert!(matches!(builder.start_from, ChangeFeedStartFrom::Now));
    assert!(builder.start_continuation.is_none());
    assert_eq!(
        builder.max_item_count,
        MaxItemCountHint::Limit(NonZeroU32::new(100).unwrap())
    );
    assert_eq!(builder.feed_poll_delay, Duration::from_secs(5));
    assert_eq!(builder.bootstrap_timeout, Duration::from_secs(120));
    assert_eq!(
        builder.checkpoint_policy,
        super::ChangeFeedCheckpointPolicy::Deferred {
            max_interval: Duration::from_secs(300),
        }
    );
    assert_eq!(builder.min_scale_count, 0);
    assert_eq!(builder.max_scale_count, 0);
    assert!(!builder.lease_verification_on_restart_enabled);
}

async fn pin_start(
    mode: ChangeFeedMode,
    source: &ContainerClient,
    leases: &ContainerClient,
    name: &str,
    partitions: usize,
) -> Result<(), Box<dyn Error>> {
    let warm = ChangeFeedProcessorBuilder::new(
        mode,
        source.clone(),
        leases.clone(),
        name,
        "warm",
        |_, _: Vec<ChangeFeedItem<Value>>| async {
            panic!("source must be empty while pinning its initial position")
        },
    )
    .with_checkpoint_policy(super::ChangeFeedCheckpointPolicy::Eager)
    .with_feed_poll_delay(Duration::from_millis(5))
    .build()?;
    warm.start().await?;
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let records = lease_records(leases).await.unwrap();
            let active: Vec<_> = records
                .iter()
                .filter(|record| record.get("range").is_some())
                .collect();
            if active.len() == partitions
                && active
                    .iter()
                    .all(|record| record.get("continuation").is_some())
            {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await?;
    warm.stop().await?;
    Ok(())
}

fn configure<T, Fut>(
    builder: ChangeFeedProcessorBuilder<T, Fut>,
) -> ChangeFeedProcessorBuilder<T, Fut> {
    builder
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(237).unwrap()))
        .with_feed_poll_delay(Duration::from_millis(1234))
        .with_bootstrap_timeout(Duration::from_secs(600))
        .with_checkpoint_policy(super::ChangeFeedCheckpointPolicy::Deferred {
            max_interval: Duration::from_secs(7),
        })
        .with_start_continuation(ContinuationToken::from_string("saved-position".into()))
        .with_min_scale_count(2)
        .with_max_scale_count(7)
        .with_lease_verification_enabled_on_restart(true)
}

async fn configuration_round_trip(mode: ChangeFeedMode) -> Result<(), Box<dyn Error>> {
    async fn on_changes(
        context: ChangeFeedProcessorContext,
        changes: Vec<ChangeFeedItem<String>>,
    ) -> crate::Result<()> {
        assert_eq!(context.lease_token(), "lease-token");
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].current().map(String::as_str), Some("order-42"));
        Ok(())
    }

    let (monitored, leases) = containers().await?;
    let builder = ChangeFeedProcessorBuilder::new(
        mode,
        monitored.clone(),
        leases.clone(),
        "order-projection",
        "worker-01",
        on_changes,
    );
    assert_defaults(&builder);
    let builder = configure(builder);
    let processor = builder.build()?;
    // Exhaustive destructuring makes added configuration fields visible to this test.
    let ChangeFeedProcessorBuilder {
        monitored_container,
        lease_container,
        workload_name,
        worker_name,
        mode: stored_mode,
        start_from,
        start_continuation,
        max_item_count,
        feed_poll_delay,
        bootstrap_timeout,
        checkpoint_policy,
        min_scale_count,
        max_scale_count,
        lease_verification_on_restart_enabled,
        handler,
    } = processor.configuration;
    assert_eq!(stored_mode, mode);

    assert_eq!(
        monitored_container.read(None).await?.into_model()?.id,
        "monitored"
    );
    assert_eq!(lease_container.read(None).await?.into_model()?.id, "leases");
    assert_eq!(workload_name, "order-projection");
    assert_eq!(worker_name, "worker-01");
    assert_eq!(
        max_item_count,
        MaxItemCountHint::Limit(NonZeroU32::new(237).unwrap())
    );
    assert_eq!(feed_poll_delay, Duration::from_millis(1234));
    assert_eq!(bootstrap_timeout, Duration::from_secs(600));
    assert_eq!(
        checkpoint_policy,
        super::ChangeFeedCheckpointPolicy::Deferred {
            max_interval: Duration::from_secs(7),
        }
    );
    assert_eq!(min_scale_count, 2);
    assert_eq!(max_scale_count, 7);
    assert!(lease_verification_on_restart_enabled);
    assert_eq!(start_from, ChangeFeedStartFrom::Now);
    assert_eq!(start_continuation.unwrap().as_str(), "saved-position");

    handler(
        ChangeFeedProcessorContext {
            lease_token: "lease-token".into(),
            diagnostics: Arc::new(DiagnosticsContext::for_testing(ActivityId::new_uuid())),
        },
        vec![serde_json::from_value(serde_json::json!({
            "current": "order-42"
        }))?],
    )
    .await?;
    Ok(())
}

#[tokio::test]
async fn latest_version_configuration_round_trip() -> Result<(), Box<dyn Error>> {
    configuration_round_trip(ChangeFeedMode::LatestVersion).await
}

#[tokio::test]
async fn all_versions_and_deletes_configuration_round_trip() -> Result<(), Box<dyn Error>> {
    configuration_round_trip(ChangeFeedMode::AllVersionsAndDeletes).await
}

#[tokio::test]
async fn start_setters_use_last_value_in_both_modes() -> Result<(), Box<dyn Error>> {
    let (monitored, leases) = containers().await?;
    let timestamp = OffsetDateTime::from_unix_timestamp(1_700_000_000)?;
    let set =
        |builder: ChangeFeedProcessorBuilder<_, _>, selection: &str, token: &str| match selection {
            "now" => builder.with_start_from_now(),
            "beginning" => builder.with_start_from_beginning(),
            "time" => builder.with_start_time(timestamp),
            "continuation" => {
                builder.with_start_continuation(ContinuationToken::from_string(token.into()))
            }
            _ => unreachable!("unknown test start setting"),
        };
    for mode in [
        ChangeFeedMode::LatestVersion,
        ChangeFeedMode::AllVersionsAndDeletes,
    ] {
        for previous in ["now", "beginning", "time", "continuation"] {
            for last in ["now", "beginning", "time", "continuation"] {
                let builder = ChangeFeedProcessorBuilder::new(
                    mode,
                    monitored.clone(),
                    leases.clone(),
                    "orders",
                    "worker",
                    on_changes,
                )
                .with_start_continuation(ContinuationToken::from_string(String::new()));
                let builder = set(builder, previous, "previous-position");
                let result = set(builder, last, "final-position").build();
                if mode == ChangeFeedMode::AllVersionsAndDeletes
                    && matches!(last, "beginning" | "time")
                {
                    assert_eq!(
                        result
                            .err()
                            .expect("unsupported start must fail")
                            .status()
                            .status_code(),
                        StatusCode::BadRequest
                    );
                    continue;
                }
                let processor = result?;
                let expected_start = match last {
                    "now" | "continuation" => ChangeFeedStartFrom::Now,
                    "beginning" => ChangeFeedStartFrom::Beginning,
                    "time" => ChangeFeedStartFrom::PointInTime(timestamp),
                    _ => unreachable!("unknown test start setting"),
                };
                assert_eq!(processor.configuration.start_from, expected_start);
                assert_eq!(
                    processor
                        .configuration
                        .start_continuation
                        .as_ref()
                        .map(ContinuationToken::as_str),
                    (last == "continuation").then_some("final-position"),
                );
            }
        }
    }
    Ok(())
}

async fn build_validation(mode: ChangeFeedMode) -> Result<(), Box<dyn Error>> {
    let (monitored, leases) = containers().await?;
    for timeout in [Duration::ZERO, Duration::MAX] {
        let result = ChangeFeedProcessorBuilder::new(
            mode,
            monitored.clone(),
            leases.clone(),
            "orders",
            "worker",
            on_changes,
        )
        .with_bootstrap_timeout(timeout)
        .build();
        let error = result.err().expect("invalid bootstrap timeout");
        assert_eq!(error.status().status_code(), StatusCode::BadRequest);
        assert!(error.to_string().contains("Invalid bootstrap timeout."));
    }
    for max_interval in [Duration::ZERO, Duration::MAX] {
        let result = ChangeFeedProcessorBuilder::new(
            mode,
            monitored.clone(),
            leases.clone(),
            "orders",
            "worker",
            |_, _: Vec<ChangeFeedItem<String>>| async { Ok(()) },
        )
        .with_checkpoint_policy(super::ChangeFeedCheckpointPolicy::Deferred { max_interval })
        .build();
        assert_eq!(
            result.err().unwrap().status().status_code(),
            StatusCode::BadRequest
        );
    }
    let cases = [
        ("orders", "worker", 0, 0, None, None),
        ("orders", "worker", 2, 2, None, None),
        ("orders", "worker", u32::MAX, 0, None, None),
        (" orders ", " worker ", 0, 1, Some(" opaque-token "), None),
        (
            "",
            "worker",
            0,
            0,
            None,
            Some("workload_name must not be blank."),
        ),
        (
            " \t\n",
            "worker",
            0,
            0,
            None,
            Some("workload_name must not be blank."),
        ),
        (
            "orders",
            "",
            0,
            0,
            None,
            Some("worker_name must not be blank."),
        ),
        (
            "orders",
            "\u{2003}",
            0,
            0,
            None,
            Some("worker_name must not be blank."),
        ),
        (
            "orders",
            "worker",
            3,
            2,
            None,
            Some("min_scale_count must not exceed a nonzero max_scale_count."),
        ),
        (
            "orders",
            "worker",
            0,
            0,
            Some(""),
            Some("start_continuation must not be blank."),
        ),
        (
            "orders",
            "worker",
            0,
            0,
            Some("\t \n"),
            Some("start_continuation must not be blank."),
        ),
    ];
    for (workload_name, worker_name, minimum, maximum, continuation, expected_error) in cases {
        let mut builder = ChangeFeedProcessorBuilder::new(
            mode,
            monitored.clone(),
            leases.clone(),
            workload_name,
            worker_name,
            on_changes,
        )
        .with_min_scale_count(minimum);
        if let Some(token) = continuation {
            builder = builder.with_start_continuation(ContinuationToken::from_string(token.into()));
        }
        let result = builder
            .with_max_scale_count(maximum)
            .with_max_item_count(MaxItemCountHint::ServerDecides)
            .with_feed_poll_delay(Duration::ZERO)
            .build();
        match (result, expected_error) {
            (Err(error), Some(message)) => {
                assert_eq!(error.status().status_code(), StatusCode::BadRequest);
                assert!(error.response().is_none());
                assert!(
                    error.to_string().contains(message),
                    "unexpected error: {error}"
                );
            }
            (Ok(processor), None) => {
                assert!(!processor.is_started());
                assert_eq!(processor.configuration.workload_name, workload_name);
                assert_eq!(processor.configuration.worker_name, worker_name);
                assert_eq!(processor.configuration.feed_poll_delay, Duration::ZERO);
                assert_eq!(processor.configuration.min_scale_count, minimum);
                assert_eq!(processor.configuration.max_scale_count, maximum);
                assert_eq!(
                    processor.configuration.max_item_count,
                    MaxItemCountHint::ServerDecides
                );
                assert_eq!(processor.configuration.start_from, ChangeFeedStartFrom::Now);
                assert_eq!(
                    processor
                        .configuration
                        .start_continuation
                        .as_ref()
                        .map(ContinuationToken::as_str),
                    continuation,
                );
            }
            (Err(error), None) => panic!("valid configuration was rejected: {error}"),
            (Ok(_), Some(message)) => panic!("invalid configuration was accepted: {message}"),
        }
    }
    Ok(())
}

#[tokio::test]
async fn latest_version_build_validates_configuration() -> Result<(), Box<dyn Error>> {
    build_validation(ChangeFeedMode::LatestVersion).await
}

#[tokio::test]
async fn all_versions_and_deletes_build_validates_configuration() -> Result<(), Box<dyn Error>> {
    build_validation(ChangeFeedMode::AllVersionsAndDeletes).await
}

#[tokio::test]
async fn mode_start_validation_with_required_handler() -> Result<(), Box<dyn Error>> {
    fn select_start<T, Fut>(
        builder: ChangeFeedProcessorBuilder<T, Fut>,
        start: u8,
    ) -> ChangeFeedProcessorBuilder<T, Fut> {
        match start {
            0 => builder.with_start_from_now(),
            1 => builder.with_start_from_beginning(),
            2 => builder.with_start_time(OffsetDateTime::UNIX_EPOCH),
            _ => builder
                .with_start_continuation(ContinuationToken::from_string("saved-position".into())),
        }
    }

    for mode in [
        ChangeFeedMode::LatestVersion,
        ChangeFeedMode::AllVersionsAndDeletes,
    ] {
        let (monitored, leases) = containers().await?;
        for start in 0..4 {
            let builder = ChangeFeedProcessorBuilder::new(
                mode,
                monitored.clone(),
                leases.clone(),
                "orders",
                "worker",
                on_changes,
            );
            let builder = select_start(builder, start);
            let result = builder.build();
            if mode == ChangeFeedMode::AllVersionsAndDeletes && matches!(start, 1 | 2) {
                let error = result.err().expect("unsupported start must be rejected");
                assert_eq!(error.status().status_code(), StatusCode::BadRequest);
                assert!(error.response().is_none());
                assert!(error.to_string().contains(
                    "AllVersionsAndDeletes mode does not support beginning or time-based starts."
                ));
            } else {
                let processor = result?;
                assert_eq!(processor.configuration.mode, mode);
                assert!(!processor.is_started());
                processor.stop().await?;
                assert_eq!(
                    processor
                        .current_state()
                        .await
                        .unwrap_err()
                        .status()
                        .status_code(),
                    StatusCode::NotImplemented,
                );
            }
        }
    }
    Ok(())
}

#[tokio::test]
async fn all_versions_and_deletes_validates_final_start_position() -> Result<(), Box<dyn Error>> {
    let (monitored, leases) = containers().await?;
    for use_continuation in [false, true] {
        let builder = ChangeFeedProcessorBuilder::new(
            ChangeFeedMode::AllVersionsAndDeletes,
            monitored.clone(),
            leases.clone(),
            "orders",
            "worker",
            on_changes,
        )
        .with_start_from_beginning()
        .with_start_time(OffsetDateTime::UNIX_EPOCH);
        let builder = if use_continuation {
            builder.with_start_continuation(ContinuationToken::from_string("resume".into()))
        } else {
            builder.with_start_from_now()
        };
        let processor = builder.build()?;
        assert_eq!(processor.configuration.start_from, ChangeFeedStartFrom::Now);
        assert_eq!(
            processor
                .configuration
                .start_continuation
                .as_ref()
                .map(ContinuationToken::as_str),
            use_continuation.then_some("resume"),
        );
    }
    Ok(())
}

async fn lease_records(leases: &ContainerClient) -> Result<Vec<Value>, Box<dyn Error>> {
    let mut records = Box::pin(
        leases
            .query_items::<Value>("SELECT * FROM c", FeedScope::full_container(), None)
            .await?,
    )
    .try_collect::<Vec<_>>()
    .await?;
    records.sort_by(|a, b| a["id"].as_str().cmp(&b["id"].as_str()));
    Ok(records)
}

#[tokio::test]
async fn configured_bootstrap_timeout_leaves_waiting_worker_stopped() -> Result<(), Box<dyn Error>>
{
    let (source, leases) = containers().await?;
    let build = |worker| {
        ChangeFeedProcessorBuilder::new(
            ChangeFeedMode::LatestVersion,
            source.clone(),
            leases.clone(),
            "timeout",
            worker,
            |_, _: Vec<ChangeFeedItem<Value>>| async {
                panic!("empty source cannot deliver changes")
            },
        )
    };
    let initial = build("initial").build()?;
    initial.start().await?;
    initial.stop().await?;
    let records = lease_records(&leases).await?;
    let info = records
        .iter()
        .find(|r| r["id"].as_str().unwrap().ends_with(".info"))
        .unwrap();
    let info_id = info["id"].as_str().unwrap();
    let lock_id = format!("{}.lock", info_id.strip_suffix(".info").unwrap());
    leases
        .delete_item(info_id.to_owned(), info_id, None)
        .await?;
    leases
        .create_item(
            lock_id.clone(),
            &lock_id,
            serde_json::json!({
                "id": lock_id,
                "workload": info["workload"],
                "ttl": 30,
                "owner": "other-initializer",
                "attempt": "held-attempt"
            }),
            None,
        )
        .await?;
    let waiting = build("waiting")
        .with_bootstrap_timeout(Duration::from_millis(50))
        .build()?;
    let error = tokio::time::timeout(Duration::from_secs(2), waiting.start())
        .await?
        .expect_err("another initializer still holds the lock");
    assert_eq!(error.status().status_code(), StatusCode::RequestTimeout);
    assert!(!waiting.is_started());
    waiting.stop().await?;
    let after = lease_records(&leases).await?;
    assert!(!after.iter().any(|r| r["id"] == info_id));
    assert_eq!(
        after.iter().find(|r| r["id"] == lock_id).unwrap()["attempt"],
        "held-attempt"
    );
    Ok(())
}

async fn start_bootstraps_and_runs(
    mode: ChangeFeedMode,
    expected_mode: &str,
) -> Result<(), Box<dyn Error>> {
    let (source, leases) = containers().await?;
    let processor = ChangeFeedProcessorBuilder::new(
        mode,
        source,
        leases.clone(),
        "orders",
        "worker",
        |_, _: Vec<ChangeFeedItem<String>>| async {
            panic!("bootstrap must not invoke callbacks");
        },
    )
    .build()?;
    assert!(lease_records(&leases).await?.is_empty());
    processor.start().await?;
    assert!(processor.is_started());
    processor.stop().await?;
    let before = lease_records(&leases).await?;
    assert!(before
        .iter()
        .any(|r| r["id"].as_str().unwrap().ends_with(".info")));
    let range_leases = before
        .iter()
        .filter(|r| r.get("range").is_some())
        .collect::<Vec<_>>();
    assert!(!range_leases.is_empty());
    for lease in range_leases {
        assert_eq!(lease["workload"]["mode"], expected_mode);
        assert!(lease["owner"].is_null());
    }
    processor.start().await?;
    processor.stop().await?;
    let after = lease_records(&leases).await?;
    assert_eq!(after.len(), before.len());
    for previous in before {
        let current = after.iter().find(|r| r["id"] == previous["id"]).unwrap();
        assert_eq!(current["workload"], previous["workload"]);
        if let Some(continuation) = previous.get("continuation") {
            assert_eq!(current.get("continuation"), Some(continuation));
        }
    }
    assert!(!processor.is_started());
    Ok(())
}

#[tokio::test]
async fn latest_version_start_bootstraps_and_runs() -> Result<(), Box<dyn Error>> {
    start_bootstraps_and_runs(ChangeFeedMode::LatestVersion, "latestVersion").await
}

#[tokio::test]
async fn all_versions_start_bootstraps_and_runs() -> Result<(), Box<dyn Error>> {
    start_bootstraps_and_runs(
        ChangeFeedMode::AllVersionsAndDeletes,
        "allVersionsAndDeletes",
    )
    .await
}

#[tokio::test]
async fn start_does_not_repair_completed_workloads() -> Result<(), Box<dyn Error>> {
    let (source, leases) = containers().await?;
    let first = ChangeFeedProcessorBuilder::new(
        ChangeFeedMode::LatestVersion,
        source.clone(),
        leases.clone(),
        "orders",
        "first",
        on_changes,
    )
    .build()?;
    first.start().await?;
    first.stop().await?;
    let mut remaining = lease_records(&leases).await?;
    let index = remaining
        .iter()
        .position(|record| record.get("range").is_some())
        .unwrap();
    let removed = remaining.remove(index);
    let id = removed["id"].as_str().unwrap();
    leases.delete_item(id.to_owned(), id, None).await?;

    let later = ChangeFeedProcessorBuilder::new(
        ChangeFeedMode::LatestVersion,
        source,
        leases.clone(),
        "orders",
        "later",
        on_changes,
    )
    .build()?;
    later.start().await?;
    later.stop().await?;
    assert_eq!(lease_records(&leases).await?, remaining);
    assert!(!later.is_started());
    Ok(())
}

#[tokio::test]
async fn start_propagates_bootstrap_mode_conflict() -> Result<(), Box<dyn Error>> {
    let (source, leases) = containers().await?;
    let first = ChangeFeedProcessorBuilder::new(
        ChangeFeedMode::LatestVersion,
        source.clone(),
        leases.clone(),
        "orders",
        "first",
        |_, _: Vec<ChangeFeedItem<String>>| async { Ok(()) },
    )
    .build()?;
    first.start().await?;
    first.stop().await?;
    let before = lease_records(&leases).await?;
    let other = ChangeFeedProcessorBuilder::new(
        ChangeFeedMode::AllVersionsAndDeletes,
        source,
        leases.clone(),
        "orders",
        "other",
        |_, _: Vec<ChangeFeedItem<String>>| async { Ok(()) },
    )
    .build()?;
    assert_eq!(
        other.start().await.unwrap_err().status().status_code(),
        StatusCode::Conflict
    );
    assert_eq!(lease_records(&leases).await?, before);
    assert!(!other.is_started());
    Ok(())
}
