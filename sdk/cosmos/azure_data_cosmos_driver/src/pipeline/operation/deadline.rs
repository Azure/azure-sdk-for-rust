// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Enforces operation deadlines and bounds retry and hedge waits.

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use futures::{future, future::Future, FutureExt};

use crate::{
    diagnostics::DiagnosticsContextBuilder, options::OperationOptionsView,
    pipeline::operation::deadline,
};

/// Waits no longer than the remaining operation budget before retrying.
///
/// Absent, zero, or unsupported delays are skipped to preserve the retry policy.
pub(crate) async fn apply_failover_delay(
    clock: &crate::clock::Clock,
    delay: Option<Duration>,
    deadline: Option<Instant>,
) {
    let Some(mut delay) = delay else {
        return;
    };
    if let Some(deadline) = deadline {
        delay = delay.min(deadline.saturating_duration_since(clock.now()));
    }
    if delay.is_zero() {
        return;
    }
    if azure_core::time::Duration::try_from(delay).is_ok() {
        clock.sleep(delay).await;
    }
}

/// Returns the journal unchanged while time remains, or a diagnosed client timeout.
///
/// The timeout consumes the journal and uses the configured latency budget in its message.
pub(crate) fn enforce_deadline_or_timeout(
    clock: &crate::clock::Clock,
    deadline: Option<Instant>,
    options: &OperationOptionsView<'_>,
    mut diagnostics: DiagnosticsContextBuilder,
) -> Result<DiagnosticsContextBuilder, crate::error::CosmosError> {
    let Some(d) = deadline else {
        return Ok(diagnostics);
    };
    if clock.now() < d {
        return Ok(diagnostics);
    }

    let timeout_duration = options
        .end_to_end_latency_policy()
        .map(|p| p.timeout())
        .unwrap_or_default();

    diagnostics.set_operation_status(
        azure_core::http::StatusCode::RequestTimeout,
        Some(crate::error::status_codes::substatus::CLIENT_OPERATION_TIMEOUT),
    );
    let diagnostics_ctx = Arc::new(diagnostics.complete());
    Err(crate::error::CosmosError::builder()
        .with_status(crate::models::CosmosStatus::from_parts(
            azure_core::http::StatusCode::RequestTimeout,
            Some(crate::error::status_codes::substatus::CLIENT_OPERATION_TIMEOUT),
        ))
        .with_message(format!(
            "end-to-end operation timeout exceeded ({timeout_duration:?})"
        ))
        .with_diagnostics(diagnostics_ctx)
        .build())
}

/// Upper bound on how long [`hedging::execute`](crate::pipeline::operation::hedging::execute) waits for in-flight hedge
/// attempts to land their diagnostics after the application-cancel
/// deadline fires. The Rust hedge path deliberately bounds this — .NET
/// v3 awaits the most-recently-completed task with no timeout, but the
/// Rust path trades slightly less-rich diagnostics-on-cancel for
/// predictable user-visible cancel latency when a transport future is
/// stuck. The effective window is further capped by
/// [`effective_harvest_window`] for callers with aggressive timeouts.
pub(crate) const HARVEST_WINDOW: Duration = Duration::from_millis(50);

/// Lower bound on the harvest window so an already-completed `Ready`
/// future still resolves before the timer fires on typical OSes.
pub(crate) const HARVEST_WINDOW_FLOOR: Duration = Duration::from_millis(1);

/// Effective harvest window for a given operation timeout:
/// `min(HARVEST_WINDOW, request_timeout / 10)`, clamped at
/// [`HARVEST_WINDOW_FLOOR`]. Returns [`HARVEST_WINDOW`] when no timeout
/// is configured. Aggressive callers (e.g. `request_timeout = 100ms`)
/// avoid a fixed 50ms cancel-latency tax.
pub(crate) fn effective_harvest_window(configured_request_timeout: Option<Duration>) -> Duration {
    match configured_request_timeout {
        Some(t) => HARVEST_WINDOW.min(t / 10).max(HARVEST_WINDOW_FLOOR),
        None => HARVEST_WINDOW,
    }
}

/// Creates an owned deadline signal without allocating a boxed future.
///
/// An absent deadline never signals. Remaining time is measured at construction;
/// unsupported durations signal immediately when polled.
pub(crate) fn deadline_signal(
    clock: &crate::clock::Clock,
    deadline: Option<Instant>,
) -> impl Future<Output = ()> + Send + use<> {
    let remaining = deadline.map(|d| d.saturating_duration_since(clock.now()));
    let clock = clock.clone();
    async move {
        match remaining {
            None => future::pending::<()>().await,
            Some(duration) if azure_core::time::Duration::try_from(duration).is_ok() => {
                clock.sleep(duration).await;
            }
            Some(_) => {}
        }
    }
}

/// Collects a pending leg's diagnostics within a bounded grace period, discarding its result.
///
/// A ready leg is polled before the timer, even for a zero window. Expiry drops
/// the unfinished future; harvesting never publishes session or routing feedback.
pub(crate) async fn harvest_remaining_attempt<F, T>(
    clock: &crate::clock::Clock,
    attempt: F,
    parent: &mut DiagnosticsContextBuilder,
    harvest_window: Duration,
) where
    F: Future<Output = (T, DiagnosticsContextBuilder)> + Unpin + Send,
{
    if azure_core::time::Duration::try_from(harvest_window).is_err() {
        return;
    }
    if let Ok((_result, diagnostics)) =
        crate::timeout::with_timeout(harvest_window, attempt, clock.clone()).await
    {
        parent.merge_hedge_attempt(diagnostics);
    }
}

/// Completes the hedge journal with the client operation-timeout status.
pub(crate) fn application_cancelled_error(
    mut diagnostics: DiagnosticsContextBuilder,
) -> crate::error::CosmosError {
    diagnostics.set_operation_status(
        azure_core::http::StatusCode::RequestTimeout,
        Some(crate::error::status_codes::substatus::CLIENT_OPERATION_TIMEOUT),
    );
    let diagnostics_ctx = Arc::new(diagnostics.complete());
    crate::error::CosmosError::builder()
        .with_status(crate::models::CosmosStatus::from_parts(
            azure_core::http::StatusCode::RequestTimeout,
            Some(crate::error::status_codes::substatus::CLIENT_OPERATION_TIMEOUT),
        ))
        .with_message("operation cancelled by application deadline during cross-region hedging")
        .with_diagnostics(diagnostics_ctx)
        .build()
}

/// Awaits a partner leg, giving the operation deadline priority on ties.
///
/// If cancellation wins, harvests only diagnostics within the grace period and
/// returns no result. A ready result observed after the deadline is also discarded.
pub(crate) async fn await_attempt_or_deadline_harvest<F, T>(
    clock: &crate::clock::Clock,
    attempt: F,
    deadline: Option<Instant>,
    parent: &mut DiagnosticsContextBuilder,
    harvest_window: Duration,
) -> Option<(T, DiagnosticsContextBuilder)>
where
    F: Future<Output = (T, DiagnosticsContextBuilder)> + Unpin + Send,
{
    if deadline::deadline_elapsed(clock, deadline) {
        deadline::harvest_remaining_attempt(clock, attempt, parent, harvest_window).await;
        return None;
    }
    let deadline_fut = deadline::deadline_signal(clock, deadline).fuse();
    let attempt = attempt.fuse();
    // The selection macro polls through pinned references because async futures may not be Unpin.
    // Stack pinning keeps both futures at stable addresses until this scope drops them.
    futures::pin_mut!(deadline_fut, attempt);
    // Bias toward the operation deadline so application cancellation wins when
    // the partner attempt's own per-attempt deadline becomes ready at the same
    // instant. Otherwise the race is classified as both-transient instead of
    // cancelled-awaiting-partner based on scheduler polling order.
    futures::select_biased! {
        () = deadline_fut => {
            deadline::harvest_remaining_attempt(clock, attempt, parent, harvest_window).await;
            None
        }
        result = attempt => {
            if deadline::deadline_elapsed(clock, deadline) {
                let (_result, diagnostics) = result;
                parent.merge_hedge_attempt(diagnostics);
                None
            } else {
                Some(result)
            }
        },
    }
}

/// Tests the operation deadline in the injected clock's domain; an absent deadline never expires.
pub(crate) fn deadline_elapsed(clock: &crate::clock::Clock, deadline: Option<Instant>) -> bool {
    deadline.is_some_and(|d| clock.now() >= d)
}

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };

    use futures::{future, poll};

    use super::{deadline, DiagnosticsContextBuilder, Duration};
    use crate::{clock::Clock, models::ActivityId, options::DiagnosticsOptions};

    /// Creates an empty journal without starting a transport attempt.
    fn diagnostics() -> DiagnosticsContextBuilder {
        DiagnosticsContextBuilder::new(
            ActivityId::from_string("operation-clock".to_owned()),
            Arc::new(DiagnosticsOptions::default()),
        )
    }

    #[tokio::test]
    async fn retry_wait_is_capped_by_injected_deadline() {
        let clock = Clock::new_test();
        let end = clock.now() + Duration::from_millis(10);
        let mut wait = Box::pin(deadline::apply_failover_delay(
            &clock,
            Some(Duration::from_secs(60)),
            Some(end),
        ));
        assert!(poll!(wait.as_mut()).is_pending());
        clock.advance(Duration::from_millis(9));
        assert!(poll!(wait.as_mut()).is_pending());
        clock.advance(Duration::from_millis(1));
        wait.await;
        assert!(deadline::deadline_elapsed(&clock, Some(end)));
        assert_eq!(clock.pending_sleep_count(), 0);
    }

    #[tokio::test]
    async fn absent_deadline_never_signals() {
        let clock = Clock::new_test();
        let mut signal = Box::pin(deadline::deadline_signal(&clock, None));
        assert!(poll!(signal.as_mut()).is_pending());
        clock.advance(Duration::from_secs(3600));
        assert!(poll!(signal.as_mut()).is_pending());
        assert_eq!(clock.pending_sleep_count(), 0);
    }

    #[tokio::test]
    async fn partner_deadline_wins_a_simultaneously_ready_result() {
        let clock = Clock::new_test();
        let end = clock.now() + Duration::from_millis(10);
        let attempt = Box::pin(async {
            clock.sleep(Duration::from_millis(10)).await;
            (42, self::diagnostics())
        });
        let mut parent = self::diagnostics();
        let mut wait = Box::pin(deadline::await_attempt_or_deadline_harvest(
            &clock,
            attempt,
            Some(end),
            &mut parent,
            Duration::from_millis(1),
        ));
        assert!(poll!(wait.as_mut()).is_pending());
        clock.advance(Duration::from_millis(10));
        assert!(wait.await.is_none());
        assert_eq!(clock.pending_sleep_count(), 0);
    }

    #[tokio::test]
    async fn harvest_polls_a_ready_result_before_a_zero_timer() {
        let clock = Clock::new_test();
        let polled = AtomicBool::new(false);
        let attempt = Box::pin(async {
            polled.store(true, Ordering::Relaxed);
            ((), self::diagnostics())
        });
        deadline::harvest_remaining_attempt(
            &clock,
            attempt,
            &mut self::diagnostics(),
            Duration::ZERO,
        )
        .await;
        assert!(polled.load(Ordering::Relaxed));
        assert_eq!(clock.pending_sleep_count(), 0);
    }

    struct DropSignal<'a>(&'a AtomicBool);

    impl Drop for DropSignal<'_> {
        /// Records structural cancellation of the unfinished hedge.
        fn drop(&mut self) {
            self.0.store(true, Ordering::Relaxed);
        }
    }

    #[tokio::test]
    async fn harvest_cancels_pending_leg_at_the_configured_boundary() {
        let clock = Clock::new_test();
        let dropped = AtomicBool::new(false);
        let guard = DropSignal(&dropped);
        let attempt = Box::pin(async move {
            let _guard = guard;
            future::pending::<((), DiagnosticsContextBuilder)>().await
        });
        let mut parent = self::diagnostics();
        let mut harvest = Box::pin(deadline::harvest_remaining_attempt(
            &clock,
            attempt,
            &mut parent,
            Duration::from_millis(7),
        ));
        assert!(poll!(harvest.as_mut()).is_pending());
        clock.advance(Duration::from_millis(6));
        assert!(poll!(harvest.as_mut()).is_pending());
        assert!(!dropped.load(Ordering::Relaxed));
        clock.advance(Duration::from_millis(1));
        harvest.await;
        assert!(dropped.load(Ordering::Relaxed));
        assert_eq!(clock.pending_sleep_count(), 0);
    }
}
