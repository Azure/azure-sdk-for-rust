use crate::AppendToUrlQuery;

#[derive(Debug, Clone, PartialEq)]
pub struct NextMarker(String);

impl NextMarker {
    pub fn new(next_marker: String) -> Self {
        Self(next_marker)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AppendToUrlQuery for NextMarker {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut().append_pair("marker", &self.0);
    }
}

impl From<String> for NextMarker {
    fn from(next_marker: String) -> Self {
        Self::new(next_marker)
    }
}
