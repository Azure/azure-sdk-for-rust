// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    http::{
        headers::{self, Header as _},
        policies::{Policy, PolicyResult},
        request::options::ClientRequestId,
        Context, Request,
    },
    Uuid,
};
use std::sync::Arc;

/// Adds a `x-ms-client-request-id` (or custom) header to each request.
///
/// Because each client can set a custom header name, this policy must be added by every client
/// to [`ClientOptions::per_call_policies`](crate::http::options::ClientOptions::per_call_policies).
#[derive(Debug)]
pub struct RequestIdPolicy(headers::HeaderName);

impl RequestIdPolicy {
    /// Creates a new policy using the default `x-ms-client-request-id` header.
    pub const fn new() -> Self {
        RequestIdPolicy(headers::CLIENT_REQUEST_ID)
    }

    /// Creates a new policy using a custom header name.
    ///
    /// You can construct a new policy for a constant or static variable.
    pub const fn with_header_name(header: &'static str) -> Self {
        RequestIdPolicy(headers::HeaderName::from_static(header))
    }
}

impl Default for RequestIdPolicy {
    fn default() -> Self {
        RequestIdPolicy::new()
    }
}

impl From<headers::HeaderName> for RequestIdPolicy {
    fn from(header_name: headers::HeaderName) -> Self {
        Self(header_name)
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl Policy for RequestIdPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        if request.headers().get_optional_str(&self.0).is_none() {
            if let Some(request_id) = ctx.value::<ClientRequestId>() {
                request.insert_header(self.0.clone(), request_id.value());
            } else {
                let request_id: String = Uuid::new_v4().into();
                request.insert_header(self.0.clone(), request_id);
            }
        }

        next[0].send(ctx, request, &next[1..]).await
    }
}

#[cfg(test)]
mod tests {
    use super::{super::test::MockPolicy, *};
    use crate::http::{headers, Method, Request};
    use std::sync::Arc;

    #[tokio::test]
    async fn header_already_present() {
        // Arrange
        let mut request = Request::new("https://example.com".parse().unwrap(), Method::Get);
        let existing_request_id = "existing-request-id";
        request.insert_header(headers::CLIENT_REQUEST_ID, existing_request_id);

        let policy = RequestIdPolicy::default();
        let next_policy = Arc::new(MockPolicy::default());
        let ctx = Context::new();

        // Act
        policy
            .send(&ctx, &mut request, &[next_policy])
            .await
            .expect("Policy execution failed");

        // Assert
        let header_value = request
            .headers()
            .get_optional_str(&headers::CLIENT_REQUEST_ID)
            .expect("Header should be present");
        assert_eq!(
            header_value, existing_request_id,
            "Header value should not change"
        );
    }

    #[tokio::test]
    async fn header_not_present() {
        // Arrange
        let mut request = Request::new("https://example.com".parse().unwrap(), Method::Get);

        let policy = RequestIdPolicy::default();
        let next_policy = Arc::new(MockPolicy::default());
        let ctx = Context::new();

        // Act
        policy
            .send(&ctx, &mut request, &[next_policy])
            .await
            .expect("Policy execution failed");

        // Assert
        let header_value = request
            .headers()
            .get_optional_str(&headers::CLIENT_REQUEST_ID)
            .expect("Header should be present");
        assert!(!header_value.is_empty(), "Header value should be generated");
    }

    #[tokio::test]
    async fn custom_header_name_with_existing_value() {
        // Arrange
        let custom_header_name = headers::HeaderName::from_static("x-custom-request-id");
        let existing_request_id = "custom-existing-request-id";

        let mut request = Request::new("https://example.com".parse().unwrap(), Method::Get);
        request.insert_header(custom_header_name.clone(), existing_request_id);

        let policy = RequestIdPolicy::with_header_name("x-custom-request-id");
        let next_policy = Arc::new(MockPolicy::default());
        let ctx = Context::new();

        // Act
        policy
            .send(&ctx, &mut request, &[next_policy])
            .await
            .expect("Policy execution failed");

        // Assert
        let header_value = request
            .headers()
            .get_optional_str(&custom_header_name)
            .expect("Custom header should be present");
        assert_eq!(
            header_value, existing_request_id,
            "Custom header value should not change"
        );
    }

    #[tokio::test]
    async fn client_request_id_in_context() {
        // Arrange
        let client_request_id = "context-request-id";
        let mut request = Request::new("https://example.com".parse().unwrap(), Method::Get);

        let mut ctx = Context::new();
        ctx.insert(ClientRequestId::new(client_request_id.to_string()));

        let policy = RequestIdPolicy::default();
        let next_policy = Arc::new(MockPolicy::default());

        // Act
        policy
            .send(&ctx, &mut request, &[next_policy])
            .await
            .expect("Policy execution failed");

        // Assert
        let header_value = request
            .headers()
            .get_optional_str(&headers::CLIENT_REQUEST_ID)
            .expect("Header should be present");
        assert_eq!(
            header_value, client_request_id,
            "Header value should match the client request ID from the context"
        );
    }
}
