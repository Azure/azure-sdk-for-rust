//! Implements the Service Bus Sender.

pub mod service_bus_message_batch;
pub mod service_bus_sender;

/// <summary>The minimum allowable size, in bytes, for a batch to be sent.</summary>
pub(crate) const MINIMUM_BATCH_SIZE_LIMIT: u64 = 24;
