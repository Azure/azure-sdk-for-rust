// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{
    policies::{Policy, PolicyResult},
    sanitizer::{Sanitizer, DEFAULT_ALLOWED_HEADER_NAMES, DEFAULT_ALLOWED_QUERY_PARAMETERS},
    Context, LoggingOptions, Request,
};
use std::sync::Arc;
use std::{borrow::Cow, collections::HashSet};
use tracing::{debug, info};

/// [`Policy`] to log a request and response.
#[derive(Clone, Debug, Default)]
pub(crate) struct LoggingPolicy {
    allowed_headers: HashSet<Cow<'static, str>>,
    allowed_query_params: HashSet<Cow<'static, str>>,
}

impl LoggingPolicy {
    /// Create a new `LoggingPolicy`.
    pub fn new(options: LoggingOptions) -> Self {
        // Create owned HashSet from the defaults and extend with any additional entries
        let mut allowed_headers: HashSet<Cow<'static, str>> =
            (*DEFAULT_ALLOWED_HEADER_NAMES).clone();
        allowed_headers.extend(options.additional_allowed_header_names);

        let mut allowed_query_params: HashSet<Cow<'static, str>> =
            (*DEFAULT_ALLOWED_QUERY_PARAMETERS).clone();
        allowed_query_params.extend(options.additional_allowed_query_params);

        Self {
            allowed_headers,
            allowed_query_params,
        }
    }
}

#[async_trait::async_trait]
impl Policy for LoggingPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        debug!(
            "==> Request: url: {}, method: {}, headers: {{ {} }}",
            request.url.sanitize(&self.allowed_query_params),
            request.method(),
            request.headers().sanitize(&self.allowed_headers)
        );
        let response = next[0].send(ctx, request, &next[1..]).await;

        if let Ok(response) = &response {
            debug!(
                "<== Response: {{ url: {}, status: {}, headers: {{ {} }} }}",
                request.url.sanitize(&self.allowed_query_params),
                response.status(),
                response.headers().sanitize(&self.allowed_headers)
            )
        } else {
            info!(
                "<== Failed response: {{ url: {}, error: {} }}",
                request.url.sanitize(&self.allowed_query_params),
                response.as_ref().err().unwrap()
            )
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{headers::Headers, AsyncRawResponse, Method, Request, StatusCode};
    use futures::StreamExt;
    use url::Url;

    fn create_test_request() -> Request {
        let url = Url::parse("https://example.com/api?sensitive=secret&allowed=value").unwrap();
        let mut request = Request::new(url, Method::Get);
        request.insert_header("authorization", "Bearer secret-token");
        request.insert_header("content-type", "application/json");
        request
    }

    #[tokio::test]
    async fn test_logging_policy_configuration() {
        // Test with additional allowed values
        let options = LoggingOptions {
            additional_allowed_header_names: vec!["custom-header".into()],
            additional_allowed_query_params: vec!["custom-param".into()],
        };
        let policy = LoggingPolicy::new(options);

        // Verify default headers are included
        assert!(policy.allowed_headers.contains("content-type"));
        // Verify additional header is included
        assert!(policy.allowed_headers.contains("custom-header"));

        // Verify default query params are included
        assert!(policy.allowed_query_params.contains("api-version"));
        // Verify additional query param is included
        assert!(policy.allowed_query_params.contains("custom-param"));
    }

    #[tokio::test]
    async fn test_logging_policy_request_sanitization() {
        let options = LoggingOptions {
            additional_allowed_header_names: vec!["content-type".into()],
            additional_allowed_query_params: vec!["allowed".into()],
        };
        let policy = LoggingPolicy::new(options);
        let ctx = Context::default();
        let mut request = create_test_request();

        // Create a mock policy that returns a simple response
        let next_policy = Arc::new(MockPolicy);

        let result = policy.send(&ctx, &mut request, &[next_policy]).await;
        assert!(result.is_ok());

        // We can't directly test the trace! macro output, but we can verify
        // that the policy processed the request and response correctly
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::Ok);
    }

    #[tokio::test]
    async fn test_logging_policy_empty_options() {
        let policy = LoggingPolicy::default();
        let ctx = Context::default();
        let mut request = create_test_request();
        let next_policy = Arc::new(MockPolicy);

        let result = policy.send(&ctx, &mut request, &[next_policy]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_logging_policy_custom_headers() {
        let options = LoggingOptions {
            additional_allowed_header_names: vec!["custom-header".into()],
            additional_allowed_query_params: vec![],
        };
        let policy = LoggingPolicy::new(options);
        let ctx = Context::default();
        let mut request =
            Request::new(Url::parse("https://example.com/api").unwrap(), Method::Post);
        request.insert_header("custom-header", "test-value");

        let next_policy = Arc::new(MockPolicy);
        let result = policy.send(&ctx, &mut request, &[next_policy]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_logging_policy_with_error_response() {
        let policy = LoggingPolicy::default();
        let ctx = Context::default();
        let mut request = create_test_request();
        let next_policy = Arc::new(ErrorPolicy);

        let result = policy.send(&ctx, &mut request, &[next_policy]).await;
        assert!(result.is_err());
    }

    /// A mock policy that always returns an error
    #[derive(Debug, Clone)]
    struct ErrorPolicy;

    #[async_trait::async_trait]
    impl Policy for ErrorPolicy {
        async fn send(
            &self,
            _ctx: &Context,
            _request: &mut Request,
            _next: &[Arc<dyn Policy>],
        ) -> PolicyResult {
            Err(crate::Error::with_message(
                crate::error::ErrorKind::Other,
                "Test error",
            ))
        }
    }
    /// A mock policy for testing that returns a simple OK response
    #[derive(Debug, Clone)]
    struct MockPolicy;

    #[async_trait::async_trait]
    impl Policy for MockPolicy {
        async fn send(
            &self,
            _ctx: &Context,
            _request: &mut Request,
            _next: &[Arc<dyn Policy>],
        ) -> PolicyResult {
            Ok(AsyncRawResponse::new(
                StatusCode::Ok,
                Headers::new(),
                futures::stream::empty::<Result<crate::Bytes, crate::Error>>().boxed(),
            ))
        }
    }
}
