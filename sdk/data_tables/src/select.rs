use azure_core::AppendToUrlQuery;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Select(Cow<'static, str>);

impl Select {
    pub fn new(s: impl Into<Cow<'static, str>>) -> Self {
        Self(s.into())
    }
}

impl AppendToUrlQuery for Select {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("$select", self.0.as_ref());
    }
}

impl<S> From<S> for Select
where
    S: Into<Cow<'static, str>>,
{
    fn from(s: S) -> Self {
        Self::new(s)
    }
}
