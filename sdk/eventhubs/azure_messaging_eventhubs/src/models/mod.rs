// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell: ignore eventdata

mod event_data;

/// An AMQP Message sent to the eventhubs service.
pub type AmqpMessage = azure_core_amqp::messaging::AmqpMessage;

/// The body of an AMQP Message sent to the eventhubs service.
pub type AmqpMessageBody = azure_core_amqp::messaging::AmqpMessageBody;

/// An AMQP Value.
pub type AmqpValue = azure_core_amqp::value::AmqpValue;

/// An event received from an Event Hub.
pub type ReceivedEventData = event_data::ReceivedEventData;

/// Event sent to an Event Hub.
pub type EventData = event_data::EventData;

use azure_core_amqp::messaging::AmqpMessageId;
use std::fmt::Debug;

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

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core_amqp::messaging::AmqpMessageId;

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
}
