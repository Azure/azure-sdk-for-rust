use crate::AppendToUrlQuery;

#[derive(Debug, Clone)]
pub struct Delimiter(String);

impl AppendToUrlQuery for Delimiter {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut().append_pair("delimiter", self.0.as_ref());
    }
}

impl From<&str> for Delimiter {
    fn from(delimiter: &str) -> Self {
        Self(delimiter.into())
    }
}

impl From<String> for Delimiter {
    fn from(delimiter: String) -> Self {
        Self(delimiter)
    }
}