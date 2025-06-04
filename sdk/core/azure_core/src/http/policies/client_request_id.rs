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
/// Clients can set a custom name by adding [`ClientRequestIdPolicy::with_header_name()`]
/// to [`ClientOptions::per_call_policies`](crate::http::options::ClientOptions::per_call_policies).
/// The default policy will not be added if a custom one has already been added.
#[derive(Debug)]
pub struct ClientRequestIdPolicy(headers::HeaderName);

impl ClientRequestIdPolicy {
    /// Creates a new policy using the default `x-ms-client-request-id` header.
    pub const fn new() -> Self {
        ClientRequestIdPolicy(headers::CLIENT_REQUEST_ID)
    }

    /// Creates a new policy using a custom header name.
    ///
    /// You can construct a new policy for a constant or static variable.
    pub const fn with_header_name(header: &'static str) -> Self {
        ClientRequestIdPolicy(headers::HeaderName::from_static(header))
    }
}

impl Default for ClientRequestIdPolicy {
    fn default() -> Self {
        ClientRequestIdPolicy::new()
    }
}

impl From<headers::HeaderName> for ClientRequestIdPolicy {
    fn from(header_name: headers::HeaderName) -> Self {
        Self(header_name)
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl Policy for ClientRequestIdPolicy {
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
    use super::*;
    use crate::{
        http::{headers, Method, Request, StatusCode},
        Bytes,
    };
    use azure_core_test::http::MockHttpClient;
    use futures::FutureExt;
    use std::sync::Arc;
    use typespec_client_core::http::{policies::TransportPolicy, RawResponse, TransportOptions};

    #[tokio::test]
    async fn header_already_present() {
        // Arrange
        let mut request = Request::new("https://example.com".parse().unwrap(), Method::Get);
        const EXISTING_REQUEST_ID: &str = "existing-request-id";
        request.insert_header(headers::CLIENT_REQUEST_ID, EXISTING_REQUEST_ID);

        let policy = ClientRequestIdPolicy::default();
        let transport = Arc::new(MockHttpClient::new(|req| {
            async move {
                // Assert
                let header_value = req
                    .headers()
                    .get_optional_str(&headers::CLIENT_REQUEST_ID)
                    .expect("Header should be present");
                assert_eq!(
                    header_value, EXISTING_REQUEST_ID,
                    "Header value should not change"
                );

                Ok(RawResponse::from_bytes(
                    StatusCode::Ok,
                    headers::Headers::new(),
                    Bytes::new(),
                ))
            }
            .boxed()
        }));
        let transport = Arc::new(TransportPolicy::new(TransportOptions::new(transport)));
        let ctx = Context::new();

        // Act
        policy
            .send(&ctx, &mut request, &[transport])
            .await
            .expect("Policy execution failed");
    }

    #[tokio::test]
    async fn header_not_present() {
        // Arrange
        let mut request = Request::new("https://example.com".parse().unwrap(), Method::Get);

        let policy = ClientRequestIdPolicy::default();
        let transport = Arc::new(MockHttpClient::new(|req| {
            async move {
                // Assert
                let header_value = req
                    .headers()
                    .get_optional_str(&headers::CLIENT_REQUEST_ID)
                    .expect("Header should be present");
                assert!(!header_value.is_empty(), "Header value should be generated");

                Ok(RawResponse::from_bytes(
                    StatusCode::Ok,
                    headers::Headers::new(),
                    Bytes::new(),
                ))
            }
            .boxed()
        }));
        let transport = Arc::new(TransportPolicy::new(TransportOptions::new(transport)));
        let ctx = Context::new();

        // Act
        policy
            .send(&ctx, &mut request, &[transport])
            .await
            .expect("Policy execution failed");
    }

    #[tokio::test]
    async fn custom_header_name_with_existing_value() {
        // Arrange
        let custom_header_name = headers::HeaderName::from_static("x-custom-request-id");
        let existing_request_id = "custom-existing-request-id";

        let mut request = Request::new("https://example.com".parse().unwrap(), Method::Get);
        request.insert_header(custom_header_name.clone(), existing_request_id);

        let policy = ClientRequestIdPolicy::with_header_name("x-custom-request-id");
        let transport = Arc::new(MockHttpClient::new(move |req| {
            let custom_header_name = custom_header_name.clone();
            async move {
                // Assert
                let header_value = req
                    .headers()
                    .get_optional_str(&custom_header_name)
                    .expect("Custom header should be present");
                assert_eq!(
                    header_value, existing_request_id,
                    "Custom header value should not change"
                );

                Ok(RawResponse::from_bytes(
                    StatusCode::Ok,
                    headers::Headers::new(),
                    Bytes::new(),
                ))
            }
            .boxed()
        }));
        let transport = Arc::new(TransportPolicy::new(TransportOptions::new(transport)));
        let ctx = Context::new();

        // Act
        policy
            .send(&ctx, &mut request, &[transport])
            .await
            .expect("Policy execution failed");
    }

    #[tokio::test]
    async fn client_request_id_in_context() {
        // Arrange
        const CLIENT_REQUEST_ID: &str = "context-request-id";
        let mut request = Request::new("https://example.com".parse().unwrap(), Method::Get);

        let mut ctx = Context::new();
        ctx.insert(ClientRequestId::new(CLIENT_REQUEST_ID.to_string()));

        let policy = ClientRequestIdPolicy::default();
        let transport = Arc::new(MockHttpClient::new(|req| {
            async move {
                // Assert
                let header_value = req
                    .headers()
                    .get_optional_str(&headers::CLIENT_REQUEST_ID)
                    .expect("Header should be present");
                assert_eq!(
                    header_value, CLIENT_REQUEST_ID,
                    "Header value should match the client request ID from the context"
                );

                Ok(RawResponse::from_bytes(
                    StatusCode::Ok,
                    headers::Headers::new(),
                    Bytes::new(),
                ))
            }
            .boxed()
        }));
        let transport = Arc::new(TransportPolicy::new(TransportOptions::new(transport)));

        // Act
        policy
            .send(&ctx, &mut request, &[transport])
            .await
            .expect("Policy execution failed");
    }
}
