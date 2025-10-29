// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// cspell: ignore retryable backoff

use azure_core::{sleep, time::Duration};
use rand::random;
use std::{fmt::Debug, pin::Pin};
use tracing::{info, warn};

/// Type alias for recovery operation function to reduce complexity
pub(crate) type RecoveryOperation<C, E> = fn(
    C,
    ErrorRecoveryAction,
) -> Pin<
    Box<dyn std::future::Future<Output = std::result::Result<(), E>> + Send + 'static>,
>;

/// Action to be taken for Eventhubs Errors
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum ErrorRecoveryAction {
    /// Error is retryable, Retry the operation.
    RetryAction,
    /// Error requires reconnecting the Connection, Session and Link.
    ReconnectConnection,
    /// Error requires reconnecting the Session and Link
    ReconnectSession,
    /// Error requires reconnecting the Link
    ReconnectLink,
    /// Error is not retryable, return the error.
    ReturnError,
}

/// Options for configuring exponential backoff retry behavior.
#[derive(Debug, Clone)]
pub struct RetryOptions {
    /// The initial backoff delay (Default is 100ms).
    pub initial_delay: Duration,

    /// The maximum backoff delay (Default is 30s).
    pub max_delay: Duration,

    /// The maximum total elapsed time for retries (Default is 60s).
    pub max_total_elapsed: Duration,

    /// The maximum number of retries (Default is 5).
    pub max_retries: u32,
}

impl Default for RetryOptions {
    fn default() -> Self {
        Self {
            initial_delay: Duration::milliseconds(200),
            max_delay: Duration::seconds(30),
            max_retries: 8,
            max_total_elapsed: Duration::seconds(60),
        }
    }
}

/// Executes an operation with exponential backoff.
///
/// This function will retry the operation with increasing delays until
/// it succeeds or the maximum number of retries is reached.
///
/// # Arguments
///
/// * `operation` - The operation to retry. This should be a function or closure that returns
///   a `Result` type.
/// * `options` - Configuration options for the retry policy.
/// * `categorize_error` - Function that determines the category of the error which has occurred.
/// * `recover_operation` - Function that handles the error recovery action based on the error category.
///
/// # Returns
///
/// * `Result<T, E>` - The result of the operation if it succeeds, or the last error if all
///   retries are exhausted.
///
pub(crate) async fn recover_with_backoff<F, Fut, T, E, C>(
    operation: F,
    options: &RetryOptions,
    categorize_error: fn(&E) -> ErrorRecoveryAction,
    recover_operation: Option<RecoveryOperation<C, E>>,
    context: Option<C>,
) -> std::result::Result<T, E>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = std::result::Result<T, E>>,
    E: Debug + std::fmt::Display,
    C: Clone,
{
    let mut current_retry = 0u32;
    let mut current_delay = options.initial_delay;

    let start_time = std::time::Instant::now();

    loop {
        match operation().await {
            Ok(result) => {
                if current_retry > 0 {
                    info!("Operation succeeded after {} retries", current_retry);
                }
                return Ok(result);
            }
            Err(err) => {
                let time_since_start = start_time.elapsed();
                info!("Operation failed with error: {}, checking for retry.", err);
                // Check if we've exhausted our retries
                if current_retry >= options.max_retries
                    || time_since_start >= options.max_total_elapsed
                {
                    warn!(
                        "Maximum retries ({}) reached or time elapsed ({:?}), returning error: {:?}",
                        options.max_retries, time_since_start, err
                    );
                    return Err(err);
                }
                // Check if we should retry this error
                let error_category = categorize_error(&err);
                match error_category {
                    ErrorRecoveryAction::RetryAction => {
                        let sleep_ms = options.initial_delay.whole_milliseconds() as u64
                            * 2u64.pow(current_retry)
                            + u64::from(random::<u8>());
                        let sleep_ms = sleep_ms.min(
                            options
                                .max_delay
                                .whole_milliseconds()
                                .try_into()
                                .unwrap_or(u64::MAX),
                        );
                        let sleep_duration = Duration::milliseconds(sleep_ms as i64);

                        info!(
                            "Operation failed with error: {:?}. Retrying after {:?} (retry {}/{})",
                            err,
                            sleep_duration,
                            current_retry + 1,
                            options.max_retries
                        );

                        // Wait for the backoff duration
                        sleep(sleep_duration).await;

                        // Calculate the next delay with exponential backoff
                        let next_delay = current_delay.saturating_mul(2);
                        current_delay = std::cmp::min(next_delay, options.max_delay);
                        // Continue to retry
                    }
                    ErrorRecoveryAction::ReturnError => {
                        warn!("Error is not retryable, returning: {:?}", err);
                        return Err(err);
                    }
                    _ => {
                        warn!("Error requires special handling: {:?}", err);
                        // Handle special error cases (e.g., reconnecting). If no recovery action is provided,
                        // return the error.
                        if let (Some(recover_operation), Some(context)) =
                            (recover_operation, context.clone())
                        {
                            recover_operation(context, error_category).await?;
                        } else {
                            return Err(err);
                        }
                    }
                }
                // Increase retry count
                current_retry += 1;
            }
        }
    }
}

/// Helper function to retry specific Azure Core operations.
///
/// This is a specialization of `retry_with_backoff` for Azure operations that return `AmqpError`.
///
/// # Arguments
///
/// * `operation` - The Azure operation to retry
/// * `options` - Configuration options for the retry policy
/// * `is_retryable` - Optional function that determines if an error should be retried
///
/// # Returns
///
/// * `Result<T>` - The result of the operation if it succeeds, or the last error if all
///   retries are exhausted.
pub(crate) async fn recover_azure_operation<F, Fut, T, C, E>(
    operation: F,
    options: &RetryOptions,
    categorize_error: fn(&E) -> ErrorRecoveryAction,
    recover_operation: Option<RecoveryOperation<C, E>>,
    context: Option<C>,
) -> std::result::Result<T, E>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = std::result::Result<T, E>>,
    E: Debug + std::error::Error,
    C: Clone,
{
    recover_with_backoff(
        operation,
        options,
        categorize_error,
        recover_operation,
        context,
    )
    .await
}

#[cfg(test)]
mod tests {
    use crate::EventHubsError;

    use super::*;
    use azure_core_test::{recorded, TestContext};
    use std::{
        result,
        sync::atomic::{AtomicUsize, Ordering},
    };
    use tracing::info;

    #[recorded::test]
    async fn test_retry_success_on_first_attempt(_ctx: TestContext) -> Result<(), EventHubsError> {
        let result = recover_with_backoff(
            || async { Ok::<_, String>("success") },
            &RetryOptions::default(),
            |_| ErrorRecoveryAction::RetryAction,
            None,
            None::<()>,
        )
        .await;

        assert_eq!(result.unwrap(), "success");
        Ok(())
    }

    #[recorded::test]
    async fn test_retry_success_after_retries(_ctx: TestContext) -> Result<(), EventHubsError> {
        let attempts = AtomicUsize::new(0);

        let result = recover_with_backoff(
            || async {
                let attempt = attempts.fetch_add(1, Ordering::SeqCst);
                if attempt < 2 {
                    Err(format!("Failed attempt {}", attempt))
                } else {
                    Ok(format!("Success on attempt {}", attempt))
                }
            },
            &RetryOptions::default(),
            |_| ErrorRecoveryAction::RetryAction,
            None,
            None::<()>,
        )
        .await;

        assert_eq!(result.unwrap(), "Success on attempt 2");
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
        Ok(())
    }

    #[recorded::test]
    async fn test_retry_exhausted(_ctx: TestContext) -> Result<(), EventHubsError> {
        let attempts = AtomicUsize::new(0);
        let options = RetryOptions {
            initial_delay: Duration::milliseconds(10),
            max_delay: Duration::milliseconds(50),
            max_retries: 2,
            max_total_elapsed: Duration::seconds(10),
        };

        let result: result::Result<&str, String> = recover_with_backoff(
            || async {
                let attempt = attempts.fetch_add(1, Ordering::SeqCst);
                Err(format!("Failed attempt {}", attempt))
            },
            &options,
            |_| ErrorRecoveryAction::RetryAction,
            None,
            None::<()>,
        )
        .await;

        assert!(result.is_err());
        assert_eq!(attempts.load(Ordering::SeqCst), 3); // Initial + 2 retries
        Ok(())
    }

    #[recorded::test]
    async fn test_retry_with_is_retryable(_ctx: TestContext) -> Result<(), EventHubsError> {
        let attempts = AtomicUsize::new(0);

        // Only retry if the error message contains "retry"
        let is_retryable = |err: &String| {
            if err.contains("retry") {
                ErrorRecoveryAction::RetryAction
            } else {
                ErrorRecoveryAction::ReturnError
            }
        };

        let result = recover_with_backoff(
            || async {
                info!("Attempting operation. {}", attempts.load(Ordering::SeqCst));
                let attempt = attempts.fetch_add(1, Ordering::SeqCst);
                match attempt {
                    0 => Err(String::from("please retry")),
                    1 => Err(String::from("don't retry")),
                    2 => Err(String::from("I told you not to retry")),
                    _ => Ok("shouldn't get here"),
                }
            },
            &RetryOptions {
                initial_delay: Duration::milliseconds(10),
                max_delay: Duration::milliseconds(50),
                max_retries: 2,
                max_total_elapsed: Duration::seconds(1),
            },
            is_retryable,
            None,
            None::<()>,
        )
        .await;

        assert_eq!(result.unwrap_err(), "I told you not to retry");
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
        Ok(())
    }
}
