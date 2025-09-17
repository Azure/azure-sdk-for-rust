// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use crate::models::{AmqpMessage, AmqpSimpleValue, AmqpValue, MessageId};
use azure_core::fmt::SafeDebug;
use azure_core_amqp::message::{AmqpAnnotationKey, AmqpMessageBody, AmqpMessageProperties};
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    sync::OnceLock,
    time::SystemTime,
};

/// The EventData struct represents the data associated with an event in an Event Hub.
///
/// This struct provides the body, content type, correlation identifier, message identifier, and properties of an event.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use azure_messaging_eventhubs::models::EventData;
///
/// let event_data = EventData::builder()
///    .with_body(b"Hello, world!")
///    .with_content_type("text/plain".to_string())
///    .with_correlation_id("correlation_id")
///    .with_message_id("message_id")
///    .add_property("key".to_string(), "value")
///    .build();
///
/// println!("{:?}", event_data);
/// ```
///
#[derive(Default, PartialEq, Clone, SafeDebug)]
#[safe(true)]
pub struct EventData {
    #[safe(false)]
    body: Option<Vec<u8>>,
    content_type: Option<String>,
    correlation_id: Option<MessageId>,
    message_id: Option<MessageId>,
    #[safe(false)]
    properties: Option<HashMap<String, AmqpSimpleValue>>,
}

impl EventData {
    /// Creates a new builder to build an `EventData`.
    pub fn builder() -> builders::EventDataBuilder {
        builders::EventDataBuilder::new()
    }

    /// The properties of the event.
    pub fn properties(&self) -> Option<&HashMap<String, AmqpSimpleValue>> {
        self.properties.as_ref()
    }

    /// The body of the event.
    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_deref()
    }

    /// The content type of the event, if one was specified.
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    /// The correlation identifier of the event, if one was specified.
    pub fn correlation_id(&self) -> Option<&MessageId> {
        self.correlation_id.as_ref()
    }

    /// The message identifier of the event, if one was specified.
    pub fn message_id(&self) -> Option<&MessageId> {
        self.message_id.as_ref()
    }

    /// Convert the provided AMQP message into an EventData object.
    fn from_message(message: &AmqpMessage) -> Self {
        // // Create an EventData from the message.
        let mut event_data_builder = EventData::builder();

        // If the AMQP message body is a single binary value, copy it to
        // the event data body.
        if let AmqpMessageBody::Binary(binary) = &message.body {
            if binary.len() == 1 {
                event_data_builder = event_data_builder.with_body(binary[0].clone());
            }
        }

        if let Some(properties) = &message.properties {
            if let Some(content_type) = &properties.content_type {
                event_data_builder = event_data_builder.with_content_type(content_type.into());
            }
            if let Some(correlation_id) = &properties.correlation_id {
                event_data_builder = event_data_builder.with_correlation_id(correlation_id.clone());
            }
            if let Some(message_id) = &properties.message_id {
                event_data_builder = event_data_builder.with_message_id(message_id.clone());
            }
        }
        if let Some(application_properties) = &message.application_properties {
            for (key, value) in application_properties.0.clone() {
                event_data_builder = event_data_builder.add_property(key, value);
            }
        }
        event_data_builder.build()
    }
}

impl<T> From<T> for EventData
where
    T: Into<Vec<u8>>,
{
    fn from(body: T) -> Self {
        Self {
            body: Some(body.into()),
            ..Default::default()
        }
    }
}

impl From<EventData> for AmqpMessage {
    fn from(event_data: EventData) -> Self {
        let mut message_builder = AmqpMessage::builder();
        if event_data.content_type.is_some()
            || event_data.correlation_id.is_some()
            || event_data.message_id.is_some()
        {
            let mut message_properties = AmqpMessageProperties::default();
            if let Some(content_type) = event_data.content_type {
                message_properties.content_type = Some(content_type.into());
            }
            if let Some(correlation_id) = event_data.correlation_id {
                message_properties.correlation_id = Some(correlation_id.into());
            }
            if let Some(message_id) = event_data.message_id {
                message_properties.message_id = Some(message_id.into());
            }

            message_builder = message_builder.with_properties(message_properties);
        }
        if let Some(properties) = event_data.properties {
            for (key, value) in properties {
                message_builder = message_builder.add_application_property(key, value);
            }
        }
        if let Some(event_body) = event_data.body {
            message_builder =
                message_builder.with_body(AmqpMessageBody::Binary(vec![event_body.to_vec()]));
        }
        message_builder.build()
    }
}

/// Represents the data associated with an event received from an Event Hub.
///
/// This struct provides the event data, enqueued time, offset, sequence number, partition key, and system properties of the event.
pub struct ReceivedEventData {
    message: AmqpMessage,
    event_data: OnceLock<EventData>,
    enqueued_time: OnceLock<Option<SystemTime>>,
    offset: OnceLock<Option<String>>,
    sequence_number: OnceLock<Option<i64>>,
    partition_key: OnceLock<Option<String>>,
    system_properties: OnceLock<HashMap<String, AmqpValue>>,
}

/// Display the [`ReceivedEventData`]. Since all the fields in `ReceivedEventData` are lazy loaded, we only display the raw AMQP message.
impl Debug for ReceivedEventData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReceivedEventData")
            .field("message", self.raw_amqp_message())
            .finish()
    }
}

const ENQUEUED_TIME_UTC: &str = "x-opt-enqueued-time";
const OFFSET: &str = "x-opt-offset";
const SEQUENCE_NUMBER: &str = "x-opt-sequence-number";
const PARTITION_KEY: &str = "x-opt-partition-key";

impl ReceivedEventData {
    /// The raw AMQP message received from the Event Hubs Service.
    pub fn raw_amqp_message(&self) -> &AmqpMessage {
        &self.message
    }

    /// The Event Data contained within the received event.
    ///
    /// Note that the conversion of AMQP message to EventData is deferred until it is needed.
    pub fn event_data(&self) -> &EventData {
        self.event_data
            .get_or_init(|| EventData::from_message(&self.message))
    }

    /// The time when the event was sent to the the Event Hub.
    pub fn enqueued_time(&self) -> Option<SystemTime> {
        *self.enqueued_time.get_or_init(|| {
            let annotations = self.message.message_annotations.as_ref()?;

            for (key, value) in annotations.0.iter() {
                if let AmqpAnnotationKey::Symbol(symbol) = key {
                    if *symbol == ENQUEUED_TIME_UTC {
                        if let AmqpValue::TimeStamp(timestamp) = value {
                            return timestamp.0;
                        }
                    }
                }
            }

            None
        })
    }

    /// The offset of the event in the Event Hub partition.
    pub fn offset(&self) -> &Option<String> {
        self.offset.get_or_init(|| {
            let annotations = self.message.message_annotations.as_ref()?;
            for (key, value) in annotations.0.iter() {
                if let AmqpAnnotationKey::Symbol(symbol) = key {
                    if *symbol == OFFSET {
                        if let AmqpValue::String(offset_value) = value {
                            return Some(offset_value.clone());
                        }
                    }
                }
            }
            None
        })
    }

    /// The sequence number of the event in the Event Hub partition.
    pub fn sequence_number(&self) -> Option<i64> {
        *self.sequence_number.get_or_init(|| {
            let annotations = self.message.message_annotations.as_ref()?;
            for (key, value) in annotations.0.iter() {
                if let AmqpAnnotationKey::Symbol(symbol) = key {
                    if *symbol == SEQUENCE_NUMBER {
                        if let AmqpValue::Long(sequence_number_value) = value {
                            return Some(*sequence_number_value);
                        }
                    }
                }
            }
            None
        })
    }

    /// The partition key of the event.
    ///
    /// If no partition key is set, then the method will return `None`.
    pub fn partition_key(&self) -> &Option<String> {
        self.partition_key.get_or_init(|| {
            let annotations = self.message.message_annotations.as_ref()?;
            for (key, value) in annotations.0.iter() {
                if let AmqpAnnotationKey::Symbol(symbol) = key {
                    if *symbol == PARTITION_KEY {
                        if let AmqpValue::String(partition_key_value) = value {
                            return Some(partition_key_value.clone());
                        }
                    }
                }
            }
            None
        })
    }

    /// The system properties of the event.
    /// These are properties that are set by the Event Hubs service.
    ///
    /// Note that if there are no system properties, this method will return an empty HashMap.
    pub fn system_properties(&self) -> &HashMap<String, AmqpValue> {
        self.system_properties.get_or_init(|| {
            let mut system_properties = HashMap::new();
            if let Some(annotations) = self.message.message_annotations.as_ref() {
                for (key, value) in annotations.0.iter() {
                    if let AmqpAnnotationKey::Symbol(symbol) = key {
                        if *symbol != ENQUEUED_TIME_UTC
                            && *symbol != OFFSET
                            && *symbol != SEQUENCE_NUMBER
                            && *symbol != PARTITION_KEY
                        {
                            system_properties.insert(symbol.0.clone(), value.clone());
                        }
                    }
                }
            }
            system_properties
        })
    }
}

impl From<AmqpMessage> for ReceivedEventData {
    fn from(message: AmqpMessage) -> Self {
        // Note that we defer calculation of all of the eventhubs specific properties until they are needed.
        Self {
            message,
            event_data: OnceLock::new(),
            enqueued_time: OnceLock::new(),
            offset: OnceLock::new(),
            sequence_number: OnceLock::new(),
            partition_key: OnceLock::new(),
            system_properties: OnceLock::new(),
        }
    }
}

/// Contains builders for types in the Event Hubs Model module.
pub mod builders {
    use super::*;

    /// A builder for the `EventData` struct.
    #[derive(Default)]
    pub struct EventDataBuilder {
        event_data: EventData,
    }

    impl EventDataBuilder {
        pub(super) fn new() -> Self {
            Self {
                event_data: Default::default(),
            }
        }

        /// Sets the body of the event.
        ///
        /// # Parameters
        ///
        /// - `body`: The body of the event.
        ///
        /// # Returns
        ///
        /// A reference to the updated builder.
        ///
        pub fn with_body<T>(mut self, body: T) -> Self
        where
            T: Into<Vec<u8>>,
        {
            self.event_data.body = Some(body.into());
            self
        }

        /// Sets the content type of the event.
        ///
        /// # Parameters
        ///
        /// - `content_type`: The content type of the event.
        ///
        /// # Returns
        ///
        /// A reference to the updated builder.
        ///
        pub fn with_content_type(mut self, content_type: String) -> Self {
            self.event_data.content_type = Some(content_type);
            self
        }

        /// Sets the correlation identifier of the event.
        ///
        /// # Parameters
        ///
        /// - `correlation_id`: The correlation identifier of the event.
        ///
        /// # Returns
        ///
        /// A reference to the updated builder.
        ///
        pub fn with_correlation_id(mut self, correlation_id: impl Into<MessageId>) -> Self {
            self.event_data.correlation_id = Some(correlation_id.into());
            self
        }

        /// Sets the message identifier of the event.
        ///
        /// # Parameters
        ///
        /// - `message_id`: The message identifier of the event.
        ///
        /// # Returns
        ///
        /// A reference to the updated builder.
        ///
        pub fn with_message_id(mut self, message_id: impl Into<MessageId>) -> Self {
            self.event_data.message_id = Some(message_id.into());
            self
        }

        /// Adds a property to the event.
        ///
        /// # Parameters
        ///
        /// - `key`: The key of the property.
        /// - `value`: The value of the property.
        ///
        /// # Returns
        ///
        /// A reference to the updated builder.
        ///
        pub fn add_property(mut self, key: String, value: impl Into<AmqpSimpleValue>) -> Self {
            if let Some(mut properties) = self.event_data.properties {
                properties.insert(key, value.into());
                self.event_data.properties = Some(properties);
            } else {
                let mut properties = HashMap::new();
                properties.insert(key, value.into());
                self.event_data.properties = Some(properties);
            }
            self
        }

        /// Builds the `EventData`.
        ///
        /// # Returns
        ///
        /// The built `EventData`.
        ///
        pub fn build(self) -> EventData {
            self.event_data
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_data_builder_with_body() {
        let body = vec![1, 2, 3];
        let event_data = EventData::builder().with_body(body.clone()).build();

        assert_eq!(event_data.body().unwrap(), &body);
    }

    #[test]
    fn test_event_data_builder_with_content_type() {
        let content_type = "application/json".to_string();
        let event_data = EventData::builder()
            .with_content_type(content_type.clone())
            .build();

        assert_eq!(event_data.content_type(), Some(content_type.as_str()));
    }

    #[test]
    fn test_event_data_builder_with_correlation_id() {
        let correlation_id = MessageId::String("correlation-id".to_string());
        let event_data = EventData::builder()
            .with_correlation_id(correlation_id.clone())
            .build();

        assert_eq!(event_data.correlation_id(), Some(&correlation_id));
    }

    #[test]
    fn test_event_data_builder_with_message_id() {
        let message_id = MessageId::String("message-id".to_string());
        let event_data = EventData::builder()
            .with_message_id(message_id.clone())
            .build();

        assert_eq!(event_data.message_id(), Some(&message_id));
    }

    #[test]
    fn test_event_data_builder_add_property() {
        let key = "key".to_string();
        let value: AmqpSimpleValue = "value".into();
        let event_data = EventData::builder()
            .add_property(key.clone(), value.clone())
            .build();

        assert_eq!(event_data.properties().unwrap().get(&key), Some(&value));
    }

    #[test]
    fn test_event_data_builder_build() {
        let body = vec![1, 2, 3];
        let content_type = "application/json".to_string();
        let correlation_id = MessageId::String("correlation-id".to_string());
        let message_id = MessageId::String("message-id".to_string());
        let key = "key".to_string();
        let value: AmqpSimpleValue = "value".into();

        let event_data = EventData::builder()
            .with_body(body.clone())
            .with_content_type(content_type.clone())
            .with_correlation_id(correlation_id.clone())
            .with_message_id(message_id.clone())
            .add_property(key.clone(), value.clone())
            .build();

        assert_eq!(event_data.body().unwrap(), &body);
        assert_eq!(event_data.content_type(), Some(content_type.as_str()));
        assert_eq!(event_data.correlation_id(), Some(&correlation_id));
        assert_eq!(event_data.message_id(), Some(&message_id));
        assert_eq!(event_data.properties().unwrap().get(&key), Some(&value));
    }
}
