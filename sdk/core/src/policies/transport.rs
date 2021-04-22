#[allow(unused_imports)]
use crate::policies::{Policy, PolicyResult};
#[allow(unused_imports)]
use crate::{Context, HttpClient, Request, Response};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TransportOptions {
    http_client: Arc<dyn HttpClient>,
}

impl TransportOptions {
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        Self { http_client }
    }
}

#[derive(Debug, Clone)]
pub struct TransportPolicy {
    options: TransportOptions,
}

impl TransportPolicy {
    pub fn new(options: TransportOptions) -> Self {
        Self { options }
    }
}

#[async_trait::async_trait]
#[cfg(not(target_arch = "wasm32"))]
impl Policy for TransportPolicy {
    async fn send(
        &self,
        _ctx: &mut Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult<Response> {
        // there must be no more policies
        assert_eq!(0, next.len());

        let response = { self.options.http_client.execute_request2(request) };

        Ok(response.await?.into())
    }
}
