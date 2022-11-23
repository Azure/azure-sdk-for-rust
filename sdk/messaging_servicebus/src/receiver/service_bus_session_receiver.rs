use fe2o3_amqp_types::primitives::OrderedMap;
use serde_amqp::Value;

use crate::{
    core::TransportReceiver,
    primitives::{
        service_bus_peeked_message::ServiceBusPeekedMessage,
        service_bus_received_message::ServiceBusReceivedMessage,
    },
    ServiceBusReceiveMode, ServiceBusReceiverOptions,
};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceBusSessionReceiverOptions {
    /// <summary>
    /// Gets or sets the number of messages that will be eagerly requested from Queues or Subscriptions and queued locally without regard to
    /// whether the receiver is actively receiving, intended to help maximize throughput by allowing the receiver to receive
    /// from a local cache rather than waiting on a service request.
    /// </summary>
    /// <exception cref="ArgumentOutOfRangeException">
    ///   A negative value is attempted to be set for the property.
    /// </exception>
    pub prefetch_count: u32,

    /// <summary>
    /// Gets or sets the <see cref="ReceiveMode"/> used to specify how messages are received. Defaults to PeekLock mode.
    /// </summary>
    pub receive_mode: ServiceBusReceiveMode,

    /// <inheritdoc cref="ServiceBusReceiverOptions.Identifier"/>
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

pub struct ServiceBusSessionReceiver<R> {
    pub(crate) inner: R,
    pub(crate) entity_path: String,
    pub(crate) identifier: String,
    pub(crate) session_id: String,
}

impl<R> ServiceBusSessionReceiver<R>
where
    R: TransportReceiver,
{
    pub async fn dispose(self) -> Result<(), R::CloseError> {
        self.inner.close().await
    }

    /// Receive a single message from the entity.
    pub async fn receive_message(
        &mut self,
    ) -> Result<Option<ServiceBusReceivedMessage>, R::ReceiveError> {
        self.receive_messages(1, None)
            .await
            .map(|mut v| v.drain(..).next())
    }

    pub async fn receive_messages(
        &mut self,
        max_messages: u32,
        max_wait_time: Option<std::time::Duration>,
    ) -> Result<Vec<ServiceBusReceivedMessage>, R::ReceiveError> {
        self.inner
            .receive_messages(max_messages, max_wait_time)
            .await
    }

    pub async fn complete_message(
        &mut self,
        message: &ServiceBusReceivedMessage,
    ) -> Result<(), R::DispositionError> {
        self.inner.complete(message, Some(&self.session_id)).await
    }

    pub async fn abandon_message(
        &mut self,
        message: &ServiceBusReceivedMessage,
        properties_to_modify: Option<OrderedMap<String, Value>>,
    ) -> Result<(), R::DispositionError> {
        self.inner
            .abandon(message, properties_to_modify, Some(&self.session_id))
            .await
    }

    pub async fn defer_message(
        &mut self,
        message: &ServiceBusReceivedMessage,
        properties_to_modify: Option<OrderedMap<String, Value>>,
    ) -> Result<(), R::DispositionError> {
        self.inner
            .defer(message, properties_to_modify, Some(&self.session_id))
            .await
    }

    pub async fn dead_letter_message(
        &mut self,
        message: &ServiceBusReceivedMessage,
        dead_letter_reason: Option<String>,
        dead_letter_error_description: Option<String>,
        properties_to_modify: Option<OrderedMap<String, Value>>,
    ) -> Result<(), R::DispositionError> {
        self.inner
            .dead_letter(
                message,
                dead_letter_reason,
                dead_letter_error_description,
                properties_to_modify,
                Some(&self.session_id),
            )
            .await
    }

    pub async fn peek_message(
        &mut self,
        from_sequence_number: Option<i64>,
    ) -> Result<Option<ServiceBusPeekedMessage>, R::RequestResponseError> {
        self.peek_messages(1, from_sequence_number)
            .await
            .map(|mut v| v.drain(..).next())
    }

    pub async fn peek_messages(
        &mut self,
        max_messages: u32, // FIXME: stop user from putting a negative number here?
        from_sequence_number: Option<i64>,
    ) -> Result<Vec<ServiceBusPeekedMessage>, R::RequestResponseError> {
        self.inner
            .peek_session_message(from_sequence_number, max_messages as i32, &self.session_id)
            .await
    }

    /// TODO: should the return type be `Result<Option<_>>`?
    pub async fn receive_deferred_message(
        &mut self,
        sequence_number: i64,
    ) -> Result<Option<ServiceBusReceivedMessage>, R::RequestResponseError> {
        self.receive_deferred_messages(std::iter::once(sequence_number))
            .await
            .map(|mut v| v.drain(..).next())
    }

    pub async fn receive_deferred_messages(
        &mut self,
        sequence_numbers: impl Iterator<Item = i64> + Send,
    ) -> Result<Vec<ServiceBusReceivedMessage>, R::RequestResponseError> {
        self.inner
            .receive_deferred_messages(sequence_numbers, Some(&self.session_id))
            .await
    }

    pub async fn renew_message_lock(
        &mut self,
        message: &mut ServiceBusReceivedMessage,
    ) -> Result<(), R::RequestResponseError> {
        todo!()
    }
}
