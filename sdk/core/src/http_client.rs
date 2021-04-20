use crate::errors::{AzureError, UnexpectedHTTPResult};
use crate::Body;
use async_std::fs::File;
use async_trait::async_trait;
use bytes::Bytes;
use futures::TryStreamExt;
use http::{Request, Response, StatusCode};
#[cfg(feature = "enable_hyper")]
use hyper_rustls::HttpsConnector;
use serde::Serialize;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait HttpClient: Send + Sync + std::fmt::Debug {
    async fn execute_request(
        &self,
        request: Request<Bytes>,
    ) -> Result<Response<Bytes>, Box<dyn std::error::Error + Sync + Send>>;

    /// This function will be the only one remaining in the trait as soon as the trait stabilizes.
    /// It will be renamed to `execute_request`. The other helper functions (ie
    /// `execute_request_check_status`) will be removed since the status check will be
    /// responsibility of another policy (not the transport one).
    async fn execute_request2(
        &self,
        request: &mut crate::Request,
    ) -> Result<crate::Response, Box<dyn std::error::Error + Sync + Send>>;

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

// TODO: To reimplement once the Request and Response are validated.
//#[cfg(feature = "enable_hyper")]
//#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
//#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
//impl HttpClient for hyper::Client<HttpsConnector<hyper::client::HttpConnector>> {
//    async fn execute_request(
//        &self,
//        request: Request<Bytes>,
//    ) -> Result<Response<Bytes>, Box<dyn std::error::Error + Sync + Send>> {
//        let mut hyper_request = hyper::Request::builder()
//            .uri(request.uri())
//            .method(request.method());
//
//        for header in request.headers() {
//            hyper_request = hyper_request.header(header.0, header.1);
//        }
//
//        let hyper_request = hyper_request.body(hyper::Body::from(request.into_body()))?;
//
//        let hyper_response = self.request(hyper_request).await?;
//
//        let mut response = Response::builder()
//            .status(hyper_response.status())
//            .version(hyper_response.version());
//
//        for (key, value) in hyper_response.headers() {
//            response = response.header(key, value);
//        }
//
//        let response = response.body(hyper::body::to_bytes(hyper_response.into_body()).await?)?;
//
//        Ok(response)
//    }
//}

#[cfg(feature = "enable_reqwest")]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl HttpClient for reqwest::Client {
    async fn execute_request(
        &self,
        request: Request<Bytes>,
    ) -> Result<Response<Bytes>, Box<dyn std::error::Error + Sync + Send>> {
        let mut reqwest_request = self.request(
            request.method().clone(),
            url::Url::parse(&request.uri().to_string()).unwrap(),
        );
        for header in request.headers() {
            reqwest_request = reqwest_request.header(header.0, header.1);
        }

        let reqwest_request = reqwest_request.body(request.into_body()).build()?;

        let reqwest_response = self.execute(reqwest_request).await?;

        let mut response = Response::builder()
            .status(reqwest_response.status())
            .version(reqwest_response.version());

        for (key, value) in reqwest_response.headers() {
            response = response.header(key, value);
        }

        let response = response.body(reqwest_response.bytes().await?)?;

        Ok(response)
    }

    async fn execute_request2(
        &self,
        request: &mut crate::Request,
    ) -> Result<crate::Response, Box<dyn std::error::Error + Sync + Send>> {
        let mut reqwest_request = self.request(
            request.method().clone(),
            url::Url::parse(&request.uri().to_string()).unwrap(),
        );
        for header in request.headers() {
            reqwest_request = reqwest_request.header(header.0, header.1);
        }

        let body = request.take_body();

        let reqwest_request = match body {
            Body::Bytes(bytes) => {
                // replace the body for subsequent retries
                request.set_body(bytes.clone().into());
                reqwest_request.body(bytes).build()?
            }
            Body::Path(path) => {
                // replace the body for subsequent retries
                request.set_body(path.clone().into());
                let file = File::open(path).await?;
                let seekable_stream = crate::SelfResetStream::new(file).await?;
                reqwest_request
                    .body(reqwest::Body::wrap_stream(seekable_stream))
                    .build()?
            }
            Body::SeekableStream(mut seekable_stream) => {
                // replace the body for subsequent retries
                let stream_clone = dyn_clone::clone_box(seekable_stream.as_ref());
                request.set_body(stream_clone.into());

                seekable_stream.reset().await?;

                reqwest_request
                    .body(reqwest::Body::wrap_stream(seekable_stream))
                    .build()?
            }
        };

        let reqwest_response = self.execute(reqwest_request).await?;
        let mut response = crate::ResponseBuilder::new(reqwest_response.status());

        for (key, value) in reqwest_response.headers() {
            response.with_header(key, value.clone());
        }

        let response = response.with_pinned_stream(Box::pin(
            reqwest_response.bytes_stream().map_err(|err| err.into()),
        ));

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
#[allow(dead_code)]
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
