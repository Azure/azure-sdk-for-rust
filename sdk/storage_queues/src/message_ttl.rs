use azure_core::AppendToUrlQuery;
use time::Duration;

#[derive(Debug, Clone)]
pub struct MessageTTL(Duration);

impl MessageTTL {
    pub fn new(message_ttl: impl Into<Duration>) -> Self {
        Self(message_ttl.into())
    }
}

impl AppendToUrlQuery for MessageTTL {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("messagettl", &self.0.whole_seconds().to_string());
    }
}

impl From<Duration> for MessageTTL {
    fn from(message_ttl: Duration) -> Self {
        Self(message_ttl)
    }
}
