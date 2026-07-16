// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`CosmosMetricsHandler`] — emits OpenTelemetry metrics from a completed
//! [`DiagnosticsContext`].

use std::collections::HashSet;
use std::sync::Mutex;

use azure_core::http::Context;
use opentelemetry::metrics::Meter;
use opentelemetry::{global, KeyValue};

use crate::diagnostics::metrics::attributes;
use crate::diagnostics::metrics::instruments::Instruments;
use crate::diagnostics::metrics::MetricsOptions;
use crate::diagnostics::{CosmosOperationContext, DiagnosticsContext, DiagnosticsHandler};

/// Instrumentation scope name used for the Cosmos [`Meter`].
const METER_NAME: &str = "azure_data_cosmos";

/// A [`DiagnosticsHandler`] that records OpenTelemetry metrics for every
/// completed Cosmos DB operation.
///
/// Register it via
/// [`CosmosClientBuilder::with_diagnostics_handler`](crate::CosmosClientBuilder::with_diagnostics_handler):
///
/// ```
/// use std::sync::Arc;
/// use azure_data_cosmos::diagnostics::CosmosMetricsHandler;
///
/// // Uses the globally-registered OpenTelemetry meter provider.
/// let handler = Arc::new(CosmosMetricsHandler::new());
/// # let _handler: Arc<CosmosMetricsHandler> = handler;
/// ```
///
/// The handler always records the stable `db.client.operation.duration`
/// histogram. Development-tier metrics and attributes are opt-in via
/// [`MetricsOptions`] (see [`with_options`](CosmosMetricsHandler::with_options)).
///
/// When no meter provider is registered, the global meter is a no-op, so
/// recording is effectively free — you can register the handler unconditionally
/// and pay nothing until an exporter is wired up.
pub struct CosmosMetricsHandler {
    instruments: Instruments,
    options: MetricsOptions,
    /// Client-instance ids (machine ids) already counted toward
    /// `active_instance.count`, so each distinct instance is counted once.
    seen_instances: Mutex<HashSet<String>>,
}

impl CosmosMetricsHandler {
    /// Creates a handler backed by the globally-registered meter provider, with
    /// default [`MetricsOptions`] (stable metric only).
    pub fn new() -> Self {
        Self::with_options(MetricsOptions::default())
    }

    /// Creates a handler backed by the global meter provider with the given
    /// [`MetricsOptions`].
    pub fn with_options(options: MetricsOptions) -> Self {
        Self::from_meter(&global::meter(METER_NAME), options)
    }

    /// Creates a handler that records into a specific [`Meter`], with default
    /// [`MetricsOptions`].
    ///
    /// Useful for tests (against an in-memory meter) or to bind metrics to a
    /// meter provider other than the global one.
    pub fn with_meter(meter: Meter) -> Self {
        Self::from_meter(&meter, MetricsOptions::default())
    }

    /// Creates a handler that records into a specific [`Meter`] with the given
    /// [`MetricsOptions`].
    pub fn with_meter_and_options(meter: Meter, options: MetricsOptions) -> Self {
        Self::from_meter(&meter, options)
    }

    fn from_meter(meter: &Meter, options: MetricsOptions) -> Self {
        Self {
            instruments: Instruments::new(meter),
            options,
            seen_instances: Mutex::new(HashSet::new()),
        }
    }

    /// Resolves `server.address`: the operation-context override if present,
    /// otherwise the host of the last contacted endpoint.
    fn server_address(
        &self,
        diagnostics: &DiagnosticsContext,
        op: Option<&CosmosOperationContext>,
    ) -> Option<String> {
        if let Some(address) = op.and_then(CosmosOperationContext::server_address) {
            return Some(address.to_string());
        }
        let requests = diagnostics.requests();
        requests
            .last()
            .and_then(|request| host_of(request.endpoint()))
    }

    /// Builds the operation-scope attribute set shared by the duration and
    /// per-operation development histograms.
    fn build_attributes(
        &self,
        diagnostics: &DiagnosticsContext,
        op: Option<&CosmosOperationContext>,
        server_address: Option<&str>,
    ) -> Vec<KeyValue> {
        let mut attrs = Vec::with_capacity(8);

        // db.system.name is constant for every Cosmos metric.
        attrs.push(KeyValue::new(
            attributes::ATTR_DB_SYSTEM_NAME,
            attributes::DB_SYSTEM_NAME_VALUE,
        ));

        // Operation-scope identity supplied by the SDK.
        if let Some(op) = op {
            if let Some(name) = op.operation_name() {
                attrs.push(KeyValue::new(
                    attributes::ATTR_DB_OPERATION_NAME,
                    name.to_string(),
                ));
            }
            if let Some(container) = op.container_name() {
                attrs.push(KeyValue::new(
                    attributes::ATTR_DB_COLLECTION_NAME,
                    container.to_string(),
                ));
            }
            if let Some(database) = op.database_name() {
                attrs.push(KeyValue::new(
                    attributes::ATTR_DB_NAMESPACE,
                    database.to_string(),
                ));
            }
        }

        // Response status and, on failure, error.type (per semantic conventions).
        match diagnostics.status() {
            Some(status) => {
                let code = u16::from(status.status_code());
                attrs.push(KeyValue::new(
                    attributes::ATTR_DB_RESPONSE_STATUS_CODE,
                    code.to_string(),
                ));
                if !status.is_success() {
                    attrs.push(KeyValue::new(attributes::ATTR_ERROR_TYPE, code.to_string()));
                }
            }
            None => {
                // No status recorded — a client/transport failure with no HTTP
                // response. Classify it as the semconv catch-all.
                attrs.push(KeyValue::new(
                    attributes::ATTR_ERROR_TYPE,
                    attributes::ERROR_TYPE_OTHER,
                ));
            }
        }

        if let Some(address) = server_address {
            attrs.push(KeyValue::new(
                attributes::ATTR_SERVER_ADDRESS,
                address.to_string(),
            ));
        }

        // Development attributes are opt-in (higher cardinality; D7).
        if self.options.development_attributes_enabled() {
            if let Some(op) = op {
                if let Some(level) = op.consistency_level() {
                    attrs.push(KeyValue::new(
                        attributes::ATTR_CONSISTENCY_LEVEL,
                        level.to_string(),
                    ));
                }
                if let Some(mode) = op.connection_mode() {
                    attrs.push(KeyValue::new(
                        attributes::ATTR_CONNECTION_MODE,
                        mode.to_string(),
                    ));
                }
            }
            if let Some(sub_status) = diagnostics.status().and_then(|s| s.sub_status()) {
                attrs.push(KeyValue::new(
                    attributes::ATTR_SUB_STATUS_CODE,
                    i64::from(sub_status.value()),
                ));
            }
            let regions = diagnostics.regions_contacted();
            if !regions.is_empty() {
                let joined = regions
                    .iter()
                    .map(|region| region.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                attrs.push(KeyValue::new(attributes::ATTR_CONTACTED_REGIONS, joined));
            }
        }

        attrs
    }

    /// Counts a client instance toward `active_instance.count` the first time its
    /// machine id is seen. This is an approximation of "active instances":
    /// without client-lifecycle hooks (a future wiring concern) the counter does
    /// not decrement, so it reflects the number of distinct client instances
    /// observed over the process's lifetime.
    fn record_active_instance(&self, machine_id: &str, server_address: Option<&str>) {
        let mut seen = self
            .seen_instances
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if seen.insert(machine_id.to_string()) {
            let mut attrs = Vec::with_capacity(2);
            attrs.push(KeyValue::new(
                attributes::ATTR_DB_SYSTEM_NAME,
                attributes::DB_SYSTEM_NAME_VALUE,
            ));
            if let Some(address) = server_address {
                attrs.push(KeyValue::new(
                    attributes::ATTR_SERVER_ADDRESS,
                    address.to_string(),
                ));
            }
            self.instruments.active_instance_count.add(1, &attrs);
        }
    }
}

impl Default for CosmosMetricsHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for CosmosMetricsHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Instruments and the seen-set are not meaningfully printable; surface
        // only the configuration.
        f.debug_struct("CosmosMetricsHandler")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl DiagnosticsHandler for CosmosMetricsHandler {
    fn handle(&self, diagnostics: &DiagnosticsContext, cx: &Context<'_>) {
        let op = cx.value::<CosmosOperationContext>();
        let server_address = self.server_address(diagnostics, op);
        let attributes = self.build_attributes(diagnostics, op, server_address.as_deref());

        // Stable metric: always recorded.
        self.instruments
            .operation_duration
            .record(diagnostics.duration().as_secs_f64(), &attributes);

        if !self.options.development_metrics_enabled() {
            return;
        }

        // Development metrics (opt-in).
        self.instruments
            .request_charge
            .record(diagnostics.total_request_charge().value(), &attributes);

        if let Some(rows) = op.and_then(CosmosOperationContext::returned_item_count) {
            self.instruments.returned_rows.record(rows, &attributes);
        }

        if let Some(machine_id) = diagnostics.machine_id() {
            self.record_active_instance(machine_id, server_address.as_deref());
        }
    }
}

/// Extracts the host portion of an endpoint URI for `server.address`.
fn host_of(endpoint: &str) -> Option<String> {
    url::Url::parse(endpoint)
        .ok()
        .and_then(|url| url.host_str().map(str::to_owned))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CosmosStatus;
    use azure_core::http::StatusCode;
    use azure_data_cosmos_driver::models::ActivityId;
    use opentelemetry::metrics::MeterProvider as _;
    use opentelemetry_sdk::metrics::data::{AggregatedMetrics, MetricData, ResourceMetrics};
    use opentelemetry_sdk::metrics::{InMemoryMetricExporter, PeriodicReader, SdkMeterProvider};
    use std::collections::HashMap;
    use std::time::Duration;

    /// A test meter provider wired to an in-memory exporter, plus the meter to
    /// build a handler from.
    struct TestMeter {
        provider: SdkMeterProvider,
        exporter: InMemoryMetricExporter,
        meter: Meter,
    }

    fn test_meter() -> TestMeter {
        let exporter = InMemoryMetricExporter::default();
        let reader = PeriodicReader::builder(exporter.clone()).build();
        let provider = SdkMeterProvider::builder().with_reader(reader).build();
        let meter = provider.meter("test");
        TestMeter {
            provider,
            exporter,
            meter,
        }
    }

    impl TestMeter {
        /// Flushes and returns the collected resource metrics.
        fn collect(&self) -> Vec<ResourceMetrics> {
            self.provider.force_flush().unwrap();
            self.exporter.get_finished_metrics().unwrap()
        }
    }

    /// Completed context for a successful `read_item` returning HTTP 200.
    fn completed(status_code: u16) -> DiagnosticsContext {
        DiagnosticsContext::for_testing_completed(
            ActivityId::new_uuid(),
            Duration::from_millis(42),
            Some(CosmosStatus::new(StatusCode::from(status_code))),
        )
    }

    fn operation_context() -> CosmosOperationContext {
        CosmosOperationContext::new()
            .with_operation_name("read_item")
            .with_database_name("my_db")
            .with_container_name("my_container")
            .with_server_address("my-account.documents.azure.com")
    }

    fn metric_names(metrics: &[ResourceMetrics]) -> Vec<String> {
        let mut names = Vec::new();
        for rm in metrics {
            for sm in rm.scope_metrics() {
                for m in sm.metrics() {
                    names.push(m.name().to_string());
                }
            }
        }
        names
    }

    /// Returns the attributes (as a string map) and count of the first data
    /// point of the `db.client.operation.duration` histogram, if present.
    fn duration_point(metrics: &[ResourceMetrics]) -> Option<(HashMap<String, String>, u64)> {
        for rm in metrics {
            for sm in rm.scope_metrics() {
                for m in sm.metrics() {
                    if m.name() != attributes::METRIC_OPERATION_DURATION {
                        continue;
                    }
                    if let AggregatedMetrics::F64(MetricData::Histogram(histogram)) = m.data() {
                        if let Some(point) = histogram.data_points().next() {
                            let attrs = point
                                .attributes()
                                .map(|kv| {
                                    (kv.key.as_str().to_string(), kv.value.as_str().into_owned())
                                })
                                .collect();
                            return Some((attrs, point.count()));
                        }
                    }
                }
            }
        }
        None
    }

    #[test]
    fn stable_duration_metric_carries_expected_attributes() {
        let harness = test_meter();
        let handler = CosmosMetricsHandler::with_meter(harness.meter.clone());

        let cx = Context::new().with_value(operation_context());
        handler.handle(&completed(200), &cx);

        let metrics = harness.collect();
        let (attrs, count) =
            duration_point(&metrics).expect("db.client.operation.duration should be emitted");

        // Exactly one operation recorded.
        assert_eq!(count, 1);

        // Stable attributes match semconv exactly.
        assert_eq!(
            attrs
                .get(attributes::ATTR_DB_SYSTEM_NAME)
                .map(String::as_str),
            Some(attributes::DB_SYSTEM_NAME_VALUE)
        );
        assert_eq!(
            attrs
                .get(attributes::ATTR_DB_OPERATION_NAME)
                .map(String::as_str),
            Some("read_item")
        );
        assert_eq!(
            attrs
                .get(attributes::ATTR_DB_COLLECTION_NAME)
                .map(String::as_str),
            Some("my_container")
        );
        assert_eq!(
            attrs.get(attributes::ATTR_DB_NAMESPACE).map(String::as_str),
            Some("my_db")
        );
        assert_eq!(
            attrs
                .get(attributes::ATTR_DB_RESPONSE_STATUS_CODE)
                .map(String::as_str),
            Some("200")
        );
        assert_eq!(
            attrs
                .get(attributes::ATTR_SERVER_ADDRESS)
                .map(String::as_str),
            Some("my-account.documents.azure.com")
        );

        // Success => no error.type, and no development attributes by default.
        assert!(!attrs.contains_key(attributes::ATTR_ERROR_TYPE));
        assert!(!attrs.contains_key(attributes::ATTR_CONSISTENCY_LEVEL));
        assert!(!attrs.contains_key(attributes::ATTR_SUB_STATUS_CODE));
    }

    #[test]
    fn failure_sets_error_type_to_status_code() {
        let harness = test_meter();
        let handler = CosmosMetricsHandler::with_meter(harness.meter.clone());

        let cx = Context::new().with_value(operation_context());
        handler.handle(&completed(404), &cx);

        let metrics = harness.collect();
        let (attrs, _) = duration_point(&metrics).expect("duration metric should be emitted");

        assert_eq!(
            attrs
                .get(attributes::ATTR_DB_RESPONSE_STATUS_CODE)
                .map(String::as_str),
            Some("404")
        );
        assert_eq!(
            attrs.get(attributes::ATTR_ERROR_TYPE).map(String::as_str),
            Some("404")
        );
    }

    #[test]
    fn development_metrics_off_by_default() {
        let harness = test_meter();
        let handler = CosmosMetricsHandler::with_meter(harness.meter.clone());

        let cx = Context::new().with_value(operation_context().with_returned_item_count(7));
        handler.handle(&completed(200), &cx);

        let names = metric_names(&harness.collect());
        assert!(names
            .iter()
            .any(|n| n == attributes::METRIC_OPERATION_DURATION));
        // Only the stable metric is emitted with default options.
        assert!(!names
            .iter()
            .any(|n| n == attributes::METRIC_OPERATION_REQUEST_CHARGE));
        assert!(!names
            .iter()
            .any(|n| n == attributes::METRIC_RESPONSE_RETURNED_ROWS));
        assert!(!names
            .iter()
            .any(|n| n == attributes::METRIC_ACTIVE_INSTANCE_COUNT));
    }

    #[test]
    fn development_metrics_emitted_when_enabled() {
        let harness = test_meter();
        let options = MetricsOptions::default()
            .with_development_metrics(true)
            .with_development_attributes(true);
        let handler = CosmosMetricsHandler::with_meter_and_options(harness.meter.clone(), options);

        let cx = Context::new().with_value(operation_context().with_returned_item_count(7));
        handler.handle(&completed(200), &cx);

        let names = metric_names(&harness.collect());
        assert!(names
            .iter()
            .any(|n| n == attributes::METRIC_OPERATION_DURATION));
        assert!(names
            .iter()
            .any(|n| n == attributes::METRIC_OPERATION_REQUEST_CHARGE));
        assert!(names
            .iter()
            .any(|n| n == attributes::METRIC_RESPONSE_RETURNED_ROWS));
    }

    #[test]
    fn missing_operation_context_still_emits_duration() {
        // With no CosmosOperationContext on the pipeline context, the handler
        // must still emit the duration metric (identity attributes just absent).
        let harness = test_meter();
        let handler = CosmosMetricsHandler::with_meter(harness.meter.clone());

        handler.handle(&completed(200), &Context::new());

        let metrics = harness.collect();
        let (attrs, count) = duration_point(&metrics).expect("duration metric should be emitted");
        assert_eq!(count, 1);
        assert_eq!(
            attrs
                .get(attributes::ATTR_DB_SYSTEM_NAME)
                .map(String::as_str),
            Some(attributes::DB_SYSTEM_NAME_VALUE)
        );
        assert!(!attrs.contains_key(attributes::ATTR_DB_OPERATION_NAME));
    }

    #[test]
    fn no_meter_provider_is_a_noop() {
        // Building from the global meter with no provider registered yields a
        // no-op meter; recording must not panic (the exporter-absent path).
        let handler = CosmosMetricsHandler::new();
        handler.handle(&completed(200), &Context::new());
    }

    #[test]
    fn host_of_extracts_host() {
        assert_eq!(
            host_of("https://my-account.documents.azure.com:443/dbs/x"),
            Some("my-account.documents.azure.com".to_string())
        );
        assert_eq!(host_of("not a url"), None);
    }
}
