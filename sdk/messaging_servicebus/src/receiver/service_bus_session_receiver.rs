//! Defines and implements `ServiceBusSessionReceiver` and `ServiceBusSessionReceiverOptions`

use fe2o3_amqp::link::DetachError;
use fe2o3_amqp_types::primitives::OrderedMap;
use serde_amqp::Value;

use crate::{
    amqp::{
        amqp_session_receiver::AmqpSessionReceiver,
        error::{AmqpDispositionError, AmqpRecvError, AmqpRequestResponseError},
    },
    core::{TransportReceiver, TransportSessionReceiver},
    primitives::{
        error::RetryError, service_bus_peeked_message::ServiceBusPeekedMessage,
        service_bus_received_message::ServiceBusReceivedMessage,
    },
    ServiceBusReceiveMode, ServiceBusReceiverOptions,
};

use super::DeadLetterOptions;

#[cfg(docsrs)]
use crate::{ServiceBusClient, ServiceBusRetryOptions};

/// Options for configuring a `ServiceBusSessionReceiver`.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceBusSessionReceiverOptions {
    /// The number of messages that will be eagerly requested from Queues or Subscriptions and
    /// queued locally without regard to whether the receiver is actively receiving, intended to
    /// help maximize throughput by allowing the receiver to receive from a local cache rather than
    /// waiting on a service request.
    pub prefetch_count: u32,

    /// Specifies how messages are received. Defaults to PeekLock mode.
    pub receive_mode: ServiceBusReceiveMode,

    /// A property used to set the [`ServiceBusSessionReceiver`] ID to identify the client. This can
    /// be used to correlate logs and exceptions. If `None` or empty, a random unique value will be
    /// used.
    pub identifier: Option<String>,
}

impl From<ServiceBusSessionReceiverOptions> for ServiceBusReceiverOptions {
    fn from(options: ServiceBusSessionReceiverOptions) -> Self {
        ServiceBusReceiverOptions {
            receive_mode: options.receive_mode,
            sub_queue: Default::default(),
            prefetch_count: options.prefetch_count,
            identifier: options.identifier,
        }
    }
}

/// The [`ServiceBusSessionReceiver`] is responsible for receiving [`ServiceBusReceivedMessage`] and
/// settling messages from session-enabled Queues and Subscriptions. It is constructed by calling
/// [`ServiceBusClient::accept_next_session_for_queue`] or
/// [`ServiceBusClient::accept_next_session_for_subscription`].
#[derive(Debug)]
pub struct ServiceBusSessionReceiver {
    pub(crate) inner: AmqpSessionReceiver,
    pub(crate) session_id: String,
}

impl ServiceBusSessionReceiver {
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

    /// Gets the session ID of the receiver.
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Closes the receiver and performs any cleanup required.
    pub async fn dispose(self) -> Result<(), DetachError> {
        self.inner.close().await
    }

    /// Receive a single message from the entity using the receiver's receive mode.
    ///
    /// This method will wait indefinitely until at least one message is received.
    pub async fn receive_message(
        &mut self,
    ) -> Result<ServiceBusReceivedMessage, RetryError<AmqpRecvError>> {
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
    ) -> Result<Vec<ServiceBusReceivedMessage>, RetryError<AmqpRecvError>> {
        self.inner.receive_messages(max_messages).await
    }

    /// Receive a single message from the entity using the receiver's receive mode with a maximum
    /// wait time.
    ///
    /// If `max_wait_time` is `None`, a default max wait time value that is equal to
    /// [`ServiceBusRetryOptions::try_timeout`] will be used.
    pub async fn receive_message_with_max_wait_time(
        &mut self,
        max_wait_time: impl Into<Option<std::time::Duration>>,
    ) -> Result<Option<ServiceBusReceivedMessage>, RetryError<AmqpRecvError>> {
        self.receive_messages_with_max_wait_time(1, max_wait_time)
            .await
            .map(|mut v| v.drain(..).next())
    }

    /// Receive messages from the entity using the receiver's receive mode with a maximum wait time.
    ///
    /// If `max_wait_time` is `None`, a default max wait time value that is equal to
    /// [`ServiceBusRetryOptions::try_timeout`] will be used. Please use
    /// [`Self::receive_messages`] if the user wants to wait indefinitely for at least one
    /// message.
    pub async fn receive_messages_with_max_wait_time(
        &mut self,
        max_messages: u32,
        max_wait_time: impl Into<Option<std::time::Duration>>,
    ) -> Result<Vec<ServiceBusReceivedMessage>, RetryError<AmqpRecvError>> {
        self.inner
            .receive_messages_with_max_wait_time(max_messages, max_wait_time.into())
            .await
    }

    /// Completes a [`ServiceBusReceivedMessage`]. This will delete the message from the service.
    pub async fn complete_message(
        &mut self,
        message: impl AsRef<ServiceBusReceivedMessage>,
    ) -> Result<(), RetryError<AmqpDispositionError>> {
        self.inner
            .complete(message.as_ref(), Some(&self.session_id))
            .await
    }

    /// Abandons a [`ServiceBusReceivedMessage`]. This will make the message available again for
    /// immediate processing as the lock on the message held by the receiver will be released.
    pub async fn abandon_message(
        &mut self,
        message: impl AsRef<ServiceBusReceivedMessage>,
        properties_to_modify: Option<OrderedMap<String, Value>>,
    ) -> Result<(), RetryError<AmqpDispositionError>> {
        self.inner
            .abandon(
                message.as_ref(),
                properties_to_modify,
                Some(&self.session_id),
            )
            .await
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
    ) -> Result<(), RetryError<AmqpDispositionError>> {
        self.inner
            .defer(
                message.as_ref(),
                properties_to_modify,
                Some(&self.session_id),
            )
            .await
    }

    /// Moves a message to the dead-letter subqueue.
    pub async fn dead_letter_message(
        &mut self,
        message: impl AsRef<ServiceBusReceivedMessage>,
        options: DeadLetterOptions,
    ) -> Result<(), RetryError<AmqpDispositionError>> {
        self.inner
            .dead_letter(
                message.as_ref(),
                options.dead_letter_reason,
                options.dead_letter_error_description,
                options.properties_to_modify,
                Some(&self.session_id),
            )
            .await
    }

    /// Fetches the next active [`ServiceBusPeekedMessage`] without changing the state of the
    /// receiver or the message source.
    ///
    /// The first call to [`Self::peek_message`] fetches the first active message for this
    /// receiver. Each subsequent call fetches the subsequent message in the entity. Unlike a
    /// received message, a peeked message will not have a lock token associated with it, and hence
    /// it cannot be Completed/Abandoned/Deferred/Deadlettered/Renewed. Also, unlike
    /// [`Self::receive_message`], this method will fetch even Deferred messages (but not
    /// Deadlettered message).
    pub async fn peek_message(
        &mut self,
        from_sequence_number: Option<i64>,
    ) -> Result<Option<ServiceBusPeekedMessage>, RetryError<AmqpRequestResponseError>> {
        self.peek_messages(1, from_sequence_number)
            .await
            .map(|mut v| v.drain(..).next())
    }

    /// Fetches a list of active messages without changing the state of the receiver or the message
    /// source.
    ///
    /// Unlike a received message, a peeked message will not have a lock token associated with it,
    /// and hence it cannot be Completed/Abandoned/Deferred/Deadlettered/Renewed. Also, unlike
    /// [`Self::receive_message`], this method will fetch even Deferred messages (but not
    /// Deadlettered message).
    pub async fn peek_messages(
        &mut self,
        max_messages: u32, // FIXME: stop user from putting a negative number here?
        from_sequence_number: Option<i64>,
    ) -> Result<Vec<ServiceBusPeekedMessage>, RetryError<AmqpRequestResponseError>> {
        self.inner
            .peek_session_messages(from_sequence_number, max_messages as i32, &self.session_id)
            .await
    }

    /// Receives a deferred message identified by `sequence_number`. An error is returned if the
    /// message is not deferred.
    pub async fn receive_deferred_message(
        &mut self,
        sequence_number: i64,
    ) -> Result<Option<ServiceBusReceivedMessage>, RetryError<AmqpRequestResponseError>> {
        self.receive_deferred_messages(std::iter::once(sequence_number))
            .await
            .map(|mut v| v.drain(..).next())
    }

    /// Receives a list of deferred messages identified by `sequence_numbers`. An error is returned
    /// if any of the messages are not deferred.
    pub async fn receive_deferred_messages(
        &mut self,
        sequence_numbers: impl Iterator<Item = i64> + Send,
    ) -> Result<Vec<ServiceBusReceivedMessage>, RetryError<AmqpRequestResponseError>> {
        self.inner
            .receive_deferred_messages(sequence_numbers, Some(&self.session_id))
            .await
    }

    /// Renews the lock on the specified message. The lock will be renewed based on the setting
    /// specified on the entity.
    pub async fn renew_message_lock(
        &mut self,
        message: &mut ServiceBusReceivedMessage,
    ) -> Result<(), RetryError<AmqpRequestResponseError>> {
        let lock_tokens = vec![message.lock_token().clone()];
        let mut expirations = self.inner.renew_message_lock(lock_tokens).await?;
        if let Some(expiration) = expirations.drain(..).next() {
            message.set_locked_until(expiration);
        }
        // TODO: what if the iterator is empty?
        Ok(())
    }

    /// Gets the session state.
    pub async fn session_state(&mut self) -> Result<Vec<u8>, RetryError<AmqpRequestResponseError>> {
        self.inner.session_state(&self.session_id).await
    }

    /// Set a custom state on the session which can be later retrieved using
    /// [`Self::session_state`]
    pub async fn set_session_state(
        &mut self,
        session_state: Vec<u8>,
    ) -> Result<(), RetryError<AmqpRequestResponseError>> {
        self.inner
            .set_session_state(&self.session_id, session_state)
            .await
    }

    /// Renews the lock on the session specified by the [`Self::session_id`]. The lock will be
    /// renewed based on the setting specified on the entity.
    pub async fn renew_session_lock(&mut self) -> Result<(), RetryError<AmqpRequestResponseError>> {
        let locked_until = self.inner.renew_session_lock(&self.session_id).await?;
        self.inner.set_session_locked_until(locked_until);
        Ok(())
    }
}
