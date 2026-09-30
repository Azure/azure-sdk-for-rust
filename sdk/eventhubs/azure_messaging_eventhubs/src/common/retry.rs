// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// cspell: ignore retryable backoff

use azure_core::{sleep, time::Duration};
use futures::future::BoxFuture;
use rand::random;
use std::{
    fmt::Debug,
    pin::Pin,
    sync::{Mutex, PoisonError},
    time::{Duration as StdDuration, Instant},
};
use tracing::{debug, info, warn};

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
    ///
    /// A receive counts only recovery actions, backoff, and link attach, and
    /// not the wait for an event. A receive link that fails after a long
    /// healthy wait starts a new recovery with a new retry count and budget.
    /// Other operations count all time from the first attempt.
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

/// Source of time for a recovery loop, so tests can control it.
pub(crate) trait RecoveryClock: Sync {
    fn now(&self) -> Instant;
    fn sleep(&self, duration: Duration) -> BoxFuture<'_, ()>;
}

/// The clock that production recovery loops use.
pub(crate) struct SystemClock;

impl RecoveryClock for SystemClock {
    fn now(&self) -> Instant {
        Instant::now()
    }

    fn sleep(&self, duration: Duration) -> BoxFuture<'_, ()> {
        Box::pin(sleep(duration))
    }
}

/// The delivery wait that proves a receive link healthy. It replaces the .NET
/// rule that an empty receive resets the retry count. It is much longer than
/// an attach round trip, so a broken link stays in one episode, and much
/// shorter than the default 60 s budget.
pub(crate) const HEALTHY_WAIT: StdDuration = StdDuration::from_secs(10);

/// The attempt calls [`DeliveryWait::start`] after its link is attached. The
/// budget does not count the time after that mark.
pub(crate) struct DeliveryWait<'a, K> {
    clock: &'a K,
    started: Mutex<Option<Instant>>,
}

impl<'a, K: RecoveryClock> DeliveryWait<'a, K> {
    pub(crate) fn new(clock: &'a K) -> Self {
        Self {
            clock,
            started: Mutex::new(None),
        }
    }

    pub(crate) fn clock(&self) -> &'a K {
        self.clock
    }

    pub(crate) fn start(&self) {
        *self.started.lock().unwrap_or_else(PoisonError::into_inner) = Some(self.clock.now());
    }

    fn take(&self) -> Option<Instant> {
        self.started
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take()
    }
}

/// The decision that ended a recovery loop with an error.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum StopReason {
    ElapsedBudgetExhausted,
    RetriesExhausted,
    NonRecoverable,
    RecoveryFailed,
    RecoveryUnavailable,
}

impl StopReason {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            StopReason::ElapsedBudgetExhausted => "elapsed_budget_exhausted",
            StopReason::RetriesExhausted => "retries_exhausted",
            StopReason::NonRecoverable => "non_recoverable",
            StopReason::RecoveryFailed => "recovery_failed",
            StopReason::RecoveryUnavailable => "recovery_unavailable",
        }
    }
}

/// Identifiers that a recovery loop adds to each of its events.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct RecoveryLogContext<'a> {
    pub connection_id: Option<&'a str>,
    pub partition_id: Option<&'a str>,
}

/// The per-operation settings of [`recover_with_policy`].
pub(crate) struct RecoveryPolicy<'a, K> {
    /// `None` charges all time. A receive sets it, so the budget does not
    /// count the waits for a delivery.
    pub delivery_wait: Option<&'a DeliveryWait<'a, K>>,
    pub log_context: RecoveryLogContext<'a>,
    pub clock: &'a K,
}

/// The state of one recovery episode, which the recovery events report.
///
/// The durations in the events have these meanings:
///
/// * `receive_wait_elapsed`: receive only. The delivery wait in the episode,
///   which is not charged.
/// * `recovery_elapsed`: the time charged to `max_total_elapsed`: all time in
///   the episode less `receive_wait_elapsed`.
///
/// A receive that fails after [`HEALTHY_WAIT`] starts a new episode.
struct Episode<'a> {
    options: &'a RetryOptions,
    log_context: RecoveryLogContext<'a>,
    max_total_elapsed: StdDuration,
    retries_attempted: u32,
    start: Instant,
    receive_wait_elapsed: Option<StdDuration>,
    recovery_elapsed: StdDuration,
}

impl Episode<'_> {
    fn update(&mut self, now: Instant) {
        self.recovery_elapsed = now
            .saturating_duration_since(self.start)
            .saturating_sub(self.receive_wait_elapsed.unwrap_or_default());
    }

    fn stop<E: Debug>(&self, stop_reason: StopReason, err: &E) {
        warn!(
            connection_id = self.log_context.connection_id.map(display),
            partition_id = self.log_context.partition_id.map(display),
            stop_reason = %stop_reason.as_str(),
            retries_attempted = self.retries_attempted,
            max_retries = self.options.max_retries,
            receive_wait_elapsed = self.receive_wait_elapsed.map(debug),
            recovery_elapsed = ?self.recovery_elapsed,
            max_total_elapsed = ?self.max_total_elapsed,
            err = ?err,
            "Operation recovery stopped, returning error."
        );
    }
}

fn backoff_delay(options: &RetryOptions, retries_attempted: u32) -> Duration {
    let sleep_ms = options.initial_delay.whole_milliseconds() as u64 * 2u64.pow(retries_attempted)
        + u64::from(random::<u8>());
    let sleep_ms = sleep_ms.min(
        options
            .max_delay
            .whole_milliseconds()
            .try_into()
            .unwrap_or(u64::MAX),
    );
    Duration::milliseconds(sleep_ms as i64)
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
    recover_with_policy(
        operation,
        options,
        categorize_error,
        recover_operation,
        context,
        RecoveryPolicy {
            delivery_wait: None,
            log_context: RecoveryLogContext::default(),
            clock: &SystemClock,
        },
    )
    .await
}

/// Executes an operation with exponential backoff under `policy`.
///
/// On failure, the loop checks the retry count and the elapsed-time budget
/// before it classifies the error, then retries or recovers. It logs one
/// warning with a `stop_reason` when it returns an error. See [`Episode`].
pub(crate) async fn recover_with_policy<F, Fut, T, E, C, K>(
    operation: F,
    options: &RetryOptions,
    categorize_error: fn(&E) -> ErrorRecoveryAction,
    recover_operation: Option<RecoveryOperation<C, E>>,
    context: Option<C>,
    policy: RecoveryPolicy<'_, K>,
) -> std::result::Result<T, E>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = std::result::Result<T, E>>,
    E: Debug + std::fmt::Display,
    C: Clone,
    K: RecoveryClock,
{
    let clock = policy.clock;
    let log_context = policy.log_context;
    let mut current_delay = options.initial_delay;

    let mut episode = Episode {
        options,
        log_context,
        // A negative limit converts to zero, so the budget stays exhausted as before.
        max_total_elapsed: StdDuration::try_from(options.max_total_elapsed).unwrap_or_default(),
        retries_attempted: 0,
        start: clock.now(),
        receive_wait_elapsed: policy.delivery_wait.map(|_| StdDuration::ZERO),
        recovery_elapsed: StdDuration::ZERO,
    };

    loop {
        let result = operation().await;
        let now = clock.now();
        let wait_start = policy.delivery_wait.and_then(DeliveryWait::take);
        let waited = wait_start.map_or(StdDuration::ZERO, |start| {
            now.saturating_duration_since(start)
        });
        let mut new_episode = false;
        if let (Err(_), Some(wait_start)) = (&result, wait_start) {
            if waited >= HEALTHY_WAIT && episode.retries_attempted > 0 {
                info!(
                    connection_id = log_context.connection_id.map(display),
                    partition_id = log_context.partition_id.map(display),
                    retries_attempted = episode.retries_attempted,
                    healthy_wait = ?waited,
                    "Receive link failed after a healthy wait, starting a new recovery episode."
                );
                episode.retries_attempted = 0;
                episode.start = wait_start;
                episode.receive_wait_elapsed = Some(StdDuration::ZERO);
                new_episode = true;
            }
        }
        if let Some(total) = episode.receive_wait_elapsed.as_mut() {
            *total += waited;
        }
        episode.update(now);

        let err = match result {
            Ok(result) => {
                if episode.retries_attempted > 0 {
                    info!(
                        connection_id = log_context.connection_id.map(display),
                        partition_id = log_context.partition_id.map(display),
                        retries_attempted = episode.retries_attempted,
                        receive_wait_elapsed = episode.receive_wait_elapsed.map(debug),
                        recovery_elapsed = ?episode.recovery_elapsed,
                        "Operation succeeded after retries."
                    );
                }
                return Ok(result);
            }
            Err(err) => err,
        };

        debug!(
            connection_id = log_context.connection_id.map(display),
            partition_id = log_context.partition_id.map(display),
            err = %err,
            retries_attempted = episode.retries_attempted,
            recovery_elapsed = ?episode.recovery_elapsed,
            "Operation failed, checking for retry."
        );
        // Check if we've exhausted our retries
        if episode.retries_attempted >= options.max_retries {
            episode.stop(StopReason::RetriesExhausted, &err);
            return Err(err);
        }
        if episode.recovery_elapsed >= episode.max_total_elapsed {
            episode.stop(StopReason::ElapsedBudgetExhausted, &err);
            return Err(err);
        }
        // Check if we should retry this error
        let error_category = categorize_error(&err);
        match error_category {
            ErrorRecoveryAction::RetryAction => {
                let sleep_duration = backoff_delay(options, episode.retries_attempted);

                debug!(
                    connection_id = log_context.connection_id.map(display),
                    partition_id = log_context.partition_id.map(display),
                    err = ?err,
                    backoff = ?sleep_duration,
                    retry = episode.retries_attempted + 1,
                    max_retries = options.max_retries,
                    "Operation failed, retrying after backoff."
                );

                // Wait for the backoff duration
                clock.sleep(sleep_duration).await;

                // Calculate the next delay with exponential backoff
                let next_delay = current_delay.saturating_mul(2);
                current_delay = std::cmp::min(next_delay, options.max_delay);
                // Continue to retry
            }
            ErrorRecoveryAction::ReturnError => {
                episode.stop(StopReason::NonRecoverable, &err);
                return Err(err);
            }
            _ => {
                warn!(
                    connection_id = log_context.connection_id.map(display),
                    partition_id = log_context.partition_id.map(display),
                    error_category = ?error_category,
                    retries_attempted = episode.retries_attempted,
                    max_retries = options.max_retries,
                    receive_wait_elapsed = episode.receive_wait_elapsed.map(debug),
                    recovery_elapsed = ?episode.recovery_elapsed,
                    err = ?err,
                    "Error requires recovery, attempting recovery action."
                );
                // Handle recoverable error cases (reconnecting connection, session, or
                // link). If no recovery action is provided, return the error.
                let (Some(recover_operation), Some(context)) = (recover_operation, context.clone())
                else {
                    episode.stop(StopReason::RecoveryUnavailable, &err);
                    return Err(err);
                };
                // Reconnects have no backoff. A new episode waits one backoff
                // step, so a link that fails after each healthy wait is slow.
                if new_episode {
                    clock.sleep(backoff_delay(options, 0)).await;
                }
                match recover_operation(context, error_category.clone()).await {
                    Ok(()) => {
                        info!(
                            connection_id = log_context.connection_id.map(display),
                            partition_id = log_context.partition_id.map(display),
                            error_category = ?error_category,
                            "Recovery action succeeded."
                        );
                    }
                    Err(recovery_err) => {
                        episode.update(clock.now());
                        episode.stop(StopReason::RecoveryFailed, &recovery_err);
                        return Err(recovery_err);
                    }
                }
            }
        }
        // Increase retry count
        episode.retries_attempted += 1;
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
pub(crate) mod test_support {
    use super::*;
    use futures::future::ready;
    use std::{
        future::Future,
        io,
        sync::{Arc, Mutex},
    };
    use tracing::{instrument::WithSubscriber, Level};

    /// A clock that moves only when a test advances it or a backoff sleeps.
    pub(crate) struct ManualClock {
        origin: Instant,
        offset: Mutex<StdDuration>,
    }

    impl ManualClock {
        pub(crate) fn new() -> Arc<Self> {
            Arc::new(Self {
                origin: Instant::now(),
                offset: Mutex::new(StdDuration::ZERO),
            })
        }

        pub(crate) fn advance(&self, by: StdDuration) {
            *self.offset.lock().unwrap() += by;
        }
    }

    impl RecoveryClock for ManualClock {
        fn now(&self) -> Instant {
            self.origin + *self.offset.lock().unwrap()
        }

        fn sleep(&self, duration: Duration) -> BoxFuture<'_, ()> {
            self.advance(StdDuration::try_from(duration).unwrap_or_default());
            Box::pin(ready(()))
        }
    }

    #[derive(Clone)]
    struct SharedBuffer(Arc<Mutex<Vec<u8>>>);

    impl io::Write for SharedBuffer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    /// Runs `future` with only WARN and higher enabled and returns the log text.
    pub(crate) async fn capture_warnings<Fut: Future>(future: Fut) -> (Fut::Output, String) {
        capture_logs(Level::WARN, future).await
    }

    /// Runs `future` with `level` and higher enabled and returns the log text.
    pub(crate) async fn capture_logs<Fut: Future>(
        level: Level,
        future: Fut,
    ) -> (Fut::Output, String) {
        let buffer = SharedBuffer(Arc::new(Mutex::new(Vec::new())));
        let writer = buffer.clone();
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(level)
            .with_ansi(false)
            .without_time()
            .with_writer(move || writer.clone())
            .finish();
        let output = future.with_subscriber(subscriber).await;
        let logs = String::from_utf8(buffer.0.lock().unwrap().clone()).unwrap();
        (output, logs)
    }

    /// Returns the single terminal recovery warning in `logs`.
    pub(crate) fn stop_warning(logs: &str) -> &str {
        let mut lines = logs
            .lines()
            .filter(|line| line.contains("Operation recovery stopped"));
        let line = lines.next().expect("no terminal recovery warning");
        assert!(lines.next().is_none(), "duplicate terminal warning: {logs}");
        line
    }
}

#[cfg(test)]
mod tests {
    use crate::EventHubsError;

    use super::{
        test_support::{capture_warnings, stop_warning, ManualClock},
        *,
    };
    use azure_core_test::{recorded, TestContext};
    use std::{
        future::Future,
        result,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
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

    type Counter = Arc<AtomicUsize>;

    fn count_recovery(
        recoveries: Counter,
        _: ErrorRecoveryAction,
    ) -> Pin<Box<dyn Future<Output = result::Result<(), String>> + Send>> {
        recoveries.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn fail_recovery(
        _: Counter,
        _: ErrorRecoveryAction,
    ) -> Pin<Box<dyn Future<Output = result::Result<(), String>> + Send>> {
        Box::pin(async { Err(String::from("reconnect failed")) })
    }

    fn reconnect_link(_: &String) -> ErrorRecoveryAction {
        ErrorRecoveryAction::ReconnectLink
    }

    /// Fails the first attempt after a 120 s wait and succeeds after that.
    async fn slow_first_failure(
        receive: bool,
        recover: RecoveryOperation<Counter, String>,
    ) -> (result::Result<usize, String>, usize, String) {
        let clock = ManualClock::new();
        let wait = DeliveryWait::new(&*clock);
        let attempts = AtomicUsize::new(0);
        let recoveries = Counter::default();
        let (result, logs) = capture_warnings(recover_with_policy(
            || {
                let attempt = attempts.fetch_add(1, Ordering::SeqCst);
                if receive {
                    wait.start();
                }
                if attempt == 0 {
                    clock.advance(StdDuration::from_secs(120));
                }
                async move {
                    if attempt > 0 {
                        return Ok(attempt);
                    }
                    Err(String::from("link closed"))
                }
            },
            &RetryOptions::default(),
            reconnect_link,
            Some(recover),
            Some(recoveries.clone()),
            RecoveryPolicy {
                delivery_wait: receive.then_some(&wait),
                log_context: RecoveryLogContext::default(),
                clock: &*clock,
            },
        ))
        .await;
        (result, recoveries.load(Ordering::SeqCst), logs)
    }

    // Send, management, and authorization operations keep their behavior: the
    // first attempt counts, so a slow attempt that fails stops at once.
    #[tokio::test]
    async fn operation_start_budget_includes_first_attempt() {
        let (result, recoveries, logs) = slow_first_failure(false, count_recovery).await;
        assert_eq!(result.unwrap_err(), "link closed");
        assert_eq!(recoveries, 0);
        let warning = stop_warning(&logs);
        assert!(
            warning.contains("stop_reason=elapsed_budget_exhausted"),
            "{warning}"
        );
        assert!(warning.contains("retries_attempted=0"), "{warning}");
        assert!(warning.contains("recovery_elapsed=120s"), "{warning}");
        assert!(warning.contains("max_total_elapsed=60s"), "{warning}");
        assert!(!warning.contains("receive_wait_elapsed"), "{warning}");
    }

    #[tokio::test]
    async fn receive_budget_excludes_delivery_wait() {
        let (result, recoveries, _) = slow_first_failure(true, count_recovery).await;
        assert_eq!(result.unwrap(), 1);
        assert_eq!(recoveries, 1);
    }

    #[tokio::test]
    async fn failed_recovery_action_is_the_stop_reason() {
        let (result, _, logs) = slow_first_failure(true, fail_recovery).await;
        assert_eq!(result.unwrap_err(), "reconnect failed");
        let warning = stop_warning(&logs);
        assert!(warning.contains("stop_reason=recovery_failed"), "{warning}");
        assert!(warning.contains("receive_wait_elapsed=120s"), "{warning}");
    }
}
