use fe2o3_amqp::link::delivery::DeliveryInfo;

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
}
