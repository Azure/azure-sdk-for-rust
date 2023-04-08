#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CreateBatchOptions {
    pub max_size_in_bytes: Option<u64>,
    pub partition_key: Option<String>,
    pub partition_id: Option<String>,
}

impl CreateBatchOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_size_in_bytes(mut self, max_size_in_bytes: u64) -> Self {
        self.max_size_in_bytes = Some(max_size_in_bytes);
        self
    }

    pub fn with_partition_key(mut self, partition_key: impl Into<String>) -> Self {
        self.partition_key = Some(partition_key.into());
        self
    }

    pub fn with_partition_id(mut self, partition_id: impl Into<String>) -> Self {
        self.partition_id = Some(partition_id.into());
        self
    }
}
