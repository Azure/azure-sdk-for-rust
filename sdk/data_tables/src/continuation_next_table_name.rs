use azure_core::AppendToUrlQuery;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinuationNextTableName(String);

impl ContinuationNextTableName {
    pub fn new(continuation_next_table_name: String) -> Self {
        Self(continuation_next_table_name)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn from_header_optional(headers: &http::HeaderMap) -> crate::Result<Option<Self>> {
        let header_as_str = headers
            .get("x-ms-continuation-NextTableName")
            .map(|item| item.to_str())
            .transpose()?;

        Ok(header_as_str
            .filter(|h| !h.is_empty())
            .map(|h| ContinuationNextTableName::new(h.to_owned())))
    }
}

impl AppendToUrlQuery for ContinuationNextTableName {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut().append_pair("NextTableName", &self.0);
    }
}

impl From<String> for ContinuationNextTableName {
    fn from(continuation_next_table_name: String) -> Self {
        Self::new(continuation_next_table_name)
    }
}

impl From<&str> for ContinuationNextTableName {
    fn from(continuation_next_table_name: &str) -> Self {
        Self::new(continuation_next_table_name.to_owned())
    }
}
