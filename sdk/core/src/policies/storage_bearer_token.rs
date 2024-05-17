use crate::{
    auth::TokenCredential,
    policies::{Policy, PolicyResult},
    ClientOptions, Context, Pipeline, Request,
};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct BearerTokenCredentialPolicy {
    credential: Arc<dyn TokenCredential>,
}

impl BearerTokenCredentialPolicy {
    pub fn new(credential: Arc<dyn TokenCredential>) -> Self {
        Self { credential }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for BearerTokenCredentialPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let access_token = self
            .credential
            .get_token(&["https://storage.azure.com/.default"])
            .await?;
        let token = access_token.token.secret();

        request.insert_header("authorization", format!("Bearer {token}"));
        debug!("the following request will be passed to the transport policy: {request:#?}");

        next[0].send(ctx, request, &next[1..]).await
    }
}
