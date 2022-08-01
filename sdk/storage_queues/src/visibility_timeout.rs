use azure_core::AppendToUrlQuery;
use time::Duration;

#[derive(Debug, Clone)]
pub struct VisibilityTimeout(Duration);

impl VisibilityTimeout {
    pub fn new(visibility_timeout: impl Into<Duration>) -> Self {
        Self(visibility_timeout.into())
    }
}

impl AppendToUrlQuery for VisibilityTimeout {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("visibilitytimeout", &self.0.whole_seconds().to_string());
    }
}

impl From<Duration> for VisibilityTimeout {
    fn from(visibility_timeout: Duration) -> Self {
        Self(visibility_timeout)
    }
}
