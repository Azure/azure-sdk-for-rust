// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Reconstructs a backdated span tree from a completed [`DiagnosticsContext`].
//!
//! Because the SDK only decides *whether* to emit a span after the operation
//! has finished (tail-based sampling), the spans it produces must be **backdated**
//! to the times the work actually happened. We use the raw `opentelemetry`
//! [`SpanBuilder`](opentelemetry::trace::SpanBuilder) API — which lets us set an
//! explicit start time and end timestamp — rather than the `azure_core` tracing
//! traits, whose backdating support is not yet available on a shipped release.
//!
//! The tree is: one operation ("root") span, plus one child span per retained
//! attempt in [`DiagnosticsContext::requests`]. Monotonic [`Instant`]s recorded
//! during execution are mapped to wall-clock [`SystemTime`]s using an anchor pair
//! `(now_instant, now_system)` captured at emission time.

use std::time::{Duration, Instant, SystemTime};

use opentelemetry::{
    trace::{Span, SpanKind, Status, TraceContextExt, Tracer},
    Context, KeyValue,
};

use azure_data_cosmos_driver::diagnostics::{DiagnosticsContext, RequestDiagnostics};

use super::attributes;

/// Span name for the operation ("root") span when the operation name is unknown.
const DEFAULT_OPERATION_SPAN_NAME: &str = "cosmosdb.operation";

/// Span name for each per-attempt ("child") span.
const REQUEST_SPAN_NAME: &str = "cosmosdb.request";

/// Emits a backdated operation span with one child span per retained attempt.
///
/// * `tracer` — the OpenTelemetry tracer to build spans on. Generic so tests can
///   pass a `SdkTracer` backed by an in-memory exporter with no global state.
/// * `diagnostics` — the completed context to reconstruct.
/// * `now_instant` / `now_system` — an anchor pair captured together at emission
///   time; monotonic instants in the context are converted to wall-clock times
///   relative to this anchor.
pub(crate) fn emit_backdated_span_tree<T>(
    tracer: &T,
    diagnostics: &DiagnosticsContext,
    now_instant: Instant,
    now_system: SystemTime,
) where
    T: Tracer,
{
    // Map a monotonic `Instant` recorded during execution to wall-clock time.
    let to_system = |instant: Instant| -> SystemTime {
        now_system - now_instant.saturating_duration_since(instant)
    };

    let requests = diagnostics.requests();

    let op_end = now_system;
    let op_start = op_end - diagnostics.duration();
    let op_failed = diagnostics.is_failure();

    // --- Operation (root) span ---
    let op_name = diagnostics
        .operation_name()
        .unwrap_or(DEFAULT_OPERATION_SPAN_NAME)
        .to_string();

    let mut root_attrs = vec![KeyValue::new(
        attributes::DB_SYSTEM_NAME,
        attributes::DB_SYSTEM_NAME_VALUE,
    )];
    if let Some(name) = diagnostics.operation_name() {
        root_attrs.push(KeyValue::new(
            attributes::DB_OPERATION_NAME,
            name.to_string(),
        ));
    }
    if let Some(status) = diagnostics.status() {
        root_attrs.push(KeyValue::new(
            attributes::DB_RESPONSE_STATUS_CODE,
            u16::from(status.status_code()).to_string(),
        ));
        if let Some(sub) = status.sub_status() {
            root_attrs.push(KeyValue::new(
                attributes::AZURE_COSMOSDB_SUB_STATUS_CODE,
                i64::from(u16::from(sub)),
            ));
        }
    }
    root_attrs.push(KeyValue::new(
        attributes::AZURE_COSMOSDB_REQUEST_CHARGE,
        diagnostics.total_request_charge().value(),
    ));
    let regions = diagnostics.regions_contacted();
    if !regions.is_empty() {
        let joined = regions
            .iter()
            .map(|r| r.as_str())
            .collect::<Vec<_>>()
            .join(",");
        root_attrs.push(KeyValue::new(
            attributes::AZURE_COSMOSDB_CONTACTED_REGIONS,
            joined,
        ));
    }
    if let Some(addr) = requests.first().and_then(server_address) {
        root_attrs.push(KeyValue::new(attributes::SERVER_ADDRESS, addr));
    }
    if let Some(machine_id) = diagnostics.machine_id() {
        root_attrs.push(KeyValue::new(
            attributes::AZURE_COSMOSDB_MACHINE_ID,
            machine_id.to_string(),
        ));
    }
    if op_failed {
        if let Some(status) = diagnostics.status() {
            root_attrs.push(KeyValue::new(
                attributes::ERROR_TYPE,
                u16::from(status.status_code()).to_string(),
            ));
        }
    }

    let root_builder = tracer
        .span_builder(op_name)
        .with_kind(SpanKind::Client)
        .with_start_time(op_start)
        .with_attributes(root_attrs);
    let mut root = tracer.build_with_context(root_builder, &Context::current());
    if op_failed {
        root.set_status(Status::error("operation failed"));
    }

    // Children hang off the root via its span context so the exporter records the
    // correct parent/child linkage.
    let parent_cx = Context::current().with_remote_span_context(root.span_context().clone());

    // --- Attempt (child) spans ---
    for req in requests.iter() {
        let child_start = to_system(req.started_at());
        let child_end = match req.completed_at() {
            Some(completed) => to_system(completed),
            None => child_start + Duration::from_millis(req.duration_ms()),
        };
        let child_end = child_end.max(child_start);

        let req_status = req.status();
        let req_failed = !req_status.is_success();

        let mut child_attrs = vec![
            KeyValue::new(attributes::DB_SYSTEM_NAME, attributes::DB_SYSTEM_NAME_VALUE),
            KeyValue::new(
                attributes::DB_RESPONSE_STATUS_CODE,
                u16::from(req_status.status_code()).to_string(),
            ),
            KeyValue::new(
                attributes::AZURE_COSMOSDB_REQUEST_CHARGE,
                req.request_charge().value(),
            ),
        ];
        if let Some(name) = diagnostics.operation_name() {
            child_attrs.push(KeyValue::new(
                attributes::DB_OPERATION_NAME,
                name.to_string(),
            ));
        }
        if let Some(sub) = req_status.sub_status() {
            child_attrs.push(KeyValue::new(
                attributes::AZURE_COSMOSDB_SUB_STATUS_CODE,
                i64::from(u16::from(sub)),
            ));
        }
        if let Some(region) = req.region() {
            child_attrs.push(KeyValue::new(
                attributes::AZURE_COSMOSDB_CONTACTED_REGIONS,
                region.as_str().to_string(),
            ));
        }
        if let Some(addr) = server_address(req) {
            child_attrs.push(KeyValue::new(attributes::SERVER_ADDRESS, addr));
        }
        if let Some(activity_id) = req.activity_id() {
            child_attrs.push(KeyValue::new(
                attributes::AZURE_COSMOSDB_ACTIVITY_ID,
                activity_id.as_str().to_string(),
            ));
        }
        if req_failed {
            child_attrs.push(KeyValue::new(
                attributes::ERROR_TYPE,
                u16::from(req_status.status_code()).to_string(),
            ));
        }

        let child_builder = tracer
            .span_builder(REQUEST_SPAN_NAME)
            .with_kind(SpanKind::Client)
            .with_start_time(child_start)
            .with_attributes(child_attrs);
        let mut child = tracer.build_with_context(child_builder, &parent_cx);
        if req_failed {
            child.set_status(Status::error("request failed"));
        }
        child.end_with_timestamp(child_end);
    }

    root.end_with_timestamp(op_end);
}

/// Extracts the host portion of a request's endpoint URL for `server.address`.
fn server_address(req: &RequestDiagnostics) -> Option<String> {
    url::Url::parse(req.endpoint())
        .ok()
        .and_then(|url| url.host_str().map(str::to_string))
}
