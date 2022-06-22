use crate::AppendToUrlQuery;

#[derive(Debug, Clone)]
pub struct Delimiter<'a>(&'a str);

impl<'a> Delimiter<'a> {
    #[must_use]
    pub fn new(delimiter: &'a str) -> Self {
        Self(delimiter)
    }
}

impl<'a> AppendToUrlQuery for Delimiter<'a> {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut().append_pair("delimiter", self.0);
    }
}

impl<'a> From<&'a str> for Delimiter<'a> {
    fn from(delimiter: &'a str) -> Self {
        Self(delimiter)
    }
}
