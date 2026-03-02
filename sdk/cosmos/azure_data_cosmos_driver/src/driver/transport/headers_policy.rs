// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB default headers policy.
//!
//! This policy sets the default HTTP headers required by Azure Cosmos DB on every
//! request. It runs early in the policy chain to ensure headers are present
//! before authorization and other policies process the request.

use azure_core::http::{
    headers::{HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT},
    policies::{Policy, PolicyResult},
    Context, Request,
};
use std::sync::Arc;

/// Default Content-Type for Cosmos DB requests.
const APPLICATION_JSON: HeaderValue = HeaderValue::from_static("application/json");

/// x-ms-version header name.
const VERSION: HeaderName = HeaderName::from_static("x-ms-version");
/// Cosmos DB REST API version.
///
/// This must match the version supported by the service. The value `2020-07-15`
/// is the same as used by the Java SDK for compatibility.
pub(crate) const COSMOS_API_VERSION: &str = "2020-07-15";

/// x-ms-cosmos-sdk-supportedcapabilities header name.
///
/// This header indicates which SDK capabilities are supported. The value is a
/// bitmask where bit 0 (value 1) would indicate partition merge support.
/// Currently set to "0" (no special capabilities enabled).
const SDK_SUPPORTED_CAPABILITIES: HeaderName =
    HeaderName::from_static("x-ms-cosmos-sdk-supportedcapabilities");

/// Supported capabilities bitmask value.
///
/// Currently no special capabilities are supported. In the future:
/// - Bit 0 (1): Partition merge support
const SUPPORTED_CAPABILITIES_VALUE: &str = "0";

/// Cache-Control header name.
const CACHE_CONTROL: HeaderName = HeaderName::from_static("cache-control");

/// Default Cache-Control value.
const NO_CACHE: HeaderValue = HeaderValue::from_static("no-cache");

/// Policy that sets default Cosmos DB headers on every request.
///
/// This policy adds the following headers if not already present:
/// - `x-ms-version`: API version (2020-07-15)
/// - `x-ms-cosmos-sdk-supportedcapabilities`: SDK capability flags (0 - no special capabilities)
/// - `Content-Type`: application/json
/// - `Accept`: application/json
/// - `Cache-Control`: no-cache
/// - `User-Agent`: Custom Cosmos SDK user agent (overrides azure_core default)
///
/// The User-Agent header is always overridden with the Cosmos SDK value to
/// ensure proper telemetry and compatibility tracking.
#[derive(Clone, Debug)]
pub(crate) struct CosmosHeadersPolicy {
    /// The user agent string to use for requests.
    user_agent: HeaderValue,
}

impl CosmosHeadersPolicy {
    /// Creates a new headers policy with the specified user agent.
    pub(crate) fn new(user_agent: impl Into<String>) -> Self {
        Self {
            user_agent: HeaderValue::from(user_agent.into()),
        }
    }
}

#[async_trait::async_trait]
impl Policy for CosmosHeadersPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let headers = request.headers_mut();

        // Set API version (always set, required by Cosmos)
        headers.insert(VERSION, HeaderValue::from_static(COSMOS_API_VERSION));

        // Set SDK supported capabilities (always set)
        headers.insert(
            SDK_SUPPORTED_CAPABILITIES,
            HeaderValue::from_static(SUPPORTED_CAPABILITIES_VALUE),
        );

        // Set Content-Type if not already present
        if headers.get_optional_str(&CONTENT_TYPE).is_none() {
            headers.insert(CONTENT_TYPE, APPLICATION_JSON.clone());
        }

        // Set Accept header (always set)
        headers.insert(ACCEPT, APPLICATION_JSON.clone());

        // Set Cache-Control (always set)
        headers.insert(CACHE_CONTROL, NO_CACHE.clone());

        // Override User-Agent with Cosmos SDK value
        // This is intentionally not conditional - we always want our user agent
        headers.insert(USER_AGENT, self.user_agent.clone());

        next[0].send(ctx, request, &next[1..]).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::{policies::Policy, Method, Url};
    use std::sync::Arc;

    /// Mock transport policy that captures the request for inspection.
    #[derive(Debug)]
    struct MockTransport;

    #[async_trait::async_trait]
    impl Policy for MockTransport {
        async fn send(
            &self,
            _ctx: &Context,
            _request: &mut Request,
            _next: &[Arc<dyn Policy>],
        ) -> PolicyResult {
            // Return a mock response - headers have been set by the time we get here
            Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "mock transport",
            ))
        }
    }

    #[tokio::test]
    async fn sets_required_headers() {
        let policy = CosmosHeadersPolicy::new("azsdk-rust-cosmos-driver/0.1.0");
        let transport: Arc<dyn Policy> = Arc::new(MockTransport);
        let policies: Vec<Arc<dyn Policy>> = vec![transport];

        let url = Url::parse("https://test.documents.azure.com/").unwrap();
        let mut request = Request::new(url, Method::Get);
        let ctx = Context::default();

        // Send through policy chain - will error at mock transport
        let _ = policy.send(&ctx, &mut request, &policies).await;

        let headers = request.headers();

        // Verify all required headers are set
        assert_eq!(headers.get_optional_str(&VERSION), Some(COSMOS_API_VERSION));
        assert_eq!(
            headers.get_optional_str(&SDK_SUPPORTED_CAPABILITIES),
            Some(SUPPORTED_CAPABILITIES_VALUE)
        );
        assert_eq!(
            headers.get_optional_str(&CONTENT_TYPE),
            Some("application/json")
        );
        assert_eq!(headers.get_optional_str(&ACCEPT), Some("application/json"));
        assert_eq!(headers.get_optional_str(&CACHE_CONTROL), Some("no-cache"));
        assert_eq!(
            headers.get_optional_str(&USER_AGENT),
            Some("azsdk-rust-cosmos-driver/0.1.0")
        );
    }

    #[tokio::test]
    async fn does_not_override_existing_content_type() {
        let policy = CosmosHeadersPolicy::new("test-user-agent");
        let transport: Arc<dyn Policy> = Arc::new(MockTransport);
        let policies: Vec<Arc<dyn Policy>> = vec![transport];

        let url = Url::parse("https://test.documents.azure.com/").unwrap();
        let mut request = Request::new(url, Method::Post);

        // Pre-set a different content type
        request
            .headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));

        let ctx = Context::default();
        let _ = policy.send(&ctx, &mut request, &policies).await;

        // Content-Type should be preserved
        assert_eq!(
            request.headers().get_optional_str(&CONTENT_TYPE),
            Some("text/plain")
        );
    }

    #[tokio::test]
    async fn always_overrides_user_agent() {
        let policy = CosmosHeadersPolicy::new("cosmos-sdk-user-agent");
        let transport: Arc<dyn Policy> = Arc::new(MockTransport);
        let policies: Vec<Arc<dyn Policy>> = vec![transport];

        let url = Url::parse("https://test.documents.azure.com/").unwrap();
        let mut request = Request::new(url, Method::Get);

        // Pre-set a different user agent (simulating azure_core default)
        request
            .headers_mut()
            .insert(USER_AGENT, HeaderValue::from_static("azure-core-default"));

        let ctx = Context::default();
        let _ = policy.send(&ctx, &mut request, &policies).await;

        // User-Agent should be overridden with our value
        assert_eq!(
            request.headers().get_optional_str(&USER_AGENT),
            Some("cosmos-sdk-user-agent")
        );
    }
}
