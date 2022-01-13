use azure_core::*;

pub struct EventGridResponse {
    response: ResponseFuture,
}

impl EventGridResponse {
    pub async fn accept(self) -> Result<(StatusCode, HeaderMap, Bytes), azure_core::Error> {
        Ok({
            let (head, body) = self
                .response
                .await
                .map_err(HttpError::ExecuteRequest)?
                .into_parts();
            (
                head.status,
                head.headers,
                body::to_bytes(body).await.map_err(HttpError::ReadBytes)?,
            )
        })
    }

    pub async fn expect(
        self,
        expected_status_code: StatusCode,
    ) -> Result<(HeaderMap, Bytes), azure_core::Error> {
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
