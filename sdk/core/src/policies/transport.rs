use crate::policies::{Policy, PolicyResult};
use crate::TransportOptions;
use crate::{Context, Request};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TransportPolicy {
    pub(crate) transport_options: TransportOptions,
}

impl TransportPolicy {
    pub fn new(transport_options: TransportOptions) -> Self {
        Self { transport_options }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for TransportPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        // there must be no more policies
        assert_eq!(0, next.len());

        log::debug!("the following request will be passed to the transport policy: {request:#?}");
        let response = { self.transport_options.send(ctx, request) };

        response.await
    }
}
