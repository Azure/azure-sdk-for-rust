use azure::cosmos::ConsistencyLevel;
use azure::cosmos::partition_key::PartitionKey;

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
