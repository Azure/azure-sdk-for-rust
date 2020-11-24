use azure_core::errors::{AzureError, UnexpectedHTTPResult};
use hyper::body;
use hyper::client::ResponseFuture;
use url::Url;

#[derive(Debug)]
pub struct PerformRequestResponse {
    pub(crate) url: Url,
    pub(crate) response_future: ResponseFuture,
}

impl PerformRequestResponse {
    pub fn url(&self) -> &Url {
        &self.url
    }

    pub async fn check_status_extract_headers_and_body(
        self,
        expected_status_code: hyper::StatusCode,
    ) -> Result<(hyper::HeaderMap, body::Bytes), AzureError> {
        let (status, headers, body) = {
            let (head, body) = self.response_future.await?.into_parts();
            (head.status, head.headers, body::to_bytes(body).await?)
        };

        if status == expected_status_code {
            Ok((headers, body))
        } else {
            Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
                expected_status_code,
                status,
                std::str::from_utf8(&body)?,
            )))
        }
    }
}

impl std::convert::From<(Url, ResponseFuture)> for PerformRequestResponse {
    fn from(values: (Url, ResponseFuture)) -> Self {
        PerformRequestResponse {
            url: values.0,
            response_future: values.1,
        }
    }
}
