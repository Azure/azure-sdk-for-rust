use std::time::Duration;

use super::service_bus_retry_mode::ServiceBusRetryMode;

const RETRIES_MIN: u8 = 0;
const RETRIES_MAX: u8 = 100;
const DELAY_MIN: Duration = Duration::from_millis(1);
const DELAY_MAX: Duration = Duration::from_secs(5 * 60);
const TRY_TIMEOUT_MIN: Duration = Duration::ZERO;
const TRY_TIMEOUT_MAX: Duration = Duration::from_secs(1 * 60 * 60); // 1 Hour

pub enum OutOfRange<T> {
    LessThanAllowed { value: T, minimum_allowed: T },
    GreaterThanAllowed { value: T, maximum_allowed: T },
}

pub struct ServiceBusRetryOptions {
    /// The approach to use for calculating retry delays.
    ///
    /// # Value
    ///
    /// The default retry mode is [`ServiceBusRetryMode::Exponential`]
    mode: ServiceBusRetryMode,

    /// The maximum number of retry attempts before considering the associated operation to have
    /// failed.
    max_retries: u8,

    /// The delay or backoff factor to apply between retry attempts.
    delay: Duration,

    /// The maximum delay to allow between retry attempts.
    max_delay: Duration,

    /// The maximum duration to wait for an operation, per attempt.
    try_timeout: Duration,
    //
    // /// A custom retry policy to be used in place of the individual option values.
    // ///
    // /// # Remarks
    // ///
    // /// When populated, this custom policy will take precedence over the individual retry options
    // /// provided.
    // policy: P,
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
    const DEFAULT_MAX_RETRIES: u8 = 3;
    const DEFAULT_DELAY: Duration = Duration::from_millis(800);
    const DEFAULT_MAX_DELAY: Duration = Duration::from_secs(1 * 60);
    const DEFAULT_TRY_TIMEOUT: Duration = Duration::from_secs(1 * 60);

    /// The approach to use for calculating retry delays.
    ///
    /// # Value
    ///
    /// The default retry mode is [`ServiceBusRetryMode::Exponential`]
    pub fn mode(&self) -> &ServiceBusRetryMode {
        &self.mode
    }

    /// The approach to use for calculating retry delays.
    ///
    /// # Value
    ///
    /// The default retry mode is [`ServiceBusRetryMode::Exponential`]
    pub fn set_mode(&mut self, mode: ServiceBusRetryMode) {
        self.mode = mode;
    }

    /// The maximum number of retry attempts before considering the associated operation to have
    /// failed.
    ///
    /// # Value
    ///
    /// The default retry limit is 3.
    pub fn max_retries(&self) -> u8 {
        self.max_retries
    }

    /// The maximum number of retry attempts before considering the associated operation to have
    /// failed.
    ///
    /// # Value
    ///
    /// The default retry limit is 3.
    ///
    /// # Error
    ///
    /// Returns `Err(OutOfRange)` when the requested number of retries is not between 0 and
    /// 100 (inclusive).
    pub fn set_max_retries(&mut self, value: u8) -> Result<(), OutOfRange<u8>> {
        if value < RETRIES_MIN {
            Err(OutOfRange::LessThanAllowed {
                value,
                minimum_allowed: RETRIES_MIN,
            })
        } else if value > RETRIES_MAX {
            Err(OutOfRange::GreaterThanAllowed {
                value,
                maximum_allowed: RETRIES_MAX,
            })
        } else {
            self.max_retries = value;
            Ok(())
        }
    }

    /// The delay between retry attempts for a fixed approach or the delay on which to base
    /// calculations for a backoff-based approach.
    ///
    /// # Value
    ///
    /// The default delay is 0.8 seconds.
    pub fn delay(&self) -> &Duration {
        &self.delay
    }

    /// The delay between retry attempts for a fixed approach or the delay on which to base
    /// calculations for a backoff-based approach.
    ///
    /// # Value
    ///
    /// The default delay is 0.8 seconds.
    ///
    /// # Error
    ///
    /// Returns `Err(OutOfRange)` when the requested delay is not between 1 millisecond and 5
    /// minutes (inclusive).
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

    /// The maximum permissible delay between retry attempts.
    ///
    /// # Value
    ///
    /// The default maximum delay is 60 seconds.
    pub fn max_delay(&self) -> &Duration {
        &self.max_delay
    }

    /// The maximum permissible delay between retry attempts.
    ///
    /// # Value
    ///
    /// The default maximum delay is 60 seconds.
    pub fn set_max_delay(&mut self, value: Duration) {
        // `std::time::Duration` is already non-negative
        self.max_delay = value;
    }

    /// The maximum duration to wait for completion of a single attempt, whether the initial attempt
    /// or a retry.
    ///
    /// # Value
    ///
    /// The default timeout is 60 seconds.
    pub fn try_timeout(&self) -> &Duration {
        &self.try_timeout
    }

    /// The maximum duration to wait for completion of a single attempt, whether the initial attempt
    /// or a retry.
    ///
    /// # Value
    ///
    /// The default timeout is 60 seconds.
    ///
    /// # Error
    ///
    /// Returns `Err(OutOfRange)`  when the requested timeout is not between [`Duratino::ZERO`] and
    /// 1 hour (inclusive)
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

    // /// Get a reference to the current retry policy
    // pub fn policy(&self) -> &P {
    //     &self.policy
    // }

    // /// Get a mutable reference to the current retri policy
    // pub fn policy_mut(&mut self) -> &mut P {
    //     &mut self.policy
    // }

    // /// Set the retry policy
    // pub fn set_policy(&mut self, policy: P) {
    //     self.policy = policy;
    // }

    // /// A custom retry policy to be used in place of the individual option values.
    // ///
    // /// # Remarks
    // ///
    // /// When populated, this custom policy will take precedence over the individual retry
    // /// options provided.
    // pub fn custom_retry_policy<Q>(self, policy: Q) -> ServiceBusRetryOptions<Q>
    // where
    //     Q: ServiceBusRetryPolicy,
    // {
    //     ServiceBusRetryOptions {
    //         mode: self.mode,
    //         max_retries: self.max_retries,
    //         delay: self.delay,
    //         max_delay: self.max_delay,
    //         try_timeout: self.try_timeout,
    //         policy,
    //     }
    // }
}
