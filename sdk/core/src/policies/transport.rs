#[cfg(not(target_arch = "wasm32"))]
use crate::policies::{Policy, PolicyResult};
#[allow(unused_imports)]
use crate::TransportOptions;
#[allow(unused_imports)]
use crate::{Context, HttpClient, Request, Response};
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TransportPolicy {
    #[allow(unused)]
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
impl Policy for TransportPolicy {
    async fn send(
        &self,
        _ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        // there must be no more policies
        assert_eq!(0, next.len());

        let response = { self.transport_options.http_client.execute_request2(request) };

        response.await
    }
}
