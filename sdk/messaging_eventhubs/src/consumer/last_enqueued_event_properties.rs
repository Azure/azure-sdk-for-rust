use time::OffsetDateTime;

/// A set of information about the enqueued state of a partition, as observed by the consumer.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct LastEnqueuedEventProperties {
    pub(crate) sequence_number: Option<i64>,
    pub(crate) offset: Option<i64>,
    pub(crate) enqueued_time: Option<OffsetDateTime>,
    pub(crate) last_received_time: Option<OffsetDateTime>,
}
