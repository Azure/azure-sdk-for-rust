// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    finalize_gateway_v2_response, gateway_v2_context, gateway_v2_diagnostics,
    gateway_v2_error_frame, gateway_v2_response, gateway_v2_transport_request, run_pipeline,
    write_gateway_v2_u32_token, AdaptiveTransport, Arc, Credential, GatewayV2MockTransportClient,
    HttpResponse, RequestSentStatus, TransportMode, TransportOutcome, NO_RETRY_449,
};

/// Regression: a Gateway 2.0 error response arrives as an HTTP-error
/// envelope (404) wrapping an RNTBD `404/1002` frame. The pipeline must
/// unwrap the body and surface the real sub-status rather than a bare 404.
#[test]
fn finalize_unwraps_gateway_v2_error_envelope() {
    let executed = finalize_gateway_v2_response(
        azure_core::http::StatusCode::NotFound,
        gateway_v2_error_frame(404, 1002),
    );

    match executed.result.outcome {
        TransportOutcome::HttpError { status, .. } => {
            assert_eq!(
                status.status_code(),
                azure_core::http::StatusCode::NotFound,
                "expected the unwrapped 404 status, got {status:?}"
            );
            assert_eq!(
                status.sub_status().map(|s| s.value()),
                Some(1002),
                "expected sub-status 1002 recovered from the RNTBD body, got {status:?}"
            );
        }
        other => panic!("expected HttpError(404/1002), got {other:?}"),
    }
}

/// A non-2xx thin-client envelope whose body is *not* an RNTBD frame (a
/// genuine proxy-level error) must fall back to the bare envelope status
/// instead of being masked as a transport failure.
#[test]
fn finalize_falls_back_on_non_rntbd_gateway_v2_error_envelope() {
    let executed = finalize_gateway_v2_response(
        azure_core::http::StatusCode::Unauthorized,
        b"unauthorized".to_vec(),
    );

    match executed.result.outcome {
        TransportOutcome::HttpError { status, .. } => {
            assert_eq!(
                status.status_code(),
                azure_core::http::StatusCode::Unauthorized,
                "expected the bare envelope 401 to be preserved, got {status:?}"
            );
        }
        other => panic!("expected HttpError(401), got {other:?}"),
    }
}

#[tokio::test]
async fn gateway_v2_pipeline_wraps_request_and_unwraps_success_response() {
    let mock = Arc::new(GatewayV2MockTransportClient::new(vec![
        gateway_v2_response(200, |_| {}, b"{}"),
    ]));
    let client = AdaptiveTransport::Gateway(mock.clone());
    let request = gateway_v2_transport_request(TransportMode::GatewayV2);
    let endpoint_key = request.endpoint.endpoint_key();
    let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
    let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");
    let mut diagnostics = gateway_v2_diagnostics();

    let result = run_pipeline(
        request,
        &gateway_v2_context(
            &client,
            endpoint_key,
            Some("account".to_owned()),
            &credential,
            &user_agent,
        ),
        &mut diagnostics,
    )
    .await;

    match result.outcome {
        TransportOutcome::Success { status, body, .. } => {
            assert_eq!(status.status_code(), azure_core::http::StatusCode::Ok);
            assert_eq!(body, b"{}".to_vec());
        }
        other => panic!("expected success, got {other:?}"),
    }
    let captured = mock.requests();
    assert_eq!(captured.len(), 1);
    assert_eq!(captured[0].method, azure_core::http::Method::Post);
    assert_eq!(
        captured[0]
            .headers
            .get_optional_str(&azure_core::http::headers::AUTHORIZATION),
        None
    );
    assert_eq!(
        captured[0]
            .headers
            .get_optional_str(&azure_core::http::headers::USER_AGENT),
        Some("test-agent")
    );
    assert!(captured[0]
        .body
        .as_ref()
        .is_some_and(|body| !body.is_empty()));
    // Gateway 2.0 never carries the V1-only no-retry-449 header.
    assert_eq!(
        captured[0].headers.get_optional_str(&NO_RETRY_449),
        None,
        "Gateway 2.0 must not send x-ms-noretry-449"
    );
}

#[tokio::test]
async fn gateway_v2_pipeline_leaves_standard_gateway_request_unwrapped() {
    let mock = Arc::new(GatewayV2MockTransportClient::new(vec![HttpResponse {
        status: 200,
        headers: azure_core::http::headers::Headers::new(),
        body: b"plain".to_vec(),
    }]));
    let client = AdaptiveTransport::Gateway(mock.clone());
    let request = gateway_v2_transport_request(TransportMode::Gateway);
    let endpoint_key = request.endpoint.endpoint_key();
    let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
    let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");
    let mut diagnostics = gateway_v2_diagnostics();

    let result = run_pipeline(
        request,
        &gateway_v2_context(&client, endpoint_key, None, &credential, &user_agent),
        &mut diagnostics,
    )
    .await;

    match result.outcome {
        TransportOutcome::Success { body, .. } => assert_eq!(body, b"plain".to_vec()),
        other => panic!("expected success, got {other:?}"),
    }
    let captured = mock.requests();
    assert_eq!(captured.len(), 1);
    assert_eq!(captured[0].method, azure_core::http::Method::Get);
    assert!(captured[0]
        .headers
        .get_optional_str(&azure_core::http::headers::AUTHORIZATION)
        .is_some());
    // Gateway V1 disables CGW's server-side 449 retry so the SDK owns
    // RetryWith, matching Java's RxGatewayStoreModel.
    assert_eq!(
        captured[0].headers.get_optional_str(&NO_RETRY_449),
        Some("true"),
        "Gateway V1 must send x-ms-noretry-449: true"
    );
}

#[tokio::test]
async fn gateway_v2_pipeline_decode_failure_is_sent_transport_error() {
    let mock = Arc::new(GatewayV2MockTransportClient::new(vec![HttpResponse {
        status: 200,
        headers: azure_core::http::headers::Headers::new(),
        body: vec![1, 2, 3],
    }]));
    let client = AdaptiveTransport::Gateway(mock);
    let request = gateway_v2_transport_request(TransportMode::GatewayV2);
    let endpoint_key = request.endpoint.endpoint_key();
    let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
    let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");
    let mut diagnostics = gateway_v2_diagnostics();

    let result = run_pipeline(
        request,
        &gateway_v2_context(
            &client,
            endpoint_key,
            Some("account".to_owned()),
            &credential,
            &user_agent,
        ),
        &mut diagnostics,
    )
    .await;

    match result.outcome {
        TransportOutcome::TransportError {
            status,
            request_sent,
            ..
        } => {
            assert_eq!(status, crate::error::status_codes::TRANSPORT_GENERATED_503);
            assert_eq!(request_sent, RequestSentStatus::Sent);
        }
        other => panic!("expected transport error, got {other:?}"),
    }
}

#[tokio::test]
async fn gateway_v2_pipeline_outer_502_propagates_unchanged_without_unwrap() {
    let mock = Arc::new(GatewayV2MockTransportClient::new(vec![HttpResponse {
        status: 502,
        headers: azure_core::http::headers::Headers::new(),
        body: vec![],
    }]));
    let client = AdaptiveTransport::Gateway(mock.clone());
    let request = gateway_v2_transport_request(TransportMode::GatewayV2);
    let endpoint_key = request.endpoint.endpoint_key();
    let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
    let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");
    let mut diagnostics = gateway_v2_diagnostics();

    let result = run_pipeline(
        request,
        &gateway_v2_context(
            &client,
            endpoint_key,
            Some("account".to_owned()),
            &credential,
            &user_agent,
        ),
        &mut diagnostics,
    )
    .await;

    match result.outcome {
        TransportOutcome::HttpError {
            status,
            body,
            request_sent,
            ..
        } => {
            assert_eq!(u16::from(status.status_code()), 502);
            assert_eq!(status.sub_status(), None);
            assert_eq!(body, Vec::<u8>::new());
            assert_eq!(request_sent, RequestSentStatus::Sent);
        }
        other => panic!("expected HTTP error, got {other:?}"),
    }
    assert_eq!(mock.requests().len(), 1);
}

#[tokio::test]
async fn gateway_v2_pipeline_inner_401_surfaces_as_inner_status() {
    let mock = Arc::new(GatewayV2MockTransportClient::new(vec![
        gateway_v2_response(401, |_| {}, b""),
    ]));
    let client = AdaptiveTransport::Gateway(mock.clone());
    let request = gateway_v2_transport_request(TransportMode::GatewayV2);
    let endpoint_key = request.endpoint.endpoint_key();
    let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
    let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");
    let mut diagnostics = gateway_v2_diagnostics();

    let result = run_pipeline(
        request,
        &gateway_v2_context(
            &client,
            endpoint_key,
            Some("account".to_owned()),
            &credential,
            &user_agent,
        ),
        &mut diagnostics,
    )
    .await;

    match result.outcome {
        TransportOutcome::HttpError {
            status,
            body,
            request_sent,
            ..
        } => {
            assert_eq!(
                status.status_code(),
                azure_core::http::StatusCode::Unauthorized
            );
            assert_eq!(status.sub_status(), None);
            assert_eq!(body, Vec::<u8>::new());
            assert_eq!(request_sent, RequestSentStatus::Sent);
        }
        other => panic!("expected HTTP error, got {other:?}"),
    }
    assert_eq!(mock.requests().len(), 1);
}

#[tokio::test]
async fn gateway_v2_pipeline_uses_inner_retry_after_for_throttle_retry() {
    let mock = Arc::new(GatewayV2MockTransportClient::new(vec![
        gateway_v2_response(
            429,
            |bytes| write_gateway_v2_u32_token(bytes, 0x000C, 0),
            b"",
        ),
        gateway_v2_response(200, |_| {}, b"{}"),
    ]));
    let client = AdaptiveTransport::Gateway(mock.clone());
    let request = gateway_v2_transport_request(TransportMode::GatewayV2);
    let endpoint_key = request.endpoint.endpoint_key();
    let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
    let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");
    let mut diagnostics = gateway_v2_diagnostics();

    let result = run_pipeline(
        request,
        &gateway_v2_context(
            &client,
            endpoint_key,
            Some("account".to_owned()),
            &credential,
            &user_agent,
        ),
        &mut diagnostics,
    )
    .await;

    assert!(matches!(result.outcome, TransportOutcome::Success { .. }));
    assert_eq!(mock.requests().len(), 2);
}
