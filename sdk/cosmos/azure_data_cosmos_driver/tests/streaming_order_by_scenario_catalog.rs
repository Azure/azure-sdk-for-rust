// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Fixture/schema tests for the streaming cross-partition `ORDER BY`
//! scenario catalog (`tests/fixtures/streaming_order_by_scenarios.json`).
//!
//! This is the single source-attributed catalog reused across every test
//! layer (comparator/continuation unit tests, mock-pipeline tests, and
//! in-memory-emulator tests). Each layer lives in a different compilation
//! unit and defines its own minimal `Deserialize` view of the fixture; this
//! file owns the strict, canonical schema validation every other layer trusts.
//!
//! Validates: no duplicate scenario IDs; every `layers` entry is a known
//! layer name; every scenario declares at least one layer and some
//! expected result; mock partitions are sorted, gapless, and tile
//! correctly; row `orderByItems` length matches the query's column count;
//! and required scenario-inventory categories are represented.

use std::collections::BTreeSet;

use serde::Deserialize;

const CATALOG_JSON: &str = include_str!("fixtures/streaming_order_by_scenarios.json");

const KNOWN_LAYERS: &[&str] = &["comparator", "mockPipeline", "inMemoryEmulator", "recorded"];

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
    #[allow(dead_code)]
    page_sizes: Vec<u32>,
    #[serde(rename = "expectedIds", default)]
    expected_ids: Vec<String>,
    #[allow(dead_code)]
    checkpoint: Option<serde_json::Value>,
    #[serde(rename = "expectedContinuation")]
    expected_continuation: Option<serde_json::Value>,
    #[serde(rename = "expectedError")]
    expected_error: Option<ExpectedError>,
}

#[derive(Deserialize)]
struct Source {
    #[allow(dead_code)]
    sdk: String,
    #[allow(dead_code)]
    path: String,
    #[allow(dead_code)]
    test: String,
}

#[derive(Deserialize)]
struct QuerySpec {
    #[allow(dead_code)]
    text: String,
    #[serde(default)]
    #[allow(dead_code)]
    parameters: Vec<serde_json::Value>,
    columns: Vec<ColumnSpec>,
}

#[derive(Deserialize)]
struct ColumnSpec {
    #[allow(dead_code)]
    expression: String,
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    rid: String,
    #[serde(rename = "orderByItems")]
    order_by_items: Vec<serde_json::Value>,
    #[allow(dead_code)]
    payload: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct ExpectedError {
    #[allow(dead_code)]
    category: String,
    #[allow(dead_code)]
    #[serde(rename = "messageFragment")]
    message_fragment: String,
}

fn load_catalog() -> Catalog {
    serde_json::from_str(CATALOG_JSON)
        .expect("catalog must be valid JSON matching the strict schema")
}

#[test]
fn catalog_has_expected_schema_version() {
    let catalog = load_catalog();
    assert_eq!(catalog.schema_version, 1);
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
            || scenario.expected_error.is_some()
            || scenario.expected_continuation.is_some()
            || all_mock_rows_empty;
        assert!(
            has_expected_result,
            "scenario {} has no explicit expected result (expectedIds, expectedError, \
             expectedContinuation, or an all-empty mock)",
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

#[test]
fn mock_row_key_counts_match_the_query_column_count() {
    let catalog = load_catalog();
    for scenario in &catalog.scenarios {
        // Malformed-envelope scenarios deliberately mismatch on purpose.
        if scenario.expected_error.is_some() {
            continue;
        }
        let Some(mock) = &scenario.mock else {
            continue;
        };
        let expected_len = scenario.query.columns.len();
        for (p_idx, partition) in mock.partitions.iter().enumerate() {
            for (pg_idx, page) in partition.pages.iter().enumerate() {
                for (row_idx, row) in page.rows.iter().enumerate() {
                    assert_eq!(
                        row.order_by_items.len(),
                        expected_len,
                        "scenario {} partition {p_idx} page {pg_idx} row {row_idx}: \
                         orderByItems length does not match the query's {expected_len} \
                         ORDER BY column(s)",
                        scenario.id,
                    );
                }
            }
        }
    }
}

#[test]
fn required_scenario_inventory_categories_are_represented() {
    let catalog = load_catalog();
    let ids: BTreeSet<&str> = catalog.scenarios.iter().map(|s| s.id.as_str()).collect();

    // One id substring per required inventory category; not every
    // category needs a dedicated scenario, but each marker must appear.
    let required_markers = [
        "single_column_asc",
        "single_column_desc",
        "multi_column",
        "duplicate_keys",
        "undefined_field",
        "mixed_type_ordering",
        "array_keys",
        "object_keys",
        "skip_count",
        "formatted_resume_filter",
        "empty_total_result",
        "empty_backend_page",
        "headers_request_charge",
        "split_",
        "invalid_rewritten_envelope",
        "malformed_token",
        "single_logical_partition",
        "filters_and_parameters",
        "unsupported_combination",
    ];
    for marker in required_markers {
        assert!(
            ids.iter().any(|id| id.contains(marker)),
            "no scenario id contains required inventory marker {marker:?}; catalog ids: {ids:?}"
        );
    }
}
