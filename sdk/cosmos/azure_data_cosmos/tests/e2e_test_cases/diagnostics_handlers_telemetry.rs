// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use azure_core::http::Context;
use azure_data_cosmos::{
    diagnostics::{
        CosmosMetricsHandler, CosmosOperationContext, CosmosTracingHandler, DiagnosticsContext,
        DiagnosticsHandler, SamplingLogHandler,
    },
    options::Region,
    RoutingStrategy,
};
use opentelemetry::{
    metrics::MeterProvider as _,
    trace::{SpanId, TracerProvider as _},
};
use opentelemetry_sdk::{
    metrics::{
        data::{AggregatedMetrics, MetricData, ResourceMetrics},
        InMemoryMetricExporter, PeriodicReader, SdkMeterProvider,
    },
    trace::{in_memory_exporter::InMemorySpanExporter, SdkTracerProvider},
};

use crate::e2e_test_cases::{
    fixture::{build_client_with_customizer, ClientSetup, E2eTest, TestResult},
    support::{item, selected_scenario_profile},
};

#[derive(Default)]
struct RecordingHandler {
    operations: Mutex<Vec<Option<String>>>,
    failures: Mutex<usize>,
}

impl RecordingHandler {
    fn operation_count(&self, operation: &str) -> usize {
        self.operations
            .lock()
            .unwrap()
            .iter()
            .filter(|value| value.as_deref() == Some(operation))
            .count()
    }

    fn failures(&self) -> usize {
        *self.failures.lock().unwrap()
    }
}

impl DiagnosticsHandler for RecordingHandler {
    fn handle(&self, diagnostics: &DiagnosticsContext, context: &Context<'_>) {
        if diagnostics.is_failure() {
            *self.failures.lock().unwrap() += 1;
        }
        self.operations.lock().unwrap().push(
            context
                .value::<CosmosOperationContext>()
                .and_then(|operation| operation.operation_name())
                .map(str::to_owned),
        );
    }
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn handlers_emit_metrics_spans_and_sampled_failures() -> TestResult {
    let Some(profile) = selected_scenario_profile("diagnostics.handlers-telemetry").await? else {
        return Ok(());
    };
    let metric_exporter = InMemoryMetricExporter::default();
    let metric_reader = PeriodicReader::builder(metric_exporter.clone()).build();
    let meter_provider = SdkMeterProvider::builder()
        .with_reader(metric_reader)
        .build();
    let metrics_handler = Arc::new(CosmosMetricsHandler::with_meter(
        meter_provider.meter("cosmos-e2e"),
    ));

    let span_exporter = InMemorySpanExporter::default();
    let tracer_provider = SdkTracerProvider::builder()
        .with_simple_exporter(span_exporter.clone())
        .build();

    let all_operations = Arc::new(RecordingHandler::default());
    let sampled_operations = Arc::new(RecordingHandler::default());
    let sampled_handler = Arc::new(SamplingLogHandler::with_handler(sampled_operations.clone()));
    let tracing_handler = Arc::new(
        CosmosTracingHandler::builder().build_with_tracer(tracer_provider.tracer("cosmos-e2e")),
    );

    let setup = ClientSetup::from_profile(
        profile.selected_runtime()?,
        profile.selected_client()?,
        RoutingStrategy::PreferredRegions(vec![Region::EAST_US, Region::WEST_US]),
    )?;
    let client = build_client_with_customizer(setup, |builder| {
        Ok(builder
            .with_diagnostics_handler(all_operations.clone())
            .with_diagnostics_handler(metrics_handler)
            .with_diagnostics_handler(sampled_handler)
            .with_diagnostics_handler(tracing_handler))
    })
    .await?;

    E2eTest::builder()
        .with_client(client)
        .run(async |fixture| {
            let before_create = all_operations.operation_count("create_item");
            let before_read = all_operations.operation_count("read_item");
            let before_sampled_failures = sampled_operations.failures();

            let value = item("telemetry", "A", 31);
            fixture
                .container
                .create_item("A", &value.id, &value, None)
                .await?;
            fixture
                .container
                .read_item("A", "missing-telemetry", None)
                .await
                .expect_err("missing read must fail");

            assert_eq!(
                all_operations.operation_count("create_item"),
                before_create + 1
            );
            assert_eq!(all_operations.operation_count("read_item"), before_read + 1);
            assert_eq!(sampled_operations.failures(), before_sampled_failures + 1);
            Ok(())
        })
        .await?;

    meter_provider.force_flush()?;
    let metrics = metric_exporter.get_finished_metrics()?;
    let read_attributes = duration_attributes(&metrics, "read_item")
        .ok_or("read_item duration metric must be emitted")?;
    assert_eq!(
        read_attributes.get("db.system.name").map(String::as_str),
        Some("azure.cosmosdb")
    );
    assert_eq!(
        read_attributes.get("db.operation.name").map(String::as_str),
        Some("read_item")
    );
    assert_eq!(
        read_attributes
            .get("db.response.status_code")
            .map(String::as_str),
        Some("404")
    );
    assert!(read_attributes.contains_key("server.address"));

    tracer_provider.force_flush()?;
    let spans = span_exporter.get_finished_spans()?;
    let root = spans
        .iter()
        .find(|span| span.name == "read_item")
        .ok_or("failed read_item root span must be exported")?;
    assert_eq!(root.parent_span_id, SpanId::INVALID);
    assert!(root.attributes.iter().any(|attribute| {
        attribute.key.as_str() == "db.operation.name" && attribute.value.as_str() == "read_item"
    }));
    assert!(root.attributes.iter().any(|attribute| {
        attribute.key.as_str() == "error.type" && attribute.value.as_str() == "404"
    }));
    let root_id = root.span_context.span_id();
    assert!(spans
        .iter()
        .any(|span| { span.name == "cosmosdb.request" && span.parent_span_id == root_id }));

    tracer_provider.shutdown()?;
    meter_provider.shutdown()?;
    Ok(())
}

fn duration_attributes(
    metrics: &[ResourceMetrics],
    operation_name: &str,
) -> Option<BTreeMap<String, String>> {
    for resource in metrics {
        for scope in resource.scope_metrics() {
            for metric in scope.metrics() {
                if metric.name() != "db.client.operation.duration" {
                    continue;
                }
                if let AggregatedMetrics::F64(MetricData::Histogram(histogram)) = metric.data() {
                    for point in histogram.data_points() {
                        let attributes: BTreeMap<_, _> = point
                            .attributes()
                            .map(|attribute| {
                                (
                                    attribute.key.as_str().to_owned(),
                                    attribute.value.as_str().into_owned(),
                                )
                            })
                            .collect();
                        if attributes.get("db.operation.name").map(String::as_str)
                            == Some(operation_name)
                        {
                            return Some(attributes);
                        }
                    }
                }
            }
        }
    }
    None
}
