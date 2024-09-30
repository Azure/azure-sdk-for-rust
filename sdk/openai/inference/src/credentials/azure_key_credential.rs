// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
use async_trait::async_trait;
use std::sync::Arc;

use azure_core::{
    credentials::Secret,
    headers::{HeaderName, HeaderValue},
    Context, Header, Policy, PolicyResult, Request,
};

/// A key credential for the [AzureOpenAIClient](crate::clients::AzureOpenAIClient).
///
/// # Example
/// ```no_run
/// use azure_openai_inference::clients::{AzureOpenAIClient, AzureOpenAIClientMethods};
///
/// let secret = std::env::var("OPENAI_KEY").expect("Set OPENAI_KEY env variable");
/// let azure_open_ai_client = AzureOpenAIClient::with_key_credential(
///    "https://my.endpoint/",
///     secret,
///     None,
/// ).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct AzureKeyCredential(Secret);

impl AzureKeyCredential {
    /// Create a new [`AzureKeyCredential`].
    pub fn new(api_key: impl Into<String>) -> Self {
        Self(Secret::new(api_key.into()))
    }
}

impl Header for AzureKeyCredential {
    fn name(&self) -> HeaderName {
        HeaderName::from_static("api-key")
    }

    fn value(&self) -> HeaderValue {
        HeaderValue::from_cow(format!("{}", self.0.secret()))
    }
}

// code lifted from BearerTokenCredentialPolicy
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for AzureKeyCredential {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        request.insert_header(Header::name(self), Header::value(self));
        next[0].send(ctx, request, &next[1..]).await
    }
}

impl Into<Arc<dyn Policy>> for AzureKeyCredential {
    fn into(self) -> Arc<dyn Policy> {
        Arc::new(self)
    }
}
