use fe2o3_amqp_types::primitives::OrderedMap;
use serde_amqp::Value;
use time::OffsetDateTime;

use crate::primitives::service_bus_peeked_message::ServiceBusPeekedMessage;
use crate::{
    core::TransportReceiver, primitives::service_bus_received_message::ServiceBusReceivedMessage,
};

use crate::{primitives::sub_queue::SubQueue, ServiceBusReceiveMode};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceBusReceiverOptions {
    pub prefetch_count: u32,
    pub receive_mode: ServiceBusReceiveMode,
    pub identifier: Option<String>,
    pub sub_queue: SubQueue,
}

pub struct ServiceBusReceiver<R> {
    pub(crate) inner: R,
    pub(crate) entity_path: String,
    pub(crate) identifier: String,
}

impl<R> ServiceBusReceiver<R>
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
        self.inner.complete(message, None).await
    }

    pub async fn abandon_message(
        &mut self,
        message: &ServiceBusReceivedMessage,
        properties_to_modify: Option<OrderedMap<String, Value>>,
    ) -> Result<(), R::DispositionError> {
        self.inner
            .abandon(message, properties_to_modify, None)
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
                None,
            )
            .await
    }

    pub async fn defer_message(
        &mut self,
        message: &ServiceBusReceivedMessage,
        properties_to_modify: Option<OrderedMap<String, Value>>,
    ) -> Result<(), R::DispositionError> {
        self.inner.defer(message, properties_to_modify, None).await
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
            .peek_message(from_sequence_number, max_messages as i32)
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
            .receive_deferred_messages(sequence_numbers, None)
            .await
    }

    pub async fn renew_message_lock(
        &mut self,
        _message: &ServiceBusReceivedMessage,
    ) -> Result<OffsetDateTime, R::RequestResponseError> {
        // TODO: what if lock token is None?
        todo!()
    }
}
