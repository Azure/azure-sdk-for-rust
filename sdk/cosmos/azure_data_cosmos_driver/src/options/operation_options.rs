// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation options that participate in runtime/account/operation resolution.

use std::time::Duration;

use azure_data_cosmos_macros::CosmosOptions;

use std::collections::HashMap;

use azure_core::http::headers::{HeaderName, HeaderValue};

use crate::{
    models::ThroughputControlGroupName,
    options::{
        ContentResponseOnWrite, EndToEndOperationLatencyPolicy, ExcludedRegions,
        ReadConsistencyStrategy,
    },
};

/// Options that apply to individual service requests.
///
/// These options follow a hierarchy where operation-level settings override
/// account-level, which override runtime-level, which override environment defaults.
///
/// `OperationOptions` is the single option group for all operation-specific
/// configuration where application-wide or account-wide defaults make sense.
#[derive(CosmosOptions, Clone, Debug)]
#[options(layers(runtime, account, operation))]
#[non_exhaustive]
pub struct OperationOptions {
    // Shared runtime options (can be set at environment/driver/operation level)
    runtime: RuntimeOptions,

    // Operation-specific options (not shared with environment/driver)
    session_token: Option<SessionToken>,
    partition_key: Option<PartitionKey>,
    quota_info_enabled: Option<QuotaInfoEnabled>,
    priority_level: Option<PriorityLevel>,

    // Just read operations
    etag_condition: Option<Precondition>,

    // Just write operations
    triggers: Option<TriggerOptions>,

    // Only StoredProc executions
    script_logging_enabled: Option<ScriptLoggingEnabled>,

    // Additional headers beyond those natively supported by the driver.
    // May be removed in the future as we analyze exactly what options are needed.
    custom_headers: Option<HashMap<HeaderName, HeaderValue>>,
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
    /// Read consistency strategy for read operations.
    #[option(env = "AZURE_COSMOS_READ_CONSISTENCY_STRATEGY")]
    pub read_consistency_strategy: Option<ReadConsistencyStrategy>,

    /// Regions to exclude from routing.
    pub excluded_regions: Option<ExcludedRegions>,

    /// Content response on write setting.
    #[option(env = "AZURE_COSMOS_CONTENT_RESPONSE_ON_WRITE")]
    pub content_response_on_write: Option<ContentResponseOnWrite>,

    /// Throughput control group name for rate limiting.
    pub throughput_control_group_name: Option<ThroughputControlGroupName>,

    /// End-to-end latency policy for timeout management.
    pub end_to_end_latency_policy: Option<EndToEndOperationLatencyPolicy>,

    /// Maximum operation-level failover retries.
    #[option(env = "AZURE_COSMOS_MAX_FAILOVER_RETRY_COUNT")]
    pub max_failover_retry_count: Option<u32>,

    /// Endpoint unavailability TTL used by routing state.
    pub endpoint_unavailability_ttl: Option<Duration>,

    /// Whether session token capturing is disabled.
    ///
    /// When `None` or `Some(false)`, session tokens are captured and resolved
    /// from response headers for session consistency (the default behavior).
    /// Set to `Some(true)` to disable session token management for scenarios where
    /// session consistency is not needed.
    pub session_capturing_disabled: Option<bool>,

    /// Maximum operation-level session retries for 404/1002 errors.
    #[option(env = "AZURE_COSMOS_MAX_SESSION_RETRY_COUNT")]
    pub max_session_retry_count: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_operation_options() {
        let options = OperationOptions::default();
        assert!(options.read_consistency_strategy.is_none());
        assert!(options.excluded_regions.is_none());
        assert!(options.content_response_on_write.is_none());
        assert!(options.throughput_control_group_name.is_none());
        assert!(options.max_failover_retry_count.is_none());
        assert!(options.max_session_retry_count.is_none());
    }

    #[test]
    fn builder_creates_options() {
        let options = OperationOptionsBuilder::new()
            .with_content_response_on_write(ContentResponseOnWrite::Disabled)
            .with_read_consistency_strategy(ReadConsistencyStrategy::Session)
            .with_max_failover_retry_count(5)
            .with_max_session_retry_count(3)
            .build();

        assert_eq!(
            options.content_response_on_write,
            Some(ContentResponseOnWrite::Disabled)
        );
        assert_eq!(
            options.read_consistency_strategy,
            Some(ReadConsistencyStrategy::Session)
        );
        assert_eq!(options.max_failover_retry_count, Some(5));
        assert_eq!(options.max_session_retry_count, Some(3));
    }

    #[test]
    fn view_resolves_across_layers() {
        use std::sync::Arc;

        let env = Arc::new(OperationOptions {
            read_consistency_strategy: Some(ReadConsistencyStrategy::Eventual),
            max_failover_retry_count: Some(3),
            ..Default::default()
        });

        let runtime = Arc::new(OperationOptions {
            content_response_on_write: Some(ContentResponseOnWrite::Enabled),
            ..Default::default()
        });

        let account = Arc::new(OperationOptions {
            max_failover_retry_count: Some(5),
            content_response_on_write: Some(ContentResponseOnWrite::Disabled),
            ..Default::default()
        });

        let operation = OperationOptions {
            read_consistency_strategy: Some(ReadConsistencyStrategy::Session),
            ..Default::default()
        };

        let view =
            OperationOptionsView::new(Some(env), Some(runtime), Some(account), Some(&operation));

        // Operation overrides env
        assert_eq!(
            view.read_consistency_strategy(),
            Some(&ReadConsistencyStrategy::Session)
        );
        // Account overrides runtime
        assert_eq!(
            view.content_response_on_write(),
            Some(&ContentResponseOnWrite::Disabled)
        );
        // Account overrides env
        assert_eq!(view.max_failover_retry_count(), Some(&5));
        // Not set anywhere
        assert!(view.excluded_regions().is_none());
        assert!(view.max_session_retry_count().is_none());
    }

    #[test]
    fn from_env_vars_parses_known_vars() {
        let options = OperationOptions::from_env_vars(|key| match key {
            "AZURE_COSMOS_READ_CONSISTENCY_STRATEGY" => Ok("Session".to_string()),
            "AZURE_COSMOS_CONTENT_RESPONSE_ON_WRITE" => Ok("true".to_string()),
            "AZURE_COSMOS_MAX_FAILOVER_RETRY_COUNT" => Ok("7".to_string()),
            "AZURE_COSMOS_MAX_SESSION_RETRY_COUNT" => Ok("3".to_string()),
            _ => Err(std::env::VarError::NotPresent),
        });

        assert_eq!(
            options.read_consistency_strategy,
            Some(ReadConsistencyStrategy::Session)
        );
        assert_eq!(
            options.content_response_on_write,
            Some(ContentResponseOnWrite::Enabled)
        );
        assert_eq!(options.max_failover_retry_count, Some(7));
        assert_eq!(options.max_session_retry_count, Some(3));
        // Fields without env annotation remain None
        assert!(options.excluded_regions.is_none());
    }

    #[test]
    fn from_env_vars_returns_none_for_missing_vars() {
        let options = OperationOptions::from_env_vars(|_| Err(std::env::VarError::NotPresent));

        assert!(options.read_consistency_strategy.is_none());
        assert!(options.content_response_on_write.is_none());
        assert!(options.excluded_regions.is_none());
        assert!(options.max_failover_retry_count.is_none());
        assert!(options.max_session_retry_count.is_none());
    }

    /// Sets additional headers to include in the request.
    pub fn with_custom_headers(mut self, headers: HashMap<HeaderName, HeaderValue>) -> Self {
        self.custom_headers = Some(headers);
        self
    }

    /// Gets the custom headers.
    pub fn custom_headers_ref(&self) -> Option<&HashMap<HeaderName, HeaderValue>> {
        self.custom_headers.as_ref()
    }
}
