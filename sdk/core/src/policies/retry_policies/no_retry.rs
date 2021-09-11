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
impl<C> Policy<C> for NoRetryPolicy
where
    C: Send + Sync,
{
    async fn send<'a, 'b, 'c>(
        &'a self,
        ctx: &'b mut PipelineContext<'a, C>,
        request: &'c mut Request,
        next: &'a [Arc<dyn Policy<C>>],
    ) -> PolicyResult<Response> {
        // just call the following policies and bubble up the error
        next[0].send(ctx, request, &next[1..]).await
    }
}
