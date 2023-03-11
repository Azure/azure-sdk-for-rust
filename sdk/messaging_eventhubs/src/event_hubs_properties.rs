use time::OffsetDateTime;

/// A set of information for an Event Hub.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct EventHubProperties {
    pub(crate) name: String,
    pub(crate) created_on: OffsetDateTime,
    pub(crate) partition_ids: Vec<String>,
}

impl EventHubProperties {
    /// The name of the Event Hub, specific to the namespace that contains it.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The date and time, in UTC, at which the Event Hub was created.
    pub fn created_on(&self) -> &OffsetDateTime {
        &self.created_on
    }

    /// The set of unique identifiers for each partition in the Event Hub.
    pub fn partition_ids(&self) -> &[String] {
        &self.partition_ids
    }
}
