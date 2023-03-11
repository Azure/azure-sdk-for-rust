use time::OffsetDateTime;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct EventHubProperties {
    pub(crate) name: String,
    pub(crate) created_on: OffsetDateTime,
    pub(crate) partition_ids: Vec<String>,
}

impl EventHubProperties {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn created_on(&self) -> &OffsetDateTime {
        &self.created_on
    }

    pub fn partition_ids(&self) -> &[String] {
        &self.partition_ids
    }
}
