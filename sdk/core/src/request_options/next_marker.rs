use crate::AppendToUrlQuery;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NextMarker(String);

impl NextMarker {
    pub fn new(next_marker: String) -> Self {
        Self(next_marker)
    }

    pub fn from_possibly_empty_string(next_marker: Option<String>) -> Option<Self> {
        if let Some(nm) = next_marker {
            if nm.is_empty() {
                None
            } else {
                Some(NextMarker::new(nm))
            }
        } else {
            None
        }
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

impl From<&str> for NextMarker {
    fn from(next_marker: &str) -> Self {
        Self::new(next_marker.to_owned())
    }
}
