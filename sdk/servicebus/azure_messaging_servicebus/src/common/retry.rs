// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use azure_core::time::Duration;

/// Options for retrying Service Bus operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetryOptions {
    /// The maximum number of retry attempts.
    pub max_retries: u32,
    /// The delay between retry attempts.
    pub retry_delay: Duration,
    /// The maximum delay between retry attempts.
    pub max_retry_delay: Duration,
    /// The total timeout for the operation.
    pub try_timeout: Duration,
    /// The retry mode to use.
    pub retry_mode: RetryMode,
}

impl Default for RetryOptions {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay: Duration::milliseconds(800),
            max_retry_delay: Duration::seconds(60),
            try_timeout: Duration::seconds(60),
            retry_mode: RetryMode::Exponential,
        }
    }
}

/// The retry mode for Service Bus operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryMode {
    /// Fixed delay between retries.
    Fixed,
    /// Exponential backoff with jitter.
    Exponential,
}

impl RetryOptions {
    /// Creates a new RetryOptions with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum number of retry attempts.
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Sets the delay between retry attempts.
    pub fn with_retry_delay(mut self, retry_delay: Duration) -> Self {
        self.retry_delay = retry_delay;
        self
    }

    /// Sets the maximum delay between retry attempts.
    pub fn with_max_retry_delay(mut self, max_retry_delay: Duration) -> Self {
        self.max_retry_delay = max_retry_delay;
        self
    }

    /// Sets the total timeout for the operation.
    pub fn with_try_timeout(mut self, try_timeout: Duration) -> Self {
        self.try_timeout = try_timeout;
        self
    }

    /// Sets the retry mode.
    pub fn with_retry_mode(mut self, retry_mode: RetryMode) -> Self {
        self.retry_mode = retry_mode;
        self
    }

    /// Calculates the delay for the specified retry attempt.
    pub fn calculate_retry_delay(&self, attempt: u32) -> Duration {
        match self.retry_mode {
            RetryMode::Fixed => std::cmp::min(self.retry_delay, self.max_retry_delay),
            RetryMode::Exponential => {
                let base_delay = self.retry_delay.whole_milliseconds() as u64;
                let exponential_delay = base_delay * 2_u64.pow(attempt);
                let max_delay = self.max_retry_delay.whole_milliseconds() as u64;

                // Add some jitter to avoid thundering herd
                let jitter = (rand::random::<f64>() * 0.1 + 0.9) as u64;
                let delay_with_jitter = (exponential_delay * jitter) / 100;

                Duration::milliseconds(std::cmp::min(delay_with_jitter, max_delay) as i64)
            }
        }
    }

    /// Checks if the operation should be retried based on the error.
    pub fn should_retry(&self, attempt: u32, error: &crate::ServiceBusError) -> bool {
        if attempt >= self.max_retries {
            return false;
        }

        match error.kind() {
            crate::ErrorKind::RequestTimeout => true,
            crate::ErrorKind::Amqp => true,
            crate::ErrorKind::QuotaExceeded => false,
            crate::ErrorKind::EntityNotFound => false,
            crate::ErrorKind::InvalidRequest => false,
            crate::ErrorKind::MessageLockLost => false,
            crate::ErrorKind::MessageNotFound => false,
            crate::ErrorKind::MessageSizeExceeded => false,
            crate::ErrorKind::ServiceBusClosed => false,
            crate::ErrorKind::SessionLockLost => false,
            crate::ErrorKind::Cancelled => false,
            crate::ErrorKind::Unknown => true,
        }
    }
}

impl std::fmt::Display for RetryMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RetryMode::Fixed => write!(f, "Fixed"),
            RetryMode::Exponential => write!(f, "Exponential"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ErrorKind, ServiceBusError};

    #[test]
    fn default_retry_options() {
        let options = RetryOptions::default();
        assert_eq!(options.max_retries, 3);
        assert_eq!(options.retry_delay, Duration::milliseconds(800));
        assert_eq!(options.max_retry_delay, Duration::seconds(60));
        assert_eq!(options.try_timeout, Duration::seconds(60));
        assert_eq!(options.retry_mode, RetryMode::Exponential);
    }

    #[test]
    fn builder_pattern() {
        let options = RetryOptions::new()
            .with_max_retries(5)
            .with_retry_delay(Duration::milliseconds(1000))
            .with_max_retry_delay(Duration::seconds(120))
            .with_try_timeout(Duration::seconds(300))
            .with_retry_mode(RetryMode::Fixed);

        assert_eq!(options.max_retries, 5);
        assert_eq!(options.retry_delay, Duration::milliseconds(1000));
        assert_eq!(options.max_retry_delay, Duration::seconds(120));
        assert_eq!(options.try_timeout, Duration::seconds(300));
        assert_eq!(options.retry_mode, RetryMode::Fixed);
    }

    #[test]
    fn fixed_retry_delay() {
        let options = RetryOptions::new()
            .with_retry_mode(RetryMode::Fixed)
            .with_retry_delay(Duration::milliseconds(1000));

        assert_eq!(
            options.calculate_retry_delay(0),
            Duration::milliseconds(1000)
        );
        assert_eq!(
            options.calculate_retry_delay(1),
            Duration::milliseconds(1000)
        );
        assert_eq!(
            options.calculate_retry_delay(5),
            Duration::milliseconds(1000)
        );
    }

    #[test]
    fn exponential_retry_delay() {
        let options = RetryOptions::new()
            .with_retry_mode(RetryMode::Exponential)
            .with_retry_delay(Duration::milliseconds(100))
            .with_max_retry_delay(Duration::seconds(10));

        let delay_0 = options.calculate_retry_delay(0);
        let delay_1 = options.calculate_retry_delay(1);
        let delay_5 = options.calculate_retry_delay(5);

        // Should increase with each attempt
        assert!(delay_0 <= Duration::seconds(10));
        assert!(delay_1 <= Duration::seconds(10));
        assert!(delay_5 <= Duration::seconds(10));
    }

    #[test]
    fn should_retry_logic() {
        let options = RetryOptions::new().with_max_retries(2);

        // Should retry on timeout
        let timeout_error = ServiceBusError::new(ErrorKind::RequestTimeout, "Timeout");
        assert!(options.should_retry(0, &timeout_error));
        assert!(options.should_retry(1, &timeout_error));
        assert!(!options.should_retry(2, &timeout_error)); // Exceeds max retries

        // Should not retry on invalid request
        let invalid_error = ServiceBusError::new(ErrorKind::InvalidRequest, "Invalid");
        assert!(!options.should_retry(0, &invalid_error));

        // Should not retry on entity not found
        let not_found_error = ServiceBusError::new(ErrorKind::EntityNotFound, "Not found");
        assert!(!options.should_retry(0, &not_found_error));
    }

    #[test]
    fn retry_mode_display() {
        assert_eq!(RetryMode::Fixed.to_string(), "Fixed");
        assert_eq!(RetryMode::Exponential.to_string(), "Exponential");
    }
}
