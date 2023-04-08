#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CreateBatchOptions {
    pub max_size_in_bytes: Option<u64>,
    pub partition_key: Option<String>,
    pub partition_id: Option<String>,
}
