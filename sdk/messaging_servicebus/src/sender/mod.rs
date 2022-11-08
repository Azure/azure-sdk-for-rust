pub mod create_message_batch_options;
pub mod error;
pub mod service_bus_message_batch;
pub mod service_bus_sender;
pub mod service_bus_sender_options;

/// <summary>The minimum allowable size, in bytes, for a batch to be sent.</summary>
pub(crate) const MINIMUM_BATCH_SIZE_LIMIT: u64 = 24;
