use azure_core::errors::*;
use hyper::{
    body::{self, Bytes},
    client::ResponseFuture,
    HeaderMap, StatusCode,
};

pub struct EventGridResponse {
    response: ResponseFuture,
}

impl EventGridResponse {
    pub async fn accept(self) -> Result<(StatusCode, HeaderMap, Bytes), AzureError> {
        Ok({
            let (head, body) = self.response.await?.into_parts();
            (head.status, head.headers, body::to_bytes(body).await?)
        })
    }

    pub async fn expect(
        self,
        expected_status_code: StatusCode,
    ) -> Result<(HeaderMap, Bytes), AzureError> {
        let (status, headers, body) = self.accept().await?;

        match status {
            _ if status == expected_status_code => Ok((headers, body)),
            _ => Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
                expected_status_code,
                status,
                std::str::from_utf8(&body)?,
            ))),
        }
    }
}

impl From<ResponseFuture> for EventGridResponse {
    fn from(response: ResponseFuture) -> EventGridResponse {
        EventGridResponse { response }
    }
}
