#[derive(Debug, Clone, PartialEq)]
pub struct ListBlobOptions {
    pub max_results: u32,
    pub include_snapshots: bool,
    pub include_metadata: bool,
    pub include_uncommittedblobs: bool,
    pub include_copy: bool,
    pub next_marker: Option<String>,
}

pub const LIST_BLOB_OPTIONS_DEFAULT: ListBlobOptions = ListBlobOptions {
    max_results: 5000,
    include_snapshots: false,
    include_metadata: false,
    include_uncommittedblobs: false,
    include_copy: false,
    next_marker: None,
};

impl ListBlobOptions {
    pub fn new(max_results: u32,
               include_snapshots: bool,
               include_metadata: bool,
               include_uncommittedblobs: bool,
               include_copy: bool,
               next_marker: Option<&str>)
               -> ListBlobOptions {

        let nm = match next_marker {
            Some(s) => Some(s.to_owned()),
            None => None,
        };

        ListBlobOptions {
            max_results: max_results,
            include_snapshots: include_snapshots,
            include_metadata: include_metadata,
            include_uncommittedblobs: include_uncommittedblobs,
            include_copy: include_copy,
            next_marker: nm,
        }
    }
}
