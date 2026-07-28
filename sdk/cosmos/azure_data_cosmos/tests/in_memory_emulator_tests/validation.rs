// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Response validation framework for comparing real Cosmos DB responses against
//! the in-memory emulator.
//!
//! Each header field can be validated at one of several strictness levels:
//! - `Exact`: values must match exactly
//! - `Exists`: both must be present (values may differ)
//! - `NonNegative`: both must be present and non-negative numbers
//! - `RequiredInEmulatorOnly`: emulator must emit the header; real backend may omit it
//! - `Ignore`: skip validation for this field
//!
//! Status codes must always match. Body payloads are compared structurally with
//! per-field rules for system properties (e.g. `_rid`, `_ts`) that differ
//! between backends.

// cspell:ignore acked llsn

use std::collections::HashMap;

use azure_data_cosmos_driver::models::{CosmosResponseHeaders, ResponseBody};
use azure_data_cosmos_driver::CosmosResponse;

/// How a single header field should be validated between real and emulator responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum HeaderMatch {
    /// Values must match exactly (after stringification).
    Exact,

    /// Both must be `Some`. Values are allowed to differ.
    Exists,

    /// Both must be present and parse as non-negative f64.
    NonNegative,

    /// Presence must match: if present in real, must be present in emulator;
    /// if absent in real, must be absent in emulator. Values may differ.
    Symmetric,

    /// Emulator must emit this header. The real backend may emit or omit it.
    /// Values are allowed to differ when both are present.
    RequiredInEmulatorOnly,

    /// Skip validation for this field entirely.
    Ignore,
}

/// Per-header validation specification for [`CosmosResponseHeaders`].
///
/// By default, all headers use [`HeaderMatch::Exact`] (values must be identical).
/// Override specific headers with [`with_rule`](Self::with_rule) to relax validation.
#[derive(Debug, Clone)]
pub struct HeaderValidationSpec {
    rules: HashMap<String, HeaderMatch>,
}

impl HeaderValidationSpec {
    /// Creates a new spec where all headers default to [`HeaderMatch::Exact`].
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    /// Sets a validation rule for a specific header, returning `self` for chaining.
    pub fn with_rule(mut self, name: impl Into<String>, rule: HeaderMatch) -> Self {
        self.rules.insert(name.into(), rule);
        self
    }

    /// Returns the validation rule for a header. Defaults to [`HeaderMatch::Exact`].
    pub fn rule_for(&self, name: &str) -> HeaderMatch {
        self.rules.get(name).copied().unwrap_or(HeaderMatch::Exact)
    }

    /// Default spec for a successful point-read or point-write operation.
    ///
    /// Many Cosmos-response headers cannot be value-compared against an
    /// emulator response. The relaxations below fall into three categories:
    ///
    /// - **`Exists` / `NonNegative`** — value is request-scoped or wall-clock
    ///   dependent and will legitimately differ, OR the header is one the
    ///   Gateway 2.0 thin-client response path does not surface even though
    ///   the emulator always pre-seeds it. Examples: `activity_id`
    ///   (per-request UUID), `request_charge` (RU model approximations),
    ///   `session_token` / `etag` / `lsn` (depend on real-account history),
    ///   `server_duration_ms`; and the replica-internal headers the thin
    ///   client strips but the emulator always emits — `transport_request_id`,
    ///   `global_committed_lsn`, `local_lsn`, `number_of_read_regions`,
    ///   `last_state_change_utc`, `gateway_version`, `service_version`.
    /// - **`Symmetric`** — header presence must match (both present or both
    ///   absent), values are allowed to differ. Used for headers whose value
    ///   depends on real-replica internals the emulator cannot meaningfully
    ///   reproduce: `correlated_activity_id` (client-set), `retry_after_ms`,
    ///   `offer_replace_pending`, `has_tentative_writes`, query/continuation
    ///   metrics (no-op on point ops).
    /// - **`RequiredInEmulatorOnly`** — emulator must emit this diagnostic
    ///   header, but Gateway-backed real/emulator responses may omit it.
    ///   Used for `internal_partition_id` on point operations.
    /// - **`Ignore`** — emulator does not (and intentionally will not)
    ///   produce these headers, or they encode internal state that has no
    ///   public meaning for the operation under test: `item_local_lsn`,
    ///   `quorum_acked_lsn`, `quorum_acked_local_lsn`, `resource_quota`,
    ///   `resource_usage`, and the index-build progress headers a real backend emits but the
    ///   in-memory emulator has no indexing engine to reproduce —
    ///   `collection_index_transformation_progress`,
    ///   `collection_lazy_indexing_progress`.
    pub fn for_point_operation() -> Self {
        Self::new()
            .with_rule("activity_id", HeaderMatch::Exists)
            .with_rule("request_charge", HeaderMatch::NonNegative)
            .with_rule("session_token", HeaderMatch::Exists)
            .with_rule("etag", HeaderMatch::Exists)
            .with_rule("continuation", HeaderMatch::Symmetric)
            .with_rule("item_count", HeaderMatch::Symmetric)
            .with_rule("index_metrics", HeaderMatch::Symmetric)
            .with_rule("query_metrics", HeaderMatch::Symmetric)
            .with_rule("server_duration_ms", HeaderMatch::Exists)
            .with_rule("lsn", HeaderMatch::Exists)
            .with_rule("item_lsn", HeaderMatch::Ignore)
            .with_rule("offer_replace_pending", HeaderMatch::Symmetric)
            .with_rule("retry_after_ms", HeaderMatch::Symmetric)
            .with_rule("correlated_activity_id", HeaderMatch::Symmetric)
            .with_rule("transport_request_id", HeaderMatch::Exists)
            .with_rule("global_committed_lsn", HeaderMatch::Exists)
            .with_rule("quorum_acked_lsn", HeaderMatch::Ignore)
            .with_rule("quorum_acked_local_lsn", HeaderMatch::Ignore)
            .with_rule("local_lsn", HeaderMatch::Exists)
            .with_rule("item_local_lsn", HeaderMatch::Ignore)
            .with_rule("number_of_read_regions", HeaderMatch::Exists)
            .with_rule("last_state_change_utc", HeaderMatch::Exists)
            .with_rule("gateway_version", HeaderMatch::Exists)
            .with_rule("service_version", HeaderMatch::Exists)
            .with_rule("resource_quota", HeaderMatch::Ignore)
            .with_rule("resource_usage", HeaderMatch::Ignore)
            .with_rule("has_tentative_writes", HeaderMatch::Symmetric)
            .with_rule("partition_key_range_id", HeaderMatch::Symmetric)
            .with_rule("internal_partition_id", HeaderMatch::RequiredInEmulatorOnly)
            .with_rule("log_results", HeaderMatch::Symmetric)
            .with_rule(
                "collection_index_transformation_progress",
                HeaderMatch::Ignore,
            )
            .with_rule("collection_lazy_indexing_progress", HeaderMatch::Ignore)
    }

    /// Spec for a delete operation (no etag in response typically).
    pub fn for_delete_operation() -> Self {
        Self::new()
            .with_rule("activity_id", HeaderMatch::Exists)
            .with_rule("request_charge", HeaderMatch::NonNegative)
            .with_rule("session_token", HeaderMatch::Exists)
            .with_rule("etag", HeaderMatch::Exists)
            .with_rule("continuation", HeaderMatch::Symmetric)
            .with_rule("item_count", HeaderMatch::Symmetric)
            .with_rule("index_metrics", HeaderMatch::Symmetric)
            .with_rule("query_metrics", HeaderMatch::Symmetric)
            .with_rule("server_duration_ms", HeaderMatch::Exists)
            .with_rule("lsn", HeaderMatch::Exists)
            .with_rule("item_lsn", HeaderMatch::Ignore)
            .with_rule("offer_replace_pending", HeaderMatch::Symmetric)
            .with_rule("retry_after_ms", HeaderMatch::Symmetric)
            .with_rule("correlated_activity_id", HeaderMatch::Symmetric)
            .with_rule("transport_request_id", HeaderMatch::Exists)
            .with_rule("global_committed_lsn", HeaderMatch::Exists)
            .with_rule("quorum_acked_lsn", HeaderMatch::Ignore)
            .with_rule("quorum_acked_local_lsn", HeaderMatch::Ignore)
            .with_rule("local_lsn", HeaderMatch::Exists)
            .with_rule("item_local_lsn", HeaderMatch::Ignore)
            .with_rule("number_of_read_regions", HeaderMatch::Exists)
            .with_rule("last_state_change_utc", HeaderMatch::Exists)
            .with_rule("gateway_version", HeaderMatch::Exists)
            .with_rule("service_version", HeaderMatch::Exists)
            .with_rule("resource_quota", HeaderMatch::Ignore)
            .with_rule("resource_usage", HeaderMatch::Ignore)
            .with_rule("has_tentative_writes", HeaderMatch::Symmetric)
            .with_rule("partition_key_range_id", HeaderMatch::Symmetric)
            .with_rule("internal_partition_id", HeaderMatch::RequiredInEmulatorOnly)
            .with_rule("log_results", HeaderMatch::Symmetric)
            .with_rule(
                "collection_index_transformation_progress",
                HeaderMatch::Ignore,
            )
            .with_rule("collection_lazy_indexing_progress", HeaderMatch::Ignore)
    }

    /// Spec for a control-plane operation (create database/container).
    pub fn for_control_plane() -> Self {
        Self::new()
            .with_rule("activity_id", HeaderMatch::Exists)
            .with_rule("request_charge", HeaderMatch::NonNegative)
            .with_rule("session_token", HeaderMatch::Symmetric)
            .with_rule("etag", HeaderMatch::Exists)
            .with_rule("continuation", HeaderMatch::Symmetric)
            .with_rule("item_count", HeaderMatch::Symmetric)
            .with_rule("index_metrics", HeaderMatch::Symmetric)
            .with_rule("query_metrics", HeaderMatch::Symmetric)
            .with_rule("server_duration_ms", HeaderMatch::Exists)
            .with_rule("item_lsn", HeaderMatch::Ignore)
            .with_rule("offer_replace_pending", HeaderMatch::Symmetric)
            .with_rule("retry_after_ms", HeaderMatch::Symmetric)
            .with_rule("correlated_activity_id", HeaderMatch::Symmetric)
            .with_rule("transport_request_id", HeaderMatch::Exists)
            .with_rule("global_committed_lsn", HeaderMatch::Exists)
            .with_rule("quorum_acked_lsn", HeaderMatch::Ignore)
            .with_rule("quorum_acked_local_lsn", HeaderMatch::Ignore)
            .with_rule("local_lsn", HeaderMatch::Exists)
            .with_rule("item_local_lsn", HeaderMatch::Ignore)
            .with_rule("number_of_read_regions", HeaderMatch::Exists)
            .with_rule("last_state_change_utc", HeaderMatch::Exists)
            .with_rule("gateway_version", HeaderMatch::Exists)
            .with_rule("service_version", HeaderMatch::Exists)
            .with_rule("resource_quota", HeaderMatch::Ignore)
            .with_rule("resource_usage", HeaderMatch::Ignore)
            .with_rule("has_tentative_writes", HeaderMatch::Symmetric)
            .with_rule("partition_key_range_id", HeaderMatch::Ignore)
            .with_rule("internal_partition_id", HeaderMatch::Ignore)
            .with_rule("log_results", HeaderMatch::Symmetric)
            .with_rule(
                "collection_index_transformation_progress",
                HeaderMatch::Ignore,
            )
            .with_rule("collection_lazy_indexing_progress", HeaderMatch::Ignore)
            .with_rule("lsn", HeaderMatch::Ignore)
    }

    /// Default spec for successful query pages.
    ///
    /// Query metrics, index metrics, and RU charges are intentionally relaxed:
    /// live backends expose useful query-engine and indexing details that the
    /// in-memory emulator cannot faithfully reproduce. Query comparison tests
    /// should log those values for diagnostics, while using this spec for basic
    /// header sanity and presence checks.
    pub fn for_query_operation() -> Self {
        Self::new()
            .with_rule("activity_id", HeaderMatch::Exists)
            .with_rule("request_charge", HeaderMatch::NonNegative)
            .with_rule("session_token", HeaderMatch::Exists)
            .with_rule("etag", HeaderMatch::Symmetric)
            .with_rule("continuation", HeaderMatch::Symmetric)
            .with_rule("item_count", HeaderMatch::Symmetric)
            .with_rule("index_metrics", HeaderMatch::Ignore)
            .with_rule("query_metrics", HeaderMatch::Ignore)
            .with_rule("server_duration_ms", HeaderMatch::Exists)
            .with_rule("lsn", HeaderMatch::Symmetric)
            .with_rule("item_lsn", HeaderMatch::Ignore)
            .with_rule("offer_replace_pending", HeaderMatch::Symmetric)
            .with_rule("retry_after_ms", HeaderMatch::Symmetric)
            .with_rule("correlated_activity_id", HeaderMatch::Symmetric)
            .with_rule("transport_request_id", HeaderMatch::Exists)
            .with_rule("global_committed_lsn", HeaderMatch::Exists)
            .with_rule("quorum_acked_lsn", HeaderMatch::Ignore)
            .with_rule("quorum_acked_local_lsn", HeaderMatch::Ignore)
            .with_rule("local_lsn", HeaderMatch::Exists)
            .with_rule("item_local_lsn", HeaderMatch::Ignore)
            .with_rule("number_of_read_regions", HeaderMatch::Exists)
            .with_rule("last_state_change_utc", HeaderMatch::Exists)
            .with_rule("gateway_version", HeaderMatch::Exists)
            .with_rule("service_version", HeaderMatch::Exists)
            .with_rule("resource_quota", HeaderMatch::Ignore)
            .with_rule("resource_usage", HeaderMatch::Ignore)
            .with_rule("has_tentative_writes", HeaderMatch::Symmetric)
            .with_rule("partition_key_range_id", HeaderMatch::Symmetric)
            .with_rule("internal_partition_id", HeaderMatch::Symmetric)
            .with_rule("log_results", HeaderMatch::Symmetric)
            .with_rule(
                "collection_index_transformation_progress",
                HeaderMatch::Ignore,
            )
            .with_rule("collection_lazy_indexing_progress", HeaderMatch::Ignore)
    }
}

/// Snapshot of a [`CosmosResponse`] for deferred comparison.
pub struct ResponseSnapshot {
    pub status_code: u16,
    pub sub_status_code: Option<u16>,
    pub headers: CosmosResponseHeaders,
    pub body: Option<serde_json::Value>,
    #[allow(dead_code)]
    pub label: String,
}

impl ResponseSnapshot {
    /// Captures a snapshot from a `CosmosResponse`.
    pub fn capture(response: &CosmosResponse, label: impl Into<String>) -> Self {
        let body = match response.body() {
            ResponseBody::NoPayload => None,
            ResponseBody::Bytes(b) if b.is_empty() => None,
            ResponseBody::Bytes(b) => serde_json::from_slice(b).ok(),
            // No production path emits `Items` for the operations exercised by
            // this validation framework today (point ops / batch / metadata).
            // Until the in-memory emulator harness grows query/changefeed
            // coverage and we decide how to stitch the per-document slices
            // into a comparable snapshot, fail loudly rather than silently
            // dropping the body and letting tests pass on a regression.
            ResponseBody::Items(_) => panic!(
                "ResponseSnapshot::capture: received Items response body but the validation \
                 framework currently only supports single-payload responses. Add feed-aware \
                 body comparison before exercising this path."
            ),
        };
        Self {
            status_code: u16::from(response.status()),
            sub_status_code: response.status().sub_status().map(|s| s.value()),
            headers: response.headers().clone(),
            body,
            label: label.into(),
        }
    }
}

/// System properties that the service injects into documents.
/// These will always differ between real and emulator, so we only assert
/// presence (not value equality).
const SYSTEM_PROPERTIES: &[&str] = &["_rid", "_self", "_etag", "_attachments", "_ts"];

/// Body fields that should match exactly between real and emulator.
fn is_user_field(key: &str) -> bool {
    !SYSTEM_PROPERTIES.contains(&key)
}

/// Compares two [`ResponseSnapshot`]s according to the given rules.
///
/// Panics with descriptive messages on mismatches.
pub fn compare_responses(
    real: &ResponseSnapshot,
    emulator: &ResponseSnapshot,
    header_spec: &HeaderValidationSpec,
    body_spec: BodyValidationSpec,
) {
    // ── Status code ──────────────────────────────────────────────
    assert_eq!(
        real.status_code, emulator.status_code,
        "Status code mismatch: real={} emulator={}",
        real.status_code, emulator.status_code,
    );

    // ── Sub-status code ──────────────────────────────────────────
    assert_eq!(
        real.sub_status_code, emulator.sub_status_code,
        "Sub-status code mismatch: real={:?} emulator={:?}",
        real.sub_status_code, emulator.sub_status_code,
    );

    // ── Headers ──────────────────────────────────────────────────
    compare_headers(&real.headers, &emulator.headers, header_spec);

    // ── Body ─────────────────────────────────────────────────────
    match body_spec {
        BodyValidationSpec::Ignore => {}
        BodyValidationSpec::StatusOnly => {} // already checked above
        BodyValidationSpec::ExactJson => {
            assert_eq!(
                real.body, emulator.body,
                "Body mismatch (exact JSON comparison)"
            );
        }
        BodyValidationSpec::StructuralMatch => {
            validate_body_structural(&real.body, &emulator.body);
        }
        BodyValidationSpec::DocumentMatch => {
            validate_body_document(&real.body, &emulator.body);
        }
    }
}

/// Compares parsed Cosmos response headers according to the given validation spec.
pub fn compare_headers(
    real: &CosmosResponseHeaders,
    emulator: &CosmosResponseHeaders,
    header_spec: &HeaderValidationSpec,
) {
    let header_pairs = extract_header_pairs(real, emulator);
    for (name, real_val, emu_val) in &header_pairs {
        validate_header_field(name, real_val, emu_val, header_spec.rule_for(name));
    }
}

/// How to validate the response body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum BodyValidationSpec {
    /// Skip body validation entirely.
    Ignore,

    /// Only validate status code (body not inspected).
    StatusOnly,

    /// Bodies must be byte-for-byte identical JSON.
    ExactJson,

    /// User fields must match exactly; system properties only need to be present.
    DocumentMatch,

    /// JSON structure must match (same keys at every level).
    StructuralMatch,
}

/// Extracts all known header fields as `(name, real_value, emulator_value)` tuples
/// for systematic iteration.
fn extract_header_pairs(
    real: &CosmosResponseHeaders,
    emulator: &CosmosResponseHeaders,
) -> Vec<(&'static str, Option<String>, Option<String>)> {
    vec![
        (
            "activity_id",
            real.activity_id.as_ref().map(|v| v.to_string()),
            emulator.activity_id.as_ref().map(|v| v.to_string()),
        ),
        (
            "request_charge",
            real.request_charge.as_ref().map(|v| v.value().to_string()),
            emulator
                .request_charge
                .as_ref()
                .map(|v| v.value().to_string()),
        ),
        (
            "session_token",
            real.session_token.as_ref().map(|v| v.to_string()),
            emulator.session_token.as_ref().map(|v| v.to_string()),
        ),
        (
            "etag",
            real.etag.as_ref().map(|v| v.to_string()),
            emulator.etag.as_ref().map(|v| v.to_string()),
        ),
        (
            "continuation",
            real.continuation.clone(),
            emulator.continuation.clone(),
        ),
        (
            "item_count",
            real.item_count.map(|v| v.to_string()),
            emulator.item_count.map(|v| v.to_string()),
        ),
        (
            "substatus",
            real.substatus.map(|v| v.value().to_string()),
            emulator.substatus.map(|v| v.value().to_string()),
        ),
        (
            "index_metrics",
            real.index_metrics.clone(),
            emulator.index_metrics.clone(),
        ),
        (
            "query_metrics",
            real.query_metrics.clone(),
            emulator.query_metrics.clone(),
        ),
        (
            "server_duration_ms",
            real.server_duration_ms.map(|v| v.to_string()),
            emulator.server_duration_ms.map(|v| v.to_string()),
        ),
        (
            "lsn",
            real.lsn.map(|v| v.to_string()),
            emulator.lsn.map(|v| v.to_string()),
        ),
        (
            "item_lsn",
            real.item_lsn.map(|v| v.to_string()),
            emulator.item_lsn.map(|v| v.to_string()),
        ),
        (
            "offer_replace_pending",
            real.offer_replace_pending.map(|v| v.to_string()),
            emulator.offer_replace_pending.map(|v| v.to_string()),
        ),
        (
            "retry_after_ms",
            real.retry_after_ms.map(|v| v.to_string()),
            emulator.retry_after_ms.map(|v| v.to_string()),
        ),
        (
            "correlated_activity_id",
            real.correlated_activity_id.clone(),
            emulator.correlated_activity_id.clone(),
        ),
        (
            "transport_request_id",
            real.transport_request_id.map(|v| v.to_string()),
            emulator.transport_request_id.map(|v| v.to_string()),
        ),
        (
            "global_committed_lsn",
            real.global_committed_lsn.map(|v| v.to_string()),
            emulator.global_committed_lsn.map(|v| v.to_string()),
        ),
        (
            "quorum_acked_lsn",
            real.quorum_acked_lsn.map(|v| v.to_string()),
            emulator.quorum_acked_lsn.map(|v| v.to_string()),
        ),
        (
            "quorum_acked_local_lsn",
            real.quorum_acked_local_lsn.map(|v| v.to_string()),
            emulator.quorum_acked_local_lsn.map(|v| v.to_string()),
        ),
        (
            "local_lsn",
            real.local_lsn.map(|v| v.to_string()),
            emulator.local_lsn.map(|v| v.to_string()),
        ),
        (
            "item_local_lsn",
            real.item_local_lsn.map(|v| v.to_string()),
            emulator.item_local_lsn.map(|v| v.to_string()),
        ),
        (
            "number_of_read_regions",
            real.number_of_read_regions.map(|v| v.to_string()),
            emulator.number_of_read_regions.map(|v| v.to_string()),
        ),
        (
            "last_state_change_utc",
            real.last_state_change_utc.clone(),
            emulator.last_state_change_utc.clone(),
        ),
        (
            "gateway_version",
            real.gateway_version.clone(),
            emulator.gateway_version.clone(),
        ),
        (
            "service_version",
            real.service_version.clone(),
            emulator.service_version.clone(),
        ),
        (
            "resource_quota",
            real.resource_quota.clone(),
            emulator.resource_quota.clone(),
        ),
        (
            "resource_usage",
            real.resource_usage.clone(),
            emulator.resource_usage.clone(),
        ),
        (
            "has_tentative_writes",
            real.has_tentative_writes.map(|v| v.to_string()),
            emulator.has_tentative_writes.map(|v| v.to_string()),
        ),
        (
            "partition_key_range_id",
            real.partition_key_range_id.clone(),
            emulator.partition_key_range_id.clone(),
        ),
        (
            "internal_partition_id",
            real.internal_partition_id.clone(),
            emulator.internal_partition_id.clone(),
        ),
        (
            "log_results",
            real.log_results.clone(),
            emulator.log_results.clone(),
        ),
        (
            "collection_index_transformation_progress",
            real.collection_index_transformation_progress
                .map(|v| v.to_string()),
            emulator
                .collection_index_transformation_progress
                .map(|v| v.to_string()),
        ),
        (
            "collection_lazy_indexing_progress",
            real.collection_lazy_indexing_progress
                .map(|v| v.to_string()),
            emulator
                .collection_lazy_indexing_progress
                .map(|v| v.to_string()),
        ),
    ]
}

/// Validates a single header field against the match rule.
fn validate_header_field(
    name: &str,
    real: &Option<String>,
    emulator: &Option<String>,
    rule: HeaderMatch,
) {
    match rule {
        HeaderMatch::Exact => {
            assert_eq!(
                real, emulator,
                "Header '{}' exact mismatch: real={:?} emulator={:?}",
                name, real, emulator,
            );
        }
        HeaderMatch::Exists => {
            if real.is_some() {
                assert!(
                    emulator.is_some(),
                    "Header '{}': present in real ({:?}) but missing from emulator",
                    name,
                    real,
                );
            }
        }
        HeaderMatch::NonNegative => {
            if real.is_some() {
                assert!(
                    emulator.is_some(),
                    "Header '{}': present in real but missing from emulator",
                    name,
                );
                if let Some(val) = emulator {
                    let parsed: f64 = val.parse().unwrap_or_else(|_| {
                        panic!(
                            "Header '{}': emulator value '{}' is not a valid number",
                            name, val
                        )
                    });
                    assert!(
                        parsed >= 0.0,
                        "Header '{}': emulator value {} is negative",
                        name,
                        parsed,
                    );
                }
            }
        }
        HeaderMatch::Symmetric => {
            match (real, emulator) {
                (Some(_), None) => {
                    panic!(
                        "Header '{}': present in real ({:?}) but missing from emulator",
                        name, real,
                    );
                }
                (None, Some(_)) => {
                    panic!(
                        "Header '{}': absent in real but present in emulator ({:?})",
                        name, emulator,
                    );
                }
                _ => {} // both present or both absent — OK
            }
        }
        HeaderMatch::RequiredInEmulatorOnly => {
            assert!(
                emulator.is_some(),
                "Header '{}': required in emulator response but missing; real={:?}",
                name,
                real,
            );
        }
        HeaderMatch::Ignore => {}
    }
}

/// Validates body as a Cosmos document: user fields must match, system props
/// only need to exist.
fn validate_body_document(real: &Option<serde_json::Value>, emulator: &Option<serde_json::Value>) {
    match (real, emulator) {
        (None, None) => {}
        (Some(_), None) => panic!("Body present in real response but missing from emulator"),
        (None, Some(_)) => panic!("Body present in emulator response but missing from real"),
        (Some(real_val), Some(emu_val)) => {
            let real_obj = real_val
                .as_object()
                .expect("real body should be a JSON object");
            let emu_obj = emu_val
                .as_object()
                .expect("emulator body should be a JSON object");

            // Every user field in real should exist in emulator with the same value
            for (key, real_field) in real_obj {
                if is_user_field(key) {
                    let emu_field = emu_obj.get(key);
                    assert_eq!(
                        Some(real_field),
                        emu_field,
                        "Body field '{}' mismatch: real={} emulator={:?}",
                        key,
                        real_field,
                        emu_field,
                    );
                }
            }

            // System properties: just check presence in emulator
            for prop in SYSTEM_PROPERTIES {
                if real_obj.contains_key(*prop) {
                    assert!(
                        emu_obj.contains_key(*prop),
                        "System property '{}' present in real but missing from emulator",
                        prop,
                    );
                }
            }
        }
    }
}

/// Validates structural similarity: same keys at every nesting level,
/// but values may differ.
fn validate_body_structural(
    real: &Option<serde_json::Value>,
    emulator: &Option<serde_json::Value>,
) {
    match (real, emulator) {
        (None, None) => {}
        (Some(_), None) => panic!("Body present in real response but missing from emulator"),
        (None, Some(_)) => panic!("Body present in emulator response but missing from real"),
        (Some(real_val), Some(emu_val)) => {
            validate_json_structure("$", real_val, emu_val);
        }
    }
}

fn validate_json_structure(path: &str, real: &serde_json::Value, emulator: &serde_json::Value) {
    match (real, emulator) {
        (serde_json::Value::Object(r), serde_json::Value::Object(e)) => {
            for key in r.keys() {
                assert!(
                    e.contains_key(key),
                    "Key '{}' at path '{}' present in real but missing from emulator",
                    key,
                    path,
                );
            }
            for key in e.keys() {
                assert!(
                    r.contains_key(key),
                    "Key '{}' at path '{}' present in emulator but missing from real",
                    key,
                    path,
                );
            }
            for (key, real_val) in r {
                if let Some(emu_val) = e.get(key) {
                    let child_path = format!("{}.{}", path, key);
                    validate_json_structure(&child_path, real_val, emu_val);
                }
            }
        }
        (serde_json::Value::Array(r), serde_json::Value::Array(e)) => {
            assert_eq!(
                r.len(),
                e.len(),
                "Array length mismatch at path '{}': real={} emulator={}",
                path,
                r.len(),
                e.len(),
            );
        }
        _ => {
            // Leaf values: just check same type
            assert_eq!(
                std::mem::discriminant(real),
                std::mem::discriminant(emulator),
                "Type mismatch at path '{}': real={} emulator={}",
                path,
                real,
                emulator,
            );
        }
    }
}
