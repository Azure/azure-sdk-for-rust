use crate::policies::{Policy, PolicyResult, Request, Response};
use crate::PipelineContext;
use std::sync::Arc;

/// Retry policy that does not retry.
///
/// Use this policy as a stub to disable retry policies altogether.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct NoRetryPolicy {
    _priv: std::marker::PhantomData<u32>,
}

#[async_trait::async_trait]
impl<R> Policy<R> for NoRetryPolicy
where
    R: Send + Sync,
{
    async fn send(
        &self,
        ctx: &mut PipelineContext<R>,
        request: &mut Request,
        next: &[Arc<dyn Policy<R>>],
    ) -> PolicyResult<Response> {
        // just call the following policies and bubble up the error
        next[0].send(ctx, request, &next[1..]).await
    }
}
