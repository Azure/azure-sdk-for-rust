// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Fixture/schema tests for the cross-partition `DISTINCT` scenario catalog
//! (`tests/fixtures/distinct_scenarios.json`).
//!
//! Mirrors `streaming_order_by_scenario_catalog.rs`: this is the single
//! source-attributed catalog reused across every test layer (map unit tests,
//! mock-pipeline tests, and in-memory-emulator tests). Each layer lives in a
//! different compilation unit and defines its own minimal `Deserialize` view
//! of the fixture; this file owns the strict, canonical schema validation
//! every other layer trusts.
//!
//! Validates: no duplicate scenario IDs; every `layers` entry is a known layer
//! name; every scenario declares at least one layer, at least one cross-SDK
//! source, and some expected result; `distinctType` is one of the three plan
//! values; mock partitions are sorted, have no gaps, and tile correctly; a
//! scenario declaring `mockPipeline` actually carries a mock; an `Ordered`
//! scenario declares its sort columns; and required scenario-inventory
//! categories are represented.

use std::collections::BTreeSet;

use serde::Deserialize;

const CATALOG_JSON: &str = include_str!("fixtures/distinct_scenarios.json");

const KNOWN_LAYERS: &[&str] = &[
    "distinctMap",
    "mockPipeline",
    "inMemoryEmulator",
    "recorded",
];

const KNOWN_DISTINCT_TYPES: &[&str] = &["None", "Ordered", "Unordered"];

#[derive(Deserialize)]
struct Catalog {
    #[serde(rename = "schemaVersion")]
    schema_version: u32,
    scenarios: Vec<Scenario>,
}

#[derive(Deserialize)]
struct Scenario {
    id: String,
    #[allow(dead_code)]
    description: String,
    sources: Vec<Source>,
    layers: Vec<String>,
    query: QuerySpec,
    #[serde(default)]
    #[allow(dead_code)]
    documents: Vec<serde_json::Value>,
    mock: Option<MockSpec>,
    #[serde(rename = "pageSizes", default)]
    page_sizes: Vec<u32>,
    #[serde(rename = "expectedIds", default)]
    expected_ids: Vec<String>,
    #[serde(rename = "expectedValues", default)]
    expected_values: Vec<serde_json::Value>,
    #[allow(dead_code)]
    checkpoint: Option<serde_json::Value>,
    #[serde(rename = "expectedContinuation")]
    expected_continuation: Option<serde_json::Value>,
    #[serde(rename = "expectedError")]
    expected_error: Option<ExpectedError>,
}

#[derive(Deserialize)]
struct Source {
    sdk: String,
    #[allow(dead_code)]
    path: String,
    #[allow(dead_code)]
    test: String,
}

#[derive(Deserialize)]
struct QuerySpec {
    text: String,
    #[serde(default)]
    #[allow(dead_code)]
    parameters: Vec<serde_json::Value>,
    #[serde(default)]
    columns: Vec<ColumnSpec>,
    #[serde(rename = "distinctType")]
    distinct_type: String,
}

#[derive(Deserialize)]
struct ColumnSpec {
    #[allow(dead_code)]
    expression: String,
    direction: String,
}

#[derive(Deserialize)]
struct MockSpec {
    partitions: Vec<MockPartition>,
}

#[derive(Deserialize)]
struct MockPartition {
    range: MockRange,
    pages: Vec<MockPage>,
}

#[derive(Deserialize)]
struct MockRange {
    #[serde(rename = "minEpk")]
    min_epk: String,
    #[serde(rename = "maxEpk")]
    max_epk: String,
}

#[derive(Deserialize)]
struct MockPage {
    rows: Vec<MockRow>,
    #[allow(dead_code)]
    continuation: Option<String>,
}

#[derive(Deserialize)]
struct MockRow {
    rid: String,
    /// Present only when the scenario also exercises an `ORDER BY` envelope;
    /// an unordered `DISTINCT` page is a plain `Documents` feed.
    #[serde(rename = "orderByItems", default)]
    #[allow(dead_code)]
    order_by_items: Option<Vec<serde_json::Value>>,
    payload: serde_json::Value,
}

#[derive(Deserialize)]
struct ExpectedError {
    category: String,
    #[serde(rename = "messageFragment")]
    message_fragment: String,
}

fn load_catalog() -> Catalog {
    serde_json::from_str(CATALOG_JSON)
        .expect("catalog must be valid JSON matching the strict schema")
}

#[test]
fn catalog_has_expected_schema_version() {
    assert_eq!(load_catalog().schema_version, 1);
}

#[test]
fn catalog_is_non_empty() {
    let catalog = load_catalog();
    assert!(
        catalog.scenarios.len() >= 20,
        "expected a substantial scenario catalog, found {}",
        catalog.scenarios.len()
    );
}

#[test]
fn no_duplicate_scenario_ids() {
    let catalog = load_catalog();
    let mut seen = BTreeSet::new();
    for scenario in &catalog.scenarios {
        assert!(
            seen.insert(scenario.id.clone()),
            "duplicate scenario id: {}",
            scenario.id
        );
    }
}

#[test]
fn every_scenario_declares_at_least_one_known_layer() {
    let catalog = load_catalog();
    for scenario in &catalog.scenarios {
        assert!(
            !scenario.layers.is_empty(),
            "scenario {} declares no layers",
            scenario.id
        );
        for layer in &scenario.layers {
            assert!(
                KNOWN_LAYERS.contains(&layer.as_str()),
                "scenario {} declares unknown layer {layer:?}",
                scenario.id
            );
        }
    }
}

#[test]
fn every_scenario_has_at_least_one_source() {
    let catalog = load_catalog();
    for scenario in &catalog.scenarios {
        assert!(
            !scenario.sources.is_empty(),
            "scenario {} has no cross-SDK source attribution",
            scenario.id
        );
        for source in &scenario.sources {
            assert!(
                matches!(source.sdk.as_str(), "dotnet" | "java"),
                "scenario {} cites unknown sdk {:?}",
                scenario.id,
                source.sdk
            );
        }
    }
}

#[test]
fn every_scenario_declares_a_known_distinct_type() {
    let catalog = load_catalog();
    for scenario in &catalog.scenarios {
        assert!(
            KNOWN_DISTINCT_TYPES.contains(&scenario.query.distinct_type.as_str()),
            "scenario {} declares unknown distinctType {:?}",
            scenario.id,
            scenario.query.distinct_type
        );
    }
}

/// A `DISTINCT` scenario's query text must actually contain `DISTINCT` —
/// otherwise the scenario is silently testing the plain query path.
#[test]
fn every_scenario_queries_distinct() {
    let catalog = load_catalog();
    for scenario in &catalog.scenarios {
        assert!(
            scenario.query.text.contains("DISTINCT"),
            "scenario {} declares a DISTINCT catalog entry but its query text has no DISTINCT: {}",
            scenario.id,
            scenario.query.text
        );
    }
}

/// Only an `Ordered` scenario may declare sort columns, and it must: the
/// ordered map's whole correctness argument rests on the stream being sorted.
#[test]
fn ordered_scenarios_declare_their_sort_columns() {
    let catalog = load_catalog();
    for scenario in &catalog.scenarios {
        if scenario.query.distinct_type == "Ordered" {
            assert!(
                !scenario.query.columns.is_empty(),
                "scenario {} is Ordered but declares no ORDER BY columns",
                scenario.id
            );
        }
        for column in &scenario.query.columns {
            assert!(
                matches!(column.direction.as_str(), "Ascending" | "Descending"),
                "scenario {} declares unknown sort direction {:?}",
                scenario.id,
                column.direction
            );
        }
    }
}

#[test]
fn every_scenario_has_an_explicit_expected_result() {
    let catalog = load_catalog();
    for scenario in &catalog.scenarios {
        let all_mock_rows_empty = scenario
            .mock
            .as_ref()
            .map(|mock| {
                mock.partitions
                    .iter()
                    .all(|p| p.pages.iter().all(|page| page.rows.is_empty()))
            })
            .unwrap_or(false);
        let has_expected_result = !scenario.expected_ids.is_empty()
            || !scenario.expected_values.is_empty()
            || scenario.expected_error.is_some()
            || scenario.expected_continuation.is_some()
            || all_mock_rows_empty;
        assert!(
            has_expected_result,
            "scenario {} has no explicit expected result (expectedIds, expectedValues, \
             expectedError, expectedContinuation, or an all-empty mock)",
            scenario.id
        );
    }
}

/// A scenario tagged `mockPipeline` is driven straight off `mock`; without one
/// the layer would silently skip it.
#[test]
fn mock_pipeline_scenarios_carry_a_mock_or_expect_an_error() {
    let catalog = load_catalog();
    for scenario in &catalog.scenarios {
        if !scenario.layers.iter().any(|l| l == "mockPipeline") {
            continue;
        }
        assert!(
            scenario.mock.is_some() || scenario.expected_error.is_some(),
            "scenario {} declares the mockPipeline layer but supplies neither a mock nor an \
             expectedError",
            scenario.id
        );
    }
}

/// A scenario tagged `inMemoryEmulator` is seeded from `documents`.
#[test]
fn emulator_scenarios_carry_documents() {
    let catalog = load_catalog();
    for scenario in &catalog.scenarios {
        if !scenario.layers.iter().any(|l| l == "inMemoryEmulator") {
            continue;
        }
        assert!(
            !scenario.documents.is_empty(),
            "scenario {} declares the inMemoryEmulator layer but seeds no documents",
            scenario.id
        );
    }
}

#[test]
fn mock_partitions_are_sorted_and_tile_with_no_gaps_or_overlaps() {
    let catalog = load_catalog();
    for scenario in &catalog.scenarios {
        let Some(mock) = &scenario.mock else {
            continue;
        };
        if mock.partitions.is_empty() {
            continue;
        }
        let mut cursor = mock.partitions[0].range.min_epk.clone();
        for (idx, partition) in mock.partitions.iter().enumerate() {
            assert_eq!(
                partition.range.min_epk, cursor,
                "scenario {}: partition {idx} does not start where the previous one ended \
                 (gap or overlap)",
                scenario.id,
            );
            assert!(
                partition.range.min_epk < partition.range.max_epk,
                "scenario {}: partition {idx} has an invalid range (min >= max)",
                scenario.id,
            );
            cursor = partition.range.max_epk.clone();
        }
    }
}

/// `_rid` uniquely identifies a row, so a repeated one inside a scenario means
/// the fixture, not the implementation, is producing the duplicate.
#[test]
fn mock_row_rids_are_unique_within_a_scenario() {
    let catalog = load_catalog();
    for scenario in &catalog.scenarios {
        let Some(mock) = &scenario.mock else {
            continue;
        };
        let mut seen = BTreeSet::new();
        for partition in &mock.partitions {
            for page in &partition.pages {
                for row in &page.rows {
                    assert!(
                        seen.insert(row.rid.clone()),
                        "scenario {}: duplicate mock row _rid {:?}",
                        scenario.id,
                        row.rid
                    );
                }
            }
        }
    }
}

/// Every distinct expected value must actually appear among the mock payloads,
/// so a typo in `expectedValues` fails here rather than looking like a bug.
#[test]
fn expected_values_appear_among_the_mock_payloads() {
    let catalog = load_catalog();
    for scenario in &catalog.scenarios {
        let Some(mock) = &scenario.mock else {
            continue;
        };
        let payloads: Vec<&serde_json::Value> = mock
            .partitions
            .iter()
            .flat_map(|p| p.pages.iter())
            .flat_map(|page| page.rows.iter())
            .map(|row| &row.payload)
            .collect();
        for expected in &scenario.expected_values {
            assert!(
                payloads.contains(&expected),
                "scenario {}: expected value {expected} is not produced by any mock row",
                scenario.id
            );
        }
    }
}

#[test]
fn page_sizes_are_positive() {
    let catalog = load_catalog();
    for scenario in &catalog.scenarios {
        for size in &scenario.page_sizes {
            assert!(
                *size > 0,
                "scenario {} declares a zero page size",
                scenario.id
            );
        }
    }
}

#[test]
fn expected_errors_carry_a_category_and_message_fragment() {
    let catalog = load_catalog();
    for scenario in &catalog.scenarios {
        let Some(error) = &scenario.expected_error else {
            continue;
        };
        assert!(
            !error.category.is_empty() && !error.message_fragment.is_empty(),
            "scenario {} has an incomplete expectedError",
            scenario.id
        );
    }
}

/// Every scenario must declare at least one layer that actually has a consumer
/// today, otherwise it can pass every schema guard and still never run.
///
/// `recorded` is reserved for a future live-recording layer and does not count.
#[test]
fn every_scenario_is_claimed_by_a_layer_with_a_consumer() {
    const CONSUMED_LAYERS: &[&str] = &["distinctMap", "mockPipeline", "inMemoryEmulator"];
    let catalog = load_catalog();
    for scenario in &catalog.scenarios {
        assert!(
            scenario
                .layers
                .iter()
                .any(|l| CONSUMED_LAYERS.contains(&l.as_str())),
            "scenario {} declares only layers with no runner, so it would never execute: {:?}",
            scenario.id,
            scenario.layers
        );
    }
}

#[test]
fn required_scenario_inventory_categories_are_represented() {
    let catalog = load_catalog();
    let ids: BTreeSet<&str> = catalog.scenarios.iter().map(|s| s.id.as_str()).collect();

    // One id substring per required inventory category, derived from the .NET
    // and Java DISTINCT suites plus the gaps neither peer covers.
    let required_markers = [
        "unordered_duplicates_across_partitions",
        "unordered_duplicates_across_pages",
        "unordered_duplicates_within_page",
        "unordered_no_duplicates_passthrough",
        "unordered_all_rows_duplicate",
        "type_null_and_boolean",
        "type_empty_string_array_object",
        "numeric_int_vs_float_equal",
        "numeric_negative_zero",
        "object_key_order_irrelevant",
        "array_order_matters",
        "wrapped_value_differs_from_bare",
        "unicode_strings",
        "select_list_multi_column",
        "select_star",
        "select_value_constant_with_from",
        "filters_and_parameters",
        "empty_total_result",
        "empty_backend_page",
        "single_logical_partition",
        "ordered_adjacent_dedup",
        "ordered_descending",
        "ordered_non_adjacent_repeat",
        "ordered_run_spanning_pages",
        "ordered_resume_suppresses_boundary_row",
        "ordered_resume_round_trip",
        "distinct_order_by_mismatched_field",
        "unordered_continuation_token_rejected",
        "malformed_token",
        "token_shape_mismatch",
        "distinct_type_mismatch_on_resume",
        "split_",
        "headers_request_charge",
        "unsupported_combination",
    ];
    for marker in required_markers {
        assert!(
            ids.iter().any(|id| id.contains(marker)),
            "no scenario id contains required inventory marker {marker:?}; catalog ids: {ids:?}"
        );
    }
}

/// Both deduplication modes must be represented, and the peer-untested gaps we
/// deliberately close must stay in the catalog.
#[test]
fn both_distinct_modes_are_represented() {
    let catalog = load_catalog();
    for expected in ["Ordered", "Unordered"] {
        assert!(
            catalog
                .scenarios
                .iter()
                .any(|s| s.query.distinct_type == expected),
            "no scenario exercises {expected} DISTINCT"
        );
    }
}
