// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::Url,
    tracing::{Attribute, Span, SpanKind, SpanStatus, Tracer, TracerProvider},
    Uuid,
};
use std::{
    fmt,
    future::Future,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Instant,
};

/// Shared by a client and all its recovered transports. No thread-local span guard
/// is held across an await: parentage is passed explicitly to each operation.
pub(crate) struct Diagnostics {
    tracer: Option<Arc<dyn Tracer>>,
    hub: String,
    host: String,
    client_id: String,
    next_component: AtomicU64,
    #[cfg(feature = "metrics")]
    metrics: Metrics,
}

impl Diagnostics {
    pub fn new(endpoint: &Url, provider: Option<Arc<dyn TracerProvider>>) -> Self {
        Self {
            tracer: provider.map(|p| {
                p.get_tracer(
                    Some("Microsoft.EventHub"),
                    env!("CARGO_PKG_NAME"),
                    Some(env!("CARGO_PKG_VERSION")),
                )
            }),
            hub: endpoint
                .path_segments()
                .and_then(|mut p| p.next())
                .unwrap_or_default()
                .into(),
            host: endpoint.host_str().unwrap_or_default().into(),
            client_id: Uuid::new_v4().to_string(),
            next_component: AtomicU64::new(0),
            #[cfg(feature = "metrics")]
            metrics: Metrics::new(),
        }
    }

    pub fn start(
        &self,
        name: &'static str,
        operation: &'static str,
        kind: SpanKind,
        parent: Option<Arc<dyn Span>>,
    ) -> Operation<'_> {
        let span = self.tracer.as_ref().map(|tracer| {
            let mut attributes = vec![
                Attribute {
                    key: "az.namespace".into(),
                    value: "Microsoft.EventHub".into(),
                },
                Attribute {
                    key: "messaging.system".into(),
                    value: "eventhubs".into(),
                },
                Attribute {
                    key: "server.address".into(),
                    value: self.host.clone().into(),
                },
            ];
            if matches!(operation, "send" | "receive") {
                let entity = if operation == "receive" {
                    "messaging.source.name"
                } else {
                    "messaging.destination.name"
                };
                attributes.push(Attribute {
                    key: entity.into(),
                    value: self.hub.clone().into(),
                });
                attributes.push(Attribute {
                    key: "messaging.operation".into(),
                    value: operation.into(),
                });
            } else {
                // Connections and sessions serve both directions. Opening either
                // is a transport operation, not a message send or receive.
                attributes.push(Attribute {
                    key: "az.eventhubs.amqp.operation".into(),
                    value: operation.into(),
                });
            }
            match parent {
                Some(parent) => {
                    tracer.start_span_with_parent(name.into(), kind, attributes, parent)
                }
                None => tracer.start_span(name.into(), kind, attributes),
            }
        });
        Operation {
            diagnostics: self,
            span,
            operation,
            started: Instant::now(),
            outcome: "cancelled",
            count: 0,
            message_count: None,
            lifecycle: None,
            logical: false,
        }
    }

    pub fn logical(
        &self,
        name: &'static str,
        operation: &'static str,
        kind: SpanKind,
    ) -> Operation<'_> {
        let mut guard = self.start(name, operation, kind, None);
        guard.logical = true;
        guard
    }

    pub async fn run<T, E: std::error::Error, F: Future<Output = Result<T, E>>>(
        &self,
        name: &'static str,
        operation: &'static str,
        kind: SpanKind,
        count: u64,
        action: impl FnOnce(Option<Arc<dyn Span>>) -> F,
    ) -> Result<T, E> {
        let mut guard = self.logical(name, operation, kind);
        let result = action(guard.span.clone()).await;
        guard.finish(&result, count);
        result
    }

    pub fn lifecycle(
        &self,
        component: &'static str,
        path: &Url,
        generation: u64,
        parent: Option<Arc<dyn Span>>,
    ) -> Operation<'_> {
        let sequence = self.next_component.fetch_add(1, Ordering::Relaxed) + 1;
        let component_id = format!("{}/{component}/{sequence}", self.client_id);
        let name = match component {
            "sender" => "EventHubs.sender.attach",
            "receiver" => "EventHubs.receiver.attach",
            "session" => "EventHubs.session.begin",
            _ => "EventHubs.connection.open",
        };
        let operation = match component {
            "receiver" => "receive",
            "sender" => "send",
            "session" => "begin",
            _ => "open",
        };
        let mut guard = self.start(name, operation, SpanKind::Client, parent);
        if let Some(span) = &guard.span {
            span.set_attribute("az.eventhubs.client.id", self.client_id.clone().into());
            span.set_attribute("az.eventhubs.amqp.component.type", component.into());
            span.set_attribute(
                "az.eventhubs.amqp.component.id",
                component_id.clone().into(),
            );
            span.set_attribute(
                "az.eventhubs.amqp.component.generation",
                (sequence as i64).into(),
            );
            span.set_attribute(
                "az.eventhubs.amqp.recovery.generation",
                (generation as i64).into(),
            );
        }
        guard.lifecycle = Some(Lifecycle {
            component,
            component_id,
            path: path.path().into(),
            generation: sequence,
            recovery_generation: generation,
        });
        guard.log_lifecycle("started");
        guard
    }

    pub fn recovery(&self, generation: u64, component: &'static str) {
        tracing::info!(event = "invalidated", client_id = %self.client_id, component, generation, "Event Hubs AMQP lifecycle");
        #[cfg(feature = "metrics")]
        self.metrics.lifecycle.add(
            1,
            &[
                opentelemetry::KeyValue::new("component", component),
                opentelemetry::KeyValue::new("outcome", "invalidated"),
            ],
        );
    }
}

struct Lifecycle {
    component: &'static str,
    component_id: String,
    path: String,
    generation: u64,
    recovery_generation: u64,
}

// Never retain the original error or expose its source chain to a tracer.
// Broker descriptions, info maps, and arbitrary custom error text may contain
// application data. The caller still receives the original error unchanged.
#[derive(Debug)]
struct OperationFailed;

impl fmt::Display for OperationFailed {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Event Hubs operation failed")
    }
}

impl std::error::Error for OperationFailed {}

pub(crate) struct Operation<'a> {
    diagnostics: &'a Diagnostics,
    pub span: Option<Arc<dyn Span>>,
    operation: &'static str,
    started: Instant,
    outcome: &'static str,
    count: u64,
    message_count: Option<u64>,
    lifecycle: Option<Lifecycle>,
    logical: bool,
}

impl Operation<'_> {
    pub fn set_message_count(&mut self, count: u64) {
        self.message_count = Some(count);
        if let Some(span) = &self.span {
            span.set_attribute("messaging.batch.message_count", (count as i64).into());
        }
    }

    pub fn finish<T, E: std::error::Error>(&mut self, result: &Result<T, E>, count: u64) {
        self.outcome = if result.is_ok() { "success" } else { "error" };
        self.count = if result.is_ok() { count } else { 0 };
        if let Some(span) = &self.span {
            // Producer counts were set when the operation started. Only a
            // logical receive learns its message count at completion; control
            // and transport spans do not claim an unknown message count.
            if self.logical && self.message_count.is_none() {
                span.set_attribute("messaging.batch.message_count", (self.count as i64).into());
            }
            if result.is_err() {
                span.record_error(&OperationFailed);
                // Avoid adding service messages or payloads to the status description.
                span.set_status(SpanStatus::Error {
                    description: "Event Hubs operation failed".into(),
                });
            }
        }
    }

    fn log_lifecycle(&self, event: &'static str) {
        if let Some(lifecycle) = &self.lifecycle {
            tracing::info!(event, client_id = %self.diagnostics.client_id, component = lifecycle.component,
                component_id = %lifecycle.component_id, generation = lifecycle.generation,
                recovery_generation = lifecycle.recovery_generation, path = %lifecycle.path, "Event Hubs AMQP lifecycle");
        }
    }
}

impl Drop for Operation<'_> {
    fn drop(&mut self) {
        if let Some(span) = &self.span {
            if self.outcome == "cancelled" {
                span.set_status(SpanStatus::Error {
                    description: "Event Hubs operation cancelled".into(),
                });
            }
            span.end();
        }
        self.log_lifecycle(self.outcome);
        #[cfg(feature = "metrics")]
        {
            use opentelemetry::KeyValue;
            if let Some(lifecycle) = &self.lifecycle {
                self.diagnostics.metrics.lifecycle.add(
                    1,
                    &[
                        KeyValue::new("component", lifecycle.component),
                        KeyValue::new("outcome", self.outcome),
                    ],
                );
            } else if self.logical {
                let attributes = [
                    KeyValue::new("messaging.system", "eventhubs"),
                    KeyValue::new("messaging.destination.name", self.diagnostics.hub.clone()),
                    KeyValue::new("server.address", self.diagnostics.host.clone()),
                    KeyValue::new("operation", self.operation),
                    KeyValue::new("outcome", self.outcome),
                ];
                self.diagnostics.metrics.operations.add(1, &attributes);
                self.diagnostics
                    .metrics
                    .duration
                    .record(self.started.elapsed().as_secs_f64(), &attributes);
                self.diagnostics.metrics.events.add(self.count, &attributes);
            }
        }
        #[cfg(not(feature = "metrics"))]
        let _ = (&self.operation, &self.started);
    }
}

#[cfg(feature = "metrics")]
struct Metrics {
    operations: opentelemetry::metrics::Counter<u64>,
    events: opentelemetry::metrics::Counter<u64>,
    duration: opentelemetry::metrics::Histogram<f64>,
    lifecycle: opentelemetry::metrics::Counter<u64>,
}

#[cfg(feature = "metrics")]
impl Metrics {
    fn new() -> Self {
        let scope = opentelemetry::InstrumentationScope::builder(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
            .build();
        let meter = opentelemetry::global::meter_with_scope(scope);
        Self::with_meter(meter)
    }

    fn with_meter(meter: opentelemetry::metrics::Meter) -> Self {
        Self {
            operations: meter.u64_counter("az.eventhubs.operations").with_description("Completed logical send and receive operations, including failures and cancellation").with_unit("{operation}").build(),
            events: meter.u64_counter("az.eventhubs.events").with_description("Successfully sent or received events").with_unit("{event}").build(),
            duration: meter.f64_histogram("az.eventhubs.operation.duration").with_description("Logical operation duration including retries").with_unit("s").build(),
            lifecycle: meter.u64_counter("az.eventhubs.amqp.lifecycle").with_description("AMQP creation attempts and recovery invalidations").with_unit("{transition}").build(),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    #[cfg(feature = "metrics")]
    use super::Metrics;
    use super::{Diagnostics, SpanKind, TracerProvider};
    use azure_core::http::Url;
    use azure_core_amqp::{
        error::{AmqpDescribedError, AmqpErrorCondition, AmqpErrorKind},
        AmqpError,
    };
    use azure_core_opentelemetry::OpenTelemetryTracerProvider;
    use futures::{future, FutureExt};
    use opentelemetry::trace::Status;
    use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider};
    use std::sync::Arc;

    pub(crate) fn provider() -> (
        Arc<dyn TracerProvider>,
        InMemorySpanExporter,
        SdkTracerProvider,
    ) {
        let exporter = InMemorySpanExporter::default();
        let sdk = SdkTracerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();
        (
            OpenTelemetryTracerProvider::new(Arc::new(sdk.clone())),
            exporter,
            sdk,
        )
    }

    fn diagnostics(provider: Option<Arc<dyn TracerProvider>>) -> Diagnostics {
        Diagnostics::new(
            &Url::parse("amqps://example.servicebus.windows.net/hub").unwrap(),
            provider,
        )
    }

    #[test]
    fn disabled_tracing_has_no_span() {
        let diagnostics = diagnostics(None);
        assert!(diagnostics
            .logical("send", "send", SpanKind::Producer)
            .span
            .is_none());
    }

    #[tokio::test]
    async fn explicit_children_and_retries_share_one_logical_span() {
        let (provider, exporter, _sdk) = provider();
        let diagnostics = diagnostics(Some(provider));
        let result: Result<(), std::io::Error> = diagnostics
            .run(
                "ProducerClient.send",
                "send",
                SpanKind::Producer,
                3,
                |parent| {
                    let diagnostics = &diagnostics;
                    async move {
                        for attempt in 0..2 {
                            let mut child = diagnostics.start(
                                "EventHubs.send.disposition",
                                "send",
                                SpanKind::Client,
                                parent.clone(),
                            );
                            let result = if attempt == 0 {
                                Err(std::io::Error::other("retry"))
                            } else {
                                Ok(())
                            };
                            child.finish(&result, 0);
                        }
                        Ok(())
                    }
                },
            )
            .await;
        result.unwrap();
        let spans = exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 3);
        let parent = &spans[2];
        assert_eq!(parent.name, "ProducerClient.send");
        assert_eq!(parent.status, Status::Unset);
        assert_eq!(parent.instrumentation_scope.name(), env!("CARGO_PKG_NAME"));
        assert!(parent
            .attributes
            .iter()
            .any(|a| a.key.as_str() == "messaging.batch.message_count" && a.value == 3_i64.into()));
        for child in &spans[..2] {
            assert_eq!(child.parent_span_id, parent.span_context.span_id());
            assert_eq!(
                child.span_context.trace_id(),
                parent.span_context.trace_id()
            );
        }
        assert!(matches!(spans[0].status, Status::Error { .. }));
    }

    #[tokio::test]
    async fn failure_and_dropped_future_end_spans() {
        let (provider, exporter, _sdk) = provider();
        let diagnostics = diagnostics(Some(provider));
        let result: Result<(), std::io::Error> = diagnostics
            .run("send", "send", SpanKind::Producer, 9, |_| async {
                Err(std::io::Error::other("terminal"))
            })
            .await;
        assert_eq!(result.unwrap_err().to_string(), "terminal");
        let pending = diagnostics.run("receive", "receive", SpanKind::Client, 1, |_| {
            future::pending::<Result<(), std::io::Error>>()
        });
        assert!(pending.now_or_never().is_none());
        let spans = exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 2);
        assert!(spans
            .iter()
            .all(|s| matches!(s.status, Status::Error { .. })));
        assert!(!spans[0].events.events.is_empty());
        assert!(spans[1]
            .attributes
            .iter()
            .any(|a| a.key.as_str() == "messaging.source.name"));
    }

    #[test]
    fn lifecycle_attempts_have_distinct_ids_and_generations() {
        let (provider, exporter, _sdk) = provider();
        let diagnostics = diagnostics(Some(provider));
        let url = Url::parse("amqps://example.servicebus.windows.net/hub/Partitions/0").unwrap();
        for _ in 0..2 {
            let mut attempt = diagnostics.lifecycle("receiver", &url, 4, None);
            attempt.finish(&Err::<(), _>(std::io::Error::other("attach failed")), 0);
        }
        let spans = exporter.get_finished_spans().unwrap();
        for name in [
            "az.eventhubs.amqp.component.id",
            "az.eventhubs.amqp.component.generation",
        ] {
            let first = spans[0]
                .attributes
                .iter()
                .find(|a| a.key.as_str() == name)
                .unwrap();
            let second = spans[1]
                .attributes
                .iter()
                .find(|a| a.key.as_str() == name)
                .unwrap();
            assert_ne!(first.value, second.value);
        }
    }

    #[cfg(feature = "metrics")]
    #[tokio::test]
    async fn metrics_count_logical_operations_without_attempt_or_identity_labels() {
        use opentelemetry::metrics::MeterProvider;
        use opentelemetry_sdk::metrics::{
            data::{AggregatedMetrics, MetricData},
            InMemoryMetricExporter, SdkMeterProvider,
        };
        let exporter = InMemoryMetricExporter::default();
        let provider = SdkMeterProvider::builder()
            .with_periodic_exporter(exporter.clone())
            .build();
        let mut diagnostics = diagnostics(None);
        diagnostics.metrics = Metrics::with_meter(provider.meter("test"));
        let result: Result<(), std::io::Error> = diagnostics
            .run("send", "send", SpanKind::Producer, 4, |parent| {
                let diagnostics = &diagnostics;
                async move {
                    let mut child =
                        diagnostics.start("send.disposition", "send", SpanKind::Client, parent);
                    child.finish(&Ok::<(), std::io::Error>(()), 0);
                    Ok(())
                }
            })
            .await;
        result.unwrap();
        let _: Result<(), std::io::Error> = diagnostics
            .run("send", "send", SpanKind::Producer, 7, |_| async {
                Err(std::io::Error::other("failure"))
            })
            .await;
        assert!(diagnostics
            .run("receive", "receive", SpanKind::Client, 1, |_| {
                future::pending::<Result<(), std::io::Error>>()
            })
            .now_or_never()
            .is_none());
        provider.force_flush().unwrap();
        let data = exporter.get_finished_metrics().unwrap();
        let metrics: Vec<_> = data
            .iter()
            .flat_map(|r| r.scope_metrics())
            .flat_map(|s| s.metrics())
            .collect();
        for (name, expected) in [("az.eventhubs.operations", 3), ("az.eventhubs.events", 4)] {
            let metric = metrics.iter().find(|m| m.name() == name).unwrap();
            let AggregatedMetrics::U64(MetricData::Sum(sum)) = metric.data() else {
                panic!("expected counter")
            };
            assert_eq!(sum.data_points().map(|p| p.value()).sum::<u64>(), expected);
            for point in sum.data_points() {
                assert!(point
                    .attributes()
                    .all(|a| !a.key.as_str().contains("id") && !a.key.as_str().contains("error")));
            }
        }
        let metric = metrics
            .iter()
            .find(|m| m.name() == "az.eventhubs.operation.duration")
            .unwrap();
        assert_eq!(metric.unit(), "s");
        let AggregatedMetrics::F64(MetricData::Histogram(histogram)) = metric.data() else {
            panic!("expected histogram")
        };
        assert_eq!(histogram.data_points().map(|p| p.count()).sum::<u64>(), 3);
        provider.shutdown().unwrap();
    }
    #[test]
    fn control_spans_do_not_claim_message_sends() {
        let (provider, exporter, _sdk) = provider();
        let diagnostics = diagnostics(Some(provider));
        let url = Url::parse("amqps://example.servicebus.windows.net/hub/ConsumerGroups/$Default")
            .unwrap();
        for component in ["connection", "session"] {
            let mut span = diagnostics.lifecycle(component, &url, 0, None);
            span.finish(&Ok::<(), std::io::Error>(()), 0);
        }
        let spans = exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 2);
        for (span, operation) in spans.iter().zip(["open", "begin"]) {
            assert!(!span.attributes.iter().any(|a| matches!(
                a.key.as_str(),
                "messaging.operation"
                    | "messaging.source.name"
                    | "messaging.destination.name"
                    | "messaging.batch.message_count"
            )));
            assert!(span
                .attributes
                .iter()
                .any(|a| a.key.as_str() == "az.eventhubs.amqp.operation"
                    && a.value == operation.into()));
        }
    }
    #[tokio::test]
    async fn broker_error_details_are_not_exported() {
        let (provider, exporter, _sdk) = provider();
        let diagnostics = diagnostics(Some(provider));
        let error = AmqpError::from(AmqpErrorKind::AmqpDescribedError(AmqpDescribedError::new(
            AmqpErrorCondition::InternalError,
            Some("private-description-marker".into()),
            vec![("private-info-key".into(), "private-info-marker".into())].into(),
        )));
        let expected = error.to_string();
        assert!(expected.contains("private-description-marker"));
        assert!(expected.contains("private-info-marker"));
        let result: Result<(), AmqpError> = diagnostics
            .run("send", "send", SpanKind::Producer, 1, |_| async {
                Err(error)
            })
            .await;
        assert_eq!(
            result.unwrap_err().to_string(),
            expected,
            "caller retains the original error"
        );
        let spans = exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 1);
        assert!(matches!(spans[0].status, Status::Error { .. }));
        assert!(
            !spans[0].events.events.is_empty(),
            "failure still emits an exception event"
        );
        let exported = format!("{:?}", spans[0]);
        for marker in [
            "private-description-marker",
            "private-info-key",
            "private-info-marker",
        ] {
            assert!(
                !exported.contains(marker),
                "exported private error detail: {marker}"
            );
        }
    }
}
