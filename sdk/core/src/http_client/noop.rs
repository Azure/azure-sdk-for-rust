use async_trait::async_trait;

#[derive(Debug)]
pub struct NoopClient;

#[async_trait]
impl crate::HttpClient for NoopClient {
    async fn execute_request(&self, request: &crate::Request) -> crate::Result<crate::Response> {
        panic!(
            "A request was called on the default http client `NoopClient`.\
	This client does nothing but panic. Make sure to enable an http\
	 client that can actually perform requests. You can do this by ensuring that the `reqwest` feature is enabled.\n\
     Request:\n{request:?}"
        );
    }
}
