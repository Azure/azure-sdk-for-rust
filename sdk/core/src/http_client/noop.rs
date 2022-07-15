use async_trait::async_trait;

#[derive(Debug)]
pub struct NoopClient;

// TODO(rylev): we probably don't want to limit this to wasm32
// as there will be wasm environments with threads.
// This should instead be a feature flag
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl crate::HttpClient for NoopClient {
    async fn execute_request(&self, _request: &crate::Request) -> crate::Result<crate::Response> {
        panic!(
            "A request was called on the default http client `NoopClient`.\
	This client does nothing but panic by default. Make sure to enable an http\
	 client that can actually perform requests. **TODO**: link to instructions on how to do this "
        );
    }
}
