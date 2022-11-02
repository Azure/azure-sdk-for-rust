use async_trait::async_trait;
use fe2o3_amqp::link::DetachError;
use tokio_util::sync::CancellationToken;

use crate::{
    core::TransportSender, primitives::service_bus_retry_options::ServiceBusRetryOptions,
    CreateMessageBatchOptions, ServiceBusMessage, ServiceBusMessageBatch,
};

use super::LINK_IDENTIFIER;

pub(crate) struct AmqpSender {
    pub identifier: u32,
    pub retry_options: ServiceBusRetryOptions,
    pub sender: fe2o3_amqp::Sender,
}

// impl AmqpSender {
//     pub(crate) fn new(sender: fe2o3_amqp::Sender) -> Self {
//         Self {
//             identifier: LINK_IDENTIFIER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
//             sender,
//         }
//     }
// }

#[async_trait]
impl TransportSender for AmqpSender {
    type Error = ();
    type CloseError = DetachError;

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
        &mut self,
        options: CreateMessageBatchOptions,
        cancellation_token: impl Into<Option<CancellationToken>> + Send,
    ) -> Result<(), Self::Error> {
        todo!()
    }

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
        messages: impl Iterator<Item = &ServiceBusMessage> + Send,
        cancellation_token: impl Into<Option<CancellationToken>> + Send,
    ) -> Result<(), Self::Error> {
        todo!()
    }

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
        message_batch: ServiceBusMessageBatch,
        cancellation_token: impl Into<Option<CancellationToken>> + Send,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    async fn schedule_messages(
        &mut self,
        messages: impl Iterator<Item = &ServiceBusMessage> + Send,
        cancellation_token: impl Into<Option<CancellationToken>> + Send,
    ) -> Result<Vec<i64>, Self::Error> {
        todo!()
    }

    async fn cancel_scheduled_messages(
        &mut self,
        sequence_numbers: &[i64],
        cancellation_token: impl Into<Option<CancellationToken>> + Send,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    /// Closes the connection to the transport producer instance.
    ///
    /// # Arguments
    ///
    /// * `cancellation_token` - An optional [CancellationToken] instance to signal the request to
    ///   cancel the operation.
    async fn close(self) -> Result<(), Self::CloseError> {
        self.sender.close().await
    }
}
