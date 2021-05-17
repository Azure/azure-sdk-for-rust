use crate::errors::*;
use async_trait::async_trait;
use bytes::Bytes;
use http::{Request, Response, StatusCode};
#[cfg(feature = "enable_hyper")]
use hyper_rustls::HttpsConnector;
use serde::Serialize;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait HttpClient: Send + Sync + std::fmt::Debug {
    async fn execute_request(&self, request: Request<Bytes>) -> Result<Response<Bytes>, HttpError>;

    async fn execute_request_check_status(
        &self,
        request: Request<Bytes>,
        expected_status: StatusCode,
    ) -> Result<Response<Bytes>, HttpError> {
        let response = self.execute_request(request).await?;
        if expected_status != response.status() {
            Err(HttpError::new_unexpected_status_code(
                expected_status,
                response.status(),
                std::str::from_utf8(response.body())?,
            ))
        } else {
            Ok(response)
        }
    }

    async fn execute_request_check_statuses(
        &self,
        request: Request<Bytes>,
        expected_statuses: &[StatusCode],
    ) -> Result<Response<Bytes>, HttpError> {
        let response = self.execute_request(request).await?;
        if !expected_statuses
            .iter()
            .any(|expected_status| *expected_status == response.status())
        {
            if expected_statuses.len() == 1 {
                Err(HttpError::new_unexpected_status_code(
                    expected_statuses[0],
                    response.status(),
                    std::str::from_utf8(response.body())?,
                ))
            } else {
                Err(HttpError::new_multiple_unexpected_status_code(
                    expected_statuses.to_vec(),
                    response.status(),
                    std::str::from_utf8(response.body())?,
                ))
            }
        } else {
            Ok(response)
        }
    }
}

#[cfg(feature = "enable_hyper")]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl HttpClient for hyper::Client<HttpsConnector<hyper::client::HttpConnector>> {
    async fn execute_request(&self, request: Request<Bytes>) -> Result<Response<Bytes>, HttpError> {
        let mut hyper_request = hyper::Request::builder()
            .uri(request.uri())
            .method(request.method());

        for header in request.headers() {
            hyper_request = hyper_request.header(header.0, header.1);
        }

        let hyper_request = hyper_request
            .body(hyper::Body::from(request.into_body()))
            .map_err(HttpError::BuildRequestError)?;

        let hyper_response = self
            .request(hyper_request)
            .await
            .map_err(HttpError::ExecuteRequestError)?;

        let mut response = Response::builder()
            .status(hyper_response.status())
            .version(hyper_response.version());

        for (key, value) in hyper_response.headers() {
            response = response.header(key, value);
        }

        let response = response
            .body(
                hyper::body::to_bytes(hyper_response.into_body())
                    .await
                    .map_err(HttpError::ReadBytesError)?,
            )
            .map_err(HttpError::BuildResponseError)?;

        Ok(response)
    }
}

#[cfg(feature = "enable_reqwest")]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl HttpClient for reqwest::Client {
    async fn execute_request(&self, request: Request<Bytes>) -> Result<Response<Bytes>, HttpError> {
        let mut reqwest_request =
            self.request(request.method().clone(), &request.uri().to_string());
        for header in request.headers() {
            reqwest_request = reqwest_request.header(header.0, header.1);
        }

        let reqwest_request = reqwest_request
            .body(request.into_body())
            .build()
            .map_err(HttpError::BuildClientRequestError)?;

        let reqwest_response = self
            .execute(reqwest_request)
            .await
            .map_err(HttpError::ExecuteRequestError)?;

        let mut response = Response::builder().status(reqwest_response.status());

        if let Some(version) = get_version(&reqwest_response) {
            response = response.version(version);
        }

        for (key, value) in reqwest_response.headers() {
            response = response.header(key, value);
        }

        let response = response
            .body(
                reqwest_response
                    .bytes()
                    .await
                    .map_err(HttpError::ReadBytesError)?,
            )
            .map_err(HttpError::BuildResponseError)?;

        Ok(response)
    }
}

// wasm can not get the http version
#[cfg(feature = "enable_reqwest")]
#[cfg(target_arch = "wasm32")]
fn get_version(_response: &reqwest::Response) -> Option<http::Version> {
    None
}

#[cfg(feature = "enable_reqwest")]
#[cfg(not(target_arch = "wasm32"))]
fn get_version(response: &reqwest::Response) -> Option<http::Version> {
    Some(response.version())
}

/// Serialize to json
pub fn to_json<T>(value: &T) -> Result<Bytes, serde_json::Error>
where
    T: ?Sized + Serialize,
{
    Ok(Bytes::from(serde_json::to_vec(value)?))
}
