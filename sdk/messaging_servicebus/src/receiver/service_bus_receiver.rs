use fe2o3_amqp::link::{delivery::DeliveryInfo, DetachError};

use crate::{
    amqp::amqp_receiver::AmqpReceiver,
    core::TransportReceiver,
    primitives::{
        service_bus_received_message::ServiceBusReceivedMessage,
        service_bus_retry_policy::{RetryError, ServiceBusRetryPolicy},
    },
};

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
        max_messages: usize,
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
