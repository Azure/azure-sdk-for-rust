/// The partitioning strategy to use when publishing events to Event Hubs.
#[derive(Debug, Clone)]
pub enum Partition {
    /// Allows a hashing key to be provided for the batch of events, which instructs Event Hubs to
    /// map the key to an automatically-assigned partition.
    ///
    /// The selection of a partition is stable for a given partition key.  Should any other events
    /// be published using the same exact partition key, Event Hubs will assign the same partition
    /// to them.
    ///
    /// The partition key should be specified when there is a need to group events together, but the
    /// partition to which they are assigned is unimportant.  If ensuring that a batch of events is
    /// assigned a specific partition, it is recommended that the [`Partition::Id`] be
    /// assigned instead.
    Key(String),

    /// Events will be published to this specific partition.
    Id(String),
}

/// The set of options that can be specified when sending an event to an Event Hubs service.
#[derive(Debug, Clone, Default)]
pub struct SendEventOptions {
    /// If specified, events be published according to this partitioning strategy.
    pub partition: Option<Partition>,
}

impl SendEventOptions {
    /// Create a new instance of [`SendEventOptions`] with default values
    ///
    /// # Default Value
    ///
    /// - `partition`: `None`
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `partition` field to `Some(Partition::Key(key))`
    pub fn with_partition_key(mut self, partition_key: impl Into<String>) -> Self {
        self.partition = Some(Partition::Key(partition_key.into()));
        self
    }

    /// Set the `partition` field to `Some(Partition::Id(id))`
    pub fn with_partition_id(mut self, partition_id: impl Into<String>) -> Self {
        self.partition = Some(Partition::Id(partition_id.into()));
        self
    }

    /// Get a reference to the partition key, if specified.
    pub fn partition_key(&self) -> Option<&str> {
        match &self.partition {
            Some(Partition::Key(key)) => Some(key),
            _ => None,
        }
    }

    /// Get a reference to the partition id, if specified.
    pub fn partition_id(&self) -> Option<&str> {
        match &self.partition {
            Some(Partition::Id(id)) => Some(id),
            _ => None,
        }
    }

    /// Consumes the options, returning the partition key, if specified.
    pub fn into_partition_key(self) -> Option<String> {
        match self.partition {
            Some(Partition::Key(key)) => Some(key),
            _ => None,
        }
    }

    /// Consumes the options, returning the partition id, if specified.
    pub fn into_partition_id(self) -> Option<String> {
        match self.partition {
            Some(Partition::Id(id)) => Some(id),
            _ => None,
        }
    }
}
