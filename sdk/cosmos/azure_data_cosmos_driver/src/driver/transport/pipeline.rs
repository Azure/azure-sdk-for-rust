// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Custom Cosmos DB HTTP pipeline without azure_core default policies.

use azure_core::http::{policies::Policy, Context, RawResponse, Request, Transport};
use std::sync::Arc;

use super::tracked_transport::{RequestAttemptTelemetryContext, TrackedTransportPolicy};

#[derive(Debug, Clone)]
pub(crate) struct CosmosPipeline {
    policies: Vec<Arc<dyn Policy>>,
}

impl CosmosPipeline {
    pub(crate) fn new(policies: Vec<Arc<dyn Policy>>, transport: Transport) -> Self {
        let mut all_policies = policies;
        all_policies.push(Arc::new(TrackedTransportPolicy::new(transport)));
        Self {
            policies: all_policies,
        }
    }

    pub(crate) async fn send(
        &self,
        ctx: &Context<'_>,
        request: &mut Request,
    ) -> azure_core::Result<RawResponse> {
        let async_response = self.policies[0]
            .send(ctx, request, &self.policies[1..])
            .await?;

        let response = async_response.try_into_raw_response().await?;

        if let Some(telemetry) = ctx.value::<RequestAttemptTelemetryContext>() {
            telemetry
                .sink()
                .record_event(crate::diagnostics::RequestEvent::new(
                    crate::diagnostics::RequestEventType::TransportComplete,
                ));
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::RequestEvent;
    use crate::driver::transport::tracked_transport::{
        RequestAttemptTelemetrySink, RequestSentStatus,
    };
    use azure_core::http::{policies::PolicyResult, response::AsyncRawResponse, Method, Url};
    use std::sync::Mutex;

    #[derive(Default)]
    struct TestTelemetry {
        request_reached_transport: Mutex<bool>,
        request_sent_status: Mutex<Option<RequestSentStatus>>,
        events: Mutex<Vec<RequestEvent>>,
    }

    impl RequestAttemptTelemetrySink for TestTelemetry {
        fn mark_reached_transport(&self) {
            if let Ok(mut reached) = self.request_reached_transport.lock() {
                *reached = true;
            }
        }

        fn set_request_sent_status(&self, request_sent_status: RequestSentStatus) {
            if let Ok(mut status) = self.request_sent_status.lock() {
                *status = Some(request_sent_status);
            }
        }

        fn record_event(&self, event: RequestEvent) {
            if let Ok(mut events) = self.events.lock() {
                events.push(event);
            }
        }
    }

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

    #[tokio::test]
    async fn pipeline_with_no_extra_policies() {
        let transport = Transport::with_policy(Arc::new(MockTransport {
            response_status: 200,
        }));

        let pipeline = CosmosPipeline::new(vec![], transport);

        let url = Url::parse("https://test.documents.azure.com/").unwrap();
        let mut request = Request::new(url, Method::Get);
        let mut ctx = Context::default();
        let telemetry = Arc::new(TestTelemetry::default());
        let telemetry_sink = telemetry.clone() as Arc<dyn RequestAttemptTelemetrySink>;
        ctx.insert(RequestAttemptTelemetryContext::new(telemetry_sink));

        let result = pipeline.send(&ctx, &mut request).await;
        assert!(result.is_ok());

        let reached = telemetry
            .request_reached_transport
            .lock()
            .map(|value| *value)
            .unwrap_or(false);
        assert!(reached);

        let has_sent_status = telemetry
            .request_sent_status
            .lock()
            .map(|status| status.is_some())
            .unwrap_or(false);
        assert!(has_sent_status);

        let has_events = telemetry
            .events
            .lock()
            .map(|events| !events.is_empty())
            .unwrap_or(false);
        assert!(has_events);
    }
}
