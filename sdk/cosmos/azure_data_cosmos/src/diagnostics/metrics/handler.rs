// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`CosmosMetricsHandler`] — emits OpenTelemetry metrics from a completed
//! [`DiagnosticsContext`].

use azure_core::http::Context;
use opentelemetry::metrics::Meter;
use opentelemetry::{global, Array, KeyValue, StringValue, Value};

use crate::diagnostics::metrics::attributes;
use crate::diagnostics::metrics::instruments::Instruments;
use crate::diagnostics::metrics::MetricsOptions;
use crate::diagnostics::{
    ClientLifetimeToken, CosmosClientInfo, CosmosOperationContext, DiagnosticsContext,
    DiagnosticsHandler,
};

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
/// histogram. The optional per-signal metrics and the extended attribute set are
/// opt-in via [`MetricsOptions`] (see [`with_options`](CosmosMetricsHandler::with_options)).
///
/// When the active-instance metric is enabled
/// ([`MetricsOptions::with_active_instance_metric`]), the handler increments the
/// `azure.cosmosdb.client.active_instance.count` up-down counter each time a
/// [`CosmosClient`](crate::CosmosClient) is built with it registered, and
/// decrements it when that client (and every database/container client derived
/// from it) is dropped. The reported value is therefore the number of live
/// client instances per account endpoint, independent of how many handler
/// objects exist.
///
/// The handler captures a [`Meter`] from the globally-registered provider at
/// construction. Install your meter provider **before** constructing the handler:
/// a `Meter` obtained while the global provider is still the default no-op stays a
/// no-op even after a real provider is installed later, so metrics would be
/// silently dropped. If you need to build the handler before the global provider
/// is ready, bind it to an explicit meter with
/// [`with_meter`](CosmosMetricsHandler::with_meter) instead.
pub struct CosmosMetricsHandler {
    instruments: Instruments,
    options: MetricsOptions,
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
        }
    }

    /// Attribute set for the active-instance up-down counter.
    ///
    /// Per the `azure.cosmosdb.client.active_instance.count` semantic
    /// convention, the counter is keyed on the account endpoint
    /// (`server.address`, plus `server.port` only when the endpoint uses a
    /// non-default port), so the value reads as "live clients per account".
    fn active_instance_attributes(client: &CosmosClientInfo) -> Vec<KeyValue> {
        let mut attrs = Vec::with_capacity(3);
        attrs.push(KeyValue::new(
            attributes::ATTR_DB_SYSTEM_NAME,
            attributes::DB_SYSTEM_NAME_VALUE,
        ));
        if let Some(address) = client.server_address() {
            attrs.push(KeyValue::new(
                attributes::ATTR_SERVER_ADDRESS,
                address.to_string(),
            ));
        }
        if let Some(port) = client.server_port() {
            attrs.push(KeyValue::new(attributes::ATTR_SERVER_PORT, i64::from(port)));
        }
        attrs
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
    /// per-operation optional histograms.
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
        // Use the effective status (operation status, else the terminal attempt's)
        // so status-less error-finalization paths still report an accurate status
        // and error.type instead of the _OTHER catch-all.
        match diagnostics.effective_status() {
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
                // No status anywhere — a client/transport failure with no HTTP
                // response and no attempt. Classify it as the semconv catch-all.
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

        // Extended attributes are opt-in (higher cardinality; D7).
        if self.options.extended_attributes_enabled() {
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
            if let Some(sub_status) = diagnostics.effective_status().and_then(|s| s.sub_status()) {
                attrs.push(KeyValue::new(
                    attributes::ATTR_SUB_STATUS_CODE,
                    i64::from(sub_status.value()),
                ));
            }
            let regions = diagnostics.regions_contacted();
            if !regions.is_empty() {
                // Semantic conventions define contacted_regions as an ordered
                // string[]; emit an OpenTelemetry array, not a joined scalar.
                let values: Vec<StringValue> = regions
                    .iter()
                    .map(|region| StringValue::from(region.as_str().to_string()))
                    .collect();
                attrs.push(KeyValue::new(
                    attributes::ATTR_CONTACTED_REGIONS,
                    Value::Array(Array::String(values)),
                ));
            }
        }

        attrs
    }
}

impl CosmosMetricsHandler {
    /// Value of the `hedge_terminal_state` dimension when a hedge demonstrably
    /// fanned out but the race retained no terminal outcome.
    ///
    /// The both-transient→failover path deliberately leaves `hedge_diagnostics`
    /// unset so a later successful retry does not carry a misleading
    /// `BothTransient` state. Those operations really did hedge, so they must be
    /// counted; this sentinel keeps the counter's attribute schema uniform
    /// (every data point carries the dimension, so `group by
    /// hedge_terminal_state` never fragments) while staying distinguishable
    /// from every real `HedgeTerminalState` value.
    const HEDGE_TERMINAL_STATE_UNRESOLVED: &'static str = "unresolved";

    /// Records the hedged-operation counter for an operation that fanned out a
    /// cross-region hedge.
    ///
    /// The low-cardinality `hedge_terminal_state` dimension is always attached;
    /// the higher-cardinality `hedge_region` dimension is added only under the
    /// extended-attributes opt-in (mirroring how contacted regions are gated on
    /// the duration metric).
    ///
    /// Fan-out is decided by [`DiagnosticsContext::hedging_started`], which is
    /// materialized from the dispatch-time fan-out log and is therefore the
    /// authoritative signal — gating on `hedge_diagnostics` instead would
    /// silently undercount, because a both-transient race that later succeeds
    /// through failover retains no terminal outcome. An aggregated operation
    /// (e.g. PATCH) whose sub-op hedged is counted once for the whole operation.
    fn record_hedged(&self, diagnostics: &DiagnosticsContext, base_attrs: &[KeyValue]) {
        if !diagnostics.hedging_started() {
            return;
        }
        let hedge = diagnostics.hedge_diagnostics();
        let mut attrs = base_attrs.to_vec();
        attrs.push(KeyValue::new(
            attributes::ATTR_HEDGE_TERMINAL_STATE,
            hedge.map_or(Self::HEDGE_TERMINAL_STATE_UNRESOLVED, |hedge| {
                hedge.terminal_state().as_str()
            }),
        ));
        if self.options.extended_attributes_enabled() {
            if let Some(alternate) = hedge.and_then(|hedge| hedge.alternate_region()) {
                attrs.push(KeyValue::new(
                    attributes::ATTR_HEDGE_REGION,
                    alternate.as_str().to_string(),
                ));
            }
        }
        self.instruments.hedged.add(1, &attrs);
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

        // Optional per-signal metrics (each opt-in).
        if self.options.request_charge_metric_enabled() {
            self.instruments
                .request_charge
                .record(diagnostics.total_request_charge().value(), &attributes);
        }

        if self.options.returned_rows_metric_enabled() {
            if let Some(rows) = op.and_then(CosmosOperationContext::returned_item_count) {
                self.instruments.returned_rows.record(rows, &attributes);
            }
        }

        // Hedging counter: emitted only when opted in and a hedge actually
        // fanned out. `record_hedged` re-checks fan-out so the invariant holds
        // regardless of call site.
        if self.options.hedged_metric_enabled() {
            self.record_hedged(diagnostics, &attributes);
        }
    }

    fn on_client_created(&self, client: &CosmosClientInfo) -> Option<ClientLifetimeToken> {
        if !self.options.active_instance_metric_enabled() {
            return None;
        }

        // Record the +1 half of the up-down counter now, and hand back a token
        // whose `Drop` records the matching -1. The token rides on the client's
        // shared state, so the counter tracks live *clients* rather than live
        // handler objects — a single handler may be registered on many clients.
        let attributes = Self::active_instance_attributes(client);
        let counter = self.instruments.active_instance.clone();
        counter.add(1, &attributes);
        Some(ClientLifetimeToken::new(move || {
            counter.add(-1, &attributes);
        }))
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
    use azure_data_cosmos_driver::diagnostics::{HedgeDiagnostics, HedgeTerminalState};
    use azure_data_cosmos_driver::models::ActivityId;
    use azure_data_cosmos_driver::options::Region;
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

    /// Completed context whose operation fanned out an `AlternateWon` hedge to
    /// West US 2.
    fn hedged_completed(status_code: u16) -> DiagnosticsContext {
        let hedge = HedgeDiagnostics::for_testing(
            Region::EAST_US,
            Some(Region::WEST_US_2),
            Some(Region::WEST_US_2),
            HedgeTerminalState::AlternateWon,
        );
        DiagnosticsContext::for_testing_with_hedge(
            ActivityId::new_uuid(),
            Duration::from_millis(42),
            Some(CosmosStatus::new(StatusCode::from(status_code))),
            Some("read_item"),
            Vec::new(),
            Some(hedge),
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

    /// Builds a [`CosmosClientInfo`] for `host`, optionally on a non-default
    /// port, the way `CosmosClientBuilder::build` would from an account
    /// endpoint.
    fn test_client_info(host: &str, port: Option<u16>) -> CosmosClientInfo {
        let url = match port {
            Some(port) => format!("https://{host}:{port}/"),
            None => format!("https://{host}/"),
        };
        CosmosClientInfo::from_endpoint(&url::Url::parse(&url).expect("valid test endpoint"))
    }

    /// Returns the summed value of the `active_instance.count` up-down counter
    /// from the most recent export, or `None` if the metric was never emitted.
    ///
    /// The in-memory exporter accumulates one snapshot per `collect()` call, so
    /// we scan every snapshot and keep the value from the last one — the current
    /// cumulative count.
    fn active_instance_value(metrics: &[ResourceMetrics]) -> Option<i64> {
        let mut latest = None;
        for rm in metrics {
            for sm in rm.scope_metrics() {
                for m in sm.metrics() {
                    if m.name() != attributes::METRIC_ACTIVE_INSTANCE_COUNT {
                        continue;
                    }
                    if let AggregatedMetrics::I64(MetricData::Sum(sum)) = m.data() {
                        latest = Some(sum.data_points().map(|point| point.value()).sum());
                    }
                }
            }
        }
        latest
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

        // Success => no error.type, and no extended attributes by default.
        assert!(!attrs.contains_key(attributes::ATTR_ERROR_TYPE));
        assert!(!attrs.contains_key(attributes::ATTR_CONSISTENCY_LEVEL));
        assert!(!attrs.contains_key(attributes::ATTR_SUB_STATUS_CODE));
    }

    /// Returns `(bounds, bucket_counts)` for the duration histogram.
    fn duration_buckets(metrics: &[ResourceMetrics]) -> Option<(Vec<f64>, Vec<u64>)> {
        for rm in metrics {
            for sm in rm.scope_metrics() {
                for m in sm.metrics() {
                    if m.name() != attributes::METRIC_OPERATION_DURATION {
                        continue;
                    }
                    if let AggregatedMetrics::F64(MetricData::Histogram(histogram)) = m.data() {
                        if let Some(point) = histogram.data_points().next() {
                            return Some((
                                point.bounds().collect(),
                                point.bucket_counts().collect(),
                            ));
                        }
                    }
                }
            }
        }
        None
    }

    /// The duration histogram must be able to *distinguish* realistic Cosmos
    /// latencies, which is the whole point of recording it as a histogram.
    ///
    /// This guards a failure mode that is invisible from the outside: the metric
    /// is recorded in seconds, but OpenTelemetry's default bucket boundaries are
    /// scaled for milliseconds. With those defaults every real operation lands in
    /// the first bucket, `histogram_quantile` degenerates to interpolation inside
    /// that one bucket, and p50/p95/p99 become constants that depend only on the
    /// quantile requested. Dashboards keep drawing plausible-looking lines that
    /// can never move, so a latency regression cannot be detected — and nothing
    /// errors to say so.
    ///
    /// Asserting on *separation* rather than on the literal boundary list keeps
    /// this a test of the property we care about: the boundaries stay free to be
    /// re-tuned, as long as they still resolve these three latencies apart.
    #[test]
    fn duration_histogram_separates_realistic_latencies() {
        let harness = test_meter();
        let handler = CosmosMetricsHandler::with_meter(harness.meter.clone());
        let cx = Context::new().with_value(operation_context());

        // A fast point read, a slow-but-normal query, and a degraded request.
        for millis in [2_u64, 30, 300] {
            handler.handle(
                &DiagnosticsContext::for_testing_completed(
                    ActivityId::new_uuid(),
                    Duration::from_millis(millis),
                    Some(CosmosStatus::new(StatusCode::from(200))),
                ),
                &cx,
            );
        }

        let metrics = harness.collect();
        let (bounds, counts) =
            duration_buckets(&metrics).expect("duration histogram should be emitted");

        assert_eq!(counts.iter().sum::<u64>(), 3, "all observations recorded");

        let occupied = counts.iter().filter(|c| **c > 0).count();
        assert_eq!(
            occupied, 3,
            "2ms/30ms/300ms must land in 3 distinct buckets, but they occupy {occupied}; \
             boundaries are {bounds:?}. Boundaries scaled for milliseconds collapse every \
             real latency into one bucket and make percentiles constant."
        );

        // Guard the specific regression: seconds-valued data against
        // millisecond-scaled boundaries. 10s is a degraded request, not a
        // routine one, so nothing sane needs a boundary above it.
        assert!(
            bounds.iter().all(|b| *b <= 10.0),
            "boundaries look millisecond-scaled for a seconds-valued metric: {bounds:?}"
        );
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
    }

    #[test]
    fn development_metrics_emitted_when_enabled() {
        let harness = test_meter();
        let options = MetricsOptions::default()
            .with_request_charge_metric(true)
            .with_returned_rows_metric(true)
            .with_extended_attributes(true);
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
    fn optional_metrics_toggle_independently() {
        // Enabling only the request-charge signal emits it — and NOT returned_rows —
        // proving the per-signal toggles are independent.
        let harness = test_meter();
        let options = MetricsOptions::default().with_request_charge_metric(true);
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
        assert!(!names
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
    fn active_instance_metric_off_by_default() {
        // With default options the active-instance counter is never touched, so
        // no such series is exported even across a full client lifecycle.
        let harness = test_meter();
        let handler = CosmosMetricsHandler::with_meter(harness.meter.clone());
        let token = handler.on_client_created(&test_client_info("acct.documents.azure.com", None));
        assert!(token.is_none(), "disabled metric must not take a token");
        assert_eq!(active_instance_value(&harness.collect()), None);
        drop(token);
        assert_eq!(active_instance_value(&harness.collect()), None);
    }

    #[test]
    fn active_instance_metric_tracks_client_lifecycle_not_handler_lifecycle() {
        let harness = test_meter();
        let options = MetricsOptions::default().with_active_instance_metric(true);

        let handler = CosmosMetricsHandler::with_meter_and_options(harness.meter.clone(), options);
        // Constructing the handler alone records nothing: a handler is not a
        // client, and one handler may be registered on many clients or none.
        assert_eq!(active_instance_value(&harness.collect()), None);

        let info = test_client_info("acct.documents.azure.com", None);
        let first = handler
            .on_client_created(&info)
            .expect("enabled metric must take a lifetime token");
        assert_eq!(active_instance_value(&harness.collect()), Some(1));

        // The same handler registered on a second client counts twice — the
        // regression this replaces counted handler objects, so it reported 1.
        let second = handler
            .on_client_created(&info)
            .expect("enabled metric must take a lifetime token");
        assert_eq!(active_instance_value(&harness.collect()), Some(2));

        drop(first);
        assert_eq!(active_instance_value(&harness.collect()), Some(1));

        // Dropping the handler while a client token is still alive must not
        // decrement: the client, not the handler, owns the count.
        drop(handler);
        assert_eq!(active_instance_value(&harness.collect()), Some(1));

        drop(second);
        assert_eq!(active_instance_value(&harness.collect()), Some(0));
    }

    #[test]
    fn active_instance_metric_is_keyed_on_account_endpoint() {
        // Per semconv the counter carries `server.address`, and `server.port`
        // only when the endpoint uses a non-default port.
        let default_port = CosmosMetricsHandler::active_instance_attributes(&test_client_info(
            "acct.documents.azure.com",
            None,
        ));
        let by_key: HashMap<_, _> = default_port
            .iter()
            .map(|kv| (kv.key.as_str().to_string(), kv.value.as_str().to_string()))
            .collect();
        assert_eq!(
            by_key
                .get(attributes::ATTR_DB_SYSTEM_NAME)
                .map(String::as_str),
            Some(attributes::DB_SYSTEM_NAME_VALUE)
        );
        assert_eq!(
            by_key
                .get(attributes::ATTR_SERVER_ADDRESS)
                .map(String::as_str),
            Some("acct.documents.azure.com")
        );
        assert!(
            !by_key.contains_key(attributes::ATTR_SERVER_PORT),
            "default port must be omitted; got {by_key:?}"
        );

        let custom_port = CosmosMetricsHandler::active_instance_attributes(&test_client_info(
            "localhost",
            Some(8081),
        ));
        assert!(
            custom_port
                .iter()
                .any(|kv| kv.key.as_str() == attributes::ATTR_SERVER_PORT
                    && kv.value.as_str() == "8081"),
            "non-default port must be emitted; got {custom_port:?}"
        );
    }

    #[test]
    fn host_of_extracts_host() {
        assert_eq!(
            host_of("https://my-account.documents.azure.com:443/dbs/x"),
            Some("my-account.documents.azure.com".to_string())
        );
        assert_eq!(host_of("not a url"), None);
    }

    /// Returns the attributes (as a string map) and summed value of the
    /// `azure.cosmosdb.client.operation.hedged` counter, if present.
    fn hedged_point(metrics: &[ResourceMetrics]) -> Option<(HashMap<String, String>, u64)> {
        for rm in metrics {
            for sm in rm.scope_metrics() {
                for m in sm.metrics() {
                    if m.name() != attributes::METRIC_OPERATION_HEDGED {
                        continue;
                    }
                    if let AggregatedMetrics::U64(MetricData::Sum(sum)) = m.data() {
                        if let Some(point) = sum.data_points().next() {
                            let attrs = point
                                .attributes()
                                .map(|kv| {
                                    (kv.key.as_str().to_string(), kv.value.as_str().into_owned())
                                })
                                .collect();
                            return Some((attrs, point.value()));
                        }
                    }
                }
            }
        }
        None
    }

    #[test]
    fn hedged_metric_off_by_default_even_for_hedged_operation() {
        let harness = test_meter();
        let handler = CosmosMetricsHandler::with_meter(harness.meter.clone());

        let cx = Context::new().with_value(operation_context());
        handler.handle(&hedged_completed(200), &cx);

        // The stable duration metric is emitted, but the opt-in hedged counter is not.
        let names = metric_names(&harness.collect());
        assert!(names
            .iter()
            .any(|n| n == attributes::METRIC_OPERATION_DURATION));
        assert!(!names
            .iter()
            .any(|n| n == attributes::METRIC_OPERATION_HEDGED));
    }

    #[test]
    fn hedged_metric_emitted_for_hedged_operation_when_enabled() {
        let harness = test_meter();
        let options = MetricsOptions::default().with_hedged_metric(true);
        let handler = CosmosMetricsHandler::with_meter_and_options(harness.meter.clone(), options);

        let cx = Context::new().with_value(operation_context());
        handler.handle(&hedged_completed(200), &cx);

        let metrics = harness.collect();
        let (attrs, value) = hedged_point(&metrics).expect("hedged counter should be emitted");
        assert_eq!(value, 1);

        // Operation identity carries through, plus the low-cardinality terminal state.
        assert_eq!(
            attrs
                .get(attributes::ATTR_DB_OPERATION_NAME)
                .map(String::as_str),
            Some("read_item")
        );
        assert_eq!(
            attrs
                .get(attributes::ATTR_HEDGE_TERMINAL_STATE)
                .map(String::as_str),
            Some("alternate_won")
        );
        // The high-cardinality region dimension stays off without extended attributes.
        assert!(!attrs.contains_key(attributes::ATTR_HEDGE_REGION));
    }

    #[test]
    fn hedged_metric_adds_region_dimension_under_extended_attributes() {
        let harness = test_meter();
        let options = MetricsOptions::default()
            .with_hedged_metric(true)
            .with_extended_attributes(true);
        let handler = CosmosMetricsHandler::with_meter_and_options(harness.meter.clone(), options);

        let cx = Context::new().with_value(operation_context());
        handler.handle(&hedged_completed(200), &cx);

        let metrics = harness.collect();
        let (attrs, _) = hedged_point(&metrics).expect("hedged counter should be emitted");
        assert_eq!(
            attrs.get(attributes::ATTR_HEDGE_REGION).map(String::as_str),
            Some("westus2")
        );
    }

    #[test]
    fn hedged_metric_not_emitted_for_non_hedged_operation() {
        let harness = test_meter();
        let options = MetricsOptions::default().with_hedged_metric(true);
        let handler = CosmosMetricsHandler::with_meter_and_options(harness.meter.clone(), options);

        // Enabled, but this operation never hedged: no counter data point.
        let cx = Context::new().with_value(operation_context());
        handler.handle(&completed(200), &cx);

        let metrics = harness.collect();
        assert!(
            hedged_point(&metrics).is_none(),
            "a non-hedged operation must not increment the hedged counter"
        );
    }

    #[test]
    fn hedged_metric_counts_hedge_without_terminal_state() {
        // A hedge that fanned out both-transient and was then resolved by a later
        // failover attempt leaves a retained `Hedging` request (so
        // `hedging_started()` is true) but no recorded terminal outcome
        // (`finalize_both_transient` deliberately does not stamp `hedge_diagnostics`
        // on the non-terminal path — see operation_pipeline.rs). That operation
        // really did hedge, so it must be counted; the dimension carries the
        // `unresolved` sentinel rather than being omitted, which keeps the
        // counter's attribute schema uniform for `group by hedge_terminal_state`.
        use azure_data_cosmos_driver::diagnostics::{ExecutionContext, RequestDiagnostics};
        use azure_data_cosmos_driver::models::RequestCharge;
        use std::time::Instant;

        let harness = test_meter();
        let options = MetricsOptions::default().with_hedged_metric(true);
        let handler = CosmosMetricsHandler::with_meter_and_options(harness.meter.clone(), options);

        let now = Instant::now();
        let hedge_leg = RequestDiagnostics::for_testing(
            "https://acct-westus2.documents.azure.com:443/",
            Some(Region::WEST_US_2),
            CosmosStatus::new(StatusCode::Ok),
            RequestCharge::new(1.0),
            now - Duration::from_millis(50),
            now,
        )
        .with_execution_context_for_testing(ExecutionContext::Hedging);
        let ctx = DiagnosticsContext::for_testing_with_hedge(
            ActivityId::new_uuid(),
            Duration::from_millis(42),
            Some(CosmosStatus::new(StatusCode::Ok)),
            Some("read_item"),
            vec![hedge_leg],
            None,
        );
        assert!(
            ctx.hedging_started(),
            "a Hedging-tagged request makes hedging_started() true"
        );

        let cx = Context::new().with_value(operation_context());
        handler.handle(&ctx, &cx);

        let metrics = harness.collect();
        let (attrs, value) = hedged_point(&metrics)
            .expect("a hedge that fanned out is counted even without a terminal outcome");
        assert_eq!(value, 1);
        assert_eq!(
            attrs
                .get(attributes::ATTR_HEDGE_TERMINAL_STATE)
                .map(String::as_str),
            Some("unresolved"),
            "the dimension is always present so the counter's schema stays uniform"
        );
    }
}
