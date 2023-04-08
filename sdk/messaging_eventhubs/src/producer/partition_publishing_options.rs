/// The set of options that can be specified for an <see cref="EventHubProducerClient" />
/// to influence its behavior when publishing directly to an Event Hub partition.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct PartitionPublishingOptions {
    /// The identifier of the producer group that this producer is associated with when publishing
    /// to the associated partition. Events will be published in the context of this group.
    ///
    /// # value
    ///
    /// The identifier of the producer group to associate with the partition; if `None`, the Event
    /// Hubs service will control the value.</value>
    ///
    /// The producer group is only recognized and relevant when certain features of the producer are
    /// enabled.  For example, it is used by idempotent publishing.
    pub producer_group_id: Option<i64>,

    /// The owner level indicates that a publishing is intended to be performed exclusively for
    /// events in the requested partition in the context of the associated producer group.  To do
    /// so, publishing will attempt to assert ownership over the partition; in the case where more
    /// than one publisher in the producer group attempts to assert ownership for the same
    /// partition, the one having a larger <see cref="OwnerLevel"/> value will "win".
    ///
    /// When an owner level is specified, other exclusive publishers which have a lower owner level
    /// within the context of the same producer group will either not be allowed to be created or,
    /// if they already exist, will encounter an exception during the next attempted operation.
    /// Should there be multiple producers in the producer group with the same owner level, each of
    /// them will be able to publish to the partition.
    ///
    /// Producers with no owner level or which belong to a different producer group are permitted to
    /// publish to the associated partition without restriction or awareness of other exclusive
    /// producers.
    ///
    /// # Value
    ///
    /// The relative priority to associate with an exclusive publisher; if `None`, the Event Hubs
    /// service will control the value.
    ///
    /// The owner level is only recognized and relevant when certain features of the producer are
    /// enabled.  For example, it is used by idempotent publishing.
    pub owner_level: Option<i16>,

    /// The starting number that should be used for the automatic sequencing of events for the
    /// associated partition, when published by this producer.
    ///
    /// # Value
    ///
    /// The starting sequence number to associate with the partition; if `None`, the Event Hubs
    /// service will control the value.
    ///
    /// The sequence number will be in the range of 0 - [`i32::MAX`]
    /// (inclusive) and will increase as events are published. When more than [`i32::MAX`]
    /// events have been published, the sequence number will roll over to 0.
    ///
    /// The starting sequence number is only recognized and relevant when certain features
    /// of the producer are enabled. For example, it is used by idempotent publishing.
    pub starting_sequence_number: Option<i32>,
}
