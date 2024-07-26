// Copyright (c) Microsoft Corp. All Rights Reserved.
// cspell: words amqp eventhub eventhubs eventdata

#![recursion_limit = "128"]

pub(crate) mod common;
pub mod consumer;
pub mod error;
pub mod producer;

pub mod models {

    use azure_core_amqp::{
        messaging::{
            AmqpAnnotationKey, AmqpMessage, AmqpMessageBody, AmqpMessageId, AmqpMessageProperties,
        },
        value::AmqpValue,
    };
    use std::{
        collections::HashMap,
        fmt::{Debug, Display, Formatter},
    };
    use tracing::warn;

    #[derive(Debug)]
    pub struct EventHubProperties {
        pub name: String,
        pub created_on: std::time::SystemTime,
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
    /// ```rust
    /// use azure_messaging_eventhubs::models::EventHubPartitionProperties;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// let partition_properties = EventHubPartitionProperties {
    ///     id: "0".to_string(),
    ///     eventhub: "example-hub".to_string(),
    ///     beginning_sequence_number: 0,
    ///     last_enqueued_sequence_number: 100,
    ///     last_enqueued_offset: "12345".to_string(),
    ///     last_enqueued_time_utc: SystemTime::now(),
    ///     is_empty: false,
    /// };
    /// ```
    ///
    #[derive(Debug)]
    pub struct EventHubPartitionProperties {
        pub id: String,
        pub eventhub: String,
        pub beginning_sequence_number: i64,
        pub last_enqueued_sequence_number: i64,
        pub last_enqueued_offset: String,
        pub last_enqueued_time_utc: std::time::SystemTime,
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
        Binary(Vec<u8>),
        String(String),
        Ulong(u64),
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

    #[derive(Debug, Default, PartialEq, Clone)]
    pub enum StartLocation {
        Offset(String),
        SequenceNumber(i64),
        EnqueuedTime(std::time::SystemTime),
        Earliest,
        #[default]
        Latest,
    }

    const ENQUEUED_TIME_ANNOTATION: &str = "amqp.annotation.x-opt-enqueued-time";
    const OFFSET_ANNOTATION: &str = "amqp.annotation.x-opt-offset";
    const SEQUENCE_NUMBER_ANNOTATION: &str = "amqp.annotation.x-opt-sequence-number";

    #[derive(Debug, PartialEq, Clone)]
    pub struct StartPosition {
        location: StartLocation,
        inclusive: bool,
    }

    impl StartPosition {
        pub fn builder() -> builders::StartPositionBuilder {
            builders::StartPositionBuilder::new()
        }
        pub fn start_expression(position: &Option<StartPosition>) -> String {
            if let Some(position) = position {
                let mut greater_than: &str = ">";
                if position.inclusive {
                    greater_than = ">=";
                }
                match &position.location {
                    StartLocation::Offset(offset) => {
                        format!("{} {}'{}'", OFFSET_ANNOTATION, greater_than, offset)
                    }
                    StartLocation::SequenceNumber(sequence_number) => {
                        format!(
                            "{} {}'{}'",
                            SEQUENCE_NUMBER_ANNOTATION, greater_than, sequence_number
                        )
                    }
                    StartLocation::EnqueuedTime(enqueued_time) => {
                        let enqueued_time = enqueued_time
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("Time went backwards")
                            .as_millis();
                        format!(
                            "{} {}'{}'",
                            ENQUEUED_TIME_ANNOTATION, greater_than, enqueued_time
                        )
                    }
                    StartLocation::Earliest => "amqp.annotation.x-opt-offset > '-1'".to_string(),
                    StartLocation::Latest => "amqp.annotation.x-opt-offset > '@latest'".to_string(),
                }
            } else {
                "amqp.annotation.x-opt-offset > '@latest'".to_string()
            }
        }
    }

    pub struct EventData {
        body: Option<Vec<u8>>,
        content_type: Option<String>,
        correlation_id: Option<MessageId>,
        message_id: Option<MessageId>,
        properties: Option<HashMap<String, AmqpValue>>,
    }

    impl EventData {
        pub fn builder() -> builders::EventDataBuilder {
            builders::EventDataBuilder::new()
        }

        pub fn properties(&self) -> Option<&HashMap<String, AmqpValue>> {
            self.properties.as_ref()
        }

        pub fn body(&self) -> Option<&[u8]> {
            self.body.as_ref().map(|b| b.as_slice())
        }

        pub fn content_type(&self) -> Option<&str> {
            self.content_type.as_deref()
        }

        pub fn correlation_id(&self) -> Option<&MessageId> {
            self.correlation_id.as_ref()
        }

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

    impl From<EventData> for AmqpMessage {
        fn from(event_data: EventData) -> Self {
            let mut message_properties_builder = AmqpMessageProperties::builder();
            let mut message_builder = AmqpMessage::builder();
            if let Some(content_type) = event_data.content_type {
                message_properties_builder =
                    message_properties_builder.with_content_type(content_type);
            }
            if let Some(correlation_id) = event_data.correlation_id {
                message_properties_builder =
                    message_properties_builder.with_correlation_id(correlation_id);
            }
            if let Some(message_id) = event_data.message_id {
                message_properties_builder = message_properties_builder.with_message_id(message_id);
            }
            if let Some(properties) = event_data.properties {
                message_builder =
                    message_builder.with_properties(message_properties_builder.build());
                for (key, value) in properties {
                    message_builder = message_builder.add_application_property(key, value);
                }
            }
            message_builder.build()
        }
    }

    pub struct ReceivedEventData {
        message: AmqpMessage,
        event_data: EventData,
        enqueued_time: std::time::SystemTime,
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
        pub fn raw_amqp_message(&self) -> &AmqpMessage {
            &self.message
        }

        pub fn event_data(&self) -> &EventData {
            &self.event_data
        }

        pub fn enqueued_time(&self) -> std::time::SystemTime {
            self.enqueued_time
        }

        pub fn offset(&self) -> &str {
            &self.offset
        }

        pub fn sequence_number(&self) -> i64 {
            self.sequence_number
        }

        pub fn partition_key(&self) -> &str {
            &self.partition_key
        }

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
                if let Some(content_type) = properties.content_type() {
                    event_data_builder = event_data_builder
                        .with_content_type(Into::<String>::into(content_type.clone()));
                }
                if let Some(correlation_id) = properties.correlation_id() {
                    event_data_builder =
                        event_data_builder.with_correlation_id(correlation_id.clone());
                }
                if let Some(message_id) = properties.message_id() {
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
            let mut enqueued_time: std::time::SystemTime = std::time::SystemTime::now();
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

    pub mod builders {
        use super::*;

        pub struct EventDataBuilder {
            event_data: EventData,
        }

        impl EventDataBuilder {
            pub fn new() -> Self {
                Self {
                    event_data: EventData {
                        body: None,
                        content_type: None,
                        correlation_id: None,
                        message_id: None,
                        properties: None,
                    },
                }
            }

            pub fn with_body(mut self, body: Vec<u8>) -> Self {
                self.event_data.body = Some(body);
                self
            }

            pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
                self.event_data.content_type = Some(content_type.into());
                self
            }

            pub fn with_correlation_id(mut self, correlation_id: impl Into<MessageId>) -> Self {
                self.event_data.correlation_id = Some(correlation_id.into());
                self
            }

            pub fn with_message_id(mut self, message_id: impl Into<MessageId>) -> Self {
                self.event_data.message_id = Some(message_id.into());
                self
            }

            pub fn add_property(
                mut self,
                key: impl Into<String>,
                value: impl Into<AmqpValue>,
            ) -> Self {
                if let Some(mut properties) = self.event_data.properties {
                    properties.insert(key.into(), value.into());
                    self.event_data.properties = Some(properties);
                } else {
                    let mut properties = HashMap::new();
                    properties.insert(key.into(), value.into());
                    self.event_data.properties = Some(properties);
                }
                self
            }

            pub fn build(self) -> EventData {
                self.event_data
            }
        }

        pub struct StartPositionBuilder {
            position: StartPosition,
        }

        impl StartPositionBuilder {
            pub(super) fn new() -> Self {
                Self {
                    position: StartPosition {
                        location: StartLocation::Latest,
                        inclusive: false,
                    },
                }
            }

            pub fn with_earliest_location(mut self) -> Self {
                self.position.location = StartLocation::Earliest;
                self
            }

            pub fn with_latest_location(mut self) -> Self {
                self.position.location = StartLocation::Latest;
                self
            }

            pub fn with_sequence_number(mut self, sequence_number: i64) -> Self {
                self.position.location = StartLocation::SequenceNumber(sequence_number);
                self
            }

            pub fn with_enqueued_time(mut self, enqueued_time: std::time::SystemTime) -> Self {
                self.position.location = StartLocation::EnqueuedTime(enqueued_time);
                self
            }

            pub fn with_offset(mut self, offset: String) -> Self {
                self.position.location = StartLocation::Offset(offset);
                self
            }

            pub fn inclusive(mut self) -> Self {
                self.position.inclusive = true;
                self
            }

            pub fn build(self) -> StartPosition {
                self.position
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use tracing::info;
        static INIT_LOGGING: std::sync::Once = std::sync::Once::new();

        #[test]
        fn setup() {
            INIT_LOGGING.call_once(|| {
                println!("Setting up test logger...");

                tracing_subscriber::fmt::init();
            });
        }

        #[test]
        fn test_start_position_builder_with_sequence_number() {
            setup();
            let sequence_number = 12345i64;
            let start_position = StartPosition::builder()
                .with_sequence_number(sequence_number)
                .build();
            assert_eq!(
                start_position.location,
                StartLocation::SequenceNumber(sequence_number)
            );
            assert_eq!(
                StartPosition::start_expression(&Some(start_position)),
                "amqp.annotation.x-opt-sequence-number >'12345'"
            );

            let start_position = StartPosition::builder()
                .with_sequence_number(sequence_number)
                .inclusive()
                .build();
            assert_eq!(
                StartPosition::start_expression(&Some(start_position)),
                "amqp.annotation.x-opt-sequence-number >='12345'"
            );
        }

        #[test]
        fn test_start_position_builder_with_enqueued_time() {
            setup();
            let enqueued_time = std::time::SystemTime::now();
            let start_position = StartPosition::builder()
                .with_enqueued_time(enqueued_time)
                .build();
            info!("enqueued_time: {:?}", enqueued_time);
            info!(
                "enqueued_time: {:?}",
                enqueued_time.duration_since(std::time::UNIX_EPOCH)
            );
            info!(
                "enqueued_time: {:?}",
                enqueued_time
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            );
            assert_eq!(
                start_position.location,
                StartLocation::EnqueuedTime(enqueued_time)
            );
            assert_eq!(
                StartPosition::start_expression(&Some(start_position)),
                format!(
                    "amqp.annotation.x-opt-enqueued-time >'{}'",
                    enqueued_time
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis()
                )
            );

            let start_position = StartPosition::builder()
                .with_enqueued_time(enqueued_time)
                .inclusive()
                .build();
            assert_eq!(
                StartPosition::start_expression(&Some(start_position)),
                format!(
                    "amqp.annotation.x-opt-enqueued-time >='{}'",
                    enqueued_time
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis()
                )
            );
        }

        #[test]
        fn test_start_position_builder_with_offset() {
            setup();
            let offset = "12345".to_string();
            let start_position = StartPosition::builder().with_offset(offset.clone()).build();
            assert_eq!(
                start_position.location,
                StartLocation::Offset(offset.clone())
            );
            assert_eq!(
                "amqp.annotation.x-opt-offset >'12345'",
                StartPosition::start_expression(&Some(start_position)),
            );

            let start_position = StartPosition::builder()
                .with_offset(offset.clone())
                .inclusive()
                .build();
            assert_eq!(
                "amqp.annotation.x-opt-offset >='12345'",
                StartPosition::start_expression(&Some(start_position)),
            );
        }

        #[test]
        fn test_start_position_builder_inclusive() {
            setup();
            let start_position = StartPosition::builder().inclusive().build();
            assert!(start_position.inclusive);
            let start_position = StartPosition::builder().build();
            assert!(!start_position.inclusive);
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
        let event_data = EventData::builder().with_content_type(content_type).build();

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
            .with_content_type(content_type)
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
