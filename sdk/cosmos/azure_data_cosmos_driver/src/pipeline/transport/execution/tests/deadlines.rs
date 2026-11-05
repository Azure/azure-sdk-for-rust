// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    gateway_v2_context, gateway_v2_diagnostics, gateway_v2_transport_request, run_pipeline,
    ActivityId, AdaptiveTransport, Arc, CosmosEndpoint, Credential, DefaultConsistencyLevel,
    DiagnosticsContextBuilder, DiagnosticsOptions, Duration, ExecutionContext,
    GatewayV2MockTransportClient, HangingTransportClient, Instant, OperationType, PipelineKind,
    PipelineTestContext, RequestSentStatus, ResourceType, TransportMode, TransportOutcome,
    TransportRequest, TransportSecurity, METADATA_MAX_PER_RETRY_DELAY, TEST_CLIENT_ID,
};

#[tokio::test]
async fn execute_transport_pipeline_times_out_in_flight_request() {
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());
    let request = TransportRequest {
        method: azure_core::http::Method::Get,
        endpoint: endpoint.clone(),
        transport_mode: TransportMode::Gateway,
        operation_type: OperationType::Read,
        effective_partition_key: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        url: endpoint.url().clone(),
        headers: azure_core::http::headers::Headers::new(),
        #[cfg(feature = "unstable_dtx")]
        resource_type: ResourceType::Database,
        body: None,
        auth_context: crate::pipeline::transport::AuthorizationContext::new(
            azure_core::http::Method::Get,
            ResourceType::Database,
            "",
        ),
        execution_context: ExecutionContext::Initial,
        routing_fallback: None,
        deadline: Some(Instant::now() + Duration::from_millis(100)),
    };
    let client = AdaptiveTransport::Gateway(Arc::new(HangingTransportClient {
        delay: Duration::from_secs(2),
    }));
    let mut diagnostics = DiagnosticsContextBuilder::new(
        ActivityId::from_string("transport-timeout".to_owned()),
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
            pipeline_type: PipelineKind::Metadata,
            transport_security: TransportSecurity::Secure,
            endpoint_key: endpoint.endpoint_key(),
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
        TransportOutcome::DeadlineExceeded { .. }
    ));

    let completed = diagnostics.complete();
    let requests = completed.requests();
    assert_eq!(requests.len(), 1);
    assert!(requests[0].timed_out());
}

#[tokio::test]
async fn expired_deadline_does_not_start_or_send_an_attempt() {
    let mock = Arc::new(GatewayV2MockTransportClient::new(Vec::new()));
    let client = AdaptiveTransport::Gateway(mock.clone());
    let mut request = gateway_v2_transport_request(TransportMode::Gateway);
    request.deadline = Some(Instant::now() - Duration::from_millis(1));
    let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
    let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");
    let context = gateway_v2_context(
        &client,
        request.endpoint.endpoint_key(),
        None,
        &credential,
        &user_agent,
    );
    let mut diagnostics = gateway_v2_diagnostics();

    let result = run_pipeline(request, &context, &mut diagnostics).await;

    assert!(matches!(
        result.outcome,
        TransportOutcome::DeadlineExceeded {
            request_sent: RequestSentStatus::NotSent,
        }
    ));
    assert!(mock.requests().is_empty());
    assert!(diagnostics.complete().requests().is_empty());
}
