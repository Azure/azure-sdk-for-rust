use azure_core::{Context, Policy, PolicyResult, Request};
use http::header::AUTHORIZATION;
use http::HeaderValue;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BearerTokenAuthorizationPolicy {
    bearer_token: String,
}

impl BearerTokenAuthorizationPolicy {
    #[allow(unused)]
    pub(crate) fn new(bearer_token: String) -> Self {
        Self { bearer_token }
    }
}

#[async_trait::async_trait]
impl Policy for BearerTokenAuthorizationPolicy {
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

        let auth_header_value = format!("Bearer {}", &self.bearer_token);

        request
            .headers_mut()
            .append(AUTHORIZATION, HeaderValue::from_str(&auth_header_value)?);

        // now next[0] is safe (will not panic) because we checked
        // at the beginning of the function.
        next[0].send(ctx, request, &next[1..]).await
    }
}
