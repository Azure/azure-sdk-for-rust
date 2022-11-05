use std::hash::Hash;
use std::time::Duration;

use async_trait::async_trait;
use fe2o3_amqp::link::SendError;
use tokio::time::error::Elapsed;

use crate::amqp::error::NotAcceptedError;

use super::service_bus_retry_options::ServiceBusRetryOptions;

pub static SERVER_BUSY_BASE_SLEEP_TIME: Duration = Duration::from_secs(10);

pub enum RetryError<E> {
    ServiceBusy,
    Operation(E),
}

pub trait ServiceBusRetryPolicyError
where
    Self: std::error::Error + From<SendError> + From<Elapsed> + From<NotAcceptedError>,
{
    fn is_scope_disposed(&self) -> bool;
}

/// An abstract representation of a policy to govern retrying of messaging operations.
///
/// It is recommended that developers without advanced needs not implement custom retry
/// policies but instead configure the default policy by specifying the desired set of
/// retry options when creating one of the Service Bus clients.
pub trait ServiceBusRetryPolicy: Eq + Hash + ToString {
    // type Ok: Send + Sync;
    type Error: ServiceBusRetryPolicyError + Send + Sync;
    type State: ServiceBusRetryPolicyState;

    fn new(options: ServiceBusRetryOptions) -> Self;

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
    fn calculate_try_timeout(&self, attempt_count: u32) -> Duration;

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
        attempt_count: u32,
    ) -> Option<Duration>;
}

pub trait ServiceBusRetryPolicyState {
    /// Determines whether or not the server returned a busy error.
    fn is_server_busy(&self) -> bool;

    fn set_server_busy(&mut self, error_message: String);

    fn reset_server_busy(&mut self);

    /// Gets the exception message when a server busy error is returned.
    fn server_busy_error_message(&self) -> Option<&str>;
}

#[async_trait]
pub(crate) trait ServiceBusRetryPolicyExt: ServiceBusRetryPolicy + Send + Sync {
    // async fn run_operation<F, MutArg, Args, Fut>(
    //     &mut self,
    //     mut operation: F,
    //     mut_arg: &'static mut MutArg,
    //     args: Args,
    //     cancellation_token: CancellationToken,
    // ) -> Result<(), RetryError<Self::Error>>
    // where
    //     F: FnMut(&mut MutArg, &Args, Duration, CancellationToken) -> Fut + Send + Sync,
    //     MutArg: Send + Sync,
    //     Args: Send + Sync,
    //     Fut: Future<Output = Result<(), Self::Error>> + Send,
    // {
    //     let mut failed_attempt_count = 0;
    //     let mut try_timeout = self.calculate_try_timeout(0);
    //     if self.state().is_server_busy() && try_timeout < SERVER_BUSY_BASE_SLEEP_TIME {
    //         // We are in a server busy state before we start processing. Since
    //         // ServerBusyBaseSleepTime > remaining time for the operation, we don't wait for the
    //         // entire Sleep time.
    //         timeout(try_timeout, cancellation_token.cancelled())
    //             .await
    //             .map_err(|_| RetryError::ServiceBusy)?
    //     }

    //     let outcome = loop {
    //         if self.state().is_server_busy() {
    //             let cancelled_fut = cancellation_token.cancelled();
    //             let _ = timeout(SERVER_BUSY_BASE_SLEEP_TIME, cancelled_fut).await;
    //         }

    //         match (operation)(mut_arg, &args, try_timeout, cancellation_token.clone()).await {
    //             Ok(outcome) => break outcome,
    //             Err(error) => {
    //                 failed_attempt_count += 1;
    //                 let retry_delay = self.calculate_retry_delay(&error, failed_attempt_count);

    //                 match (
    //                     retry_delay,
    //                     error.is_scope_disposed(),
    //                     cancellation_token.is_cancelled(),
    //                 ) {
    //                     (Some(retry_delay), false, false) => {
    //                         log::error!("{}", &error);

    //                         let _ = timeout(retry_delay, cancellation_token.cancelled()).await;
    //                         try_timeout = self.calculate_try_timeout(failed_attempt_count);
    //                     }
    //                     _ => return Err(RetryError::Operation(error)),
    //                 }
    //             }
    //         }
    //     };

    //     Ok(outcome)
    // }

    fn set_server_busy(&mut self, error_message: String) {
        let state = self.state_mut();

        state.set_server_busy(error_message);
    }

    fn reset_server_busy(&mut self) {
        self.state_mut().reset_server_busy();
    }

    async fn schedule_reset_server_busy(&mut self) {
        tokio::time::sleep(SERVER_BUSY_BASE_SLEEP_TIME).await;
        self.reset_server_busy()
    }
}

impl<T> ServiceBusRetryPolicyExt for T where T: ServiceBusRetryPolicy + Send + Sync {}

macro_rules! run_operation {
    ($policy:ident, $policy_ty:ty, $cancellation_token:ident, $op:expr) => {{
        let mut failed_attempt_count = 0;
        let mut try_timeout = $policy.calculate_try_timeout(0);
        if $policy.state().is_server_busy()
            && try_timeout
                < crate::primitives::service_bus_retry_policy::SERVER_BUSY_BASE_SLEEP_TIME
        {
            // We are in a server busy state before we start processing. Since
            // ServerBusyBaseSleepTime > remaining time for the operation, we don't wait for the
            // entire Sleep time.
            tokio::time::timeout(try_timeout, $cancellation_token.cancelled())
                .await
                .map_err(|_| crate::primitives::service_bus_retry_policy::RetryError::ServiceBusy)?
        }

        let outcome = loop {
            if $policy.state().is_server_busy() {
                let cancelled_fut = $cancellation_token.cancelled();
                let _ = tokio::time::timeout(
                    crate::primitives::service_bus_retry_policy::SERVER_BUSY_BASE_SLEEP_TIME,
                    cancelled_fut,
                )
                .await;
            }

            let outcome = match tokio::time::timeout(try_timeout, async { $op }).await {
                Ok(result) => result.map_err(<$policy_ty>::Error::from),
                Err(err) => Err(<$policy_ty>::Error::from(err)),
            };
            match outcome {
                Ok(outcome) => break outcome,
                Err(error) => {
                    failed_attempt_count += 1;
                    let retry_delay = $policy.calculate_retry_delay(&error, failed_attempt_count);

                    match (
                        retry_delay,
                        error.is_scope_disposed(),
                        $cancellation_token.is_cancelled(),
                    ) {
                        (Some(retry_delay), false, false) => {
                            log::error!("{}", &error);

                            let _ =
                                tokio::time::timeout(retry_delay, $cancellation_token.cancelled())
                                    .await;
                            try_timeout = $policy.calculate_try_timeout(failed_attempt_count);
                        }
                        _ => return Err(RetryError::Operation(error)),
                    }
                }
            }
        };

        Result::<(), RetryError<<$policy_ty>::Error>>::Ok(outcome)
    }};
}

pub(crate) use run_operation;
