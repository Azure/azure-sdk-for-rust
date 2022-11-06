use fe2o3_amqp::link::DetachError;

use crate::{
    amqp::amqp_sender::AmqpSender,
    core::TransportSender,
    primitives::service_bus_retry_policy::{RetryError, ServiceBusRetryPolicy},
    ServiceBusMessage,
};

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
        message: impl Into<ServiceBusMessage>,
    ) -> Result<(), RetryError<RP::Error>> {
        let iter = std::iter::once(message.into());
        self.send_messages(iter).await
    }

    pub async fn send_messages<M, I>(&mut self, messages: M) -> Result<(), RetryError<RP::Error>>
    where
        M: IntoIterator<Item = I>,
        M::IntoIter: ExactSizeIterator + Send,
        I: Into<ServiceBusMessage>,
    {
        let messages = messages.into_iter().map(|m| m.into());
        self.inner.send(messages).await
    }

    pub async fn dispose(self) -> Result<(), DetachError> {
        self.inner.close().await
    }
}
