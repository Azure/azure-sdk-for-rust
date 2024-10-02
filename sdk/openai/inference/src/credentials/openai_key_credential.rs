// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
use async_trait::async_trait;
use std::sync::Arc;

use azure_core::{
    credentials::Secret,
    headers::{HeaderName, HeaderValue, AUTHORIZATION},
    Context, Header, Policy, PolicyResult, Request,
};

/// A key credential for the [OpenAIClient](crate::clients::OpenAIClient).
///
/// # Example
/// ```no_run
/// use azure_openai_inference::clients::{OpenAIClient, OpenAIClientMethods};
///
/// let secret = std::env::var("OPENAI_KEY").expect("Set OPENAI_KEY env variable");
/// let open_ai_client = OpenAIClient::with_key_credential(
///     secret,
///     None,
/// ).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct OpenAIKeyCredential(Secret);

impl OpenAIKeyCredential {
    pub fn new(access_token: impl Into<String>) -> Self {
        Self(Secret::new(access_token.into()))
    }
}

impl Header for OpenAIKeyCredential {
    fn name(&self) -> HeaderName {
        AUTHORIZATION
    }

    fn value(&self) -> HeaderValue {
        HeaderValue::from_cow(format!("Bearer {}", &self.0.secret()))
    }
}

// code lifted from BearerTokenCredentialPolicy
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for OpenAIKeyCredential {
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

impl From<OpenAIKeyCredential> for Arc<dyn Policy> {
    fn from(credential: OpenAIKeyCredential) -> Arc<dyn Policy> {
        Arc::new(credential)
    }
}
