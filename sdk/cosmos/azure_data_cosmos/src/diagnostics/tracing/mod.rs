// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Tail-sampled OpenTelemetry tracing for Cosmos DB operations.
//!
//! This module is gated behind the off-by-default `otel_tracing` feature. It
//! provides [`CosmosTracingHandler`], a [`DiagnosticsHandler`](crate::diagnostics::DiagnosticsHandler)
//! that reconstructs a **backdated** span tree — one operation span plus one child
//! per retained attempt — for operations selected by tail-based sampling (failures
//! and threshold breaches). Fast, successful operations emit nothing.

mod attributes;
mod handler;
mod span_builder;

pub use handler::CosmosTracingHandler;

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant, SystemTime};

    use azure_core::http::StatusCode;
    use azure_data_cosmos_driver::diagnostics::{DiagnosticsContext, RequestDiagnostics};
    use azure_data_cosmos_driver::models::{ActivityId, RequestCharge};
    use azure_data_cosmos_driver::options::Region;
    use azure_data_cosmos_driver::{CosmosStatus, DiagnosticsThresholds};
    use opentelemetry::trace::{SpanId, TracerProvider};
    use opentelemetry_sdk::trace::{in_memory_exporter::InMemorySpanExporter, SdkTracerProvider};

    use super::attributes;
    use super::handler::should_emit_span;
    use super::span_builder::emit_backdated_span_tree;

    /// Builds a completed context: `duration` long, final `status`, and one
    /// synthetic attempt per `(offset_ms, dur_ms, status)` triple. `offset_ms` is
    /// how far before `anchor` the attempt started.
    fn context(
        duration: Duration,
        status: Option<CosmosStatus>,
        operation_name: Option<&str>,
        attempts: &[(u64, u64, CosmosStatus)],
        anchor: Instant,
    ) -> DiagnosticsContext {
        let requests = attempts
            .iter()
            .map(|(offset_ms, dur_ms, req_status)| {
                let started = anchor - Duration::from_millis(*offset_ms);
                let completed = started + Duration::from_millis(*dur_ms);
                RequestDiagnostics::for_testing(
                    "https://acct.documents.azure.com:443/",
                    Some(Region::new("West US 2")),
                    *req_status,
                    RequestCharge::new(2.0),
                    started,
                    completed,
                )
            })
            .collect();
        DiagnosticsContext::for_testing_with_requests(
            ActivityId::new_uuid(),
            duration,
            status,
            operation_name,
            requests,
        )
    }

    fn exportable() -> (SdkTracerProvider, InMemorySpanExporter) {
        let exporter = InMemorySpanExporter::default();
        let provider = SdkTracerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();
        (provider, exporter)
    }

    #[test]
    fn fast_success_emits_no_span() {
        // A 5ms successful point read is below every threshold and is not a
        // failure, so tail-based sampling must skip it entirely.
        let thresholds = DiagnosticsThresholds::default();
        let now = Instant::now();
        let ctx = context(
            Duration::from_millis(5),
            Some(CosmosStatus::new(StatusCode::Ok)),
            Some("read_item"),
            &[(5, 5, CosmosStatus::new(StatusCode::Ok))],
            now,
        );
        assert!(!should_emit_span(&ctx, &thresholds));
    }

    #[test]
    fn failure_and_slow_are_sampled() {
        let thresholds = DiagnosticsThresholds::default();
        let now = Instant::now();

        // Failure → emit.
        let failed = context(
            Duration::from_millis(5),
            Some(CosmosStatus::new(StatusCode::TooManyRequests)),
            Some("read_item"),
            &[(5, 5, CosmosStatus::new(StatusCode::TooManyRequests))],
            now,
        );
        assert!(should_emit_span(&failed, &thresholds));

        // Slow point op (> 1s) → emit.
        let slow = context(
            Duration::from_millis(1500),
            Some(CosmosStatus::new(StatusCode::Ok)),
            Some("read_item"),
            &[(1500, 1500, CosmosStatus::new(StatusCode::Ok))],
            now,
        );
        assert!(should_emit_span(&slow, &thresholds));
    }

    #[test]
    fn emits_backdated_parent_child_tree() {
        let (provider, exporter) = exportable();
        let tracer = provider.tracer("test");

        let now_instant = Instant::now();
        let now_system = SystemTime::now();
        // A ~250ms failed operation with two attempts.
        let ctx = context(
            Duration::from_millis(250),
            Some(CosmosStatus::new(StatusCode::TooManyRequests)),
            Some("read_item"),
            &[
                (250, 100, CosmosStatus::new(StatusCode::TooManyRequests)),
                (120, 110, CosmosStatus::new(StatusCode::TooManyRequests)),
            ],
            now_instant,
        );

        emit_backdated_span_tree(&tracer, &ctx, now_instant, now_system);
        provider.force_flush().unwrap();

        let spans = exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 3, "root + two attempt children");

        // The root span carries the operation name and has no parent.
        let root = spans
            .iter()
            .find(|s| s.name == "read_item")
            .expect("root span present");
        assert_eq!(root.parent_span_id, SpanId::INVALID);

        // Root is backdated into the past and lasts a non-zero interval.
        assert!(root.end_time <= now_system);
        assert!(root.start_time < root.end_time);
        let root_span_id = root.span_context.span_id();

        let children: Vec<_> = spans
            .iter()
            .filter(|s| s.name == "cosmosdb.request")
            .collect();
        assert_eq!(children.len(), 2);
        for child in &children {
            // Correct parentage and backdated (past) timestamps.
            assert_eq!(child.parent_span_id, root_span_id);
            assert!(child.end_time <= now_system);
            assert!(child.start_time <= child.end_time);
        }

        // Semantic-convention attributes are present on the root span.
        assert!(root.attributes.iter().any(|kv| {
            kv.key.as_str() == attributes::DB_SYSTEM_NAME
                && kv.value.as_str() == attributes::DB_SYSTEM_NAME_VALUE
        }));
        assert!(root
            .attributes
            .iter()
            .any(|kv| kv.key.as_str() == attributes::DB_OPERATION_NAME));
    }
}
