use std::time::Duration;

use crate::event_hubs_retry_mode::EventHubsRetryMode;

const MAX_RETRIES: u32 = 100;
const DEFAULT_MAX_RETRIES: u32 = 3;
const DEFAULT_DELAY: Duration = Duration::from_millis(800);
const DEFAULT_MAXIMUM_DELAY: Duration = Duration::from_secs(60);
const DEFAULT_TRY_TIMEOUT: Duration = Duration::from_secs(60);

/// The maximum number of retry attempts before considering the associated operation to have failed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaxRetries(pub(crate) u32);

impl MaxRetries {
    /// Creates a new instance with the specified value. If the value is greater than 100, the value
    /// is returned as an error.
    pub fn new(value: u32) -> Result<Self, u32> {
        Self::try_from(value)
    }

    /// Gets the inner value.
    pub fn value(&self) -> u32 {
        self.0
    }
}

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

#[cfg(test)]
mod tests {
    use crate::{EventHubsRetryOptions, MaxRetries, event_hubs_retry_options::{DEFAULT_MAXIMUM_DELAY, DEFAULT_DELAY, DEFAULT_TRY_TIMEOUT}, EventHubsRetryMode};

    #[test]
    fn default_values() {
        let options = EventHubsRetryOptions {
            max_retries: MaxRetries::try_from(5).unwrap(),
            ..Default::default()
        };

        assert_eq!(options.max_retries.0, 5);
        assert_eq!(options.delay, DEFAULT_DELAY);
        assert_eq!(options.maximum_delay, DEFAULT_MAXIMUM_DELAY);
        assert_eq!(options.try_timeout, DEFAULT_TRY_TIMEOUT);
        assert_eq!(options.mode, EventHubsRetryMode::default());
    }
}
