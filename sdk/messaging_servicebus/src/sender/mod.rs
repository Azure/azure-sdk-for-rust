//! Sender for Service Bus queues and topics.

cfg_either_rustls_or_native_tls! {
    pub mod service_bus_sender;
}

pub mod service_bus_message_batch;

/// The minimum allowable size, in bytes, for a batch to be sent.
pub(crate) const MINIMUM_BATCH_SIZE_LIMIT: u64 = 24;
