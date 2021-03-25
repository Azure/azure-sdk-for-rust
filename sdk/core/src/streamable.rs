use bytes::Bytes;
use http::HeaderMap;
use http_body::Body;

enum SupportedResponse {
    Reqwest(reqwest::Response),
    Hyper(hyper::Response<hyper::Body>),
}

pub struct Streamable {
    response: SupportedResponse,
}

impl Streamable {
    pub(crate) fn new_hyper(response: hyper::Response<hyper::Body>) -> Self {
        Self {
            response: SupportedResponse::Hyper(response),
        }
    }

    pub(crate) fn new_reqwest(response: reqwest::Response) -> Self {
        Self {
            response: SupportedResponse::Reqwest(response),
        }
    }

    pub fn headers(&self) -> &HeaderMap {
        match &self.response {
            SupportedResponse::Reqwest(reqwest_response) => reqwest_response.headers(),
            SupportedResponse::Hyper(hyper_response) => hyper_response.headers(),
        }
    }

    pub fn status(&self) -> http::StatusCode {
        match &self.response {
            SupportedResponse::Reqwest(reqwest_response) => reqwest_response.status(),
            SupportedResponse::Hyper(hyper_response) => hyper_response.status(),
        }
    }

    pub async fn into_bytes(self) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
        Ok(match self.response {
            SupportedResponse::Reqwest(reqwest_response) => reqwest_response.bytes().await?,
            SupportedResponse::Hyper(hyper_response) => {
                hyper::body::to_bytes(hyper_response.into_body()).await?
            }
        })
    }

    pub async fn chunk(
        &mut self,
    ) -> Result<Option<Bytes>, Box<dyn std::error::Error + Send + Sync>> {
        match &mut self.response {
            SupportedResponse::Reqwest(reqwest_response) => reqwest_response
                .chunk()
                .await
                .map_err(|reqwest_err| reqwest_err.into()),
            SupportedResponse::Hyper(hyper_response) => hyper_response
                .data()
                .await
                .transpose()
                .map_err(|err| err.into()),
        }
    }
}
