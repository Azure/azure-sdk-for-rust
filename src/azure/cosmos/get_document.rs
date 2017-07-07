use azure::cosmos::ConsistencyLevel;

#[derive(Debug, Clone)]
pub struct GetDocumentOptions<'a> {
    pub consistency_level_override: Option<ConsistencyLevel>,
    pub session_token: Option<&'a str>,
    pub if_none_match: Option<&'a str>,
    pub partition_key: Option<Vec<&'a str>>,
}

pub const GET_DOCUMENT_OPTIONS_DEFAULT: GetDocumentOptions = GetDocumentOptions {
    consistency_level_override: None,
    session_token: None,
    if_none_match: None,
    partition_key: None,
};
