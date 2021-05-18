use azure_core::errors::*;

pub struct EventGridResponse {
    response: ResponseFuture,
}

impl EventGridResponse {
    pub async fn accept(self) -> Result<(StatusCode, HeaderMap, Bytes), AzureError> {
        Ok({
            let (head, body) = self
                .response
                .await
                .map_err(HttpError::ExecuteRequestError)?
                .into_parts();
            (
                head.status,
                head.headers,
                body::to_bytes(body)
                    .await
                    .map_err(HttpError::ReadBytesError)?,
            )
        })
    }

    pub async fn expect(
        self,
        expected_status_code: StatusCode,
    ) -> Result<(HeaderMap, Bytes), AzureError> {
        let (status, headers, body) = self.accept().await?;

        match status {
            _ if status == expected_status_code => Ok((headers, body)),
            _ => Err(HttpError::new_unexpected_status_code(
                expected_status_code,
                status,
                std::str::from_utf8(&body)?,
            )
            .into()),
        }
    }
}

impl From<ResponseFuture> for EventGridResponse {
    fn from(response: ResponseFuture) -> EventGridResponse {
        EventGridResponse { response }
    }
}
