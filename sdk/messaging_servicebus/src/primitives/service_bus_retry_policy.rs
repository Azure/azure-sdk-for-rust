use std::hash::Hash;
use std::time::Duration;

pub static SERVER_BUSY_BASE_SLEEP_TIME: Duration = Duration::from_secs(10);

pub enum RetryError<E> {
    ServiceBusy,
    Operation(E),
}

/// An abstract representation of a policy to govern retrying of messaging operations.
///
/// It is recommended that developers without advanced needs not implement custom retry
/// policies but instead configure the default policy by specifying the desired set of
/// retry options when creating one of the Service Bus clients.
pub trait ServiceBusRetryPolicy: Eq + Hash + ToString {
    type Ok: Send + Sync;
    type Error: std::error::Error + Send + Sync;
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
        last_error: &Self::Error,
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
}

pub(crate) mod private {
    use std::future::Future;
    use std::time::Duration;

    use async_trait::async_trait;
    use tokio::time::timeout;
    use tokio_util::sync::CancellationToken;

    use crate::core::TransportConnectionScope;

    use super::{
        RetryError, ServiceBusRetryPolicy, ServiceBusRetryPolicyState, SERVER_BUSY_BASE_SLEEP_TIME,
    };

    #[async_trait]
    pub(crate) trait ServiceBusRetryPolicyExt: ServiceBusRetryPolicy {
        async fn run_operation<F, T1, Fut, S>(
            &mut self,
            operation: F,
            t1: T1,
            scope: S,
            cancellation_token: CancellationToken,
        ) -> Result<Self::Ok, RetryError<Self::Error>>
        where
            F: Fn(T1, Duration, &CancellationToken) -> Fut + Send + Sync,
            T1: Clone + Send + Sync,
            Fut: Future<Output = Result<Self::Ok, Self::Error>> + Send,
            S: TransportConnectionScope + Send + Sync,
        {
            let mut failed_attempt_count = 0;
            let mut try_timeout = self.calculate_try_timeout(0);
            if self.state().is_server_busy() && try_timeout < SERVER_BUSY_BASE_SLEEP_TIME {
                // We are in a server busy state before we start processing. Since
                // ServerBusyBaseSleepTime > remaining time for the operation, we don't wait for the
                // entire Sleep time.
                timeout(try_timeout, cancellation_token.cancelled())
                    .await
                    .map_err(|_| RetryError::ServiceBusy)?
            }

            let outcome = loop {
                if self.state().is_server_busy() {
                    let cancelled_fut = cancellation_token.cancelled();
                    let _ = timeout(SERVER_BUSY_BASE_SLEEP_TIME, cancelled_fut).await;
                }

                match (operation)(t1.clone(), try_timeout, &cancellation_token).await {
                    Ok(outcome) => break outcome,
                    Err(error) => {
                        failed_attempt_count += 1;
                        let retry_delay = self.calculate_retry_delay(&error, failed_attempt_count);

                        match (
                            retry_delay,
                            scope.is_disposed(),
                            cancellation_token.is_cancelled(),
                        ) {
                            (Some(retry_delay), false, false) => {
                                log::error!("{}", &error);

                                let _ = timeout(retry_delay, cancellation_token.cancelled()).await;
                                try_timeout = self.calculate_try_timeout(failed_attempt_count);
                            }
                            _ => return Err(RetryError::Operation(error)),
                        }
                    }
                }
            };

            Ok(outcome)
        }

        fn set_server_busy(&mut self, error_message: String) {
            let state = self.state_mut();

            *state.is_server_busy_mut() = true;
            *state.server_busy_error_message_mut() = error_message;
        }

        fn reset_server_busy(&mut self) {
            *self.state_mut().is_server_busy_mut() = false;
        }

        async fn schedule_reset_server_busy(&mut self) {
            tokio::time::sleep(SERVER_BUSY_BASE_SLEEP_TIME).await;
            self.reset_server_busy()
        }
    }

    impl<T> ServiceBusRetryPolicyExt for T where T: ServiceBusRetryPolicy {}
}
