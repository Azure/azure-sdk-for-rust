use std::time::Duration;

use crate::event_hubs_retry_mode::EventHubsRetryMode;

const MAX_RETRIES: u32 = 100;
const DEFAULT_MAX_RETRIES: u32 = 3;
const DEFAULT_DELAY: Duration = Duration::from_millis(800);
const DEFAULT_MAXIMUM_DELAY: Duration = Duration::from_secs(60);
const DEFAULT_TRY_TIMEOUT: Duration = Duration::from_secs(60);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaxRetries(pub u32);

impl Default for MaxRetries {
    fn default() -> Self {
        Self(DEFAULT_MAX_RETRIES)
    }
}

impl TryFrom<u32> for MaxRetries {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > MAX_RETRIES {
            Err(value)
        } else {
            Ok(Self(value))
        }
    }
}

/// The set of options that can be specified to influence how
/// retry attempts are made, and a failure is eligible to be retried.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventHubsRetryOptions {
    /// The maximum number of retry attempts before considering the associated operation to have failed
    pub max_retries: MaxRetries,

    /// The delay or back-off factor to apply between retry attempts
    pub delay: Duration,

    /// The maximum delay to allow between retry attempts
    pub maximum_delay: Duration,

    /// The maximum duration to wait for an operation, per attempt
    pub try_timeout: Duration,

    /// The approach to use for calculating retry delays
    ///
    /// The default retry mode is [`EventHubsRetryMode::Exponential`]
    pub mode: EventHubsRetryMode,
}

impl Default for EventHubsRetryOptions {
    fn default() -> Self {
        Self {
            max_retries: MaxRetries::default(),
            delay: DEFAULT_DELAY,
            maximum_delay: DEFAULT_MAXIMUM_DELAY,
            try_timeout: DEFAULT_TRY_TIMEOUT,
            mode: EventHubsRetryMode::default(),
        }
    }
}
