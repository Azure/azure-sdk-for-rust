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
}

impl std::convert::From<(Url, ResponseFuture)> for PerformRequestResponse {
    fn from(values: (Url, ResponseFuture)) -> Self {
        PerformRequestResponse {
            url: values.0,
            response_future: values.1,
        }
    }
}
