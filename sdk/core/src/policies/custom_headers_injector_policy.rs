use crate::policies::{Policy, PolicyResult};
use crate::{PipelineContext, Request, Response};
use http::header::HeaderMap;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CustomHeaders(pub HeaderMap);

impl From<HeaderMap> for CustomHeaders {
    fn from(header_map: HeaderMap) -> Self {
        Self(header_map)
    }
}

#[derive(Clone, Debug, Default)]
pub struct CustomHeadersInjectorPolicy {}

#[async_trait::async_trait]
impl<C> Policy<C> for CustomHeadersInjectorPolicy
where
    C: Send + Sync,
{
    async fn send(
        &self,
        ctx: &mut PipelineContext<C>,
        request: &mut Request,
        next: &[Arc<dyn Policy<C>>],
    ) -> PolicyResult<Response> {
        if let Some(CustomHeaders(custom_headers)) = ctx.get_inner_context().get::<CustomHeaders>()
        {
            custom_headers
                .iter()
                .for_each(|(header_name, header_value)| {
                    log::trace!(
                        "injecting custom context header {:?} with value {:?}",
                        header_name,
                        header_value
                    );
                    request
                        .headers_mut()
                        .insert(header_name, header_value.to_owned());
                });
        }

        next[0].send(ctx, request, &next[1..]).await
    }
}
