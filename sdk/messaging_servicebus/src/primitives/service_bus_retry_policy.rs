//! Defines the retry policy trait for Service Bus operations.

use std::hash::Hash;
use std::time::Duration as StdDuration;

use async_trait::async_trait;
use fe2o3_amqp::link::SendError;
use fe2o3_amqp_management::error::Error as ManagementError;

use super::service_bus_retry_options::ServiceBusRetryOptions;

pub(crate) static SERVER_BUSY_BASE_SLEEP_TIME: StdDuration = StdDuration::from_secs(10);

/// Trait for operation errors that can be retried.
pub trait ServiceBusRetryPolicyError: std::error::Error + Send + Sync + 'static {
    /// Returns true if the connection scope is disposed.
    fn is_scope_disposed(&self) -> bool;
}

impl ServiceBusRetryPolicyError for fe2o3_amqp_management::error::Error {
    fn is_scope_disposed(&self) -> bool {
        use fe2o3_amqp::link::{LinkStateError, RecvError};
        match self {
            ManagementError::Send(error) => match error {
                SendError::LinkStateError(LinkStateError::IllegalSessionState) => true,
                _ => false,
            },
            ManagementError::Recv(error) => match error {
                RecvError::LinkStateError(LinkStateError::IllegalSessionState) => true,
                _ => false,
            },
            _ => false,
        }
    }
}

/// An abstract representation of a policy to govern retrying of messaging operations.
///
/// It is recommended that developers without advanced needs not implement custom retry
/// policies but instead configure the default policy by specifying the desired set of
/// retry options when creating one of the Service Bus clients.
pub trait ServiceBusRetryPolicy: Eq + Hash + ToString {
    /// The type of state maintained by the retry policy.
    type State: ServiceBusRetryPolicyState;

    /// Creates a new retry policy instance with the specified options.
    fn new(options: ServiceBusRetryOptions) -> Self;

    /// Gets the retry options for the policy.
    fn options(&self) -> &ServiceBusRetryOptions;

    /// Gets the state for the policy.
    fn state(&self) -> &Self::State;

    /// Gets the state mutably for the policy.
    fn state_mut(&mut self) -> &mut Self::State;

    /// Calculates the amount of time to allow the current attempt for an operation to
    /// complete before considering it to be timed out.
    fn calculate_try_timeout(&self, attempt_count: u32) -> StdDuration;

    /// Calculates the amount of time to wait before another attempt should be made.
    fn calculate_retry_delay<E: ServiceBusRetryPolicyError>(
        &self,
        last_error: &E,
        attempt_count: u32,
    ) -> Option<StdDuration>;
}

/// Trait for state maintained by a retry policy.
pub trait ServiceBusRetryPolicyState {
    /// Determines whether or not the server returned a busy error.
    fn is_server_busy(&self) -> bool;

    /// Sets the server busy state.
    fn set_server_busy(&mut self, error_message: String);

    /// Resets the server busy state.
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
    ($policy:ident, $policy_ty:ty, $err_ty:ty, $try_timeout:ident, $op:expr) => {{
        let mut failed_attempt_count = 0;
        if crate::primitives::service_bus_retry_policy::ServiceBusRetryPolicyState::is_server_busy($policy.state())
            && $try_timeout
                < crate::primitives::service_bus_retry_policy::SERVER_BUSY_BASE_SLEEP_TIME
        {
            // We are in a server busy state before we start processing. Since
            // ServerBusyBaseSleepTime > remaining time for the operation, we don't wait for the
            // entire Sleep time.
            tokio::time::sleep($try_timeout).await;
        }

        let outcome = loop {
            if crate::primitives::service_bus_retry_policy::ServiceBusRetryPolicyState::is_server_busy($policy.state()) {
                tokio::time::sleep(
                    crate::primitives::service_bus_retry_policy::SERVER_BUSY_BASE_SLEEP_TIME,
                )
                .await;
            }

            let outcome = match tokio::time::timeout($try_timeout, async { $op }).await {
                Ok(result) => result.map_err(<$err_ty>::from),
                Err(err) => Err(<$err_ty>::from(err)),
            };
            match outcome {
                Ok(outcome) => break outcome,
                Err(error) => {
                    failed_attempt_count += 1;
                    let retry_delay = $policy.calculate_retry_delay(&error, failed_attempt_count);

                    match (
                        retry_delay,
                        crate::primitives::service_bus_retry_policy::ServiceBusRetryPolicyError::is_scope_disposed(&error)
                    ) {
                        (Some(retry_delay), false) => {
                            log::error!("{}", &error);
                            tokio::time::sleep(retry_delay).await;
                            $try_timeout = $policy.calculate_try_timeout(failed_attempt_count);
                        }
                        _ => return Err(crate::primitives::error::RetryError::Operation(error)),
                    }
                }
            }
        };

        Result::<_, crate::primitives::error::RetryError<$err_ty>>::Ok(outcome)
    }};

    ($policy:ident, $policy_ty:ty, $err_ty:ty, $try_timeout:ident, $cancellation_token:ident, $op:expr) => {{
        let mut failed_attempt_count = 0;
        if crate::primitives::service_bus_retry_policy::ServiceBusRetryPolicyState::is_server_busy($policy.state())
            && $try_timeout
                < crate::primitives::service_bus_retry_policy::SERVER_BUSY_BASE_SLEEP_TIME
        {
            // We are in a server busy state before we start processing. Since
            // ServerBusyBaseSleepTime > remaining time for the operation, we don't wait for the
            // entire Sleep time.
            tokio::time::timeout($try_timeout, $cancellation_token.cancelled())
                .await
                .map_err(|_| crate::primitives::error::RetryError::ServiceBusy)?
        }

        let outcome = loop {
            if crate::primitives::service_bus_retry_policy::ServiceBusRetryPolicyState::is_server_busy($policy.state()) {
                let cancelled_fut = $cancellation_token.cancelled();
                let _ = tokio::time::timeout(
                    crate::primitives::service_bus_retry_policy::SERVER_BUSY_BASE_SLEEP_TIME,
                    cancelled_fut,
                )
                .await;
            }

            let outcome = match tokio::time::timeout($try_timeout, async { $op }).await {
                Ok(result) => result.map_err(<$err_ty>::from),
                Err(err) => Err(<$err_ty>::from(err)),
            };
            match outcome {
                Ok(outcome) => break outcome,
                Err(error) => {
                    failed_attempt_count += 1;
                    let retry_delay = $policy.calculate_retry_delay(&error, failed_attempt_count);

                    match (
                        retry_delay,
                        crate::primitives::service_bus_retry_policy::ServiceBusRetryPolicyError::is_scope_disposed(&error),
                        $cancellation_token.is_cancelled(),
                    ) {
                        (Some(retry_delay), false, false) => {
                            log::error!("{}", &error);

                            let _ =
                                tokio::time::timeout(retry_delay, $cancellation_token.cancelled())
                                    .await;
                            $try_timeout = $policy.calculate_try_timeout(failed_attempt_count);
                        }
                        _ => return Err(crate::primitives::error::RetryError::Operation(error)),
                    }
                }
            }
        };

        Result::<_, crate::primitives::error::RetryError<$err_ty>>::Ok(outcome)
    }};
}

pub(crate) use run_operation;
