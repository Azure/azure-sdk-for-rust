// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Recovers failed attempts against the same endpoint, using explicit retry limits and progress.

use std::time::Duration;

use crate::{
    clock::Clock,
    diagnostics::{ExecutionContext, FailedTransportShardDiagnostics, RequestSentStatus},
    driver::jitter,
    error::{status_codes, CosmosError},
    pipeline::RecoveryAction,
};

use super::{
    adaptive_transport::AdaptiveTransport,
    dispatch::{DispatchError, DispatchErrorDetails},
    types::{TransportOutcome, TransportRequest, TransportResult},
    TransportPipelineConfig,
};

pub(crate) const METADATA_MAX_THROTTLE_ATTEMPTS: u32 = 9;
pub(crate) const METADATA_MAX_THROTTLE_WAIT: Duration = Duration::from_secs(30);
pub(crate) const METADATA_MAX_PER_RETRY_DELAY: Duration = Duration::from_secs(5);
pub(crate) const DATA_PLANE_MAX_THROTTLE_ATTEMPTS: u32 = 18;
pub(crate) const DATA_PLANE_MAX_PER_RETRY_DELAY: Duration = Duration::from_secs(15);
pub(crate) const DATA_PLANE_MAX_THROTTLE_WAIT: Duration = DATA_PLANE_MAX_PER_RETRY_DELAY
    .checked_mul(DATA_PLANE_MAX_THROTTLE_ATTEMPTS)
    .expect("data-plane throttle wait budget overflows Duration");
pub(super) const MAX_LOCAL_CONNECTIVITY_RETRIES: u32 = 1;

/// Fixed throttle limits for one transport invocation.
#[derive(Clone, Debug)]
pub(crate) struct ThrottleRetryPolicy {
    pub max_attempts: u32,
    pub max_wait_time: Duration,
    pub max_per_retry_delay: Duration,
    pub fallback_base_delay: Duration,
    pub backoff_factor: f64,
    pub backoff_jitter_ratio: f64,
}

impl ThrottleRetryPolicy {
    /// Combines the supplied limits with the default exponential backoff and jitter settings.
    // Inline the small policy constructor.
    #[inline(always)]
    pub(crate) fn with_limits(
        max_attempts: u32,
        max_wait_time: Duration,
        max_per_retry_delay: Duration,
    ) -> Self {
        Self {
            max_attempts,
            max_wait_time,
            max_per_retry_delay,
            fallback_base_delay: Duration::from_millis(5),
            backoff_factor: 2.0,
            backoff_jitter_ratio: 0.25,
        }
    }

    /// Computes a bounded service or fallback delay without applying retry budgets.
    pub(crate) fn retry_delay(&self, attempt_count: u32, retry_after_ms: Option<u64>) -> Duration {
        let delay = match retry_after_ms {
            Some(ms) => Duration::from_millis(ms),
            None => {
                let multiplier = self
                    .backoff_factor
                    .powi(attempt_count.min(i32::MAX as u32) as i32);
                let seconds = (self.fallback_base_delay.as_secs_f64() * multiplier)
                    .min(self.max_per_retry_delay.as_secs_f64());
                let base = Duration::from_secs_f64(seconds);
                let ratio = self.backoff_jitter_ratio.clamp(0.0, 1.0);
                if ratio == 0.0 || base.is_zero() {
                    base
                } else {
                    Duration::from_secs_f64(jitter::with_jitter(base.as_secs_f64(), ratio))
                }
            }
        };
        delay.min(self.max_per_retry_delay)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct ThrottleRetryProgress {
    pub attempt_count: u32,
    /// Delay charged before the deadline cap reduces the actual sleep.
    pub cumulative_budgeted_delay: Duration,
    pub forced_final_retry_used: bool,
}

impl ThrottleRetryProgress {
    /// Selects a service or fallback delay and charges an ordinary retry against the budget.
    ///
    /// Returns `None` without changing progress when either limit is exhausted. The charged
    /// delay includes the per-retry cap, but not any later deadline cap.
    pub(crate) fn try_retry(
        &mut self,
        retry_after_ms: Option<u64>,
        policy: &ThrottleRetryPolicy,
    ) -> Option<Duration> {
        if self.attempt_count >= policy.max_attempts {
            return None;
        }
        let delay = policy.retry_delay(self.attempt_count, retry_after_ms);
        let cumulative = self.cumulative_budgeted_delay.saturating_add(delay);
        if cumulative > policy.max_wait_time {
            return None;
        }
        self.attempt_count += 1;
        self.cumulative_budgeted_delay = cumulative;
        Some(delay)
    }
}

#[derive(Default)]
pub(super) struct TransportRetryProgress {
    pub throttle: ThrottleRetryProgress,
    pub local_connectivity_retry_count: u32,
    pub excluded_shard_id: Option<u64>,
    pub failed_shards: Vec<FailedTransportShardDiagnostics>,
}

impl TransportRetryProgress {
    /// Chooses the attempt's diagnostic context, giving local shard retries precedence over throttling.
    #[inline(always)]
    pub(super) fn execution_context(&self, initial: ExecutionContext) -> ExecutionContext {
        if self.local_connectivity_retry_count > 0 {
            ExecutionContext::TransportRetry
        } else if self.throttle.attempt_count > 0 {
            ExecutionContext::OperationRetry
        } else {
            initial
        }
    }
}

/// Retries an eligible connectivity failure or 429 against the same endpoint, or returns its result.
///
/// Connectivity replay takes precedence; the execution loop checks the deadline before retrying.
pub(super) async fn recover(
    error: DispatchError,
    request: &TransportRequest,
    config: &TransportPipelineConfig<'_>,
    transport: &AdaptiveTransport,
    progress: &mut TransportRetryProgress,
    clock: Clock,
) -> RecoveryAction<TransportResult> {
    let details = DispatchErrorDetails::from(error);
    if let Some(action) = try_connectivity_retry(&details, config, transport, progress) {
        return action;
    }
    try_throttle_retry(
        details.result,
        request,
        &config.throttle_policy,
        progress,
        clock,
    )
    .await
}

/// Records an eligible failed shard and excludes it from the next dispatch.
///
/// Returns `None` without changing progress when replay is unsafe or no other shard is available.
pub(super) fn try_connectivity_retry(
    details: &DispatchErrorDetails,
    config: &TransportPipelineConfig<'_>,
    transport: &AdaptiveTransport,
    progress: &mut TransportRetryProgress,
) -> Option<RecoveryAction<TransportResult>> {
    let shard_id = details.shard_id?;
    if progress.local_connectivity_retry_count >= MAX_LOCAL_CONNECTIVITY_RETRIES
        || !should_retry_connectivity_failure(&details.result, config.allow_sent_transport_retry)
        || !transport.can_retry_on_different_shard(shard_id, &config.endpoint_key)
    {
        return None;
    }
    if let Some(failed) = details.failed_shard() {
        progress.failed_shards.push(failed);
    }
    progress.local_connectivity_retry_count += 1;
    progress.excluded_shard_id = Some(shard_id);
    Some(RecoveryAction::Retry)
}

/// Charges and waits for an eligible 429 retry, or returns the original result.
///
/// A forced-final retry only consumes its own allowance. Deadlines cap waits but do not
/// terminate recovery; the execution loop checks expiration before the next attempt.
async fn try_throttle_retry(
    result: TransportResult,
    request: &TransportRequest,
    policy: &ThrottleRetryPolicy,
    progress: &mut TransportRetryProgress,
    clock: Clock,
) -> RecoveryAction<TransportResult> {
    #[cfg(feature = "unstable_dtx")]
    let is_dtx = request.resource_type == crate::models::ResourceType::DistributedTransactionBatch;
    #[cfg(not(feature = "unstable_dtx"))]
    let is_dtx = false;

    if !result.outcome.is_retryable_throttle(is_dtx) {
        return RecoveryAction::Fail(result);
    }
    let retry_after_ms = result
        .cosmos_headers()
        .and_then(|headers| headers.retry_after_ms);
    if let Some(delay) = progress.throttle.try_retry(retry_after_ms, policy) {
        clock
            .sleep(request.deadline.cap_delay(delay, clock.now()))
            .await;
        return RecoveryAction::Retry;
    }
    if progress.throttle.attempt_count >= policy.max_attempts
        || progress.throttle.forced_final_retry_used
    {
        return RecoveryAction::Fail(result);
    }
    progress.throttle.forced_final_retry_used = true;
    let delay = request.deadline.forced_final_delay(clock.now());
    if !delay.is_zero() {
        clock.sleep(delay).await;
    }
    RecoveryAction::Retry
}

/// Checks for a synthetic connectivity failure whose sent status permits replay under caller policy.
pub(super) fn should_retry_connectivity_failure(
    result: &TransportResult,
    allow_sent_transport_retry: bool,
) -> bool {
    match &result.outcome {
        TransportOutcome::TransportError {
            error,
            request_sent,
            ..
        } => {
            is_connectivity_error(error)
                && (*request_sent == RequestSentStatus::NotSent || allow_sent_transport_retry)
        }
        _ => false,
    }
}

/// Recognizes client-generated connection, I/O, and timeout errors, excluding service responses.
fn is_connectivity_error(error: &CosmosError) -> bool {
    if error.is_from_wire() {
        return false;
    }
    matches!(
        error.status().sub_status(),
        Some(status_codes::substatus::TRANSPORT_GENERATED_503)
            | Some(status_codes::substatus::TRANSPORT_CONNECTION_FAILED)
            | Some(status_codes::substatus::TRANSPORT_IO_FAILED)
            | Some(status_codes::substatus::TRANSPORT_DNS_FAILED)
            | Some(status_codes::substatus::TRANSPORT_HTTP2_INCOMPATIBLE)
            | Some(status_codes::substatus::TRANSPORT_BODY_READ_FAILED)
            | Some(status_codes::substatus::CLIENT_OPERATION_TIMEOUT)
    )
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "unstable_dtx")]
    use super::TransportOutcome;
    use super::{
        try_throttle_retry, Clock, RecoveryAction, ThrottleRetryPolicy, ThrottleRetryProgress,
        TransportRetryProgress, DATA_PLANE_MAX_PER_RETRY_DELAY, DATA_PLANE_MAX_THROTTLE_ATTEMPTS,
        DATA_PLANE_MAX_THROTTLE_WAIT, METADATA_MAX_PER_RETRY_DELAY, METADATA_MAX_THROTTLE_ATTEMPTS,
        METADATA_MAX_THROTTLE_WAIT,
    };
    use crate::{
        diagnostics::ExecutionContext,
        error::{status_codes, CosmosStatus},
        models::CosmosResponseHeaders,
        pipeline::transport::{tests::test_request, types::TransportResult},
    };
    use std::{task::Poll, time::Duration};

    #[test]
    fn delay_sampling_preserves_jitter_bounds_and_disabled_cases() {
        for millis in [5, 10, 160] {
            let base = Duration::from_millis(millis);
            let mut policy = ThrottleRetryPolicy {
                fallback_base_delay: base,
                backoff_jitter_ratio: 0.0,
                ..metadata_policy()
            };
            assert_eq!(
                ThrottleRetryProgress::default().try_retry(Some(millis), &policy),
                Some(base)
            );
            assert_eq!(
                ThrottleRetryProgress::default().try_retry(None, &policy),
                Some(base)
            );
            policy.backoff_jitter_ratio = 0.25;
            let jittered = ThrottleRetryProgress::default()
                .try_retry(None, &policy)
                .unwrap();
            assert!(jittered >= base.mul_f64(0.75));
            assert!(jittered <= base.mul_f64(1.25));
        }
        assert_eq!(
            ThrottleRetryProgress::default().try_retry(
                None,
                &ThrottleRetryPolicy {
                    fallback_base_delay: Duration::ZERO,
                    ..metadata_policy()
                }
            ),
            Some(Duration::ZERO),
        );
    }

    #[test]
    fn extreme_retry_progress_keeps_backoff_and_budget_arithmetic_bounded() {
        let policy = ThrottleRetryPolicy {
            max_attempts: u32::MAX,
            max_wait_time: Duration::MAX,
            max_per_retry_delay: Duration::from_secs(5),
            backoff_jitter_ratio: 0.0,
            ..metadata_policy()
        };
        let mut progress = ThrottleRetryProgress {
            attempt_count: u32::MAX - 1,
            cumulative_budgeted_delay: Duration::MAX - Duration::from_secs(1),
            ..Default::default()
        };

        assert_eq!(
            progress.try_retry(None, &policy),
            Some(Duration::from_secs(5))
        );
        assert_eq!(progress.attempt_count, u32::MAX);
        assert_eq!(progress.cumulative_budgeted_delay, Duration::MAX);
        assert_eq!(progress.try_retry(None, &policy), None);
    }

    fn metadata_policy() -> ThrottleRetryPolicy {
        ThrottleRetryPolicy::with_limits(
            METADATA_MAX_THROTTLE_ATTEMPTS,
            METADATA_MAX_THROTTLE_WAIT,
            METADATA_MAX_PER_RETRY_DELAY,
        )
    }

    fn throttled(body: &[u8], retry_after_ms: Option<u64>) -> TransportResult {
        let headers = CosmosResponseHeaders {
            retry_after_ms,
            ..Default::default()
        };
        TransportResult::from_http_response(
            CosmosStatus::from_parts(
                azure_core::http::StatusCode::TooManyRequests,
                Some(status_codes::substatus::RU_BUDGET_EXCEEDED),
            ),
            headers,
            body.to_vec(),
        )
    }

    #[test]
    fn service_delay_takes_precedence_over_fallback() {
        let policy = ThrottleRetryPolicy {
            backoff_jitter_ratio: 0.0,
            ..metadata_policy()
        };
        assert_eq!(
            ThrottleRetryProgress::default().try_retry(Some(42), &policy),
            Some(Duration::from_millis(42)),
        );
        for (attempt_count, millis) in [(0, 5), (1, 10), (5, 160)] {
            let mut progress = ThrottleRetryProgress {
                attempt_count,
                ..Default::default()
            };
            assert_eq!(
                progress.try_retry(None, &policy),
                Some(Duration::from_millis(millis)),
            );
            assert_eq!(progress.attempt_count, attempt_count + 1);
            assert_eq!(
                progress.cumulative_budgeted_delay,
                Duration::from_millis(millis)
            );
        }
    }

    #[test]
    fn count_limits_stop_before_observing_a_delay() {
        for max_attempts in [0, 1, 2, 9, 18] {
            let policy = ThrottleRetryPolicy {
                max_attempts,
                ..metadata_policy()
            };
            for attempt_count in 0..=max_attempts {
                for retry_after in [None, Some(0), Some(42)] {
                    let mut progress = ThrottleRetryProgress {
                        attempt_count,
                        ..Default::default()
                    };
                    let before = progress;
                    let delay = progress.try_retry(retry_after, &policy);
                    assert_eq!(delay.is_some(), attempt_count < max_attempts);
                    if delay.is_none() {
                        assert_eq!(progress, before);
                    }
                }
            }
        }
    }

    #[cfg(feature = "unstable_dtx")]
    #[tokio::test]
    async fn body_bearing_dtx_throttles_are_left_to_the_coordinator() {
        let policy = metadata_policy();
        for body in [&[][..], &b"coordinator-result"[..]] {
            for is_dtx in [false, true] {
                let mut request = test_request(None);
                if is_dtx {
                    request.resource_type =
                        crate::models::ResourceType::DistributedTransactionBatch;
                }
                let mut progress = TransportRetryProgress::default();
                let action = try_throttle_retry(
                    throttled(body, Some(0)),
                    &request,
                    &policy,
                    &mut progress,
                    Clock::new_test(),
                )
                .await;
                assert_eq!(
                    matches!(action, RecoveryAction::Retry),
                    !is_dtx || body.is_empty(),
                );
                assert_eq!(
                    progress.throttle.attempt_count,
                    u32::from(!is_dtx || body.is_empty())
                );
                if let RecoveryAction::Fail(result) = action {
                    let TransportOutcome::HttpError { body: actual, .. } = result.outcome else {
                        panic!("expected the original coordinator response");
                    };
                    assert_eq!(actual, body);
                }
            }
        }
    }

    #[tokio::test]
    async fn non_throttled_responses_do_not_consume_retry_budget() {
        let success = TransportResult::from_http_response(
            CosmosStatus::new(azure_core::http::StatusCode::Ok),
            Default::default(),
            vec![],
        );
        let mut progress = TransportRetryProgress::default();
        assert!(matches!(
            try_throttle_retry(
                success,
                &test_request(None),
                &metadata_policy(),
                &mut progress,
                Clock::new_test()
            )
            .await,
            RecoveryAction::Fail(_)
        ));
        assert_eq!(progress.throttle, ThrottleRetryProgress::default());
    }

    #[test]
    fn request_class_caps_and_cumulative_equality_are_preserved() {
        for (count, wait, cap) in [
            (
                METADATA_MAX_THROTTLE_ATTEMPTS,
                METADATA_MAX_THROTTLE_WAIT,
                METADATA_MAX_PER_RETRY_DELAY,
            ),
            (
                DATA_PLANE_MAX_THROTTLE_ATTEMPTS,
                DATA_PLANE_MAX_THROTTLE_WAIT,
                DATA_PLANE_MAX_PER_RETRY_DELAY,
            ),
        ] {
            let policy = ThrottleRetryPolicy::with_limits(count, wait, cap);
            let mut progress = ThrottleRetryProgress {
                cumulative_budgeted_delay: wait - cap,
                ..Default::default()
            };
            assert_eq!(progress.try_retry(Some(60_000), &policy), Some(cap),);
            assert_eq!(progress.attempt_count, 1);
            assert_eq!(progress.cumulative_budgeted_delay, wait);
        }
    }

    #[tokio::test]
    async fn forced_final_changes_only_its_flag_and_does_not_stop_ordinary_retry() {
        let policy = ThrottleRetryPolicy {
            max_wait_time: Duration::from_millis(1),
            ..metadata_policy()
        };
        let clock = Clock::new_test();
        let request = test_request(None);
        let mut progress = TransportRetryProgress::default();
        assert_eq!(progress.throttle.try_retry(Some(2), &policy), None);
        assert_eq!(progress.throttle, ThrottleRetryProgress::default());
        let forced = ThrottleRetryProgress {
            forced_final_retry_used: true,
            ..Default::default()
        };
        assert!(matches!(
            try_throttle_retry(
                throttled(&[], Some(2)),
                &request,
                &policy,
                &mut progress,
                clock.clone()
            )
            .await,
            RecoveryAction::Retry
        ));
        assert_eq!(progress.throttle, forced);
        assert!(matches!(
            try_throttle_retry(
                throttled(&[], Some(2)),
                &request,
                &policy,
                &mut progress,
                clock.clone()
            )
            .await,
            RecoveryAction::Fail(_)
        ));
        assert_eq!(progress.throttle, forced);
        assert!(matches!(
            try_throttle_retry(
                throttled(&[], Some(0)),
                &request,
                &policy,
                &mut progress,
                clock
            )
            .await,
            RecoveryAction::Retry
        ));
        assert_eq!(
            progress.throttle,
            ThrottleRetryProgress {
                attempt_count: 1,
                ..forced
            },
        );
    }

    #[tokio::test]
    async fn deadline_capping_does_not_reduce_budgeted_delay() {
        let clock = Clock::new_test();
        let request = test_request(Some(clock.now() + Duration::from_millis(250)));
        let policy = metadata_policy();
        let mut progress = TransportRetryProgress::default();
        {
            let mut retry = Box::pin(try_throttle_retry(
                throttled(&[], Some(5000)),
                &request,
                &policy,
                &mut progress,
                clock.clone(),
            ));
            assert!(futures::poll!(retry.as_mut()).is_pending());
            clock.advance(Duration::from_millis(149));
            assert!(futures::poll!(retry.as_mut()).is_pending());
            clock.advance(Duration::from_millis(1));
            assert!(matches!(
                futures::poll!(retry.as_mut()),
                Poll::Ready(RecoveryAction::Retry)
            ));
        }
        assert_eq!(
            progress.throttle,
            ThrottleRetryProgress {
                attempt_count: 1,
                cumulative_budgeted_delay: Duration::from_secs(5),
                ..Default::default()
            },
        );
    }

    #[tokio::test]
    async fn expired_deadlines_only_cap_waits() {
        for max_wait_time in [Duration::from_secs(30), Duration::from_millis(1)] {
            let clock = Clock::new_test();
            let request = test_request(Some(clock.now()));
            let policy = ThrottleRetryPolicy {
                max_wait_time,
                ..metadata_policy()
            };
            let mut progress = TransportRetryProgress::default();
            assert!(matches!(
                try_throttle_retry(
                    throttled(&[], Some(2)),
                    &request,
                    &policy,
                    &mut progress,
                    clock.clone()
                )
                .await,
                RecoveryAction::Retry
            ));
            let forced = max_wait_time == Duration::from_millis(1);
            assert_eq!(progress.throttle.forced_final_retry_used, forced);
            assert_eq!(progress.throttle.attempt_count, u32::from(!forced));
            assert_eq!(clock.pending_sleep_count(), 0);
        }
    }

    #[test]
    fn diagnostic_context_preserves_retry_precedence() {
        let mut progress = TransportRetryProgress::default();
        progress.throttle.forced_final_retry_used = true;
        assert_eq!(
            progress.execution_context(ExecutionContext::Initial),
            ExecutionContext::Initial
        );
        progress.throttle.attempt_count = 1;
        assert_eq!(
            progress.execution_context(ExecutionContext::Initial),
            ExecutionContext::OperationRetry
        );
        progress.local_connectivity_retry_count = 1;
        assert_eq!(
            progress.execution_context(ExecutionContext::Initial),
            ExecutionContext::TransportRetry
        );
    }
}
