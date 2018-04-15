use azure::core::incompletevector::ContinuationToken;
use azure::cosmos::ConsistencyLevel;

#[derive(Debug, Clone)]
pub struct QueryDocumentOptions<'a> {
    pub max_item_count: Option<u64>,
    pub continuation_token: Option<&'a ContinuationToken>,
    pub enable_cross_partition: Option<bool>,
    pub consistency_level_override: Option<ConsistencyLevel>,
    pub session_token: Option<&'a str>,
}

impl<'a> Default for QueryDocumentOptions<'a> {
    fn default() -> QueryDocumentOptions<'a> {
        QUERY_DOCUMENTS_OPTIONS_DEFAULT.clone()
    }
}

const QUERY_DOCUMENTS_OPTIONS_DEFAULT: QueryDocumentOptions = QueryDocumentOptions {
    max_item_count: None,
    continuation_token: None,
    enable_cross_partition: Some(true),
    consistency_level_override: None,
    session_token: None,
};
