//! Receive mode for Service Bus receiver.

/// The mode in which to receive messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ServiceBusReceiveMode {
    /// Once a message is received in this mode, the receiver has a lock on the message for a
    /// particular duration. If the message is not settled by this time, it lands back on Service
    /// Bus to be fetched by the next receive operation.
    ///
    /// This is the default value for [`ServiceBusReceiveMode`], and should be used for
    /// guaranteed delivery.
    PeekLock,

    /// ReceiveAndDelete will delete the message from Service Bus as soon as the message is
    /// delivered.
    ReceiveAndDelete,
}

impl Default for ServiceBusReceiveMode {
    fn default() -> Self {
        Self::PeekLock
    }
}
