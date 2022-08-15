use azure_core::AppendToUrlQuery;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockListType {
    Committed,
    Uncommitted,
    All,
}

impl BlockListType {
    pub fn to_str(&self) -> &str {
        match self {
            BlockListType::All => "all",
            BlockListType::Committed => "committed",
            BlockListType::Uncommitted => "uncommitted",
        }
    }
}

impl AppendToUrlQuery for BlockListType {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("blocklisttype", self.to_str());
    }
}
