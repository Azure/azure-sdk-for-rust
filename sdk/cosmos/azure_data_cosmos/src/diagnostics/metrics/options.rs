// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Configuration for the Cosmos DB metrics handler.

/// Controls which optional metrics and attributes the
/// [`CosmosMetricsHandler`](super::CosmosMetricsHandler) emits.
///
/// Defaults follow design decision **D7**: only the stable, low-cardinality
/// operation-scope signal is on. The development-tier metrics and the
/// higher-cardinality development attributes are **off by default** and must be
/// opted into explicitly, so enabling metrics never silently multiplies a
/// backend's time-series count.
///
/// # Examples
///
/// ```
/// use azure_data_cosmos::diagnostics::MetricsOptions;
///
/// // Stable metric only (default).
/// let stable = MetricsOptions::default();
/// assert!(!stable.development_metrics_enabled());
/// assert!(!stable.development_attributes_enabled());
///
/// // Opt into the full development tier.
/// let dev = MetricsOptions::default()
///     .with_development_metrics(true)
///     .with_development_attributes(true);
/// assert!(dev.development_metrics_enabled());
/// assert!(dev.development_attributes_enabled());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MetricsOptions {
    development_metrics: bool,
    development_attributes: bool,
}

impl MetricsOptions {
    /// Returns options with every development signal disabled — the stable
    /// operation-duration metric only.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables (or disables) the development-tier metrics:
    /// `azure.cosmosdb.client.operation.request_charge`,
    /// `db.client.response.returned_rows`, and
    /// `azure.cosmosdb.client.active_instance.count`.
    #[must_use]
    pub fn with_development_metrics(mut self, enabled: bool) -> Self {
        self.development_metrics = enabled;
        self
    }

    /// Enables (or disables) the development-tier attributes on every emitted
    /// metric: consistency level, contacted regions, sub-status code, and
    /// connection mode. These can be higher cardinality, so they are opt-in.
    #[must_use]
    pub fn with_development_attributes(mut self, enabled: bool) -> Self {
        self.development_attributes = enabled;
        self
    }

    /// Whether the development-tier metrics are emitted.
    pub fn development_metrics_enabled(&self) -> bool {
        self.development_metrics
    }

    /// Whether the development-tier attributes are attached to metrics.
    pub fn development_attributes_enabled(&self) -> bool {
        self.development_attributes
    }
}
