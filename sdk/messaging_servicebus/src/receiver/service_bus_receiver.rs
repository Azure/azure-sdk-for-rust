use fe2o3_amqp::link::{delivery::DeliveryInfo, DetachError};

use crate::{
    amqp::amqp_receiver::AmqpReceiver,
    core::TransportReceiver,
    primitives::{
        service_bus_received_message::ServiceBusReceivedMessage,
        service_bus_retry_policy::{RetryError, ServiceBusRetryPolicy},
    },
};

use super::error::ServiceBusRecvError;

pub struct ServiceBusReceiver<RP: ServiceBusRetryPolicy> {
    pub(crate) inner: AmqpReceiver<RP>,
    pub(crate) entity_path: String,
    pub(crate) identifier: String,
}

impl<RP> ServiceBusReceiver<RP>
where
    RP: ServiceBusRetryPolicy + Send,
{
    pub async fn dispose(self) -> Result<(), DetachError> {
        self.inner.close().await
    }

    pub async fn receive_messages(
        &mut self,
        max_messages: u32,
        max_wait_time: Option<std::time::Duration>,
    ) -> Result<Vec<ServiceBusReceivedMessage>, ServiceBusRecvError> {
        self.inner
            .receive_messages(max_messages, max_wait_time)
            .await
    }

    pub async fn complete_message(
        &mut self,
        message: impl Into<DeliveryInfo>,
    ) -> Result<(), RetryError<RP::Error>> {
        let delivery_info = message.into();
        self.inner.complete(delivery_info).await
    }
}
