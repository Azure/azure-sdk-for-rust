// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Monotonic time and sleeps with test-controlled clocks.

#[cfg(test)]
use std::sync::Arc;
use std::time::{Duration, Instant};
#[cfg(test)]
use tokio::sync::watch;

/// A real clock by default, or a shared, manually advanced clock in unit tests.
#[derive(Clone, Debug, Default)]
pub(crate) struct Clock(ClockKind);

#[derive(Clone, Debug, Default)]
enum ClockKind {
    #[default]
    Real,
    #[cfg(test)]
    Fake(Arc<FakeClock>),
}

#[cfg(not(test))]
const _: () = assert!(std::mem::size_of::<Clock>() == 0);

#[cfg(test)]
#[derive(Debug)]
struct FakeClock {
    time: watch::Sender<Instant>,
}

impl Clock {
    /// Returns the current monotonic time from this clock.
    pub fn now(&self) -> Instant {
        match &self.0 {
            ClockKind::Real => Instant::now(),
            #[cfg(test)]
            ClockKind::Fake(inner) => *inner.time.borrow(),
        }
    }

    /// Sleeps from the first poll until this clock advances by `duration`.
    ///
    /// Durations outside the runtime timer's supported range use a zero-duration sleep.
    pub async fn sleep(&self, duration: Duration) {
        let duration = azure_core::time::Duration::try_from(duration)
            .unwrap_or(azure_core::time::Duration::ZERO);
        match &self.0 {
            ClockKind::Real => azure_core::sleep(duration).await,
            #[cfg(test)]
            ClockKind::Fake(inner) => {
                let mut updates = inner.time.subscribe();
                let deadline = *updates.borrow()
                    + Duration::try_from(duration).expect("sleep duration is nonnegative");
                updates
                    .wait_for(|now| *now >= deadline)
                    .await
                    .expect("the clock remains alive while sleeping");
            }
        }
    }

    /// Creates an independent fake clock that moves only when advanced; clones share its time.
    #[cfg(test)]
    pub fn new_test() -> Self {
        Self(ClockKind::Fake(Arc::new(FakeClock {
            time: watch::channel(Instant::now()).0,
        })))
    }

    /// Moves a fake clock forward and wakes pending sleeps to recheck their deadlines.
    ///
    /// Panics for a real clock or if the resulting instant cannot be represented.
    #[cfg(test)]
    pub fn advance(&self, duration: Duration) {
        self.fake().time.send_modify(|now| *now += duration);
    }

    /// Returns the number of registered fake-clock sleeps. Panics for a real clock.
    #[cfg(test)]
    pub fn pending_sleep_count(&self) -> usize {
        self.fake().time.receiver_count()
    }

    /// Borrows the fake clock state, rejecting test controls on a real clock.
    #[cfg(test)]
    fn fake(&self) -> &FakeClock {
        match &self.0 {
            ClockKind::Real => panic!("test controls require a fake clock"),
            ClockKind::Fake(inner) => inner,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Clock, Duration};
    use futures::task::{self, ArcWake};
    use std::{
        future::Future,
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        task::Context,
    };

    #[derive(Default)]
    struct WakeSignal(AtomicBool);

    impl ArcWake for WakeSignal {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            arc_self.0.store(true, Ordering::SeqCst);
        }
    }

    #[test]
    fn clones_share_time_but_new_clocks_are_independent() {
        let clock = Clock::new_test();
        let clone = clock.clone();
        let independent = Clock::new_test();
        let start = clock.now();
        let other_start = independent.now();

        clone.advance(Duration::from_secs(5));

        assert_eq!(clock.now(), start + Duration::from_secs(5));
        assert_eq!(clone.now(), clock.now());
        assert_eq!(independent.now(), other_start);
    }

    #[test]
    fn advancing_wakes_sleepers_and_releases_only_elapsed_deadlines() {
        let clock = Clock::new_test();
        let clone = clock.clone();
        let signal = Arc::new(WakeSignal::default());
        let waker = task::waker(signal.clone());
        let mut context = Context::from_waker(&waker);
        let mut first = Box::pin(clock.sleep(Duration::from_secs(1)));
        let mut second = Box::pin(clone.sleep(Duration::from_secs(2)));
        assert!(first.as_mut().poll(&mut context).is_pending());
        assert!(second.as_mut().poll(&mut context).is_pending());
        assert_eq!(clock.pending_sleep_count(), 2);

        clock.advance(Duration::from_millis(999));
        assert!(signal.0.swap(false, Ordering::SeqCst));
        assert!(first.as_mut().poll(&mut context).is_pending());
        assert!(second.as_mut().poll(&mut context).is_pending());

        clone.advance(Duration::from_millis(1));
        assert!(signal.0.swap(false, Ordering::SeqCst));
        assert!(first.as_mut().poll(&mut context).is_ready());
        assert!(second.as_mut().poll(&mut context).is_pending());
        assert_eq!(clock.pending_sleep_count(), 1);

        clock.advance(Duration::from_secs(1));
        assert!(signal.0.load(Ordering::SeqCst));
        assert!(second.as_mut().poll(&mut context).is_ready());
        assert_eq!(clock.pending_sleep_count(), 0);
    }

    #[tokio::test]
    async fn sleeps_start_when_polled_and_cancellation_removes_registration() {
        let clock = Clock::new_test();
        {
            let mut sleep = Box::pin(clock.sleep(Duration::from_secs(1)));
            clock.advance(Duration::from_secs(2));
            assert_eq!(clock.pending_sleep_count(), 0);
            assert!(futures::poll!(sleep.as_mut()).is_pending());
            assert_eq!(clock.pending_sleep_count(), 1);
        }
        assert_eq!(clock.pending_sleep_count(), 0);
        clock.advance(Duration::from_secs(1));
        clock.sleep(Duration::ZERO).await;
        clock.sleep(Duration::MAX).await;
        assert_eq!(clock.pending_sleep_count(), 0);
    }

    #[test]
    #[should_panic(expected = "test controls require a fake clock")]
    fn real_clock_rejects_advancement() {
        Clock::default().advance(Duration::ZERO);
    }

    #[test]
    #[should_panic(expected = "test controls require a fake clock")]
    fn real_clock_rejects_sleep_inspection() {
        Clock::default().pending_sleep_count();
    }
}
