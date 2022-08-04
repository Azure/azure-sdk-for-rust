use crate::request_options::Timeout;
use crate::{AppendToUrlQuery, Context, Policy, PolicyResult, Request};
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct TimeoutPolicy {
    default_timeout: Option<Timeout>,
}

impl TimeoutPolicy {
    pub fn new(default_timeout: Option<Timeout>) -> Self {
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
