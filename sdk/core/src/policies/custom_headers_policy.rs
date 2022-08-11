use crate::headers::Headers;
use crate::policies::{Policy, PolicyResult};
use crate::{Context, Request};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CustomHeaders(Headers);

impl From<Headers> for CustomHeaders {
    fn from(h: Headers) -> Self {
        Self(h)
    }
}

#[derive(Clone, Debug, Default)]
pub struct CustomHeadersPolicy {}

#[async_trait::async_trait]
impl Policy for CustomHeadersPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        if let Some(CustomHeaders(custom_headers)) = ctx.get::<CustomHeaders>() {
            custom_headers
                .iter()
                .for_each(|(header_name, header_value)| {
                    log::trace!(
                        "injecting custom context header {:?} with value {:?}",
                        header_name,
                        header_value
                    );
                    request.insert_header(header_name.clone(), header_value.clone());
                });
        }

        next[0].send(ctx, request, &next[1..]).await
    }
}
