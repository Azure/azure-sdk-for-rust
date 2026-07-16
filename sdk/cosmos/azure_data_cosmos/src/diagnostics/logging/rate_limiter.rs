// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! A count-per-interval rate limiter with a failure reserve.
//!
//! [`RateLimiter`] caps how many diagnostics emissions are allowed per fixed
//! time window during a storm, while always letting a bounded number of
//! *failures* through even after the normal budget is exhausted. When emissions
//! are suppressed within a window, the limiter surfaces a single
//! "suppressed N until reset" notice — exactly once per window — the first time
//! it is consulted after that window ends.
//!
//! The limiter is deterministic and clock-injectable ([`RateLimiter::check`]
//! takes the current [`Instant`]), so its behavior can be unit-tested without
//! sleeping.

use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Default number of sampled diagnostics lines allowed per window (~100/min).
pub(crate) const DEFAULT_MAX_PER_WINDOW: u32 = 100;

/// Default rate-limiting window length.
pub(crate) const DEFAULT_WINDOW: Duration = Duration::from_secs(60);

/// Default number of failures always allowed per window, even past the cap.
pub(crate) const DEFAULT_FAILURE_RESERVE: u32 = 10;

/// Configuration for a [`RateLimiter`].
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct RateLimiterConfig {
    /// Maximum number of emissions allowed per [`window`](Self::window).
    pub max_per_window: u32,
    /// Length of a rate-limiting window.
    pub window: Duration,
    /// Number of failures permitted per window *in addition to* the normal cap,
    /// so a bounded number of failures is always logged during a storm.
    pub failure_reserve: u32,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            max_per_window: DEFAULT_MAX_PER_WINDOW,
            window: DEFAULT_WINDOW,
            failure_reserve: DEFAULT_FAILURE_RESERVE,
        }
    }
}

/// The outcome of a [`RateLimiter::check`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct LimitDecision {
    /// Whether the caller should emit for this event.
    pub emit: bool,
    /// When `Some(n)`, the caller should emit exactly one "suppressed `n` until
    /// reset" notice for the window that just ended.
    pub suppression_notice: Option<u32>,
}

#[derive(Debug)]
struct State {
    window_start: Instant,
    /// Total emissions this window (including reserve failures).
    emitted: u32,
    /// Failures emitted this window (counts toward the failure reserve).
    failures_emitted: u32,
    /// Emissions suppressed this window.
    suppressed: u32,
}

/// A count-per-interval limiter with a failure reserve. Cheap to share behind an
/// `Arc`; internally synchronized.
#[derive(Debug)]
pub(crate) struct RateLimiter {
    config: RateLimiterConfig,
    state: Mutex<State>,
}

impl RateLimiter {
    /// Creates a limiter with the given configuration, anchored at "now".
    pub(crate) fn new(config: RateLimiterConfig) -> Self {
        Self {
            config,
            state: Mutex::new(State {
                window_start: Instant::now(),
                emitted: 0,
                failures_emitted: 0,
                suppressed: 0,
            }),
        }
    }

    /// Records an emission attempt at `now` and returns whether it is allowed.
    ///
    /// `is_failure` marks the event as a failure, which may be admitted from the
    /// failure reserve once the normal per-window budget is exhausted.
    ///
    /// When the call is the first after a window rolled over and the previous
    /// window suppressed anything, the returned [`LimitDecision::suppression_notice`]
    /// carries that window's suppressed count so the caller can emit a single
    /// notice.
    pub(crate) fn check(&self, is_failure: bool, now: Instant) -> LimitDecision {
        let mut state = self.state.lock().unwrap();

        let mut suppression_notice = None;
        if now.saturating_duration_since(state.window_start) >= self.config.window {
            if state.suppressed > 0 {
                suppression_notice = Some(state.suppressed);
            }
            state.window_start = now;
            state.emitted = 0;
            state.failures_emitted = 0;
            state.suppressed = 0;
        }

        let within_normal_budget = state.emitted < self.config.max_per_window;
        // Past the normal cap, a bounded number of failures may still pass from
        // the reserve. Only reserve admissions count against `failures_emitted`,
        // so the reserve is genuinely *in addition to* the cap (a cap filled
        // with failures must not consume it).
        let from_reserve = !within_normal_budget
            && is_failure
            && state.failures_emitted < self.config.failure_reserve;
        let allowed = within_normal_budget || from_reserve;

        if allowed {
            state.emitted += 1;
            if from_reserve {
                state.failures_emitted += 1;
            }
            LimitDecision {
                emit: true,
                suppression_notice,
            }
        } else {
            state.suppressed += 1;
            LimitDecision {
                emit: false,
                suppression_notice,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caps_emissions_within_a_window() {
        let limiter = RateLimiter::new(RateLimiterConfig {
            max_per_window: 5,
            window: Duration::from_millis(100),
            failure_reserve: 0,
        });
        let t0 = Instant::now();

        let mut emitted = 0;
        let mut notices = 0;
        for _ in 0..50 {
            let d = limiter.check(false, t0);
            if d.emit {
                emitted += 1;
            }
            if d.suppression_notice.is_some() {
                notices += 1;
            }
        }

        // Only the cap is admitted; the rest are suppressed and no notice has
        // fired yet (the window has not rolled over).
        assert_eq!(emitted, 5);
        assert_eq!(notices, 0);
    }

    #[test]
    fn emits_exactly_one_suppression_notice_per_window() {
        let limiter = RateLimiter::new(RateLimiterConfig {
            max_per_window: 5,
            window: Duration::from_millis(100),
            failure_reserve: 0,
        });
        let t0 = Instant::now();
        for _ in 0..50 {
            limiter.check(false, t0);
        }

        // First check after the window ends carries the suppressed count once.
        let t1 = t0 + Duration::from_millis(150);
        let first = limiter.check(false, t1);
        assert!(first.emit, "new window admits again");
        assert_eq!(first.suppression_notice, Some(45), "50 - 5 suppressed");

        // Subsequent checks in the new window do not repeat the notice.
        let second = limiter.check(false, t1);
        assert_eq!(second.suppression_notice, None);
    }

    #[test]
    fn failures_admitted_from_reserve_past_cap() {
        let limiter = RateLimiter::new(RateLimiterConfig {
            max_per_window: 2,
            window: Duration::from_secs(60),
            failure_reserve: 3,
        });
        let t = Instant::now();

        // Fill the normal cap with successes.
        assert!(limiter.check(false, t).emit);
        assert!(limiter.check(false, t).emit);
        // Further successes are suppressed.
        assert!(!limiter.check(false, t).emit);
        // Failures still pass, up to the reserve of 3.
        assert!(limiter.check(true, t).emit);
        assert!(limiter.check(true, t).emit);
        assert!(limiter.check(true, t).emit);
        // Reserve exhausted: further failures are suppressed.
        assert!(!limiter.check(true, t).emit);
    }

    #[test]
    fn reserve_is_additional_to_a_cap_filled_with_failures() {
        let limiter = RateLimiter::new(RateLimiterConfig {
            max_per_window: 2,
            window: Duration::from_secs(60),
            failure_reserve: 3,
        });
        let t = Instant::now();

        // Fill the normal cap with failures. These are admitted by the normal
        // budget, so they must NOT consume the failure reserve.
        assert!(limiter.check(true, t).emit);
        assert!(limiter.check(true, t).emit);
        // The full reserve of 3 is still available for post-cap failures.
        assert!(limiter.check(true, t).emit);
        assert!(limiter.check(true, t).emit);
        assert!(limiter.check(true, t).emit);
        // Reserve now exhausted.
        assert!(!limiter.check(true, t).emit);
    }
}
