use std::hash::Hash;
use std::time::Duration;

use crate::diagnostics::ServiceBusEventSource;

pub static SERVER_BUSY_BASE_SLEEP_TIME: Duration = Duration::from_secs(10);

/// An abstract representation of a policy to govern retrying of messaging operations.
///
/// It is recommended that developers without advanced needs not implement custom retry
/// policies but instead configure the default policy by specifying the desired set of
/// retry options when creating one of the Service Bus clients.
pub trait ServiceBusRetryPolicy: Eq + Hash + ToString {
    type Ok;
    type Error;
    type State: ServiceBusRetryPolicyState;

    fn state(&self) -> &Self::State;

    fn state_mut(&mut self) -> &mut Self::State;

    /// Calculates the amount of time to allow the current attempt for an operation to
    /// complete before considering it to be timed out.
    ///
    /// # Arguments
    ///
    /// * `attempt_count` - The number of total attempts that have been made, including the initial attempt before any retries.</param>
    ///
    /// # Returns
    ///
    /// The amount of time to allow for an operation to complete.
    fn calculate_try_timeout(&self, attempt_count: i32) -> Duration;

    /// Calculates the amount of time to wait before another attempt should be made.
    ///
    /// # Arguments
    ///
    /// * `last_error` - The last exception that was observed for the operation to be retried.
    /// * `attemptCount` - The number of total attempts that have been made, including the initial
    ///   attempt before any retries.
    ///
    /// # Returns
    ///
    /// The amount of time to delay before retrying the associated operation; if `None`, then
    /// the operation is no longer eligible to be retried.
    fn calculate_retry_delay(
        &self,
        last_error: Self::Error,
        attempt_count: i32,
    ) -> Option<Duration>;
}

pub trait ServiceBusRetryPolicyState {
    /// Determines whether or not the server returned a busy error.
    fn is_server_busy(&self) -> bool;

    fn is_server_busy_mut(&mut self) -> &mut bool;

    /// Gets the exception message when a server busy error is returned.
    fn server_busy_error_message(&self) -> &String;

    fn server_busy_error_message_mut(&mut self) -> &mut String;

    /// The instance of <see cref="ServiceBusEventSource" /> which can be mocked for testing.
    fn logger(&self) -> &ServiceBusEventSource;

    fn logger_mut(&mut self) -> &mut ServiceBusEventSource;
}

pub(crate) mod private {
    use std::future::Future;
    use std::time::Duration;

    use async_trait::async_trait;
    use tokio_util::sync::CancellationToken;

    use crate::core::TransportConnectionScope;

    use super::ServiceBusRetryPolicy;

    /// TODO:
    #[async_trait]
    pub trait ServiceBusRetryPolicyExt: ServiceBusRetryPolicy {
        async fn run_operation_not_verbose<F, T1, Fut, S>(
            &mut self,
            operation: F,
            t1: T1,
            scope: S,
            cancellation_token: CancellationToken,
        ) -> Result<(), Self::Error>
        where
            F: Fn(T1, Duration, CancellationToken) -> Fut + Send,
            T1: Send,
            Fut: Future<Output = Result<Self::Ok, Self::Error>>,
            S: TransportConnectionScope + Send,
        {
            self.run_operation(operation, t1, scope, cancellation_token, false)
                .await
        }

        async fn run_operation<F, T1, Fut, S>(
            &mut self,
            operation: F,
            t1: T1,
            scope: S,
            cancellation_token: CancellationToken,
            log_retries_as_verbose: bool,
        ) -> Result<(), Self::Error>
        where
            F: Fn(T1, Duration, CancellationToken) -> Fut + Send,
            T1: Send,
            Fut: Future<Output = Result<Self::Ok, Self::Error>>,
            S: TransportConnectionScope + Send,
        {
            todo!()
        }

        fn set_server_busy(&mut self, error_message: String) {
            todo!()
        }

        fn reset_server_busy(&mut self) {
            todo!()
        }

        async fn schedule_reset_server_busy(&mut self) {
            todo!()
        }
    }
}
