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
    fn body(&self) -> &[u8];
    fn body_as_string(&self) -> Result<String>;
    fn content_type(&self) -> Option<&String>;
    fn correlation_id(&self) -> Option<&String>;
    fn message_id(&self) -> Option<&String>;
    fn new<T: Into<Vec<u8>>>(body: T) -> Self;
    fn properties(&self) -> &HashMap<String, String>;
    fn property(&self, key: &str) -> Option<&String>;
    fn reply_to(&self) -> Option<&String>;
    fn reply_to_session_id(&self) -> Option<&String>;
    fn scheduled_enqueue_time(&self) -> Option<OffsetDateTime>;
    fn session_id(&self) -> Option<&String>;
    fn set_content_type<impl Into<String>: Into<String>>(&mut self, content_type: impl Into<String>);
    fn set_correlation_id<impl Into<String>: Into<String>>(&mut self, correlation_id: impl Into<String>);
    fn set_message_id<impl Into<String>: Into<String>>(&mut self, message_id: impl Into<String>);
    fn set_property<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(&mut self, key: impl Into<String>, value: impl Into<String>);
    fn set_reply_to<impl Into<String>: Into<String>>(&mut self, reply_to: impl Into<String>);
    fn set_reply_to_session_id<impl Into<String>: Into<String>>(&mut self, reply_to_session_id: impl Into<String>);
    fn set_scheduled_enqueue_time(&mut self, scheduled_enqueue_time: OffsetDateTime);
    fn set_session_id<impl Into<String>: Into<String>>(&mut self, session_id: impl Into<String>);
    fn set_subject<impl Into<String>: Into<String>>(&mut self, subject: impl Into<String>);
    fn set_time_to_live(&mut self, time_to_live: Duration);
    fn subject(&self) -> Option<&String>;
    fn time_to_live(&self) -> Option<Duration>;
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
    fn count(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn maximum_size_in_bytes(&self) -> usize;
    fn size_in_bytes(&self) -> usize;
    fn try_add_message(&mut self, message: Message) -> bool;
}
#[derive(Clone, Debug)]
pub struct ReceivedMessage {
}
impl ReceivedMessage {
    fn body(&self) -> &[u8];
    fn body_as_string(&self) -> Result<String>;
    fn correlation_id(&self) -> Option<&String>;
    fn delivery_count(&self) -> Option<u32>;
    fn enqueued_time_utc(&self) -> Option<OffsetDateTime>;
    fn lock_token(&self) -> Option<Uuid>;
    fn message_id(&self) -> Option<&String>;
    fn properties(&self) -> &HashMap<String, String>;
    fn property(&self, key: &str) -> Option<&String>;
    fn sequence_number(&self) -> Option<i64>;
    fn session_id(&self) -> Option<&String>;
    fn system_properties(&self) -> &SystemProperties;
}
#[derive(Debug)]
pub struct ServiceBusError {
}
impl ServiceBusError {
    fn kind(&self) -> &ErrorKind;
    fn message(&self) -> &str;
    fn new<impl Into<String>: Into<String>>(kind: ErrorKind, message: impl Into<String>) -> Self;
    fn with_source<impl Into<String>: Into<String>>(kind: ErrorKind, message: impl Into<String>, source: Box<dyn std::error::Error + 'static>) -> Self;
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
        fn builder() -> ServiceBusClientBuilder;
        async fn close(&self) -> Result<()>;
        async fn create_receiver(&self, queue_name: &str, options: Option<CreateReceiverOptions>) -> Result<Receiver>;
        async fn create_receiver_for_subscription(&self, topic_name: &str, subscription_name: &str, options: Option<CreateReceiverOptions>) -> Result<Receiver>;
        async fn create_sender(&self, queue_or_topic_name: &str, _options: Option<CreateSenderOptions>) -> Result<Sender>;
        fn fully_qualified_namespace(&self) -> &str;
    }
    #[derive(Default)]
    pub struct ServiceBusClientBuilder {
    }
    impl ServiceBusClientBuilder {
        fn new() -> Self;
        async fn open(self, fully_qualified_namespace: &str, credential: Arc<dyn TokenCredential>) -> Result<ServiceBusClient>;
        fn with_application_id(self, application_id: String) -> Self;
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
        async fn abandon_message(&self, message: &ReceivedMessage, _options: Option<AbandonMessageOptions>) -> Result<()>;
        async fn close(&self) -> Result<()>;
        async fn complete_message(&self, message: &ReceivedMessage, _options: Option<CompleteMessageOptions>) -> Result<()>;
        async fn dead_letter_message(&self, message: &ReceivedMessage, options: Option<DeadLetterMessageOptions>) -> Result<()>;
        async fn defer_message(&self, message: &ReceivedMessage, options: Option<DeferMessageOptions>) -> Result<()>;
        fn entity_name(&self) -> &str;
        async fn peek_messages(&self, max_count: u32, options: Option<PeekMessagesOptions>) -> Result<Vec<ReceivedMessage>>;
        async fn receive_deferred_message(&self, sequence_number: i64, _options: Option<ReceiveDeferredMessagesOptions>) -> Result<Option<ReceivedMessage>>;
        async fn receive_deferred_messages(&self, sequence_numbers: &[i64], _options: Option<ReceiveDeferredMessagesOptions>) -> Result<Vec<ReceivedMessage>>;
        async fn receive_message(&self, options: Option<ReceiveMessageOptions>) -> Result<Option<ReceivedMessage>>;
        async fn receive_messages(&self, max_message_count: usize, options: Option<ReceiveMessageOptions>) -> Result<Vec<ReceivedMessage>>;
        fn receive_mode(&self) -> ReceiveMode;
        async fn renew_message_lock(&self, message: &ReceivedMessage, _options: Option<RenewMessageLockOptions>) -> Result<OffsetDateTime>;
        fn subscription_name(&self) -> Option<&str>;
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
        async fn cancel_scheduled_message(&self, sequence_number: i64, _options: Option<CancelScheduledMessagesOptions>) -> Result<()>;
        async fn close(&self) -> Result<()>;
        async fn create_message_batch(&self, options: Option<CreateMessageBatchOptions>) -> crate::Result<crate::MessageBatch>;
        fn entity_name(&self) -> &str;
        async fn schedule_message(&self, message: Message, scheduled_enqueue_time: time::OffsetDateTime, _options: Option<ScheduleMessageOptions>) -> Result<i64>;
        async fn send_message(&self, message: Message, _options: Option<SendMessageOptions>) -> Result<()>;
        async fn send_message_batch(&self, batch: crate::MessageBatch, _options: Option<SendMessageBatchOptions>) -> crate::Result<()>;
        async fn send_messages(&self, messages: Vec<Message>, _options: Option<SendMessagesOptions>) -> Result<()>;
    }
    impl Drop for Sender {
        fn drop(&mut self);
    }
}
```
