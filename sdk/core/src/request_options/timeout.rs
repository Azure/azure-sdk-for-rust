use crate::AppendToUrlQuery;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct Timeout(Duration);

impl Timeout {
    pub fn new(duration: Duration) -> Self {
        Self(duration)
    }
}

impl AppendToUrlQuery for Timeout {
    fn append_to_url_query(&self, url: &mut url::Url) {
        if url.query_pairs().any(|(k, _)| k == "timeout") {
            return;
        }

        url.query_pairs_mut()
            .append_pair("timeout", &format!("{}", self.0.as_secs()));
    }
}

impl From<Duration> for Timeout {
    fn from(d: Duration) -> Self {
        Self(d)
    }
}
