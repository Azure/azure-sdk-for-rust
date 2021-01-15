use crate::errors::{AzureError, UnexpectedHTTPResult};
use async_trait::async_trait;
use bytes::Bytes;
use http::{Request, Response, StatusCode};
use hyper::{self, body, Body};
use hyper_rustls::HttpsConnector;
use serde::Serialize;

#[async_trait]
pub trait HttpClient: Send + Sync + std::fmt::Debug {
    async fn execute_request(
        &self,
        request: Request<Bytes>,
    ) -> Result<Response<Vec<u8>>, Box<dyn std::error::Error + Sync + Send>>;

    async fn execute_request_check_status(
        &self,
        request: Request<Bytes>,
        expected_status: StatusCode,
    ) -> Result<Response<Vec<u8>>, Box<dyn std::error::Error + Sync + Send>> {
        let response = self.execute_request(request).await?;
        if expected_status != response.status() {
            Err(Box::new(AzureError::from(UnexpectedHTTPResult::new(
                expected_status,
                response.status(),
                std::str::from_utf8(response.body())?,
            ))))
        } else {
            Ok(response)
        }
    }

    async fn execute_request_check_statuses(
        &self,
        request: Request<Bytes>,
        expected_statuses: &[StatusCode],
    ) -> Result<Response<Vec<u8>>, Box<dyn std::error::Error + Sync + Send>> {
        let response = self.execute_request(request).await?;
        if !expected_statuses
            .iter()
            .any(|expected_status| *expected_status == response.status())
        {
            if expected_statuses.len() == 1 {
                Err(Box::new(AzureError::from(UnexpectedHTTPResult::new(
                    expected_statuses[0],
                    response.status(),
                    std::str::from_utf8(response.body())?,
                ))))
            } else {
                Err(Box::new(AzureError::from(
                    UnexpectedHTTPResult::new_multiple(
                        expected_statuses.to_vec(),
                        response.status(),
                        std::str::from_utf8(response.body())?,
                    ),
                )))
            }
        } else {
            Ok(response)
        }
    }
}

pub static EMPTY_BODY: &[u8; 0] = &[];

#[async_trait]
impl HttpClient for hyper::Client<HttpsConnector<hyper::client::HttpConnector>> {
    async fn execute_request(
        &self,
        request: Request<Bytes>,
    ) -> Result<Response<Vec<u8>>, Box<dyn std::error::Error + Sync + Send>> {
        let mut hyper_request = hyper::Request::builder()
            .uri(request.uri())
            .method(request.method());

        for header in request.headers() {
            hyper_request = hyper_request.header(header.0, header.1);
        }

        let body = request.into_body();
        let hyper_request = hyper_request.body(Body::from(body))?;

        let hyper_response = self.request(hyper_request).await?;

        let mut response = Response::builder()
            .status(hyper_response.status())
            .version(hyper_response.version());

        for (key, value) in hyper_response.headers() {
            response = response.header(key, value);
        }

        let response = response.body(body::to_bytes(hyper_response.into_body()).await?.to_vec())?;

        Ok(response)
    }
}

#[async_trait]
impl HttpClient for reqwest::Client {
    async fn execute_request(
        &self,
        request: Request<Bytes>,
    ) -> Result<Response<Vec<u8>>, Box<dyn std::error::Error + Sync + Send>> {
        let mut reqwest_request =
            self.request(request.method().clone(), &request.uri().to_string());
        for header in request.headers() {
            reqwest_request = reqwest_request.header(header.0, header.1);
        }

        let body = request.into_body();
        let reqwest_request = reqwest_request.body(body).build()?;

        let reqwest_response = self.execute(reqwest_request).await?;

        let mut response = Response::builder()
            .status(reqwest_response.status())
            .version(reqwest_response.version());

        for (key, value) in reqwest_response.headers() {
            response = response.header(key, value);
        }

        let response = response.body(reqwest_response.bytes().await?.to_vec())?;

        Ok(response)
    }
}

/// Serialize to json
pub fn to_json<T>(value: &T) -> Result<Bytes, Box<dyn std::error::Error + Sync + Send>>
where
    T: ?Sized + Serialize,
{
    Ok(Bytes::from(serde_json::to_vec(value)?))
}
