// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation options that participate in runtime/account/operation resolution.

use std::collections::HashMap;
use std::time::Duration;

use azure_core::http::headers::{HeaderName, HeaderValue};
use azure_data_cosmos_macros::CosmosOptions;

use crate::{
    models::ThroughputControlGroupName,
    options::{
        ContentResponseOnWrite, EndToEndOperationLatencyPolicy, ExcludedRegions,
        ReadConsistencyStrategy,
    },
};

/// Options that apply to individual Cosmos DB requests.
///
/// `OperationOptions` controls cross-cutting concerns such as consistency, routing,
/// retry behavior, and custom headers. These settings can be specified at multiple
/// levels — each per-operation options type (e.g., `ItemReadOptions`)
/// has an `operation` field of this type.
///
/// # Layered Resolution
///
/// When the same option is set at multiple levels, the most specific value wins:
///
/// 1. **Operation** — set on the per-request options (highest priority)
/// 2. **Account** — set on `CosmosClientOptions` when building the client
/// 3. **Runtime** — application-wide defaults
/// 4. **Environment** — loaded from `AZURE_COSMOS_*` environment variables (lowest priority)
///
/// A field set to `None` means "inherit from a lower-priority level."
/// A field set to `Some(value)` overrides all lower levels.
#[derive(CosmosOptions, Clone, Debug)]
#[options(layers(runtime, account, operation))]
#[non_exhaustive]
pub struct OperationOptions {
    /// Read consistency strategy for this request.
    ///
    /// Controls the consistency guarantee for read operations. Set to `None` to
    /// inherit the account or runtime default.
    #[option(env = "AZURE_COSMOS_READ_CONSISTENCY_STRATEGY")]
    pub read_consistency_strategy: Option<ReadConsistencyStrategy>,

    /// Regions to exclude from request routing.
    ///
    /// When set, the SDK will not route this request to the specified regions.
    /// Set to `Some(empty)` to clear exclusions; `None` inherits from a lower level.
    pub excluded_regions: Option<ExcludedRegions>,

    /// Whether write responses include the resource body.
    ///
    /// [`ContentResponseOnWrite::Enabled`] returns the written resource in the response.
    /// [`ContentResponseOnWrite::Disabled`] suppresses the body to reduce bandwidth.
    /// `None` inherits from a lower level (default: disabled).
    #[option(env = "AZURE_COSMOS_CONTENT_RESPONSE_ON_WRITE")]
    pub content_response_on_write: Option<ContentResponseOnWrite>,

    /// Throughput control group name for this request.
    ///
    /// References a group registered at runtime via
    /// [`CosmosDriverRuntimeBuilder::register_throughput_control_group()`](crate::driver::CosmosDriverRuntimeBuilder::register_throughput_control_group).
    ///
    /// `None` inherits from a lower-priority level or falls back to the
    /// container's default group.
    pub throughput_control_group: Option<ThroughputControlGroupName>,

    /// End-to-end timeout policy for this request.
    pub end_to_end_latency_policy: Option<EndToEndOperationLatencyPolicy>,

    /// Maximum number of region failover retries.
    #[option(env = "AZURE_COSMOS_MAX_FAILOVER_RETRY_COUNT")]
    pub max_failover_retry_count: Option<u32>,

    /// How long an endpoint is considered unavailable after a failure.
    pub endpoint_unavailability_ttl: Option<Duration>,

    /// Disables automatic session token management.
    ///
    /// When `None` or `Some(false)`, session tokens are captured from responses
    /// and sent on subsequent requests for session consistency.
    /// Set to `Some(true)` to disable this behavior.
    pub session_capturing_disabled: Option<bool>,

    /// Maximum number of session-consistency retries on 404/1002 errors.
    #[option(env = "AZURE_COSMOS_MAX_SESSION_RETRY_COUNT")]
    pub max_session_retry_count: Option<u32>,

    // Additional headers beyond those natively supported by the driver.
    // May be removed in the future as we analyze exactly what options are needed.
    custom_headers: Option<HashMap<HeaderName, HeaderValue>>,
}

impl OperationOptions {
    /// Sets additional headers to include in the request.
    pub fn with_custom_headers(mut self, headers: HashMap<HeaderName, HeaderValue>) -> Self {
        self.custom_headers = Some(headers);
        self
    }

    /// Gets the custom headers.
    pub fn custom_headers(&self) -> Option<&HashMap<HeaderName, HeaderValue>> {
        self.custom_headers.as_ref()
    }
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
        assert!(options.throughput_control_group.is_none());
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
}
