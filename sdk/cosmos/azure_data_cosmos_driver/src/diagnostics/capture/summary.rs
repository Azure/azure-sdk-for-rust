// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The aggregatable **summary** reducer and the `AZD1` detail projection.
//!
//! The summary is the always-built, request-style roll-up: a flat, aggregatable record close in
//! spirit to .NET's `CosmosDiagnostics` summary. Its field set is seeded from the SDK's existing
//! high-value telemetry signals — final/per-attempt status, error classification, service request
//! id, request charge (RU), retry/throttle counts, and total elapsed time — so a reader gets the
//! signals a TSG branches on first, without scrolling a tree.
//!
//! When the binary detail tier is opted in, the same parsed log is projected into a
//! [`WireTree`](super::wire::WireTree) and encoded as `AZD1`.

use super::attrs;
use super::preamble;
use super::recorder::Parsed;
use super::wire::{self, NodeKind, WireNode, WireTree};
use super::Outcome;
use azure_core::fmt::SafeDebug;
use serde::Serialize;
use std::collections::BTreeMap;

/// Compact SDK/driver/User-Agent provenance, rehydrated once from the process-global preamble.
#[derive(Clone, SafeDebug, Serialize)]
pub struct ClientInfo {
    /// SDK name + version (the public crate when registered, else the driver).
    pub sdk_version: String,
    /// Cosmos driver version.
    pub driver_version: String,
    /// Full User-Agent string in the SDK's canonical shape.
    pub user_agent: String,
}

fn client_info() -> ClientInfo {
    let p = preamble::get();
    ClientInfo {
        sdk_version: format!("{} {}", p.sdk_name, p.sdk_version),
        driver_version: p.driver_version.clone(),
        user_agent: p.user_agent(),
    }
}

/// The first error encountered, surfaced in the summary.
#[derive(Clone, SafeDebug, Serialize)]
pub struct TopError {
    /// HTTP status.
    pub status: u16,
    /// Coarse error classification.
    pub error_kind: String,
    /// Cosmos sub-status code (finer classification), when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_status: Option<u16>,
    /// Service request id of the failing attempt.
    pub service_request_id: String,
}

/// The default, aggregatable summary view (built only past the gate).
#[derive(Clone, SafeDebug, Serialize)]
pub struct Summary {
    /// Operation name.
    pub operation: String,
    /// `success` or `error`.
    pub outcome: &'static str,
    /// Total attempt count.
    pub attempt_count: u32,
    /// Retries (attempts beyond the first).
    pub retry_count: u32,
    /// Total elapsed time (nanoseconds).
    pub total_elapsed_ns: u64,
    /// Summed request charge across attempts (RU), as `f64` for exactness.
    pub total_request_charge: f64,
    /// Count of throttled (`429`) attempts.
    pub throttle_count: u32,
    /// Status-code histogram (status string -> count).
    pub status_counts: BTreeMap<String, u32>,
    /// Service request id of the final attempt.
    pub final_service_request_id: Option<String>,
    /// Number of fan-out children.
    pub child_count: usize,
    /// The first error, when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_error: Option<TopError>,
    /// Compact client/version provenance (from the interned preamble).
    pub client: ClientInfo,
}

impl Summary {
    /// Serializes the summary to compact JSON.
    pub fn to_json(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Summary serializes")
    }

    /// Serializes the summary to pretty JSON (for samples / human reading).
    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).expect("Summary serializes")
    }
}

/// Coarse error classification for a status code; `None` for 2xx.
fn error_kind(status: u16) -> Option<&'static str> {
    if (200..300).contains(&status) {
        return None;
    }
    Some(match status {
        429 => "throttled",
        404 => "not_found",
        400..=499 => "client_error",
        500..=599 => "server_error",
        _ => "unknown",
    })
}

/// Reduces a parsed capture log to the aggregatable [`Summary`].
pub(crate) fn summarize(parsed: &Parsed) -> Summary {
    let mut status_counts: BTreeMap<String, u32> = BTreeMap::new();
    let mut total_request_charge = 0.0f64;
    let mut throttle_count = 0u32;
    let mut top_error: Option<TopError> = None;

    for attempt in &parsed.attempts {
        *status_counts.entry(attempt.status.to_string()).or_insert(0) += 1;
        total_request_charge += f64::from(attempt.request_charge);
        if attempt.status == 429 {
            throttle_count += 1;
        }
        if top_error.is_none() {
            if let Some(kind) = error_kind(attempt.status) {
                top_error = Some(TopError {
                    status: attempt.status,
                    error_kind: kind.to_string(),
                    sub_status: attempt.sub_status,
                    service_request_id: attempt.service_request_id.clone(),
                });
            }
        }
    }

    Summary {
        operation: parsed.operation.clone(),
        outcome: match parsed.outcome {
            Outcome::Success => "success",
            Outcome::Error => "error",
        },
        attempt_count: parsed.attempt_count,
        retry_count: parsed.attempt_count.saturating_sub(1),
        total_elapsed_ns: parsed.total_ns,
        // RU is carried compactly as f32 on the hot path; round on the way out so the f32 -> f64
        // widening does not leak noise like 8.39999962 into the summary.
        total_request_charge: (total_request_charge * 10_000.0).round() / 10_000.0,
        throttle_count,
        status_counts,
        final_service_request_id: parsed.attempts.last().map(|a| a.service_request_id.clone()),
        child_count: parsed.children.len(),
        top_error,
        client: client_info(),
    }
}

/// Projects a parsed capture log into the `AZD1` [`WireTree`] (the detail tier).
pub(crate) fn build_wiretree(parsed: &Parsed) -> WireTree {
    let info = client_info();
    let mut nodes = Vec::with_capacity(1 + parsed.attempts.len() + parsed.children.len());
    nodes.push(WireNode {
        parent: None,
        kind: NodeKind::Operation as u8,
        start_ns: 0,
        duration_ns: parsed.total_ns,
        status: 0,
        attrs: vec![
            (attrs::ATTR_OPERATION.to_string(), parsed.operation.clone()),
            (attrs::ATTR_ENDPOINT.to_string(), parsed.endpoint.clone()),
            (
                attrs::ATTR_CLIENT_REQUEST_ID.to_string(),
                parsed.client_request_id.clone(),
            ),
            (
                attrs::ATTR_ATTEMPT_COUNT.to_string(),
                parsed.attempt_count.to_string(),
            ),
            (
                attrs::ATTR_OUTCOME.to_string(),
                match parsed.outcome {
                    Outcome::Success => "success".to_string(),
                    Outcome::Error => "error".to_string(),
                },
            ),
            (attrs::ATTR_SDK_VERSION.to_string(), info.sdk_version),
            (attrs::ATTR_DRIVER_VERSION.to_string(), info.driver_version),
            (attrs::ATTR_USER_AGENT.to_string(), info.user_agent),
        ],
    });
    for attempt in &parsed.attempts {
        let mut node_attrs = vec![
            ("attempt_index".to_string(), attempt.index.to_string()),
            (
                attrs::ATTR_STATUS_CODE.to_string(),
                attempt.status.to_string(),
            ),
            (
                attrs::ATTR_SERVICE_REQUEST_ID.to_string(),
                attempt.service_request_id.clone(),
            ),
            (
                attrs::ATTR_REQUEST_CHARGE.to_string(),
                attempt.request_charge.to_string(),
            ),
        ];
        if let Some(kind) = error_kind(attempt.status) {
            node_attrs.push((attrs::ATTR_ERROR_KIND.to_string(), kind.to_string()));
        }
        if let Some(sub) = attempt.sub_status {
            node_attrs.push((attrs::ATTR_SUB_STATUS.to_string(), sub.to_string()));
        }
        if !attempt.request_sent.is_empty() {
            node_attrs.push((
                attrs::ATTR_REQUEST_SENT.to_string(),
                attempt.request_sent.clone(),
            ));
        }
        nodes.push(WireNode {
            parent: Some(0),
            kind: NodeKind::Attempt as u8,
            start_ns: attempt.start_ns,
            duration_ns: attempt.duration_ns,
            status: attempt.status,
            attrs: node_attrs,
        });
    }
    for child in &parsed.children {
        nodes.push(WireNode {
            parent: Some(0),
            kind: NodeKind::Routing as u8,
            start_ns: child.start_ns,
            duration_ns: child.duration_ns,
            status: 0,
            attrs: vec![
                (
                    attrs::ATTR_PLAN_NODE_ID.to_string(),
                    child.plan_node_id.clone(),
                ),
                (attrs::ATTR_FEED_RANGE.to_string(), child.feed_range.clone()),
            ],
        });
    }
    WireTree {
        operation: parsed.operation.clone(),
        nodes,
    }
}

/// Encodes a parsed log to the `AZD1` binary blob (auto-compressing large payloads).
pub(crate) fn build_detailed_blob(parsed: &Parsed) -> Vec<u8> {
    wire::encode_auto(&build_wiretree(parsed))
}
