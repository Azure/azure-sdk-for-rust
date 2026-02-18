// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Request options for Cosmos DB operations.

use azure_core::http::headers::Headers;

use crate::{
    models::{ETagCondition, PartitionKey, SessionToken, ThroughputControlGroupName},
    options::{
        ContentResponseOnWrite, DedicatedGatewayOptions, DiagnosticsThresholds,
        EndToEndOperationLatencyPolicy, ExcludedRegions, PriorityLevel, QuotaInfoEnabled,
        ReadConsistencyStrategy, RuntimeOptions, ScriptLoggingEnabled, TriggerOptions,
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
/// use azure_data_cosmos_driver::options::{OperationOptions, PriorityLevel, ContentResponseOnWrite};
/// use azure_data_cosmos_driver::models::PartitionKey;
///
/// let options = OperationOptions::new()
///     .with_partition_key(PartitionKey::from("my-partition"))
///     .with_priority_level(PriorityLevel::Low)
///     .with_content_response_on_write(ContentResponseOnWrite::DISABLED);
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct OperationOptions {
    // Shared runtime options (can be set at environment/driver/operation level)
    runtime: RuntimeOptions,

    // Operation-specific options (not shared with environment/driver)
    session_token: Option<SessionToken>,
    partition_key: Option<PartitionKey>,
    quota_info_enabled: Option<QuotaInfoEnabled>,
    priority_level: Option<PriorityLevel>,

    // Just read operations
    etag_condition: Option<ETagCondition>,

    // Just write operations
    triggers: Option<TriggerOptions>,

    // Only StoredProc executions
    script_logging_enabled: Option<ScriptLoggingEnabled>,
}

impl OperationOptions {
    /// Creates a new empty `OperationOptions`.
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

    /// Creates effective runtime options by merging with a base.
    ///
    /// Operation-level settings take precedence over the base settings.
    pub fn effective_runtime(&self, base: &RuntimeOptions) -> RuntimeOptions {
        self.runtime.merge_with_base(base)
    }

    /// Sets the trigger options for this operation.
    pub fn with_triggers(mut self, triggers: TriggerOptions) -> Self {
        self.triggers = Some(triggers);
        self
    }

    /// Gets the trigger options.
    pub fn triggers_ref(&self) -> Option<&TriggerOptions> {
        self.triggers.as_ref()
    }

    /// Sets the read consistency strategy for this operation.
    pub fn with_read_consistency_strategy(mut self, strategy: ReadConsistencyStrategy) -> Self {
        self.runtime.read_consistency_strategy = Some(strategy);
        self
    }

    /// Gets the read consistency strategy.
    pub fn read_consistency_strategy_ref(&self) -> Option<&ReadConsistencyStrategy> {
        self.runtime.read_consistency_strategy.as_ref()
    }

    /// Sets the session token for session consistency.
    pub fn with_session_token(mut self, token: SessionToken) -> Self {
        self.session_token = Some(token);
        self
    }

    /// Gets the session token.
    pub fn session_token_ref(&self) -> Option<&SessionToken> {
        self.session_token.as_ref()
    }

    /// Sets the ETag condition for optimistic concurrency.
    pub(crate) fn with_etag_condition(mut self, condition: ETagCondition) -> Self {
        self.etag_condition = Some(condition);
        self
    }

    /// Gets the ETag condition.
    pub(crate) fn etag_condition_ref(&self) -> Option<&ETagCondition> {
        self.etag_condition.as_ref()
    }

    /// Sets the partition key for this operation.
    pub fn with_partition_key(mut self, key: PartitionKey) -> Self {
        self.partition_key = Some(key);
        self
    }

    /// Gets the partition key.
    pub fn partition_key_ref(&self) -> Option<&PartitionKey> {
        self.partition_key.as_ref()
    }

    /// Sets whether the response should include the content after write operations.
    pub fn with_content_response_on_write(mut self, value: ContentResponseOnWrite) -> Self {
        self.runtime.content_response_on_write = Some(value);
        self
    }

    /// Gets the content response on write setting.
    pub fn content_response_on_write_ref(&self) -> Option<&ContentResponseOnWrite> {
        self.runtime.content_response_on_write.as_ref()
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

    /// Sets the dedicated gateway options for integrated cache.
    pub fn with_dedicated_gateway_options(mut self, options: DedicatedGatewayOptions) -> Self {
        self.runtime.dedicated_gateway_options = Some(options);
        self
    }

    /// Gets the dedicated gateway options.
    pub fn dedicated_gateway_options_ref(&self) -> Option<&DedicatedGatewayOptions> {
        self.runtime.dedicated_gateway_options.as_ref()
    }

    /// Sets the diagnostics thresholds for this operation.
    pub fn with_diagnostics_thresholds(mut self, thresholds: DiagnosticsThresholds) -> Self {
        self.runtime.diagnostics_thresholds = Some(thresholds);
        self
    }

    /// Gets the diagnostics thresholds.
    pub fn diagnostics_thresholds_ref(&self) -> Option<&DiagnosticsThresholds> {
        self.runtime.diagnostics_thresholds.as_ref()
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
        self.runtime.excluded_regions = Some(regions);
        self
    }

    /// Gets the excluded regions.
    pub fn excluded_regions_ref(&self) -> Option<&ExcludedRegions> {
        self.runtime.excluded_regions.as_ref()
    }

    /// Sets the priority level for this operation.
    pub fn with_priority_level(mut self, level: PriorityLevel) -> Self {
        self.priority_level = Some(level);
        self
    }

    /// Gets the priority level.
    pub fn priority_level_ref(&self) -> Option<&PriorityLevel> {
        self.priority_level.as_ref()
    }

    /// Sets whether script logging is enabled.
    pub fn with_script_logging_enabled(mut self, value: ScriptLoggingEnabled) -> Self {
        self.script_logging_enabled = Some(value);
        self
    }

    /// Gets the script logging enabled setting.
    pub fn script_logging_enabled_ref(&self) -> Option<&ScriptLoggingEnabled> {
        self.script_logging_enabled.as_ref()
    }

    /// Sets whether quota info is included in responses.
    pub fn with_quota_info_enabled(mut self, value: QuotaInfoEnabled) -> Self {
        self.quota_info_enabled = Some(value);
        self
    }

    /// Gets the quota info enabled setting.
    pub fn quota_info_enabled_ref(&self) -> Option<&QuotaInfoEnabled> {
        self.quota_info_enabled.as_ref()
    }

    /// Sets custom HTTP headers to include in the request.
    ///
    /// Custom headers are best-effort: the caller may specify any header, but whether
    /// it is actually sent to the service is at the discretion of the library. There
    /// is no guarantee that a given custom header will be included in the outgoing request.
    pub fn with_custom_headers(mut self, headers: Headers) -> Self {
        self.runtime.custom_headers = Some(headers);
        self
    }

    /// Gets the custom headers.
    pub fn custom_headers_ref(&self) -> Option<&Headers> {
        self.runtime.custom_headers.as_ref()
    }
}
