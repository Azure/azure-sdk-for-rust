// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod bootstrap_primary;
mod capabilities;
mod catalog;
mod diagnostics_success_and_error;
mod fixture;
mod item_create_conflict;
mod item_not_found;
mod item_optimistic_concurrency;
mod item_upsert;
#[path = "item_lifecycle.rs"]
mod lifecycle;
mod query_invalid_syntax;
mod query_parameterized_filter;
mod support;

const IMPLEMENTED_TESTS: &[&str] = &[
    "capabilities::capability_document_is_versioned",
    "bootstrap_primary::bootstrap_primary_endpoint",
    "lifecycle::item_lifecycle",
    "item_upsert::upsert_creates_then_updates",
    "item_create_conflict::duplicate_create_preserves_original",
    "item_not_found::not_found_does_not_cross_partition_keys",
    "item_optimistic_concurrency::stale_etag_preserves_successful_update",
    "query_parameterized_filter::parameterized_query_filters_and_orders",
    "query_invalid_syntax::invalid_query_is_not_an_empty_feed",
    "diagnostics_success_and_error::diagnostics_cover_success_and_error",
];

#[test]
fn e2e_scenario_catalog_is_valid() {
    catalog::validate_catalog(IMPLEMENTED_TESTS).expect("E2E scenario catalog must be valid");
}
