// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Custom Cosmos DB HTTP pipeline without azure_core default policies.
//!
//! This pipeline provides full control over request processing, bypassing
//! azure_core's default retry, logging, and telemetry policies. Cosmos DB
//! has its own retry logic and telemetry requirements.

use super::tracked_transport::EventEmitter;
use crate::diagnostics::RequestEventType;
use azure_core::http::{
    policies::{Policy, PolicyResult},
    Context, RawResponse, Request, Transport,
};
use std::sync::Arc;

/// A custom HTTP pipeline for Cosmos DB requests.
///
/// Unlike [`azure_core::http::Pipeline`], this pipeline does not inject any
/// default policies. It gives the Cosmos driver full control over:
/// - Retry behavior (Cosmos has specific retry requirements for 429, 503, etc.)
/// - Request instrumentation and telemetry
/// - Error handling
///
/// The pipeline executes policies in order, with the transport policy always last.
#[derive(Debug, Clone)]
pub(crate) struct CosmosPipeline {
    policies: Vec<Arc<dyn Policy>>,
}

impl CosmosPipeline {
    /// Creates a new pipeline from a list of policies and a transport.
    ///
    /// The transport policy is automatically appended as the final policy.
    /// A tracked transport policy is used to emit request lifecycle events.
    ///
    /// # Arguments
    ///
    /// * `policies` - Policies to execute in order before the transport
    /// * `transport` - The HTTP transport to use for sending requests
    pub(crate) fn new(policies: Vec<Arc<dyn Policy>>, transport: Transport) -> Self {
        let mut all_policies = policies;
        // Use our tracked transport policy instead of the standard TransportPolicy
        all_policies.push(Arc::new(TrackedTransportPolicy::new(transport)));
        Self {
            policies: all_policies,
        }
    }

    /// Sends a request through the pipeline and returns the buffered response.
    ///
    /// This method buffers the entire response body before returning.
    /// It emits [`RequestEventType::TransportComplete`] after the body is fully buffered.
    ///
    /// To enable event tracking, insert an [`EventEmitter`] into the context
    /// before calling this method. Events will be emitted to the emitter's
    /// channel during request processing.
    pub(crate) async fn send(
        &self,
        ctx: &Context<'_>,
        request: &mut Request,
    ) -> azure_core::Result<RawResponse> {
        let async_response = self.policies[0]
            .send(ctx, request, &self.policies[1..])
            .await?;

        // Buffer the entire response body
        let response = async_response.try_into_raw_response().await?;

        // Emit TransportComplete now that headers AND body are fully received
        if let Some(emitter) = ctx.value::<EventEmitter>() {
            emitter.emit_type(RequestEventType::TransportComplete);
        }

        Ok(response)
    }

}

/// A transport policy that emits request lifecycle events.
///
/// This policy wraps the standard transport and emits events at key points:
/// - `TransportStart` - Before calling the underlying transport
/// - `ResponseHeadersReceived` - When response headers arrive (body still streaming)
/// - `TransportFailed` - On error, with details about the failure
///
/// Note: `TransportComplete` is NOT emitted here. It is emitted by
/// [`CosmosPipeline::send()`] after the response body is fully buffered.
///
/// Events are emitted to an [`EventEmitter`] if one is present in the context.
///
/// # Limitations
///
/// Due to reqwest's high-level abstraction, we cannot track fine-grained
/// connection events (DNS resolution, TLS handshake, etc.). We only know
/// when transport starts and ends.
#[derive(Debug)]
struct TrackedTransportPolicy {
    transport: Transport,
}

impl TrackedTransportPolicy {
    fn new(transport: Transport) -> Self {
        Self { transport }
    }

    fn get_emitter<'a>(ctx: &'a Context<'_>) -> Option<&'a EventEmitter> {
        ctx.value::<EventEmitter>()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl Policy for TrackedTransportPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        // This must be the last policy
        assert_eq!(
            0,
            next.len(),
            "TrackedTransportPolicy must be the last policy"
        );

        let emitter = Self::get_emitter(ctx);

        // Emit: Transport is starting
        // From here, reqwest handles DNS, connection pool, TLS, and sending internally.
        // We cannot distinguish these phases with reqwest's API.
        if let Some(e) = emitter {
            e.emit_type(RequestEventType::TransportStart);
        }

        // Send the request through the underlying transport
        let result = self.transport.send(ctx, request).await;

        match &result {
            Ok(_response) => {
                // Response headers received - body is still a stream at this point.
                // TransportComplete will be emitted by CosmosPipeline::send() after buffering.
                if let Some(e) = emitter {
                    e.emit_type(RequestEventType::ResponseHeadersReceived);
                }
            }
            Err(err) => {
                // Transport failed - emit failure event with error details.
                // Retry safety analysis is done via RequestSentStatus in
                // request_diagnostics.rs and RequestEventType::indicates_request_sent().
                if let Some(e) = emitter {
                    e.emit_with_details(RequestEventType::TransportFailed, err.to_string());
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::{policies::PolicyResult, response::AsyncRawResponse, Method, Url};

    /// Mock transport policy for testing.
    #[derive(Debug)]
    struct MockTransport {
        response_status: u16,
    }

    #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    impl Policy for MockTransport {
        async fn send(
            &self,
            _ctx: &Context,
            _request: &mut Request,
            _next: &[Arc<dyn Policy>],
        ) -> PolicyResult {
            use azure_core::http::{headers::Headers, StatusCode};

            let status = StatusCode::from(self.response_status);
            let headers = Headers::new();
            let body = azure_core::Bytes::new();

            Ok(AsyncRawResponse::from_bytes(status, headers, body))
        }
    }

    /// Mock policy that records when it's called.
    #[derive(Debug)]
    struct RecordingPolicy {
        name: &'static str,
    }

    #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    impl Policy for RecordingPolicy {
        async fn send(
            &self,
            ctx: &Context,
            request: &mut Request,
            next: &[Arc<dyn Policy>],
        ) -> PolicyResult {
            // Add a header to prove we ran
            request.headers_mut().insert(
                azure_core::http::headers::HeaderName::from_static("x-test-policy"),
                azure_core::http::headers::HeaderValue::from(self.name.to_string()),
            );
            next[0].send(ctx, request, &next[1..]).await
        }
    }

    #[tokio::test]
    async fn pipeline_executes_policies_in_order() {
        let policy: Arc<dyn Policy> = Arc::new(RecordingPolicy { name: "test" });
        let transport = Transport::with_policy(Arc::new(MockTransport {
            response_status: 200,
        }));

        let pipeline = CosmosPipeline::new(vec![policy], transport);

        let url = Url::parse("https://test.documents.azure.com/").unwrap();
        let mut request = Request::new(url, Method::Get);
        let ctx = Context::default();

        let result = pipeline.send(&ctx, &mut request).await;
        assert!(result.is_ok());

        // Verify our policy ran
        assert_eq!(
            request.headers().get_optional_str(
                &azure_core::http::headers::HeaderName::from_static("x-test-policy")
            ),
            Some("test")
        );
    }

    #[tokio::test]
    async fn pipeline_with_no_extra_policies() {
        let transport = Transport::with_policy(Arc::new(MockTransport {
            response_status: 200,
        }));

        // Create pipeline with only the transport (no extra policies)
        let pipeline = CosmosPipeline::new(vec![], transport);

        let url = Url::parse("https://test.documents.azure.com/").unwrap();
        let mut request = Request::new(url, Method::Get);
        let ctx = Context::default();

        let result = pipeline.send(&ctx, &mut request).await;
        assert!(result.is_ok());
    }
}
