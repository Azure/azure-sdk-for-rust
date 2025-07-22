// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{
    headers::{Header, HeaderValue, CONTENT_LENGTH},
    options::TransportOptions,
    policies::{Policy, PolicyResult},
    Context, Method, Request,
};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::debug;

/// The final pipeline policy that defines the HTTP transport.
#[derive(Debug, Clone)]
pub struct TransportPolicy {
    pub(crate) transport_options: TransportOptions,
}

impl TransportPolicy {
    pub fn new(transport_options: TransportOptions) -> Self {
        Self { transport_options }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for TransportPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        // there must be no more policies
        assert_eq!(0, next.len());

        if request.body().is_empty()
            && matches!(request.method(), Method::Patch | Method::Post | Method::Put)
        {
            request.add_mandatory_header(EMPTY_CONTENT_LENGTH);
        }

        debug!(?request, "sending request '{}'", request.url);
        let response = { self.transport_options.send(ctx, request) };

        response.await
    }
}

const EMPTY_CONTENT_LENGTH: &EmptyContentLength = &EmptyContentLength;

struct EmptyContentLength;

impl Header for EmptyContentLength {
    fn name(&self) -> crate::http::headers::HeaderName {
        CONTENT_LENGTH
    }
    fn value(&self) -> crate::http::headers::HeaderValue {
        HeaderValue::from("0")
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;
    use crate::http::{headers::Headers, RawResponse, StatusCode};

    #[derive(Debug)]
    struct MockTransport;

    #[async_trait]
    impl Policy for MockTransport {
        async fn send(
            &self,
            _ctx: &Context,
            _request: &mut Request,
            _next: &[Arc<dyn Policy>],
        ) -> PolicyResult {
            PolicyResult::Ok(RawResponse::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                Vec::new(),
            ))
        }
    }

    #[tokio::test]
    async fn test_content_length() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let transport =
            TransportPolicy::new(TransportOptions::new_custom_policy(Arc::new(MockTransport)));

        let mut request = Request::new("http://localhost".parse()?, Method::Get);
        transport.send(&Context::new(), &mut request, &[]).await?;
        assert!(!request.headers().iter().any(|h| CONTENT_LENGTH.eq(h.0)));

        request.headers = Headers::new();
        request.method = Method::Patch;
        transport.send(&Context::new(), &mut request, &[]).await?;
        assert_eq!(
            request
                .headers()
                .get_with(&CONTENT_LENGTH, |v| v.as_str().parse::<u16>())
                .unwrap(),
            0u16
        );

        request.headers = Headers::new();
        request.method = Method::Post;
        transport.send(&Context::new(), &mut request, &[]).await?;
        assert_eq!(
            request
                .headers()
                .get_with(&CONTENT_LENGTH, |v| v.as_str().parse::<u16>())
                .unwrap(),
            0u16
        );

        request.headers = Headers::new();
        request.method = Method::Put;
        transport.send(&Context::new(), &mut request, &[]).await?;
        assert_eq!(
            request
                .headers()
                .get_with(&CONTENT_LENGTH, |v| v.as_str().parse::<u16>())
                .unwrap(),
            0u16
        );

        // The HttpClient would add this normally.
        request.headers = Headers::new();
        request.body = "{}".into();
        transport.send(&Context::new(), &mut request, &[]).await?;
        request
            .headers()
            .get_with(&CONTENT_LENGTH, |v| v.as_str().parse::<u16>())
            .expect_err("expected no content-length header");

        Ok(())
    }
}
