// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! In-memory-emulator integration tests for cross-partition `DISTINCT`.
//!
//! Unlike the mock-pipeline tests in `driver::dataflow::distinct`, these drive
//! the real path end to end: planner -> query-plan generation -> per-partition
//! execution (including the emulator's own per-partition deduplication) ->
//! client-side `Distinct` stage.
//!
//! Scenarios come from `tests/fixtures/distinct_scenarios.json`, the same
//! source-attributed catalog every other layer reads.

use std::sync::Arc;
use std::time::Duration;

use azure_core::http::Url;
use serde::Deserialize;

use azure_data_cosmos_driver::driver::CosmosDriver;
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig,
    VirtualRegion,
};
use azure_data_cosmos_driver::models::{
    ContainerReference, CosmosOperation, FeedRange, ItemReference, MaxItemCountHint, PartitionKey,
    PartitionKeyDefinition,
};
use azure_data_cosmos_driver::options::{DriverOptions, OperationOptions, PlanOptions};

const GATEWAY_URL: &str = "https://eastus.emulator.local";

const CATALOG_JSON: &str = include_str!("../fixtures/distinct_scenarios.json");

#[derive(Deserialize)]
struct Catalog {
    scenarios: Vec<Scenario>,
}

#[derive(Deserialize)]
struct Scenario {
    id: String,
    layers: Vec<String>,
    query: QuerySpec,
    #[serde(default)]
    documents: Vec<serde_json::Value>,
    #[serde(rename = "pageSizes", default)]
    page_sizes: Vec<u32>,
    #[serde(rename = "expectedIds", default)]
    expected_ids: Vec<String>,
    #[serde(rename = "expectedValues", default)]
    expected_values: Vec<serde_json::Value>,
    checkpoint: Option<serde_json::Value>,
    #[serde(rename = "expectedError")]
    expected_error: Option<ExpectedError>,
}

#[derive(Deserialize)]
struct QuerySpec {
    text: String,
    #[serde(default)]
    parameters: Vec<serde_json::Value>,
    #[serde(rename = "distinctType")]
    distinct_type: String,
}

#[derive(Deserialize)]
struct ExpectedError {
    category: String,
    #[serde(rename = "messageFragment")]
    message_fragment: String,
}

fn catalog() -> Catalog {
    serde_json::from_str(CATALOG_JSON).expect("catalog must parse")
}

/// Builds a two-physical-partition in-memory emulator container and a driver
/// wired to it.
async fn setup() -> (Arc<InMemoryEmulatorHttpClient>, Arc<CosmosDriver>) {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    let store = emulator.store();
    store.create_database("testdb");
    let container_config = ContainerConfig::new()
        .with_partition_count(2)
        .build()
        .unwrap();
    store.create_container_with_config(
        "testdb",
        "testcoll",
        PartitionKeyDefinition::new(vec![std::borrow::Cow::Borrowed("/pk")]),
        container_config,
    );

    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime builds against the in-memory emulator");
    let account = azure_data_cosmos_driver::models::AccountReference::with_master_key(
        Url::parse(GATEWAY_URL).unwrap(),
        "ZW11bGF0b3Ita2V5",
    );
    let driver = runtime
        .create_driver(DriverOptions::builder(account).build())
        .await
        .expect("driver initializes against the emulator");
    (emulator, driver)
}

async fn seed(
    driver: &CosmosDriver,
    container: &ContainerReference,
    documents: &[serde_json::Value],
) {
    for document in documents {
        let id = document["id"]
            .as_str()
            .expect("fixture document needs an id")
            .to_owned();
        let pk = document["pk"]
            .as_str()
            .expect("fixture document needs a pk")
            .to_owned();
        let item_ref = ItemReference::from_name(container, PartitionKey::from(pk), id);
        driver
            .execute_singleton_operation(
                CosmosOperation::create_item(item_ref)
                    .with_body(serde_json::to_vec(document).unwrap()),
                OperationOptions::default(),
            )
            .await
            .expect("seed item created");
    }
}

fn query_body(query: &QuerySpec) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({
        "query": query.text,
        "parameters": query.parameters,
    }))
    .unwrap()
}

fn query_operation(
    container: &ContainerReference,
    query: &QuerySpec,
    page_size: u32,
) -> CosmosOperation {
    CosmosOperation::query_items(container.clone(), Some(FeedRange::full()))
        .with_body(query_body(query))
        .with_max_item_count(MaxItemCountHint::Limit(
            std::num::NonZeroU32::new(page_size.max(1)).unwrap(),
        ))
}

/// Extracts every row from a response body, whichever wire shape it arrived in.
///
/// The cross-partition pipeline emits pre-split `Items`; a raw single-partition
/// backend page arrives as a `{"Documents":[...]}` envelope.
fn documents_of(
    response: azure_data_cosmos_driver::models::CosmosResponse,
) -> Vec<serde_json::Value> {
    use azure_data_cosmos_driver::models::ResponseBody;
    match response.into_body() {
        ResponseBody::NoPayload => Vec::new(),
        ResponseBody::Items(items) => items
            .iter()
            .map(|item| {
                super::parse_json_body(item).expect("item should parse as text or binary JSON")
            })
            .collect(),
        ResponseBody::Bytes(bytes) => {
            let value =
                super::parse_json_body(&bytes).expect("page should parse as text or binary JSON");
            value["Documents"].as_array().cloned().unwrap_or_default()
        }
    }
}

/// Sorts values by their serialized form so an unordered result can be
/// compared deterministically.
fn sorted(mut values: Vec<serde_json::Value>) -> Vec<String> {
    let mut text: Vec<String> = values.drain(..).map(|v| v.to_string()).collect();
    text.sort();
    text
}

fn assert_matches_expected(scenario: &Scenario, actual: Vec<serde_json::Value>) {
    if !scenario.expected_ids.is_empty() {
        let mut ids: Vec<String> = actual
            .iter()
            .map(|v| v["id"].as_str().unwrap_or_default().to_owned())
            .collect();
        ids.sort();
        let mut expected = scenario.expected_ids.clone();
        expected.sort();
        assert_eq!(ids, expected, "scenario {}", scenario.id);
        return;
    }
    if scenario.query.distinct_type == "Ordered" {
        // A matching ORDER BY makes the output order deterministic.
        assert_eq!(
            actual, scenario.expected_values,
            "scenario {} produced the wrong ordered stream",
            scenario.id
        );
    } else {
        assert_eq!(
            sorted(actual),
            sorted(scenario.expected_values.clone()),
            "scenario {} produced the wrong deduplicated set",
            scenario.id
        );
    }
}

/// Drains a query fully, one plan, honoring `page_size`.
async fn drain_all(
    driver: &CosmosDriver,
    container: &ContainerReference,
    query: &QuerySpec,
    page_size: u32,
) -> Vec<serde_json::Value> {
    let mut plan = Box::pin(driver.plan_operation(
        query_operation(container, query, page_size),
        &OperationOptions::default(),
        None,
        &PlanOptions::default(),
    ))
    .await
    .expect("plan builds");

    let mut all = Vec::new();
    while let Some(response) = driver
        .execute_plan(
            &mut plan,
            Some(container.clone()),
            OperationOptions::default(),
        )
        .await
        .expect("page executes")
    {
        all.extend(documents_of(response));
    }
    all
}

/// Catalog-driven: every `inMemoryEmulator` scenario without a checkpoint or an
/// expected error must produce its expected rows, at every declared page size.
#[tokio::test]
async fn catalog_emulator_scenarios_dedupe_as_expected() {
    let mut ran = 0usize;
    for scenario in &catalog().scenarios {
        if !scenario.layers.iter().any(|l| l == "inMemoryEmulator")
            || scenario.checkpoint.is_some()
            || scenario.expected_error.is_some()
        {
            continue;
        }
        let (_emulator, driver) = setup().await;
        let container = driver
            .resolve_container("testdb", "testcoll", OperationOptions::default())
            .await
            .expect("container resolves");
        seed(&driver, &container, &scenario.documents).await;

        let page_sizes = if scenario.page_sizes.is_empty() {
            vec![10]
        } else {
            scenario.page_sizes.clone()
        };
        for page_size in page_sizes {
            let actual = drain_all(&driver, &container, &scenario.query, page_size).await;
            assert_matches_expected(scenario, actual);
        }
        ran += 1;
    }
    // Exact, not a floor: a scenario added to the catalog but silently skipped
    // here would otherwise look covered while never executing.
    let expected = catalog()
        .scenarios
        .iter()
        .filter(|s| {
            s.layers.iter().any(|l| l == "inMemoryEmulator")
                && s.checkpoint.is_none()
                && s.expected_error.is_none()
        })
        .count();
    assert_eq!(ran, expected, "every eligible emulator scenario must run");
    assert!(
        ran >= 8,
        "expected the catalog to drive a meaningful number of emulator scenarios, ran {ran}"
    );
}

/// Catalog-driven: every scenario declaring an `expectedError` must fail with a
/// matching message, rather than silently returning partial or duplicated rows.
#[tokio::test]
async fn catalog_emulator_error_scenarios_fail_as_expected() {
    let mut ran = 0usize;
    for scenario in &catalog().scenarios {
        if !scenario.layers.iter().any(|l| l == "inMemoryEmulator") {
            continue;
        }
        let Some(expected) = &scenario.expected_error else {
            continue;
        };
        let (_emulator, driver) = setup().await;
        let container = driver
            .resolve_container("testdb", "testcoll", OperationOptions::default())
            .await
            .expect("container resolves");
        seed(&driver, &container, &scenario.documents).await;

        let outcome = match expected.category.as_str() {
            // The unsupported-feature check happens while planning.
            "clientUnsupportedQueryFeature" => Box::pin(driver.plan_operation(
                query_operation(&container, &scenario.query, 10),
                &OperationOptions::default(),
                None,
                &PlanOptions::default(),
            ))
            .await
            .err()
            .map(|e| e.to_string()),
            // The continuation refusal happens when the caller mints a token.
            "clientDistinctContinuationUnsupported" => {
                let mut plan = Box::pin(driver.plan_operation(
                    query_operation(&container, &scenario.query, 1),
                    &OperationOptions::default(),
                    None,
                    &PlanOptions::default(),
                ))
                .await
                .expect("an unordered DISTINCT query plans successfully");
                let _ = driver
                    .execute_plan(
                        &mut plan,
                        Some(container.clone()),
                        OperationOptions::default(),
                    )
                    .await
                    .expect("the first page executes");
                plan.to_continuation_token().err().map(|e| e.to_string())
            }
            other => panic!(
                "scenario {} declares unhandled category {other}",
                scenario.id
            ),
        };

        let message = outcome.unwrap_or_else(|| {
            panic!(
                "scenario {} expected a {} failure but the operation succeeded",
                scenario.id, expected.category
            )
        });
        assert!(
            message.contains(&expected.message_fragment),
            "scenario {}: error {message:?} does not contain {:?}",
            scenario.id,
            expected.message_fragment
        );
        ran += 1;
    }
    assert!(
        ran >= 3,
        "expected several emulator error scenarios, ran {ran}"
    );
}

/// An ordered DISTINCT query drained across a serialized continuation token
/// must produce exactly the same rows, in the same order, as a single drain.
///
/// Mirrors .NET `DistinctQueryTests.TestDistinct_ContinuationTokenSupportAsync`
/// and Java `DistinctQueryTests.queryDocumentsWithOrderBy`.
#[tokio::test]
async fn distinct_under_a_window_resumes_across_tokens() {
    // A continuation for `DISTINCT` + `OFFSET`/`LIMIT`/`TOP` nests
    // `SkipTake { child: Distinct { .. } }`. Resuming has to peel and restore
    // both layers in the right order: lose the window's remaining budget and
    // the query over-returns, lose the dedup hash and it repeats the boundary
    // value. Draining page-by-page must equal a single drain either way.
    for scenario_id in [
        "ordered_distinct_top_resume_round_trip",
        "ordered_distinct_offset_limit_resume_round_trip",
    ] {
        let scenarios = catalog();
        let scenario = scenarios
            .scenarios
            .iter()
            .find(|s| s.id == scenario_id)
            .unwrap_or_else(|| panic!("catalog carries {scenario_id}"));

        let (_emulator, driver) = setup().await;
        let container = driver
            .resolve_container("testdb", "testcoll", OperationOptions::default())
            .await
            .expect("container resolves");
        seed(&driver, &container, &scenario.documents).await;

        let single = drain_all(&driver, &container, &scenario.query, 10).await;
        assert_eq!(
            single, scenario.expected_values,
            "{scenario_id}: single drain must honor the window over deduplicated values"
        );

        for &page_size in &scenario.page_sizes {
            let mut resumed = Vec::new();
            let mut token = None;
            loop {
                let mut plan = Box::pin(driver.plan_operation(
                    query_operation(&container, &scenario.query, page_size),
                    &OperationOptions::default(),
                    token.as_ref(),
                    &PlanOptions::default(),
                ))
                .await
                .expect("plan builds (fresh or resumed)");

                let Some(response) = driver
                    .execute_plan(
                        &mut plan,
                        Some(container.clone()),
                        OperationOptions::default(),
                    )
                    .await
                    .expect("page executes")
                else {
                    break;
                };
                resumed.extend(documents_of(response));

                // Once the window is satisfied the pipeline is drained and
                // mints no further token; stop rather than replaying the last.
                match plan.to_continuation_token() {
                    Ok(next) => token = Some(next),
                    Err(_) => break,
                }
                if resumed.len() > scenario.expected_values.len() + 4 {
                    panic!("{scenario_id}: resume loop did not converge; got {resumed:?}");
                }
            }

            assert_eq!(
                resumed, scenario.expected_values,
                "{scenario_id}: resuming at page size {page_size} must not drop, \
                 duplicate, or over-return rows"
            );
        }
    }
}

#[tokio::test]
async fn ordered_distinct_resume_matches_a_single_drain() {
    let scenarios = catalog();
    let scenario = scenarios
        .scenarios
        .iter()
        .find(|s| s.id == "ordered_resume_round_trip_matches_full_drain")
        .expect("catalog carries the ordered resume round-trip scenario");

    let (_emulator, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container resolves");
    seed(&driver, &container, &scenario.documents).await;

    let single = drain_all(&driver, &container, &scenario.query, 10).await;
    assert_eq!(single, scenario.expected_values);

    // Now drain the same query one page at a time, round-tripping a
    // continuation token between every page.
    let mut resumed = Vec::new();
    let mut token = None;
    loop {
        let mut plan = Box::pin(driver.plan_operation(
            query_operation(&container, &scenario.query, 1),
            &OperationOptions::default(),
            token.as_ref(),
            &PlanOptions::default(),
        ))
        .await
        .expect("plan builds (fresh or resumed)");

        let Some(response) = driver
            .execute_plan(
                &mut plan,
                Some(container.clone()),
                OperationOptions::default(),
            )
            .await
            .expect("page executes")
        else {
            break;
        };
        resumed.extend(documents_of(response));

        token = Some(
            plan.to_continuation_token()
                .expect("an ordered DISTINCT query is resumable"),
        );
        if resumed.len() > scenario.expected_values.len() + 4 {
            panic!("resume loop did not converge; got {resumed:?}");
        }
    }

    assert_eq!(
        resumed, scenario.expected_values,
        "resuming across continuation tokens must not drop or duplicate rows"
    );
}

/// Reads the container's physical partition count through the emulator's
/// `pkranges` endpoint.
async fn physical_partition_count(emulator: &InMemoryEmulatorHttpClient) -> usize {
    let url = format!("{GATEWAY_URL}/dbs/testdb/colls/testcoll/pkranges");
    let request =
        azure_core::http::Request::new(Url::parse(&url).unwrap(), azure_core::http::Method::Get);
    let response = emulator.execute_request(&request).await.unwrap();
    let raw = response.try_into_raw_response().await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(raw.body().as_ref()).unwrap();
    body["PartitionKeyRanges"]
        .as_array()
        .expect("pkranges response carries a PartitionKeyRanges array")
        .len()
}

/// A partition split partway through a DISTINCT drain must not resurrect an
/// already-emitted value. Neither .NET nor Java covers this for DISTINCT.
#[tokio::test]
async fn split_mid_drain_does_not_reemit_deduplicated_values() {
    for scenario_id in [
        "split_mid_query_unordered_does_not_reemit",
        "split_mid_query_ordered_does_not_reemit",
        // `DISTINCT` under a window: the split must preserve both the dedup
        // state and the window's remaining budget.
        "split_mid_query_windowed_distinct_does_not_reemit",
    ] {
        let scenarios = catalog();
        let scenario = scenarios
            .scenarios
            .iter()
            .find(|s| s.id == scenario_id)
            .expect("catalog carries the split scenario");

        let (emulator, driver) = setup().await;
        let container = driver
            .resolve_container("testdb", "testcoll", OperationOptions::default())
            .await
            .expect("container resolves");
        seed(&driver, &container, &scenario.documents).await;

        let mut plan = Box::pin(driver.plan_operation(
            query_operation(&container, &scenario.query, 1),
            &OperationOptions::default(),
            None,
            &PlanOptions::default(),
        ))
        .await
        .expect("plan builds");

        let ranges_before = physical_partition_count(&emulator).await;

        let mut all = Vec::new();
        let mut pages = 0usize;
        let mut pages_after_split = 0usize;
        while let Some(response) = driver
            .execute_plan(
                &mut plan,
                Some(container.clone()),
                OperationOptions::default(),
            )
            .await
            .expect("page executes")
        {
            all.extend(documents_of(response));
            pages += 1;
            if pages == 1 {
                // Split the first physical partition mid-drain. The fan-out
                // root absorbs it; `Distinct` is never rebuilt, so its
                // deduplication state has to survive.
                emulator
                    .store()
                    .split_partition("testdb", "testcoll", 0, Duration::ZERO);
                // Deterministic completion rather than a sleep, so the test
                // cannot flake on split timing.
                emulator.store().drain_pending_control_plane().await;

                // Without this the test could pass while covering nothing: a
                // change to split timing or page shape would silently turn the
                // split into a no-op.
                let ranges_after = physical_partition_count(&emulator).await;
                assert!(
                    ranges_after > ranges_before,
                    "{scenario_id}: the split must actually change the topology \
                     (before={ranges_before}, after={ranges_after})"
                );
            } else if pages > 1 {
                pages_after_split += 1;
            }
        }

        assert!(
            pages_after_split > 0,
            "{scenario_id}: the drain must continue past the split for the test to prove anything"
        );
        assert_matches_expected(scenario, all);
    }
}
