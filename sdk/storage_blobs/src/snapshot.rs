use azure_core::AppendToUrlQuery;

// This type could also be a DateTime
// but the docs clearly states to treat is
// as opaque so we do not convert it in
// any way.
// see: https://docs.microsoft.com/rest/api/storageservices/get-blob
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snapshot(String);

impl Snapshot {
    pub fn new(snapshot: String) -> Self {
        Self(snapshot)
    }
}

impl AppendToUrlQuery for &Snapshot {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut().append_pair("snapshot", &self.0);
    }
}

impl<S> From<S> for Snapshot
where
    S: Into<String>,
{
    fn from(snapshot: S) -> Self {
        Self::new(snapshot.into())
    }
}
