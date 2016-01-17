#[derive(Debug, Clone, PartialEq)]
pub struct LeaseBlobOptions {
    pub max_results: u32,
    pub include_snapshots: bool,
    pub include_metadata: bool,
    pub include_uncommittedblobs: bool,
    pub include_copy: bool,
    pub next_marker: Option<String>,
    pub prefix: Option<String>,
    pub timeout: Option<u64>,
}

pub const LEASE_BLOB_OPTIONS_DEFAULT: LeaseBlobOptions = LeaseBlobOptions {
    max_results: 5000,
    include_snapshots: false,
    include_metadata: false,
    include_uncommittedblobs: false,
    include_copy: false,
    next_marker: None,
    prefix: None,
    timeout: None,
};
