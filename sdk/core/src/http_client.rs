#[allow(unused_imports)]
use crate::Body;
use crate::HttpError;
use async_trait::async_trait;
use bytes::Bytes;
#[allow(unused_imports)]
use futures::TryStreamExt;
use http::{Request, Response, StatusCode};
use serde::Serialize;

/// Construct a new `HttpClient` with the `reqwest` backend.
#[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
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
    async fn execute_request(&self, request: Request<Bytes>) -> Result<Response<Bytes>, HttpError>;

    /// Send out a request using `azure_core`'s types.
    ///
    /// This function will be the only one remaining in the trait as soon as the trait stabilizes.
    /// It will be renamed to `execute_request`. The other helper functions (ie
    /// `execute_request_check_status`) will be removed since the status check will be
    /// responsibility of another policy (not the transport one). It does not consume the request.
    /// Implementors are expected to clone the necessary parts of the request and pass them to the
    /// underlying transport.
    async fn execute_request2(
        &self,
        request: &crate::Request,
    ) -> Result<crate::Response, HttpError>;

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
    ) -> Result<Response<Bytes>, HttpError> {
        let response = self.execute_request(request).await?;
        let status = response.status();
        if (200..400).contains(&status.as_u16()) {
            Ok(response)
        } else {
            let body = std::str::from_utf8(response.body())?.to_owned();
            Err(crate::HttpError::StatusCode { status, body })
        }
    }
}

#[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl HttpClient for reqwest::Client {
    async fn execute_request(&self, request: Request<Bytes>) -> Result<Response<Bytes>, HttpError> {
        let url = url::Url::parse(&request.uri().to_string())?;
        let mut reqwest_request = self.request(request.method().clone(), url);
        for (header, value) in request.headers() {
            reqwest_request = reqwest_request.header(header, value);
        }

        let reqwest_request = reqwest_request
            .body(request.into_body())
            .build()
            .map_err(|error| HttpError::BuildClientRequest(error.into()))?;

        let reqwest_response = self
            .execute(reqwest_request)
            .await
            .map_err(|error| HttpError::ExecuteRequest(error.into()))?;

        let mut response = Response::builder().status(reqwest_response.status());

        for (key, value) in reqwest_response.headers() {
            response = response.header(key, value);
        }

        let response = response
            .body(
                reqwest_response
                    .bytes()
                    .await
                    .map_err(|error| HttpError::ReadBytes(error.into()))?,
            )
            .map_err(HttpError::BuildResponse)?;

        Ok(response)
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn execute_request2(
        &self,
        request: &crate::Request,
    ) -> Result<crate::Response, HttpError> {
        let url = url::Url::parse(&request.uri().to_string())?;
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
                .map_err(|error| HttpError::BuildClientRequest(error.into()))?,
            Body::SeekableStream(mut seekable_stream) => {
                seekable_stream.reset().await.unwrap(); // TODO: remove unwrap when `HttpError` has been removed

                reqwest_request
                    .body(reqwest::Body::wrap_stream(seekable_stream))
                    .build()
                    .map_err(|error| HttpError::BuildClientRequest(error.into()))?
            }
        };

        let reqwest_response = self
            .execute(reqwest_request)
            .await
            .map_err(|error| HttpError::ExecuteRequest(error.into()))?;
        let mut response = crate::ResponseBuilder::new(reqwest_response.status());

        for (key, value) in reqwest_response.headers() {
            response.with_header(key, value.clone());
        }

        let response =
            response.with_pinned_stream(Box::pin(reqwest_response.bytes_stream().map_err(|e| {
                crate::error::Error::full(
                    crate::error::ErrorKind::Io,
                    e,
                    "error converting `reqwest` request into a byte stream",
                )
            })));

        Ok(response)
    }

    #[cfg(target_arch = "wasm32")]
    /// Stub implementation. Will remove as soon as reqwest starts
    /// supporting wasm.
    async fn execute_request2(
        &self,
        _request: &crate::Request,
    ) -> Result<crate::Response, HttpError> {
        let response = crate::ResponseBuilder::new(http::StatusCode::OK);

        let response = response.with_pinned_stream(Box::pin(crate::BytesStream::new_empty()));

        Ok(response)
    }
}

/// Serialize a type to json.
pub fn to_json<T>(value: &T) -> Result<Bytes, serde_json::Error>
where
    T: ?Sized + Serialize,
{
    Ok(Bytes::from(serde_json::to_vec(value)?))
}
