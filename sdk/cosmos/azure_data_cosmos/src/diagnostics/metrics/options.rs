// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Configuration for the Cosmos DB metrics handler.

/// Controls which optional metrics and attributes the
/// [`CosmosMetricsHandler`](super::CosmosMetricsHandler) emits.
///
/// The options are per-signal: each optional metric and the extended attribute
/// set is toggled on its own, focused on *what* is emitted rather than on a
/// "preview/development" tier. Everything optional is **off by default** — only
/// the stable, low-cardinality operation-duration metric is emitted — so enabling
/// metrics never silently multiplies a backend's time-series count (design
/// decision **D7**); each additional signal is an explicit opt-in.
///
/// # Examples
///
/// ```
/// use azure_data_cosmos::diagnostics::MetricsOptions;
///
/// // Stable metric only (default).
/// let stable = MetricsOptions::default();
/// assert!(!stable.request_charge_metric_enabled());
/// assert!(!stable.returned_rows_metric_enabled());
/// assert!(!stable.extended_attributes_enabled());
///
/// // Opt into just the request-charge metric.
/// let charge = MetricsOptions::default().with_request_charge_metric(true);
/// assert!(charge.request_charge_metric_enabled());
/// assert!(!charge.returned_rows_metric_enabled());
///
/// // Opt into everything.
/// let full = MetricsOptions::default()
///     .with_request_charge_metric(true)
///     .with_returned_rows_metric(true)
///     .with_active_instance_metric(true)
///     .with_hedged_metric(true)
///     .with_extended_attributes(true);
/// assert!(full.request_charge_metric_enabled());
/// assert!(full.returned_rows_metric_enabled());
/// assert!(full.active_instance_metric_enabled());
/// assert!(full.hedged_metric_enabled());
/// assert!(full.extended_attributes_enabled());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MetricsOptions {
    request_charge_metric: bool,
    returned_rows_metric: bool,
    active_instance_metric: bool,
    hedged_metric: bool,
    extended_attributes: bool,
}

impl MetricsOptions {
    /// Returns options with every optional signal disabled — the stable
    /// operation-duration metric only.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables (or disables) the `azure.cosmosdb.client.operation.request_charge`
    /// histogram (request units consumed per operation). Off by default.
    #[must_use]
    pub fn with_request_charge_metric(mut self, enabled: bool) -> Self {
        self.request_charge_metric = enabled;
        self
    }

    /// Enables (or disables) the `db.client.response.returned_rows` histogram
    /// (rows/items returned per operation). Off by default.
    #[must_use]
    pub fn with_returned_rows_metric(mut self, enabled: bool) -> Self {
        self.returned_rows_metric = enabled;
        self
    }

    /// Enables (or disables) the
    /// `azure.cosmosdb.client.active_instance.count` up-down counter, which
    /// tracks the number of live [`CosmosClient`](crate::CosmosClient)
    /// instances: it is incremented by one when a client is built with this
    /// handler registered and decremented by one when that client — and every
    /// database/container client derived from it — has been dropped. The counter
    /// is keyed on the account endpoint (`server.address`, plus `server.port`
    /// for non-default ports), so sharing one handler across several clients
    /// still reports each client, and registering the same handler twice on one
    /// client still reports that client once. Off by default.
    ///
    /// Note that two *distinct* metrics handlers built from the same meter and
    /// registered on the same client each record their own `+1`, just as they
    /// each record their own duration and request-charge samples. Register a
    /// single metrics handler per meter.
    #[must_use]
    pub fn with_active_instance_metric(mut self, enabled: bool) -> Self {
        self.active_instance_metric = enabled;
        self
    }

    /// Enables (or disables) the `azure.cosmosdb.client.operation.hedged` counter
    /// (operations that dispatched a cross-region hedge fan-out). Off by default.
    ///
    /// The counter increments only for operations where hedging actually fanned
    /// out, so it is near-zero cardinality; it always carries the low-cardinality
    /// `hedge_terminal_state` dimension. The higher-cardinality hedge-region
    /// dimension is added only when [`with_extended_attributes`](Self::with_extended_attributes)
    /// is also enabled.
    #[must_use]
    pub fn with_hedged_metric(mut self, enabled: bool) -> Self {
        self.hedged_metric = enabled;
        self
    }

    /// Enables (or disables) the extended attribute set on every emitted metric:
    /// consistency level, contacted regions, sub-status code, and connection
    /// mode. These can be higher cardinality, so they are opt-in and off by
    /// default.
    #[must_use]
    pub fn with_extended_attributes(mut self, enabled: bool) -> Self {
        self.extended_attributes = enabled;
        self
    }

    /// Whether the request-charge histogram is emitted.
    pub fn request_charge_metric_enabled(&self) -> bool {
        self.request_charge_metric
    }

    /// Whether the returned-rows histogram is emitted.
    pub fn returned_rows_metric_enabled(&self) -> bool {
        self.returned_rows_metric
    }

    /// Whether the active-instance up-down counter is emitted.
    pub fn active_instance_metric_enabled(&self) -> bool {
        self.active_instance_metric
    }

    /// Whether the hedged-operation counter is emitted.
    pub fn hedged_metric_enabled(&self) -> bool {
        self.hedged_metric
    }

    /// Whether the extended attribute set is attached to emitted metrics.
    pub fn extended_attributes_enabled(&self) -> bool {
        self.extended_attributes
    }
}
