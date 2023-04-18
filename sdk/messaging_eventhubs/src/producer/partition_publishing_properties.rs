pub struct PartitionPublishingProperties {
    // pub is_idempotent_publishing_enabled: bool,
    pub producer_group_id: Option<i64>,
    pub owner_level: Option<i16>,
    pub last_published_sequence_number: Option<i32>,
}
