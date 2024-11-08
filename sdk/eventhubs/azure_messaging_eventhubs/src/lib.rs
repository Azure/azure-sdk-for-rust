// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

#![recursion_limit = "128"]
#![warn(missing_docs)]
// cspell: words amqp eventdata
#![doc = include_str!("../README.md")]

/// let consumer_client = ConsumerClient::new("fully_qualified_domain", "eventhub_name", None, my_credentials, None);
/// let partition_properties = consumer_client.get_partition_properties("0").await?;
pub(crate) mod common;

/// Types related to consuming events from an Event Hub.
pub mod consumer;

/// Types related to errors processing events.
pub mod error;

/// Types to create and send events to an Event Hub.
pub mod producer;

/// Model types sent to and received from an Event Hub.
pub mod models {
    /// An AMQP Message sent to the eventhubs service.
    pub type AmqpMessage = azure_core_amqp::messaging::AmqpMessage;

    /// The body of an AMQP message.
    pub type AmqpMessageBody = azure_core_amqp::messaging::AmqpMessageBody;

    /// The unique identifier of an AMQP message.
    pub type AmqpMessageId = azure_core_amqp::messaging::AmqpMessageId;

    /// The properties of an AMQP message.
    pub type AmqpMessageProperties = azure_core_amqp::messaging::AmqpMessageProperties;

    /// An AMQP Value.
    pub type AmqpValue = azure_core_amqp::value::AmqpValue;

    use azure_core_amqp::messaging::AmqpAnnotationKey;
    use std::{
        collections::HashMap,
        fmt::{Debug, Display, Formatter},
    };
    use tracing::warn;

    /// Represents the properties of an Event Hub.
    ///
    /// This struct provides detailed information about an Event Hub, including its name, creation time, and the unique identifiers of its partitions.
    ///
    /// # Fields
    ///
    /// - `name`: A `String` representing the name of the Event Hub.
    /// - `created_on`: A `std::time::SystemTime` representing the UTC time when the Event Hub was created.
    /// - `partition_ids`: A `Vec<String>` containing the unique identifiers of the partitions in the Event Hub.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// # use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let my_credentials = DefaultAzureCredential::new()?;
    /// let consumer_client = azure_messaging_eventhubs::consumer::ConsumerClient::new("fully_qualified_domain".to_string(), "eventhub_name".to_string(), None, my_credentials, None);
    ///
    /// let eventhub_properties = consumer_client.get_eventhub_properties().await?;
    ///
    /// for partition_id in eventhub_properties.partition_ids {
    ///    println!("Partition ID: {}", partition_id);
    /// }
    /// # Ok(()) }
    /// ```
    ///
    #[derive(Debug)]
    pub struct EventHubProperties {
        /// The name of the Event Hub.
        pub name: String,

        /// The time when the Event Hub was created.
        pub created_on: Option<std::time::SystemTime>,

        /// The unique identifiers of the partitions in the Event Hub.
        pub partition_ids: Vec<String>,
    }

    /// Represents the properties of an Event Hub partition.
    ///
    /// This struct provides detailed information about a specific partition within an Event Hub, including its unique identifier, the Event Hub it belongs to, sequence numbers for events, and more.
    ///
    /// # Fields
    ///
    /// - `id`: A `String` representing the unique identifier of the partition.
    /// - `eventhub`: A `String` representing the name of the Event Hub this partition belongs to.
    /// - `beginning_sequence_number`: An `i64` representing the sequence number of the earliest event that can be received from this partition.
    /// - `last_enqueued_sequence_number`: An `i64` representing the sequence number of the latest event that has been enqueued in this partition.
    /// - `last_enqueued_offset`: A `String` representing the offset of the latest event that has been enqueued in this partition. This can be used to start receiving from this event onwards.
    /// - `last_enqueued_time_utc`: A `std::time::SystemTime` representing the UTC time when the last event was enqueued in this partition.
    /// - `is_empty`: A `bool` indicating whether the partition is empty (i.e., contains no events).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// # use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let my_credentials = DefaultAzureCredential::new()?;
    /// let consumer_client = azure_messaging_eventhubs::consumer::ConsumerClient::new("fully_qualified_domain".to_string(), "eventhub_name".to_string(), None, my_credentials, None);
    ///
    /// let partition_properties = consumer_client.get_partition_properties("0".to_string()).await?;
    /// # Ok(()) }
    /// ```
    ///
    #[derive(Debug)]
    pub struct EventHubPartitionProperties {
        /// The unique identifier of the partition.
        pub id: String,

        /// The name of the Event Hub this partition belongs to.
        pub eventhub: String,

        /// The sequence number of the earliest event that can be received from this partition.
        pub beginning_sequence_number: i64,

        /// The sequence number of the latest event that has been enqueued in this partition.
        pub last_enqueued_sequence_number: i64,

        /// The offset of the latest event that has been enqueued in this partition.
        pub last_enqueued_offset: String,

        /// The UTC time when the last event was enqueued in this partition.
        /// This will be `None` if the partition is empty.
        pub last_enqueued_time_utc: Option<std::time::SystemTime>,

        /// Indicates whether the partition is empty.
        pub is_empty: bool,
    }

    /// Uniquely identifies a message.
    ///
    /// This type can be used to uniquely identify a message within a message broker or messaging system.
    /// The message producer is usually responsible for setting the message-id in such a way that it is
    /// assured to be globally unique.
    ///
    #[derive(Debug, PartialEq, Clone)]
    pub enum MessageId {
        /// A binary representation of the message identifier.
        Binary(Vec<u8>),

        /// A string representation of the message identifier.
        String(String),

        /// A 64-bit unsigned integer representation of the message identifier.
        Ulong(u64),

        /// A UUID representation of the message identifier.
        Uuid(uuid::Uuid),
    }

    impl From<u64> for MessageId {
        fn from(value: u64) -> Self {
            Self::Ulong(value)
        }
    }

    impl From<uuid::Uuid> for MessageId {
        fn from(value: uuid::Uuid) -> Self {
            Self::Uuid(value)
        }
    }

    impl From<Vec<u8>> for MessageId {
        fn from(value: Vec<u8>) -> Self {
            Self::Binary(value)
        }
    }

    impl From<&str> for MessageId {
        fn from(value: &str) -> Self {
            Self::String(value.to_string())
        }
    }

    impl From<String> for MessageId {
        fn from(value: String) -> Self {
            Self::String(value)
        }
    }

    impl From<MessageId> for uuid::Uuid {
        fn from(message_id: MessageId) -> Self {
            match message_id {
                MessageId::Uuid(uuid) => uuid,
                _ => panic!("Cannot convert MessageId to Uuid"),
            }
        }
    }

    impl From<MessageId> for Vec<u8> {
        fn from(message_id: MessageId) -> Self {
            match message_id {
                MessageId::Binary(binary) => binary,
                _ => panic!("Cannot convert MessageId to Vec<u8>"),
            }
        }
    }

    impl From<MessageId> for String {
        fn from(message_id: MessageId) -> Self {
            match message_id {
                MessageId::String(string) => string,
                _ => panic!("Cannot convert MessageId to String"),
            }
        }
    }

    impl From<MessageId> for u64 {
        fn from(message_id: MessageId) -> Self {
            match message_id {
                MessageId::Ulong(ulong) => ulong,
                _ => panic!("Cannot convert MessageId to u64"),
            }
        }
    }

    impl From<AmqpMessageId> for MessageId {
        fn from(message_id: AmqpMessageId) -> Self {
            match message_id {
                AmqpMessageId::String(string) => MessageId::String(string),
                AmqpMessageId::Uuid(uuid) => MessageId::Uuid(uuid),
                AmqpMessageId::Binary(binary) => MessageId::Binary(binary),
                AmqpMessageId::Ulong(ulong) => MessageId::Ulong(ulong),
            }
        }
    }

    impl From<MessageId> for AmqpMessageId {
        fn from(message_id: MessageId) -> Self {
            match message_id {
                MessageId::String(string) => AmqpMessageId::String(string),
                MessageId::Uuid(uuid) => AmqpMessageId::Uuid(uuid),
                MessageId::Binary(binary) => AmqpMessageId::Binary(binary),
                MessageId::Ulong(ulong) => AmqpMessageId::Ulong(ulong),
            }
        }
    }

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
    #[derive(Default, PartialEq, Clone)]
    pub struct EventData {
        body: Option<Vec<u8>>,
        content_type: Option<String>,
        correlation_id: Option<MessageId>,
        message_id: Option<MessageId>,
        properties: Option<HashMap<String, AmqpValue>>,
    }

    impl EventData {
        /// Creates a new builder to build an `EventData`.
        pub fn builder() -> builders::EventDataBuilder {
            builders::EventDataBuilder::new()
        }

        /// The properties of the event.
        pub fn properties(&self) -> Option<&HashMap<String, AmqpValue>> {
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
    }

    impl Debug for EventData {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("EventData")
                .field("body", &self.body)
                .field("content_type", &self.content_type)
                .field("correlation_id", &self.correlation_id)
                .field("message_id", &self.message_id)
                .field("properties", &self.properties)
                .finish()
        }
    }

    impl Display for EventData {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "EventData: [")?;
            write!(f, "  body: {:?}, ", self.body)?;
            write!(f, "  content_type: {:?}, ", self.content_type)?;
            write!(f, "  correlation_id: {:?}, ", self.correlation_id)?;
            write!(f, "  message_id: {:?}, ", self.message_id)?;
            write!(f, "  properties: {:?}", self.properties)?;
            write!(f, "]")
        }
    }

    impl<T> From<T> for EventData
    where
        T: Into<Vec<u8>>,
    {
        fn from(body: T) -> Self {
            Self {
                body: Some(body.into()),
                content_type: None,
                correlation_id: None,
                message_id: None,
                properties: None,
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
        event_data: EventData,
        enqueued_time: Option<std::time::SystemTime>,
        offset: String,
        sequence_number: i64,
        partition_key: String,
        system_properties: HashMap<String, AmqpValue>,
    }

    impl Debug for ReceivedEventData {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ReceivedEventData")
                //                .field("message", &self.message)
                .field("event_data", &self.event_data)
                .field("enqueued_time", &self.enqueued_time)
                .field("offset", &self.offset)
                .field("sequence_number", &self.sequence_number)
                .field("partition_key", &self.partition_key)
                .field("system_properties", &self.system_properties)
                .finish()
        }
    }

    impl ReceivedEventData {
        /// The raw AMQP message received from the Event Hubs Service.
        pub fn raw_amqp_message(&self) -> &AmqpMessage {
            &self.message
        }

        /// The Event Data contained within the received event.
        pub fn event_data(&self) -> &EventData {
            &self.event_data
        }

        /// The time when the event was sent to the the Event Hub.
        pub fn enqueued_time(&self) -> Option<std::time::SystemTime> {
            self.enqueued_time
        }

        /// The offset of the event in the Event Hub partition.
        pub fn offset(&self) -> &str {
            &self.offset
        }

        /// The sequence number of the event in the Event Hub partition.
        pub fn sequence_number(&self) -> i64 {
            self.sequence_number
        }

        /// The partition key of the event.
        pub fn partition_key(&self) -> &str {
            &self.partition_key
        }

        /// The system properties of the event.
        pub fn system_properties(&self) -> &HashMap<String, AmqpValue> {
            &self.system_properties
        }
    }

    const ENQUEUED_TIME_UTC: &str = "x-opt-enqueued-time";
    const OFFSET: &str = "x-opt-offset";
    const SEQUENCE_NUMBER: &str = "x-opt-sequence-number";
    const PARTITION_KEY: &str = "x-opt-partition-key";

    impl From<AmqpMessage> for ReceivedEventData {
        fn from(message: AmqpMessage) -> Self {
            // Create an eventdata from the message.
            let mut event_data_builder = EventData::builder();

            // If the AMQP message body is a single binary value, copy it to
            // the event data body.
            if let AmqpMessageBody::Binary(binary) = message.body() {
                if binary.len() == 1 {
                    event_data_builder = event_data_builder.with_body(binary[0].clone());
                }
            }

            if let Some(properties) = message.properties() {
                if let Some(content_type) = &properties.content_type {
                    event_data_builder = event_data_builder
                        .with_content_type(Into::<String>::into(content_type.clone()));
                }
                if let Some(correlation_id) = &properties.correlation_id {
                    event_data_builder =
                        event_data_builder.with_correlation_id(correlation_id.clone());
                }
                if let Some(message_id) = &properties.message_id {
                    event_data_builder = event_data_builder.with_message_id(message_id.clone());
                }
            }
            if let Some(application_properties) = message.application_properties() {
                for (key, value) in application_properties.0.clone() {
                    event_data_builder = event_data_builder.add_property(key, value);
                }
            }
            let event_data = event_data_builder.build();

            // Extract the Eventhubs specific properties from the message.
            let mut enqueued_time: Option<std::time::SystemTime> =
                Some(std::time::SystemTime::now());
            let mut sequence_number: i64 = 0;
            let mut partition_key: String = String::new();
            let mut offset: String = String::new();
            let mut system_properties: HashMap<String, AmqpValue> = HashMap::new();
            if let Some(annotations) = message.message_annotations() {
                for (key, value) in annotations.0.clone() {
                    if let AmqpAnnotationKey::Symbol(symbol) = key {
                        if symbol == ENQUEUED_TIME_UTC {
                            if let AmqpValue::TimeStamp(timestamp) = value {
                                enqueued_time = timestamp.0;
                            }
                        } else if symbol == OFFSET {
                            if let AmqpValue::String(offset_value) = value {
                                offset = offset_value;
                            }
                        } else if symbol == SEQUENCE_NUMBER {
                            if let AmqpValue::Long(sequence_number_value) = value {
                                sequence_number = sequence_number_value;
                            }
                        } else if symbol == PARTITION_KEY {
                            if let AmqpValue::String(partition_key_value) = value {
                                partition_key = partition_key_value;
                            }
                        } else {
                            if system_properties.contains_key(&symbol.0) {
                                warn!("Duplicate system property found: {}", symbol.0);
                            }
                            system_properties.insert(symbol.0, value);
                        }
                    }
                }
            }

            Self {
                event_data,
                message,
                enqueued_time,
                offset,
                sequence_number,
                partition_key,
                system_properties,
            }
        }
    }

    /// Contains builders for types in the EventHubs Model module.
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
            pub fn add_property(mut self, key: String, value: impl Into<AmqpValue>) -> Self {
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
}

#[cfg(test)]
mod tests {
    use super::models::*;
    use azure_core_amqp::{messaging::AmqpMessageId, value::AmqpValue};

    #[test]
    fn test_message_id_from_u64() {
        let value: u64 = 123;
        let message_id = MessageId::from(value);
        assert_eq!(message_id, MessageId::Ulong(value));
    }

    #[test]
    fn test_message_id_from_uuid() {
        let value = uuid::Uuid::new_v4();
        let message_id = MessageId::from(value);
        assert_eq!(message_id, MessageId::Uuid(value));
    }

    #[test]
    fn test_message_id_from_vec_u8() {
        let value: Vec<u8> = vec![1, 2, 3];
        let message_id = MessageId::from(value.clone());
        assert_eq!(message_id, MessageId::Binary(value));
    }

    #[test]
    fn test_message_id_from_string() {
        let value = "test".to_string();
        let message_id = MessageId::from(value.clone());
        assert_eq!(message_id, MessageId::String(value));
    }

    #[test]
    fn test_message_id_into_uuid() {
        let value = uuid::Uuid::new_v4();
        let message_id: MessageId = value.into();
        assert_eq!(message_id, MessageId::Uuid(value));
    }

    #[test]
    fn test_message_id_into_vec_u8() {
        let value: Vec<u8> = vec![1, 2, 3];
        let message_id: MessageId = value.clone().into();
        assert_eq!(message_id, MessageId::Binary(value));
    }

    #[test]
    fn test_message_id_into_string() {
        let value = "test".to_string();
        let message_id: MessageId = value.clone().into();
        assert_eq!(message_id, MessageId::String(value));
    }

    #[test]
    fn test_message_id_into_u64() {
        let value: u64 = 123;
        let message_id: MessageId = value.into();
        assert_eq!(message_id, MessageId::Ulong(value));
    }

    #[test]
    fn test_message_id_from_amqp_message_id() {
        let value = AmqpMessageId::Binary(vec![2, 4, 6, 8]);
        let message_id = MessageId::from(value.clone());
        assert_eq!(message_id, MessageId::from(value));
    }

    #[test]
    fn test_message_id_into_amqp_message_id() {
        let value = AmqpMessageId::Binary(vec![3, 5, 7, 9]);
        let message_id: MessageId = value.clone().into();
        assert_eq!(message_id, MessageId::from(value));
    }
    #[test]
    fn test_event_data_builder_with_body() {
        let body = vec![1, 2, 3];
        let event_data = EventData::builder().with_body(body.clone()).build();

        assert_eq!(event_data.body().unwrap(), &body);
    }

    #[test]
    fn test_event_data_builder_with_content_type() {
        let content_type = "application/json";
        let event_data = EventData::builder()
            .with_content_type(content_type.to_string())
            .build();

        assert_eq!(event_data.content_type(), Some(content_type));
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
        let value: AmqpValue = "value".into();
        let event_data = EventData::builder()
            .add_property(key.clone(), value.clone())
            .build();

        assert_eq!(event_data.properties().unwrap().get(&key), Some(&value));
    }

    #[test]
    fn test_event_data_builder_build() {
        let body = vec![1, 2, 3];
        let content_type = "application/json";
        let correlation_id = MessageId::String("correlation-id".to_string());
        let message_id = MessageId::String("message-id".to_string());
        let key = "key".to_string();
        let value: AmqpValue = "value".into();

        let event_data = EventData::builder()
            .with_body(body.clone())
            .with_content_type(content_type.to_string())
            .with_correlation_id(correlation_id.clone())
            .with_message_id(message_id.clone())
            .add_property(key.clone(), value.clone())
            .build();

        assert_eq!(event_data.body().unwrap(), &body);
        assert_eq!(event_data.content_type(), Some(content_type));
        assert_eq!(event_data.correlation_id(), Some(&correlation_id));
        assert_eq!(event_data.message_id(), Some(&message_id));
        assert_eq!(event_data.properties().unwrap().get(&key), Some(&value));
    }
}
