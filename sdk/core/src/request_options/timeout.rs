use crate::AppendToUrlQuery;
use time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct Timeout(Duration);

impl Timeout {
    pub fn new(duration: Duration) -> Self {
        Self(duration)
    }
}

impl AppendToUrlQuery for Timeout {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("timeout", &format!("{}", self.0.whole_seconds()));
    }
}

impl From<Duration> for Timeout {
    fn from(d: Duration) -> Self {
        Self(d)
    }
}
