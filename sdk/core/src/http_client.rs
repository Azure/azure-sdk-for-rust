use std::sync::Arc;

#[derive(Clone)]
pub struct HttpClientArc(Arc<reqwest::Client>);

impl HttpClientArc {
    pub fn new(http_client: reqwest::Client) -> HttpClientArc {
        HttpClientArc(Arc::new(http_client))
    }
}

impl AsRef<reqwest::Client> for HttpClientArc {
    fn as_ref(&self) -> &reqwest::Client {
        self.0.as_ref()
    }
}

impl Default for HttpClientArc {
    fn default() -> Self {
        Self::new(reqwest::Client::new())
    }
}
