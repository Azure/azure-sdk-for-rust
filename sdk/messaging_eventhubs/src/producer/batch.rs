// cspell: words amqp

use super::ProducerClient;

use crate::amqp_client::sender::AmqpSenderTrait;
use crate::models::EventData;
use azure_core::error::Result;

pub struct AddEventDataOptions {}

pub struct EventDataBatch<'a> {
    producer: &'a ProducerClient,
    serialized_messages: Vec<Vec<u8>>,
    size_in_bytes: u64,
    max_size_in_bytes: u64,
    partition_key: Option<String>,
    partition_id: Option<String>,
}

impl<'a> EventDataBatch<'a> {
    pub(crate) fn new(
        producer: &'a ProducerClient,
        options: Option<EventDataBatchOptions>,
    ) -> Self {
        Self {
            producer,
            serialized_messages: Vec::new(),
            size_in_bytes: 0,
            max_size_in_bytes: options.as_ref().map_or(std::u64::MAX, |o| {
                o.max_size_in_bytes.unwrap_or(std::u64::MAX)
            }),
            partition_key: options.as_ref().and_then(|o| o.partition_key.clone()),
            partition_id: options.and_then(|o| o.partition_id),
        }
    }

    pub(crate) async fn attach(&mut self) -> Result<()> {
        let sender = self.producer.ensure_sender(self.get_batch_path()).await?;
        self.max_size_in_bytes = sender.lock().await.max_message_size().await.unwrap();
        Ok(())
    }

    pub fn size(&self) -> u64 {
        self.size_in_bytes
    }

    pub fn len(&self) -> usize {
        self.serialized_messages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.serialized_messages.is_empty()
    }

    pub fn add_amqp_message(
        &self,
        message: crate::amqp_client::messaging::AmqpMessage,
        #[allow(unused_variables)] options: Option<AddEventDataOptions>,
    ) -> Result<()> {
        unimplemented!()
    }

    pub fn add_event_data(
        &mut self,
        event_data: impl Into<EventData>,
        #[allow(unused_variables)] options: Option<AddEventDataOptions>,
    ) -> Result<()> {
        let event_data = event_data.into();
        self.add_amqp_message(event_data.into(), options)
    }

    fn get_batch_path(&self) -> String {
        if self.partition_id.is_none() {
            format!("{}", self.producer.base_url())
        } else {
            format!(
                "{}/Partitions/{}",
                self.producer.base_url(),
                self.partition_id.as_ref().unwrap()
            )
        }
    }
}
pub struct EventDataBatchOptions {
    pub(crate) max_size_in_bytes: Option<u64>,
    pub(crate) partition_key: Option<String>,
    pub(crate) partition_id: Option<String>,
}

impl EventDataBatchOptions {
    pub fn builder() -> EventDataBatchOptionsBuilder {
        EventDataBatchOptionsBuilder::new()
    }
}

pub struct EventDataBatchOptionsBuilder {
    max_size_in_bytes: Option<u64>,
    partition_key: Option<String>,
    partition_id: Option<String>,
}

impl EventDataBatchOptionsBuilder {
    pub fn new() -> Self {
        Self {
            max_size_in_bytes: None,
            partition_key: None,
            partition_id: None,
        }
    }

    pub fn max_size_in_bytes(mut self, max_size_in_bytes: u64) -> Self {
        self.max_size_in_bytes = Some(max_size_in_bytes);
        self
    }

    pub fn partition_key(mut self, partition_key: impl Into<String>) -> Self {
        self.partition_key = Some(partition_key.into());
        self
    }

    pub fn partition_id(mut self, partition_id: impl Into<String>) -> Self {
        self.partition_id = Some(partition_id.into());
        self
    }

    pub fn build(self) -> EventDataBatchOptions {
        EventDataBatchOptions {
            max_size_in_bytes: self.max_size_in_bytes,
            partition_key: self.partition_key,
            partition_id: self.partition_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_builder() {
        let options = EventDataBatchOptions::builder()
            .max_size_in_bytes(1024)
            .partition_key("pk")
            .partition_id("pid")
            .build();

        assert_eq!(options.max_size_in_bytes, Some(1024));
        assert_eq!(options.partition_key, Some("pk".to_string()));
        assert_eq!(options.partition_id, Some("pid".to_string()));
    }
}
