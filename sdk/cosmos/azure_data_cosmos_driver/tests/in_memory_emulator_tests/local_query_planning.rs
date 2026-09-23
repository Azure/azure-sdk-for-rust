// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Production local-query-plan provider selection tests.

use std::{borrow::Cow, sync::Arc};

use azure_core::http::Url;
use azure_data_cosmos_driver::{
    driver::CosmosDriver,
    in_memory_emulator::{
        ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig,
        VirtualRegion,
    },
    models::{AccountReference, CosmosOperation, FeedRange, PartitionKeyDefinition},
    options::{DriverOptions, OperationOptions, PlanOptions, QueryPlanMode},
};

use super::{host_recorder::HostRecorder, GATEWAY_URL};

async fn setup() -> (
    Arc<InMemoryEmulatorHttpClient>,
    Arc<HostRecorder>,
    Arc<CosmosDriver>,
) {
    let recorder = HostRecorder::new();
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);
    let emulator =
        Arc::new(InMemoryEmulatorHttpClient::new(config).with_request_observer(recorder.clone()));
    emulator.store().create_database("testdb");
    emulator.store().create_container_with_config(
        "testdb",
        "testcoll",
        PartitionKeyDefinition::new(vec![Cow::Borrowed("/pk")]),
        ContainerConfig::new()
            .with_partition_count(2)
            .build()
            .unwrap(),
    );

    let runtime = emulator.runtime_builder().build().await.unwrap();
    let account =
        AccountReference::with_master_key(Url::parse(GATEWAY_URL).unwrap(), "ZW11bGF0b3Ita2V5");
    let driver = runtime
        .create_driver(DriverOptions::builder(account).build())
        .await
        .unwrap();
    (emulator, recorder, driver)
}

#[tokio::test]
async fn per_request_gateway_only_mode_bypasses_local_query_planning() {
    let (_emulator, recorder, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();
    recorder.clear();
    let options = PlanOptions::default().with_query_plan_mode(QueryPlanMode::GatewayOnly);

    driver
        .plan_operation(
            query(&container, "SELECT * FROM c WHERE c.pk = 'a'"),
            &OperationOptions::default(),
            None,
            &options,
        )
        .await
        .unwrap();
    assert_eq!(recorder.query_plan_count(), 1);
}

#[cfg(not(feature = "__internal_native_query_plan"))]
#[tokio::test]
async fn per_plan_mode_does_not_leak_to_subsequent_queries() {
    let (_emulator, recorder, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();
    recorder.clear();
    driver
        .plan_operation(
            query(&container, "SELECT * FROM c WHERE c.pk = 'a'"),
            &OperationOptions::default(),
            None,
            &PlanOptions::default().with_query_plan_mode(QueryPlanMode::GatewayOnly),
        )
        .await
        .unwrap();
    assert_eq!(recorder.query_plan_count(), 1);
    recorder.clear();
    driver
        .plan_operation(
            query(&container, "SELECT * FROM c WHERE c.pk = 'a'"),
            &OperationOptions::default(),
            None,
            &PlanOptions::default(),
        )
        .await
        .unwrap();

    assert_eq!(recorder.query_plan_count(), 0);
}

fn query(
    container: &azure_data_cosmos_driver::models::ContainerReference,
    text: &str,
) -> CosmosOperation {
    query_with_parameters(container, text, serde_json::json!([]))
}

fn query_with_parameters(
    container: &azure_data_cosmos_driver::models::ContainerReference,
    text: &str,
    parameters: serde_json::Value,
) -> CosmosOperation {
    CosmosOperation::query_items(container.clone(), Some(FeedRange::full())).with_body(
        serde_json::to_vec(&serde_json::json!({
            "query": text,
            "parameters": parameters,
        }))
        .unwrap(),
    )
}

#[cfg(not(feature = "__internal_native_query_plan"))]
#[tokio::test]
async fn eligible_query_skips_gateway_query_plan() {
    let (_emulator, recorder, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();
    recorder.clear();

    let mut plan = driver
        .plan_operation(
            query(&container, "SELECT * FROM c WHERE c.pk = 'a'"),
            &OperationOptions::default(),
            None,
            &PlanOptions::default(),
        )
        .await
        .unwrap();
    assert_eq!(recorder.query_plan_count(), 0);
    assert_eq!(recorder.routing_metadata_count(), 0);

    while driver
        .execute_plan(
            &mut plan,
            Some(container.clone()),
            OperationOptions::default(),
        )
        .await
        .unwrap()
        .is_some()
    {}
    assert_eq!(recorder.query_plan_count(), 0);
    assert_eq!(recorder.document_query_count(), 1);
}

#[tokio::test]
async fn contradictory_query_short_circuits_all_query_io() {
    let (_emulator, recorder, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();
    recorder.clear();

    let mut plan = driver
        .plan_operation(
            query(
                &container,
                "SELECT * FROM c WHERE c.pk = 'a' AND c.pk = 'b'",
            ),
            &OperationOptions::default(),
            None,
            &PlanOptions::default(),
        )
        .await
        .unwrap();
    assert_eq!(recorder.query_plan_count(), 0);
    assert_eq!(recorder.routing_metadata_count(), 0);
    assert_eq!(recorder.document_query_count(), 0);
    assert!(driver
        .execute_plan(&mut plan, Some(container), OperationOptions::default(),)
        .await
        .unwrap()
        .is_none());
    assert_eq!(recorder.query_plan_count(), 0);
    assert_eq!(recorder.routing_metadata_count(), 0);
    assert_eq!(recorder.document_query_count(), 0);
}

#[tokio::test]
async fn contradictory_buffered_query_is_exempt_from_zero_window() {
    let (_emulator, recorder, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();
    recorder.clear();
    let mut plan = driver
        .plan_operation(
            query(
                &container,
                "SELECT DISTINCT VALUE c.value FROM c WHERE c.pk = 'a' AND c.pk = 'b'",
            ),
            &OperationOptions::default(),
            None,
            &PlanOptions::default().with_max_buffered_query_window(0),
        )
        .await
        .unwrap();
    assert!(driver
        .execute_plan(&mut plan, Some(container), OperationOptions::default())
        .await
        .unwrap()
        .is_none());
    assert_eq!(recorder.query_plan_count(), 0);
    assert_eq!(recorder.routing_metadata_count(), 0);
    assert_eq!(recorder.document_query_count(), 0);
}

#[tokio::test]
async fn supported_order_by_skips_gateway_query_plan() {
    let (_emulator, recorder, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();
    recorder.clear();

    let mut plan = driver
        .plan_operation(
            query(
                &container,
                "SELECT c.pk, c.id AS itemId FROM c ORDER BY c.pk",
            ),
            &OperationOptions::default(),
            None,
            &PlanOptions::default(),
        )
        .await
        .unwrap();
    assert_eq!(recorder.query_plan_count(), 0);
    while driver
        .execute_plan(
            &mut plan,
            Some(container.clone()),
            OperationOptions::default(),
        )
        .await
        .unwrap()
        .is_some()
    {}
    assert_eq!(recorder.query_plan_count(), 0);
}

#[derive(Clone, Copy)]
enum FallbackOutcome {
    Plan,
    Error,
}

#[tokio::test]
async fn unsupported_families_fall_back_to_gateway_once() {
    let (_emulator, recorder, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();

    let scenarios = [
        (
            "aggregate",
            "SELECT VALUE COUNT(1) FROM c",
            FallbackOutcome::Error,
        ),
        (
            "group_by",
            "SELECT c.pk, COUNT(1) FROM c GROUP BY c.pk",
            FallbackOutcome::Error,
        ),
        (
            "dcount",
            "SELECT VALUE DCOUNT(c.pk) FROM c",
            FallbackOutcome::Plan,
        ),
        (
            "hybrid",
            "SELECT VALUE RRF(c.score) FROM c",
            FallbackOutcome::Plan,
        ),
        (
            "rewrite_unavailable",
            "SELECT VALUE 1 ORDER BY 1",
            FallbackOutcome::Error,
        ),
    ];

    for (name, text, expected) in scenarios {
        recorder.clear();
        let result = driver
            .plan_operation(
                query(&container, text),
                &OperationOptions::default(),
                None,
                &PlanOptions::default(),
            )
            .await;

        assert_eq!(
            recorder.query_plan_count(),
            1,
            "{name} must issue exactly one Gateway query-plan request"
        );
        match expected {
            FallbackOutcome::Plan => {
                assert!(result.is_ok(), "{name} should produce a Gateway plan");
            }

            FallbackOutcome::Error => {
                assert!(result.is_err(), "{name} should surface a planning error");
            }
        }
    }
}

#[tokio::test]
async fn vector_queries_fall_back_to_gateway_once() {
    let (_emulator, recorder, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();

    let scenarios = [
        query(
            &container,
            "SELECT TOP 5 c.id FROM c \
             ORDER BY VectorDistance(c.vector, [0.1, 0.2])",
        ),
        query_with_parameters(
            &container,
            "SELECT VALUE c.id FROM c \
             ORDER BY VectorDistance(c.vector, @vector, true) \
             OFFSET @offset LIMIT @limit",
            serde_json::json!([
                {"name": "@vector", "value": [0.1, 0.2]},
                {"name": "@offset", "value": 2},
                {"name": "@limit", "value": 3}
            ]),
        ),
    ];

    for operation in scenarios {
        recorder.clear();
        let result = driver
            .plan_operation(
                operation,
                &OperationOptions::default(),
                None,
                &PlanOptions::default(),
            )
            .await;

        assert_eq!(
            recorder.query_plan_count(),
            1,
            "vector query must issue exactly one Gateway query-plan request"
        );
        assert!(
            result.is_err(),
            "the in-memory Gateway cannot synthesize authoritative vector metadata"
        );
        assert_eq!(recorder.document_query_count(), 0);
    }
}
