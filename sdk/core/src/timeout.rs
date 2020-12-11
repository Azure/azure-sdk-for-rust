use crate::AppendToUrlQuery;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Timeout(Duration);

impl Timeout {
    pub fn new(duration: Duration) -> Self {
        Self(duration)
    }
}

impl AppendToUrlQuery for Timeout {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("timeout", &format!("{}", self.0.as_secs()));
    }
}

impl AppendToUrlQuery for Option<Timeout> {
    fn append_to_url_query(&self, url: &mut url::Url) {
        self.as_ref()
            .map(|timeout| AppendToUrlQuery::append_to_url_query(timeout, url));
    }
}
