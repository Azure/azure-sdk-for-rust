use async_trait::async_trait;

use crate::{sealed::Sealed, CreateMessageBatchOptions, ServiceBusMessage};

use super::TransportMessageBatch;

// Conditional import for docs.rs
#[cfg(docsrs)]
use crate::ServiceBusSender;

/// Provides an abstraction for generalizing an Service Bus entity Producer so that a dedicated
/// instance may provide operations for a specific transport, such as AMQP or JMS.  It is intended
/// that the public [`ServiceBusSender`] employ a transport producer via containment and delegate
/// operations to it rather than understanding protocol-specific details for different transports.
#[async_trait]
pub(crate) trait TransportSender: Sealed {
    /// Error with sending a message
    type SendError: std::error::Error + Send;

    /// Error with scheduling a message
    type ScheduleError: std::error::Error + Send;

    /// Error with closing a sender
    type CloseError: std::error::Error + Send;

    /// Error with creating a message batch
    type CreateMessageBatchError: std::error::Error + Send;

    /// The message batch type
    type MessageBatch: TransportMessageBatch + Send;

    /// Get the entity path
    fn entity_path(&self) -> &str;

    /// Get the identifier
    fn identifier(&self) -> &str;

    /// Creates a size-constraint batch to which [`ServiceBusMessage`] may be added using
    /// a try-based pattern.  If a message would exceed the maximum allowable size of the batch, the
    /// batch will not allow adding the message and signal that scenario using its return value.
    ///
    /// Because messages that would violate the size constraint cannot be added, publishing a batch
    /// will not trigger an exception when attempting to send the message to the Queue/Topic.
    fn create_message_batch(
        &self,
        options: CreateMessageBatchOptions,
    ) -> Result<Self::MessageBatch, Self::CreateMessageBatchError>;

    /// Sends a list of messages to the associated Service Bus entity using a batched approach. If
    /// the size of the messages exceed the maximum size of a single batch, an exception will be
    /// triggered and the send will fail. In order to ensure that the messages being sent will fit
    /// in a batch, use [`TransportSender::send_batch`] instead.
    async fn send(
        &mut self,
        messages: impl Iterator<Item = ServiceBusMessage> + ExactSizeIterator + Send,
    ) -> Result<(), Self::SendError>;

    /// Sends a [`Self::MessageBatch`] to the associated Queue/Topic.
    async fn send_batch(
        &mut self,
        message_batch: Self::MessageBatch,
    ) -> Result<(), Self::SendError>;

    /// Schedules a list of messages to appear on Service Bus at a later time.
    async fn schedule_messages(
        &mut self,
        messages: impl Iterator<Item = ServiceBusMessage> + Send,
    ) -> Result<Vec<i64>, Self::ScheduleError>;

    /// Cancels one or more messages that were scheduled.
    async fn cancel_scheduled_messages(
        &mut self,
        sequence_numbers: Vec<i64>,
    ) -> Result<(), Self::ScheduleError>;

    /// Closes the connection to the transport producer instance.
    async fn close(self) -> Result<(), Self::CloseError>;
}
