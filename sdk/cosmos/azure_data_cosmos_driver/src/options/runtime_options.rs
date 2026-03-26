// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Runtime-configurable options shared across environment, driver, and operation levels.

use std::time::Duration;

use azure_data_cosmos_macros::CosmosOptions;

use crate::{models::ThroughputControlGroupName, options::EndToEndOperationLatencyPolicy};

/// Runtime-configurable options that can be set at environment, driver, or operation level.
///
/// These options follow a hierarchy where operation-level settings override driver-level,
/// which in turn override runtime-level, which override environment-level defaults.
///
/// The `#[derive(CosmosOptions)]` macro generates:
/// - [`RuntimeOptionsView`] — snapshot view for resolving across layers
/// - [`RuntimeOptionsBuilder`] — fluent builder for constructing options
/// - `Default` — all fields `None`
/// - `from_env()` / `from_env_vars()` — environment variable loading
#[derive(CosmosOptions, Clone, Debug)]
#[options(layers(runtime, account, operation))]
#[non_exhaustive]
pub struct RuntimeOptions {
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
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_runtime_options() {
        let options = RuntimeOptions::default();
        assert!(options.throughput_control_group_name.is_none());
        assert!(options.max_failover_retry_count.is_none());
    }

    #[test]
    fn builder_creates_options() {
        let options = RuntimeOptionsBuilder::new()
            .with_max_failover_retry_count(5)
            .build();

        assert_eq!(options.max_failover_retry_count, Some(5));
    }

    #[test]
    fn view_resolves_across_layers() {
        use std::sync::Arc;

        let env = Arc::new(RuntimeOptions {
            max_failover_retry_count: Some(3),
            ..Default::default()
        });

        let runtime = Arc::new(RuntimeOptions {
            session_capturing_disabled: Some(true),
            ..Default::default()
        });

        let account = Arc::new(RuntimeOptions {
            max_failover_retry_count: Some(5),
            ..Default::default()
        });

        let operation = RuntimeOptions {
            session_capturing_disabled: Some(false),
            ..Default::default()
        };

        let view =
            RuntimeOptionsView::new(Some(env), Some(runtime), Some(account), Some(&operation));

        // Account overrides env
        assert_eq!(view.max_failover_retry_count(), Some(&5));
        // Operation overrides runtime
        assert_eq!(view.session_capturing_disabled(), Some(&false));
        // Not set anywhere
        assert!(view.throughput_control_group_name().is_none());
    }

    #[test]
    fn from_env_vars_parses_known_vars() {
        let options = RuntimeOptions::from_env_vars(|key| match key {
            "AZURE_COSMOS_MAX_FAILOVER_RETRY_COUNT" => Ok("7".to_string()),
            _ => Err(std::env::VarError::NotPresent),
        });

        assert_eq!(options.max_failover_retry_count, Some(7));
        // Fields without env annotation remain None
        assert!(options.throughput_control_group_name.is_none());
    }

    #[test]
    fn from_env_vars_returns_none_for_missing_vars() {
        let options = RuntimeOptions::from_env_vars(|_| Err(std::env::VarError::NotPresent));

        assert!(options.max_failover_retry_count.is_none());
    }
}
