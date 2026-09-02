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
    Array, Context, KeyValue, StringValue, Value,
};

use azure_data_cosmos_driver::diagnostics::{
    DiagnosticsContext, ExecutionContext, RequestDiagnostics,
};

use crate::diagnostics::attributes;
use crate::diagnostics::CosmosOperationContext;

/// Span name for the operation ("root") span when the operation name is unknown.
const DEFAULT_OPERATION_SPAN_NAME: &str = "cosmosdb.operation";

/// Span name for each per-attempt ("child") span.
const REQUEST_SPAN_NAME: &str = "cosmosdb.request";

/// Builds an OpenTelemetry ordered `string[]` [`Value`] from region names.
///
/// The Cosmos semantic conventions model region lists (contacted, requested,
/// responded) as ordered `string[]`; this emits an array value rather than a
/// joined scalar.
fn region_string_array<'a>(regions: impl IntoIterator<Item = &'a str>) -> Value {
    let values: Vec<StringValue> = regions
        .into_iter()
        .map(|region| StringValue::from(region.to_string()))
        .collect();
    Value::Array(Array::String(values))
}

/// Emits a backdated operation span with one child span per retained attempt.
///
/// * `tracer` — the OpenTelemetry tracer to build spans on. Generic so tests can
///   pass a `SdkTracer` backed by an in-memory exporter with no global state.
/// * `diagnostics` — the completed context to reconstruct.
/// * `op` — the SDK-supplied operation identity (name, database, container), if
///   present on the pipeline context. Supplies the operation name when the driver
///   context does not carry one, and the `db.namespace` / `db.collection.name`
///   attributes the driver context never knows.
/// * `now_instant` / `now_system` — an anchor pair captured together at emission
///   time; monotonic instants in the context are converted to wall-clock times
///   relative to this anchor.
pub(crate) fn emit_backdated_span_tree<T>(
    tracer: &T,
    diagnostics: &DiagnosticsContext,
    op: Option<&CosmosOperationContext>,
    reason: Option<&str>,
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

    // Wall-clock end of one attempt: its recorded completion, or its start plus
    // reported duration when no completion instant was captured.
    let child_end_of = |req: &RequestDiagnostics| -> SystemTime {
        let start = to_system(req.started_at());
        let end = match req.completed_at() {
            Some(completed) => to_system(completed),
            None => start + Duration::from_millis(req.duration_ms()),
        };
        end.max(start)
    };

    // Anchor the root's end to the last attempt's completion, NOT to
    // `now_system` (the handler's invocation time). Handlers run in registration
    // order, so using the invocation time would fold any earlier handler's delay
    // into this span while the child timestamps stay anchored to the real request
    // instants — inflating and shifting the reconstructed operation span. Fall
    // back to `now_system` only when there are no retained attempts.
    let op_end = requests
        .iter()
        .map(&child_end_of)
        .max()
        .unwrap_or(now_system);
    // The root span must fully contain its backdated children. A normal
    // operation's `duration()` already reaches back to before its first attempt,
    // but an aggregate operation's `duration()` is the SUM of its sub-op
    // durations (see `DiagnosticsContext::aggregate_sub_operations`) and omits
    // the wall-clock gaps between them, so `op_end - duration()` can land AFTER
    // the earliest child. Extend the start back to the earliest retained attempt
    // so the reconstructed root spans first-attempt start to last-attempt end.
    let duration_start = op_end - diagnostics.duration();
    let op_start = requests
        .iter()
        .map(|req| to_system(req.started_at()))
        .min()
        .map(|earliest| earliest.min(duration_start))
        .unwrap_or(duration_start);
    let op_failed = diagnostics.is_failure();

    // --- Operation (root) span ---
    // Prefer the SDK-supplied operation identity (the caller-facing operation,
    // e.g. `patch_item`) so the span label agrees with the metric
    // `db.operation.name` and the tail-sampling classifier, which both read the
    // same `CosmosOperationContext`. Fall back to the driver context's operation
    // name for operations not surfaced through the SDK wrapper (which therefore
    // carry no `CosmosOperationContext`). Preferring the driver value here would
    // mislabel an aggregate whose surfaced sub-op differs from the operation —
    // e.g. a PATCH that fails during its internal Read would report `read_item`
    // on the span while the metric reports `patch_item`.
    let op_name_ref = op
        .and_then(CosmosOperationContext::operation_name)
        .or_else(|| diagnostics.operation_name());
    let op_name = op_name_ref
        .unwrap_or(DEFAULT_OPERATION_SPAN_NAME)
        .to_string();

    let mut root_attrs = vec![KeyValue::new(
        attributes::DB_SYSTEM_NAME,
        attributes::DB_SYSTEM_NAME_VALUE,
    )];
    if let Some(name) = op_name_ref {
        root_attrs.push(KeyValue::new(
            attributes::DB_OPERATION_NAME,
            name.to_string(),
        ));
    }
    if let Some(namespace) = op.and_then(CosmosOperationContext::database_name) {
        root_attrs.push(KeyValue::new(
            attributes::DB_NAMESPACE,
            namespace.to_string(),
        ));
    }
    if let Some(collection) = op.and_then(CosmosOperationContext::container_name) {
        root_attrs.push(KeyValue::new(
            attributes::DB_COLLECTION_NAME,
            collection.to_string(),
        ));
    }
    if let Some(status) = diagnostics.effective_status() {
        root_attrs.push(KeyValue::new(
            attributes::DB_RESPONSE_STATUS_CODE,
            u16::from(status.status_code()).to_string(),
        ));
        if let Some(sub) = status.sub_status() {
            root_attrs.push(KeyValue::new(
                attributes::SUB_STATUS_CODE,
                i64::from(u16::from(sub)),
            ));
        }
    }
    root_attrs.push(KeyValue::new(
        attributes::REQUEST_CHARGE,
        diagnostics.total_request_charge().value(),
    ));
    let regions = diagnostics.regions_contacted();
    if !regions.is_empty() {
        // Semantic conventions define contacted_regions as an ordered string[];
        // emit an OpenTelemetry array, not a joined scalar.
        let values: Vec<StringValue> = regions
            .iter()
            .map(|r| StringValue::from(r.as_str().to_string()))
            .collect();
        root_attrs.push(KeyValue::new(
            attributes::CONTACTED_REGIONS,
            Value::Array(Array::String(values)),
        ));
    }
    // Hedging surfacing: only when a cross-region hedge actually fanned out.
    // These attributes stay off the common (non-hedged) sampled span entirely.
    //
    // Fan-out is decided by `hedging_started()` (materialized from the
    // dispatch-time fan-out log), consistently with the hedged metric counter
    // and the sampled log line. The per-outcome `HEDGE_REGION` /
    // `HEDGE_TERMINAL_STATE` fields are additionally gated on
    // `hedge_diagnostics()`, which is `None` when a both-transient race was
    // resolved by a later failover attempt — so those fields are simply absent
    // there rather than carrying empty or placeholder values. The region
    // history is still emitted on that path, which is exactly where it is most
    // useful.
    //
    // Both region arrays come from the driver already bounded by
    // `max_request_diagnostics`, so a retry storm cannot produce an unbounded
    // span attribute. When the driver elided the middle of a history, the
    // matching `*_total` count is emitted alongside so the truncation is
    // explicit in the telemetry rather than silent.
    if diagnostics.hedging_started() {
        root_attrs.push(KeyValue::new(attributes::HEDGING_STARTED, true));
        if let Some(hedge) = diagnostics.hedge_diagnostics() {
            if let Some(alternate) = hedge.alternate_region() {
                root_attrs.push(KeyValue::new(
                    attributes::HEDGE_REGION,
                    alternate.as_str().to_string(),
                ));
            }
            root_attrs.push(KeyValue::new(
                attributes::HEDGE_TERMINAL_STATE,
                hedge.terminal_state().as_str(),
            ));
        }
        let requested = diagnostics.requested_regions();
        if !requested.is_empty() {
            root_attrs.push(KeyValue::new(
                attributes::REQUESTED_REGIONS,
                region_string_array(requested.iter().map(|r| r.region.as_str())),
            ));
            let total = diagnostics.total_requested_regions();
            if total > requested.len() {
                root_attrs.push(KeyValue::new(
                    attributes::REQUESTED_REGIONS_TOTAL,
                    total as i64,
                ));
            }
        }
        let responded = diagnostics.responded_regions();
        if !responded.is_empty() {
            root_attrs.push(KeyValue::new(
                attributes::RESPONDED_REGIONS,
                region_string_array(responded.iter().map(|region| region.as_str())),
            ));
            let total = diagnostics.total_responded_regions();
            if total > responded.len() {
                root_attrs.push(KeyValue::new(
                    attributes::RESPONDED_REGIONS_TOTAL,
                    total as i64,
                ));
            }
        }
    }
    // Prefer the caller-supplied server-address override (mirroring the metrics
    // handler) before falling back to the host of the first contacted endpoint,
    // so an override changes both the metric and the root span consistently.
    let server_addr = op
        .and_then(CosmosOperationContext::server_address)
        .map(str::to_string)
        .or_else(|| requests.first().and_then(server_address));
    if let Some(addr) = server_addr {
        root_attrs.push(KeyValue::new(attributes::SERVER_ADDRESS, addr));
    }
    if let Some(machine_id) = diagnostics.machine_id() {
        root_attrs.push(KeyValue::new(
            attributes::MACHINE_ID,
            machine_id.to_string(),
        ));
    }
    // Record why the operation was sampled (failure, or which threshold), so the
    // root span carries the same reason the sampled log line does.
    if let Some(reason) = reason {
        root_attrs.push(KeyValue::new(
            attributes::SAMPLING_REASON,
            reason.to_string(),
        ));
    }
    if op_failed {
        if let Some(status) = diagnostics.effective_status() {
            root_attrs.push(KeyValue::new(
                attributes::ERROR_TYPE,
                u16::from(status.status_code()).to_string(),
            ));
        } else {
            // A failure with no status anywhere — report the semconv catch-all so
            // the Error-marked root span still carries an error.type.
            root_attrs.push(KeyValue::new(
                attributes::ERROR_TYPE,
                attributes::ERROR_TYPE_OTHER,
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
    // Each child prefers its own request-level operation name, which is set
    // only when the operation aggregates requests from several sub-operations
    // (a PATCH's `patch_read_item` / `patch_replace_item`). Everywhere else it
    // is unset and the child inherits the operation's name, so a retry storm on
    // a plain `read_item` still labels every attempt `read_item`.
    for req in requests.iter() {
        let child_start = to_system(req.started_at());
        let child_end = child_end_of(req);

        let req_status = req.status();
        let req_failed = !req_status.is_success();

        let mut child_attrs = vec![
            KeyValue::new(attributes::DB_SYSTEM_NAME, attributes::DB_SYSTEM_NAME_VALUE),
            KeyValue::new(
                attributes::DB_RESPONSE_STATUS_CODE,
                u16::from(req_status.status_code()).to_string(),
            ),
            KeyValue::new(attributes::REQUEST_CHARGE, req.request_charge().value()),
        ];
        if let Some(name) = req.operation_name().or(op_name_ref) {
            child_attrs.push(KeyValue::new(
                attributes::DB_OPERATION_NAME,
                name.to_string(),
            ));
        }
        if let Some(sub) = req_status.sub_status() {
            child_attrs.push(KeyValue::new(
                attributes::SUB_STATUS_CODE,
                i64::from(u16::from(sub)),
            ));
        }
        if let Some(region) = req.region() {
            child_attrs.push(KeyValue::new(
                attributes::CONTACTED_REGIONS,
                Value::Array(Array::String(vec![StringValue::from(
                    region.as_str().to_string(),
                )])),
            ));
        }
        // Tag the speculative hedge leg so the child span is attributable to the
        // hedge fan-out rather than an initial/retry dispatch. This tag is
        // present only for a *retained* hedge-leg record: when the primary wins
        // a clean race the alternate leg is structurally cancelled before it
        // produces a per-request record, so no child span exists for it. In that
        // case the authoritative hedge signal lives on the root span
        // (`hedge_started` / `hedge_region` / `hedge_terminal_state`, plus the
        // alternate region in `requested_regions`), so the fan-out is still
        // attributable even without a tagged child.
        if matches!(req.execution_context(), ExecutionContext::Hedging) {
            child_attrs.push(KeyValue::new(attributes::HEDGE_LEG, true));
        }
        if let Some(addr) = server_address(req) {
            child_attrs.push(KeyValue::new(attributes::SERVER_ADDRESS, addr));
        }
        if let Some(activity_id) = req.activity_id() {
            child_attrs.push(KeyValue::new(
                attributes::ACTIVITY_ID,
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
