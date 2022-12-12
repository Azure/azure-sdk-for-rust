// use const_format::concatcp;

// use super::amqp_constants;

pub(crate) const ENQUEUED_TIME_UTC_NAME: &str = "x-opt-enqueued-time";
pub(crate) const SCHEDULED_ENQUEUE_TIME_UTC_NAME: &str = "x-opt-scheduled-enqueue-time";
pub(crate) const SEQUENCE_NUMBER_NAME: &str = "x-opt-sequence-number";
pub(crate) const ENQUEUE_SEQUENCE_NUMBER_NAME: &str = "x-opt-enqueue-sequence-number";
pub(crate) const LOCKED_UNTIL_NAME: &str = "x-opt-locked-until";
pub(crate) const PARTITION_KEY_NAME: &str = "x-opt-partition-key";
// pub(crate) const PARTITION_ID_NAME: &str = "x-opt-partition-id";
pub(crate) const VIA_PARTITION_KEY_NAME: &str = "x-opt-via-partition-key";
pub(crate) const DEAD_LETTER_SOURCE_NAME: &str = "x-opt-deadletter-source";
pub(crate) const MESSAGE_STATE_NAME: &str = "x-opt-message-state";
// pub(crate) const TIME_SPAN_NAME: &str = concatcp!(amqp_constants::VENDOR, ":timespan");
// pub(crate) const URI_NAME: &str = concatcp!(amqp_constants::VENDOR, ":uri");
// pub(crate) const DATE_TIME_OFFSET_NAME: &str =
// concatcp!(amqp_constants::VENDOR, ":datetime-offset");

/// Property key representing dead-letter reason, when a message is received from a dead-letter subqueue of an entity.
/// This key and the associated values are stored in the [`ServiceBusReceivedMessage.application_properties`] dictionary
/// for dead lettered messages.
pub(crate) const DEAD_LETTER_REASON_HEADER: &str = "DeadLetterReason";

/// Property key representing detailed error description, when a message is received from a dead-letter subqueue of an entity.
/// This key and the associated values are stored in the [`ServiceBusReceivedMessage.application_properties`] dictionary
/// for dead lettered messages.
pub(crate) const DEAD_LETTER_ERROR_DESCRIPTION_HEADER: &str = "DeadLetterErrorDescription";
