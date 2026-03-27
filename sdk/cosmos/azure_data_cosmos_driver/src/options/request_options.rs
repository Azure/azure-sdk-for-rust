// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Request options for Cosmos DB operations.

use crate::{
    models::ThroughputControlGroupName,
    options::{
        ContentResponseOnWrite, EndToEndOperationLatencyPolicy, ExcludedRegions, OperationOptions,
        ReadConsistencyStrategy, RuntimeOptions,
    },
};

/// Options that can be applied to Cosmos DB operations.
///
/// This struct provides a fluent builder interface for configuring request options
/// such as consistency levels, session tokens, triggers, and other policies.
///
/// # Runtime Options
///
/// Many settings (like `throughput_control_group_name`, `dedicated_gateway_options`, etc.)
/// are shared with `EnvironmentOptions` and `DriverOptions` via [`RuntimeOptions`].
/// Operation-level settings override driver-level, which override environment-level defaults.
///
/// # Example
///
/// ```rust,no_run
/// use azure_data_cosmos_driver::options::{RequestOptions, ContentResponseOnWrite};
///
/// let options = RequestOptions::new()
///     .with_content_response_on_write(ContentResponseOnWrite::Disabled);
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct RequestOptions {
    // Shared runtime options (can be set at environment/driver/operation level)
    runtime: RuntimeOptions,
    // Operation options (read_consistency, excluded_regions, content_response_on_write)
    operation: OperationOptions,
}

impl RequestOptions {
    /// Creates a new empty `RequestOptions`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the embedded runtime options.
    ///
    /// These are the options shared with environment and driver levels.
    pub fn runtime(&self) -> &RuntimeOptions {
        &self.runtime
    }

    /// Returns a mutable reference to the embedded runtime options.
    pub fn runtime_mut(&mut self) -> &mut RuntimeOptions {
        &mut self.runtime
    }

    /// Returns the embedded operation options.
    pub fn operation(&self) -> &OperationOptions {
        &self.operation
    }

    /// Sets the read consistency strategy for this operation.
    pub fn with_read_consistency_strategy(mut self, strategy: ReadConsistencyStrategy) -> Self {
        self.operation.read_consistency_strategy = Some(strategy);
        self
    }

    /// Gets the read consistency strategy.
    pub fn read_consistency_strategy_ref(&self) -> Option<&ReadConsistencyStrategy> {
        self.operation.read_consistency_strategy.as_ref()
    }

    /// Sets whether the response should include the content after write operations.
    pub fn with_content_response_on_write(mut self, value: ContentResponseOnWrite) -> Self {
        self.operation.content_response_on_write = Some(value);
        self
    }

    /// Gets the content response on write setting.
    pub fn content_response_on_write_ref(&self) -> Option<&ContentResponseOnWrite> {
        self.operation.content_response_on_write.as_ref()
    }

    /// Sets the throughput control group name for this operation.
    pub fn with_throughput_control_group_name(mut self, name: ThroughputControlGroupName) -> Self {
        self.runtime.throughput_control_group_name = Some(name);
        self
    }

    /// Gets the throughput control group name.
    pub fn throughput_control_group_name_ref(&self) -> Option<&ThroughputControlGroupName> {
        self.runtime.throughput_control_group_name.as_ref()
    }

    /// Sets the end-to-end operation latency policy.
    pub fn with_end_to_end_latency_policy(
        mut self,
        policy: EndToEndOperationLatencyPolicy,
    ) -> Self {
        self.runtime.end_to_end_latency_policy = Some(policy);
        self
    }

    /// Gets the end-to-end operation latency policy.
    pub fn end_to_end_latency_policy_ref(&self) -> Option<&EndToEndOperationLatencyPolicy> {
        self.runtime.end_to_end_latency_policy.as_ref()
    }

    /// Sets the regions to exclude from routing.
    pub fn with_excluded_regions(mut self, regions: ExcludedRegions) -> Self {
        self.operation.excluded_regions = Some(regions);
        self
    }

    /// Gets the excluded regions.
    pub fn excluded_regions_ref(&self) -> Option<&ExcludedRegions> {
        self.operation.excluded_regions.as_ref()
    }
}
