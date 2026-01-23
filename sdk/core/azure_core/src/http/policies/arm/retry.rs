// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! ARM-specific retry policy configuration and utilities.
//!
//! This module provides ARM-specific retry configurations with appropriate defaults
//! for Azure Resource Manager operations.

use crate::{
    http::{
        headers::HeaderName, ClientOptions, ExponentialRetryOptions, FixedRetryOptions,
        RetryOptions,
    },
    time::Duration,
};

/// ARM-specific retry headers to check for retry-after information.
///
/// This includes standard headers as well as ARM-specific rate limit headers.
pub const ARM_RETRY_HEADERS: &[HeaderName] = &[
    HeaderName::from_static("x-ms-ratelimit-remaining-subscription-writes"),
    HeaderName::from_static("x-ms-ratelimit-remaining-subscription-reads"),
    HeaderName::from_static("x-ms-ratelimit-remaining-tenant-writes"),
    HeaderName::from_static("x-ms-ratelimit-remaining-tenant-reads"),
    HeaderName::from_static("x-ms-ratelimit-remaining-subscription-resource-requests"),
    HeaderName::from_static("x-ms-ratelimit-remaining-subscription-resource-entities-read"),
    HeaderName::from_static("x-ms-retry-after-ms"),
    HeaderName::from_static("retry-after-ms"),
    typespec_client_core::http::headers::RETRY_AFTER,
];

/// ARM-specific exponential retry options with sensible defaults.
///
/// These defaults are tuned for Azure Resource Manager operations which may
/// take longer than typical data plane operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArmExponentialRetryOptions {
    /// The initial delay between retries.
    /// Default: 4 seconds
    pub initial_delay: Duration,

    /// Maximum number of retry attempts.
    /// Default: 8
    pub max_retries: u32,

    /// Maximum total elapsed time for retries.
    /// Default: 10 minutes
    pub max_total_elapsed: Duration,

    /// Maximum delay between individual retries.
    /// Default: 60 seconds
    pub max_delay: Duration,
}

impl Default for ArmExponentialRetryOptions {
    fn default() -> Self {
        Self {
            initial_delay: Duration::seconds(4),
            max_retries: 8,
            max_total_elapsed: Duration::minutes(10),
            max_delay: Duration::seconds(60),
        }
    }
}

impl From<ArmExponentialRetryOptions> for ExponentialRetryOptions {
    fn from(opts: ArmExponentialRetryOptions) -> Self {
        Self {
            initial_delay: opts.initial_delay,
            max_retries: opts.max_retries,
            max_total_elapsed: opts.max_total_elapsed,
            max_delay: opts.max_delay,
        }
    }
}

/// ARM-specific fixed retry options with sensible defaults.
///
/// These defaults are tuned for Azure Resource Manager operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArmFixedRetryOptions {
    /// The delay between retries.
    /// Default: 10 seconds
    pub delay: Duration,

    /// Maximum number of retry attempts.
    /// Default: 5
    pub max_retries: u32,

    /// Maximum total elapsed time for retries.
    /// Default: 10 minutes
    pub max_total_elapsed: Duration,
}

impl Default for ArmFixedRetryOptions {
    fn default() -> Self {
        Self {
            delay: Duration::seconds(10),
            max_retries: 5,
            max_total_elapsed: Duration::minutes(10),
        }
    }
}

impl From<ArmFixedRetryOptions> for FixedRetryOptions {
    fn from(opts: ArmFixedRetryOptions) -> Self {
        Self {
            delay: opts.delay,
            max_retries: opts.max_retries,
            max_total_elapsed: opts.max_total_elapsed,
        }
    }
}

/// Configures `ClientOptions` with ARM-specific retry settings using exponential backoff.
///
/// This is the recommended retry strategy for ARM operations as it provides
/// exponential backoff with jitter, which helps avoid thundering herd problems.
///
/// # Example
///
/// ```rust
/// use azure_core::http::policies::arm::arm_exponential_retry_options;
/// use azure_core::http::ClientOptions;
///
/// let mut client_options = ClientOptions::default();
/// arm_exponential_retry_options(&mut client_options);
/// ```
pub fn arm_exponential_retry_options(client_options: &mut ClientOptions) {
    arm_exponential_retry_options_with(client_options, ArmExponentialRetryOptions::default())
}

/// Configures `ClientOptions` with ARM-specific retry settings using custom exponential backoff options.
///
/// # Example
///
/// ```rust
/// use azure_core::{http::policies::arm::{arm_exponential_retry_options_with, ArmExponentialRetryOptions}, time::Duration};
/// use azure_core::http::ClientOptions;
///
/// let mut client_options = ClientOptions::default();
/// let options = ArmExponentialRetryOptions {
///     initial_delay: Duration::seconds(2),
///     max_retries: 5,
///     ..Default::default()
/// };
/// arm_exponential_retry_options_with(&mut client_options, options);
/// ```
pub fn arm_exponential_retry_options_with(
    client_options: &mut ClientOptions,
    options: ArmExponentialRetryOptions,
) {
    client_options.retry = RetryOptions::exponential(options.into());
}

/// Configures `ClientOptions` with ARM-specific retry settings using fixed delay.
///
/// # Example
///
/// ```rust
/// use azure_core::http::policies::arm::arm_fixed_retry_options;
/// use azure_core::http::ClientOptions;
///
/// let mut client_options = ClientOptions::default();
/// arm_fixed_retry_options(&mut client_options);
/// ```
pub fn arm_fixed_retry_options(client_options: &mut ClientOptions) {
    arm_fixed_retry_options_with(client_options, ArmFixedRetryOptions::default())
}

/// Configures `ClientOptions` with ARM-specific retry settings using custom fixed delay options.
///
/// # Example
///
/// ```rust
/// use azure_core::{http::policies::arm::{arm_fixed_retry_options_with, ArmFixedRetryOptions}, time::Duration};
/// use azure_core::http::ClientOptions;
///
/// let mut client_options = ClientOptions::default();
/// let options = ArmFixedRetryOptions {
///     delay: Duration::seconds(5),
///     max_retries: 3,
///     ..Default::default()
/// };
/// arm_fixed_retry_options_with(&mut client_options, options);
/// ```
pub fn arm_fixed_retry_options_with(
    client_options: &mut ClientOptions,
    options: ArmFixedRetryOptions,
) {
    client_options.retry = RetryOptions::fixed(options.into());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arm_exponential_retry_options_default() {
        let options = ArmExponentialRetryOptions::default();
        assert_eq!(options.initial_delay, Duration::seconds(4));
        assert_eq!(options.max_retries, 8);
        assert_eq!(options.max_total_elapsed, Duration::minutes(10));
        assert_eq!(options.max_delay, Duration::seconds(60));
    }

    #[test]
    fn test_arm_fixed_retry_options_default() {
        let options = ArmFixedRetryOptions::default();
        assert_eq!(options.delay, Duration::seconds(10));
        assert_eq!(options.max_retries, 5);
        assert_eq!(options.max_total_elapsed, Duration::minutes(10));
    }

    #[test]
    fn test_arm_retry_headers_contains_standard() {
        // Verify we include the standard RETRY_AFTER header
        assert!(ARM_RETRY_HEADERS.contains(&typespec_client_core::http::headers::RETRY_AFTER));
    }

    #[test]
    fn test_arm_retry_headers_contains_arm_specific() {
        // Verify we include ARM-specific headers
        let x_ms_retry = HeaderName::from_static("x-ms-retry-after-ms");
        assert!(ARM_RETRY_HEADERS.contains(&x_ms_retry));
    }

    #[test]
    fn test_arm_exponential_retry_options_configuration() {
        let mut client_options = ClientOptions::default();
        arm_exponential_retry_options(&mut client_options);
        // Just verify we can configure without panicking
    }

    #[test]
    fn test_arm_fixed_retry_options_configuration() {
        let mut client_options = ClientOptions::default();
        arm_fixed_retry_options(&mut client_options);
        // Just verify we can configure without panicking
    }

    #[test]
    fn test_arm_exponential_retry_options_with_custom() {
        let mut client_options = ClientOptions::default();
        let options = ArmExponentialRetryOptions {
            initial_delay: Duration::seconds(2),
            max_retries: 5,
            max_total_elapsed: Duration::minutes(5),
            max_delay: Duration::seconds(30),
        };
        arm_exponential_retry_options_with(&mut client_options, options);
        // Just verify we can configure without panicking
    }

    #[test]
    fn test_arm_fixed_retry_options_with_custom() {
        let mut client_options = ClientOptions::default();
        let options = ArmFixedRetryOptions {
            delay: Duration::seconds(5),
            max_retries: 3,
            max_total_elapsed: Duration::minutes(5),
        };
        arm_fixed_retry_options_with(&mut client_options, options);
        // Just verify we can configure without panicking
    }
}
