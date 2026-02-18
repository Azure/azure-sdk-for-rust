// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Runtime-configurable options shared across environment, driver, and operation levels.

use azure_core::http::headers::Headers;
// Note: `std::sync::RwLock` is used intentionally here instead of `tokio::sync::RwLock`
// because `RuntimeOptions` may be read from synchronous contexts (e.g., builder construction,
// configuration merging). The lock is held only briefly for reads/writes of option values,
// so contention is minimal. Using `std::sync::RwLock` also avoids coupling the crate to a
// specific async runtime (tokio), which is important for runtime-agnostic design.
use std::sync::{Arc, RwLock};

use crate::{
    models::ThroughputControlGroupName,
    options::{
        ContentResponseOnWrite, DedicatedGatewayOptions, DiagnosticsThresholds,
        EndToEndOperationLatencyPolicy, ExcludedRegions, ReadConsistencyStrategy,
    },
};

/// Runtime-configurable options that can be set at environment, driver, or operation level.
///
/// These options follow a hierarchy where operation-level settings override driver-level,
/// which in turn override environment-level defaults.
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct RuntimeOptions {
    /// Throughput control group name for rate limiting.
    pub throughput_control_group_name: Option<ThroughputControlGroupName>,
    /// Dedicated gateway options for integrated cache.
    pub dedicated_gateway_options: Option<DedicatedGatewayOptions>,
    /// Diagnostics thresholds for logging and monitoring.
    pub diagnostics_thresholds: Option<DiagnosticsThresholds>,
    /// End-to-end latency policy for timeout management.
    pub end_to_end_latency_policy: Option<EndToEndOperationLatencyPolicy>,
    /// Custom HTTP headers to include in requests.
    ///
    /// These headers are best-effort: the caller may specify any header, but whether
    /// it is actually sent to the service is at the discretion of the library. There
    /// is no guarantee that a given custom header will be included in the outgoing request.
    pub custom_headers: Option<Headers>,
    /// Regions to exclude from routing.
    pub excluded_regions: Option<ExcludedRegions>,
    /// Read consistency strategy for read operations.
    pub read_consistency_strategy: Option<ReadConsistencyStrategy>,
    /// Content response on write setting.
    pub content_response_on_write: Option<ContentResponseOnWrite>,
}

impl RuntimeOptions {
    /// Creates a new empty runtime options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a builder for creating runtime options.
    ///
    /// Use this to construct a new `RuntimeOptions` with specific values.
    pub fn builder() -> RuntimeOptionsBuilder {
        RuntimeOptionsBuilder::default()
    }

    /// Returns a builder initialized with this instance's values.
    ///
    /// Use this to create a modified copy of the current options.
    pub fn to_builder(&self) -> RuntimeOptionsBuilder {
        RuntimeOptionsBuilder::from_options(self.clone())
    }

    /// Merges this options with a base, returning a new options where
    /// `self` values take precedence over `base` values.
    pub fn merge_with_base(&self, base: &RuntimeOptions) -> RuntimeOptions {
        RuntimeOptions {
            throughput_control_group_name: self
                .throughput_control_group_name
                .clone()
                .or_else(|| base.throughput_control_group_name.clone()),
            dedicated_gateway_options: self
                .dedicated_gateway_options
                .clone()
                .or_else(|| base.dedicated_gateway_options.clone()),
            diagnostics_thresholds: self
                .diagnostics_thresholds
                .clone()
                .or_else(|| base.diagnostics_thresholds.clone()),
            end_to_end_latency_policy: self
                .end_to_end_latency_policy
                .clone()
                .or_else(|| base.end_to_end_latency_policy.clone()),
            custom_headers: self
                .custom_headers
                .clone()
                .or_else(|| base.custom_headers.clone()),
            excluded_regions: self
                .excluded_regions
                .clone()
                .or_else(|| base.excluded_regions.clone()),
            read_consistency_strategy: self
                .read_consistency_strategy
                .or(base.read_consistency_strategy),
            content_response_on_write: self
                .content_response_on_write
                .or(base.content_response_on_write),
        }
    }
}

/// Builder for creating [`RuntimeOptions`].
///
/// # Example
///
/// ```
/// use azure_data_cosmos_driver::options::{RuntimeOptions, RuntimeOptionsBuilder, ContentResponseOnWrite};
///
/// let options = RuntimeOptionsBuilder::new()
///     .with_content_response_on_write(ContentResponseOnWrite::DISABLED)
///     .build();
///
/// // Or modify an existing instance
/// let modified = options.to_builder()
///     .with_content_response_on_write(ContentResponseOnWrite::ENABLED)
///     .build();
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct RuntimeOptionsBuilder {
    options: RuntimeOptions,
}

impl RuntimeOptionsBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a builder from existing runtime options.
    pub fn from_options(options: RuntimeOptions) -> Self {
        Self { options }
    }

    /// Sets the throughput control group name.
    pub fn with_throughput_control_group_name(mut self, name: ThroughputControlGroupName) -> Self {
        self.options.throughput_control_group_name = Some(name);
        self
    }

    /// Sets the dedicated gateway options.
    pub fn with_dedicated_gateway_options(mut self, options: DedicatedGatewayOptions) -> Self {
        self.options.dedicated_gateway_options = Some(options);
        self
    }

    /// Sets the diagnostics thresholds.
    pub fn with_diagnostics_thresholds(mut self, thresholds: DiagnosticsThresholds) -> Self {
        self.options.diagnostics_thresholds = Some(thresholds);
        self
    }

    /// Sets the end-to-end latency policy.
    pub fn with_end_to_end_latency_policy(
        mut self,
        policy: EndToEndOperationLatencyPolicy,
    ) -> Self {
        self.options.end_to_end_latency_policy = Some(policy);
        self
    }

    /// Sets the custom headers.
    ///
    /// Custom headers are best-effort: the caller may specify any header, but whether
    /// it is actually sent to the service is at the discretion of the library. There
    /// is no guarantee that a given custom header will be included in the outgoing request.
    pub fn with_custom_headers(mut self, headers: Headers) -> Self {
        self.options.custom_headers = Some(headers);
        self
    }

    /// Sets the excluded regions.
    pub fn with_excluded_regions(mut self, regions: ExcludedRegions) -> Self {
        self.options.excluded_regions = Some(regions);
        self
    }

    /// Sets the read consistency strategy.
    pub fn with_read_consistency_strategy(mut self, strategy: ReadConsistencyStrategy) -> Self {
        self.options.read_consistency_strategy = Some(strategy);
        self
    }

    /// Sets the content response on write setting.
    pub fn with_content_response_on_write(mut self, value: ContentResponseOnWrite) -> Self {
        self.options.content_response_on_write = Some(value);
        self
    }

    /// Builds the [`RuntimeOptions`].
    pub fn build(self) -> RuntimeOptions {
        self.options
    }
}

/// Thread-safe wrapper for runtime options.
///
/// Provides interior mutability for runtime configuration changes.
/// Used by `EnvironmentOptions` and `DriverOptions` to allow runtime modification
/// of default settings.
#[derive(Clone, Debug, Default)]
pub struct SharedRuntimeOptions(Arc<RwLock<RuntimeOptions>>);

impl SharedRuntimeOptions {
    /// Creates a new empty shared runtime options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates shared runtime options from existing runtime options.
    pub fn from_options(options: RuntimeOptions) -> Self {
        Self(Arc::new(RwLock::new(options)))
    }

    /// Returns a snapshot of the current runtime options.
    ///
    /// If the lock is poisoned (a thread panicked while holding it), this
    /// recovers the inner data via [`std::sync::PoisonError::into_inner`] rather than
    /// propagating the panic.
    pub fn snapshot(&self) -> RuntimeOptions {
        self.0
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    /// Acquires a write guard, recovering from a poisoned lock if necessary.
    fn write_guard(&self) -> std::sync::RwLockWriteGuard<'_, RuntimeOptions> {
        self.0
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Sets the throughput control group name.
    pub fn set_throughput_control_group_name(&self, name: Option<ThroughputControlGroupName>) {
        self.write_guard().throughput_control_group_name = name;
    }

    /// Sets the dedicated gateway options.
    pub fn set_dedicated_gateway_options(&self, options: Option<DedicatedGatewayOptions>) {
        self.write_guard().dedicated_gateway_options = options;
    }

    /// Sets the diagnostics thresholds.
    pub fn set_diagnostics_thresholds(&self, thresholds: Option<DiagnosticsThresholds>) {
        self.write_guard().diagnostics_thresholds = thresholds;
    }

    /// Sets the end-to-end latency policy.
    pub fn set_end_to_end_latency_policy(&self, policy: Option<EndToEndOperationLatencyPolicy>) {
        self.write_guard().end_to_end_latency_policy = policy;
    }

    /// Sets the custom headers.
    ///
    /// Custom headers are best-effort: the caller may specify any header, but whether
    /// it is actually sent to the service is at the discretion of the library. There
    /// is no guarantee that a given custom header will be included in the outgoing request.
    pub fn set_custom_headers(&self, headers: Option<Headers>) {
        self.write_guard().custom_headers = headers;
    }

    /// Sets the excluded regions.
    pub fn set_excluded_regions(&self, regions: Option<ExcludedRegions>) {
        self.write_guard().excluded_regions = regions;
    }

    /// Sets the read consistency strategy.
    pub fn set_read_consistency_strategy(&self, strategy: Option<ReadConsistencyStrategy>) {
        self.write_guard().read_consistency_strategy = strategy;
    }

    /// Sets the content response on write setting.
    pub fn set_content_response_on_write(&self, value: Option<ContentResponseOnWrite>) {
        self.write_guard().content_response_on_write = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_runtime_options() {
        let options = RuntimeOptions::default();
        assert!(options.throughput_control_group_name.is_none());
        assert!(options.content_response_on_write.is_none());
    }

    #[test]
    fn builder_creates_options() {
        let options = RuntimeOptions::builder()
            .with_content_response_on_write(ContentResponseOnWrite::DISABLED)
            .build();

        assert_eq!(
            options.content_response_on_write,
            Some(ContentResponseOnWrite::DISABLED)
        );
    }

    #[test]
    fn to_builder_creates_modified_copy() {
        let original = RuntimeOptions::builder()
            .with_content_response_on_write(ContentResponseOnWrite::ENABLED)
            .with_read_consistency_strategy(ReadConsistencyStrategy::Eventual)
            .build();

        let modified = original
            .to_builder()
            .with_content_response_on_write(ContentResponseOnWrite::DISABLED)
            .build();

        // Modified value changed
        assert_eq!(
            modified.content_response_on_write,
            Some(ContentResponseOnWrite::DISABLED)
        );
        // Unmodified value preserved
        assert_eq!(
            modified.read_consistency_strategy,
            Some(ReadConsistencyStrategy::Eventual)
        );
        // Original unchanged
        assert_eq!(
            original.content_response_on_write,
            Some(ContentResponseOnWrite::ENABLED)
        );
    }

    #[test]
    fn merge_with_base() {
        let base = RuntimeOptions {
            content_response_on_write: Some(ContentResponseOnWrite::ENABLED),
            read_consistency_strategy: Some(ReadConsistencyStrategy::Eventual),
            ..Default::default()
        };

        let override_opts = RuntimeOptions {
            content_response_on_write: Some(ContentResponseOnWrite::DISABLED),
            ..Default::default()
        };

        let merged = override_opts.merge_with_base(&base);

        // Override takes precedence
        assert_eq!(
            merged.content_response_on_write,
            Some(ContentResponseOnWrite::DISABLED)
        );
        // Base value used when override is None
        assert_eq!(
            merged.read_consistency_strategy,
            Some(ReadConsistencyStrategy::Eventual)
        );
    }

    #[test]
    fn shared_runtime_options_snapshot() {
        let shared = SharedRuntimeOptions::new();

        // Initially empty
        assert!(shared.snapshot().content_response_on_write.is_none());

        // Modify
        shared.set_content_response_on_write(Some(ContentResponseOnWrite::ENABLED));

        // Verify change
        assert_eq!(
            shared.snapshot().content_response_on_write,
            Some(ContentResponseOnWrite::ENABLED)
        );
    }
}
