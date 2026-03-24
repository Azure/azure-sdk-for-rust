// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB default headers policy.
//!
//! This policy sets the User-Agent header with a Cosmos-specific value,
//! overriding the default set by azure_core's `UserAgentPolicy`.

use azure_core::http::{
    headers::{HeaderValue, USER_AGENT},
    policies::{Policy, PolicyResult},
    Context, Request,
};
use std::sync::Arc;

/// Policy that overrides the User-Agent header with a Cosmos SDK value.
///
/// The azure_core pipeline includes a `UserAgentPolicy` that sets the User-Agent
/// header as a prefix. Cosmos DB uses a different convention where the user agent
/// suffix is appended after the SDK identifier. This policy runs after
/// `UserAgentPolicy` and replaces the header entirely.
#[derive(Clone, Debug)]
pub(crate) struct CosmosHeadersPolicy {
    user_agent: HeaderValue,
}

impl CosmosHeadersPolicy {
    /// Creates a new headers policy with an optional user agent suffix.
    ///
    /// The resulting User-Agent header will be:
    /// - `azsdk-rust-cosmos/{version}` if no suffix is provided
    /// - `azsdk-rust-cosmos/{version} {suffix}` if a suffix is provided
    pub(crate) fn new(crate_version: &str, suffix: Option<&str>) -> Self {
        let user_agent = match suffix {
            Some(s) if !s.is_empty() => format!("azsdk-rust-cosmos/{crate_version} {s}"),
            _ => format!("azsdk-rust-cosmos/{crate_version}"),
        };
        Self {
            user_agent: HeaderValue::from(user_agent),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl Policy for CosmosHeadersPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        // Override User-Agent with Cosmos SDK value
        request.insert_header(USER_AGENT, self.user_agent.clone());

        next[0].send(ctx, request, &next[1..]).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::{Method, Url};

    /// Mock transport policy that captures the request for inspection.
    #[derive(Debug)]
    struct MockTransport;

    #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    impl Policy for MockTransport {
        async fn send(
            &self,
            _ctx: &Context,
            _request: &mut Request,
            _next: &[Arc<dyn Policy>],
        ) -> PolicyResult {
            Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "mock transport",
            ))
        }
    }

    #[tokio::test]
    async fn sets_user_agent_without_suffix() {
        let policy = CosmosHeadersPolicy::new("0.31.0", None);
        let transport: Arc<dyn Policy> = Arc::new(MockTransport);
        let policies: Vec<Arc<dyn Policy>> = vec![transport];

        let url = Url::parse("https://test.documents.azure.com/").unwrap();
        let mut request = Request::new(url, Method::Get);
        let ctx = Context::default();

        let _ = policy.send(&ctx, &mut request, &policies).await;

        assert_eq!(
            request.headers().get_optional_str(&USER_AGENT),
            Some("azsdk-rust-cosmos/0.31.0")
        );
    }

    #[tokio::test]
    async fn sets_user_agent_with_suffix() {
        let policy = CosmosHeadersPolicy::new("0.31.0", Some("my-app"));
        let transport: Arc<dyn Policy> = Arc::new(MockTransport);
        let policies: Vec<Arc<dyn Policy>> = vec![transport];

        let url = Url::parse("https://test.documents.azure.com/").unwrap();
        let mut request = Request::new(url, Method::Get);
        let ctx = Context::default();

        let _ = policy.send(&ctx, &mut request, &policies).await;

        assert_eq!(
            request.headers().get_optional_str(&USER_AGENT),
            Some("azsdk-rust-cosmos/0.31.0 my-app")
        );
    }

    #[tokio::test]
    async fn overrides_existing_user_agent() {
        let policy = CosmosHeadersPolicy::new("0.31.0", None);
        let transport: Arc<dyn Policy> = Arc::new(MockTransport);
        let policies: Vec<Arc<dyn Policy>> = vec![transport];

        let url = Url::parse("https://test.documents.azure.com/").unwrap();
        let mut request = Request::new(url, Method::Get);

        // Simulate azure_core's UserAgentPolicy having set a value
        request
            .headers_mut()
            .insert(USER_AGENT, HeaderValue::from_static("azure-core-default"));

        let ctx = Context::default();
        let _ = policy.send(&ctx, &mut request, &policies).await;

        assert_eq!(
            request.headers().get_optional_str(&USER_AGENT),
            Some("azsdk-rust-cosmos/0.31.0")
        );
    }
}
