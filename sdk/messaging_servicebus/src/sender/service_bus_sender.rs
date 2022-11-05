use fe2o3_amqp::link::DetachError;

use crate::{
    amqp::amqp_sender::AmqpSender, core::TransportSender,
    primitives::service_bus_retry_policy::ServiceBusRetryPolicy, ServiceBusMessage,
};

use super::error::ServiceBusSenderError;

pub struct ServiceBusSender<RP: ServiceBusRetryPolicy> {
    pub(crate) inner: AmqpSender<RP>,
    pub(crate) entity_path: String,
    pub(crate) identifier: String,
}

impl<RP> ServiceBusSender<RP>
where
    RP: ServiceBusRetryPolicy + Send + Sync,
{
    pub async fn send_message(
        &mut self,
        _message: ServiceBusMessage,
    ) -> Result<(), ServiceBusSenderError> {
        todo!()
    }

    pub async fn send_messages(
        &mut self,
        _messages: impl IntoIterator<Item = &ServiceBusMessage>,
    ) -> Result<(), ServiceBusSenderError> {
        todo!()
    }

    pub async fn dispose(self) -> Result<(), DetachError> {
        self.inner.close().await
    }
}
