use azure_core::{
    auth::TokenCredential,
    error::{ErrorKind, ResultExt},
    headers::*,
    prelude::*,
    Context, Policy, PolicyResult, Request,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthorizationPolicy {
    credentials: Arc<dyn TokenCredential>,
    scope: String,
}

impl std::fmt::Debug for AuthorizationPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthorizationPolicy")
            .field("credentials", &"...")
            .field("scope", &self.scope)
            .finish()
    }
}

impl AuthorizationPolicy {
    pub(crate) fn new(credentials: Arc<dyn TokenCredential>, scope: String) -> Self {
        Self { credentials, scope }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl Policy for AuthorizationPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        assert!(
            !next.is_empty(),
            "Authorization policies cannot be the last policy of a pipeline"
        );

        let bearer_token = self
            .credentials
            .get_token(&self.scope)
            .await
            .context(ErrorKind::Credential, "failed to get bearer token")?;

        request.insert_header(
            AUTHORIZATION,
            format!("Bearer {}", bearer_token.token.secret()),
        );

        next[0].send(ctx, request, &next[1..]).await
    }
}

#[derive(Debug, Clone)]
pub struct TimeoutPolicy {
    default_timeout: Option<Timeout>,
}

impl TimeoutPolicy {
    pub(crate) fn new(default_timeout: Option<Timeout>) -> Self {
        Self { default_timeout }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl Policy for TimeoutPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let timeout = ctx.get::<Timeout>().or(self.default_timeout.as_ref());
        timeout.append_to_url_query(request.url_mut());
        next[0].send(ctx, request, &next[1..]).await
    }
}
