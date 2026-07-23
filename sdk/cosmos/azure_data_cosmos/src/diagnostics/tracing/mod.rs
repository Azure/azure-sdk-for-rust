// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Tail-sampled OpenTelemetry tracing for Cosmos DB operations.
//!
//! This module is gated behind the off-by-default `distributed_tracing` feature. It
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
    use crate::diagnostics::CosmosOperationContext;

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
        assert!(!should_emit_span(&ctx, &thresholds, None));
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
        assert!(should_emit_span(&failed, &thresholds, None));

        // Slow point op (> 1s) → emit.
        let slow = context(
            Duration::from_millis(1500),
            Some(CosmosStatus::new(StatusCode::Ok)),
            Some("read_item"),
            &[(1500, 1500, CosmosStatus::new(StatusCode::Ok))],
            now,
        );
        assert!(should_emit_span(&slow, &thresholds, None));
    }

    #[test]
    fn non_point_threshold_uses_operation_name_from_context() {
        // Production driver contexts carry no operation name, so a 2s operation
        // would be classified as a point op (1s threshold) and wrongly sampled.
        // Passing the SDK operation identity switches it to the non-point (3s)
        // threshold.
        let thresholds = DiagnosticsThresholds::default();
        let now = Instant::now();
        let slow_non_point = context(
            Duration::from_millis(2000),
            Some(CosmosStatus::new(StatusCode::Ok)),
            None, // driver context has no operation name (the production case)
            &[(2000, 2000, CosmosStatus::new(StatusCode::Ok))],
            now,
        );

        // Without an operation identity: falls back to the point (1s) threshold
        // and emits.
        assert!(should_emit_span(&slow_non_point, &thresholds, None));

        // With the SDK operation identity: a query is a non-point op (3s
        // threshold), so a 2s operation is NOT sampled.
        let op = CosmosOperationContext::new().with_operation_name("query_items");
        assert!(!should_emit_span(&slow_non_point, &thresholds, Some(&op)));
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

        emit_backdated_span_tree(&tracer, &ctx, None, now_instant, now_system);
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

    #[test]
    fn op_context_supplies_identity_when_driver_context_lacks_it() {
        let (provider, exporter) = exportable();
        let tracer = provider.tracer("test");

        let now_instant = Instant::now();
        let now_system = SystemTime::now();
        // A slow operation with NO driver-side operation name — the production
        // case, since the driver never records one. The SDK-supplied
        // CosmosOperationContext carries the operation identity instead.
        let ctx = context(
            Duration::from_millis(1500),
            Some(CosmosStatus::new(StatusCode::Ok)),
            None,
            &[(1500, 1500, CosmosStatus::new(StatusCode::Ok))],
            now_instant,
        );
        let op = CosmosOperationContext::new()
            .with_operation_name("read_item")
            .with_database_name("my_db")
            .with_container_name("my_container");

        emit_backdated_span_tree(&tracer, &ctx, Some(&op), now_instant, now_system);
        provider.force_flush().unwrap();

        let spans = exporter.get_finished_spans().unwrap();
        // The op context names the root span and supplies db.operation.name,
        // db.namespace, and db.collection.name.
        let root = spans
            .iter()
            .find(|s| s.name == "read_item")
            .expect("root span named from op context");
        assert!(root.attributes.iter().any(|kv| {
            kv.key.as_str() == attributes::DB_OPERATION_NAME && kv.value.as_str() == "read_item"
        }));
        assert!(root.attributes.iter().any(|kv| {
            kv.key.as_str() == attributes::DB_NAMESPACE && kv.value.as_str() == "my_db"
        }));
        assert!(root.attributes.iter().any(|kv| {
            kv.key.as_str() == attributes::DB_COLLECTION_NAME && kv.value.as_str() == "my_container"
        }));
    }

    #[test]
    fn incomplete_context_is_not_sampled_even_when_slow() {
        // A finalized context with neither a status nor any attempts does not
        // represent a completed operation. The tail-sampling gate must not emit
        // for it even when its elapsed duration alone crosses a threshold.
        let thresholds = DiagnosticsThresholds::default();
        let ctx = DiagnosticsContext::for_testing_with_requests(
            ActivityId::new_uuid(),
            Duration::from_millis(5000),
            None,
            None,
            Vec::new(),
        );
        assert!(!ctx.is_completed());
        assert!(!should_emit_span(&ctx, &thresholds, None));
    }

    #[test]
    fn op_context_server_address_overrides_endpoint_host() {
        // The tracing root span must honor a caller-supplied server.address
        // override (as the metrics handler does), not just the endpoint host.
        let (provider, exporter) = exportable();
        let tracer = provider.tracer("test");
        let now_instant = Instant::now();
        let now_system = SystemTime::now();
        let ctx = context(
            Duration::from_millis(1500),
            Some(CosmosStatus::new(StatusCode::Ok)),
            Some("read_item"),
            &[(1500, 1500, CosmosStatus::new(StatusCode::Ok))],
            now_instant,
        );
        let op = CosmosOperationContext::new()
            .with_operation_name("read_item")
            .with_server_address("override.example.com");

        emit_backdated_span_tree(&tracer, &ctx, Some(&op), now_instant, now_system);
        provider.force_flush().unwrap();

        let spans = exporter.get_finished_spans().unwrap();
        let root = spans
            .iter()
            .find(|s| s.name == "read_item")
            .expect("root span present");
        assert!(
            root.attributes.iter().any(|kv| {
                kv.key.as_str() == attributes::SERVER_ADDRESS
                    && kv.value.as_str() == "override.example.com"
            }),
            "root server.address must honor the op-context override"
        );
    }

    #[test]
    fn root_span_contains_children_when_duration_underestimates_window() {
        // An aggregate operation's duration() is the SUM of its sub-op durations
        // and omits the gaps between them, so `op_end - duration()` can fall
        // AFTER the earliest attempt. The reconstructed root must still start no
        // later than its earliest child so the span tree stays well-formed.
        let (provider, exporter) = exportable();
        let tracer = provider.tracer("test");
        let now_instant = Instant::now();
        let now_system = SystemTime::now();
        // duration (50ms) is far smaller than the 250ms window the attempts span.
        let ctx = context(
            Duration::from_millis(50),
            Some(CosmosStatus::new(StatusCode::TooManyRequests)),
            Some("read_item"),
            &[
                (250, 40, CosmosStatus::new(StatusCode::TooManyRequests)),
                (60, 40, CosmosStatus::new(StatusCode::Ok)),
            ],
            now_instant,
        );

        emit_backdated_span_tree(&tracer, &ctx, None, now_instant, now_system);
        provider.force_flush().unwrap();

        let spans = exporter.get_finished_spans().unwrap();
        let root = spans
            .iter()
            .find(|s| s.name == "read_item")
            .expect("root span present");
        let earliest_child = spans
            .iter()
            .filter(|s| s.name == "cosmosdb.request")
            .map(|s| s.start_time)
            .min()
            .expect("attempt children present");
        assert!(
            root.start_time <= earliest_child,
            "root must start no later than its earliest child"
        );
    }
}
