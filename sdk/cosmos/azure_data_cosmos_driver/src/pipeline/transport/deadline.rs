// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Deadline decisions over explicitly supplied time.

use std::time::{Duration, Instant};

const DEADLINE_RETRY_SAFETY_MARGIN: Duration = Duration::from_millis(100);
const MIN_REMAINING_REQUEST_TIMEOUT: Duration = Duration::from_millis(1);

/// Returns the time remaining at `now`, with a one-millisecond minimum for dispatch.
pub(super) fn remaining_request_timeout(deadline: Instant, now: Instant) -> Duration {
    deadline
        .saturating_duration_since(now)
        .max(MIN_REMAINING_REQUEST_TIMEOUT)
}

/// Caps a retry delay to leave 100 ms for another attempt, or zero when less time remains.
pub(super) fn deadline_capped_delay(requested: Duration, remaining: Duration) -> Duration {
    requested.min(remaining.saturating_sub(DEADLINE_RETRY_SAFETY_MARGIN))
}

/// Returns half the remaining time for a forced-final wait, skipping waits below 100 ms.
///
/// No deadline means an immediate retry; an expired deadline returns `None`.
pub(super) fn forced_final_retry_delay(remaining: Option<Duration>) -> Option<Duration> {
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

#[cfg(test)]
mod tests {
    use super::{deadline_capped_delay, forced_final_retry_delay, remaining_request_timeout};
    use std::time::{Duration, Instant};

    #[test]
    fn ordinary_delay_preserves_a_saturating_margin() {
        for (remaining, expected) in [(0, 0), (50, 0), (100, 0), (250, 150), (1000, 500)] {
            assert_eq!(
                deadline_capped_delay(Duration::from_millis(500), Duration::from_millis(remaining)),
                Duration::from_millis(expected),
            );
        }
    }

    #[test]
    fn forced_final_delay_boundaries() {
        assert_eq!(forced_final_retry_delay(None), Some(Duration::ZERO));
        assert_eq!(forced_final_retry_delay(Some(Duration::ZERO)), None);
        for (remaining, expected) in [(50, 0), (150, 0), (199, 0), (200, 100), (400, 200)] {
            assert_eq!(
                forced_final_retry_delay(Some(Duration::from_millis(remaining))),
                Some(Duration::from_millis(expected)),
            );
        }
    }

    #[test]
    fn remaining_timeout_has_a_one_millisecond_floor() {
        let now = Instant::now();
        assert_eq!(
            remaining_request_timeout(now - Duration::from_secs(1), now),
            Duration::from_millis(1)
        );
        assert_eq!(
            remaining_request_timeout(now + Duration::from_secs(1), now),
            Duration::from_secs(1)
        );
    }
}
