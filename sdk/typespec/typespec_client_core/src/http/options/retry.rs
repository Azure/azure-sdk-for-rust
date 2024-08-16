// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::policies::{
    ExponentialRetryPolicy, FixedRetryPolicy, NoRetryPolicy, Policy, RetryPolicy,
};
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;

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
            RetryMode::Custom(_) => write!(f, "Custom"),
            RetryMode::None => write!(f, "None"),
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
}

impl RetryOptions {
    /// A retry strategy where attempts happen at intervals that get exponentially longer with each retry.
    pub fn exponential(options: ExponentialRetryOptions) -> Self {
        Self {
            mode: RetryMode::Exponential(options),
        }
    }

    /// A retry strategy where attempts happen at fixed intervals; each delay is a consistent duration.
    pub fn fixed(options: FixedRetryOptions) -> Self {
        Self {
            mode: RetryMode::Fixed(options),
        }
    }

    /// A custom retry using the supplied retry policy.
    pub fn custom<T: RetryPolicy + 'static>(policy: Arc<T>) -> Self {
        Self {
            mode: RetryMode::Custom(policy),
        }
    }

    /// No retries will be attempted.
    pub fn none() -> Self {
        Self {
            mode: RetryMode::None,
        }
    }

    pub(crate) fn to_policy(&self) -> Arc<dyn Policy> {
        match &self.mode {
            RetryMode::Exponential(options) => Arc::new(ExponentialRetryPolicy::new(
                options.initial_delay,
                options.max_retries,
                options.max_total_elapsed,
                options.max_delay,
            )),
            RetryMode::Fixed(options) => Arc::new(FixedRetryPolicy::new(
                options.delay,
                options.max_retries,
                options.max_total_elapsed,
            )),
            RetryMode::Custom(c) => c.clone(),
            RetryMode::None => Arc::new(NoRetryPolicy::default()),
        }
    }
}

/// Options for how an exponential retry strategy should behave.
///
/// # Example
///
/// Configuring retry to be exponential with 10 retries max and an initial delay of 1 second.
/// ```
/// # use core::time::Duration;
/// # use typespec_client_core::http::{ExponentialRetryOptions, RetryOptions};
/// RetryOptions::exponential(
///    ExponentialRetryOptions::default()
///        .max_retries(10u32)
///        .initial_delay(Duration::from_secs(1)),
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

impl ExponentialRetryOptions {
    setters! {
        initial_delay: Duration => initial_delay,
        max_retries: u32 => max_retries,
        max_total_elapsed: Duration => max_total_elapsed,
        max_delay: Duration => max_delay,
    }
}

impl Default for ExponentialRetryOptions {
    fn default() -> Self {
        Self {
            initial_delay: Duration::from_millis(200),
            max_retries: 8,
            max_total_elapsed: Duration::from_secs(60),
            max_delay: Duration::from_secs(30),
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
///    FixedRetryOptions::default()
///        .max_retries(10u32)
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

impl FixedRetryOptions {
    setters! {
        #[doc = "Set the delay between retry attempts."]
        delay: Duration => delay,
        #[doc = "Set the maximum number of retry attempts before giving up."]
        max_retries: u32 => max_retries,
        #[doc = "Set the maximum permissible elapsed time since starting to retry."]
        max_total_elapsed: Duration => max_total_elapsed,
    }
}

impl Default for FixedRetryOptions {
    fn default() -> Self {
        Self {
            delay: Duration::from_millis(200),
            max_retries: 8,
            max_total_elapsed: Duration::from_secs(60),
        }
    }
}
