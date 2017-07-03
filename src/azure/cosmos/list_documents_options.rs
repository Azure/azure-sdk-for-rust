use azure::cosmos::ConsistencyLevel;
use azure::core::incompletevector::{ContinuationToken, NO_CONTINUATION_TOKEN};

pub struct ListDocumentsOptions<'a> {
    pub max_item_count: Option<u32>,
    pub continuation_token: &'a ContinuationToken,
    pub consistency_level_override: Option<ConsistencyLevel>,
    pub session_token: Option<&'a str>,
    pub incremental_feed: bool,
    pub if_none_match: Option<&'a str>,
    pub partition_range_id: Option<&'a str>,
}

pub const LIST_DOCUMENTS_OPTIONS_DEFAULT: ListDocumentsOptions = ListDocumentsOptions {
    max_item_count: None,
    continuation_token: NO_CONTINUATION_TOKEN,
    consistency_level_override: None,
    session_token: None,
    incremental_feed: false,
    if_none_match: None,
    partition_range_id: None,
};
