#[allow(unused_imports)]
use crate::error::{Error, ErrorKind, ResultExt};
#[allow(unused_imports)]
use crate::Body;
#[allow(unused_imports)]
use crate::{headers::Headers, PinnedStream};
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
    /// It does not consume the request. Implementors are expected to clone the necessary parts
    /// of the request and pass them to the underlying transport.
    async fn execute_request(&self, request: &crate::Request) -> crate::Result<crate::Response>;

    /// Send out the request and collect the response body.
    /// An error is returned if the status is not success.
    async fn execute_request_check_status(
        &self,
        request: &crate::Request,
    ) -> crate::Result<crate::CollectedResponse> {
        let rsp = self.execute_request(request).await?;
        let (status, headers, body) = rsp.deconstruct();
        let body = crate::collect_pinned_stream(body).await?;
        if !status.is_success() {
            return Err(ErrorKind::http_response_from_body(status.as_u16(), &body).into_error());
        }
        Ok(crate::CollectedResponse::new(status, headers, body))
    }
}

#[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
impl HttpClient for reqwest::Client {
    async fn execute_request(&self, request: &crate::Request) -> crate::Result<crate::Response> {
        let url = request.url().clone();
        let mut reqwest_request = self.request(request.method().clone(), url);
        for (name, value) in request.headers().iter() {
            reqwest_request = reqwest_request.header(name, value);
        }

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

        let status = reqwest_response.status();
        let headers = Headers::from(reqwest_response.headers());
        let body: PinnedStream = Box::pin(reqwest_response.bytes_stream().map_err(|error| {
            Error::full(
                ErrorKind::Io,
                error,
                "error converting `reqwest` request into a byte stream",
            )
        }));

        Ok(crate::Response::new(status, headers, body))
    }
}

/// Serialize a type to json.
pub fn to_json<T>(value: &T) -> crate::Result<Bytes>
where
    T: ?Sized + Serialize,
{
    Ok(Bytes::from(serde_json::to_vec(value)?))
}
