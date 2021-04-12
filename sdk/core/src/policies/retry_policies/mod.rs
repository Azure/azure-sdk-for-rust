mod exponential_retry;
mod linear_retry;
mod no_retry;
pub use exponential_retry::*;
pub use linear_retry::*;
pub use no_retry::*;

/// In order to define a custom policy implement this trait. This trait defines the "template" upon
/// which policy instances will be created. The struct that actually implements the retry policy
/// must implement `RetryPolicyInstance` instead. The distinction is that `RetryPolicy` is not
/// allowed to mutate `self` while `RetryPolicyInstance` is. The lifecycle of a
/// `RetryPolicyInstance` is a single send operation while a `RetryPolicy` can live as long as a
/// pipeline has a reference of it.
pub trait RetryPolicy {
    fn create_instance(&self) -> Box<dyn RetryPolicyInstance>;
}

#[async_trait::async_trait]
pub trait RetryPolicyInstance {
    /// Return `true` if the pipeline should retry the execution, `false` if it has to return an
    /// error. You can perform any operation here as long as it's not blocking.
    async fn should_retry(&mut self) -> bool;
}
