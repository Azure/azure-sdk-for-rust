use azure::core::incompletevector::{ContinuationToken, NO_CONTINUATION_TOKEN};

#[derive(Clone)]
pub struct ListBlobOptions<'a> {
    pub max_results: u32,
    pub include_snapshots: bool,
    pub include_metadata: bool,
    pub include_uncommittedblobs: bool,
    pub include_copy: bool,
    pub next_marker: &'a ContinuationToken,
    pub prefix: Option<String>,
    pub timeout: Option<u64>,
}

pub const LIST_BLOB_OPTIONS_DEFAULT: ListBlobOptions = ListBlobOptions {
    max_results: 5000,
    include_snapshots: false,
    include_metadata: false,
    include_uncommittedblobs: false,
    include_copy: false,
    next_marker: NO_CONTINUATION_TOKEN,
    prefix: None,
    timeout: None,
};
