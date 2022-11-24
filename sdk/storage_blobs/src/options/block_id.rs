use azure_core::AppendToUrlQuery;
use bytes::Bytes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockId(Bytes);

impl BlockId {
    /// Returns a new block id,
    ///
    pub fn new(block_id: impl Into<Bytes>) -> Self {
        Self(block_id.into())
    }

    /// Returns clone of bytes,
    ///
    pub fn bytes(&self) -> Bytes {
        self.0.clone()
    }
}

impl AppendToUrlQuery for BlockId {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("blockid", &base64::encode(&self.0));
    }
}

impl<B> From<B> for BlockId
where
    B: Into<Bytes>,
{
    fn from(v: B) -> Self {
        Self::new(v)
    }
}

impl AsRef<[u8]> for BlockId {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
