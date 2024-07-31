use crate::{
    auth::TokenCredential,
    policies::{Policy, PolicyResult},
    Context, Request,
};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct BearerTokenCredentialPolicy<'a> {
    credential: Arc<dyn TokenCredential>,
    scopes: &'a [&'a str],
}

impl<'a> BearerTokenCredentialPolicy<'a> {
    pub fn new(credential: Arc<dyn TokenCredential>, scopes: &'a [&'a str]) -> Self {
        Self { credential, scopes }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for BearerTokenCredentialPolicy<'_> {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let access_token = self.credential.get_token(self.scopes).await?;
        let token = access_token.token.secret();

        request.insert_header("authorization", format!("Bearer {token}"));
        debug!("the following request will be passed to the transport policy: {request:#?}");

        next[0].send(ctx, request, &next[1..]).await
    }
}
