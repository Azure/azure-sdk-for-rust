// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Shared scaffolding for the source-attributed scenario catalogs
//! (`skip_take_scenario_catalog.rs` and `streaming_order_by_scenario_catalog.rs`).
//!
//! The two catalogs describe different fixtures with different schema-specific
//! fields, but they share the cross-SDK [`Source`] attribution shape, the set
//! of [`KNOWN_LAYERS`], and the structural invariants every catalog must hold
//! (unique scenario IDs, known layer names, at least one source per scenario).
//! Those common pieces live here so the two validators stay in lock-step
//! instead of drifting apart.

// Included via `mod` into two separate integration-test crates; not every
// helper is exercised by both, so suppress unused warnings crate-wide.
#![allow(dead_code)]

use std::collections::BTreeSet;

use serde::Deserialize;

/// Layer names a scenario may declare. Each names a test layer that consumes
/// the catalog (comparator/continuation unit tests, mock-pipeline tests,
/// in-memory-emulator tests, or a recorded-response replay).
pub const KNOWN_LAYERS: &[&str] = &["comparator", "mockPipeline", "inMemoryEmulator", "recorded"];

/// Cross-SDK source attribution for a catalog scenario: which SDK's test the
/// scenario was ported from, the file it lives in, and the test name.
#[derive(Deserialize)]
pub struct Source {
    pub sdk: String,
    pub path: String,
    pub test: String,
}

/// Asserts that every scenario ID in `ids` is unique.
pub fn assert_unique_scenario_ids<'a>(ids: impl IntoIterator<Item = &'a str>) {
    let mut seen = BTreeSet::new();
    for id in ids {
        assert!(seen.insert(id), "duplicate scenario id: {id}");
    }
}

/// Asserts that a scenario declares at least one layer and that every declared
/// layer is a [`KNOWN_LAYERS`] entry.
pub fn assert_scenario_layers_known(scenario_id: &str, layers: &[String]) {
    assert!(
        !layers.is_empty(),
        "scenario {scenario_id} declares no layers"
    );
    for layer in layers {
        assert!(
            KNOWN_LAYERS.contains(&layer.as_str()),
            "scenario {scenario_id} declares unknown layer {layer:?}"
        );
    }
}

/// Asserts that a scenario carries at least one cross-SDK [`Source`].
pub fn assert_scenario_has_source(scenario_id: &str, sources: &[Source]) {
    assert!(
        !sources.is_empty(),
        "scenario {scenario_id} has no cross-SDK source attribution"
    );
}
