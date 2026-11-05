// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    format_transport_error_details_cosmos, gateway_v2_context, gateway_v2_diagnostics,
    gateway_v2_transport_request, run_pipeline, test_endpoint_key, test_request, ActivityId,
    AdaptiveTransport, Arc, CosmosEndpoint, Credential, DiagnosticsContextBuilder,
    DiagnosticsOptions, Duration, GatewayV2MockTransportClient, HangingTransportClient,
    HttpResponse, Instant, PipelineKind, PipelineTestContext, RequestSentStatus,
    SuccessfulTransportClient, TransportMode, TransportOutcome, TransportSecurity,
    METADATA_MAX_PER_RETRY_DELAY, TEST_CLIENT_ID,
};
use crate::diagnostics::RequestEventType;

#[tokio::test]
async fn routing_fallback_is_recorded_in_request_diagnostics() {
    use crate::driver::pipeline::components::RoutingFallbackReason;

    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());
    let mut request = test_request(None);
    request.routing_fallback =
        Some(RoutingFallbackReason::PatchVerificationReadWriteEndpointUnavailableOrExcluded);
    let client = AdaptiveTransport::Gateway(Arc::new(SuccessfulTransportClient));
    let mut diagnostics = DiagnosticsContextBuilder::new(
        ActivityId::from_string("routing-fallback".to_owned()),
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
            max_throttle_attempts: 0,
            max_throttle_wait_time: Duration::ZERO,
            max_throttle_per_retry_delay: METADATA_MAX_PER_RETRY_DELAY,
        },
        &mut diagnostics,
    )
    .await;

    assert!(matches!(result.outcome, TransportOutcome::Success { .. }));
    let completed = diagnostics.complete();
    let requests = completed.requests();
    assert_eq!(requests.len(), 1);
    let fallback = requests[0]
        .events()
        .iter()
        .find(|event| event.event_type() == &crate::diagnostics::RequestEventType::RoutingFallback)
        .expect("routing fallback event must be recorded");
    assert_eq!(
        fallback.details(),
        Some("patch_verification_read_write_endpoint_unavailable_or_excluded")
    );
}

#[tokio::test]
async fn execute_transport_pipeline_preserves_client_unauthorized_in_diagnostics() {
    let client = AdaptiveTransport::Gateway(Arc::new(HangingTransportClient {
        delay: Duration::from_secs(1),
    }));
    let mut diagnostics = DiagnosticsContextBuilder::new(
        ActivityId::from_string("transport-signing-failure".to_owned()),
        Arc::new(DiagnosticsOptions::default()),
    );

    let result = run_pipeline(
        test_request(Some(Instant::now() + Duration::from_secs(1))),
        &PipelineTestContext {
            transport: &client,
            allow_sent_transport_retry: false,
            credential: &Credential::from(azure_core::credentials::Secret::new("***not-base64***")),
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
        TransportOutcome::TransportError {
            status,
            request_sent,
            ..
        } => {
            assert_eq!(status, crate::error::status_codes::CLIENT_GENERATED_401);
            assert_eq!(
                status.sub_status(),
                Some(crate::error::status_codes::substatus::CLIENT_GENERATED_401)
            );
            assert_eq!(request_sent, RequestSentStatus::NotSent);
        }
        other => panic!("expected transport error, got {other:?}"),
    }

    let completed = diagnostics.complete();
    let requests = completed.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(
        requests[0].status(),
        &crate::error::status_codes::CLIENT_GENERATED_401
    );
    assert_eq!(requests[0].request_sent(), RequestSentStatus::NotSent);
    assert!(requests[0].events().is_empty());
}

#[test]
fn format_transport_error_details_includes_error_chain() {
    let inner = std::io::Error::new(std::io::ErrorKind::ConnectionReset, "socket reset");
    let cosmos = crate::error::CosmosError::builder()
        .with_status(crate::error::status_codes::TRANSPORT_IO_FAILED)
        .with_message("failed to execute `reqwest` request")
        .with_source(inner)
        .build();

    let details = format_transport_error_details_cosmos(&cosmos);
    assert!(details.contains("failed to execute `reqwest` request"));
    assert!(details.contains("socket reset"));
}

#[tokio::test]
async fn completion_records_events_in_wire_order() {
    for (mode, account, expected_events, sends) in [
        (
            TransportMode::Gateway,
            None,
            vec![
                RequestEventType::TransportStart,
                RequestEventType::ResponseHeadersReceived,
            ],
            1,
        ),
        (
            TransportMode::GatewayV2,
            Some("account".to_owned()),
            vec![
                RequestEventType::TransportStart,
                RequestEventType::ResponseHeadersReceived,
                RequestEventType::TransportFailed,
            ],
            1,
        ),
        (TransportMode::GatewayV2, None, vec![], 0),
    ] {
        let mock = Arc::new(GatewayV2MockTransportClient::new(vec![HttpResponse {
            status: 200,
            headers: Default::default(),
            body: b"not an RNTBD frame".to_vec(),
        }]));
        let client = AdaptiveTransport::Gateway(mock.clone());
        let request = gateway_v2_transport_request(mode);
        let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
        let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");
        let context = gateway_v2_context(
            &client,
            request.endpoint.endpoint_key(),
            account,
            &credential,
            &user_agent,
        );
        let mut diagnostics = gateway_v2_diagnostics();
        let result = run_pipeline(request, &context, &mut diagnostics).await;
        let completed = diagnostics.complete();
        assert_eq!(mock.requests().len(), sends);
        assert_eq!(completed.requests().len(), 1);
        let record = &completed.requests()[0];
        assert!(record.is_completed());
        assert_eq!(
            record
                .events()
                .iter()
                .map(|event| event.event_type())
                .collect::<Vec<_>>(),
            expected_events.iter().collect::<Vec<_>>()
        );
        if sends == 0 {
            assert!(matches!(
                result.outcome,
                TransportOutcome::TransportError {
                    status: crate::error::status_codes::CLIENT_BAD_REQUEST,
                    request_sent: RequestSentStatus::NotSent,
                    ..
                }
            ));
        }
    }
}

#[cfg(feature = "fault_injection")]
mod fault_injection {
    use super::{
        gateway_v2_context, gateway_v2_diagnostics, run_pipeline, test_request, AdaptiveTransport,
        Arc, Credential, HttpResponse, TransportOutcome,
    };
    use crate::{
        fault_injection::FaultInjectionEvaluation,
        pipeline::transport::cosmos_transport_client::{
            HttpRequest, TransportClient, TransportError,
        },
    };
    use async_trait::async_trait;
    use azure_core::{credentials::Secret, http::headers::HeaderValue};

    #[derive(Debug)]
    struct EvaluationTransport {
        enabled: bool,
        evaluations: Vec<FaultInjectionEvaluation>,
    }

    #[async_trait]
    impl TransportClient for EvaluationTransport {
        async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            assert_eq!(request.evaluation_collector.is_some(), self.enabled);
            if let Some(collector) = &request.evaluation_collector {
                collector.push_all(&mut self.evaluations.clone());
            }
            Ok(HttpResponse {
                status: 200,
                headers: Default::default(),
                body: Vec::new(),
            })
        }
    }

    #[tokio::test]
    async fn evaluations_are_recorded_after_terminal_journal_copy() {
        for (enabled, record_evaluation) in [(false, true), (true, false), (true, true)] {
            let evaluations = if record_evaluation {
                vec![FaultInjectionEvaluation::Applied {
                    rule_id: "transport-rule".to_owned(),
                }]
            } else {
                Vec::new()
            };
            let client = AdaptiveTransport::Gateway(Arc::new(EvaluationTransport {
                enabled,
                evaluations: evaluations.clone(),
            }));
            let request = test_request(None);
            let credential = Credential::from(Secret::new("dGVzdA=="));
            let user_agent = HeaderValue::from_static("test-agent");
            let context = gateway_v2_context(
                &client,
                request.endpoint.endpoint_key(),
                None,
                &credential,
                &user_agent,
            );
            let mut parent = gateway_v2_diagnostics();
            parent.set_fault_injection_enabled(enabled);
            let mut diagnostics = parent.clone_for_hedge_attempt();

            let result = run_pipeline(request, &context, &mut diagnostics).await;
            assert!(matches!(result.outcome, TransportOutcome::Success { .. }));

            // Read the journal first so finalizing the leg cannot consume its mirrored record.
            let journaled = parent.complete();
            assert_eq!(journaled.requests().len(), 1);
            assert!(journaled.requests()[0].is_completed());
            assert!(journaled.requests()[0]
                .fault_injection_evaluations()
                .is_empty());

            let completed = diagnostics.complete();
            assert_eq!(completed.requests().len(), 1);
            assert!(completed.requests()[0].is_completed());
            let expected = if enabled { evaluations } else { Vec::new() };
            assert_eq!(
                completed.requests()[0].fault_injection_evaluations(),
                expected
            );
        }
    }
}
