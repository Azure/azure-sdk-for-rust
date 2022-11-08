use crate::{
    core::{TransportMessageBatch, TransportSender},
    CreateMessageBatchOptions, ServiceBusMessage, ServiceBusMessageBatch,
};

pub struct ServiceBusSender<S> {
    pub(crate) inner: S,
    pub(crate) entity_path: String,
    pub(crate) identifier: String,
}

impl<S> ServiceBusSender<S>
where
    S: TransportSender + Send + Sync,
    S::MessageBatch: TransportMessageBatch,
{
    pub async fn create_message_batch(
        &self,
        options: CreateMessageBatchOptions,
    ) -> Result<ServiceBusMessageBatch<S::MessageBatch>, S::CreateMessageBatchError> {
        let inner = self.inner.create_message_batch(options).await?;
        Ok(ServiceBusMessageBatch { inner })
    }

    pub async fn send_message(
        &mut self,
        message: impl Into<ServiceBusMessage>,
    ) -> Result<(), S::SendError> {
        let iter = std::iter::once(message.into());
        self.send_messages(iter).await
    }

    pub async fn send_messages<M, I>(&mut self, messages: M) -> Result<(), S::SendError>
    where
        M: IntoIterator<Item = I>,
        M::IntoIter: ExactSizeIterator + Send,
        I: Into<ServiceBusMessage>,
    {
        let messages = messages.into_iter().map(|m| m.into());
        self.inner.send(messages).await
    }

    pub async fn send_message_batch(
        &mut self,
        batch: ServiceBusMessageBatch<S::MessageBatch>,
    ) -> Result<(), S::SendError> {
        self.inner.send_batch(batch.inner).await
    }

    pub async fn dispose(self) -> Result<(), S::CloseError> {
        self.inner.close().await
    }
}
