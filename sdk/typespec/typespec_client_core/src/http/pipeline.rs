// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{
    policies::{CustomHeadersPolicy, LoggingPolicy, Policy, TransportPolicy},
    BufResponse, ClientOptions, Context, PipelineOptions, Request,
};
use std::sync::Arc;

/// Execution pipeline.
///
/// A pipeline follows a precise flow:
///
/// 1. Client library-specified per-call policies are executed. Per-call policies can fail and bail out of the pipeline
///    immediately.
/// 2. User-specified per-call policies in [`ClientOptions::per_call_policies`] are executed.
/// 3. The retry policy is executed. It allows to re-execute the following policies.
/// 4. The [`CustomHeadersPolicy`] is executed
/// 5. Client library-specified per-retry policies. Per-retry polices are always executed at least once but are
///    re-executed in case of retries.
/// 6. User-specified per-retry policies in [`ClientOptions::per_try_policies`] are executed.
/// 7. The transport policy is executed. Transport policy is always the last policy and is the policy that
///    actually constructs the [`BufResponse`] to be passed up the pipeline.
///
/// A pipeline is immutable. In other words a policy can either succeed and call the following
/// policy of fail and return to the calling policy. Arbitrary policy "skip" must be avoided (but
/// cannot be enforced by code). All policies except Transport policy can assume there is another following policy (so
/// `self.pipeline[0]` is always valid).
#[derive(Debug, Clone)]
pub struct Pipeline {
    pipeline: Vec<Arc<dyn Policy>>,
}

/// Options for the [`Pipeline::send`] function.
#[derive(Debug, Default)]
pub struct PipelineSendOptions {}

impl Pipeline {
    /// Creates a new pipeline with user-specified and client library-specified policies.
    ///
    /// # Arguments
    /// * `options` - The client options.
    /// * `per_call_policies` - Policies to be executed per call, before the policies in `ClientOptions::per_call_policies`.
    /// * `per_try_policies` - Policies to be executed per try, before the policies in `ClientOptions::per_try_policies`.
    /// * `pipeline_options` - Additional options for the pipeline.
    ///
    pub fn new(
        options: ClientOptions,
        per_call_policies: Vec<Arc<dyn Policy>>,
        per_try_policies: Vec<Arc<dyn Policy>>,
        pipeline_options: Option<PipelineOptions>,
    ) -> Self {
        let mut pipeline: Vec<Arc<dyn Policy>> = Vec::with_capacity(
            options.per_call_policies.len()
                + per_call_policies.len()
                + options.per_try_policies.len()
                + per_try_policies.len()
                + 2,
        );

        pipeline.extend_from_slice(&per_call_policies);
        pipeline.extend_from_slice(&options.per_call_policies);

        let pipeline_options = pipeline_options.unwrap_or_default();

        let retry_policy = options.retry.to_policy(pipeline_options.retry_headers);
        pipeline.push(retry_policy);

        pipeline.push(Arc::new(CustomHeadersPolicy::default()));

        pipeline.push(Arc::new(LoggingPolicy::new(options.logging)));

        pipeline.extend_from_slice(&per_try_policies);
        pipeline.extend_from_slice(&options.per_try_policies);

        let transport: Arc<dyn Policy> =
            Arc::new(TransportPolicy::new(options.transport.unwrap_or_default()));
        pipeline.push(transport);

        Self { pipeline }
    }

    /// Gets the policies in the order a [`Request`] is processed.
    pub fn policies(&self) -> &[Arc<dyn Policy>] {
        &self.pipeline
    }

    /// Sends a [`Request`] through each configured [`Policy`] and gets a [`BufResponse`] that is processed by each policy in reverse.
    pub async fn send(
        &self,
        ctx: &Context<'_>,
        request: &mut Request,
        _options: Option<PipelineSendOptions>,
    ) -> crate::Result<BufResponse> {
        self.pipeline[0]
            .send(ctx, request, &self.pipeline[1..])
            .await
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        http::{
            headers::{Headers, RETRY_AFTER},
            policies::{PolicyResult, RetryHeaders},
            BufResponse, JsonFormat, Method, Response, StatusCode, Transport,
        },
        stream::BytesStream,
        Bytes,
    };
    use serde::Deserialize;

    #[tokio::test]
    async fn deserializes_response() {
        #[derive(Debug)]
        struct Responder {}

        #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
        #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
        impl Policy for Responder {
            async fn send(
                &self,
                _ctx: &Context,
                _request: &mut Request,
                _next: &[Arc<dyn Policy>],
            ) -> PolicyResult {
                let buffer = Bytes::from_static(br#"{"foo":1,"bar":"baz"}"#);
                let stream: BytesStream = buffer.into();
                let response = BufResponse::new(StatusCode::Ok, Headers::new(), Box::pin(stream));
                Ok(std::future::ready(response).await)
            }
        }

        #[derive(Debug, Deserialize)]
        struct Model {
            foo: i32,
            bar: String,
        }

        // Simulated service method
        async fn service_method() -> crate::Result<Response<Model, JsonFormat>> {
            let options = ClientOptions {
                transport: Some(Transport::with_policy(Arc::new(Responder {}))),
                ..Default::default()
            };
            let pipeline_options = PipelineOptions {
                retry_headers: RetryHeaders {
                    retry_headers: vec![RETRY_AFTER],
                },
            };
            let pipeline = Pipeline::new(options, Vec::new(), Vec::new(), Some(pipeline_options));
            let mut request = Request::new("http://localhost".parse().unwrap(), Method::Get);
            let raw_response = pipeline
                .send(&Context::default(), &mut request, None)
                .await?;
            Ok(raw_response.into())
        }

        let model = service_method().await.unwrap().into_body().await.unwrap();

        assert_eq!(1, model.foo);
        assert_eq!("baz", &model.bar);
    }
}
