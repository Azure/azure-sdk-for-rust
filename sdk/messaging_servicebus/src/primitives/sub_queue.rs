//! Defines sub queues that can be received from.

/// Sub queues that can be received from
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SubQueue {
    /// No sub queue, the queue itself will be used
    None = 0,

    /// The dead letter queue, which contains messages that have been dead lettered
    DeadLetter = 1,

    /// The transfer dead-letter subqueue contains messages that have been dead-lettered when
    /// the following conditions apply:
    ///
    /// - A message passes through more than four queues or topics that are chained together.
    /// - The destination queue or topic is disabled or deleted.
    /// - The destination queue or topic exceeds the maximum entity size.
    ///
    /// Please see [dead-lettering-in-forwardto-or-sendvia-scenarios](https://docs.microsoft.com/en-us/azure/service-bus-messaging/service-bus-dead-letter-queues#dead-lettering-in-forwardto-or-sendvia-scenarios)
    TransferDeadLetter = 2,
}

impl Default for SubQueue {
    fn default() -> Self {
        Self::None
    }
}
