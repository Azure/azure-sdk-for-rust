// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    http::{
        headers::{HeaderName, RETRY_AFTER},
        policies::{
            ExponentialRetryPolicy, FixedRetryPolicy, NoRetryPolicy, Policy, RetryHeaders,
            RetryPolicy,
        },
    },
    time::Duration,
};
use std::fmt::Debug;
use std::sync::Arc;

/// The algorithm to apply when calculating the delay between retry attempts.
#[derive(Clone)]
enum RetryMode {
    /// Retry attempts will delay based on a back-off strategy,
    /// where each attempt will increase the duration that it waits before retrying.
    ///
    /// This is the default.
    Exponential(ExponentialRetryOptions),

    /// Retry attempts happen at fixed intervals; each delay is a consistent duration.
    Fixed(FixedRetryOptions),

    /// A custom retry policy
    Custom(Arc<dyn Policy>),

    /// Do not retry attempts.
    None,
}

impl Debug for RetryMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RetryMode::Exponential(o) => write!(f, "Exponential({o:?})"),
            RetryMode::Fixed(o) => write!(f, "Fixed({o:?})"),
            RetryMode::Custom(_) => f.write_str("Custom"),
            RetryMode::None => f.write_str("None"),
        }
    }
}

impl Default for RetryMode {
    fn default() -> Self {
        RetryMode::Exponential(ExponentialRetryOptions::default())
    }
}

/// Specify how retries should behave.
///
/// Note that not all requests can be retried. These options will only be used
/// when a retry is attempted.
///
/// The default is an exponential retry policy using the default `ExponentialRetryOptions`.
#[derive(Clone, Debug, Default)]
pub struct RetryOptions {
    /// The algorithm to use for calculating retry delays.
    mode: RetryMode,
    retry_headers: RetryHeaders,
}

impl RetryOptions {
    /// A retry strategy where attempts happen at intervals that get exponentially longer with each retry.
    pub fn exponential(options: ExponentialRetryOptions) -> Self {
        Self {
            mode: RetryMode::Exponential(options),
            retry_headers: RetryHeaders {
                retry_headers: vec![RETRY_AFTER],
                error_header: None,
            },
        }
    }

    /// A retry strategy where attempts happen at fixed intervals; each delay is a consistent duration.
    pub fn fixed(options: FixedRetryOptions) -> Self {
        Self {
            mode: RetryMode::Fixed(options),
            retry_headers: RetryHeaders {
                retry_headers: vec![RETRY_AFTER],
                error_header: None,
            },
        }
    }

    /// A custom retry using the supplied retry policy.
    pub fn custom<T: RetryPolicy + 'static>(policy: Arc<T>) -> Self {
        Self {
            mode: RetryMode::Custom(policy),
            retry_headers: RetryHeaders::default(),
        }
    }

    /// No retries will be attempted.
    pub fn none() -> Self {
        Self {
            mode: RetryMode::None,
            retry_headers: RetryHeaders::default(),
        }
    }

    /// Defines a set of HTTP headers which, if present on a response,
    /// indicate that the response should be retried after a delay.
    /// The boolean indicates whether the header value is a number of seconds to wait.
    /// If true, the header value is a header which conforms to the `Retry-After` HTTP header specification.
    /// If false, the header value is a number of milliseconds to wait.
    ///
    /// # Arguments
    /// * `headers` - A list of HTTP headers to check for retry information.
    ///
    pub fn with_retry_after_headers(
        mut self,
        headers: &[HeaderName],
        error_header: Option<HeaderName>,
    ) -> Self {
        self.retry_headers.retry_headers = headers.to_vec();
        self.retry_headers.error_header = error_header;
        self
    }

    pub(crate) fn to_policy(&self) -> Arc<dyn Policy> {
        match &self.mode {
            RetryMode::Exponential(options) => Arc::new(ExponentialRetryPolicy::new(
                options.initial_delay,
                options.max_retries,
                options.max_total_elapsed,
                options.max_delay,
                self.retry_headers.clone(),
            )),
            RetryMode::Fixed(options) => Arc::new(FixedRetryPolicy::new(
                options.delay,
                options.max_retries,
                options.max_total_elapsed,
                self.retry_headers.clone(),
            )),
            RetryMode::Custom(c) => c.clone(),
            RetryMode::None => Arc::new(NoRetryPolicy::new(self.retry_headers.clone())),
        }
    }
}

/// Options for how an exponential retry strategy should behave.
///
/// # Example
///
/// Configuring retry to be exponential with 10 retries max and an initial delay of 1 second.
/// ```
/// # use typespec_client_core::time::Duration;
/// # use typespec_client_core::http::{ExponentialRetryOptions, RetryOptions};
/// RetryOptions::exponential(
///     ExponentialRetryOptions {
///         max_retries: 10u32,
///         initial_delay: Duration::seconds(1),
///         ..Default::default()
///     }
/// );
/// ```
#[derive(Clone, Debug)]
pub struct ExponentialRetryOptions {
    /// The initial delay between retry attempts. The delay will increase with each retry.
    ///
    /// The default is 200 milliseconds.
    pub initial_delay: Duration,

    /// The maximum number of retry attempts before giving up.
    ///
    /// The default is 8.
    pub max_retries: u32,

    /// The maximum permissible elapsed time since starting to retry before giving up.
    ///
    /// The default is 1 minute.
    pub max_total_elapsed: Duration,

    /// The maximum permissible time between retries.
    ///
    /// The default is 30 seconds. For SRE reasons, this is only respected when above 1 second.
    pub max_delay: Duration,
}

impl Default for ExponentialRetryOptions {
    fn default() -> Self {
        Self {
            initial_delay: Duration::milliseconds(200),
            max_retries: 8,
            max_total_elapsed: Duration::seconds(60),
            max_delay: Duration::seconds(30),
        }
    }
}

/// Options for how a fixed retry strategy should behave.
///
/// # Example
///
/// Configuring retry to be fixed with 10 retries max.
/// ```
/// # use typespec_client_core::http::{FixedRetryOptions, RetryOptions};
/// RetryOptions::fixed(
///     FixedRetryOptions {
///         max_retries: 10u32,
///         ..Default::default()
///     }
/// );
/// ```
#[derive(Clone, Debug)]
pub struct FixedRetryOptions {
    /// The delay between retry attempts.
    ///
    /// The default is 200 milliseconds.
    pub delay: Duration,

    /// The maximum number of retry attempts before giving up.
    ///
    /// The default is 8.
    pub max_retries: u32,

    /// The maximum permissible elapsed time since starting to retry.
    ///
    /// The default is 1 minute.
    pub max_total_elapsed: Duration,
}

impl Default for FixedRetryOptions {
    fn default() -> Self {
        Self {
            delay: Duration::milliseconds(200),
            max_retries: 8,
            max_total_elapsed: Duration::seconds(60),
        }
    }
}
