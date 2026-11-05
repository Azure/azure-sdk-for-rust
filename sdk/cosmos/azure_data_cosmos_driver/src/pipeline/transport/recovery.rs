// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Recovers failed attempts against the same endpoint, using explicit retry limits and progress.

use std::time::{Duration, Instant};

use crate::{
    diagnostics::{
        DiagnosticsContextBuilder, ExecutionContext, FailedTransportShardDiagnostics,
        RequestHandle, RequestSentStatus,
    },
    driver::jitter,
    error::{status_codes, CosmosError},
    pipeline::{Clock, RecoveryAction},
};

use super::{
    adaptive_transport::AdaptiveTransport,
    deadline::Deadline,
    diagnostics,
    dispatch::{DispatchError, DispatchErrorDetails},
    request::PreparationError,
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
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct ThrottleRetryProgress {
    pub attempt_count: u32,
    /// Delay charged before the deadline cap reduces the actual sleep.
    pub cumulative_budgeted_delay: Duration,
    pub forced_final_retry_used: bool,
}

#[derive(Default)]
pub(super) struct TransportRetryProgress {
    pub throttle: ThrottleRetryProgress,
    pub local_connectivity_retry_count: u32,
    pub excluded_shard_id: Option<u64>,
    pub failed_shards: Vec<FailedTransportShardDiagnostics>,
}

#[derive(Debug, PartialEq)]
pub(super) enum RetryDelay {
    Service(Duration),
    Jittered { base: Duration, ratio: f64 },
}

impl RetryDelay {
    /// Returns the service delay unchanged, or samples jitter around the fallback delay.
    // Inline delay selection; jitter sampling stays in its helper.
    #[inline(always)]
    fn sample(self) -> Duration {
        match self {
            Self::Service(delay) => delay,
            Self::Jittered { base, ratio } if ratio == 0.0 || base.is_zero() => base,
            Self::Jittered { base, ratio } => {
                Duration::from_secs_f64(jitter::with_jitter(base.as_secs_f64(), ratio))
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum ThrottleDecision {
    Retry {
        delay: Duration,
        next: ThrottleRetryProgress,
    },
    ForcedFinal {
        next: ThrottleRetryProgress,
    },
    Propagate,
}

#[derive(Debug, PartialEq)]
enum ThrottleWait {
    RetryThrottle {
        delay: Duration,
        next: ThrottleRetryProgress,
    },
    ForceFinalThrottle {
        delay: Duration,
        next: ThrottleRetryProgress,
    },
    Propagate,
    DeadlineExceeded,
}

impl ThrottleDecision {
    /// Returns whether the throttle budget requires propagating the original result.
    #[inline(always)]
    fn is_terminal(&self) -> bool {
        matches!(self, Self::Propagate)
    }
}

/// Records an authorization or encoding failure and ends the invocation without sending.
// Inline the thin failure adapter; diagnostics retains its call frame.
#[inline(always)]
pub(super) fn recover_prep_error(
    error: PreparationError,
    handle: RequestHandle,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> RecoveryAction<TransportResult> {
    diagnostics::fail_preparation(&error, handle, diagnostics);
    RecoveryAction::Fail(error.into())
}

/// Retries an eligible connectivity failure or 429 against the same endpoint, or returns its result.
///
/// Connectivity replay takes precedence. Ordinary throttle progress changes only after the wait
/// and deadline check; a forced-final retry changes only its own flag.
pub(super) async fn recover_dispatch_error(
    error: DispatchError,
    request: &TransportRequest,
    config: &TransportPipelineConfig<'_>,
    transport: &AdaptiveTransport,
    progress: &mut TransportRetryProgress,
    clock: Clock,
) -> RecoveryAction<TransportResult> {
    let details = DispatchErrorDetails::from(error);
    if let Some(shard_id) = connectivity_retry_candidate(
        &details.result,
        details.shard_id,
        progress.local_connectivity_retry_count,
        config.allow_sent_transport_retry,
    ) {
        if transport.can_retry_on_different_shard(shard_id, &config.endpoint_key) {
            if let Some(failed) = details.failed_shard() {
                progress.failed_shards.push(failed);
            }
            progress.local_connectivity_retry_count += 1;
            progress.excluded_shard_id = Some(shard_id);
            return RecoveryAction::Retry;
        }
    }

    #[cfg(feature = "unstable_dtx")]
    let is_dtx = request.resource_type == crate::models::ResourceType::DistributedTransactionBatch;
    #[cfg(not(feature = "unstable_dtx"))]
    let is_dtx = false;

    let Some(source) = throttle_delay_source(
        &details.result,
        &config.throttle_policy,
        progress.throttle,
        is_dtx,
    ) else {
        return RecoveryAction::Fail(details.result);
    };
    let decision =
        evaluate_throttle_budget(source.sample(), &config.throttle_policy, progress.throttle);
    if decision.is_terminal() {
        return RecoveryAction::Fail(details.result);
    }
    match plan_throttle_wait(decision, request.deadline, clock.now()) {
        ThrottleWait::RetryThrottle { delay, next } => {
            clock.sleep(delay).await;
            if request.deadline.exceeded(clock.now()) {
                return RecoveryAction::Fail(TransportResult::deadline_exceeded(
                    RequestSentStatus::Sent,
                ));
            }
            progress.throttle = next;
        }
        ThrottleWait::ForceFinalThrottle { delay, next } => {
            if !delay.is_zero() {
                clock.sleep(delay).await;
            }
            progress.throttle = next;
        }
        ThrottleWait::Propagate => return RecoveryAction::Fail(details.result),
        ThrottleWait::DeadlineExceeded => {
            return RecoveryAction::Fail(TransportResult::deadline_exceeded(
                RequestSentStatus::Sent,
            ));
        }
    }
    RecoveryAction::Retry
}

/// Selects and samples a retry delay, then applies the throttle budget.
///
/// Bootstrap uses this shared policy without applying the transport loop's forced-final retry.
// Inline the thin policy adapter; budget evaluation remains separate.
#[inline(always)]
pub(crate) fn throttle_retry(
    retry_after_ms: Option<u64>,
    policy: &ThrottleRetryPolicy,
    progress: ThrottleRetryProgress,
) -> ThrottleDecision {
    match retry_delay_source(retry_after_ms, policy, progress) {
        Some(source) => evaluate_throttle_budget(source.sample(), policy, progress),
        None => ThrottleDecision::Propagate,
    }
}

/// Chooses the attempt's diagnostic context, giving local shard retries precedence over throttling.
#[inline(always)]
pub(super) fn execution_context(
    initial: ExecutionContext,
    progress: &TransportRetryProgress,
) -> ExecutionContext {
    if progress.local_connectivity_retry_count > 0 {
        ExecutionContext::TransportRetry
    } else if progress.throttle.attempt_count > 0 {
        ExecutionContext::OperationRetry
    } else {
        initial
    }
}

/// Returns the failed shard's ID when the error is safe to replay and the local retry is unused.
// Inline the small eligibility adapter.
#[inline(always)]
pub(super) fn connectivity_retry_candidate(
    result: &TransportResult,
    shard_id: Option<u64>,
    retry_count: u32,
    allow_sent_transport_retry: bool,
) -> Option<u64> {
    shard_id.filter(|_| {
        retry_count < MAX_LOCAL_CONNECTIVITY_RETRIES
            && should_retry_connectivity_failure(result, allow_sent_transport_retry)
    })
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

/// Selects a delay for a retryable 429, leaving body-bearing DTX responses to the coordinator.
pub(super) fn throttle_delay_source(
    result: &TransportResult,
    policy: &ThrottleRetryPolicy,
    progress: ThrottleRetryProgress,
    is_distributed_transaction_request: bool,
) -> Option<RetryDelay> {
    if !result
        .outcome
        .is_retryable_throttle(is_distributed_transaction_request)
    {
        return None;
    }
    retry_delay_source(
        result
            .cosmos_headers()
            .and_then(|headers| headers.retry_after_ms),
        policy,
        progress,
    )
}

/// Describes the service delay or exponential fallback without sampling jitter.
///
/// Returns `None` when the ordinary retry count is exhausted.
pub(super) fn retry_delay_source(
    retry_after_ms: Option<u64>,
    policy: &ThrottleRetryPolicy,
    progress: ThrottleRetryProgress,
) -> Option<RetryDelay> {
    if progress.attempt_count >= policy.max_attempts {
        return None;
    }
    Some(match retry_after_ms {
        Some(ms) => RetryDelay::Service(Duration::from_millis(ms)),
        None => RetryDelay::Jittered {
            base: policy
                .fallback_base_delay
                .mul_f64(policy.backoff_factor.powi(progress.attempt_count as i32)),
            ratio: policy.backoff_jitter_ratio.clamp(0.0, 1.0),
        },
    })
}

/// Caps the realized delay and returns the retry's next progress, or handles wait-budget exhaustion.
///
/// The caller checks the count limit first. Only the forced-final flag changes for a forced retry.
pub(super) fn evaluate_throttle_budget(
    realized_delay: Duration,
    policy: &ThrottleRetryPolicy,
    progress: ThrottleRetryProgress,
) -> ThrottleDecision {
    let delay = realized_delay.min(policy.max_per_retry_delay);
    let cumulative = progress.cumulative_budgeted_delay + delay;
    if cumulative > policy.max_wait_time {
        return if progress.forced_final_retry_used {
            ThrottleDecision::Propagate
        } else {
            ThrottleDecision::ForcedFinal {
                next: ThrottleRetryProgress {
                    forced_final_retry_used: true,
                    ..progress
                },
            }
        };
    }
    ThrottleDecision::Retry {
        delay,
        next: ThrottleRetryProgress {
            attempt_count: progress.attempt_count + 1,
            cumulative_budgeted_delay: cumulative,
            ..progress
        },
    }
}

/// Applies the deadline at `now` to a throttle decision, returning a wait, timeout, or propagation.
///
/// The next progress retains the delay charged before deadline capping.
fn plan_throttle_wait(
    decision: ThrottleDecision,
    deadline: Deadline,
    now: Instant,
) -> ThrottleWait {
    match decision {
        ThrottleDecision::Retry { delay, next } => {
            if deadline.exceeded(now) {
                ThrottleWait::DeadlineExceeded
            } else {
                ThrottleWait::RetryThrottle {
                    delay: deadline.cap_delay(delay, now),
                    next,
                }
            }
        }
        ThrottleDecision::ForcedFinal { next } => match deadline.forced_final_delay(now) {
            Some(delay) => ThrottleWait::ForceFinalThrottle { delay, next },
            None => ThrottleWait::Propagate,
        },
        ThrottleDecision::Propagate => ThrottleWait::Propagate,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        connectivity_retry_candidate, evaluate_throttle_budget, execution_context,
        plan_throttle_wait, retry_delay_source, throttle_delay_source, Deadline, RetryDelay,
        ThrottleDecision, ThrottleRetryPolicy, ThrottleRetryProgress, ThrottleWait,
        TransportRetryProgress, DATA_PLANE_MAX_PER_RETRY_DELAY, DATA_PLANE_MAX_THROTTLE_ATTEMPTS,
        DATA_PLANE_MAX_THROTTLE_WAIT, METADATA_MAX_PER_RETRY_DELAY, METADATA_MAX_THROTTLE_ATTEMPTS,
        METADATA_MAX_THROTTLE_WAIT,
    };
    use crate::{
        diagnostics::{ExecutionContext, RequestSentStatus},
        error::{status_codes, CosmosError, CosmosStatus},
        models::CosmosResponseHeaders,
        pipeline::transport::types::{TransportOutcome, TransportResult},
    };
    use std::time::{Duration, Instant};

    #[test]
    fn delay_sampling_preserves_jitter_bounds_and_disabled_cases() {
        for millis in [5, 10, 160] {
            let base = Duration::from_millis(millis);
            assert_eq!(RetryDelay::Service(base).sample(), base);
            assert_eq!(RetryDelay::Jittered { base, ratio: 0.0 }.sample(), base);
            let jittered = RetryDelay::Jittered { base, ratio: 0.25 }.sample();
            assert!(jittered >= base.mul_f64(0.75));
            assert!(jittered <= base.mul_f64(1.25));
        }
        assert_eq!(
            RetryDelay::Jittered {
                base: Duration::ZERO,
                ratio: 0.25
            }
            .sample(),
            Duration::ZERO,
        );
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
        let policy = metadata_policy();
        let progress = ThrottleRetryProgress::default();
        assert_eq!(
            throttle_delay_source(&throttled(&[], Some(42)), &policy, progress, false),
            Some(RetryDelay::Service(Duration::from_millis(42))),
        );
        for (attempt_count, millis) in [(0, 5), (1, 10), (5, 160)] {
            assert_eq!(
                retry_delay_source(
                    None,
                    &policy,
                    ThrottleRetryProgress {
                        attempt_count,
                        ..progress
                    }
                ),
                Some(RetryDelay::Jittered {
                    base: Duration::from_millis(millis),
                    ratio: 0.25
                }),
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
                    let source = retry_delay_source(
                        retry_after,
                        &policy,
                        ThrottleRetryProgress {
                            attempt_count,
                            ..Default::default()
                        },
                    );
                    assert_eq!(source.is_some(), attempt_count < max_attempts);
                }
            }
        }
    }

    #[test]
    fn body_bearing_dtx_throttles_are_left_to_the_coordinator() {
        let policy = metadata_policy();
        for body in [&[][..], &b"coordinator-result"[..]] {
            for is_dtx in [false, true] {
                assert_eq!(
                    throttle_delay_source(
                        &throttled(body, Some(42)),
                        &policy,
                        Default::default(),
                        is_dtx
                    )
                    .is_some(),
                    !is_dtx || body.is_empty(),
                );
            }
        }
        let success = TransportResult::from_http_response(
            CosmosStatus::new(azure_core::http::StatusCode::Ok),
            Default::default(),
            vec![],
        );
        assert_eq!(
            throttle_delay_source(&success, &policy, Default::default(), false),
            None
        );
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
            let progress = ThrottleRetryProgress {
                cumulative_budgeted_delay: wait - cap,
                ..Default::default()
            };
            assert_eq!(
                evaluate_throttle_budget(Duration::from_secs(60), &policy, progress),
                ThrottleDecision::Retry {
                    delay: cap,
                    next: ThrottleRetryProgress {
                        attempt_count: 1,
                        cumulative_budgeted_delay: wait,
                        ..progress
                    }
                },
            );
        }
    }

    #[test]
    fn forced_final_changes_only_its_flag_and_does_not_stop_ordinary_retry() {
        let policy = ThrottleRetryPolicy {
            max_wait_time: Duration::from_millis(1),
            ..metadata_policy()
        };
        let progress = ThrottleRetryProgress::default();
        let next = ThrottleRetryProgress {
            forced_final_retry_used: true,
            ..progress
        };
        assert_eq!(
            evaluate_throttle_budget(Duration::from_millis(2), &policy, progress),
            ThrottleDecision::ForcedFinal { next },
        );
        assert_eq!(
            evaluate_throttle_budget(Duration::from_millis(2), &policy, next),
            ThrottleDecision::Propagate
        );
        assert_eq!(
            evaluate_throttle_budget(Duration::ZERO, &policy, next),
            ThrottleDecision::Retry {
                delay: Duration::ZERO,
                next: ThrottleRetryProgress {
                    attempt_count: 1,
                    ..next
                }
            },
        );
    }

    #[test]
    fn deadline_capping_does_not_reduce_budgeted_delay() {
        let now = Instant::now();
        let policy = metadata_policy();
        let decision =
            evaluate_throttle_budget(Duration::from_secs(5), &policy, Default::default());
        assert_eq!(
            plan_throttle_wait(
                decision,
                Deadline::from(Some(now + Duration::from_millis(250))),
                now
            ),
            ThrottleWait::RetryThrottle {
                delay: Duration::from_millis(150),
                next: ThrottleRetryProgress {
                    attempt_count: 1,
                    cumulative_budgeted_delay: Duration::from_secs(5),
                    ..Default::default()
                },
            },
        );
    }

    #[test]
    fn expired_deadline_distinguishes_ordinary_and_forced_waits() {
        let now = Instant::now();
        let deadline = Deadline::from(Some(now));
        let next = ThrottleRetryProgress::default();
        assert_eq!(
            plan_throttle_wait(
                ThrottleDecision::Retry {
                    delay: Duration::ZERO,
                    next
                },
                deadline,
                now,
            ),
            ThrottleWait::DeadlineExceeded
        );
        assert_eq!(
            plan_throttle_wait(ThrottleDecision::ForcedFinal { next }, deadline, now),
            ThrottleWait::Propagate
        );
    }

    #[test]
    fn connectivity_requires_an_eligible_error_shard_and_unused_retry() {
        for request_sent in [
            RequestSentStatus::NotSent,
            RequestSentStatus::Sent,
            RequestSentStatus::Unknown,
        ] {
            let result = TransportResult {
                outcome: TransportOutcome::TransportError {
                    status: status_codes::TRANSPORT_GENERATED_503,
                    error: CosmosError::builder()
                        .with_status(status_codes::TRANSPORT_IO_FAILED)
                        .build(),
                    request_sent,
                },
            };
            for allow_sent in [false, true] {
                assert_eq!(
                    connectivity_retry_candidate(&result, Some(7), 0, allow_sent),
                    (request_sent == RequestSentStatus::NotSent || allow_sent).then_some(7),
                );
                assert_eq!(
                    connectivity_retry_candidate(&result, None, 0, allow_sent),
                    None
                );
                assert_eq!(
                    connectivity_retry_candidate(&result, Some(7), 1, allow_sent),
                    None
                );
            }
        }
        assert_eq!(
            connectivity_retry_candidate(
                &TransportResult::deadline_exceeded(RequestSentStatus::Unknown),
                Some(7),
                0,
                true,
            ),
            None
        );
    }

    #[test]
    fn diagnostic_context_preserves_retry_precedence() {
        let mut progress = TransportRetryProgress::default();
        progress.throttle.forced_final_retry_used = true;
        assert_eq!(
            execution_context(ExecutionContext::Initial, &progress),
            ExecutionContext::Initial
        );
        progress.throttle.attempt_count = 1;
        assert_eq!(
            execution_context(ExecutionContext::Initial, &progress),
            ExecutionContext::OperationRetry
        );
        progress.local_connectivity_retry_count = 1;
        assert_eq!(
            execution_context(ExecutionContext::Initial, &progress),
            ExecutionContext::TransportRetry
        );
    }
}
