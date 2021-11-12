#[cfg(not(target_arch = "wasm32"))]
use crate::policies::{Policy, PolicyResult};
#[allow(unused_imports)]
use crate::TransportOptions;
#[allow(unused_imports)]
use crate::{HttpClient, OverridableContext, Request, Response};
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TransportPolicy {
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
    async fn send<'a>(
        &'a self,
        _ctx: &'a OverridableContext<'a>,
        request: &'a mut Request,
        next: &'a [Arc<dyn Policy>],
    ) -> PolicyResult<Response> {
        // there must be no more policies
        assert_eq!(0, next.len());

        let response = { self.transport_options.http_client.execute_request2(request) };

        Ok(response.await?)
    }
}
