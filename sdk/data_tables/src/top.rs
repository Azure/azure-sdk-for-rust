use azure_core::AppendToUrlQuery;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Top(u32);

impl Top {
    #[must_use]
    pub fn new(s: u32) -> Self {
        Self(s)
    }
}

impl AppendToUrlQuery for Top {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("$top", &self.0.to_string());
    }
}

impl From<u32> for Top {
    fn from(s: u32) -> Self {
        Self::new(s)
    }
}
