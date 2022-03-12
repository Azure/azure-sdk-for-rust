use azure_core::{auth::TokenCredential, Context, Policy, PolicyResult, Request};
use http::header::AUTHORIZATION;
use http::HeaderValue;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthorizationPolicy {
    credential: Arc<dyn TokenCredential>,
    resource: String,
}

impl std::fmt::Debug for AuthorizationPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AuthorizationPolicy")
            .field("credential", &"TokenCredential")
            .field("resource", &self.resource)
            .finish()
    }
}

impl AuthorizationPolicy {
    pub(crate) fn new<T>(credential: Arc<dyn TokenCredential>, resource: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            credential,
            resource: resource.into(),
        }
    }
}

#[async_trait::async_trait]
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

        let token = self.credential.get_token(&self.resource).await?;
        let auth_header_value = format!("Bearer {}", token.token.secret().clone());

        request
            .headers_mut()
            .append(AUTHORIZATION, HeaderValue::from_str(&auth_header_value)?);

        next[0].send(ctx, request, &next[1..]).await
    }
}
