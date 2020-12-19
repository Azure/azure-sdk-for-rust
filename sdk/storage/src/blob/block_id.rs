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
