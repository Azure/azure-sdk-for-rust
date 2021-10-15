#[cfg(not(target_arch = "wasm32"))]
use crate::policies::{Policy, PolicyResult};
#[allow(unused_imports)]
use crate::TransportOptions;
#[allow(unused_imports)]
use crate::{Context, HttpClient, PipelineContext, Request, Response};
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TransportPolicy {
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) transport_options: TransportOptions,
}

impl TransportPolicy {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(transport_options: TransportOptions) -> Self {
        Self { transport_options }
    }
}

#[async_trait::async_trait]
#[cfg(not(target_arch = "wasm32"))]
impl<C> Policy<C> for TransportPolicy
where
    C: Send + Sync,
{
    async fn send(
        &self,
        ctx: &mut PipelineContext<C>,
        request: &mut Request,
        next: &[Arc<dyn Policy<C>>],
    ) -> PolicyResult<Response> {
        // there must be no more policies
        assert_eq!(0, next.len());

        let response = {
            self.transport_options
                .http_client
                .execute_request2(ctx.get_inner_context(), request)
        };

        Ok(response.await?)
    }
}
