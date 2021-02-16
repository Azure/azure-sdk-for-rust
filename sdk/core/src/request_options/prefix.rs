use crate::AppendToUrlQuery;

#[derive(Debug, Clone)]
pub struct Prefix<'a>(&'a str);

impl<'a> Prefix<'a> {
    pub fn new(prefix: &'a str) -> Self {
        Self(prefix)
    }
}

impl<'a> AppendToUrlQuery for Prefix<'a> {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut().append_pair("prefix", &self.0);
    }
}

impl<'a> From<&'a str> for Prefix<'a> {
    fn from(s: &'a str) -> Self {
        Self(s)
    }
}
