#[allow(unused_imports)]
use crate::error::{Error, ErrorKind, ResultExt};
#[allow(unused_imports)]
use crate::Body;
use async_trait::async_trait;
use bytes::Bytes;
#[allow(unused_imports)]
use futures::TryStreamExt;
use serde::Serialize;

/// Construct a new `HttpClient` with the `reqwest` backend.
#[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
#[cfg(not(target_arch = "wasm32"))]
pub fn new_http_client() -> std::sync::Arc<dyn HttpClient> {
    std::sync::Arc::new(reqwest::Client::new())
}

/// An HTTP client which can send requests.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait HttpClient: Send + Sync + std::fmt::Debug {
    /// Send out a request using `azure_core`'s types.
    ///
    /// This function will be the only one remaining in the trait as soon as the trait stabilizes.
    /// It will be renamed to `execute_request`. The other helper functions (ie
    /// `execute_request_check_status`) will be removed since the status check will be
    /// responsibility of another policy (not the transport one). It does not consume the request.
    /// Implementors are expected to clone the necessary parts of the request and pass them to the
    /// underlying transport.
    async fn execute_request2(&self, request: &crate::Request) -> crate::Result<crate::Response>;
}

#[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
impl HttpClient for reqwest::Client {
    async fn execute_request2(&self, request: &crate::Request) -> crate::Result<crate::Response> {
        let url = request.url().clone();
        let mut reqwest_request = self.request(request.method(), url);
        for (name, value) in request.headers().iter() {
            reqwest_request = reqwest_request.header(name, value);
        }

        // We clone the body since we need to give ownership of it to Reqwest.
        let body = request.body().clone();

        let reqwest_request = match body {
            Body::Bytes(bytes) => reqwest_request
                .body(bytes)
                .build()
                .context(ErrorKind::Other, "failed to build request")?,
            Body::SeekableStream(mut seekable_stream) => {
                seekable_stream.reset().await.unwrap(); // TODO: remove unwrap when `HttpError` has been removed

                reqwest_request
                    .body(reqwest::Body::wrap_stream(seekable_stream))
                    .build()
                    .context(ErrorKind::Other, "failed to build request")?
            }
        };

        let reqwest_response = self
            .execute(reqwest_request)
            .await
            .context(ErrorKind::Io, "failed to execute request")?;
        let mut response = crate::ResponseBuilder::new(reqwest_response.status());

        for (key, value) in reqwest_response.headers() {
            response.with_header(key, value.clone());
        }

        let response =
            response.with_pinned_stream(Box::pin(reqwest_response.bytes_stream().map_err(|e| {
                Error::full(
                    ErrorKind::Io,
                    e,
                    "error converting `reqwest` request into a byte stream",
                )
            })));

        Ok(response)
    }
}

/// Serialize a type to json.
pub fn to_json<T>(value: &T) -> crate::Result<Bytes>
where
    T: ?Sized + Serialize,
{
    Ok(Bytes::from(serde_json::to_vec(value)?))
}
