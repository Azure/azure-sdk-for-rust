// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Races a future against an injected clock's timer.

use crate::pipeline::Clock;
use futures::{
    future::{self, Either},
    pin_mut,
};
use std::{future::Future, time::Duration};

/// The timer completed before the supplied future produced a value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TimedOut;

/// Returns the future's output, or [`TimedOut`] when the timer completes first.
///
/// Polls the future before the timer, even for zero durations, and drops the losing future.
/// Durations outside the timer's supported range use a zero-duration timer.
pub(crate) async fn with_timeout<F: Future>(
    duration: Duration,
    future: F,
    clock: Clock,
) -> Result<F::Output, TimedOut> {
    let timer = clock.sleep(duration);
    // `select` needs pinned futures: async futures may not be Unpin.
    // `pin_mut!` keeps these locals at stable addresses until they are dropped.
    pin_mut!(future, timer);
    match future::select(future, timer).await {
        Either::Left((output, _)) => Ok(output),
        Either::Right(((), _)) => Err(TimedOut),
    }
}

#[cfg(test)]
mod tests {
    use super::{with_timeout, Clock, Duration, TimedOut};
    use futures::future;
    use std::cell::Cell;

    struct DropSignal<'a>(&'a Cell<bool>);

    impl Drop for DropSignal<'_> {
        fn drop(&mut self) {
            self.0.set(true);
        }
    }

    #[tokio::test]
    async fn ready_borrowing_future_wins_even_with_zero_duration() {
        let mut calls = 0;
        let work = async {
            calls += 1;
            Err::<(), _>("operation failed")
        };
        assert_eq!(
            with_timeout(Duration::ZERO, work, Clock::new_test()).await,
            Ok(Err("operation failed"))
        );
        assert_eq!(calls, 1);
    }

    #[tokio::test]
    async fn returns_output_when_work_finishes_no_later_than_timer() {
        for duration in [Duration::from_millis(5), Duration::from_millis(10)] {
            let clock = Clock::new_test();
            let work = async {
                clock.sleep(Duration::from_millis(5)).await;
                42
            };
            let mut race = Box::pin(with_timeout(duration, work, clock.clone()));
            assert!(futures::poll!(race.as_mut()).is_pending());
            assert_eq!(clock.pending_sleep_count(), 2);
            clock.advance(Duration::from_millis(5));
            assert_eq!(race.await, Ok(42));
            assert_eq!(clock.pending_sleep_count(), 0);
        }
    }

    #[tokio::test(start_paused = true)]
    async fn unsupported_duration_uses_zero_duration_timer() {
        for clock in [Clock::default(), Clock::new_test()] {
            assert_eq!(
                with_timeout(Duration::MAX, future::pending::<()>(), clock).await,
                Err(TimedOut)
            );
        }
    }

    #[tokio::test]
    async fn timeout_drops_unfinished_work() {
        let clock = Clock::new_test();
        let dropped = Cell::new(false);
        let guard = DropSignal(&dropped);
        let work = async move {
            let _guard = guard;
            future::pending::<()>().await
        };
        let mut race = Box::pin(with_timeout(Duration::from_millis(10), work, clock.clone()));
        assert!(futures::poll!(race.as_mut()).is_pending());
        clock.advance(Duration::from_millis(10));
        assert_eq!(race.await, Err(TimedOut));
        assert!(dropped.get());
        assert_eq!(clock.pending_sleep_count(), 0);
    }

    #[tokio::test]
    async fn cancelling_the_race_drops_unfinished_work() {
        let clock = Clock::new_test();
        let dropped = Cell::new(false);
        let guard = DropSignal(&dropped);
        let work = async move {
            let _guard = guard;
            future::pending::<()>().await
        };
        let mut race = Box::pin(with_timeout(Duration::from_secs(60), work, clock.clone()));
        assert!(futures::poll!(race.as_mut()).is_pending());
        assert_eq!(clock.pending_sleep_count(), 1);
        drop(race);
        assert!(dropped.get());
        assert_eq!(clock.pending_sleep_count(), 0);
    }
}
