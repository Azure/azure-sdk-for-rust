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
///     .with_extended_attributes(true);
/// assert!(full.request_charge_metric_enabled());
/// assert!(full.returned_rows_metric_enabled());
/// assert!(full.extended_attributes_enabled());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MetricsOptions {
    request_charge_metric: bool,
    returned_rows_metric: bool,
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

    /// Whether the extended attribute set is attached to emitted metrics.
    pub fn extended_attributes_enabled(&self) -> bool {
        self.extended_attributes
    }
}
