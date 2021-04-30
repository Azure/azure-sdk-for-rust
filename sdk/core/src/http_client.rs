use crate::errors::*;
use async_trait::async_trait;
use bytes::Bytes;
use http::{Request, Response, StatusCode};
#[cfg(feature = "enable_hyper")]
use hyper_rustls::HttpsConnector;
use serde::Serialize;

#[cfg(feature = "enable_wasi_experimental_http")]
use wasi_experimental_http;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait HttpClient: Send + Sync + std::fmt::Debug {
    async fn execute_request(
        &self,
        request: Request<Bytes>,
    ) -> Result<Response<Bytes>, Box<dyn std::error::Error + Sync + Send>>;

    async fn execute_request_check_status(
        &self,
        request: Request<Bytes>,
        expected_status: StatusCode,
    ) -> Result<Response<Bytes>, Box<dyn std::error::Error + Sync + Send>> {
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
    ) -> Result<Response<Bytes>, Box<dyn std::error::Error + Sync + Send>> {
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

// #[cfg(feature = "enable_hyper")]
// #[cfg(not(feature = "enable_wasi_experimental_http"))]
// // #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
// #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
// impl HttpClient for hyper::Client<HttpsConnector<hyper::client::HttpConnector>> {
//     async fn execute_request(
//         &self,
//         request: Request<Bytes>,
//     ) -> Result<Response<Bytes>, Box<dyn std::error::Error + Sync + Send>> {
//         let mut hyper_request = hyper::Request::builder()
//             .uri(request.uri())
//             .method(request.method());

//         for header in request.headers() {
//             hyper_request = hyper_request.header(header.0, header.1);
//         }

//         let hyper_request = hyper_request.body(hyper::Body::from(request.into_body()))?;

//         let hyper_response = self.request(hyper_request).await?;

//         let mut response = Response::builder()
//             .status(hyper_response.status())
//             .version(hyper_response.version());

//         for (key, value) in hyper_response.headers() {
//             response = response.header(key, value);
//         }

//         let response = response.body(hyper::body::to_bytes(hyper_response.into_body()).await?)?;

//         Ok(response)
//     }
// }

// #[cfg(feature = "enable_reqwest")]
// // #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
// #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
// #[cfg(not(feature = "enable_wasi_experimental_http"))]
// impl HttpClient for reqwest::Client {
//     async fn execute_request(
//         &self,
//         request: Request<Bytes>,
//     ) -> Result<Response<Bytes>, Box<dyn std::error::Error + Sync + Send>> {
//         let mut reqwest_request =
//             self.request(request.method().clone(), &request.uri().to_string());
//         for header in request.headers() {
//             reqwest_request = reqwest_request.header(header.0, header.1);
//         }

//         let reqwest_request = reqwest_request.body(request.into_body()).build()?;

//         let reqwest_response = self.execute(reqwest_request).await?;

//         let mut response = Response::builder().status(reqwest_response.status());

//         if let Some(version) = get_version(&reqwest_response) {
//             response = response.version(version);
//         }

//         for (key, value) in reqwest_response.headers() {
//             response = response.header(key, value);
//         }

//         let response = response.body(reqwest_response.bytes().await?)?;

//         Ok(response)
//     }
// }

#[cfg(feature = "enable_wasi_experimental_http")]
#[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
#[derive(Debug)]
pub struct WasiHttpClient {}

#[cfg(feature = "enable_wasi_experimental_http")]
#[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
#[cfg_attr(target_os = "wasi", async_trait(?Send))]
impl HttpClient for WasiHttpClient {
    async fn execute_request(
        &self,
        request: Request<Bytes>,
    ) -> Result<Response<Bytes>, Box<dyn std::error::Error + Sync + Send>> {
        let mut r = http::request::Builder::new()
            .uri(request.uri())
            .method(request.method().clone());
        for header in request.headers() {
            r = r.header(header.0, header.1);
        }
        let r = r.body(Some(request.body().clone()))?;
        let mut x = wasi_experimental_http::request(r)?;
        let headers = x.headers_get_all()?;
        let mut res = http::response::Builder::new().status(x.status_code);
        for header in headers {
            res = res.header(header.0.unwrap(), header.1);
        }
        let res = res.body(Bytes::from(x.body_read_all()?))?;
        Ok(res)
    }
}

// wasm can not get the http version
#[cfg(feature = "enable_reqwest")]
#[cfg(not(feature = "enable_wasi_experimental_http"))]
#[cfg(target_arch = "wasm32")]
fn get_version(_response: &reqwest::Response) -> Option<http::Version> {
    None
}

#[cfg(feature = "enable_reqwest")]
#[cfg(not(feature = "enable_wasi_experimental_http"))]
#[cfg(not(target_arch = "wasm32"))]
fn get_version(response: &reqwest::Response) -> Option<http::Version> {
    Some(response.version())
}

/// Serialize to json
pub fn to_json<T>(value: &T) -> Result<Bytes, Box<dyn std::error::Error + Sync + Send>>
where
    T: ?Sized + Serialize,
{
    Ok(Bytes::from(serde_json::to_vec(value)?))
}
