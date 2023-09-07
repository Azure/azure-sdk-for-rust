//! Defines and implements `ServiceBusReceiver` and `ServiceBusReceiverOptions`


use fe2o3_amqp_types::primitives::OrderedMap;
use serde_amqp::Value;

use crate::amqp::amqp_receiver::AmqpReceiver;


use crate::primitives::service_bus_peeked_message::ServiceBusPeekedMessage;
use crate::util::IntoAzureCoreError;
use crate::{
    core::TransportReceiver, primitives::service_bus_received_message::ServiceBusReceivedMessage,
};

use crate::{primitives::sub_queue::SubQueue, ServiceBusReceiveMode};

use super::DeadLetterOptions;

#[cfg(docsrs)]
use crate::{ServiceBusClient, ServiceBusRetryOptions};

/// The set of options that can be specified when creating a [`ServiceBusReceiver`]
/// to configure its behavior.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceBusReceiverOptions {
    /// The number of messages that will be eagerly requested from Queues or Subscriptions and
    /// queued locally without regard to whether the receiver is actively receiving, intended to
    /// help maximize throughput by allowing the receiver to receive from a local cache rather than
    /// waiting on a service request.
    pub prefetch_count: u32,

    /// Specifies how messages are received. Defaults to [`ServiceBusReceiveMode::PeekLock`] mode.
    pub receive_mode: ServiceBusReceiveMode,

    /// A property used to set the [`ServiceBusReceiver`] ID to identify the client. This can be
    /// used to correlate logs and exceptions. If `None` or empty, a random unique value will be
    /// used.
    pub identifier: Option<String>,

    /// The subqueue to connect the receiver to. By default, the receiver will not connect to a
    /// subqueue.
    pub sub_queue: SubQueue,
}

/// The [`ServiceBusReceiver`] is responsible for receiving
/// [`ServiceBusReceivedMessage`] and settling messages from Queues and Subscriptions.
/// It is constructed by calling [`ServiceBusClient::create_receiver_for_queue`] or
/// [`ServiceBusClient::create_receiver_for_subscription`].
#[derive(Debug)]
pub struct ServiceBusReceiver {
    pub(crate) inner: AmqpReceiver,
}

impl ServiceBusReceiver {
    /// The entity path that the receiver is connected to, specific to the Service Bus
    /// namespace that contains it.
    pub fn entity_path(&self) -> &str {
        self.inner.entity_path()
    }

    /// The identifier of the receiver.
    pub fn identifier(&self) -> &str {
        self.inner.identifier()
    }

    /// The number of messages that will be eagerly requested from Queues or Subscriptions and
    /// queued locally without regard to whether the receiver is actively receiving, intended to
    /// help maximize throughput by allowing the receiver to receive from a local cache rather than
    /// waiting on a service request.
    pub fn prefetch_count(&self) -> u32 {
        self.inner.prefetch_count()
    }

    /// Specifies how messages are received.
    pub fn receive_mode(&self) -> ServiceBusReceiveMode {
        self.inner.receive_mode()
    }

    /// Closes the receiver and performs any cleanup required.
    pub async fn dispose(self) -> Result<(), azure_core::Error> {
        self.inner.close().await.map_err(IntoAzureCoreError::into_azure_core_error)
    }

    /// Receive a single message from the entity using the receiver's receive mode.
    ///
    /// This method will wait indefinitely until at least one message is received.
    pub async fn receive_message(
        &mut self,
    ) -> Result<ServiceBusReceivedMessage, azure_core::Error> {
        self.receive_messages(1).await.map(|mut v| {
            v.drain(..)
                .next()
                .expect("At least one message should be received.")
        })
    }

    /// Receive messages from the entity using the receiver's receive mode.
    ///
    /// This method will wait indefinitely until at least one message is received.
    pub async fn receive_messages(
        &mut self,
        max_messages: u32,
    ) -> Result<Vec<ServiceBusReceivedMessage>, azure_core::Error> {
        self.inner.receive_messages(max_messages).await.map_err(Into::into)
    }

    /// Receive a single message from the entity using the receiver's receive mode with a maximum
    /// wait time.
    ///
    /// If `max_wait_time` is `None`, a default max wait time value that is equal to
    /// [`ServiceBusRetryOptions::try_timeout`] will be used.
    pub async fn receive_message_with_max_wait_time(
        &mut self,
        max_wait_time: impl Into<Option<std::time::Duration>>,
    ) -> Result<Option<ServiceBusReceivedMessage>, azure_core::Error> {
        self.receive_messages_with_max_wait_time(1, max_wait_time)
            .await
            .map(|mut v| v.drain(..).next())
    }

    /// Receive messages from the entity using the receiver's receive mode with a maximum wait time.
    ///
    /// If `max_wait_time` is `None`, a default max wait time value that is equal to
    /// [`ServiceBusRetryOptions::try_timeout`] will be used. Please use
    /// [`ServiceBusReceiver::receive_messages`] if the user wants to wait indefinitely for at
    /// least one message.
    pub async fn receive_messages_with_max_wait_time(
        &mut self,
        max_messages: u32,
        max_wait_time: impl Into<Option<std::time::Duration>>,
    ) -> Result<Vec<ServiceBusReceivedMessage>, azure_core::Error> {
        self.inner
            .receive_messages_with_max_wait_time(max_messages, max_wait_time.into())
            .await
            .map_err(Into::into)
    }

    /// Completes a [`ServiceBusReceivedMessage`]. This will delete the message from the service.
    pub async fn complete_message(
        &mut self,
        message: impl AsRef<ServiceBusReceivedMessage>,
    ) -> Result<(), azure_core::Error> {
        self.inner.complete(message.as_ref(), None).await.map_err(Into::into)
    }

    /// Abandons a [`ServiceBusReceivedMessage`]. This will make the message available again for
    /// immediate processing as the lock on the message held by the receiver will be released.
    pub async fn abandon_message(
        &mut self,
        message: impl AsRef<ServiceBusReceivedMessage>,
        properties_to_modify: Option<OrderedMap<String, Value>>,
    ) -> Result<(), azure_core::Error> {
        self.inner
            .abandon(message.as_ref(), properties_to_modify, None)
            .await
            .map_err(Into::into)
    }

    /// Moves a message to the dead-letter subqueue.
    pub async fn dead_letter_message(
        &mut self,
        message: impl AsRef<ServiceBusReceivedMessage>,
        options: DeadLetterOptions,
    ) -> Result<(), azure_core::Error> {
        self.inner
            .dead_letter(
                message.as_ref(),
                options.dead_letter_reason,
                options.dead_letter_error_description,
                options.properties_to_modify,
                None,
            )
            .await
            .map_err(Into::into)
    }

    /// Indicates that the receiver wants to defer the processing for the message.
    ///
    /// In order to receive this message again in the future, you will need to save the
    /// [`ServiceBusReceivedMessage::sequence_number`] and receive it using
    /// [`receive_deferred_message(seq_num)`]. Deferring messages does not impact message's
    /// expiration, meaning that deferred messages can still expire. This operation can only be
    /// performed on messages that were received by this receiver.
    pub async fn defer_message(
        &mut self,
        message: impl AsRef<ServiceBusReceivedMessage>,
        properties_to_modify: Option<OrderedMap<String, Value>>,
    ) -> Result<(), azure_core::Error> {
        self.inner
            .defer(message.as_ref(), properties_to_modify, None)
            .await
            .map_err(Into::into)
    }

    /// Fetches the next active [`ServiceBusPeekedMessage`] without changing the state of the
    /// receiver or the message source.
    ///
    /// The first call to [`Self::peek_message`] fetches the first active message
    /// for this receiver. Each subsequent call fetches the subsequent message in the entity. Unlike
    /// a received message, a peeked message will not have a lock token associated with it, and
    /// hence it cannot be Completed/Abandoned/Deferred/Deadlettered/Renewed. Also, unlike
    /// [`Self::receive_message`], this method will fetch even Deferred messages
    /// (but not Deadlettered message).
    pub async fn peek_message(
        &mut self,
        from_sequence_number: Option<i64>,
    ) -> Result<Option<ServiceBusPeekedMessage>, azure_core::Error> {
        self.peek_messages(1, from_sequence_number)
            .await
            .map(|mut v| v.drain(..).next())
    }

    /// Fetches a list of active messages without changing the state of the receiver or the message
    /// source.
    ///
    /// Unlike a received message, a peeked message will not have a lock token associated with it,
    /// and hence it cannot be Completed/Abandoned/Deferred/Deadlettered/Renewed. Also, unlike
    /// [`ServiceBusReceiver::receive_message`], this method will fetch even Deferred messages
    /// (but not Deadlettered message).
    pub async fn peek_messages(
        &mut self,
        max_messages: u32, // FIXME: stop user from putting a negative number here?
        from_sequence_number: Option<i64>,
    ) -> Result<Vec<ServiceBusPeekedMessage>, azure_core::Error> {
        self.inner
            .peek_messages(from_sequence_number, max_messages as i32)
            .await
            .map_err(Into::into)
    }

    /// Receives a deferred message identified by `sequence_number`. An error is returned if the
    /// message is not deferred.
    pub async fn receive_deferred_message(
        &mut self,
        sequence_number: i64,
    ) -> Result<Option<ServiceBusReceivedMessage>, azure_core::Error> {
        self.receive_deferred_messages(std::iter::once(sequence_number))
            .await
            .map(|mut v| v.drain(..).next())
    }

    /// Receives a list of deferred messages identified by `sequence_numbers`. An error is returned
    /// if any of the messages are not deferred.
    pub async fn receive_deferred_messages<Seq>(
        &mut self,
        sequence_numbers: Seq,
    ) -> Result<Vec<ServiceBusReceivedMessage>, azure_core::Error>
    where
        Seq: IntoIterator<Item = i64> + Send,
        Seq::IntoIter: Send,
    {
        self.inner
            .receive_deferred_messages(sequence_numbers.into_iter(), None)
            .await
            .map_err(Into::into)
    }

    /// Renews the lock on the specified message. The lock will be renewed based on the setting
    /// specified on the entity.
    pub async fn renew_message_lock(
        &mut self,
        message: &mut ServiceBusReceivedMessage,
    ) -> Result<(), azure_core::Error> {
        let lock_tokens = vec![message.lock_token().clone()];
        let mut expirations = self.inner.renew_message_lock(lock_tokens).await?;
        if let Some(expiration) = expirations.drain(..).next() {
            message.set_locked_until(expiration);
        }
        // TODO: what if the iterator is empty?
        Ok(())
    }
}
