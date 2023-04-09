use std::time::Duration;

use rand::Rng;

use crate::{
    event_hubs_retry_mode::EventHubsRetryMode, event_hubs_retry_options::EventHubsRetryOptions,
    event_hubs_retry_policy::EventHubsRetryPolicy,
};

const DEFAULT_JITTER_FACTOR: f64 = 0.08;
const DEFAULT_MINIMUM_THROTTLE_SECONDS: u32 = 4;
const DEFAULT_MAXIMUM_THROTTLE_SECONDS: u32 = 8;

pub struct BasicRetryPolicy {
    options: EventHubsRetryOptions,
    jitter_factor: f64,
    minimum_throttle_seconds: u32,
    maximum_throttle_seconds: u32,
}

impl BasicRetryPolicy {
    //     fn should_retry_last_error(&self, last_error: &dyn std::error::Error) -> bool {
    //         todo!()
    //     }
}

impl From<EventHubsRetryOptions> for BasicRetryPolicy {
    fn from(options: EventHubsRetryOptions) -> Self {
        Self {
            options,
            jitter_factor: DEFAULT_JITTER_FACTOR,
            minimum_throttle_seconds: DEFAULT_MINIMUM_THROTTLE_SECONDS,
            maximum_throttle_seconds: DEFAULT_MAXIMUM_THROTTLE_SECONDS,
        }
    }
}

impl EventHubsRetryPolicy for BasicRetryPolicy {
    fn calculate_try_timeout(&self, _attempt_count: u32) -> std::time::Duration {
        self.options.try_timeout
    }

    fn calculate_retry_delay(
        &self,
        last_error: &dyn std::error::Error,
        attempt_count: u32,
    ) -> Option<std::time::Duration> {
        if self.options.delay == std::time::Duration::ZERO
            || self.options.maximum_delay == std::time::Duration::ZERO
            || attempt_count > self.options.max_retries.0
        // || !self.should_retry_last_error(last_error) // TODO:
        {
            return None;
        }

        let base_jitter_seconds = self.options.delay.as_secs_f64() * self.jitter_factor;
        let retry_delay = match &self.options.mode {
            EventHubsRetryMode::Fixed => {
                calculate_fixed_delay(self.options.delay.as_secs_f64(), base_jitter_seconds)
            }
            EventHubsRetryMode::Exponential => calculate_exponential_delay(
                self.options.delay.as_secs_f64(),
                base_jitter_seconds,
                attempt_count,
            ),
        };

        // TODO: determine if the error represents a request to throttle

        // Adjust the delay
        if retry_delay > self.options.maximum_delay {
            Some(self.options.maximum_delay)
        } else {
            Some(retry_delay)
        }
    }
}

fn calculate_fixed_delay(base_delay_seconds: f64, base_jitter_seconds: f64) -> Duration {
    let mut rng = rand::thread_rng();
    let delay = base_delay_seconds * (rng.gen_range(0.0..1.0) * base_jitter_seconds);
    if delay > Duration::MAX.as_secs_f64() {
        Duration::MAX
    } else {
        Duration::from_secs_f64(delay)
    }
}

fn calculate_exponential_delay(
    base_delay_seconds: f64,
    base_jitter_seconds: f64,
    attempt_count: u32,
) -> Duration {
    let mut rng = rand::thread_rng();
    let delay = (f64::powi(2.0, attempt_count as i32) * base_delay_seconds)
        + (rng.gen_range(0.0..1.0) * base_jitter_seconds);
    if delay > Duration::MAX.as_secs_f64() {
        Duration::MAX
    } else {
        Duration::from_secs_f64(delay)
    }
}
