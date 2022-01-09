use crate::AppendToUrlQuery;

#[derive(Debug, Clone)]
pub struct Prefix(String);

impl Prefix {
    pub fn new(prefix: String) -> Self {
        Self(prefix)
    }
}

impl AppendToUrlQuery for Prefix {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut().append_pair("prefix", &self.0);
    }
}

impl From<String> for Prefix {
    fn from(prefix: String) -> Self {
        Self::new(prefix)
    }
}

impl From<&str> for Prefix {
    fn from(prefix: &str) -> Self {
        Self::new(prefix.to_owned())
    }
}
