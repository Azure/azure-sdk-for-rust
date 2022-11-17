use async_trait::async_trait;

use crate::{CreateMessageBatchOptions, ServiceBusMessage};

/// Provides an abstraction for generalizing an Service Bus entity Producer so that a dedicated instance may provide operations
/// for a specific transport, such as AMQP or JMS.  It is intended that the public <see cref="ServiceBusSender" /> employ
/// a transport producer via containment and delegate operations to it rather than understanding protocol-specific details
/// for different transports.
#[async_trait]
pub trait TransportSender {
    type Error: std::error::Error + Send;
    type SendError: std::error::Error + Send;
    type CloseError: std::error::Error + Send;
    type MessageBatch: Send;
    type CreateMessageBatchError: std::error::Error + Send;

    /// Creates a size-constraint batch to which <see cref="ServiceBusMessage" /> may be added using
    /// a try-based pattern.  If a message would exceed the maximum allowable size of the batch, the
    /// batch will not allow adding the message and signal that scenario using its return value.
    ///
    /// Because messages that would violate the size constraint cannot be added, publishing a batch
    /// will not trigger an exception when attempting to send the message to the Queue/Topic.
    ///
    /// # Arguments
    ///
    /// * `options` - The set of options to consider when creating this batch.
    /// * `cancellation_token` - An optional <see cref="CancellationToken"/> instance to signal the
    ///   request to cancel the operation.
    ///
    /// # Returns
    ///
    /// An [ServiceBusMessageBatch] with the requested `options`
    async fn create_message_batch(
        &self,
        options: CreateMessageBatchOptions,
    ) -> Result<Self::MessageBatch, Self::CreateMessageBatchError>;

    /// Sends a list of messages to the associated Service Bus entity using a batched approach. If
    /// the size of the messages exceed the maximum size of a single batch, an exception will be
    /// triggered and the send will fail. In order to ensure that the messages being sent will fit
    /// in a batch, use <see cref="SendBatchAsync"/> instead.
    ///
    /// # Arguments
    ///
    /// * `messages` - The list of messages to send.
    /// * `cancellationToken` - An optional <see cref="CancellationToken"/> instance to signal the
    ///   request to cancel the operation.
    async fn send(
        &mut self,
        messages: impl Iterator<Item = ServiceBusMessage> + ExactSizeIterator + Send,
    ) -> Result<(), Self::SendError>;

    /// Sends a <see cref="ServiceBusMessageBatch"/> to the associated Queue/Topic.
    ///
    /// # Arguments
    ///
    /// * `message_batch` - The set of messages to send.
    /// * `cancellation_token` - An optional <see cref="CancellationToken"/> instance to signal the
    ///   request to cancel the operation.
    ///
    /// # Returns
    ///
    /// A task to be resolved on when the operation has completed.
    async fn send_batch(
        &mut self,
        message_batch: Self::MessageBatch,
    ) -> Result<(), Self::SendError>;

    async fn schedule_messages(
        &mut self,
        messages: impl Iterator<Item = ServiceBusMessage> + Send,
    ) -> Result<Vec<i64>, Self::SendError>;

    async fn cancel_scheduled_messages(
        &mut self,
        sequence_numbers: Vec<i64>,
    ) -> Result<(), Self::SendError>;

    /// Closes the connection to the transport producer instance.
    ///
    /// # Arguments
    ///
    /// * `cancellation_token` - An optional [CancellationToken] instance to signal the request to
    ///   cancel the operation.
    async fn close(self) -> Result<(), Self::CloseError>;
}
