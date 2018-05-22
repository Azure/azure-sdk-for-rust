use azure::cosmos::{partition_key::PartitionKey, ConsistencyLevel, document::IndexingDirective};
use azure::core::incompletevector::ContinuationToken;

#[derive(Debug, Clone)]
pub struct CreateDocumentOptions<'a> {
    pub indexing_directive: Option<IndexingDirective>,
    pub partition_key: PartitionKey<'a>,
    pub is_upsert: bool
}

impl<'a> ::std::default::Default for CreateDocumentOptions<'a> {
    fn default() -> Self {
        CreateDocumentOptions {
            indexing_directive: None,
            partition_key: PartitionKey::new(),
            is_upsert: false
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetDocumentOptions<'a> {
    pub consistency_level_override: Option<ConsistencyLevel>,
    pub session_token: Option<&'a str>,
    pub if_none_match: Option<&'a str>,
    pub partition_key: PartitionKey<'a>,
}

impl<'a> ::std::default::Default for GetDocumentOptions<'a> {
    fn default() -> Self {
        GetDocumentOptions {
            consistency_level_override: None,
            session_token: None,
            if_none_match: None,
            partition_key: PartitionKey::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListDocumentsOptions<'a> {
    pub max_item_count: Option<u64>,
    pub continuation_token: Option<&'a ContinuationToken>,
    pub consistency_level_override: Option<ConsistencyLevel>,
    pub session_token: Option<&'a str>,
    pub incremental_feed: bool,
    pub if_none_match: Option<&'a str>,
    pub partition_range_id: Option<&'a str>,
}

impl<'a> Default for ListDocumentsOptions<'a> {
    fn default() -> ListDocumentsOptions<'a> {
        ListDocumentsOptions {
            max_item_count: None,
            continuation_token: None,
            consistency_level_override: None,
            session_token: None,
            incremental_feed: false,
            if_none_match: None,
            partition_range_id: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeleteDocumentOptions<'a> {
    pub if_match: Option<&'a str>,
    pub partition_key: PartitionKey<'a>,
}

impl<'a> ::std::default::Default for DeleteDocumentOptions<'a> {
    fn default() -> Self {
        DeleteDocumentOptions {
            if_match: None,
            partition_key: PartitionKey::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReplaceDocumentOptions<'a> {
    pub indexing_directive: Option<IndexingDirective>,
    pub if_match: Option<&'a str>,
    pub partition_key: PartitionKey<'a>,
}

impl<'a> ::std::default::Default for ReplaceDocumentOptions<'a> {
    fn default() -> Self {
        ReplaceDocumentOptions {
            indexing_directive: None,
            if_match: None,
            partition_key: PartitionKey::new(),
        }
    }
}
