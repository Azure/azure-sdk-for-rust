pub(crate) struct PartitionPublishingState {
    pub(crate) partition_id: String,
    pub(crate) producer_group_id: Option<i64>,
    pub(crate) owner_level: Option<i16>,
    pub(crate) last_published_sequence_number: Option<i32>,
}

impl PartitionPublishingState {
    pub fn new(partition_id: String) -> Self {
        Self {
            partition_id,
            producer_group_id: None,
            owner_level: None,
            last_published_sequence_number: None,
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.producer_group_id.is_some()
            && self.owner_level.is_some()
            && self.last_published_sequence_number.is_some()
    }
}
