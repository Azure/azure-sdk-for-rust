//! Defines the retry policy trait for Service Bus operations.

use std::time::Duration as StdDuration;

use async_trait::async_trait;
use fe2o3_amqp::link::SendError;
use fe2o3_amqp_management::error::Error as ManagementError;

use super::service_bus_retry_options::ServiceBusRetryOptions;

pub(crate) static SERVER_BUSY_BASE_SLEEP_TIME: StdDuration = StdDuration::from_secs(10);

/// Trait for operation errors that can be retried.
pub trait ServiceBusRetryPolicyError: std::error::Error + Send + Sync + 'static {
    /// Returns true if the error is recoverable by recovering the connection scope.
    fn should_try_recover(&self) -> bool;

    /// Returns true if the connection scope is disposed.
    fn is_scope_disposed(&self) -> bool;
}

pub(crate) fn should_try_recover_from_management_error(
    error: &fe2o3_amqp_management::error::Error,
) -> bool {
    use fe2o3_amqp::link::{LinkStateError, RecvError};
    matches!(
        error,
        ManagementError::Send(SendError::LinkStateError(
            LinkStateError::IllegalSessionState
        )) | ManagementError::Recv(RecvError::LinkStateError(
            LinkStateError::IllegalSessionState
        ))
    )
}

// TODO: use azure_core::retry_policy::RetryPolicy?

/// An abstract representation of a policy to govern retrying of messaging operations.
///
/// It is recommended that developers without advanced needs not implement custom retry
/// policies but instead configure the default policy by specifying the desired set of
/// retry options when creating one of the Service Bus clients.
pub trait ServiceBusRetryPolicy {
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

    /// Calculates the amount of time to wait before another attempt should be made and whether
    /// or not another attempt should be made.
    ///
    /// Returns None if no more attempts should be made.
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

/// Runs the operation with the retry policy.
///
/// TODO: This is a rather temporary solution as there would be weird lifetime issue if the same
/// thing is implemented as a method.
macro_rules! run_operation {
    ($policy:tt, $err_ty:ty, $try_timeout:ident, $op:expr) => {{
        let mut _failed_attempt_count = 0; // avoid accidental shadowing
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

            let outcome = match tokio::time::timeout($try_timeout, $op).await {
                Ok(result) => result.map_err(<$err_ty>::from),
                Err(elapsed) => Err(<$err_ty>::from(elapsed)),
            };
            match outcome {
                Ok(outcome) => break outcome,
                Err(error) => {
                    _failed_attempt_count += 1;
                    let _retry_delay = $policy.calculate_retry_delay(&error, _failed_attempt_count);
                    // TODO: check if the error is recoverable

                    match (
                        _retry_delay,
                        crate::primitives::service_bus_retry_policy::ServiceBusRetryPolicyError::is_scope_disposed(&error)
                    ) {
                        (Some(retry_delay), false) => {
                            log::error!("{}", &error);
                            tokio::time::sleep(retry_delay).await;
                            $try_timeout = $policy.calculate_try_timeout(_failed_attempt_count);
                        }
                        _ => return Err(crate::primitives::error::RetryError::Operation(error)),
                    }
                }
            }
        };

        Result::<_, crate::primitives::error::RetryError<$err_ty>>::Ok(outcome)
    }};

    ($policy:tt, $err_ty:ty, $try_timeout:ident, $op:expr, $recover_op:expr) => {{
        let mut _failed_attempt_count = 0; // avoid accidental shadowing
        let mut _should_try_recover = false; // avoid accidental shadowing
        let mut _is_scope_disposed = false; // avoid accidental shadowing
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

            // Recover before trying the operation
            if _should_try_recover {
                match tokio::time::timeout($try_timeout, $recover_op).await {
                    Ok(result) => {
                        if let Err(recover_error) = result {
                            log::error!("Failed to recover {}", &recover_error);
                            _is_scope_disposed = crate::primitives::service_bus_retry_policy::ServiceBusRetryPolicyError::is_scope_disposed(&recover_error);
                        }
                    }
                    Err(elapsed) => {
                        let err = <$err_ty>::from(elapsed);
                        _is_scope_disposed = crate::primitives::service_bus_retry_policy::ServiceBusRetryPolicyError::is_scope_disposed(&err);
                    }
                }
            }

            let outcome = match tokio::time::timeout($try_timeout, $op).await {
                Ok(result) => result.map_err(<$err_ty>::from),
                Err(elapsed) => Err(<$err_ty>::from(elapsed)),
            };
            match outcome {
                Ok(outcome) => break outcome,
                Err(error) => {
                    // TODO: error handling strategy
                    // 1. check if the transport object should try to recover An error is
                    //    recoverable if it indicates the session/connection event loop has stopped.
                    // 2. if not recoverable, the error is simply retried until the retry policy is
                    //    exhausted.
                    // 3. if recoverable, the transport is recovered (in the next iter) before
                    //    retrying the operation. If the recover operation fails, then try to see if
                    //    the connection scope is disposed. If it is, then the error is not
                    //    recoverable and the retry policy is exhausted.

                    _failed_attempt_count += 1;
                    let _retry_delay = $policy.calculate_retry_delay(&error, _failed_attempt_count);
                    _is_scope_disposed |= crate::primitives::service_bus_retry_policy::ServiceBusRetryPolicyError::is_scope_disposed(&error);
                    // TODO: check if the error is recoverable
                    _should_try_recover = crate::primitives::service_bus_retry_policy::ServiceBusRetryPolicyError::should_try_recover(&error);

                    match (_retry_delay, _is_scope_disposed) {
                        (Some(retry_delay), false) => {
                            log::error!("{}", &error);
                            tokio::time::sleep(retry_delay).await;
                            $try_timeout = $policy.calculate_try_timeout(_failed_attempt_count);
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
