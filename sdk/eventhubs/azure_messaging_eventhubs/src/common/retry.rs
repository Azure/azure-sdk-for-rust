// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// cspell: ignore retryable backoff

use azure_core::{error::Result, time::Duration};
use rand::{rng, Rng};
use std::fmt::Debug;
use tracing::{info, warn};

/// Options for configuring exponential backoff retry behavior.
#[derive(Debug, Clone)]
pub struct RetryOptions {
    /// The initial backoff delay (Default is 100ms).
    pub initial_delay: Duration,

    /// The maximum backoff delay (Default is 30s).
    pub max_delay: Duration,

    /// The maximum number of retries (Default is 5).
    pub max_retries: usize,

    /// The jitter factor to apply to backoff timing (0.0 to 1.0) (Default is 0.2).
    /// A jitter factor of 0.2 means the delay will be randomly adjusted by up to ±20%.
    pub jitter: f64,
}

impl Default for RetryOptions {
    fn default() -> Self {
        Self {
            initial_delay: Duration::milliseconds(100),
            max_delay: Duration::seconds(30),
            max_retries: 5,
            jitter: 0.2,
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
/// * `is_retryable` - Optional function that determines if an error should be retried.
///   If not provided, all errors will be retried.
///
/// # Returns
///
/// * `Result<T, E>` - The result of the operation if it succeeds, or the last error if all
///   retries are exhausted.
///
pub(crate) async fn retry_with_backoff<F, Fut, T, E>(
    operation: F,
    options: &RetryOptions,
    is_retryable: Option<fn(&E) -> bool>,
) -> std::result::Result<T, E>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = std::result::Result<T, E>>,
    E: Debug + std::fmt::Display,
{
    let mut current_retry = 0;
    let mut current_delay = options.initial_delay;

    loop {
        match operation().await {
            Ok(result) => {
                if current_retry > 0 {
                    info!("Operation succeeded after {} retries", current_retry);
                }
                return Ok(result);
            }
            Err(err) => {
                info!("Operation failed with error: {}, checking for retry.", err);
                // Check if we should retry this error
                if let Some(checker) = is_retryable {
                    if !checker(&err) {
                        warn!("Error is not retryable, returning: {:?}", err);
                        return Err(err);
                    }
                }

                // Check if we've exhausted our retries
                if current_retry >= options.max_retries {
                    warn!(
                        "Maximum retries ({}) reached, returning error: {:?}",
                        options.max_retries, err
                    );
                    return Err(err);
                }

                // Apply jitter to the delay
                let jittered_delay = if options.jitter > 0.0 {
                    let jitter_range = options.jitter * current_delay.as_seconds_f64();
                    let jitter_amount = rng().random_range(-jitter_range..jitter_range);
                    let jittered_secs = current_delay.as_seconds_f64() + jitter_amount;
                    Duration::seconds_f64(jittered_secs.max(0.001)) // Ensure we don't go negative
                } else {
                    current_delay
                };

                info!(
                    "Operation failed with error: {:?}. Retrying after {:?} (retry {}/{})",
                    err,
                    jittered_delay,
                    current_retry + 1,
                    options.max_retries
                );

                // Wait for the backoff duration
                azure_core::sleep::sleep(jittered_delay).await;

                // Increase retry count
                current_retry += 1;

                // Calculate the next delay with exponential backoff
                let next_delay = current_delay.saturating_mul(2);
                current_delay = std::cmp::min(next_delay, options.max_delay);
            }
        }
    }
}

/// Helper function to retry specific Azure Core operations.
///
/// This is a specialization of `retry_with_backoff` for Azure operations that return `azure_core::error::Result`.
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
pub(crate) async fn retry_azure_operation<F, Fut, T>(
    operation: F,
    options: &RetryOptions,
    is_retryable: Option<fn(&azure_core::Error) -> bool>,
) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    retry_with_backoff(operation, options, is_retryable).await
}

#[cfg(test)]
mod tests {
    use tracing::info;

    use crate::consumer;

    use super::*;
    use std::{
        result,
        sync::atomic::{AtomicUsize, Ordering},
    };

    #[tokio::test]
    async fn test_retry_success_on_first_attempt() {
        let result = retry_with_backoff(
            || async { Ok::<_, String>("success") },
            &RetryOptions::default(),
            None,
        )
        .await;

        assert_eq!(result.unwrap(), "success");
    }

    #[tokio::test]
    async fn test_retry_success_after_retries() {
        let attempts = AtomicUsize::new(0);

        let result = retry_with_backoff(
            || async {
                let attempt = attempts.fetch_add(1, Ordering::SeqCst);
                if attempt < 2 {
                    Err(format!("Failed attempt {}", attempt))
                } else {
                    Ok(format!("Success on attempt {}", attempt))
                }
            },
            &RetryOptions::default(),
            None,
        )
        .await;

        assert_eq!(result.unwrap(), "Success on attempt 2");
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_exhausted() {
        let attempts = AtomicUsize::new(0);
        let options = RetryOptions {
            initial_delay: Duration::milliseconds(10),
            max_delay: Duration::milliseconds(50),
            max_retries: 2,
            jitter: 0.0,
        };

        let result: result::Result<&str, String> = retry_with_backoff(
            || async {
                let attempt = attempts.fetch_add(1, Ordering::SeqCst);
                Err(format!("Failed attempt {}", attempt))
            },
            &options,
            None,
        )
        .await;

        assert!(result.is_err());
        assert_eq!(attempts.load(Ordering::SeqCst), 3); // Initial + 2 retries
    }

    #[tokio::test]
    async fn test_retry_with_is_retryable() {
        consumer::tests::setup();
        let attempts = AtomicUsize::new(0);

        // Only retry if the error message contains "retry"
        let is_retryable = |err: &String| err.contains("retry");

        let result = retry_with_backoff(
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
                jitter: 0.0,
            },
            Some(is_retryable),
        )
        .await;

        assert_eq!(result.unwrap_err(), "I told you not to retry");
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
    }
}
