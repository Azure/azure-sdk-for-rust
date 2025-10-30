// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use typespec::http::RawResponse;

use crate::http::{
    policies::{Buffer, LoggingPolicy, Policy, TransportPolicy},
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
/// 4. Client library-specified per-retry policies. Per-retry polices are always executed at least once but are
///    re-executed in case of retries.
/// 5. User-specified per-retry policies in [`ClientOptions::per_try_policies`] are executed.
/// 6. The transport policy is executed. Transport policy is always the last policy and is the policy that
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
pub struct PipelineSendOptions;

/// Options for the [`Pipeline::stream`] function.
#[derive(Debug, Default)]
pub struct PipelineStreamOptions;

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
        // The number of policies we'll push to the pipeline Vec ourselves.
        const BUILT_IN_LEN: usize = 3;
        let mut pipeline: Vec<Arc<dyn Policy>> = Vec::with_capacity(
            per_call_policies.len()
                + options.per_call_policies.len()
                + per_try_policies.len()
                + options.per_try_policies.len()
                + BUILT_IN_LEN,
        );

        #[cfg(debug_assertions)]
        let initial_capacity = pipeline.capacity();

        pipeline.extend_from_slice(&per_call_policies);
        pipeline.extend_from_slice(&options.per_call_policies);

        let pipeline_options = pipeline_options.unwrap_or_default();

        let retry_policy = options.retry.to_policy(pipeline_options.retry_headers);
        pipeline.push(retry_policy);

        pipeline.extend_from_slice(&per_try_policies);
        pipeline.extend_from_slice(&options.per_try_policies);

        pipeline.push(Arc::new(LoggingPolicy::new(options.logging)));

        let transport: Arc<dyn Policy> =
            Arc::new(TransportPolicy::new(options.transport.unwrap_or_default()));
        pipeline.push(transport);

        // Make sure we didn't have to resize the Vec.
        #[cfg(debug_assertions)]
        debug_assert_eq!(pipeline.len(), initial_capacity);

        Self { pipeline }
    }

    /// Gets the policies in the order a [`Request`] is processed.
    pub fn policies(&self) -> &[Arc<dyn Policy>] {
        &self.pipeline
    }

    /// Sends a [`Request`] through each configured [`Policy`] and gets a [`RawResponse`] that is processed by each policy in reverse.
    pub async fn send(
        &self,
        ctx: &Context<'_>,
        request: &mut Request,
        _options: Option<PipelineSendOptions>,
    ) -> crate::Result<RawResponse> {
        // Signal the TransportPolicy to buffer the entire response.
        let mut ctx = ctx.to_borrowed();
        ctx.insert(Buffer);

        self.pipeline[0]
            .send(&ctx, request, &self.pipeline[1..])
            .await?
            .try_into_raw_response()
            .await
    }

    /// Sends a [`Request`] through each configured [`Policy`] to get a [`BufResponse`] that is processed by each policy in reverse.
    pub async fn stream(
        &self,
        ctx: &Context<'_>,
        request: &mut Request,
        _options: Option<PipelineStreamOptions>,
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
        error::{Error, ErrorKind},
        http::{
            headers::{Headers, RETRY_AFTER},
            policies::{PolicyResult, RetryHeaders},
            BufResponse, FixedRetryOptions, JsonFormat, Method, Response, RetryOptions, StatusCode,
            Transport,
        },
        stream::BytesStream,
        Bytes,
    };
    use futures::{lock::Mutex, StreamExt, TryStreamExt};
    use serde::Deserialize;
    use std::collections::VecDeque;
    use time::Duration;

    #[derive(Debug, Deserialize)]
    struct Model {
        foo: i32,
        bar: String,
    }

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

        let model = service_method().await.unwrap().into_body().unwrap();

        assert_eq!(1, model.foo);
        assert_eq!("baz", &model.bar);
    }

    #[derive(Debug, Default)]
    struct Counter {
        count: Mutex<usize>,
    }

    impl Counter {
        async fn count(&self) -> usize {
            let count = self.count.lock().await;
            *count
        }
    }

    #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    impl Policy for Counter {
        async fn send(
            &self,
            ctx: &Context,
            request: &mut Request,
            next: &[Arc<dyn Policy>],
        ) -> PolicyResult {
            let result = next[0].send(ctx, request, &next[1..]).await;

            // Increment the counter after the response.
            let mut count = self.count.lock().await;
            *count += 1;

            result
        }
    }

    #[tokio::test]
    async fn send_retries_in_pipeline() {
        #[derive(Debug)]
        struct Responder {
            responses: Mutex<VecDeque<BufResponse>>,
        }

        impl Default for Responder {
            fn default() -> Self {
                let mut headers = Headers::new();
                headers.insert("content-type", "application/json");
                headers.insert("transfer-encoding", "chunked");

                Self {
                    responses: Mutex::new(VecDeque::from_iter([
                        BufResponse::from_bytes(
                            StatusCode::TooManyRequests,
                            Headers::new(),
                            Vec::new(),
                        ),
                        BufResponse::new(
                            StatusCode::Ok,
                            headers.clone(),
                            futures::stream::iter([
                                Ok(Bytes::from_static(br#"{"foo":1,"#)),
                                // Simulate an I/O error from default reqwest::Client.
                                Err(Error::new(ErrorKind::Io, "connection reset")),
                            ])
                            .boxed(),
                        ),
                        BufResponse::new(
                            StatusCode::Ok,
                            headers,
                            futures::stream::iter([
                                Ok(Bytes::from_static(br#"{"foo":1,"#)),
                                Ok(Bytes::from_static(br#""bar":"baz"}"#)),
                            ])
                            .boxed(),
                        ),
                    ])),
                }
            }
        }

        #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
        #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
        impl Policy for Responder {
            async fn send(
                &self,
                _ctx: &Context,
                _request: &mut Request,
                _next: &[Arc<dyn Policy>],
            ) -> PolicyResult {
                let mut responses = self.responses.lock().await;
                let response = responses.pop_front().expect("expected BufResponse");
                Ok(response)
            }
        }

        let per_call_count = Arc::new(Counter::default());
        let per_try_count = Arc::new(Counter::default());

        // Simulated service method
        async fn service_method(
            per_call_count: Arc<Counter>,
            per_try_count: Arc<Counter>,
        ) -> crate::Result<Response<Model, JsonFormat>> {
            let options = ClientOptions {
                retry: RetryOptions::fixed(FixedRetryOptions {
                    delay: Duration::milliseconds(1),
                    ..Default::default()
                }),
                transport: Some(Transport::with_policy(Arc::new(Responder::default()))),
                ..Default::default()
            };
            let pipeline = Pipeline::new(options, vec![per_call_count], vec![per_try_count], None);
            let mut request = Request::new("http://localhost".parse().unwrap(), Method::Get);
            let raw_response = pipeline
                .send(&Context::default(), &mut request, None)
                .await?;
            Ok(raw_response.into())
        }

        let resp = service_method(per_call_count.clone(), per_try_count.clone())
            .await
            .expect("expected Response");
        assert_eq!(per_try_count.count().await, 3);
        assert_eq!(per_call_count.count().await, 1);

        let model = resp.into_body().expect("expected Model");
        assert_eq!(per_try_count.count().await, 3);
        assert_eq!(per_call_count.count().await, 1);

        assert_eq!(1, model.foo);
        assert_eq!("baz", &model.bar);
    }

    #[tokio::test]
    async fn stream_out_of_pipeline() {
        #[derive(Debug)]
        struct Responder {
            responses: Mutex<VecDeque<BufResponse>>,
        }

        impl Default for Responder {
            fn default() -> Self {
                let mut headers = Headers::new();
                headers.insert("content-type", "application/x-octet-stream");
                headers.insert("transfer-encoding", "chunked");

                Self {
                    responses: Mutex::new(VecDeque::from_iter([
                        BufResponse::from_bytes(
                            StatusCode::TooManyRequests,
                            Headers::new(),
                            Vec::new(),
                        ),
                        BufResponse::new(
                            StatusCode::Ok,
                            headers.clone(),
                            futures::stream::iter([
                                Ok(vec![0xde, 0xad].into()),
                                Ok(vec![0xbe, 0xef].into()),
                                // Simulate an I/O error from default reqwest::Client.
                                Err(Error::new(ErrorKind::Io, "connection reset")),
                            ])
                            .boxed(),
                        ),
                        BufResponse::from_bytes(
                            StatusCode::ImATeapot,
                            Headers::new(),
                            r#"unexpected"#,
                        ),
                    ])),
                }
            }
        }

        #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
        #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
        impl Policy for Responder {
            async fn send(
                &self,
                _ctx: &Context,
                _request: &mut Request,
                _next: &[Arc<dyn Policy>],
            ) -> PolicyResult {
                let mut responses = self.responses.lock().await;
                let response = responses.pop_front().expect("expected BufResponse");
                Ok(response)
            }
        }

        let per_call_count = Arc::new(Counter::default());
        let per_try_count = Arc::new(Counter::default());

        // Simulated service method
        async fn service_method(
            per_call_count: Arc<Counter>,
            per_try_count: Arc<Counter>,
        ) -> crate::Result<BufResponse> {
            let options = ClientOptions {
                retry: RetryOptions::fixed(FixedRetryOptions {
                    delay: Duration::milliseconds(1),
                    ..Default::default()
                }),
                transport: Some(Transport::with_policy(Arc::new(Responder::default()))),
                ..Default::default()
            };
            let pipeline = Pipeline::new(options, vec![per_call_count], vec![per_try_count], None);
            let mut request = Request::new("http://localhost".parse().unwrap(), Method::Get);
            pipeline
                .stream(&Context::default(), &mut request, None)
                .await
        }

        let resp = service_method(per_call_count.clone(), per_try_count.clone())
            .await
            .expect("expected BufResponse");
        assert_eq!(per_try_count.count().await, 2);
        assert_eq!(per_call_count.count().await, 1);

        let mut stream = resp.into_body().into_stream();
        assert_eq!(
            stream.try_next().await.expect("first chunk"),
            Some(vec![0xde, 0xad].into())
        );
        assert_eq!(
            stream.try_next().await.expect("second chunk"),
            Some(vec![0xbe, 0xef].into())
        );
        assert!(matches!(stream.try_next().await, Err(e) if *e.kind() == ErrorKind::Io));

        // Make sure we never went back through the pipeline policies.
        assert_eq!(per_try_count.count().await, 2);
        assert_eq!(per_call_count.count().await, 1);
    }
}
