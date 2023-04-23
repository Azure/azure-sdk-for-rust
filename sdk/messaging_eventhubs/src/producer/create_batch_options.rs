/// A set of options that can be specified when creating a batch of events.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CreateBatchOptions {
    /// The maximum size of the batch in bytes.  If not specified, the maximum size will be
    /// determined by the Event Hub.
    pub max_size_in_bytes: Option<u64>,

    /// The partition key to associate with the batch of events. See
    /// [`crate::producer::Partition::Key] for more details.
    pub partition_key: Option<String>,
}

impl CreateBatchOptions {
    /// Create a new instance of [`CreateBatchOptions`] with default values
    ///
    /// # Default Value
    ///
    /// - `max_size_in_bytes`: `None`
    /// - `partition_key`: `None`
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `max_size_in_bytes` field
    pub fn with_max_size_in_bytes(mut self, max_size_in_bytes: u64) -> Self {
        self.max_size_in_bytes = Some(max_size_in_bytes);
        self
    }

    /// Set the `partition_key` field
    pub fn with_partition_key(mut self, partition_key: impl Into<String>) -> Self {
        self.partition_key = Some(partition_key.into());
        self
    }
}
