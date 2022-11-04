use fe2o3_amqp::link::DetachError;

use crate::{
    amqp::amqp_sender::AmqpSender,
    core::{TransportClient, TransportSender},
    primitives::service_bus_connection::ServiceBusConnection,
    ServiceBusMessage, ServiceBusSenderOptions,
};

use super::error::ServiceBusSenderError;

pub struct ServiceBusSender {
    pub(crate) inner: AmqpSender,
    pub(crate) entity_path: String,
    pub(crate) identifier: String,
}

impl ServiceBusSender {
    pub async fn send_message(
        &mut self,
        message: ServiceBusMessage,
    ) -> Result<(), ServiceBusSenderError> {
        todo!()
    }

    pub async fn send_messages(
        &mut self,
        messages: impl IntoIterator<Item = &ServiceBusMessage>,
    ) -> Result<(), ServiceBusSenderError> {
        todo!()
    }

    pub async fn dispose(self) -> Result<(), DetachError> {
        self.inner.close().await
    }
}
