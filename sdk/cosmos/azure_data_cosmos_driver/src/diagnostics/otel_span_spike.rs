// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! THROWAWAY feasibility spike — **not production tracing wiring.**
//!
//! Behind the off-by-default `otel_spans_spike` feature, and in-crate so it can build a real
//! [`DiagnosticsContext`](super::DiagnosticsContext) via the `pub(crate)`
//! [`DiagnosticsContextBuilder`](super::DiagnosticsContextBuilder) — importing from
//! `crate::diagnostics` only, with no dependency on any prototype capture engine.
//!
//! It proves the OpenTelemetry mapping in `DIAGNOSTICS-CONTRACT.md` is feasible: a *completed*
//! `DiagnosticsContext` can be reconstructed into a **backdated** OTel span tree using the raw
//! `opentelemetry` [`SpanBuilder`](opentelemetry::trace::SpanBuilder) with
//! [`with_start_time`](opentelemetry::trace::SpanBuilder::with_start_time) /
//! [`with_end_time`](opentelemetry::trace::SpanBuilder::with_end_time) — timestamps the
//! `azure_core::tracing` abstraction does not expose (it builds spans at "now"). The recorded
//! attempts are laid onto an explicit, clearly-backdated timeline (injected durations) and the
//! operation → attempt parent/child relationships are reconstructed, then asserted.

use super::{DiagnosticsContext, DiagnosticsContextBuilder, ExecutionContext};
use crate::driver::routing::CosmosEndpoint;
use crate::models::{ActivityId, SubStatusCode};
use crate::options::{DiagnosticsOptions, Region};
use azure_core::http::StatusCode;

use opentelemetry::trace::{
    Span, SpanBuilder, SpanId, SpanKind, TraceContextExt, Tracer, TracerProvider,
};
use opentelemetry::{Context, KeyValue};
use opentelemetry_sdk::trace::{
    InMemorySpanExporterBuilder, SdkTracerProvider, SimpleSpanProcessor,
};

use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// A clearly-backdated operation start: 2020-01-01T00:00:00Z.
const OP_START_UNIX_SECS: u64 = 1_577_836_800;

/// Builds a realistic completed `DiagnosticsContext` (retry `429` -> `200`) via the
/// `pub(crate)` builder — the same path the driver pipeline uses.
fn completed_context() -> DiagnosticsContext {
    use super::{PipelineType, TransportHttpVersion, TransportKind, TransportSecurity};

    let mut builder = DiagnosticsContextBuilder::new(
        ActivityId::from_string("op-activity-1".to_string()),
        Arc::new(DiagnosticsOptions::default()),
    );

    let start = |b: &mut DiagnosticsContextBuilder, ec, region: Region, url: &str| {
        let endpoint = CosmosEndpoint::regional(region, url::Url::parse(url).unwrap());
        b.start_request(
            ec,
            PipelineType::DataPlane,
            TransportSecurity::Secure,
            TransportKind::Gateway,
            TransportHttpVersion::Http11,
            &endpoint,
        )
    };

    let h1 = start(
        &mut builder,
        ExecutionContext::Initial,
        Region::EAST_US_2,
        "https://east/",
    );
    builder.complete_request(
        h1,
        StatusCode::TooManyRequests,
        Some(SubStatusCode::new(3200)),
    );
    let h2 = start(
        &mut builder,
        ExecutionContext::Retry,
        Region::WEST_US_2,
        "https://west/",
    );
    builder.complete_request(h2, StatusCode::Ok, None);
    builder.set_operation_status(StatusCode::Ok, None);
    builder.complete()
}

#[test]
fn reconstructs_completed_diagnostics_into_backdated_span_tree() {
    let ctx = completed_context();

    let exporter = InMemorySpanExporterBuilder::new().build();
    let provider = SdkTracerProvider::builder()
        .with_span_processor(SimpleSpanProcessor::new(exporter.clone()))
        .build();
    let tracer = provider.tracer("cosmos-diagnostics-retro-spike");

    let op_start = UNIX_EPOCH + Duration::from_secs(OP_START_UNIX_SECS);

    // Lay the recorded attempts out sequentially on the backdated timeline (injected durations).
    let requests = ctx.requests();
    let mut cursor = op_start;
    let mut windows: Vec<(SystemTime, SystemTime)> = Vec::with_capacity(requests.len());
    for (i, _req) in requests.iter().enumerate() {
        let start = cursor;
        let end = start + Duration::from_millis(3 + i as u64);
        windows.push((start, end));
        cursor = end;
    }
    let op_end = cursor;

    // Root (operation) span, backdated.
    let root = tracer.build(
        SpanBuilder::from_name("Cosmos read_item")
            .with_kind(SpanKind::Client)
            .with_start_time(op_start)
            .with_end_time(op_end)
            .with_attributes(vec![
                KeyValue::new(
                    "az.client_request_id",
                    ctx.activity_id().as_str().to_string(),
                ),
                KeyValue::new("db.operation", "read_item"),
            ]),
    );
    let root_cx = Context::current().with_span(root);
    let root_span_id = root_cx.span().span_context().span_id();

    // Attempt (child) spans, backdated, parented to the operation root.
    for (req, (start, end)) in requests.iter().zip(windows.iter().copied()) {
        let mut attrs = vec![
            KeyValue::new("server.address", req.endpoint().to_string()),
            KeyValue::new("db.cosmosdb.request_charge", req.request_charge().value()),
        ];
        if let Some(id) = req.activity_id() {
            // Note: azure_core's constant is `az.service_request.id` (a dot before `id`).
            attrs.push(KeyValue::new(
                "az.service_request.id",
                id.as_str().to_string(),
            ));
        }
        let mut child = tracer.build_with_context(
            SpanBuilder::from_name("read_item attempt")
                .with_kind(SpanKind::Client)
                .with_start_time(start)
                .with_end_time(end)
                .with_attributes(attrs),
            &root_cx,
        );
        child.end_with_timestamp(end);
    }
    root_cx.span().end_with_timestamp(op_end);
    let _ = provider.force_flush();

    // ---- assertions ----
    let spans = exporter
        .get_finished_spans()
        .expect("in-memory exporter returns spans");
    assert_eq!(
        spans.len(),
        ctx.request_count() + 1,
        "one operation root span + one span per attempt"
    );

    let root_data = spans
        .iter()
        .find(|s| s.parent_span_id == SpanId::INVALID)
        .expect("exactly one root span (no parent)");
    assert_eq!(root_data.span_context.span_id(), root_span_id);
    assert_eq!(
        root_data.start_time, op_start,
        "root span carries the injected, backdated operation start"
    );
    let a_year_ago = SystemTime::now()
        .checked_sub(Duration::from_secs(365 * 24 * 3600))
        .expect("valid time");
    assert!(
        root_data.start_time < a_year_ago,
        "root span start is far in the past (retroactive reconstruction, not 'now')"
    );

    let children: Vec<_> = spans
        .iter()
        .filter(|s| s.parent_span_id != SpanId::INVALID)
        .collect();
    assert_eq!(children.len(), ctx.request_count());
    for child in &children {
        assert_eq!(
            child.parent_span_id, root_span_id,
            "attempt spans are children of the operation root"
        );
        assert!(
            child.start_time >= op_start && child.end_time <= op_end,
            "attempt span falls within the operation window"
        );
    }

    // Sequential, non-overlapping layout in our reconstruction.
    let mut ordered = children.clone();
    ordered.sort_by_key(|s| s.start_time);
    for pair in ordered.windows(2) {
        assert!(
            pair[0].end_time <= pair[1].start_time,
            "attempts are laid out sequentially"
        );
    }

    // Attribute alignment: operation carries the client request id, attempts the service
    // request id (the azure_core `az.service_request.id` name, dot included).
    assert!(root_data
        .attributes
        .iter()
        .any(|kv| kv.key.as_str() == "az.client_request_id"));
    assert!(children.iter().all(|c| c
        .attributes
        .iter()
        .any(|kv| kv.key.as_str() == "az.service_request.id")));
}
