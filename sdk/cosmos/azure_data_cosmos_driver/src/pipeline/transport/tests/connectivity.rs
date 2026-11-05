// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    connectivity_failure, run_pipeline, scripted_transport, should_retry_connectivity_failure,
    test_endpoint_key, test_request, ActivityId, AdaptiveTransport, Arc, Credential,
    DiagnosticsContextBuilder, DiagnosticsOptions, Duration, ExecutionContext, HttpClientConfig,
    HttpRequest, HttpResponse, Instant, Mutex, PipelineKind, PipelineTestContext,
    RequestSentStatus, ScriptedFactory, TransportClient, TransportError, TransportOutcome,
    TransportSecurity, MAX_LOCAL_CONNECTIVITY_RETRIES, METADATA_MAX_PER_RETRY_DELAY,
    TEST_CLIENT_ID,
};
use async_trait::async_trait;
use std::collections::VecDeque;

#[derive(Debug)]
struct MixedRecoveryTransport {
    shard: u64,
    responses: Arc<Mutex<VecDeque<Result<HttpResponse, TransportError>>>>,
    requests: Arc<Mutex<Vec<(u64, HttpRequest)>>>,
}

#[async_trait]
impl TransportClient for MixedRecoveryTransport {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        self.requests
            .lock()
            .unwrap()
            .push((self.shard, request.clone()));
        self.responses
            .lock()
            .unwrap()
            .pop_front()
            .expect("unexpected extra dispatch")
    }
}

/// A request that definitively never left the client is safe to retry for
/// every operation type, so the caller's flag is not consulted.
#[test]
fn unsent_connectivity_failure_retries_regardless_of_caller_flag() {
    let result = connectivity_failure(RequestSentStatus::NotSent);
    assert!(should_retry_connectivity_failure(&result, false));
    assert!(should_retry_connectivity_failure(&result, true));
}

/// Once the request may have reached the backend, the same predicate the
/// operation pipeline uses for cross-region failover decides this retry.
/// Callers pass `CosmosOperation::allows_ambiguous_outcome_retry`, so the
/// two layers cannot disagree about a single failure.
#[test]
fn possibly_sent_connectivity_failure_defers_to_caller_flag() {
    for request_sent in [RequestSentStatus::Sent, RequestSentStatus::Unknown] {
        let result = connectivity_failure(request_sent);
        assert!(
            !should_retry_connectivity_failure(&result, false),
            "{request_sent:?} must not retry when the operation forbids ambiguous retries"
        );
        assert!(
            should_retry_connectivity_failure(&result, true),
            "{request_sent:?} must retry when the operation allows ambiguous retries"
        );
    }
}

/// The retry is bounded to a single extra same-endpoint attempt; widening
/// it would multiply duplicate executions for non-idempotent writes.
#[test]
fn local_connectivity_retries_are_capped_at_one() {
    assert_eq!(MAX_LOCAL_CONNECTIVITY_RETRIES, 1);
}

#[tokio::test]
async fn execute_transport_pipeline_retries_not_sent_connectivity_error_on_different_shard() {
    let client = scripted_transport(
        crate::error::status_codes::TRANSPORT_CONNECTION_FAILED,
        "first shard failed",
        crate::error::status_codes::TRANSPORT_CONNECTION_FAILED,
        "second shard failed",
    );
    let mut diagnostics = DiagnosticsContextBuilder::new(
        ActivityId::from_string("transport-retry-not-sent".to_owned()),
        Arc::new(DiagnosticsOptions::default()),
    );

    let result = run_pipeline(
        test_request(Some(Instant::now() + Duration::from_secs(2))),
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
            max_throttle_wait_time: Duration::from_secs(30),
            max_throttle_per_retry_delay: METADATA_MAX_PER_RETRY_DELAY,
        },
        &mut diagnostics,
    )
    .await;

    match result.outcome {
        TransportOutcome::TransportError { error, .. } => {
            assert!(error.to_string().contains("second shard failed"));
        }
        other => panic!("expected transport error, got {other:?}"),
    }

    let diagnostics = diagnostics.complete();
    let requests = diagnostics.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[1].local_shard_retry_count(), 1);
    assert_eq!(requests[1].failed_transport_shards().len(), 1);
    let recorded = requests[1].failed_transport_shards()[0].error();
    assert!(
        recorded.ends_with("first shard failed"),
        "unexpected: {recorded}"
    );
}

#[tokio::test]
async fn execute_transport_pipeline_only_retries_unknown_connectivity_error_when_allowed() {
    let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
    let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");

    let client_without_retry = scripted_transport(
        crate::error::status_codes::TRANSPORT_IO_FAILED,
        "first io shard failed",
        crate::error::status_codes::TRANSPORT_IO_FAILED,
        "second io shard failed",
    );
    let mut diagnostics = DiagnosticsContextBuilder::new(
        ActivityId::from_string("transport-retry-io-disabled".to_owned()),
        Arc::new(DiagnosticsOptions::default()),
    );
    let result_without_retry = run_pipeline(
        test_request(Some(Instant::now() + Duration::from_secs(2))),
        &PipelineTestContext {
            transport: &client_without_retry,
            allow_sent_transport_retry: false,
            credential: &credential,
            user_agent: &user_agent,
            client_id: &TEST_CLIENT_ID,
            pipeline_type: PipelineKind::DataPlane,
            transport_security: TransportSecurity::Secure,
            endpoint_key: test_endpoint_key(),
            account_name: None,
            collection_rid: None,
            max_throttle_attempts: 9,
            max_throttle_wait_time: Duration::from_secs(30),
            max_throttle_per_retry_delay: METADATA_MAX_PER_RETRY_DELAY,
        },
        &mut diagnostics,
    )
    .await;

    match result_without_retry.outcome {
        TransportOutcome::TransportError {
            error,
            request_sent,
            ..
        } => {
            assert!(error.to_string().contains("first io shard failed"));
            assert_eq!(request_sent, RequestSentStatus::Unknown);
        }
        other => panic!("expected transport error, got {other:?}"),
    }

    let client_with_retry = scripted_transport(
        crate::error::status_codes::TRANSPORT_IO_FAILED,
        "first io shard failed",
        crate::error::status_codes::TRANSPORT_IO_FAILED,
        "second io shard failed",
    );
    let mut diagnostics = DiagnosticsContextBuilder::new(
        ActivityId::from_string("transport-retry-io-enabled".to_owned()),
        Arc::new(DiagnosticsOptions::default()),
    );
    let result_with_retry = run_pipeline(
        test_request(Some(Instant::now() + Duration::from_secs(2))),
        &PipelineTestContext {
            transport: &client_with_retry,
            allow_sent_transport_retry: true,
            credential: &credential,
            user_agent: &user_agent,
            client_id: &TEST_CLIENT_ID,
            pipeline_type: PipelineKind::DataPlane,
            transport_security: TransportSecurity::Secure,
            endpoint_key: test_endpoint_key(),
            account_name: None,
            collection_rid: None,
            max_throttle_attempts: 9,
            max_throttle_wait_time: Duration::from_secs(30),
            max_throttle_per_retry_delay: METADATA_MAX_PER_RETRY_DELAY,
        },
        &mut diagnostics,
    )
    .await;

    match result_with_retry.outcome {
        TransportOutcome::TransportError { error, .. } => {
            assert!(error.to_string().contains("second io shard failed"));
        }
        other => panic!("expected transport error, got {other:?}"),
    }
}

#[tokio::test]
async fn mixed_recovery_keeps_target_and_consumes_shard_exclusion_once() {
    let failure = || {
        Err(TransportError::new(
            crate::error::CosmosError::builder()
                .with_status(crate::error::status_codes::TRANSPORT_DNS_FAILED)
                .build(),
            RequestSentStatus::Unknown,
        ))
    };
    let mut headers = azure_core::http::headers::Headers::new();
    headers.insert("x-ms-retry-after-ms", "0");
    let responses = Arc::new(Mutex::new(VecDeque::from([
        failure(),
        Ok(HttpResponse {
            status: 429,
            headers,
            body: vec![],
        }),
        failure(),
    ])));
    let captured = Arc::new(Mutex::new(Vec::new()));
    let clients: Vec<Arc<dyn TransportClient>> = (1..=2)
        .map(|shard| {
            Arc::new(MixedRecoveryTransport {
                shard,
                responses: responses.clone(),
                requests: captured.clone(),
            }) as Arc<dyn TransportClient>
        })
        .collect();
    let pool = crate::options::ConnectionPoolOptions::builder()
        .with_max_http2_streams_per_client(1)
        .with_http2_fan_out_threshold_percent(100)
        .with_min_http2_connections_per_endpoint(2)
        .with_max_http2_connections_per_endpoint(2)
        .build()
        .unwrap();
    let transport = AdaptiveTransport::from_config(
        &pool,
        Arc::new(ScriptedFactory::new(clients)),
        HttpClientConfig::dataplane_gateway(&pool, crate::diagnostics::TransportHttpVersion::Http2),
    )
    .unwrap();
    let request = test_request(None);
    let target = request.url.clone();
    let mut diagnostics = DiagnosticsContextBuilder::new(
        ActivityId::from_string("mixed-recovery".to_owned()),
        Arc::new(DiagnosticsOptions::default()),
    );
    let result = run_pipeline(
        request,
        &PipelineTestContext {
            transport: &transport,
            credential: &Credential::from(azure_core::credentials::Secret::new("dGVzdA==")),
            allow_sent_transport_retry: false,
            user_agent: &azure_core::http::headers::HeaderValue::from_static("test-agent"),
            client_id: &TEST_CLIENT_ID,
            pipeline_type: PipelineKind::DataPlane,
            transport_security: TransportSecurity::Secure,
            endpoint_key: test_endpoint_key(),
            account_name: None,
            collection_rid: None,
            max_throttle_attempts: 9,
            max_throttle_wait_time: Duration::from_secs(30),
            max_throttle_per_retry_delay: METADATA_MAX_PER_RETRY_DELAY,
        },
        &mut diagnostics,
    )
    .await;
    assert!(matches!(
        result.outcome,
        TransportOutcome::TransportError {
            request_sent: RequestSentStatus::NotSent,
            ..
        }
    ));
    let requests = captured.lock().unwrap();
    assert_eq!(
        requests.iter().map(|(shard, _)| *shard).collect::<Vec<_>>(),
        vec![1, 2, 1]
    );
    for (_, request) in requests.iter() {
        assert_eq!(request.url, target);
        assert_eq!(request.method, azure_core::http::Method::Get);
        assert_eq!(request.timeout, None);
        assert!(request
            .headers
            .get_optional_str(&azure_core::http::headers::AUTHORIZATION)
            .is_some());
    }
    let completed = diagnostics.complete();
    let records = completed.requests();
    assert_eq!(records.len(), 3);
    for (record, context, count) in [
        (&records[0], ExecutionContext::Initial, 0),
        (&records[1], ExecutionContext::TransportRetry, 1),
        (&records[2], ExecutionContext::TransportRetry, 1),
    ] {
        assert_eq!(record.execution_context(), context);
        assert_eq!(record.local_shard_retry_count(), count);
        assert_eq!(record.failed_transport_shards().len(), count as usize);
    }
}
