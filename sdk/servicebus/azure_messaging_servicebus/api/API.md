# azure_messaging_servicebus

- **Description**: Rust client for Azure Service Bus
- **Edition**: 2021

## Features

- `default`
  - `azure_core_amqp/default`

```rust
#![recursion_limit = "128"]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![attr = RecursionLimit {limit:128}]
pub use azure_messaging_servicebus::receiver::AbandonMessageOptions;
pub use azure_messaging_servicebus::sender::CancelScheduledMessagesOptions;
pub use azure_messaging_servicebus::receiver::CompleteMessageOptions;
pub use azure_messaging_servicebus::sender::CreateMessageBatchOptions;
pub use azure_messaging_servicebus::client::CreateReceiverOptions;
pub use azure_messaging_servicebus::client::CreateSenderOptions;
pub use azure_messaging_servicebus::receiver::DeadLetterMessageOptions;
pub use azure_messaging_servicebus::receiver::DeferMessageOptions;
pub use azure_messaging_servicebus::receiver::PeekMessagesOptions;
pub use azure_messaging_servicebus::receiver::ReceiveDeferredMessagesOptions;
pub use azure_messaging_servicebus::receiver::ReceiveMessageOptions;
pub use azure_messaging_servicebus::receiver::ReceiveMode;
pub use azure_messaging_servicebus::receiver::Receiver;
pub use azure_messaging_servicebus::receiver::RenewMessageLockOptions;
pub use azure_messaging_servicebus::sender::ScheduleMessageOptions;
pub use azure_messaging_servicebus::sender::ScheduleMessagesOptions;
pub use azure_messaging_servicebus::sender::SendMessageBatchOptions;
pub use azure_messaging_servicebus::sender::SendMessageOptions;
pub use azure_messaging_servicebus::sender::SendMessagesOptions;
pub use azure_messaging_servicebus::sender::Sender;
pub use azure_messaging_servicebus::client::ServiceBusClient;
pub use azure_messaging_servicebus::client::ServiceBusClientBuilder;
pub use azure_messaging_servicebus::client::ServiceBusClientOptions;
pub use azure_messaging_servicebus::client::SubQueue;
#[derive(Clone, Debug)]
pub struct Message {
}
impl Message {
    pub fn body(&self) -> &[u8];
    pub fn body_as_string(&self) -> Result<String>;
    pub fn content_type(&self) -> Option<&String>;
    pub fn correlation_id(&self) -> Option<&String>;
    pub fn message_id(&self) -> Option<&String>;
    pub fn new<T: Into<Vec<u8>>>(body: T) -> Self;
    pub fn properties(&self) -> &HashMap<String, String>;
    pub fn property(&self, key: &str) -> Option<&String>;
    pub fn reply_to(&self) -> Option<&String>;
    pub fn reply_to_session_id(&self) -> Option<&String>;
    pub fn scheduled_enqueue_time(&self) -> Option<OffsetDateTime>;
    pub fn session_id(&self) -> Option<&String>;
    pub fn set_content_type<impl Into<String>: Into<String>>(&mut self, content_type: impl Into<String>);
    pub fn set_correlation_id<impl Into<String>: Into<String>>(&mut self, correlation_id: impl Into<String>);
    pub fn set_message_id<impl Into<String>: Into<String>>(&mut self, message_id: impl Into<String>);
    pub fn set_property<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(&mut self, key: impl Into<String>, value: impl Into<String>);
    pub fn set_reply_to<impl Into<String>: Into<String>>(&mut self, reply_to: impl Into<String>);
    pub fn set_reply_to_session_id<impl Into<String>: Into<String>>(&mut self, reply_to_session_id: impl Into<String>);
    pub fn set_scheduled_enqueue_time(&mut self, scheduled_enqueue_time: OffsetDateTime);
    pub fn set_session_id<impl Into<String>: Into<String>>(&mut self, session_id: impl Into<String>);
    pub fn set_subject<impl Into<String>: Into<String>>(&mut self, subject: impl Into<String>);
    pub fn set_time_to_live(&mut self, time_to_live: Duration);
    pub fn subject(&self) -> Option<&String>;
    pub fn time_to_live(&self) -> Option<Duration>;
}
impl From<&str> for Message {
    fn from(body: &str) -> Self;
}
impl From<Message> for azure_core_amqp::AmqpMessage {
    fn from(message: Message) -> Self;
}
impl From<String> for Message {
    fn from(body: String) -> Self;
}
#[derive(Debug)]
pub struct MessageBatch {
}
impl MessageBatch {
    const DEFAULT_MAX_SIZE_BYTES: usize = _;
    pub fn count(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn maximum_size_in_bytes(&self) -> usize;
    pub fn size_in_bytes(&self) -> usize;
    pub fn try_add_message(&mut self, message: Message) -> bool;
}
#[derive(Clone, Debug)]
pub struct ReceivedMessage {
}
impl ReceivedMessage {
    pub fn body(&self) -> &[u8];
    pub fn body_as_string(&self) -> Result<String>;
    pub fn correlation_id(&self) -> Option<&String>;
    pub fn delivery_count(&self) -> Option<u32>;
    pub fn enqueued_time_utc(&self) -> Option<OffsetDateTime>;
    pub fn lock_token(&self) -> Option<Uuid>;
    pub fn message_id(&self) -> Option<&String>;
    pub fn properties(&self) -> &HashMap<String, String>;
    pub fn property(&self, key: &str) -> Option<&String>;
    pub fn sequence_number(&self) -> Option<i64>;
    pub fn session_id(&self) -> Option<&String>;
    pub fn system_properties(&self) -> &SystemProperties;
}
#[derive(Debug)]
pub struct ServiceBusError {
}
impl ServiceBusError {
    pub fn kind(&self) -> &ErrorKind;
    pub fn message(&self) -> &str;
    pub fn new<impl Into<String>: Into<String>>(kind: ErrorKind, message: impl Into<String>) -> Self;
    pub fn with_source<impl Into<String>: Into<String>>(kind: ErrorKind, message: impl Into<String>, source: Box<dyn std::error::Error + 'static>) -> Self;
}
impl Display for ServiceBusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
impl Error for ServiceBusError {
    fn source(&self) -> Option<&dyn std::error::Error + 'static>;
}
impl From<AmqpError> for ServiceBusError {
    fn from(error: azure_core_amqp::AmqpError) -> Self;
}
impl From<Error> for ServiceBusError {
    fn from(error: azure_core::error::Error) -> Self;
}
#[derive(Debug, Eq, PartialEq)]
pub enum ErrorKind {
    Amqp,
    Cancelled,
    EntityNotFound,
    InvalidRequest,
    MessageLockLost,
    MessageNotFound,
    MessageSizeExceeded,
    QuotaExceeded,
    RequestTimeout,
    ServiceBusClosed,
    SessionLockLost,
    Unknown,
}
impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
pub type Result<T> = std::result::Result<T, ServiceBusError>;
pub mod client {
    #[derive(Clone)]
    pub struct CreateReceiverOptions {
        pub receive_mode: crate::ReceiveMode,
        pub sub_queue: Option<SubQueue>,
    }
    impl Default for CreateReceiverOptions {
        fn default() -> Self;
    }
    #[derive(Clone, Default)]
    pub struct CreateSenderOptions;
    pub struct ServiceBusClient {
    }
    impl ServiceBusClient {
        pub fn builder() -> ServiceBusClientBuilder;
        pub async fn close(&self) -> Result<()>;
        pub async fn create_receiver(&self, queue_name: &str, options: Option<CreateReceiverOptions>) -> Result<Receiver>;
        pub async fn create_receiver_for_subscription(&self, topic_name: &str, subscription_name: &str, options: Option<CreateReceiverOptions>) -> Result<Receiver>;
        pub async fn create_sender(&self, queue_or_topic_name: &str, _options: Option<CreateSenderOptions>) -> Result<Sender>;
        pub fn fully_qualified_namespace(&self) -> &str;
    }
    #[derive(Default)]
    pub struct ServiceBusClientBuilder {
    }
    impl ServiceBusClientBuilder {
        pub fn new() -> Self;
        pub async fn open(self, fully_qualified_namespace: &str, credential: Arc<dyn TokenCredential>) -> Result<ServiceBusClient>;
        pub fn with_application_id(self, application_id: String) -> Self;
    }
    #[derive(Clone, Debug)]
    pub struct ServiceBusClientOptions {
        pub api_version: String,
        pub application_id: Option<String>,
    }
    impl Default for ServiceBusClientOptions {
        fn default() -> Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum SubQueue {
        DeadLetter,
        Transfer,
    }
}
pub mod models {
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "PascalCase")]
    pub struct NamespaceProperties {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub namespace_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub namespace_type: Option<NamespaceType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub modified_at: Option<time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub messaging_sku: Option<MessagingSku>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub messaging_units: Option<i32>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "PascalCase")]
    pub struct QueueRuntimeProperties {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub queue_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size_in_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total_message_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub active_message_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dead_letter_message_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scheduled_message_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfer_message_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfer_dead_letter_message_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub accessed_at: Option<time::OffsetDateTime>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "PascalCase")]
    pub struct SubscriptionRuntimeProperties {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub topic_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subscription_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total_message_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub active_message_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dead_letter_message_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfer_message_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfer_dead_letter_message_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub accessed_at: Option<time::OffsetDateTime>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "PascalCase")]
    pub struct TopicRuntimeProperties {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub topic_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size_in_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subscription_count: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scheduled_message_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub accessed_at: Option<time::OffsetDateTime>,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "PascalCase")]
    pub enum AccessRights {
        Manage,
        Send,
        Listen,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "PascalCase")]
    pub enum EntityState {
        Active,
        Disabled,
        SendDisabled,
        ReceiveDisabled,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "PascalCase")]
    pub enum EntityStatus {
        Active,
        Creating,
        Deleting,
        Disabled,
        ReceiveDisabled,
        Renaming,
        Restoring,
        SendDisabled,
        Unknown,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "PascalCase")]
    pub enum MessagingSku {
        Basic,
        Standard,
        Premium,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "PascalCase")]
    pub enum NamespaceType {
        Messaging,
        Mixed,
        NotificationHub,
        Relay,
    }
}
pub mod receiver {
    #[derive(Clone, Debug, Default)]
    pub struct AbandonMessageOptions {
        pub properties_to_modify: Option<std::collections::HashMap<String, String>>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct CompleteMessageOptions;
    #[derive(Clone, Debug, Default)]
    pub struct DeadLetterMessageOptions {
        pub reason: Option<String>,
        pub error_description: Option<String>,
        pub properties_to_modify: Option<std::collections::HashMap<String, String>>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct DeferMessageOptions {
        pub properties_to_modify: Option<std::collections::HashMap<String, String>>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct PeekMessagesOptions {
        pub from_sequence_number: Option<i64>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct ReceiveDeferredMessagesOptions {
    }
    #[derive(Clone, Debug)]
    pub struct ReceiveMessageOptions {
        pub max_message_count: u32,
        pub max_wait_time: Option<azure_core::time::Duration>,
    }
    impl Default for ReceiveMessageOptions {
        fn default() -> Self;
    }
    pub struct Receiver {
    }
    impl Receiver {
        pub async fn abandon_message(&self, message: &ReceivedMessage, _options: Option<AbandonMessageOptions>) -> Result<()>;
        pub async fn close(&self) -> Result<()>;
        pub async fn complete_message(&self, message: &ReceivedMessage, _options: Option<CompleteMessageOptions>) -> Result<()>;
        pub async fn dead_letter_message(&self, message: &ReceivedMessage, options: Option<DeadLetterMessageOptions>) -> Result<()>;
        pub async fn defer_message(&self, message: &ReceivedMessage, options: Option<DeferMessageOptions>) -> Result<()>;
        pub fn entity_name(&self) -> &str;
        pub async fn peek_messages(&self, max_count: u32, options: Option<PeekMessagesOptions>) -> Result<Vec<ReceivedMessage>>;
        pub async fn receive_deferred_message(&self, sequence_number: i64, _options: Option<ReceiveDeferredMessagesOptions>) -> Result<Option<ReceivedMessage>>;
        pub async fn receive_deferred_messages(&self, sequence_numbers: &[i64], _options: Option<ReceiveDeferredMessagesOptions>) -> Result<Vec<ReceivedMessage>>;
        pub async fn receive_message(&self, options: Option<ReceiveMessageOptions>) -> Result<Option<ReceivedMessage>>;
        pub async fn receive_messages(&self, max_message_count: usize, options: Option<ReceiveMessageOptions>) -> Result<Vec<ReceivedMessage>>;
        pub fn receive_mode(&self) -> ReceiveMode;
        pub async fn renew_message_lock(&self, message: &ReceivedMessage, _options: Option<RenewMessageLockOptions>) -> Result<OffsetDateTime>;
        pub fn subscription_name(&self) -> Option<&str>;
    }
    impl Drop for Receiver {
        fn drop(&mut self);
    }
    #[derive(Clone, Debug, Default)]
    pub struct RenewMessageLockOptions;
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ReceiveMode {
        PeekLock,
        ReceiveAndDelete,
    }
}
pub mod sender {
    #[derive(Clone, Debug, Default)]
    pub struct CancelScheduledMessagesOptions;
    #[derive(Clone, Debug, Default)]
    pub struct CreateMessageBatchOptions {
        pub maximum_size_in_bytes: Option<usize>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct ScheduleMessageOptions;
    #[derive(Clone, Debug, Default)]
    pub struct ScheduleMessagesOptions;
    #[derive(Clone, Debug, Default)]
    pub struct SendMessageBatchOptions;
    #[derive(Clone, Debug, Default)]
    pub struct SendMessageOptions;
    #[derive(Clone, Debug, Default)]
    pub struct SendMessagesOptions;
    pub struct Sender {
    }
    impl Sender {
        pub async fn cancel_scheduled_message(&self, sequence_number: i64, _options: Option<CancelScheduledMessagesOptions>) -> Result<()>;
        pub async fn close(&self) -> Result<()>;
        pub async fn create_message_batch(&self, options: Option<CreateMessageBatchOptions>) -> crate::Result<crate::MessageBatch>;
        pub fn entity_name(&self) -> &str;
        pub async fn schedule_message(&self, message: Message, scheduled_enqueue_time: time::OffsetDateTime, _options: Option<ScheduleMessageOptions>) -> Result<i64>;
        pub async fn send_message(&self, message: Message, _options: Option<SendMessageOptions>) -> Result<()>;
        pub async fn send_message_batch(&self, batch: crate::MessageBatch, _options: Option<SendMessageBatchOptions>) -> crate::Result<()>;
        pub async fn send_messages(&self, messages: Vec<Message>, _options: Option<SendMessagesOptions>) -> Result<()>;
    }
    impl Drop for Sender {
        fn drop(&mut self);
    }
}
```
