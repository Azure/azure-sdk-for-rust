// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::super::{realize_retry_delay, RetryDelay};
use super::{
    gateway_v2_context, gateway_v2_diagnostics, gateway_v2_transport_request, run_pipeline,
    test_endpoint_key, test_request, ActivityId, AdaptiveTransport, AlwaysThrottlesTransportClient,
    AlwaysThrottlesZeroDelayClient, Arc, ClientIdRecordingRetryTransport, CosmosEndpoint,
    Credential, DiagnosticsContextBuilder, DiagnosticsOptions, Duration, ExecutionContext,
    GatewayV2MockTransportClient, HttpResponse, PipelineKind, PipelineTestContext, TransportMode,
    TransportOutcome, TransportSecurity, METADATA_MAX_PER_RETRY_DELAY, TEST_CLIENT_ID,
    TEST_CLIENT_ID_HEADER,
};

#[test]
fn delay_realization_preserves_jitter_bounds_and_disabled_cases() {
    for millis in [5, 10, 160] {
        let base = Duration::from_millis(millis);
        assert_eq!(realize_retry_delay(RetryDelay::Service(base)), base);
        assert_eq!(
            realize_retry_delay(RetryDelay::Jittered { base, ratio: 0.0 }),
            base
        );
        let jittered = realize_retry_delay(RetryDelay::Jittered { base, ratio: 0.25 });
        assert!(jittered >= base.mul_f64(0.75));
        assert!(jittered <= base.mul_f64(1.25));
    }
    assert_eq!(
        realize_retry_delay(RetryDelay::Jittered {
            base: Duration::ZERO,
            ratio: 0.25
        }),
        Duration::ZERO,
    );
}

#[tokio::test]
async fn client_id_is_stable_across_retry_and_overrides_caller_header() {
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());
    let mut request = test_request(None);
    request
        .headers
        .insert(TEST_CLIENT_ID_HEADER.clone(), "caller-supplied");

    let recording = Arc::new(ClientIdRecordingRetryTransport::default());
    let client = AdaptiveTransport::Gateway(recording.clone());
    let mut diagnostics = DiagnosticsContextBuilder::new(
        ActivityId::from_string("client-id-retry".to_owned()),
        Arc::new(DiagnosticsOptions::default()),
    );

    let result = run_pipeline(
        request,
        &PipelineTestContext {
            transport: &client,
            allow_sent_transport_retry: false,
            credential: &Credential::from(azure_core::credentials::Secret::new("dGVzdA==")),
            user_agent: &azure_core::http::headers::HeaderValue::from_static("test-agent"),
            client_id: &TEST_CLIENT_ID,
            pipeline_type: PipelineKind::DataPlane,
            transport_security: TransportSecurity::Secure,
            endpoint_key: endpoint.endpoint_key(),
            account_name: None,
            collection_rid: None,
            max_throttle_attempts: 1,
            max_throttle_wait_time: Duration::from_secs(1),
            max_throttle_per_retry_delay: METADATA_MAX_PER_RETRY_DELAY,
        },
        &mut diagnostics,
    )
    .await;

    assert!(matches!(result.outcome, TransportOutcome::Success { .. }));
    assert_eq!(
        *recording.client_ids.lock().unwrap(),
        vec![
            Some(TEST_CLIENT_ID.as_str().to_owned()),
            Some(TEST_CLIENT_ID.as_str().to_owned()),
        ]
    );
}

/// End-to-end regression: `max_throttle_attempts = 0` must surface the
/// first 429 to the caller with **exactly one** request on the wire,
/// honoring the `MaxRetryAttemptsOnRateLimitedRequests = 0` .NET-parity
/// contract.
///
/// This test guards the full `execute_transport_pipeline` loop, including
/// the one-shot `forced_final_retry` safety net which is suppressed when
/// the user has explicitly opted out of throttle retries.
/// The pure count-limit test only covers the classifier; forced-final recovery follows
/// classifier returns `Propagate`, so it is invisible to the
/// classifier-level test.
#[tokio::test]
async fn execute_transport_pipeline_with_zero_max_attempts_does_not_retry_429() {
    let request_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let client = AdaptiveTransport::Gateway(Arc::new(AlwaysThrottlesTransportClient {
        request_count: Arc::clone(&request_count),
    }));
    let mut diagnostics = DiagnosticsContextBuilder::new(
        ActivityId::from_string("throttle-max-attempts-zero".to_owned()),
        Arc::new(DiagnosticsOptions::default()),
    );

    let result = run_pipeline(
        test_request(None),
        &PipelineTestContext {
            transport: &client,
            allow_sent_transport_retry: false,
            credential: &Credential::from(azure_core::credentials::Secret::new("dGVzdA==")),
            user_agent: &azure_core::http::headers::HeaderValue::from_static("test-agent"),
            client_id: &TEST_CLIENT_ID,
            pipeline_type: PipelineKind::DataPlane,
            transport_security: TransportSecurity::Secure,
            endpoint_key: test_endpoint_key(),
            account_name: None,
            collection_rid: None,
            max_throttle_attempts: 0,
            max_throttle_wait_time: Duration::from_secs(30),
            max_throttle_per_retry_delay: METADATA_MAX_PER_RETRY_DELAY,
        },
        &mut diagnostics,
    )
    .await;

    // Exactly one request hit the wire — no throttle retry, no forced
    // final retry.
    assert_eq!(
        request_count.load(std::sync::atomic::Ordering::SeqCst),
        1,
        "max_throttle_attempts=0 must surface the first 429 with no retry, but observed {} total transport requests",
        request_count.load(std::sync::atomic::Ordering::SeqCst),
    );

    // The 429 propagates as an HttpError so the caller can react to it.
    match result.outcome {
        TransportOutcome::HttpError { status, .. } => {
            assert!(
                status.is_throttled(),
                "expected 429/throttled outcome, got {:?}",
                status,
            );
        }
        other => panic!("expected HttpError(429), got {other:?}"),
    }

    // Diagnostics record the single attempt.
    let completed = diagnostics.complete();
    assert_eq!(completed.requests().len(), 1);
}

/// End-to-end companion: with the default budget (≥ 1 attempt) the
/// forced-final retry remains active. This pins the historical safety-net
/// behavior so a future change to the gating logic that over-suppresses
/// the forced-final retry would be caught by tests.
#[tokio::test]
async fn execute_transport_pipeline_with_default_attempts_uses_forced_final_retry() {
    let request_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let client = AdaptiveTransport::Gateway(Arc::new(AlwaysThrottlesTransportClient {
        request_count: Arc::clone(&request_count),
    }));
    let mut diagnostics = DiagnosticsContextBuilder::new(
        ActivityId::from_string("throttle-forced-final".to_owned()),
        Arc::new(DiagnosticsOptions::default()),
    );

    // A very tight cumulative-wait budget (1ms) so the regular throttle
    // loop bails out immediately, leaving only the forced-final-retry
    // path to consider.
    let _ = run_pipeline(
        test_request(None),
        &PipelineTestContext {
            transport: &client,
            allow_sent_transport_retry: false,
            credential: &Credential::from(azure_core::credentials::Secret::new("dGVzdA==")),
            user_agent: &azure_core::http::headers::HeaderValue::from_static("test-agent"),
            client_id: &TEST_CLIENT_ID,
            pipeline_type: PipelineKind::DataPlane,
            transport_security: TransportSecurity::Secure,
            endpoint_key: test_endpoint_key(),
            account_name: None,
            collection_rid: None,
            max_throttle_attempts: 9,
            max_throttle_wait_time: Duration::from_millis(1),
            max_throttle_per_retry_delay: METADATA_MAX_PER_RETRY_DELAY,
        },
        &mut diagnostics,
    )
    .await;

    // Initial request + one forced-final-retry attempt = 2 total.
    assert_eq!(
        request_count.load(std::sync::atomic::Ordering::SeqCst),
        2,
        "default (≥1 attempt) configuration must permit the one-shot \
         forced-final retry, expected 2 transport requests, observed {}",
        request_count.load(std::sync::atomic::Ordering::SeqCst),
    );
}

/// End-to-end fault-injection regression: a transport that *always* throws
/// 429 must be retried exactly `max_throttle_attempts` times before the
/// throttle propagates to the caller, for **several** configured limits.
///
/// This is the direct verification that the
/// [`ThrottlingRetryOptionsView::max_retry_count`](crate::options::ThrottlingRetryOptionsView::max_retry_count)
/// knob (surfaced here as
/// [`PipelineTestContext::max_throttle_attempts`]) is honored: with a
/// generous cumulative-wait budget and no end-to-end deadline, the only
/// limiter is the attempt count, so the total number of requests that hit
/// the wire is deterministic.
///
/// Wire-request accounting for `max_throttle_attempts = N` (where `N > 0`):
///
/// * `1` initial attempt, plus
/// * `N` throttle retries (the classifier keeps retrying while
///   `attempt_count < N`).
///
/// Total = `N + 1`. Once the count budget is exhausted the one-shot
/// `forced_final_retry` safety net is suppressed too, matching the
/// .NET-parity `MaxRetryAttemptsOnRateLimitedRequests` semantic. The
/// forced-final retry still fires when the *cumulative-wait* budget is
/// the limiter (rather than the count), which is covered by
/// [`execute_transport_pipeline_with_default_attempts_uses_forced_final_retry`].
/// The `N = 0` opt-out (exactly one request, no forced-final retry) is
/// covered by
/// [`execute_transport_pipeline_with_zero_max_attempts_does_not_retry_429`].
#[tokio::test]
async fn execute_transport_pipeline_honors_configured_max_throttle_attempts() {
    for max_throttle_attempts in [1_u32, 2, 3, 5] {
        let request_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let client = AdaptiveTransport::Gateway(Arc::new(AlwaysThrottlesTransportClient {
            request_count: Arc::clone(&request_count),
        }));
        let mut diagnostics = DiagnosticsContextBuilder::new(
            ActivityId::from_string(format!("throttle-attempts-{max_throttle_attempts}")),
            Arc::new(DiagnosticsOptions::default()),
        );

        let result = run_pipeline(
            // No deadline so the cumulative-wait/deadline guards never cut
            // the loop short — the attempt count is the sole limiter.
            test_request(None),
            &PipelineTestContext {
                transport: &client,
                allow_sent_transport_retry: false,
                credential: &Credential::from(azure_core::credentials::Secret::new("dGVzdA==")),
                user_agent: &azure_core::http::headers::HeaderValue::from_static("test-agent"),
                client_id: &TEST_CLIENT_ID,
                pipeline_type: PipelineKind::DataPlane,
                transport_security: TransportSecurity::Secure,
                endpoint_key: test_endpoint_key(),
                account_name: None,
                collection_rid: None,
                max_throttle_attempts,
                // Generous budget so the cumulative-wait cap is never the
                // limiter for these small attempt counts.
                max_throttle_wait_time: Duration::from_secs(300),
                max_throttle_per_retry_delay: METADATA_MAX_PER_RETRY_DELAY,
            },
            &mut diagnostics,
        )
        .await;

        // 1 initial + N throttle retries = N + 1. The forced-final retry
        // is gated on `attempt_count < max_attempts`, so once the count
        // budget is exhausted the safety net does NOT fire too — matching
        // .NET's `MaxRetryAttemptsOnRateLimitedRequests` semantic.
        let expected = max_throttle_attempts as usize + 1;
        assert_eq!(
            request_count.load(std::sync::atomic::Ordering::SeqCst),
            expected,
            "max_throttle_attempts={max_throttle_attempts} must yield {expected} total \
             transport requests (1 initial + {max_throttle_attempts} retries), but observed {}",
            request_count.load(std::sync::atomic::Ordering::SeqCst),
        );

        // After the budget is exhausted the 429 propagates to the caller.
        match result.outcome {
            TransportOutcome::HttpError { status, .. } => {
                assert!(
                    status.is_throttled(),
                    "expected 429/throttled outcome for \
                     max_throttle_attempts={max_throttle_attempts}, got {status:?}",
                );
            }
            other => panic!(
                "expected HttpError(429) for max_throttle_attempts={max_throttle_attempts}, \
                 got {other:?}"
            ),
        }
    }
}

/// End-to-end: the per-class *default* throttle budgets (data-plane 18 /
/// metadata 9, from `default_throttle_budget`) drive the real
/// `execute_transport_pipeline` loop to the expected number of wire
/// attempts, confirming the values wired in by the operation pipeline are
/// the ones honored on the wire. Follows the same mock + `N + 1` accounting
/// as `execute_transport_pipeline_honors_configured_max_throttle_attempts`;
/// `x-ms-retry-after-ms: 0` makes the attempt count the sole limiter so the
/// forced-final retry stays suppressed (total = 1 initial + N retries).
#[tokio::test]
async fn execute_transport_pipeline_honors_class_default_throttle_budgets() {
    use crate::pipeline::transport::recovery::{
        DATA_PLANE_MAX_PER_RETRY_DELAY, DATA_PLANE_MAX_THROTTLE_ATTEMPTS,
        DATA_PLANE_MAX_THROTTLE_WAIT, METADATA_MAX_PER_RETRY_DELAY, METADATA_MAX_THROTTLE_ATTEMPTS,
        METADATA_MAX_THROTTLE_WAIT,
    };

    // The data-plane default must retry strictly more often than metadata,
    // so the two cases below can never collapse to the same wire count.
    assert!(DATA_PLANE_MAX_THROTTLE_ATTEMPTS > METADATA_MAX_THROTTLE_ATTEMPTS);

    for (pipeline_type, attempts, wait, per_retry) in [
        (
            PipelineKind::DataPlane,
            DATA_PLANE_MAX_THROTTLE_ATTEMPTS,
            DATA_PLANE_MAX_THROTTLE_WAIT,
            DATA_PLANE_MAX_PER_RETRY_DELAY,
        ),
        (
            PipelineKind::Metadata,
            METADATA_MAX_THROTTLE_ATTEMPTS,
            METADATA_MAX_THROTTLE_WAIT,
            METADATA_MAX_PER_RETRY_DELAY,
        ),
    ] {
        let request_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let client = AdaptiveTransport::Gateway(Arc::new(AlwaysThrottlesZeroDelayClient {
            request_count: Arc::clone(&request_count),
        }));
        let mut diagnostics = DiagnosticsContextBuilder::new(
            ActivityId::from_string(format!("throttle-default-{pipeline_type:?}")),
            Arc::new(DiagnosticsOptions::default()),
        );

        let result = run_pipeline(
            test_request(None),
            &PipelineTestContext {
                transport: &client,
                allow_sent_transport_retry: false,
                credential: &Credential::from(azure_core::credentials::Secret::new("dGVzdA==")),
                user_agent: &azure_core::http::headers::HeaderValue::from_static("test-agent"),
                client_id: &TEST_CLIENT_ID,
                pipeline_type,
                transport_security: TransportSecurity::Secure,
                endpoint_key: test_endpoint_key(),
                account_name: None,
                collection_rid: None,
                max_throttle_attempts: attempts,
                max_throttle_wait_time: wait,
                max_throttle_per_retry_delay: per_retry,
            },
            &mut diagnostics,
        )
        .await;

        // 1 initial + N retries = N + 1 (forced-final retry suppressed once
        // the attempt budget is the limiter).
        let expected = attempts as usize + 1;
        assert_eq!(
            request_count.load(std::sync::atomic::Ordering::SeqCst),
            expected,
            "{pipeline_type:?} default budget ({attempts} attempts) must yield {expected} \
             wire requests, observed {}",
            request_count.load(std::sync::atomic::Ordering::SeqCst),
        );

        match result.outcome {
            TransportOutcome::HttpError { status, .. } => assert!(
                status.is_throttled(),
                "expected 429 to propagate for {pipeline_type:?}, got {status:?}",
            ),
            other => panic!("expected HttpError(429) for {pipeline_type:?}, got {other:?}"),
        }
    }
}

#[tokio::test]
async fn ordinary_retry_can_follow_forced_final_without_changing_target() {
    let response = |status, delay| {
        let mut headers = azure_core::http::headers::Headers::new();
        headers.insert("x-ms-retry-after-ms", delay);
        HttpResponse {
            status,
            headers,
            body: Vec::new(),
        }
    };
    let mock = Arc::new(GatewayV2MockTransportClient::new(vec![
        response(429, "2"),
        response(429, "0"),
        response(200, "0"),
    ]));
    let client = AdaptiveTransport::Gateway(mock.clone());
    let request = gateway_v2_transport_request(TransportMode::Gateway);
    let expected_url = request.url.clone();
    let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
    let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");
    let mut context = gateway_v2_context(
        &client,
        request.endpoint.endpoint_key(),
        None,
        &credential,
        &user_agent,
    );
    context.max_throttle_wait_time = Duration::from_millis(1);
    let mut diagnostics = gateway_v2_diagnostics();

    let result = run_pipeline(request, &context, &mut diagnostics).await;

    assert!(matches!(result.outcome, TransportOutcome::Success { .. }));
    let captured = mock.requests();
    assert_eq!(captured.len(), 3);
    for request in &captured {
        assert_eq!(request.url, expected_url);
        assert_eq!(request.method, azure_core::http::Method::Get);
        assert_eq!(request.timeout, None);
        assert!(request
            .headers
            .get_optional_str(&azure_core::http::headers::AUTHORIZATION)
            .is_some());
    }
    let completed = diagnostics.complete();
    assert_eq!(
        completed
            .requests()
            .iter()
            .map(|r| r.execution_context())
            .collect::<Vec<_>>(),
        vec![
            ExecutionContext::Initial,
            ExecutionContext::Initial,
            ExecutionContext::OperationRetry
        ],
    );
}
