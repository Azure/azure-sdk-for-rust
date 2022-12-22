//! Implements `ServiceBusSender` and `ServiceBusSenderOptions`

use time::OffsetDateTime;

use crate::{
    core::TransportSender, CreateMessageBatchOptions, ServiceBusMessage, ServiceBusMessageBatch,
};

// Conditional import for docs.rs
#[cfg(docsrs)]
use crate::ServiceBusClient;

/// The set of options that can be specified when creating a [`ServiceBusSender`]
/// to configure its behavior.
#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceBusSenderOptions {
    /// A property used to set the [`ServiceBusSender`] ID to identify the client. This can be used
    /// to correlate logs and exceptions. If `None` or empty, a random unique value will be used.
    pub identifier: Option<String>,
}

/// A client responsible for sending [`ServiceBusMessage`] to a specific Service Bus entity (Queue
/// or Topic). It can be used for both session and non-session entities. It is constructed by
/// calling [`ServiceBusClient::create_sender`].
#[derive(Debug)]
pub struct ServiceBusSender<S> {
    pub(crate) inner: S,
}

impl<S> ServiceBusSender<S>
where
    S: TransportSender,
{
    /// The path of the entity that the sender is connected to, specific to the
    /// Service Bus namespace that contains it.
    pub fn entity_path(&self) -> &str {
        self.inner.entity_path()
    }

    /// Gets the ID to identify this sender. This can be used to correlate logs and exceptions.
    pub fn identifier(&self) -> &str {
        self.inner.identifier()
    }

    /// Creates a size-constraint batch to which [`ServiceBusMessage`] may be added using
    /// a [`ServiceBusMessageBatch::try_add_message`]. If a message would exceed the maximum
    /// allowable size of the batch, the batch will not allow adding the message and signal that
    /// scenario by returning an error.
    ///
    /// Because messages that would violate the size constraint cannot be added, publishing a batch
    /// will not trigger an error when attempting to send the messages to the Queue/Topic.
    pub fn create_message_batch(
        &self,
        options: CreateMessageBatchOptions,
    ) -> Result<ServiceBusMessageBatch<S::MessageBatch>, S::CreateMessageBatchError> {
        let inner = self.inner.create_message_batch(options)?;
        Ok(ServiceBusMessageBatch { inner })
    }

    /// Sends a single [`ServiceBusMessage`] to the Queue/Topic.
    pub async fn send_message(
        &mut self,
        message: impl Into<ServiceBusMessage>,
    ) -> Result<(), S::SendError> {
        let iter = std::iter::once(message.into());
        self.send_messages(iter).await
    }

    /// Sends a set of [`ServiceBusMessage`] to the Queue/Topic.
    pub async fn send_messages<M, I>(&mut self, messages: M) -> Result<(), S::SendError>
    where
        M: IntoIterator<Item = I>,
        M::IntoIter: ExactSizeIterator + Send,
        I: Into<ServiceBusMessage>,
    {
        let messages = messages.into_iter().map(|m| m.into());
        self.inner.send(messages).await
    }

    /// Sends a [`ServiceBusMessageBatch`] to the Queue/Topic.
    pub async fn send_message_batch(
        &mut self,
        batch: ServiceBusMessageBatch<S::MessageBatch>,
    ) -> Result<(), S::SendError> {
        self.inner.send_batch(batch.inner).await
    }

    /// Schedules a single [`ServiceBusMessage`] to appear on the Queue/Topic at a later time.
    pub async fn schedule_message(
        &mut self,
        message: impl Into<ServiceBusMessage>,
        enqueue_time: OffsetDateTime,
    ) -> Result<i64, S::ScheduleError> {
        let messages = std::iter::once(message.into());
        let seq_nums = self.schedule_messages(messages, enqueue_time).await?;
        // PANIC: there should be exactly one sequence number returned
        assert_eq!(seq_nums.len(), 1);
        Ok(seq_nums[0])
    }

    /// Schedules a set of [`ServiceBusMessage`] to appear on the Queue/Topic at a later time.
    pub async fn schedule_messages<M, I>(
        &mut self,
        messages: M,
        enqueue_time: OffsetDateTime,
    ) -> Result<Vec<i64>, S::ScheduleError>
    where
        M: IntoIterator<Item = I>,
        M::IntoIter: ExactSizeIterator + Send,
        I: Into<ServiceBusMessage>,
    {
        let iter = messages.into_iter();
        if iter.len() == 0 {
            return Ok(vec![]);
        }
        let messages = iter.map(|m| {
            let mut m = m.into();
            m.set_scheduled_enqueue_time(enqueue_time);
            m
        });
        self.inner.schedule_messages(messages).await
    }

    /// Cancels a single scheduled [`ServiceBusMessage`] that was previously scheduled with
    pub async fn cancel_scheduled_message(
        &mut self,
        sequence_number: i64,
    ) -> Result<(), S::ScheduleError> {
        // The request will always encode the sequence numbers as a Vec, so it doesn't hurt to
        // allocate a Vec here.
        self.cancel_scheduled_messages(std::iter::once(sequence_number))
            .await
    }

    /// Cancels a set of scheduled [`ServiceBusMessage`] that were previously scheduled with
    pub async fn cancel_scheduled_messages<I>(
        &mut self,
        sequence_numbers: I,
    ) -> Result<(), S::ScheduleError>
    where
        I: IntoIterator<Item = i64>,
        I::IntoIter: ExactSizeIterator + Send,
    {
        let iter: Vec<i64> = sequence_numbers.into_iter().collect();
        if iter.is_empty() {
            return Ok(());
        }

        self.inner.cancel_scheduled_messages(iter).await
    }

    /// Closes the sender and performs any cleanup required.
    pub async fn dispose(self) -> Result<(), S::CloseError> {
        self.inner.close().await
    }
}
