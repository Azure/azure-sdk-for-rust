// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    gateway_v2_context, gateway_v2_diagnostics, gateway_v2_transport_request, run_pipeline,
    test_request, AdaptiveTransport, Arc, Clock, Credential, Duration,
    GatewayV2MockTransportClient, HangingTransportClient, PipelineKind, RequestSentStatus,
    TransportMode, TransportOutcome,
};
use std::task::Poll;

#[tokio::test]
async fn execute_transport_pipeline_times_out_in_flight_request() {
    let clock = Clock::new_test();
    clock.advance(Duration::from_secs(3600));
    let request = test_request(Some(clock.now() + Duration::from_millis(100)));
    let client = AdaptiveTransport::Gateway(Arc::new(HangingTransportClient));
    let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
    let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");
    let mut context = gateway_v2_context(
        &client,
        request.endpoint.endpoint_key(),
        None,
        &credential,
        &user_agent,
    );
    context.pipeline_type = PipelineKind::Metadata;
    context.clock = clock.clone();
    let mut diagnostics = gateway_v2_diagnostics();

    let result = {
        let mut execution = Box::pin(run_pipeline(request, &context, &mut diagnostics));
        assert!(futures::poll!(execution.as_mut()).is_pending());
        assert_eq!(clock.pending_sleep_count(), 1);
        clock.advance(Duration::from_millis(99));
        assert!(futures::poll!(execution.as_mut()).is_pending());
        clock.advance(Duration::from_millis(1));
        let Poll::Ready(result) = futures::poll!(execution.as_mut()) else {
            panic!("dispatch must time out at the fake deadline");
        };
        result
    };

    assert!(matches!(
        result.outcome,
        TransportOutcome::DeadlineExceeded {
            request_sent: RequestSentStatus::Unknown
        }
    ));
    assert_eq!(clock.pending_sleep_count(), 0);

    let completed = diagnostics.complete();
    let requests = completed.requests();
    assert_eq!(requests.len(), 1);
    assert!(requests[0].timed_out());
}

#[tokio::test]
async fn expired_deadline_does_not_start_or_send_an_attempt() {
    let clock = Clock::new_test();
    clock.advance(Duration::from_secs(3600));
    let mock = Arc::new(GatewayV2MockTransportClient::new(Vec::new()));
    let client = AdaptiveTransport::Gateway(mock.clone());
    let mut request = gateway_v2_transport_request(TransportMode::Gateway);
    request.deadline = Some(clock.now() + Duration::from_millis(1)).into();
    let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
    let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");
    let mut context = gateway_v2_context(
        &client,
        request.endpoint.endpoint_key(),
        None,
        &credential,
        &user_agent,
    );
    context.clock = clock.clone();
    clock.advance(Duration::from_millis(1));
    let mut diagnostics = gateway_v2_diagnostics();

    let result = run_pipeline(request, &context, &mut diagnostics).await;

    assert!(matches!(
        result.outcome,
        TransportOutcome::DeadlineExceeded {
            request_sent: RequestSentStatus::NotSent,
        }
    ));
    assert!(mock.requests().is_empty());
    assert_eq!(clock.pending_sleep_count(), 0);
    assert!(diagnostics.complete().requests().is_empty());
}
