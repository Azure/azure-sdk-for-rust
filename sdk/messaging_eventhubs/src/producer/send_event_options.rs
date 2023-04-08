#[derive(Debug, Clone, Default)]
pub struct SendEventOptions {
    pub partition_key: Option<String>,
    pub partition_id: Option<String>,
}

impl SendEventOptions {
    pub fn new() -> Self {
        Self::default()
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
