use tokio::time::{sleep, Duration};

#[async_trait::async_trait]
pub trait RetryPolicy {
    async fn create_instance(&self) -> Box<dyn RetryPolicyInstance>;
}

#[async_trait::async_trait]
pub trait RetryPolicyInstance {
    async fn retry(&mut self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinearRetryPolicy {
    max_retries: u32,
    seconds_backoff: u32,
}

impl Default for LinearRetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            seconds_backoff: 2,
        }
    }
}

#[async_trait::async_trait]
impl RetryPolicy for LinearRetryPolicy {
    async fn create_instance(&self) -> Box<dyn RetryPolicyInstance> {
        Box::new(LinearRetryPolicyInstance {
            linear_retry_policy: self.clone(),
            current_retries: 0,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExponentialBackoffRetryPolicy {
    max_retries: u32,
    seconds_backoff: u32,
}

impl Default for ExponentialBackoffRetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            seconds_backoff: 2,
        }
    }
}

#[async_trait::async_trait]
impl RetryPolicy for ExponentialBackoffRetryPolicy {
    async fn create_instance(&self) -> Box<dyn RetryPolicyInstance> {
        Box::new(ExponentialBackoffRetryPolicyInstance {
            linear_retry_policy: self.clone(),
            current_retries: 0,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinearRetryPolicyInstance {
    linear_retry_policy: LinearRetryPolicy,
    current_retries: u32,
}

#[async_trait::async_trait]
impl RetryPolicyInstance for LinearRetryPolicyInstance {
    async fn retry(&mut self) -> bool {
        self.current_retries += 1;
        if self.current_retries <= self.linear_retry_policy.max_retries {
            sleep(Duration::from_secs(
                self.linear_retry_policy.seconds_backoff as u64,
            ))
            .await;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExponentialBackoffRetryPolicyInstance {
    linear_retry_policy: ExponentialBackoffRetryPolicy,
    current_retries: u32,
}

#[async_trait::async_trait]
impl RetryPolicyInstance for ExponentialBackoffRetryPolicyInstance {
    async fn retry(&mut self) -> bool {
        self.current_retries += 1;
        if self.current_retries <= self.linear_retry_policy.max_retries {
            sleep(Duration::from_secs(
                (self.linear_retry_policy.seconds_backoff * self.current_retries) as u64,
            ))
            .await;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoRetryPolicy {}

#[async_trait::async_trait]
impl RetryPolicy for NoRetryPolicy {
    async fn create_instance(&self) -> Box<dyn RetryPolicyInstance> {
        Box::new(NoRetryPolicy {})
    }
}

#[async_trait::async_trait]
impl RetryPolicyInstance for NoRetryPolicy {
    async fn retry(&mut self) -> bool {
        false
    }
}
