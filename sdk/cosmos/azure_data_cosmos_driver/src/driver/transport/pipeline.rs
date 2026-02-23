// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Custom Cosmos DB HTTP pipeline without azure_core default policies.
//!
//! This pipeline provides full control over request processing, bypassing
//! azure_core's default retry, logging, and telemetry policies. Cosmos DB
//! has its own retry logic and telemetry requirements.

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

        Ok(response)
    }
}

/// A transport policy wrapper.
#[derive(Debug)]
struct TrackedTransportPolicy {
    transport: Transport,
}

impl TrackedTransportPolicy {
    fn new(transport: Transport) -> Self {
        Self { transport }
    }
}

#[async_trait::async_trait]
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

        // Send the request through the underlying transport
        self.transport.send(ctx, request).await
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

    #[async_trait::async_trait]
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

    #[async_trait::async_trait]
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
