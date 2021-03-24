use azure_core::AppendToUrlQuery;

// This type could also be a DateTime
// but the docs clearly states to treat is
// as opaque so we do not convert it in
// any way.
// see: https://docs.microsoft.com/en-us/rest/api/storageservices/get-blob
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionId(String);

impl VersionId {
    pub fn new(version_id: String) -> Self {
        Self(version_id)
    }
}

impl AppendToUrlQuery for &VersionId {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut().append_pair("version_id", &self.0);
    }
}

impl<S> From<S> for VersionId
where
    S: Into<String>,
{
    fn from(version_id: S) -> Self {
        Self::new(version_id.into())
    }
}
