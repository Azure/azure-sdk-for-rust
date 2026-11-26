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
    models::{AccountReference, CosmosOperation, FeedRange, PartitionKeyDefinition, ResponseBody},
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
        AccountReference::with_account_key(Url::parse(GATEWAY_URL).unwrap(), "ZW11bGF0b3Ita2V5");
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

// Native plans this query and runs it as-is; only the local planner
// short-circuits contradictory filters.
#[cfg(not(feature = "__internal_native_query_plan"))]
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

/// Whether the driver uses the native planner: the feature is enabled and
/// `azure_cosmos_queryplan_sys` bundles a library for this target.
const NATIVE_PLANNER: bool = cfg!(all(
    feature = "__internal_native_query_plan",
    any(
        target_os = "windows",
        target_os = "linux",
        target_os = "macos"
    ),
    any(target_arch = "x86_64", target_arch = "aarch64"),
    not(target_env = "musl")
));

/// Expected Gateway query-plan requests: one, unless the native planner
/// handles the query without falling back.
fn expected_gateway_plans(handled_by_native: bool) -> usize {
    if NATIVE_PLANNER && handled_by_native {
        0
    } else {
        1
    }
}

#[tokio::test]
async fn advanced_families_plan_natively_or_fall_back_once() {
    let (_emulator, recorder, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();

    // (name, query, expected outcome, handled by native planner)
    let scenarios = [
        (
            "distinct_count",
            "SELECT VALUE COUNT(1) FROM (SELECT DISTINCT VALUE c.pk FROM c) AS t",
            FallbackOutcome::Plan,
            true,
        ),
        (
            "subroot",
            "SELECT VALUE x FROM c.child x",
            FallbackOutcome::Plan,
            true,
        ),
        (
            "reserved_alias",
            "SELECT COUNT(1) AS OFFSET FROM c",
            FallbackOutcome::Error,
            false,
        ),
        (
            "dcount",
            "SELECT VALUE DCOUNT(c.pk) FROM c",
            FallbackOutcome::Plan,
            false,
        ),
        (
            "hybrid",
            "SELECT VALUE RRF(c.score) FROM c",
            FallbackOutcome::Plan,
            false,
        ),
        (
            "rewrite_unavailable",
            "SELECT VALUE 1 ORDER BY 1",
            // Native plans this query; without it, planning fails.
            if NATIVE_PLANNER {
                FallbackOutcome::Plan
            } else {
                FallbackOutcome::Error
            },
            true,
        ),
    ];

    for (name, text, expected, handled_by_native) in scenarios {
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
            expected_gateway_plans(handled_by_native),
            "{name} must issue the expected number of Gateway query-plan requests"
        );
        match expected {
            FallbackOutcome::Plan => {
                assert!(result.is_ok(), "{name} should produce a plan");
            }

            FallbackOutcome::Error => {
                assert!(result.is_err(), "{name} should surface a planning error");
            }
        }
    }
}

#[tokio::test]
async fn aggregate_and_group_by_skip_gateway_planning() {
    let (_emulator, recorder, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();
    for sql in [
        "SELECT VALUE COUNT(1) FROM c",
        "SELECT AVG(c.price) AS mean FROM c",
        "SELECT c.pk, COUNT(1) FROM c GROUP BY c.pk",
        "SELECT VALUE COUNT(1) FROM c JOIN t IN c.tags",
    ] {
        recorder.clear();
        driver
            .plan_operation(
                query(&container, sql),
                &OperationOptions::default(),
                None,
                &PlanOptions::default(),
            )
            .await
            .unwrap();
        assert_eq!(recorder.query_plan_count(), 0, "{sql}");
    }
}

#[tokio::test]
async fn vector_queries_plan_natively_or_fall_back_once() {
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
            expected_gateway_plans(true),
            "vector query must issue the expected number of Gateway query-plan requests"
        );
        if NATIVE_PLANNER {
            assert!(result.is_ok(), "native should plan the vector query");
        } else {
            assert!(
                result.is_err(),
                "the in-memory Gateway cannot synthesize authoritative vector metadata"
            );
        }
        assert_eq!(recorder.document_query_count(), 0);
    }
}

/// Plans and drains `text`, returning every row or the first error.
async fn run_query(
    driver: &CosmosDriver,
    container: &azure_data_cosmos_driver::models::ContainerReference,
    text: &str,
) -> azure_data_cosmos_driver::error::Result<Vec<serde_json::Value>> {
    let mut plan = driver
        .plan_operation(
            query(container, text),
            &OperationOptions::default(),
            None,
            &PlanOptions::default(),
        )
        .await?;
    let mut rows = Vec::new();
    while let Some(response) = driver
        .execute_plan(
            &mut plan,
            Some(container.clone()),
            OperationOptions::default(),
        )
        .await?
    {
        match response.into_body() {
            ResponseBody::NoPayload => {}
            ResponseBody::Items(items) => rows.extend(
                items
                    .iter()
                    .map(|item| super::parse_json_body(item).expect("item should parse")),
            ),
            ResponseBody::Bytes(bytes) => {
                let page = super::parse_json_body(&bytes).expect("page should parse");
                rows.extend(page["Documents"].as_array().cloned().unwrap_or_default());
            }
        }
    }
    Ok(rows)
}

#[tokio::test]
async fn empty_range_plans_return_correct_rows() {
    let (_emulator, recorder, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();

    // Native returns the same empty range for all of these and the driver
    // runs that plan as-is, like a Gateway plan; only the local planner
    // short-circuits contradictory filters. The in-memory emulator returns no
    // rows for `SELECT VALUE 1`, so assert it is executed.
    recorder.clear();
    run_query(&driver, &container, "SELECT VALUE 1")
        .await
        .unwrap();
    assert!(
        recorder.document_query_count() > 0,
        "SELECT VALUE 1 must not be short-circuited"
    );

    for text in [
        "SELECT VALUE COUNT(1) FROM c WHERE false",
        "SELECT COUNT(1) AS n FROM c WHERE c.pk = 'a' AND c.pk = 'b'",
    ] {
        match run_query(&driver, &container, text).await {
            Ok(rows) => assert!(!rows.is_empty(), "{text} must not return an empty success"),
            Err(_) => {} // explicit unsupported-aggregate error
        }
    }

    recorder.clear();
    let rows = run_query(
        &driver,
        &container,
        "SELECT * FROM c WHERE c.pk = 'a' AND c.pk = 'b'",
    )
    .await
    .unwrap();
    assert!(rows.is_empty());
    assert_eq!(recorder.query_plan_count(), 0);
    if !NATIVE_PLANNER {
        assert_eq!(recorder.document_query_count(), 0);
    }
}
