use azure_core::{PipelineContext, Policy, PolicyResult, Request, Response};
use http::header::AUTHORIZATION;
use http::HeaderValue;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationPolicy {
    bearer_token: String,
}

impl AuthorizationPolicy {
    pub(crate) fn new(bearer_token: String) -> Self {
        Self { bearer_token }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct DataLakeContext {}

#[async_trait::async_trait]
impl Policy<DataLakeContext> for AuthorizationPolicy {
    async fn send(
        &self,
        ctx: &mut PipelineContext<DataLakeContext>,
        request: &mut Request,
        next: &[Arc<dyn Policy<DataLakeContext>>],
    ) -> PolicyResult<Response> {
        let auth_header_value = format!("Bearer {}", &self.bearer_token);

        request
            .headers_mut()
            .append(AUTHORIZATION, HeaderValue::from_str(&auth_header_value)?);

        // now next[0] is safe (will not panic) because we checked
        // at the beginning of the function.
        next[0].send(ctx, request, &next[1..]).await
    }
}
