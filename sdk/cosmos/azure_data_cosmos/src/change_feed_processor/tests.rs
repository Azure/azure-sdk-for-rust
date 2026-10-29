// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{ChangeFeedProcessorBuilder, ChangeFeedProcessorContext, ProcessorStartFrom};
use crate::{
    feed::ContinuationToken,
    models::ChangeFeedItem,
    options::{ChangeFeedMode, MaxItemCountHint, Region},
    AccountReference, ContainerClient, CosmosClientBuilder, CosmosRuntimeBuilder, FeedScope,
    RoutingStrategy,
};
use azure_core::{
    credentials::Secret,
    http::{StatusCode, Url},
};
use azure_data_cosmos_driver::in_memory_emulator::{
    InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
};
use azure_data_cosmos_driver::{
    models::{AccountReference as DriverAccountReference, CosmosOperation, DatabaseReference},
    options::{DriverOptions, OperationOptions},
};
use futures::TryStreamExt;
use serde_json::Value;
use std::{error::Error, num::NonZeroU32, sync::Arc, time::Duration};
use time::OffsetDateTime;

async fn on_changes(
    _context: ChangeFeedProcessorContext,
    _changes: Vec<ChangeFeedItem<String>>,
) -> crate::Result<()> {
    Ok(())
}

async fn containers() -> Result<(ContainerClient, ContainerClient), Box<dyn Error>> {
    let endpoint = "https://eastus.emulator.local";
    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(VirtualAccountConfig::new(
        vec![VirtualRegion::new("East US", Url::parse(endpoint)?)],
    )?));
    let store = emulator.store();
    store.create_database("processor-tests");
    store.create_container(
        "processor-tests",
        "monitored",
        serde_json::from_value(serde_json::json!({
            "paths": ["/id"],
            "kind": "Hash",
            "version": 2
        }))?,
    );
    let driver = emulator
        .runtime_builder()
        .build()
        .await?
        .create_driver(
            DriverOptions::builder(DriverAccountReference::with_account_key(
                endpoint.parse()?,
                "dGVzdGtleQ==",
            ))
            .build(),
        )
        .await?;
    driver
        .execute_singleton_operation(
            CosmosOperation::create_container(DatabaseReference::from_name(
                driver.account().clone(),
                "processor-tests",
            ))
            .with_body(serde_json::to_vec(&serde_json::json!({
                "id": "leases",
                "partitionKey": {"paths": ["/id"], "kind": "Hash", "version": 2},
                "defaultTtl": -1
            }))?),
            OperationOptions::default(),
        )
        .await?;
    let client = CosmosClientBuilder::new()
        .with_runtime(
            CosmosRuntimeBuilder::from(emulator.runtime_builder())
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
    Ok((
        database.container_client("monitored", None).await?,
        database.container_client("leases", None).await?,
    ))
}

fn assert_defaults<T, H>(builder: &ChangeFeedProcessorBuilder<T, H>) {
    assert!(matches!(builder.start_from, ProcessorStartFrom::Now));
    assert_eq!(
        builder.max_item_count,
        MaxItemCountHint::Limit(NonZeroU32::new(100).unwrap())
    );
    assert_eq!(builder.feed_poll_delay, Duration::from_secs(5));
    assert_eq!(builder.min_scale_count, 0);
    assert_eq!(builder.max_scale_count, 0);
    assert!(!builder.lease_verification_on_restart_enabled);
}

fn configure<T, H>(builder: ChangeFeedProcessorBuilder<T, H>) -> ChangeFeedProcessorBuilder<T, H> {
    builder
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(237).unwrap()))
        .with_feed_poll_delay(Duration::from_millis(1234))
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
    for register_first in [false, true] {
        let builder = ChangeFeedProcessorBuilder::new(
            mode,
            monitored.clone(),
            leases.clone(),
            "order-projection",
            "worker-01",
        );
        assert_defaults(&builder);
        let builder = if register_first {
            let builder = builder.handle_change_feed(on_changes);
            assert_defaults(&builder);
            configure(builder)
        } else {
            configure(builder).handle_change_feed(on_changes)
        };
        let processor = builder.build()?;
        // Exhaustive destructuring makes added configuration fields visible to this test.
        let ChangeFeedProcessorBuilder {
            monitored_container,
            lease_container,
            processor_name,
            instance_name,
            mode: stored_mode,
            start_from,
            max_item_count,
            feed_poll_delay,
            min_scale_count,
            max_scale_count,
            lease_verification_on_restart_enabled,
            handler,
            item: _,
        } = processor.configuration;
        assert_eq!(stored_mode, mode);

        assert_eq!(
            monitored_container.read(None).await?.into_model()?.id,
            "monitored"
        );
        assert_eq!(lease_container.read(None).await?.into_model()?.id, "leases");
        assert_eq!(processor_name, "order-projection");
        assert_eq!(instance_name, "worker-01");
        assert_eq!(
            max_item_count,
            MaxItemCountHint::Limit(NonZeroU32::new(237).unwrap())
        );
        assert_eq!(feed_poll_delay, Duration::from_millis(1234));
        assert_eq!(min_scale_count, 2);
        assert_eq!(max_scale_count, 7);
        assert!(lease_verification_on_restart_enabled);
        let ProcessorStartFrom::Continuation(token) = start_from else {
            panic!("initial continuation was not preserved");
        };
        assert_eq!(token.as_str(), "saved-position");

        handler(
            ChangeFeedProcessorContext {
                lease_token: "lease-token".into(),
            },
            vec![serde_json::from_value(serde_json::json!({
                "current": "order-42"
            }))?],
        )
        .await?;
    }
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
async fn latest_version_start_positions_round_trip() -> Result<(), Box<dyn Error>> {
    let (monitored, leases) = containers().await?;
    let timestamp = OffsetDateTime::from_unix_timestamp(1_700_000_000)?;
    for expected in [
        ProcessorStartFrom::Now,
        ProcessorStartFrom::Beginning,
        ProcessorStartFrom::PointInTime(timestamp),
        ProcessorStartFrom::Continuation(ContinuationToken::from_string("initial-position".into())),
    ] {
        let builder = ChangeFeedProcessorBuilder::new(
            ChangeFeedMode::LatestVersion,
            monitored.clone(),
            leases.clone(),
            "orders",
            "worker",
        )
        .with_start_continuation(ContinuationToken::from_string(String::new()));
        let builder = match &expected {
            ProcessorStartFrom::Now => builder.with_start_from_now(),
            ProcessorStartFrom::Beginning => builder.with_start_from_beginning(),
            ProcessorStartFrom::PointInTime(time) => builder.with_start_time(*time),
            ProcessorStartFrom::Continuation(token) => builder
                .with_start_from_beginning()
                .with_start_continuation(token.clone()),
        };
        let processor = builder.handle_change_feed(on_changes).build()?;
        match (expected, processor.configuration.start_from) {
            (ProcessorStartFrom::Now, ProcessorStartFrom::Now)
            | (ProcessorStartFrom::Beginning, ProcessorStartFrom::Beginning) => {}
            (
                ProcessorStartFrom::PointInTime(expected),
                ProcessorStartFrom::PointInTime(actual),
            ) => {
                assert_eq!(actual, expected);
            }
            (
                ProcessorStartFrom::Continuation(expected),
                ProcessorStartFrom::Continuation(actual),
            ) => {
                assert_eq!(actual, expected);
            }
            _ => panic!("initial start position was not preserved"),
        }
    }
    Ok(())
}

async fn build_validation(mode: ChangeFeedMode) -> Result<(), Box<dyn Error>> {
    let (monitored, leases) = containers().await?;
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
            Some("processor_name must not be blank."),
        ),
        (
            " \t\n",
            "worker",
            0,
            0,
            None,
            Some("processor_name must not be blank."),
        ),
        (
            "orders",
            "",
            0,
            0,
            None,
            Some("instance_name must not be blank."),
        ),
        (
            "orders",
            "\u{2003}",
            0,
            0,
            None,
            Some("instance_name must not be blank."),
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
    for (processor_name, instance_name, minimum, maximum, continuation, expected_error) in cases {
        let mut builder = ChangeFeedProcessorBuilder::new(
            mode,
            monitored.clone(),
            leases.clone(),
            processor_name,
            instance_name,
        )
        .with_min_scale_count(minimum);
        if let Some(token) = continuation {
            builder = builder.with_start_continuation(ContinuationToken::from_string(token.into()));
        }
        let result = builder
            .handle_change_feed(on_changes)
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
                assert_eq!(processor.configuration.processor_name, processor_name);
                assert_eq!(processor.configuration.instance_name, instance_name);
                assert_eq!(processor.configuration.feed_poll_delay, Duration::ZERO);
                assert_eq!(processor.configuration.min_scale_count, minimum);
                assert_eq!(processor.configuration.max_scale_count, maximum);
                assert_eq!(
                    processor.configuration.max_item_count,
                    MaxItemCountHint::ServerDecides
                );
                match (&processor.configuration.start_from, continuation) {
                    (ProcessorStartFrom::Now, None) => {}
                    (ProcessorStartFrom::Continuation(actual), Some(expected)) => {
                        assert_eq!(actual.as_str(), expected);
                    }
                    _ => panic!("validation changed the initial start position"),
                }
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
async fn mode_start_validation_before_and_after_handler_registration() -> Result<(), Box<dyn Error>>
{
    fn select_start<T, H>(
        builder: ChangeFeedProcessorBuilder<T, H>,
        start: u8,
    ) -> ChangeFeedProcessorBuilder<T, H> {
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
        for register_first in [false, true] {
            for start in 0..4 {
                let builder = ChangeFeedProcessorBuilder::new(
                    mode,
                    monitored.clone(),
                    leases.clone(),
                    "orders",
                    "worker",
                );
                let builder = if register_first {
                    select_start(builder.handle_change_feed(on_changes), start)
                } else {
                    select_start(builder, start).handle_change_feed(on_changes)
                };
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
                    for error in [
                        processor.start().await.unwrap_err(),
                        processor.stop().await.unwrap_err(),
                        processor.current_state().await.unwrap_err(),
                    ] {
                        assert_eq!(error.status().status_code(), StatusCode::NotImplemented);
                    }
                }
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
        )
        .with_start_from_beginning()
        .handle_change_feed(on_changes)
        .with_start_time(OffsetDateTime::UNIX_EPOCH);
        let builder = if use_continuation {
            builder.with_start_continuation(ContinuationToken::from_string("resume".into()))
        } else {
            builder.with_start_from_now()
        };
        let processor = builder.build()?;
        match (&processor.configuration.start_from, use_continuation) {
            (ProcessorStartFrom::Now, false) => {}
            (ProcessorStartFrom::Continuation(token), true) => assert_eq!(token.as_str(), "resume"),
            _ => panic!("the final start position was not preserved"),
        }
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

async fn start_bootstraps_only(
    mode: ChangeFeedMode,
    expected_mode: &str,
) -> Result<(), Box<dyn Error>> {
    let (source, leases) = containers().await?;
    let processor =
        ChangeFeedProcessorBuilder::new(mode, source, leases.clone(), "orders", "worker")
            .handle_change_feed(|_, _: Vec<ChangeFeedItem<String>>| async {
                panic!("bootstrap must not invoke callbacks");
            })
            .build()?;
    assert!(lease_records(&leases).await?.is_empty());
    assert_eq!(
        processor.start().await.unwrap_err().status().status_code(),
        StatusCode::NotImplemented
    );
    assert!(!processor.is_started());
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
        assert!(lease["continuation"].is_null());
    }
    assert_eq!(
        processor.start().await.unwrap_err().status().status_code(),
        StatusCode::NotImplemented
    );
    assert_eq!(lease_records(&leases).await?, before);
    assert!(!processor.is_started());
    Ok(())
}

#[tokio::test]
async fn latest_version_start_bootstraps_only() -> Result<(), Box<dyn Error>> {
    start_bootstraps_only(ChangeFeedMode::LatestVersion, "latestVersion").await
}

#[tokio::test]
async fn all_versions_start_bootstraps_only() -> Result<(), Box<dyn Error>> {
    start_bootstraps_only(
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
    )
    .handle_change_feed(on_changes)
    .build()?;
    assert_eq!(
        first.start().await.unwrap_err().status().status_code(),
        StatusCode::NotImplemented
    );
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
    )
    .with_lease_verification_enabled_on_restart(true)
    .handle_change_feed(on_changes)
    .build()?;
    assert_eq!(
        later.start().await.unwrap_err().status().status_code(),
        StatusCode::NotImplemented
    );
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
    )
    .handle_change_feed(|_, _: Vec<ChangeFeedItem<String>>| async { Ok(()) })
    .build()?;
    assert_eq!(
        first.start().await.unwrap_err().status().status_code(),
        StatusCode::NotImplemented
    );
    let before = lease_records(&leases).await?;
    let other = ChangeFeedProcessorBuilder::new(
        ChangeFeedMode::AllVersionsAndDeletes,
        source,
        leases.clone(),
        "orders",
        "other",
    )
    .handle_change_feed(|_, _: Vec<ChangeFeedItem<String>>| async { Ok(()) })
    .build()?;
    assert_eq!(
        other.start().await.unwrap_err().status().status_code(),
        StatusCode::Conflict
    );
    assert_eq!(lease_records(&leases).await?, before);
    assert!(!other.is_started());
    Ok(())
}
