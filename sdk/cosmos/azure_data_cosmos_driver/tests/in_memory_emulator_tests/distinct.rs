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

use std::sync::{Arc, Mutex};
use std::time::Duration;

use azure_core::http::{headers::HeaderName, Request, Url};
use serde::Deserialize;

use azure_data_cosmos_driver::driver::CosmosDriver;
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, RequestObserver,
    VirtualAccountConfig, VirtualRegion,
};
use azure_data_cosmos_driver::models::{
    ContainerReference, CosmosOperation, FeedRange, ItemReference, MaxItemCountHint, PartitionKey,
    PartitionKeyDefinition,
};
use azure_data_cosmos_driver::options::{
    BinaryEncodingOptions, DriverOptions, OperationOptions, OperationOptionsBuilder, PlanOptions,
};

const GATEWAY_URL: &str = "https://eastus.emulator.local";
static IS_QUERY: HeaderName = HeaderName::from_static("x-ms-documentdb-isquery");
static IS_QUERY_PLAN: HeaderName = HeaderName::from_static("x-ms-cosmos-is-query-plan-request");
static SUPPORTED_FORMATS: HeaderName =
    HeaderName::from_static("x-ms-cosmos-supported-serialization-formats");

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

#[derive(Debug, Default)]
struct QueryRequestRecorder {
    binary_modes: Mutex<Vec<bool>>,
}

impl QueryRequestRecorder {
    fn take(&self) -> Vec<bool> {
        std::mem::take(&mut *self.binary_modes.lock().unwrap())
    }
}

impl RequestObserver for QueryRequestRecorder {
    fn on_request(&self, request: &Request) {
        let headers = request.headers();
        let is_true = |name: &HeaderName| {
            headers
                .get_optional_str(name)
                .is_some_and(|value| value.eq_ignore_ascii_case("true"))
        };
        if !is_true(&IS_QUERY) || is_true(&IS_QUERY_PLAN) {
            return;
        }
        let binary = headers
            .get_optional_str(&SUPPORTED_FORMATS)
            .is_some_and(|value| {
                value
                    .split(',')
                    .any(|format| format.trim().eq_ignore_ascii_case("CosmosBinary"))
            });
        self.binary_modes.lock().unwrap().push(binary);
    }
}

/// Builds a two-physical-partition in-memory emulator container and a driver
/// wired to it.
async fn setup() -> (Arc<InMemoryEmulatorHttpClient>, Arc<CosmosDriver>) {
    setup_with_observer(None).await
}

async fn setup_with_observer(
    observer: Option<Arc<dyn RequestObserver>>,
) -> (Arc<InMemoryEmulatorHttpClient>, Arc<CosmosDriver>) {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    let emulator = InMemoryEmulatorHttpClient::new(config);
    let emulator = match observer {
        Some(observer) => emulator.with_request_observer(observer),
        None => emulator,
    };
    let emulator = Arc::new(emulator);
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

async fn setup_with_query_recorder() -> (
    Arc<InMemoryEmulatorHttpClient>,
    Arc<CosmosDriver>,
    Arc<QueryRequestRecorder>,
) {
    let recorder = Arc::new(QueryRequestRecorder::default());
    let observer: Arc<dyn RequestObserver> = recorder.clone();
    let (emulator, driver) = setup_with_observer(Some(observer)).await;
    (emulator, driver, recorder)
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

/// Extracts every `Documents` item from a response body.
fn documents_of(
    response: azure_data_cosmos_driver::models::CosmosResponse,
) -> Vec<serde_json::Value> {
    let Ok(bytes) = response.into_body().single() else {
        return Vec::new();
    };
    let value: serde_json::Value = if azure_data_cosmos_driver::binary_json::is_binary(&bytes) {
        azure_data_cosmos_driver::binary_json::from_slice(&bytes).unwrap()
    } else {
        serde_json::from_slice(&bytes).unwrap()
    };
    value["Documents"].as_array().cloned().unwrap_or_default()
}

async fn drain_query_with_options(
    driver: &CosmosDriver,
    container: &ContainerReference,
    query: &QuerySpec,
    planning_options: OperationOptions,
    execution_options: OperationOptions,
) -> (Vec<serde_json::Value>, Vec<bool>) {
    let mut plan = Box::pin(driver.plan_operation(
        query_operation(container, query, 1),
        &planning_options,
        None,
        &PlanOptions::default(),
    ))
    .await
    .unwrap();
    let mut values = Vec::new();
    let mut formats = Vec::new();

    while let Some(response) = driver
        .execute_plan(
            &mut plan,
            Some(container.clone()),
            execution_options.clone(),
        )
        .await
        .unwrap()
    {
        let bytes = response.body().clone().single().unwrap();
        let is_binary = azure_data_cosmos_driver::binary_json::is_binary(&bytes);
        formats.push(is_binary);
        values.extend(documents_of(response));
    }

    (values, formats)
}

async fn drain_with_cross_encoding_resume(
    driver: &CosmosDriver,
    container: &ContainerReference,
    query: &QuerySpec,
    mint_options: OperationOptions,
    resume_options: OperationOptions,
) -> Vec<serde_json::Value> {
    let mut plan = Box::pin(driver.plan_operation(
        query_operation(container, query, 1),
        &mint_options,
        None,
        &PlanOptions::default(),
    ))
    .await
    .unwrap();
    let first = driver
        .execute_plan(&mut plan, Some(container.clone()), mint_options)
        .await
        .unwrap()
        .expect("ordered query must emit a first page");
    let mut values = documents_of(first);
    let token = plan.to_continuation_token().unwrap();

    let mut resumed = Box::pin(driver.plan_operation(
        query_operation(container, query, 1),
        &resume_options,
        Some(&token),
        &PlanOptions::default(),
    ))
    .await
    .unwrap();
    while let Some(response) = driver
        .execute_plan(
            &mut resumed,
            Some(container.clone()),
            resume_options.clone(),
        )
        .await
        .unwrap()
    {
        values.extend(documents_of(response));
    }
    values
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
            .resolve_container("testdb", "testcoll")
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
            .resolve_container("testdb", "testcoll")
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
async fn ordered_distinct_resume_matches_a_single_drain() {
    let scenarios = catalog();
    let scenario = scenarios
        .scenarios
        .iter()
        .find(|s| s.id == "ordered_resume_round_trip_matches_full_drain")
        .expect("catalog carries the ordered resume round-trip scenario");

    let (_emulator, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll")
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
    ] {
        let scenarios = catalog();
        let scenario = scenarios
            .scenarios
            .iter()
            .find(|s| s.id == scenario_id)
            .expect("catalog carries the split scenario");

        let (emulator, driver) = setup().await;
        let container = driver
            .resolve_container("testdb", "testcoll")
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

#[tokio::test]
async fn text_and_binary_query_pages_have_pipeline_parity() {
    let (_emulator, driver, recorder) = setup_with_query_recorder().await;
    let container = driver
        .resolve_container("testdb", "testcoll")
        .await
        .unwrap();
    let documents: Vec<serde_json::Value> = [
        ("a", "pk1", 1),
        ("b", "pk2", 1),
        ("c", "pk3", 2),
        ("d", "pk4", 2),
        ("e", "pk5", 3),
    ]
    .into_iter()
    .map(|(id, pk, value)| serde_json::json!({"id": id, "pk": pk, "value": value}))
    .collect();
    seed(&driver, &container, &documents).await;

    for query in [
        QuerySpec {
            text: "SELECT * FROM c".to_owned(),
            parameters: Vec::new(),
            distinct_type: "None".to_owned(),
        },
        QuerySpec {
            text: "SELECT DISTINCT VALUE c.value FROM c".to_owned(),
            parameters: Vec::new(),
            distinct_type: "Unordered".to_owned(),
        },
        QuerySpec {
            text: "SELECT DISTINCT VALUE c.value FROM c ORDER BY c.value".to_owned(),
            parameters: Vec::new(),
            distinct_type: "Ordered".to_owned(),
        },
    ] {
        let (text, text_formats) = drain_query_with_options(
            &driver,
            &container,
            &query,
            OperationOptions::default(),
            OperationOptions::default(),
        )
        .await;
        let text_request_modes = recorder.take();
        let binary_options = OperationOptionsBuilder::new()
            .with_binary_encoding(BinaryEncodingOptions::new().with_enabled(true))
            .build();
        let (binary, binary_formats) = drain_query_with_options(
            &driver,
            &container,
            &query,
            binary_options.clone(),
            binary_options.clone(),
        )
        .await;
        let binary_request_modes = recorder.take();
        let binary_as_text_options = OperationOptionsBuilder::new()
            .with_binary_encoding(
                BinaryEncodingOptions::new()
                    .with_enabled(true)
                    .with_request_text_response(true),
            )
            .build();
        let (binary_as_text, binary_as_text_formats) = drain_query_with_options(
            &driver,
            &container,
            &query,
            binary_as_text_options.clone(),
            binary_as_text_options,
        )
        .await;
        let binary_as_text_request_modes = recorder.take();
        let (_, planned_binary_formats) = drain_query_with_options(
            &driver,
            &container,
            &query,
            binary_options.clone(),
            OperationOptions::default(),
        )
        .await;
        let planned_binary_request_modes = recorder.take();
        let (_, planned_text_formats) = drain_query_with_options(
            &driver,
            &container,
            &query,
            OperationOptions::default(),
            binary_options,
        )
        .await;
        let planned_text_request_modes = recorder.take();

        if query.distinct_type == "Ordered" {
            assert_eq!(binary, text, "{}", query.text);
            assert_eq!(binary_as_text, text, "{}", query.text);
        } else {
            assert_eq!(sorted(binary), sorted(text.clone()), "{}", query.text);
            assert_eq!(sorted(binary_as_text), sorted(text), "{}", query.text);
        }
        assert!(
            text_formats.iter().all(|is_binary| !is_binary),
            "{}",
            query.text
        );
        assert!(
            binary_formats.iter().all(|is_binary| *is_binary),
            "{}",
            query.text
        );
        assert!(
            !binary_request_modes.is_empty()
                && binary_request_modes.iter().all(|is_binary| *is_binary),
            "binary negotiation must reach per-partition requests: {}",
            query.text
        );
        assert!(
            !text_request_modes.is_empty() && text_request_modes.iter().all(|is_binary| !is_binary),
            "text requests must not advertise binary: {}",
            query.text
        );
        assert!(
            binary_as_text_formats.iter().all(|is_binary| !is_binary),
            "{}",
            query.text
        );
        assert!(
            !binary_as_text_request_modes.is_empty()
                && binary_as_text_request_modes
                    .iter()
                    .all(|is_binary| *is_binary),
            "text-response mode must still negotiate a binary wire: {}",
            query.text
        );
        assert!(
            planned_binary_formats.iter().all(|is_binary| *is_binary),
            "planning must own response encoding even when execution options differ: {}",
            query.text
        );
        assert!(
            !planned_binary_request_modes.is_empty()
                && planned_binary_request_modes
                    .iter()
                    .all(|is_binary| *is_binary),
            "binary planning must own wire negotiation: {}",
            query.text
        );
        assert!(
            planned_text_formats.iter().all(|is_binary| !is_binary),
            "execution options must not enable binary after text planning: {}",
            query.text
        );
        assert!(
            !planned_text_request_modes.is_empty()
                && planned_text_request_modes
                    .iter()
                    .all(|is_binary| !is_binary),
            "binary execution options must not change text-planned negotiation: {}",
            query.text
        );
    }
}

#[tokio::test]
async fn ordered_distinct_tokens_resume_across_binary_modes() {
    let (_emulator, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll")
        .await
        .unwrap();
    let documents: Vec<serde_json::Value> = [("a", "pk1", 1), ("b", "pk2", 2), ("c", "pk3", 3)]
        .into_iter()
        .map(|(id, pk, value)| serde_json::json!({"id": id, "pk": pk, "value": value}))
        .collect();
    seed(&driver, &container, &documents).await;
    let binary = OperationOptionsBuilder::new()
        .with_binary_encoding(BinaryEncodingOptions::new().with_enabled(true))
        .build();
    for (query, expected) in [
        (
            QuerySpec {
                text: "SELECT DISTINCT VALUE c.value FROM c ORDER BY c.value".to_owned(),
                parameters: Vec::new(),
                distinct_type: "Ordered".to_owned(),
            },
            vec![
                serde_json::json!(1),
                serde_json::json!(2),
                serde_json::json!(3),
            ],
        ),
        (
            QuerySpec {
                text: "SELECT DISTINCT VALUE c.value FROM c WHERE c.value >= @min ORDER BY c.value"
                    .to_owned(),
                parameters: vec![serde_json::json!({"name": "@min", "value": 2})],
                distinct_type: "Ordered".to_owned(),
            },
            vec![serde_json::json!(2), serde_json::json!(3)],
        ),
    ] {
        assert_eq!(
            drain_with_cross_encoding_resume(
                &driver,
                &container,
                &query,
                OperationOptions::default(),
                binary.clone(),
            )
            .await,
            expected,
            "text-minted token must resume in binary mode: {}",
            query.text
        );
        assert_eq!(
            drain_with_cross_encoding_resume(
                &driver,
                &container,
                &query,
                binary.clone(),
                OperationOptions::default(),
            )
            .await,
            expected,
            "binary-minted token must resume in text mode: {}",
            query.text
        );
    }
}
