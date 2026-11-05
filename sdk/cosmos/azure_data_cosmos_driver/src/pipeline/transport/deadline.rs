// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Deadline checks, remaining time, and bounded retry waits.

use std::time::{Duration, Instant};

const DEADLINE_RETRY_SAFETY_MARGIN: Duration = Duration::from_millis(100);
const MIN_REMAINING_REQUEST_TIMEOUT: Duration = Duration::from_millis(1);

/// Checks whether the deadline has passed; no deadline never expires.
// Inline the optional clock check.
#[inline(always)]
pub(super) fn exceeded(deadline: Option<Instant>) -> bool {
    deadline.is_some_and(|deadline| Instant::now() >= deadline)
}

/// Returns the remaining time, saturating at zero, or `None` when there is no deadline.
// Inline the optional clock sample.
#[inline(always)]
pub(super) fn remaining(deadline: Option<Instant>) -> Option<Duration> {
    deadline.map(|deadline| deadline.saturating_duration_since(Instant::now()))
}

/// Floors the sampled remaining time at one millisecond for dispatch.
// Inline the small duration adjustment.
#[inline(always)]
pub(super) fn request_timeout(remaining: Duration) -> Duration {
    remaining.max(MIN_REMAINING_REQUEST_TIMEOUT)
}

/// Caps a retry delay to leave 100 ms for another attempt, or zero when less time remains.
// Inline the small duration adjustment.
#[inline(always)]
pub(super) fn cap_delay(requested: Duration, remaining: Duration) -> Duration {
    requested.min(remaining.saturating_sub(DEADLINE_RETRY_SAFETY_MARGIN))
}

/// Returns half the remaining time for a forced-final wait, skipping waits below 100 ms.
///
/// No deadline means an immediate retry; an expired deadline returns `None`.
// Inline the small wait selection.
#[inline(always)]
pub(super) fn forced_final_delay(remaining: Option<Duration>) -> Option<Duration> {
    match remaining {
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

/// Sleeps for the retry delay, using zero if it exceeds the timer's supported range.
// Inline the thin timer adapter.
#[inline(always)]
pub(super) async fn sleep(delay: Duration) {
    azure_core::sleep(
        azure_core::time::Duration::try_from(delay).unwrap_or(azure_core::time::Duration::ZERO),
    )
    .await;
}

#[cfg(test)]
mod tests {
    use super::{cap_delay, exceeded, forced_final_delay, remaining, request_timeout};
    use std::time::{Duration, Instant};

    #[test]
    fn ordinary_delay_preserves_a_saturating_margin() {
        for (remaining, expected) in [(0, 0), (50, 0), (100, 0), (250, 150), (1000, 500)] {
            assert_eq!(
                cap_delay(Duration::from_millis(500), Duration::from_millis(remaining)),
                Duration::from_millis(expected),
            );
        }
    }

    #[test]
    fn forced_final_delay_boundaries() {
        assert_eq!(forced_final_delay(None), Some(Duration::ZERO));
        assert_eq!(forced_final_delay(Some(Duration::ZERO)), None);
        for (remaining, expected) in [(50, 0), (150, 0), (199, 0), (200, 100), (400, 200)] {
            assert_eq!(
                forced_final_delay(Some(Duration::from_millis(remaining))),
                Some(Duration::from_millis(expected)),
            );
        }
    }

    #[test]
    fn remaining_timeout_has_a_one_millisecond_floor() {
        assert_eq!(request_timeout(Duration::ZERO), Duration::from_millis(1));
        assert_eq!(
            request_timeout(Duration::from_secs(1)),
            Duration::from_secs(1)
        );
    }

    #[test]
    fn expired_remaining_time_is_zero_before_applying_the_dispatch_floor() {
        let expired = Some(Instant::now() - Duration::from_secs(1));
        assert!(exceeded(expired));
        assert_eq!(remaining(expired), Some(Duration::ZERO));
        assert!(!exceeded(None));
        assert_eq!(remaining(None), None);
    }
}
