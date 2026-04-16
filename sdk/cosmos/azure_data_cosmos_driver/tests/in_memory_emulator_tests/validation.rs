// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Response validation framework for comparing real Cosmos DB responses against
//! the in-memory emulator.
//!
//! Each header field can be validated at one of several strictness levels:
//! - `Exact`: values must match exactly
//! - `Exists`: both must be present (values may differ)
//! - `NonNegative`: both must be present and non-negative numbers
//! - `Ignore`: skip validation for this field
//!
//! Status codes must always match. Body payloads are compared structurally with
//! per-field rules for system properties (e.g. `_rid`, `_ts`) that differ
//! between backends.

use std::collections::HashMap;

use azure_data_cosmos_driver::models::CosmosResponseHeaders;
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
    /// Most Cosmos-specific headers should be present in both; values like
    /// activity_id, session_token, and etag will differ between backends.
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
            .with_rule("lsn", HeaderMatch::Ignore)
    }
}

/// Snapshot of a [`CosmosResponse`] for deferred comparison.
pub struct ResponseSnapshot {
    pub status_code: u16,
    pub sub_status_code: Option<u32>,
    pub headers: CosmosResponseHeaders,
    pub body: Option<serde_json::Value>,
    #[allow(dead_code)]
    pub label: String,
}

impl ResponseSnapshot {
    /// Captures a snapshot from a `CosmosResponse`.
    pub fn capture(response: &CosmosResponse, label: impl Into<String>) -> Self {
        let body = if response.body().is_empty() {
            None
        } else {
            serde_json::from_slice(response.body()).ok()
        };
        Self {
            status_code: u16::from(response.status().status_code()),
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
    let header_pairs = extract_header_pairs(&real.headers, &emulator.headers);
    for (name, real_val, emu_val) in &header_pairs {
        validate_header_field(name, real_val, emu_val, header_spec.rule_for(name));
    }

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
            real.activity_id.as_ref().map(|v| v.as_str().to_owned()),
            emulator.activity_id.as_ref().map(|v| v.as_str().to_owned()),
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
            real.session_token.as_ref().map(|v| v.as_str().to_owned()),
            emulator
                .session_token
                .as_ref()
                .map(|v| v.as_str().to_owned()),
        ),
        (
            "etag",
            real.etag.as_ref().map(|v| v.as_str().to_owned()),
            emulator.etag.as_ref().map(|v| v.as_str().to_owned()),
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
