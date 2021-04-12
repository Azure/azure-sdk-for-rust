use crate::policies::{RetryPolicy, RetryPolicyInstance};
use crate::sleep::sleep;
use chrono::{DateTime, Local};
use std::time::Duration;

/// Built in retry policies that support linear and exponential backoff (with an added random delay
/// up to 256 ms). There also is a no retry policy to conform to
/// [https://docs.microsoft.com/en-us/azure/architecture/best-practices/retry-service-specific#retry-mechanism-10](https://docs.microsoft.com/en-us/azure/architecture/best-practices/retry-service-specific#retry-mechanism-10).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExponentialRetryPolicy {
    delay: Duration,
    max_retries: u32,
    max_delay: Duration,
}

impl Default for ExponentialRetryPolicy {
    fn default() -> Self {
        Self {
            delay: Duration::from_secs(3),
            max_retries: 3,
            max_delay: Duration::from_secs(30),
        }
    }
}

impl RetryPolicy for ExponentialRetryPolicy {
    fn create_instance(&self) -> Box<dyn RetryPolicyInstance> {
        Box::new(ExponentialRetryPolicyInstance {
            retry_policy: self.clone(),
            current_retries: 0,
            first_retry_time: None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExponentialRetryPolicyInstance {
    retry_policy: ExponentialRetryPolicy,
    first_retry_time: Option<DateTime<Local>>,
    current_retries: u32,
}

impl ExponentialRetryPolicyInstance {
    fn is_expired(&mut self) -> bool {
        if self.current_retries > self.retry_policy.max_retries {
            return true;
        }

        if self.first_retry_time.is_none() {
            self.first_retry_time = Some(Local::now());
        }

        if Local::now()
            > self.first_retry_time.unwrap()
                + chrono::Duration::from_std(self.retry_policy.max_delay).unwrap()
        {
            return true;
        }

        false
    }
}

#[async_trait::async_trait]
impl RetryPolicyInstance for ExponentialRetryPolicyInstance {
    async fn should_retry(&mut self) -> bool {
        self.current_retries += 1;
        if !self.is_expired() {
            let sleep_ms: u64 = self.retry_policy.delay.as_millis() as u64
                * self.current_retries as u64
                + rand::random::<u8>() as u64;
            sleep(Duration::from_millis(sleep_ms)).await;
            true
        } else {
            false
        }
    }
}
