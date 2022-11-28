use time::OffsetDateTime;

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
    pub fn create_message_batch(
        &self,
        options: CreateMessageBatchOptions,
    ) -> Result<ServiceBusMessageBatch<S::MessageBatch>, S::CreateMessageBatchError> {
        let inner = self.inner.create_message_batch(options)?;
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

    pub async fn schedule_message(
        &mut self,
        message: impl Into<ServiceBusMessage>,
        enqueue_time: OffsetDateTime,
    ) -> Result<i64, S::ScheduleError> {
        let messages = std::iter::once(message.into());
        let seq_nums = self.schedule_messages(messages, enqueue_time).await?;
        // PANIC: there should be exactly one sequence number returned
        assert_eq!(seq_nums.len(), 1);
        Ok(seq_nums[0])
    }

    pub async fn schedule_messages<M, I>(
        &mut self,
        messages: M,
        enqueue_time: OffsetDateTime,
    ) -> Result<Vec<i64>, S::ScheduleError>
    where
        M: IntoIterator<Item = I>,
        M::IntoIter: ExactSizeIterator + Send,
        I: Into<ServiceBusMessage>,
    {
        let iter = messages.into_iter();
        if iter.len() == 0 {
            return Ok(vec![]);
        }
        let messages = iter.map(|m| {
            let mut m = m.into();
            m.set_scheduled_enqueue_time(enqueue_time);
            m
        });
        self.inner.schedule_messages(messages).await
    }

    pub async fn cancel_scheduled_message(
        &mut self,
        sequence_number: i64,
    ) -> Result<(), S::ScheduleError> {
        // The request will always encode the sequence numbers as a Vec, so it doesn't hurt to
        // allocate a Vec here.
        self.cancel_scheduled_messages(std::iter::once(sequence_number))
            .await
    }

    pub async fn cancel_scheduled_messages<I>(
        &mut self,
        sequence_numbers: I,
    ) -> Result<(), S::ScheduleError>
    where
        I: IntoIterator<Item = i64>,
        I::IntoIter: ExactSizeIterator + Send,
    {
        let iter: Vec<i64> = sequence_numbers.into_iter().collect();
        if iter.is_empty() {
            return Ok(());
        }

        self.inner.cancel_scheduled_messages(iter).await
    }

    pub async fn dispose(self) -> Result<(), S::CloseError> {
        self.inner.close().await
    }
}
