// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Tail-sampled OpenTelemetry tracing for Cosmos DB operations.
//!
//! This module is gated behind the off-by-default `distributed_tracing` feature. It
//! provides [`CosmosTracingHandler`], a [`DiagnosticsHandler`](crate::diagnostics::DiagnosticsHandler)
//! that reconstructs a **backdated** span tree — one operation span plus one child
//! per retained attempt — for operations selected by tail-based sampling (failures
//! and threshold breaches). Fast, successful operations emit nothing.

mod handler;
mod span_builder;

pub use handler::CosmosTracingHandler;

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant, SystemTime};

    use azure_core::http::StatusCode;
    use azure_data_cosmos_driver::diagnostics::{
        DiagnosticsContext, ExecutionContext, HedgeDiagnostics, HedgeTerminalState,
        RequestDiagnostics,
    };
    use azure_data_cosmos_driver::models::{ActivityId, RequestCharge};
    use azure_data_cosmos_driver::options::Region;
    use azure_data_cosmos_driver::{CosmosStatus, DiagnosticsThresholds};
    use opentelemetry::trace::{SpanId, TracerProvider};
    use opentelemetry::{Array, Value};
    use opentelemetry_sdk::trace::{in_memory_exporter::InMemorySpanExporter, SdkTracerProvider};

    use super::handler::should_emit_span;
    use super::span_builder::emit_backdated_span_tree;
    use crate::diagnostics::attributes;
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

        emit_backdated_span_tree(&tracer, &ctx, None, None, now_instant, now_system);
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
        // A slow operation with NO driver-side operation name — exercising the
        // op-context fallback path. The SDK-supplied CosmosOperationContext
        // carries the operation identity instead.
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

        emit_backdated_span_tree(&tracer, &ctx, Some(&op), None, now_instant, now_system);
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
    fn op_context_operation_name_wins_over_driver_context() {
        // When both the driver context and the SDK operation context carry a
        // name, the span must use the caller-facing op-context identity — so a
        // PATCH whose surfaced sub-op is a Replace is still labeled `patch_item`,
        // consistent with the `db.operation.name` metric and the tail-sampling
        // classifier (which both read the op context).
        let (provider, exporter) = exportable();
        let tracer = provider.tracer("test");

        let now_instant = Instant::now();
        let now_system = SystemTime::now();
        let ctx = context(
            Duration::from_millis(1500),
            Some(CosmosStatus::new(StatusCode::Ok)),
            Some("replace_item"), // driver-side sub-op name
            &[(1500, 1500, CosmosStatus::new(StatusCode::Ok))],
            now_instant,
        );
        let op = CosmosOperationContext::new().with_operation_name("patch_item");

        emit_backdated_span_tree(&tracer, &ctx, Some(&op), None, now_instant, now_system);
        provider.force_flush().unwrap();

        let spans = exporter.get_finished_spans().unwrap();
        let root = spans
            .iter()
            .find(|s| s.name == "patch_item")
            .expect("root span named from op context, not the driver sub-op");
        assert!(root.attributes.iter().any(|kv| {
            kv.key.as_str() == attributes::DB_OPERATION_NAME && kv.value.as_str() == "patch_item"
        }));
        assert!(
            !spans.iter().any(|s| s.name == "replace_item"),
            "the driver sub-op name must not label the operation span"
        );
    }

    #[test]
    fn patch_sub_operations_are_visible_on_attempt_spans() {
        // A PATCH is one caller-facing operation (`patch_item`) implemented as a
        // read + replace. The root span keeps the caller's name, but each
        // attempt span must say which half of the read-modify-write it was, so
        // an operator can tell a slow/failing Read from a slow/failing Replace
        // without the decomposition being flattened away.
        let (provider, exporter) = exportable();
        let tracer = provider.tracer("test");

        let now_instant = Instant::now();
        let now_system = SystemTime::now();
        let started = now_instant - Duration::from_millis(1500);
        let requests = vec![
            RequestDiagnostics::for_testing(
                "https://acct.documents.azure.com:443/",
                Some(Region::new("West US 2")),
                CosmosStatus::new(StatusCode::Ok),
                RequestCharge::new(1.0),
                started,
                started + Duration::from_millis(500),
            )
            .for_testing_with_operation_name("patch_read_item"),
            RequestDiagnostics::for_testing(
                "https://acct.documents.azure.com:443/",
                Some(Region::new("West US 2")),
                CosmosStatus::new(StatusCode::Ok),
                RequestCharge::new(4.0),
                started + Duration::from_millis(600),
                started + Duration::from_millis(1500),
            )
            .for_testing_with_operation_name("patch_replace_item"),
        ];
        let ctx = DiagnosticsContext::for_testing_with_requests(
            ActivityId::new_uuid(),
            Duration::from_millis(1500),
            Some(CosmosStatus::new(StatusCode::Ok)),
            Some("patch_item"),
            requests,
        );
        let op = CosmosOperationContext::new().with_operation_name("patch_item");

        emit_backdated_span_tree(&tracer, &ctx, Some(&op), None, now_instant, now_system);
        provider.force_flush().unwrap();

        let spans = exporter.get_finished_spans().unwrap();
        let root = spans
            .iter()
            .find(|s| s.name == "patch_item")
            .expect("root span keeps the caller-facing operation name");
        assert!(
            root.attributes.iter().any(|kv| {
                kv.key.as_str() == attributes::DB_OPERATION_NAME
                    && kv.value.as_str() == "patch_item"
            }),
            "the operation the caller invoked is still `patch_item`"
        );

        let child_names: Vec<String> = spans
            .iter()
            .filter(|s| s.name == "cosmosdb.request")
            .filter_map(|s| {
                s.attributes
                    .iter()
                    .find(|kv| kv.key.as_str() == attributes::DB_OPERATION_NAME)
                    .map(|kv| kv.value.as_str().to_string())
            })
            .collect();
        assert_eq!(
            child_names,
            vec!["patch_read_item", "patch_replace_item"],
            "each attempt span names the sub-operation that issued it"
        );
    }

    #[test]
    fn attempt_spans_fall_back_to_the_operation_name() {
        // The non-PATCH case: attempts carry no per-request name, so every child
        // inherits the operation's identity exactly as before. This is the
        // overwhelmingly common path and must not regress.
        let (provider, exporter) = exportable();
        let tracer = provider.tracer("test");

        let now_instant = Instant::now();
        let now_system = SystemTime::now();
        let ctx = context(
            Duration::from_millis(1500),
            Some(CosmosStatus::new(StatusCode::Ok)),
            Some("read_item"),
            &[
                (1500, 700, CosmosStatus::new(StatusCode::TooManyRequests)),
                (700, 700, CosmosStatus::new(StatusCode::Ok)),
            ],
            now_instant,
        );

        emit_backdated_span_tree(&tracer, &ctx, None, None, now_instant, now_system);
        provider.force_flush().unwrap();

        let spans = exporter.get_finished_spans().unwrap();
        let children: Vec<_> = spans
            .iter()
            .filter(|s| s.name == "cosmosdb.request")
            .collect();
        assert_eq!(children.len(), 2);
        for child in children {
            assert!(
                child.attributes.iter().any(|kv| {
                    kv.key.as_str() == attributes::DB_OPERATION_NAME
                        && kv.value.as_str() == "read_item"
                }),
                "an unnamed attempt inherits the operation name"
            );
        }
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

        emit_backdated_span_tree(&tracer, &ctx, Some(&op), None, now_instant, now_system);
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

        emit_backdated_span_tree(&tracer, &ctx, None, None, now_instant, now_system);
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

    /// Builds a hedged operation in its **production `AlternateWon` shape**: the
    /// primary East US leg lost the race and was structurally dropped (so it has
    /// no retained per-request record), leaving only the winning speculative
    /// hedge leg to West US 2 (200) tagged `ExecutionContext::Hedging`, with
    /// `AlternateWon` hedge diagnostics naming both regions. The dropped primary
    /// region is recovered for `requested_regions()` from the hedge diagnostics.
    fn hedged_context(anchor: Instant) -> DiagnosticsContext {
        let hedge_leg = RequestDiagnostics::for_testing(
            "https://acct-westus2.documents.azure.com:443/",
            Some(Region::WEST_US_2),
            CosmosStatus::new(StatusCode::Ok),
            RequestCharge::new(2.0),
            anchor - Duration::from_millis(150),
            anchor - Duration::from_millis(50),
        )
        .with_execution_context_for_testing(ExecutionContext::Hedging);
        let hedge = HedgeDiagnostics::for_testing(
            Region::EAST_US,
            Some(Region::WEST_US_2),
            Some(Region::WEST_US_2),
            HedgeTerminalState::AlternateWon,
        );
        DiagnosticsContext::for_testing_with_hedge(
            ActivityId::new_uuid(),
            Duration::from_millis(250),
            Some(CosmosStatus::new(StatusCode::Ok)),
            Some("read_item"),
            vec![hedge_leg],
            Some(hedge),
        )
    }

    /// Returns the string members of a `string[]` span attribute, if present.
    fn string_array_attr(
        span: &opentelemetry_sdk::trace::SpanData,
        key: &str,
    ) -> Option<Vec<String>> {
        span.attributes.iter().find_map(|kv| {
            if kv.key.as_str() != key {
                return None;
            }
            match &kv.value {
                Value::Array(Array::String(values)) => {
                    Some(values.iter().map(|v| v.as_str().to_string()).collect())
                }
                _ => None,
            }
        })
    }

    #[test]
    fn hedged_operation_surfaces_hedging_span_attributes() {
        let (provider, exporter) = exportable();
        let tracer = provider.tracer("test");
        let now_instant = Instant::now();
        let now_system = SystemTime::now();

        let ctx = hedged_context(now_instant);
        emit_backdated_span_tree(&tracer, &ctx, None, None, now_instant, now_system);
        provider.force_flush().unwrap();

        let spans = exporter.get_finished_spans().unwrap();
        let root = spans
            .iter()
            .find(|s| s.name == "read_item")
            .expect("root span present");

        // hedging_started = true (bool).
        assert!(
            root.attributes.iter().any(|kv| {
                kv.key.as_str() == attributes::HEDGING_STARTED
                    && matches!(kv.value, Value::Bool(true))
            }),
            "root span must carry hedging_started = true"
        );
        // hedge region = the alternate region the hedge fanned out to.
        assert!(root.attributes.iter().any(|kv| {
            kv.key.as_str() == attributes::HEDGE_REGION && kv.value.as_str() == "westus2"
        }));
        // terminal state = alternate_won.
        assert!(root.attributes.iter().any(|kv| {
            kv.key.as_str() == attributes::HEDGE_TERMINAL_STATE
                && kv.value.as_str() == "alternate_won"
        }));
        // requested_regions carries both dispatched regions — the winning hedge
        // leg (westus2) plus the structurally-dropped primary (eastus) recovered
        // from the hedge diagnostics.
        let requested = string_array_attr(root, attributes::REQUESTED_REGIONS)
            .expect("requested_regions string[] present");
        assert!(requested.iter().any(|r| r == "eastus"));
        assert!(requested.iter().any(|r| r == "westus2"));
        // responded_regions carries only the winning hedge region: the dropped
        // primary leg never produced a service reply.
        let responded = string_array_attr(root, attributes::RESPONDED_REGIONS)
            .expect("responded_regions string[] present");
        assert!(responded.iter().any(|r| r == "westus2"));
        assert!(
            !responded.iter().any(|r| r == "eastus"),
            "the structurally-dropped primary leg must not appear in responded_regions"
        );
        // Nothing was truncated, so the exact-count attributes stay off the span
        // entirely — they exist only to make an elision explicit, and emitting
        // them unconditionally would put a redundant integer on every hedged
        // span.
        for key in [
            attributes::REQUESTED_REGIONS_TOTAL,
            attributes::RESPONDED_REGIONS_TOTAL,
        ] {
            assert!(
                !root.attributes.iter().any(|kv| kv.key.as_str() == key),
                "{key} must be absent when the region history was not truncated"
            );
        }

        // The speculative hedge leg's child span is tagged.
        let tagged = spans
            .iter()
            .filter(|s| s.name == "cosmosdb.request")
            .any(|s| {
                s.attributes.iter().any(|kv| {
                    kv.key.as_str() == attributes::HEDGE_LEG
                        && matches!(kv.value, Value::Bool(true))
                })
            });
        assert!(tagged, "the hedge leg child span must carry the hedge tag");
    }

    /// Builds a hedged operation in its **production `PrimaryWonAfterHedge`
    /// shape**: the primary East US leg won after the threshold, so the
    /// speculative West US 2 hedge leg was structurally cancelled and has no
    /// retained record. Only the primary (Initial, 200) survives; the hedge
    /// region is recovered from the hedge diagnostics.
    fn primary_won_after_hedge_context(anchor: Instant) -> DiagnosticsContext {
        let primary = RequestDiagnostics::for_testing(
            "https://acct-eastus.documents.azure.com:443/",
            Some(Region::EAST_US),
            CosmosStatus::new(StatusCode::Ok),
            RequestCharge::new(2.0),
            anchor - Duration::from_millis(300),
            anchor - Duration::from_millis(180),
        );
        let hedge = HedgeDiagnostics::for_testing(
            Region::EAST_US,
            Some(Region::WEST_US_2),
            Some(Region::EAST_US),
            HedgeTerminalState::PrimaryWonAfterHedge,
        );
        DiagnosticsContext::for_testing_with_hedge(
            ActivityId::new_uuid(),
            Duration::from_millis(200),
            Some(CosmosStatus::new(StatusCode::Ok)),
            Some("read_item"),
            vec![primary],
            Some(hedge),
        )
    }

    #[test]
    fn primary_won_after_hedge_recovers_hedge_region_without_child_span() {
        // When the primary wins cleanly the hedge leg is dropped, so there is no
        // child span to tag. The authoritative hedge signal must still be on the
        // root span, and the dropped hedge region recovered into requested_regions.
        let (provider, exporter) = exportable();
        let tracer = provider.tracer("test");
        let now_instant = Instant::now();
        let now_system = SystemTime::now();

        let ctx = primary_won_after_hedge_context(now_instant);
        emit_backdated_span_tree(&tracer, &ctx, None, None, now_instant, now_system);
        provider.force_flush().unwrap();

        let spans = exporter.get_finished_spans().unwrap();
        let root = spans
            .iter()
            .find(|s| s.name == "read_item")
            .expect("root span present");

        assert!(root.attributes.iter().any(|kv| {
            kv.key.as_str() == attributes::HEDGING_STARTED && matches!(kv.value, Value::Bool(true))
        }));
        assert!(root.attributes.iter().any(|kv| {
            kv.key.as_str() == attributes::HEDGE_REGION && kv.value.as_str() == "westus2"
        }));
        assert!(root.attributes.iter().any(|kv| {
            kv.key.as_str() == attributes::HEDGE_TERMINAL_STATE
                && kv.value.as_str() == "primary_won_after_hedge"
        }));
        // The dropped hedge leg's region is recovered into requested_regions...
        let requested = string_array_attr(root, attributes::REQUESTED_REGIONS)
            .expect("requested_regions string[] present");
        assert!(requested.iter().any(|r| r == "eastus"));
        assert!(requested.iter().any(|r| r == "westus2"));
        // ...but only the winning primary produced a response.
        let responded = string_array_attr(root, attributes::RESPONDED_REGIONS)
            .expect("responded_regions string[] present");
        assert_eq!(responded, vec!["eastus".to_string()]);
        // The hedge leg was structurally dropped, so no child span carries the
        // hedge tag; the root span's hedge attributes carry the signal instead.
        let any_hedge_child = spans
            .iter()
            .filter(|s| s.name == "cosmosdb.request")
            .any(|s| {
                s.attributes.iter().any(|kv| {
                    kv.key.as_str() == attributes::HEDGE_LEG
                        && matches!(kv.value, Value::Bool(true))
                })
            });
        assert!(
            !any_hedge_child,
            "a structurally-dropped hedge leg must not produce a tagged child span"
        );
    }

    #[test]
    fn non_hedged_operation_omits_hedging_span_attributes() {
        let (provider, exporter) = exportable();
        let tracer = provider.tracer("test");
        let now_instant = Instant::now();
        let now_system = SystemTime::now();

        // A plain failed operation — no hedge diagnostics, all Initial legs.
        let ctx = context(
            Duration::from_millis(5),
            Some(CosmosStatus::new(StatusCode::TooManyRequests)),
            Some("read_item"),
            &[(5, 5, CosmosStatus::new(StatusCode::TooManyRequests))],
            now_instant,
        );
        emit_backdated_span_tree(&tracer, &ctx, None, None, now_instant, now_system);
        provider.force_flush().unwrap();

        let spans = exporter.get_finished_spans().unwrap();
        for span in &spans {
            for kv in span.attributes.iter() {
                let key = kv.key.as_str();
                assert_ne!(key, attributes::HEDGING_STARTED);
                assert_ne!(key, attributes::HEDGE_REGION);
                assert_ne!(key, attributes::HEDGE_TERMINAL_STATE);
                assert_ne!(key, attributes::REQUESTED_REGIONS);
                assert_ne!(key, attributes::RESPONDED_REGIONS);
                assert_ne!(key, attributes::HEDGE_LEG);
            }
        }
    }
}
