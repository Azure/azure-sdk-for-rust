// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Deadline checks, remaining time, and bounded retry waits.

use std::time::{Duration, Instant};

const DEADLINE_RETRY_SAFETY_MARGIN: Duration = Duration::from_millis(100);
const MIN_REMAINING_REQUEST_TIMEOUT: Duration = Duration::from_millis(1);

/// An optional operation deadline, evaluated against a caller-supplied instant.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Deadline(Option<Instant>);

impl From<Option<Instant>> for Deadline {
    #[inline(always)]
    fn from(deadline: Option<Instant>) -> Self {
        Self(deadline)
    }
}

impl Deadline {
    /// Checks whether `now` has reached the deadline; no deadline never expires.
    #[inline(always)]
    pub fn exceeded(self, now: Instant) -> bool {
        self.0.is_some_and(|deadline| now >= deadline)
    }

    /// Returns time remaining at `now`, saturating at zero, or `None` without a deadline.
    #[inline(always)]
    pub fn remaining(self, now: Instant) -> Option<Duration> {
        self.0
            .map(|deadline| deadline.saturating_duration_since(now))
    }

    /// Returns the remaining time floored at 1 ms for dispatch, or `None` without a deadline.
    #[inline(always)]
    pub fn request_timeout(self, now: Instant) -> Option<Duration> {
        self.remaining(now)
            .map(|remaining| remaining.max(MIN_REMAINING_REQUEST_TIMEOUT))
    }

    /// Caps a retry delay to leave 100 ms after `now` for another attempt; no deadline leaves it unchanged.
    #[inline(always)]
    pub fn cap_delay(self, requested: Duration, now: Instant) -> Duration {
        self.remaining(now).map_or(requested, |remaining| {
            requested.min(remaining.saturating_sub(DEADLINE_RETRY_SAFETY_MARGIN))
        })
    }

    /// Returns half the time remaining at `now` for a forced-final wait, skipping waits below 100 ms.
    ///
    /// No deadline means an immediate retry; an expired deadline returns `None`.
    // Inline the small wait selection.
    #[inline(always)]
    pub fn forced_final_delay(self, now: Instant) -> Option<Duration> {
        match self.remaining(now) {
            None => Some(Duration::ZERO),
            Some(remaining) if remaining.is_zero() => None,
            Some(remaining) => {
                let half = remaining / 2;
                Some(if half < DEADLINE_RETRY_SAFETY_MARGIN {
                    Duration::ZERO
                } else {
                    half
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Deadline;
    use std::time::{Duration, Instant};

    #[test]
    fn ordinary_delay_preserves_a_saturating_margin() {
        let now = Instant::now();
        let requested = Duration::from_millis(500);
        assert_eq!(Deadline::default().cap_delay(requested, now), requested);
        for (remaining, expected) in [(0, 0), (50, 0), (100, 0), (250, 150), (1000, 500)] {
            let deadline = Deadline::from(Some(now + Duration::from_millis(remaining)));
            assert_eq!(
                deadline.cap_delay(requested, now),
                Duration::from_millis(expected),
            );
        }
    }

    #[test]
    fn forced_final_delay_boundaries() {
        let now = Instant::now();
        assert_eq!(
            Deadline::default().forced_final_delay(now),
            Some(Duration::ZERO)
        );
        assert_eq!(Deadline::from(Some(now)).forced_final_delay(now), None);
        for (remaining, expected) in [(50, 0), (150, 0), (199, 0), (200, 100), (400, 200)] {
            let deadline = Deadline::from(Some(now + Duration::from_millis(remaining)));
            assert_eq!(
                deadline.forced_final_delay(now),
                Some(Duration::from_millis(expected)),
            );
        }
    }

    #[test]
    fn remaining_timeout_has_a_one_millisecond_floor() {
        let now = Instant::now();
        assert_eq!(Deadline::default().request_timeout(now), None);
        for remaining in [
            Duration::ZERO,
            Duration::from_micros(500),
            Duration::from_secs(1),
        ] {
            let deadline = Deadline::from(Some(now + remaining));
            assert_eq!(
                deadline.request_timeout(now),
                Some(remaining.max(Duration::from_millis(1)))
            );
        }
    }

    #[test]
    fn expired_remaining_time_is_zero_before_applying_the_dispatch_floor() {
        let now = Instant::now();
        let expired = Deadline::from(Some(now - Duration::from_secs(1)));
        assert!(expired.exceeded(now));
        assert_eq!(expired.remaining(now), Some(Duration::ZERO));
        assert_eq!(expired.request_timeout(now), Some(Duration::from_millis(1)));
        assert_eq!(
            expired.cap_delay(Duration::from_secs(1), now),
            Duration::ZERO
        );
        assert_eq!(expired.forced_final_delay(now), None);
        assert!(!Deadline::default().exceeded(now));
        assert_eq!(Deadline::default().remaining(now), None);
    }

    #[test]
    fn deadline_checks_use_the_supplied_instant() {
        let now = Instant::now();
        let expires = now + Duration::from_secs(1);
        let deadline = Deadline::from(Some(expires));
        assert!(!deadline.exceeded(now));
        assert_eq!(deadline.remaining(now), Some(Duration::from_secs(1)));
        assert!(deadline.exceeded(expires));
        assert_eq!(deadline.remaining(expires), Some(Duration::ZERO));
    }
}
