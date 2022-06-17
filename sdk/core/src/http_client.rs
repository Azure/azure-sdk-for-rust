use crate::error::Result;
#[allow(unused_imports)]
use crate::error::{Error, ErrorKind, ResultExt};
#[allow(unused_imports)]
use crate::Body;
use async_trait::async_trait;
use bytes::Bytes;
#[allow(unused_imports)]
use futures::TryStreamExt;
use http::{Request, Response, StatusCode};
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
    /// Send out a request using `hyperium/http`'s types.
    ///
    /// This method is considered deprecated and should not be used in new code.
    async fn execute_request(&self, request: Request<Bytes>) -> Result<Response<Bytes>>;

    /// Send out a request using `azure_core`'s types.
    ///
    /// This function will be the only one remaining in the trait as soon as the trait stabilizes.
    /// It will be renamed to `execute_request`. The other helper functions (ie
    /// `execute_request_check_status`) will be removed since the status check will be
    /// responsibility of another policy (not the transport one). It does not consume the request.
    /// Implementors are expected to clone the necessary parts of the request and pass them to the
    /// underlying transport.
    async fn execute_request2(&self, request: &crate::Request) -> Result<crate::Response>;

    /// Send out a request and validate it was in the `2xx` range, using
    /// `hyperium/http`'s types.
    ///
    /// Note: the `expected_status` parameter is never used, and instead we
    /// always validate the status was in the `2xx` range. This method should
    /// be considered deprecated, and should not be used in new code.
    async fn execute_request_check_status(
        &self,
        request: Request<Bytes>,
        _expected_status: StatusCode,
    ) -> Result<Response<Bytes>> {
        let response = self.execute_request(request).await?;
        let status = response.status();
        if (200..400).contains(&status.as_u16()) {
            Ok(response)
        } else {
            let body = response.into_body();
            Err(ErrorKind::http_response_from_body(status.into(), &body).into())
        }
    }
}

#[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
impl HttpClient for reqwest::Client {
    async fn execute_request(&self, request: Request<Bytes>) -> Result<Response<Bytes>> {
        let url = url::Url::parse(&request.uri().to_string())?;
        let mut reqwest_request = self.request(request.method().clone(), url);
        for (header, value) in request.headers() {
            reqwest_request = reqwest_request.header(header, value);
        }

        let reqwest_request = reqwest_request
            .body(request.into_body())
            .build()
            .context(ErrorKind::DataConversion, "failed to build request")?;

        let reqwest_response = self
            .execute(reqwest_request)
            .await
            .context(ErrorKind::Io, "failed to execute request")?;

        let mut response = Response::builder().status(reqwest_response.status());

        for (key, value) in reqwest_response.headers() {
            response = response.header(key, value);
        }

        let response = response
            .body(
                reqwest_response
                    .bytes()
                    .await
                    .context(ErrorKind::Io, "failed to read response bytes")?,
            )
            .context(ErrorKind::DataConversion, "failed to build response")?;

        Ok(response)
    }

    async fn execute_request2(&self, request: &crate::Request) -> Result<crate::Response> {
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
pub fn to_json<T>(value: &T) -> Result<Bytes>
where
    T: ?Sized + Serialize,
{
    Ok(Bytes::from(serde_json::to_vec(value)?))
}
