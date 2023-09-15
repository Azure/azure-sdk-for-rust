use rand::Rng;
use std::{fmt::Display, time::Duration};

use crate::primitives::{
    service_bus_retry_mode::ServiceBusRetryMode,
    service_bus_retry_options::ServiceBusRetryOptions,
    service_bus_retry_policy::{
        ServiceBusRetryPolicy, ServiceBusRetryPolicyError, ServiceBusRetryPolicyState,
    },
};

const JITTER_FACTOR: f64 = 0.08;

/// State for the basic retry policy.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BasicRetryPolicyState {
    /// The server is not busy.
    ServerNotBusy,
    /// The server is busy.
    ServerBusy {
        /// The error message if the server is busy.
        error_message: String,
    },
}

impl Default for BasicRetryPolicyState {
    fn default() -> Self {
        Self::ServerNotBusy
    }
}

impl ServiceBusRetryPolicyState for BasicRetryPolicyState {
    fn is_server_busy(&self) -> bool {
        match self {
            BasicRetryPolicyState::ServerNotBusy => false,
            BasicRetryPolicyState::ServerBusy { .. } => true,
        }
    }

    fn set_server_busy(&mut self, error_message: String) {
        *self = BasicRetryPolicyState::ServerBusy { error_message };
    }

    fn reset_server_busy(&mut self) {
        *self = BasicRetryPolicyState::ServerNotBusy;
    }

    fn server_busy_error_message(&self) -> Option<&str> {
        match self {
            BasicRetryPolicyState::ServerNotBusy => None,
            BasicRetryPolicyState::ServerBusy { error_message } => Some(error_message),
        }
    }
}

/// The default retry policy for the Service Bus client library, respecting the
/// configuration specified as a set of [`ServiceBusRetryOptions`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BasicRetryPolicy {
    options: ServiceBusRetryOptions,
    state: BasicRetryPolicyState,
}

impl Display for BasicRetryPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BasicRetryPolicy")
    }
}

impl From<ServiceBusRetryOptions> for BasicRetryPolicy {
    fn from(options: ServiceBusRetryOptions) -> Self {
        Self {
            options,
            state: BasicRetryPolicyState::default(),
        }
    }
}

impl ServiceBusRetryPolicy for BasicRetryPolicy {
    fn options(&self) -> &ServiceBusRetryOptions {
        &self.options
    }

    fn state(&self) -> &dyn ServiceBusRetryPolicyState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut dyn ServiceBusRetryPolicyState {
        &mut self.state
    }

    fn calculate_try_timeout(&self, _attempt_count: u32) -> std::time::Duration {
        self.options.try_timeout
    }

    fn calculate_retry_delay(
        &self,
        _last_error: &dyn ServiceBusRetryPolicyError,
        attempt_count: u32,
    ) -> Option<std::time::Duration> {
        if self.options.max_retries == 0
            || self.options.delay.is_zero()
            || self.options.max_delay.is_zero()
            || attempt_count > self.options.max_retries
        {
            return None;
        }

        let base_jitter_seconds = self.options.delay.as_secs_f64() * JITTER_FACTOR;
        let mut rng = rand::thread_rng();
        let retry_delay = match self.options.mode {
            ServiceBusRetryMode::Fixed => calculate_fixed_delay(
                self.options.delay.as_secs_f64(),
                base_jitter_seconds,
                &mut rng,
            ),
            ServiceBusRetryMode::Exponential => calculate_exponential_delay(
                attempt_count,
                self.options.delay.as_secs_f64(),
                base_jitter_seconds,
                &mut rng,
            ),
        };

        if retry_delay < self.options.max_delay {
            Some(retry_delay)
        } else {
            Some(self.options.max_delay)
        }
    }
}

fn calculate_fixed_delay(
    base_delay_seconds: f64,
    base_jitter_seconds: f64,
    rng: &mut impl Rng,
) -> Duration {
    let delay = base_delay_seconds + rng.gen_range(0.0..1.0) * base_jitter_seconds;
    if delay > Duration::MAX.as_secs_f64() {
        Duration::MAX
    } else {
        Duration::from_secs_f64(delay)
    }
}

fn calculate_exponential_delay(
    attempt_count: u32,
    base_delay_seconds: f64,
    base_jitter_seconds: f64,
    rng: &mut impl Rng,
) -> Duration {
    // var delay = (Math.Pow(2, attemptCount) * baseDelaySeconds) + (random.NextDouble() * baseJitterSeconds);
    // return delay > MaximumTimeSpanSeconds ? TimeSpan.MaxValue : TimeSpan.FromSeconds(delay);
    let delay = f64::powi(attempt_count as f64, 2) * base_delay_seconds
        + rng.gen_range(0.0..1.0) * base_jitter_seconds;
    if delay > Duration::MAX.as_secs_f64() {
        Duration::MAX
    } else {
        Duration::from_secs_f64(delay)
    }
}
