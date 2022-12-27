use async_trait::async_trait;
use fe2o3_amqp_types::primitives::{OrderedMap, Timestamp};
use serde_amqp::Value;
use std::time::Duration as StdDuration;
use time::OffsetDateTime;

use crate::{
    primitives::{
        service_bus_peeked_message::ServiceBusPeekedMessage,
        service_bus_received_message::ServiceBusReceivedMessage,
    },
    sealed::Sealed,
    ServiceBusReceiveMode,
};

// Conditional import for docs.rs
#[cfg(docsrs)]
use crate::ServiceBusReceiver;

/// Trait for session receiver
#[async_trait]
pub(crate) trait TransportSessionReceiver: TransportReceiver {
    // /// TODO: dispose/close will consume the ownership, is this still necessary?
    // /// Indicates whether the session link has been closed. This is useful for session receiver scenarios because once the link is closed for a
    // /// session receiver it will not be reopened.
    // ///
    // fn is_session_link_closed(&self) -> bool;

    /// Gets the session id for the current session.
    fn session_id(&self) -> Option<&str>;

    /// Get locked until time for the session
    fn session_locked_until(&self) -> Option<OffsetDateTime>;

    /// Set locked until time for the session
    fn set_session_locked_until(&mut self, session_locked_until: OffsetDateTime);

    /// Renews the lock on the session specified by the `session_id`. The lock will be renewed based
    /// on the setting specified on the entity.
    async fn renew_session_lock(
        &mut self,
        session_id: &str,
    ) -> Result<OffsetDateTime, Self::RequestResponseError>;

    /// Gets the session state.
    async fn session_state(
        &mut self,
        session_id: &str,
    ) -> Result<Vec<u8>, Self::RequestResponseError>;

    /// Set a custom state on the session which can be later retrieved using
    /// [`TransportSessionReceiver::session_state`]
    async fn set_session_state(
        &mut self,
        session_id: &str,
        session_state: Vec<u8>,
    ) -> Result<(), Self::RequestResponseError>;
}

/// Trait for a receiver.
#[async_trait]
pub(crate) trait TransportReceiver: Sealed {
    /// Error with request-response operations
    type RequestResponseError: std::error::Error + Send;

    /// Error with receiving messages
    type ReceiveError: std::error::Error + Send;

    /// Error with disposing messages
    type DispositionError: std::error::Error + Send;

    /// Error with closing the receiver
    type CloseError: std::error::Error + Send;

    /// Get the entity path
    fn entity_path(&self) -> &str;

    /// Get the identifier
    fn identifier(&self) -> &str;

    /// Get the prefetch count
    fn prefetch_count(&self) -> u32;

    /// Get the receive mode
    fn receive_mode(&self) -> ServiceBusReceiveMode;

    /// Receives a set of [`ServiceBusReceivedMessage`] from the entity using
    /// [`ServiceBusReceiveMode`] mode.
    ///
    /// This method should poll indefinitely until at least one message is received.
    async fn receive_messages(
        &mut self,
        max_messages: u32,
    ) -> Result<Vec<ServiceBusReceivedMessage>, Self::ReceiveError>;

    /// Receives a set of [`ServiceBusReceivedMessage`] from the entity using
    /// [`ServiceBusReceiveMode`] mode.
    ///
    /// The `max_wait_time` parameter specifies the maximum amount of time the receiver will wait to
    /// receive the specified number of messages. If the `max_wait_time` is not specified, a default
    /// value that is provided by the `ServiceBusReceiverOptions` will be used.
    async fn receive_messages_with_max_wait_time(
        &mut self,
        max_messages: u32,
        max_wait_time: Option<StdDuration>,
    ) -> Result<Vec<ServiceBusReceivedMessage>, Self::ReceiveError>;

    /// Closes the connection to the transport consumer instance.
    async fn close(self) -> Result<(), Self::CloseError>;

    /// Completes a [`ServiceBusReceivedMessage`]. This will delete the message from the service.
    ///
    /// This operation can only be performed on a message that was received by this receiver
    /// when [`ServiceBusReceiveMode`] is set to [`ServiceBusReceiveMode::PeekLock`].
    async fn complete(
        &mut self,
        message: &ServiceBusReceivedMessage,
        session_id: Option<&str>,
    ) -> Result<(), Self::DispositionError>;

    /// Indicates that the receiver wants to defer the processing for the message.
    ///
    /// In order to receive this message again in the future, you will need to save the
    /// [`ServiceBusReceivedMessage::sequence_number`] and receive it using
    /// `receive_deferred_message()`. Deferring messages does not impact message's expiration,
    /// meaning that deferred messages can still expire. This operation can only be performed on
    /// messages that were received by this receiver.
    async fn defer(
        &mut self,
        message: &ServiceBusReceivedMessage,
        properties_to_modify: Option<OrderedMap<String, Value>>,
        session_id: Option<&str>,
    ) -> Result<(), Self::DispositionError>;

    /// Fetches the next batch of active messages without changing the state of the receiver or the
    /// message source.
    ///
    /// Unlike a received message, peeked message will not have lock token associated with it, and
    /// hence it cannot be Completed/Abandoned/Deferred/Deadlettered/Renewed. Also, unlike
    /// [`TransportReceiver::receive_messages`], this method will fetch even Deferred messages (but
    /// not Deadlettered messages).
    async fn peek_messages(
        &mut self,
        sequence_number: Option<i64>,
        message_count: i32,
    ) -> Result<Vec<ServiceBusPeekedMessage>, Self::RequestResponseError>;

    /// Fetches the next batch of active messages for a session without changing the state of the
    /// receiver or the message source.
    async fn peek_session_messages(
        &mut self,
        sequence_number: Option<i64>,
        message_count: i32,
        session_id: &str,
    ) -> Result<Vec<ServiceBusPeekedMessage>, Self::RequestResponseError>;

    /// Abandons a [`ServiceBusReceivedMessage`]. This will make the message available again for
    /// processing.
    ///
    /// Abandoning a message will increase the delivery count on the message. This operation can
    /// only be performed on messages that were received by this receiver when
    /// [`ServiceBusReceiveMode`] is set to [`ServiceBusReceiveMode.PeekLock`].
    async fn abandon(
        &mut self,
        message: &ServiceBusReceivedMessage,
        properties_to_modify: Option<OrderedMap<String, Value>>,
        session_id: Option<&str>,
    ) -> Result<(), Self::DispositionError>;

    /// Moves a message to the dead-letter subqueue.
    ///
    /// In order to receive a message from the dead-letter queue, you will need a new
    /// [`ServiceBusReceiver`] with the corresponding path.
    /// This operation can only be performed on messages that were received by this receiver
    /// when [`ServiceBusReceiveMode`] is set to [`ServiceBusReceiveMode::PeekLock`].
    async fn dead_letter(
        &mut self,
        message: &ServiceBusReceivedMessage,
        dead_letter_reason: Option<String>,
        dead_letter_error_description: Option<String>,
        properties_to_modify: Option<OrderedMap<String, Value>>,
        session_id: Option<&str>,
    ) -> Result<(), Self::DispositionError>;

    /// Receives a [`Vec`] of deferred messages identified by `sequence_numbers`.
    async fn receive_deferred_messages(
        &mut self,
        sequence_numbers: impl Iterator<Item = i64> + Send,
        session_id: Option<&str>,
    ) -> Result<Vec<ServiceBusReceivedMessage>, Self::RequestResponseError>;

    /// Renews the lock on the message. The lock will be renewed based on the setting specified on
    /// the queue.
    async fn renew_message_lock(
        &mut self,
        lock_tokens: Vec<fe2o3_amqp::types::primitives::Uuid>,
    ) -> Result<Vec<Timestamp>, Self::RequestResponseError>;
}
