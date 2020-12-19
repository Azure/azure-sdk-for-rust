use azure_core::AppendToUrlQuery;

#[derive(Debug, Clone)]
pub struct BlockId(Vec<u8>);

impl BlockId {
    pub fn new(block_id: Vec<u8>) -> Self {
        Self(block_id)
    }
}

impl AppendToUrlQuery for BlockId {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("blockid", &base64::encode(&self.0));
    }
}

impl From<Vec<u8>> for BlockId {
    fn from(v: Vec<u8>) -> Self {
        Self(v)
    }
}

impl From<&[u8]> for BlockId {
    fn from(slice: &[u8]) -> Self {
        Self(slice.to_owned())
    }
}

impl From<&str> for BlockId {
    fn from(s: &str) -> Self {
        Self(s.as_bytes().to_owned())
    }
}
