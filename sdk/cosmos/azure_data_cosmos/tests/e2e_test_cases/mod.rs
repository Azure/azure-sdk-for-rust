// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod bootstrap_backup_fallback;
mod bootstrap_primary;
mod capabilities;
mod catalog;
mod change_feed_all_versions_starts;
mod change_feed_pagination_resume;
mod configuration_binary_routing;
mod consistency_feed_read_strategies;
mod consistency_session_management;
mod diagnostics_handlers_telemetry;
mod diagnostics_success_and_error;
mod fixture;
mod item_create_conflict;
mod item_hierarchical_partition_key;
mod item_lifecycle;
mod item_not_found;
mod item_optimistic_concurrency;
mod item_patch_state;
mod item_scalar_partition_keys;
mod item_upsert;
mod item_validation_contracts;
mod management_resource_lifecycle;
mod numeric_unique_key_equivalence;
mod query_feed_ranges;
mod query_invalid_syntax;
mod query_pagination_resume;
mod query_parameterized_filter;
mod quoted_partition_key_paths;
mod resilience_deadline;
mod resilience_hedging;
mod resilience_throttling_retry;
mod resilience_transient_retries;
mod support;
mod transactional_batch_atomicity;

const IMPLEMENTED_TESTS: &[&str] = &[
    "capabilities::capability_document_is_versioned",
    "management_resource_lifecycle::database_and_container_resource_lifecycle",
    "bootstrap_primary::bootstrap_primary_endpoint",
    "bootstrap_backup_fallback::unreachable_primary_uses_ordered_backup",
    "configuration_binary_routing::binary_text_and_routing_options_preserve_behavior",
    "consistency_feed_read_strategies::feeds_honor_account_and_operation_consistency",
    "consistency_session_management::explicit_tokens_work_when_automatic_capture_is_disabled",
    "consistency_session_management::disabled_capture_exposes_delayed_replica",
    "item_lifecycle::crud_lifecycle",
    "item_upsert::upsert_creates_then_updates",
    "item_create_conflict::duplicate_create_preserves_original",
    "item_not_found::not_found_does_not_cross_partition_keys",
    "item_optimistic_concurrency::stale_etag_preserves_successful_update",
    "item_scalar_partition_keys::scalar_partition_key_values_remain_distinct",
    "item_hierarchical_partition_key::hierarchical_key_point_and_prefix_operations",
    "transactional_batch_atomicity::batch_success_and_failure_are_atomic",
    "item_patch_state::patch_paths_persist_exact_post_images",
    "item_validation_contracts::invalid_writes_preserve_state",
    "numeric_unique_key_equivalence::wide_integers_follow_service_unique_key_equivalence",
    "quoted_partition_key_paths::quoted_paths_work_across_item_operations",
    "query_parameterized_filter::parameterized_query_filters_and_orders",
    "query_invalid_syntax::invalid_query_is_not_an_empty_feed",
    "query_pagination_resume::query_resumes_without_loss_or_duplication",
    "query_feed_ranges::feed_ranges_cover_partition_key_routing",
    "change_feed_pagination_resume::change_feed_resumes_without_replay",
    "change_feed_all_versions_starts::all_versions_rejects_unsupported_starts",
    "diagnostics_success_and_error::diagnostics_cover_success_and_error",
    "diagnostics_handlers_telemetry::handlers_emit_metrics_spans_and_sampled_failures",
    "resilience_throttling_retry::bounded_throttling_retries_succeed_with_attempt_history",
    "resilience_deadline::operation_deadline_preempts_delayed_response",
    "resilience_hedging::alternate_region_wins_delayed_primary_hedge",
    "resilience_transient_retries::request_timeout_retries_respect_failover_budget",
    "resilience_transient_retries::service_unavailable_is_retried_with_attempt_history",
    "resilience_transient_retries::response_timeout_is_retried_with_attempt_history",
];

#[test]
fn e2e_scenario_catalog_is_valid() {
    catalog::validate_catalog(IMPLEMENTED_TESTS).expect("E2E scenario catalog must be valid");
}

#[test]
fn implementation_registry_names_compiled_tests() {
    let executable = std::env::current_exe().expect("current E2E test executable must be known");
    let output = std::process::Command::new(executable)
        .arg("--list")
        .output()
        .expect("E2E test executable must support libtest --list");
    assert!(
        output.status.success(),
        "listing compiled E2E tests failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let listed = String::from_utf8(output.stdout).expect("libtest list must be UTF-8");
    for test in IMPLEMENTED_TESTS {
        assert!(
            listed.lines().any(|line| {
                line.strip_suffix(": test") == Some(format!("e2e_test_cases::{test}").as_str())
            }),
            "implementation registry entry '{test}' does not name a compiled E2E test"
        );
    }
}
