//! Implements the Service Bus Sender.

pub mod service_bus_message_batch;
pub mod service_bus_sender;

/// The minimum allowable size, in bytes, for a batch to be sent.
pub(crate) const MINIMUM_BATCH_SIZE_LIMIT: u64 = 24;
