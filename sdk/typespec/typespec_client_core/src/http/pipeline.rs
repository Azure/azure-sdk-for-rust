// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{
    policies::{CustomHeadersPolicy, Policy, TransportPolicy},
    ClientOptions, Context, RawResponse, Request, RetryOptions,
};
use std::sync::Arc;

/// Execution pipeline.
///
/// A pipeline follows a precise flow:
///
/// 1. Client library-specified per-call policies are executed. Per-call policies can fail and bail out of the pipeline
///    immediately.
/// 2. User-specified per-call policies are executed.
/// 3. Built-in per-call policies are executed.
/// 4. The retry policy is executed. It allows to re-execute the following policies.
/// 5. Client library-specified per-retry policies. Per-retry polices are always executed at least once but are re-executed
///    in case of retries.
/// 6. User-specified per-retry policies are executed.
/// 7. The authorization policy is executed. Authorization can depend on the HTTP headers and/or the request body so it
///    must be executed right before sending the request to the transport. Also, the authorization
///    can depend on the current time so it must be executed at every retry.
/// 8. The transport policy is executed. Transport policy is always the last policy and is the policy that
///    actually constructs the `Response` to be passed up the pipeline.
///
/// A pipeline is immutable. In other words a policy can either succeed and call the following
/// policy of fail and return to the calling policy. Arbitrary policy "skip" must be avoided (but
/// cannot be enforced by code). All policies except Transport policy can assume there is another following policy (so
/// `self.pipeline[0]` is always valid).
#[derive(Debug, Clone)]
pub struct Pipeline {
    pipeline: Vec<Arc<dyn Policy>>,
}

impl Pipeline {
    /// Creates a new pipeline given the client library crate name and version,
    /// along with user-specified and client library-specified policies.
    pub fn new(
        options: ClientOptions,
        per_call_policies: Vec<Arc<dyn Policy>>,
        per_retry_policies: Vec<Arc<dyn Policy>>,
    ) -> Self {
        let mut pipeline: Vec<Arc<dyn Policy>> = Vec::with_capacity(
            options.per_call_policies.len()
                + per_call_policies.len()
                + options.per_try_policies.len()
                + per_retry_policies.len()
                + 2,
        );

        pipeline.extend_from_slice(&per_call_policies);
        pipeline.extend_from_slice(&options.per_call_policies);

        pipeline.push(Arc::new(CustomHeadersPolicy::default()));

        // TODO: Consider whether this should be initially customizable as we onboard more services.
        let retry_policy = RetryOptions::default().to_policy();
        pipeline.push(retry_policy);

        pipeline.extend_from_slice(&per_retry_policies);
        pipeline.extend_from_slice(&options.per_try_policies);

        let transport: Arc<dyn Policy> =
            Arc::new(TransportPolicy::new(options.transport.unwrap_or_default()));

        pipeline.push(transport);

        Self { pipeline }
    }

    pub fn replace_policy(&mut self, policy: Arc<dyn Policy>, position: usize) -> Arc<dyn Policy> {
        std::mem::replace(&mut self.pipeline[position], policy)
    }

    pub fn policies(&self) -> &[Arc<dyn Policy>] {
        &self.pipeline
    }

    pub async fn send(
        &self,
        ctx: &Context<'_>,
        request: &mut Request,
    ) -> crate::Result<RawResponse> {
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
            headers::Headers, policies::PolicyResult, JsonFormat, Method, RawResponse, Response,
            StatusCode, TransportOptions,
        },
        stream::BytesStream,
    };
    use bytes::Bytes;
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
                let response = RawResponse::new(StatusCode::Ok, Headers::new(), Box::pin(stream));
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
                transport: Some(TransportOptions::new_custom_policy(Arc::new(Responder {}))),
                ..Default::default()
            };
            let pipeline = Pipeline::new(options, Vec::new(), Vec::new());
            let mut request = Request::new("http://localhost".parse().unwrap(), Method::Get);
            let raw_response = pipeline.send(&Context::default(), &mut request).await?;
            Ok(raw_response.into())
        }

        let model = service_method().await.unwrap().into_body().await.unwrap();

        assert_eq!(1, model.foo);
        assert_eq!("baz", &model.bar);
    }
}
