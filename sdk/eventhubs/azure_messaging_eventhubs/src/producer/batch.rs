// Copyright (c) Microsoft Corp. All Rights Reserved.

// cspell: words amqp

use std::sync::Mutex;

use super::ProducerClient;

use crate::models::EventData;
use azure_core::error::Result;
use azure_core_amqp::{
    messaging::{AmqpAnnotations, AmqpMessage, AmqpMessageProperties},
    sender::AmqpSenderTrait,
    value::{AmqpSymbol, AmqpValue},
};
use log::debug;
use uuid::Uuid;

pub struct AddEventDataOptions {}

struct EventDataBatchState {
    serialized_messages: Vec<Vec<u8>>,
    size_in_bytes: u64,
    batch_envelope: Option<AmqpMessage>,
}

pub struct EventDataBatch<'a> {
    producer: &'a ProducerClient,

    batch_state: Mutex<EventDataBatchState>,
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
            batch_state: Mutex::new(EventDataBatchState {
                serialized_messages: Vec::new(),
                size_in_bytes: 0,
                batch_envelope: None,
            }),
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
        self.batch_state.lock().unwrap().size_in_bytes
    }

    pub fn len(&self) -> usize {
        self.batch_state.lock().unwrap().serialized_messages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn calculate_actual_size_for_payload(length: usize) -> u64 {
        const MESSAGE_HEADER_SIZE_32: usize = 8;
        const MESSAGE_HEADER_SIZE_8: usize = 5;
        if length < 256 {
            length.checked_add(MESSAGE_HEADER_SIZE_8).unwrap() as u64
        } else {
            length.checked_add(MESSAGE_HEADER_SIZE_32).unwrap() as u64
        }
    }

    pub fn add_amqp_message(
        &self,
        message: AmqpMessage,
        #[allow(unused_variables)] options: Option<AddEventDataOptions>,
    ) -> Result<()> {
        let mut message = message;
        if message.properties().is_none() || message.properties().unwrap().message_id().is_none() {
            let mut message_properties = AmqpMessageProperties::default();
            if let Some(properties) = message.properties() {
                message_properties = properties.clone()
            }
            message_properties.set_message_id(Uuid::new_v4().to_string());
            message.set_properties(message_properties);
        }
        if self.partition_key.is_some() {
            let mut message_annotations = AmqpAnnotations::default();
            if let Some(annotations) = message.message_annotations() {
                message_annotations = annotations.clone()
            }
            message_annotations.insert(
                Into::<AmqpSymbol>::into("x-opt-partition-key"),
                Into::<AmqpValue>::into(self.partition_key.as_ref().unwrap().clone()),
            );
            message.set_message_annotations(message_annotations);
        }

        let mut batch_state = self.batch_state.lock().unwrap();
        if batch_state.serialized_messages.len() == 0 {
            // The first message serialized is the batch envelope - we capture the parameters from the first message to use for the batch
            batch_state.size_in_bytes += AmqpMessage::serialize(message.clone())?.len() as u64;
            batch_state.batch_envelope = Some(message.clone());
        }
        let serialized_message = AmqpMessage::serialize(message)?;
        let actual_message_size = Self::calculate_actual_size_for_payload(serialized_message.len());
        if batch_state
            .size_in_bytes
            .checked_add(actual_message_size)
            .unwrap()
            > self.max_size_in_bytes
        {
            debug!("Batch is full. Cannot add more messages.");
            debug!("Message size: {actual_message_size}");
            debug!("Current batch size: {:?}", batch_state.size_in_bytes);
            debug!("Max batch size: {:?}", self.max_size_in_bytes);
            if batch_state.serialized_messages.len() == 0 {
                batch_state.batch_envelope = None;
                batch_state.size_in_bytes = 0;
            }
            return Err(azure_core::Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Batch is full. Cannot add more messages.",
            )));
        }
        batch_state.size_in_bytes += actual_message_size;
        batch_state.serialized_messages.push(serialized_message);

        Ok(())
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
