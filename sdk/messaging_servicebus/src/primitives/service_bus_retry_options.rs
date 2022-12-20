//! Defines the [`ServiceBusRetryOptions`] struct.

use std::time::Duration;

use super::service_bus_retry_mode::ServiceBusRetryMode;

const RETRIES_MAX: u32 = 100;
const DELAY_MIN: Duration = Duration::from_millis(1);
const DELAY_MAX: Duration = Duration::from_secs(5 * 60);
const TRY_TIMEOUT_MIN: Duration = Duration::ZERO;
const TRY_TIMEOUT_MAX: Duration = Duration::from_secs(60 * 60); // 1 Hour

/// The error type returned when a retry option value is out of range.
#[derive(Debug, thiserror::Error)]
pub enum OutOfRange<T> {
    /// The value is less than the minimum allowed value.
    #[error("The value {value} is less than the minimum allowed value {minimum_allowed}.")]
    LessThanAllowed {
        /// The value that was provided.
        value: T,
        /// The minimum allowed value.
        minimum_allowed: T,
    },
    /// The value is greater than the maximum allowed value.
    #[error("The value {value} is greater than the maximum allowed value {maximum_allowed}.")]
    GreaterThanAllowed {
        /// The value that was provided.
        value: T,
        /// The maximum allowed value.
        maximum_allowed: T,
    },
}

/// The set of options that can be specified to influence how
/// retry attempts are made, and a failure is eligible to be retried.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceBusRetryOptions {
    /// The approach to use for calculating retry delays.
    pub mode: ServiceBusRetryMode,

    /// The maximum number of retry attempts before considering the associated operation to have
    /// failed. Default to [`ServiceBusRetryOptions::DEFAULT_MAX_RETRIES`]
    pub max_retries: u32,

    /// The delay or backoff factor to apply between retry attempts. Default to
    /// [`ServiceBusRetryOptions::DEFAULT_DELAY`]
    pub delay: Duration,

    /// The maximum delay to allow between retry attempts. Default to
    /// [`ServiceBusRetryOptions::DEFAULT_MAX_DELAY`]
    pub max_delay: Duration,

    /// The maximum duration to wait for an operation, per attempt. Default to
    /// [`ServiceBusRetryOptions::DEFAULT_TRY_TIMEOUT`]
    pub try_timeout: Duration,
}

impl Default for ServiceBusRetryOptions {
    fn default() -> Self {
        Self {
            mode: ServiceBusRetryMode::default(),
            max_retries: Self::DEFAULT_MAX_RETRIES,
            delay: Self::DEFAULT_DELAY,
            max_delay: Self::DEFAULT_MAX_DELAY,
            try_timeout: Self::DEFAULT_TRY_TIMEOUT,
        }
    }
}

impl ServiceBusRetryOptions {
    /// Default value for [`ServiceBusRetryOptions::max_retries`].
    pub const DEFAULT_MAX_RETRIES: u32 = 3;

    /// Default value for [`ServiceBusRetryOptions::delay`].
    pub const DEFAULT_DELAY: Duration = Duration::from_millis(800);

    /// Default value for [`ServiceBusRetryOptions::max_delay`].
    pub const DEFAULT_MAX_DELAY: Duration = Duration::from_secs(60);

    /// Default value for [`ServiceBusRetryOptions::try_timeout`].
    pub const DEFAULT_TRY_TIMEOUT: Duration = Duration::from_secs(60);

    /// The approach to use for calculating retry delays.
    pub fn mode(&self) -> &ServiceBusRetryMode {
        &self.mode
    }

    /// The approach to use for calculating retry delays.
    pub fn set_mode(&mut self, mode: ServiceBusRetryMode) {
        self.mode = mode;
    }

    /// Gets the maximum number of retry attempts before considering the associated operation to have
    /// failed.
    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }

    /// Sets the maximum number of retry attempts before considering the associated operation to have
    /// failed. Returns `Err(OutOfRange)` when the requested number of retries not between 0 and 100
    /// (inclusive).
    pub fn set_max_retries(&mut self, value: u32) -> Result<(), OutOfRange<u32>> {
        if value > RETRIES_MAX {
            Err(OutOfRange::GreaterThanAllowed {
                value,
                maximum_allowed: RETRIES_MAX,
            })
        } else {
            self.max_retries = value;
            Ok(())
        }
    }

    /// Gets the delay between retry attempts for a fixed approach or the delay on which to base
    /// calculations for a backoff-based approach.
    pub fn delay(&self) -> &Duration {
        &self.delay
    }

    /// Sets the delay between retry attempts for a fixed approach or the delay on which to base
    /// calculations for a backoff-based approach. Returns `Err(OutOfRange)` when the requested
    /// delay is not between 1 millisecond and 5 minutes (inclusive).
    pub fn set_delay(&mut self, value: Duration) -> Result<(), OutOfRange<Duration>> {
        if value < DELAY_MIN {
            Err(OutOfRange::LessThanAllowed {
                value,
                minimum_allowed: DELAY_MIN,
            })
        } else if value > DELAY_MAX {
            Err(OutOfRange::GreaterThanAllowed {
                value,
                maximum_allowed: DELAY_MAX,
            })
        } else {
            self.delay = value;
            Ok(())
        }
    }

    /// Gets the maximum permissible delay between retry attempts.
    pub fn max_delay(&self) -> &Duration {
        &self.max_delay
    }

    /// Sets the maximum permissible delay between retry attempts.
    pub fn set_max_delay(&mut self, value: Duration) {
        // `std::time::Duration` is already non-negative
        self.max_delay = value;
    }

    /// Gets the maximum duration to wait for completion of a single attempt, whether the initial attempt
    /// or a retry.
    pub fn try_timeout(&self) -> &Duration {
        &self.try_timeout
    }

    /// Sets the maximum duration to wait for completion of a single attempt, whether the initial
    /// attempt or a retry. Returns `Err(OutOfRange)`  when the requested timeout is not between
    /// [`Duration::ZERO`] and 1 hour (inclusive)
    pub fn set_try_timeout(&mut self, value: Duration) -> Result<(), OutOfRange<Duration>> {
        if value < TRY_TIMEOUT_MIN {
            Err(OutOfRange::LessThanAllowed {
                value,
                minimum_allowed: TRY_TIMEOUT_MIN,
            })
        } else if value > TRY_TIMEOUT_MAX {
            Err(OutOfRange::GreaterThanAllowed {
                value,
                maximum_allowed: TRY_TIMEOUT_MAX,
            })
        } else {
            self.try_timeout = value;
            Ok(())
        }
    }
}
