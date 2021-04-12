use crate::policies::{RetryPolicy, RetryPolicyInstance};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct NoRetryPolicy {}

impl RetryPolicy for NoRetryPolicy {
    fn create_instance(&self) -> Box<dyn RetryPolicyInstance> {
        Box::new(NoRetryPolicy {})
    }
}

#[async_trait::async_trait]
impl RetryPolicyInstance for NoRetryPolicy {
    async fn should_retry(&mut self) -> bool {
        false
    }
}
