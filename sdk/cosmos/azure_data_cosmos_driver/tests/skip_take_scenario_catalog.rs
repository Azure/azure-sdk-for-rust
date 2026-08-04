// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Fixture/schema tests for the cross-partition `OFFSET`/`LIMIT`/`TOP`
//! scenario catalog (`tests/fixtures/skip_take_scenarios.json`).
//!
//! This is the single source-attributed catalog that drives the real
//! in-memory-emulator test layer (`tests/in_memory_emulator_tests/skip_take.rs`).
//! This file owns the strict, canonical schema validation that the emulator
//! layer trusts, mirroring the streaming `ORDER BY` catalog's validator so the
//! two converge on a shared shape.
//!
//! Validates: schema version; a non-trivial scenario count; no duplicate
//! scenario IDs; every `layers` entry is a known layer; every scenario has at
//! least one cross-SDK source and at least one document; the `assertion` mode
//! is known and internally consistent with the declared expectation
//! (`exactOrdered` windows must live in a single logical partition and name
//! documents that exist; `empty` windows expect zero rows; `unorderedSubsetCount`
//! windows expect a positive count no larger than the seed set); and that the
//! required scenario-inventory categories are represented.

use std::collections::BTreeSet;

use serde::Deserialize;

const CATALOG_JSON: &str = include_str!("fixtures/skip_take_scenarios.json");

const KNOWN_LAYERS: &[&str] = &["comparator", "mockPipeline", "inMemoryEmulator", "recorded"];

const KNOWN_ASSERTIONS: &[&str] = &["exactOrdered", "unorderedSubsetCount", "empty"];

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
    #[allow(dead_code)]
    query: QuerySpec,
    documents: Vec<Document>,
    #[serde(rename = "partitionCount")]
    partition_count: u32,
    assertion: String,
    #[serde(rename = "expectedIds", default)]
    expected_ids: Vec<String>,
    #[serde(rename = "expectedCount")]
    expected_count: Option<u64>,
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
}

#[derive(Deserialize)]
struct Document {
    id: String,
    pk: String,
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
        catalog.scenarios.len() >= 10,
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
fn every_scenario_has_documents_and_a_valid_partition_count() {
    let catalog = load_catalog();
    for scenario in &catalog.scenarios {
        assert!(
            !scenario.documents.is_empty(),
            "scenario {} seeds no documents",
            scenario.id
        );
        assert!(
            scenario.partition_count >= 1,
            "scenario {} has an invalid partitionCount {}",
            scenario.id,
            scenario.partition_count
        );
    }
}

#[test]
fn every_scenario_uses_a_known_and_consistent_assertion() {
    let catalog = load_catalog();
    for scenario in &catalog.scenarios {
        assert!(
            KNOWN_ASSERTIONS.contains(&scenario.assertion.as_str()),
            "scenario {} uses unknown assertion {:?}",
            scenario.id,
            scenario.assertion
        );

        let doc_ids: BTreeSet<&str> = scenario.documents.iter().map(|d| d.id.as_str()).collect();

        match scenario.assertion.as_str() {
            "exactOrdered" => {
                assert!(
                    !scenario.expected_ids.is_empty(),
                    "scenario {} is exactOrdered but names no expectedIds",
                    scenario.id
                );
                // Exact ordering is only deterministic within a single logical
                // partition, so every document must share one partition key.
                let distinct_pks: BTreeSet<&str> =
                    scenario.documents.iter().map(|d| d.pk.as_str()).collect();
                assert_eq!(
                    distinct_pks.len(),
                    1,
                    "scenario {} is exactOrdered but spans multiple partition keys {distinct_pks:?}; \
                     cross-partition order is unspecified",
                    scenario.id
                );
                for id in &scenario.expected_ids {
                    assert!(
                        doc_ids.contains(id.as_str()),
                        "scenario {} expects id {id:?} that is not among its seeded documents",
                        scenario.id
                    );
                }
            }
            "unorderedSubsetCount" => {
                let count = scenario.expected_count.unwrap_or_else(|| {
                    panic!(
                        "scenario {} is unorderedSubsetCount but has no expectedCount",
                        scenario.id
                    )
                });
                assert!(
                    count >= 1 && count as usize <= scenario.documents.len(),
                    "scenario {} expectedCount {count} is out of range for {} documents",
                    scenario.id,
                    scenario.documents.len()
                );
            }
            "empty" => {
                assert_eq!(
                    scenario.expected_count,
                    Some(0),
                    "scenario {} is empty but expectedCount is not 0",
                    scenario.id
                );
                assert!(
                    scenario.expected_ids.is_empty(),
                    "scenario {} is empty but names expectedIds",
                    scenario.id
                );
            }
            other => unreachable!("assertion {other:?} was allowed but not handled"),
        }
    }
}

#[test]
fn required_scenario_inventory_categories_are_represented() {
    let catalog = load_catalog();
    let ids: BTreeSet<&str> = catalog.scenarios.iter().map(|s| s.id.as_str()).collect();

    // One id substring per required inventory category.
    let required_markers = [
        "top_zero",
        "top_one",
        "top_larger_than_total",
        "top_cross_partition",
        "offset_beyond_total",
        "limit_zero",
        "offset_limit_single_partition",
        "offset_limit_cross_partition",
        "boundary_spanning",
    ];
    for marker in required_markers {
        assert!(
            ids.iter().any(|id| id.contains(marker)),
            "no scenario id contains required inventory marker {marker:?}; catalog ids: {ids:?}"
        );
    }
}
