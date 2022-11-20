use fe2o3_amqp::link::delivery::DeliveryInfo;
use fe2o3_amqp_types::primitives::OrderedMap;
use serde_amqp::Value;

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

    pub async fn receive_message() {
        todo!()
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
        message: impl Into<DeliveryInfo>,
    ) -> Result<(), R::CompleteError> {
        let delivery_info = message.into();
        self.inner.complete(delivery_info).await
    }

    pub async fn abandon_message(
        &mut self,
        message: &ServiceBusReceivedMessage,
        properties_to_modify: Option<OrderedMap<String, Value>>,
    ) -> Result<(), ()> {
        todo!()
    }

    pub async fn dead_letter_message(
        &mut self,
        message: &ServiceBusReceivedMessage,
        properties_to_modify: Option<OrderedMap<String, Value>>,
        dead_letter_reason: Option<String>,
        dead_letter_error_description: Option<String>,
    ) -> Result<(), ()> {
        todo!()
    }

    pub async fn defer_message(
        &mut self,
        message: &ServiceBusReceivedMessage,
        properties_to_modify: Option<OrderedMap<String, Value>>,
    ) -> Result<(), ()> {
        todo!()
    }

    pub async fn peek_message(&mut self, from_sequence_number: Option<i64>) {
        todo!()
    }

    pub async fn peek_messages(&mut self, max_messages: u32, from_sequence_number: Option<i64>) {
        todo!()
    }

    pub async fn receive_deferred_message(&mut self, sequence_number: i64) {
        todo!()
    }

    pub async fn renew_message_lock(&mut self, message: &ServiceBusReceivedMessage) {
        todo!()
    }
}
