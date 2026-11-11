// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{error, sleep};
use crate::{
    change_feed_processor::operation_options,
    driver::pipeline::components::{
        ThrottleRetryState, DATA_PLANE_MAX_PER_RETRY_DELAY, DATA_PLANE_MAX_THROTTLE_ATTEMPTS,
        DATA_PLANE_MAX_THROTTLE_WAIT,
    },
    error::status_codes::{self, substatus::SYSTEM_RESOURCE_UNAVAILABLE},
    options::{OperationOptions, ThrottlingRetryOptions},
    CosmosDriver, CosmosError, CosmosStatus, Result,
};
use azure_core::http::StatusCode;
use futures::FutureExt;
use std::{
    future::Future,
    time::{Duration, Instant},
};

const READ_TURN_TIMEOUT: Duration = Duration::from_secs(5);

pub(super) struct ReadRetry {
    state: ThrottleRetryState,
}

pub(super) struct Cooldown {
    pub until: Instant,
    pub retry: bool,
}

impl ReadRetry {
    pub fn for_driver(driver: &CosmosDriver) -> (Self, OperationOptions, Duration) {
        let mut options = operation_options();
        let view = driver.operation_options_view(&options);
        let throttling = view.throttling_retry_options();
        let state = ThrottleRetryState::with_limits(
            throttling
                .max_retry_count()
                .copied()
                .unwrap_or(DATA_PLANE_MAX_THROTTLE_ATTEMPTS),
            throttling
                .max_retry_wait_time()
                .copied()
                .unwrap_or(DATA_PLANE_MAX_THROTTLE_WAIT),
            DATA_PLANE_MAX_PER_RETRY_DELAY,
        );
        let timeout = view
            .end_to_end_latency_policy()
            .map(|policy| policy.timeout().min(READ_TURN_TIMEOUT))
            .unwrap_or(READ_TURN_TIMEOUT);
        // Only CFP page reads defer throttling; metadata and lease writes keep their retries.
        options.throttling_retry_options = Some(ThrottlingRetryOptions {
            max_retry_count: Some(0),
            ..Default::default()
        });
        options.end_to_end_latency_policy = Some(timeout.into());
        (Self { state }, options, timeout)
    }

    pub fn is_throttled(error: &CosmosError) -> bool {
        error.status().is_throttled()
            && error.status().sub_status() != Some(SYSTEM_RESOURCE_UNAVAILABLE)
    }

    pub fn cooldown(&mut self, retry_after_ms: Option<u64>, now: Instant) -> Result<Cooldown> {
        let delay = match retry_after_ms.filter(|delay| *delay > 0) {
            Some(delay) => Duration::from_millis(delay),
            None => {
                let mut fallback = self.state.clone();
                fallback.attempt_count = fallback.attempt_count.min(32);
                fallback
                    .fallback_delay()
                    .min(DATA_PLANE_MAX_PER_RETRY_DELAY)
            }
        };
        let delay_ms = u64::try_from(delay.as_millis())
            .map_err(|_| {
                error(
                    StatusCode::BadRequest,
                    "Change feed retry delay is not representable.",
                )
            })?
            .max(1);
        let delay = Duration::from_millis(delay_ms);
        let until = now.checked_add(delay).ok_or_else(|| {
            error(
                StatusCode::BadRequest,
                "Change feed retry delay is not representable.",
            )
        })?;
        // Cooldown does not occupy receive demand, so do not shorten a service Retry-After.
        let mut policy = self.state.clone();
        policy.max_per_retry_delay = delay;
        let next = policy
            .cumulative_delay
            .checked_add(delay)
            .and_then(|_| policy.next_throttle_retry(Some(delay_ms)));
        let retry = next.is_some();
        if let Some((_, next)) = next {
            self.state = next;
        }
        Ok(Cooldown { until, retry })
    }

    pub fn reset(&mut self) {
        self.state.attempt_count = 0;
        self.state.cumulative_delay = Duration::ZERO;
    }

    pub async fn bounded<T>(deadline: Instant, work: impl Future<Output = Result<T>>) -> Result<T> {
        let work = work.fuse();
        let elapsed = sleep(deadline.saturating_duration_since(Instant::now())).fuse();
        futures::pin_mut!(work, elapsed);
        futures::select_biased! {
            _ = elapsed => Err(CosmosError::builder()
                .with_status(CosmosStatus::from_parts(
                    StatusCode::RequestTimeout, Some(status_codes::substatus::CLIENT_OPERATION_TIMEOUT),
                ))
                .with_message("Change feed read turn timed out.")
                .build()),
            result = work => result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ReadRetry, ThrottleRetryState};
    use crate::{CosmosError, CosmosStatus};
    use azure_core::http::StatusCode;
    use std::time::{Duration, Instant};

    #[test]
    fn system_resource_throttles_keep_routing_recovery() {
        let ordinary = CosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::TooManyRequests))
            .build();
        let routing = CosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::TooManyRequests).with_sub_status(3092))
            .build();
        assert!(ReadRetry::is_throttled(&ordinary));
        assert!(!ReadRetry::is_throttled(&routing));
    }

    #[test]
    fn throttle_budget_survives_turns_and_resets_only_after_success() {
        let mut retry = ReadRetry {
            state: ThrottleRetryState::with_limits(
                2,
                Duration::from_secs(60),
                Duration::from_secs(15),
            ),
        };
        let now = Instant::now();
        let first = retry.cooldown(Some(20_000), now).unwrap();
        assert!(first.retry);
        assert_eq!(first.until.duration_since(now), Duration::from_secs(20));
        assert!(retry.cooldown(Some(20_000), now).unwrap().retry);
        let exhausted = retry.cooldown(Some(20_000), now).unwrap();
        assert!(!exhausted.retry);
        assert_eq!(exhausted.until.duration_since(now), Duration::from_secs(20));
        retry.reset();
        assert!(retry.cooldown(Some(20_000), now).unwrap().retry);
    }

    #[test]
    fn wait_budget_exhaustion_still_honors_the_final_cooldown() {
        let mut retry = ReadRetry {
            state: ThrottleRetryState::with_limits(
                9,
                Duration::from_millis(10),
                Duration::from_secs(15),
            ),
        };
        let now = Instant::now();
        let cooldown = retry.cooldown(Some(20), now).unwrap();
        assert!(!cooldown.retry);
        assert_eq!(
            cooldown.until.duration_since(now),
            Duration::from_millis(20)
        );
    }

    #[test]
    fn missing_or_zero_retry_after_uses_bounded_positive_backoff() {
        let mut retry = ReadRetry {
            state: ThrottleRetryState::with_limits(
                9,
                Duration::from_secs(30),
                Duration::from_secs(15),
            ),
        };
        let now = Instant::now();
        for header in [None, Some(0)] {
            let cooldown = retry.cooldown(header, now).unwrap();
            assert!(cooldown.retry);
            assert!(cooldown.until > now);
            assert!(cooldown.until.duration_since(now) <= Duration::from_secs(15));
        }
    }
}
