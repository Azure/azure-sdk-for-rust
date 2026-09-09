# azure_messaging_eventhubs

- **Description**: Rust client for Azure Eventhubs Service
- **Edition**: 2021

## Features

- `default`
  - `azure_core_amqp/default`
- `fe2o3_amqp`
- `fe2o3_amqp_rustls`
- `fe2o3_amqp_ws`
- `fe2o3_amqp_ws_rustls`
- `in_memory_checkpoint_store`

```rust
#![cfg(feature = "in_memory_checkpoint_store")]
#![recursion_limit = "128"]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![attr = RecursionLimit {limit:128}]
pub use azure_messaging_eventhubs::error::EventHubsError;
pub use azure_messaging_eventhubs::error::Result;
pub struct BufferedProducerClient {
}
impl BufferedProducerClient {
    async fn abort(&self) -> Result<()>;
    fn buffered_event_count(&self, partition_id: &str) -> usize;
    fn builder() -> builders::BufferedProducerClientBuilder;
    async fn close(&self) -> Result<()>;
    async fn enqueue_event<impl Into<EventData>: Into<EventData>>(&self, event: impl Into<EventData>, options: Option<EnqueueEventOptions>) -> Result<()>;
    async fn enqueue_events<E, impl IntoIterator<Item = E>: IntoIterator<Item = E>>(&self, events: impl IntoIterator<Item = E>, options: Option<EnqueueEventOptions>) -> Result<()> where E: Into<EventData>;
    async fn flush(&self) -> Result<()>;
    fn total_buffered_event_count(&self) -> usize;
}
impl Debug for BufferedProducerClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
impl Drop for BufferedProducerClient {
    fn drop(&mut self);
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct ConnectionString {
    pub endpoint: String,
    pub fully_qualified_namespace: String,
    pub shared_access_key_name: Option<String>,
    pub shared_access_key: Option<azure_core::credentials::Secret>,
    pub shared_access_signature: Option<azure_core::credentials::Secret>,
    pub entity_path: Option<String>,
}
impl FromStr for ConnectionString {
    type Err = Error;
    fn from_str(connection_string: &str) -> Result<Self, <Self as >::Err>;
}
impl TryFrom<&Secret> for ConnectionString {
    type Error = Error;
    fn try_from(secret: &Secret) -> Result<Self, <Self as >::Error>;
}
pub struct ConsumerClient {
}
impl ConsumerClient {
    fn builder() -> builders::ConsumerClientBuilder;
    async fn close(self) -> Result<()>;
    async fn get_eventhub_properties(&self) -> Result<EventHubProperties>;
    async fn get_partition_properties(&self, partition_id: &str) -> Result<EventHubPartitionProperties>;
    async fn open_receiver_on_partition(&self, partition_id: String, options: Option<OpenReceiverOptions>) -> Result<EventReceiver>;
}
#[derive(Clone, Debug, Default)]
pub struct EnqueueEventOptions {
    pub partition_id: Option<String>,
    pub partition_key: Option<String>,
}
pub struct EventDataBatch<'a> {
}
impl<'a> EventDataBatch<'a> {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn size(&self) -> u64;
    fn try_add_amqp_message<impl Into<AmqpMessage>: Into<AmqpMessage>>(&self, message: impl Into<AmqpMessage>, options: Option<AddEventDataOptions>) -> Result<bool>;
    fn try_add_event_data<impl Into<EventData>: Into<EventData>>(&self, event_data: impl Into<EventData>, options: Option<AddEventDataOptions>) -> Result<bool>;
}
#[derive(Default)]
pub struct EventDataBatchOptions {
    pub max_size_in_bytes: Option<u64>,
    pub partition_key: Option<String>,
    pub partition_id: Option<String>,
}
pub struct EventProcessor {
}
impl EventProcessor {
    fn builder() -> builders::EventProcessorBuilder;
    async fn close(self) -> Result<()>;
    async fn next_partition_client(&self) -> Result<Arc<PartitionClient>>;
    async fn run(&self) -> Result<()>;
    async fn shutdown(&self) -> Result<()>;
}
impl Send for EventProcessor {
}
impl Sync for EventProcessor {
}
pub struct EventReceiver {
}
impl EventReceiver {
    async fn close(self) -> Result<()>;
    fn partition_id(&self) -> &str;
    fn stream_events(&self) -> impl Stream<Item = Result<ReceivedEventData>> + '_;
}
impl Drop for EventReceiver {
    fn drop(&mut self);
}
#[cfg(feature = "in_memory_checkpoint_store")]
pub struct InMemoryCheckpointStore {
}
#[cfg(feature = "in_memory_checkpoint_store")]
impl InMemoryCheckpointStore {
    fn new() -> Self;
    fn update_ownership(&self, ownership: &Ownership) -> Result<Ownership>;
}
#[cfg(feature = "in_memory_checkpoint_store")]
impl CheckpointStore for InMemoryCheckpointStore {
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn claim_ownership(&self, ownerships: &[Ownership]) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Vec<Ownership>>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn list_checkpoints(&self, namespace: &str, event_hub_name: &str, consumer_group: &str) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Vec<Checkpoint>>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn list_ownerships(&self, namespace: &str, event_hub_name: &str, consumer_group: &str) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Vec<Ownership>>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn update_checkpoint(&self, checkpoint: Checkpoint) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
}
#[cfg(feature = "in_memory_checkpoint_store")]
impl Default for InMemoryCheckpointStore {
    fn default() -> Self;
}
#[derive(Clone, Debug, Default)]
pub struct OpenReceiverOptions {
    pub owner_level: Option<i64>,
    pub prefetch: Option<u32>,
    pub start_position: Option<StartPosition>,
    pub receive_timeout: Option<azure_core::time::Duration>,
}
pub struct ProducerClient {
}
impl ProducerClient {
    fn builder() -> builders::ProducerClientBuilder;
    async fn close(self) -> Result<()>;
    async fn create_batch(&self, batch_options: Option<EventDataBatchOptions>) -> Result<EventDataBatch<'_>>;
    async fn get_eventhub_properties(&self) -> Result<EventHubProperties>;
    async fn get_partition_properties(&self, partition_id: &str) -> Result<EventHubPartitionProperties>;
    async fn send_batch(&self, batch: EventDataBatch<'_>, options: Option<SendBatchOptions>) -> Result<()>;
    async fn send_event<impl Into<EventData>: Into<EventData>>(&self, event: impl Into<EventData>, options: Option<SendEventOptions>) -> Result<()>;
    async fn send_message<M>(&self, message: M, options: Option<SendMessageOptions>) -> Result<()> where M: Into<AmqpMessage> + Debug + Send;
}
#[derive(Clone, Debug)]
pub struct RetryOptions {
    pub initial_delay: azure_core::time::Duration,
    pub max_delay: azure_core::time::Duration,
    pub max_total_elapsed: azure_core::time::Duration,
    pub max_retries: u32,
}
impl Default for RetryOptions {
    fn default() -> Self;
}
#[derive(Debug)]
#[non_exhaustive]
pub struct SendBatchFailedContext {
    pub partition_id: String,
    pub events: Vec<crate::models::EventData>,
    pub error: crate::EventHubsError,
}
#[derive(Clone, Debug, Default)]
pub struct SendBatchOptions {
}
#[derive(Debug)]
#[non_exhaustive]
pub struct SendBatchSucceededContext {
    pub partition_id: String,
    pub events: Vec<crate::models::EventData>,
}
#[derive(Debug, Default)]
pub struct SendEventOptions {
    pub partition_id: Option<String>,
}
impl From<SendEventOptions> for SendMessageOptions {
    fn from(options: SendEventOptions) -> Self;
}
#[derive(Debug, Default)]
pub struct SendMessageOptions {
    pub partition_id: Option<String>,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StartPosition {
    pub location: StartLocation,
    pub inclusive: bool,
}
#[derive(Clone, Copy, Debug)]
pub enum ProcessorStrategy {
    Balanced,
    Greedy,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub enum StartLocation {
    Offset(String),
    SequenceNumber(i64),
    EnqueuedTime(std::time::SystemTime),
    Earliest,
    #[default]
    Latest,
}
#[async_trait]
pub trait CheckpointStore: Send + Sync {
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn claim_ownership(&self, ownerships: &[Ownership]) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Vec<Ownership>>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn list_checkpoints(&self, namespace: &str, event_hub_name: &str, consumer_group: &str) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Vec<Checkpoint>>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn list_ownerships(&self, namespace: &str, event_hub_name: &str, consumer_group: &str) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Vec<Ownership>>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn update_checkpoint(&self, checkpoint: Checkpoint) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
}
pub mod builders {
    pub struct BufferedProducerClientBuilder {
    }
    impl BufferedProducerClientBuilder {
        async fn open(self, fully_qualified_namespace: &str, eventhub: &str, credential: Arc<dyn azure_core::credentials::TokenCredential>) -> Result<BufferedProducerClient>;
        async fn open_with_connection_string(self, connection_string: &str, eventhub: Option<&str>) -> Result<BufferedProducerClient>;
        fn with_application_id(self, application_id: String) -> Self;
        fn with_custom_endpoint(self, endpoint: String) -> Self;
        fn with_max_buffered_event_count_per_partition(self, count: usize) -> Self;
        fn with_max_wait_time(self, max_wait_time: Duration) -> Self;
        fn with_on_send_failed<F, Fut>(self, handler: F) -> Self where F: Fn(SendBatchFailedContext) -> Fut + Send + Sync + 'static, Fut: Future<Output = ()> + Send + 'static;
        fn with_on_send_succeeded<F, Fut>(self, handler: F) -> Self where F: Fn(SendBatchSucceededContext) -> Fut + Send + Sync + 'static, Fut: Future<Output = ()> + Send + 'static;
        fn with_retry_options(self, retry_options: RetryOptions) -> Self;
    }
    impl Default for BufferedProducerClientBuilder {
        fn default() -> Self;
    }
    #[derive(Default)]
    pub struct ConsumerClientBuilder {
    }
    impl ConsumerClientBuilder {
        async fn open(self, fully_qualified_namespace: &str, eventhub_name: String, credential: Arc<dyn azure_core::credentials::TokenCredential>) -> Result<super::ConsumerClient>;
        async fn open_with_connection_string(self, connection_string: &str, eventhub: Option<&str>) -> Result<super::ConsumerClient>;
        fn with_application_id(self, application_id: String) -> Self;
        fn with_consumer_group(self, consumer_group: String) -> Self;
        fn with_custom_endpoint(self, endpoint: String) -> Self;
        fn with_instance_id(self, instance_id: String) -> Self;
        fn with_retry_options(self, retry_options: RetryOptions) -> Self;
        fn with_transport(self, transport: AmqpTransport) -> Self;
    }
    #[derive(Default)]
    pub struct EventProcessorBuilder {
    }
    impl EventProcessorBuilder {
        async fn build(self, consumer_client: ConsumerClient, checkpoint_store: Arc<dyn CheckpointStore + Send + Sync>) -> Result<Arc<EventProcessor>>;
        fn with_load_balancing_strategy(self, load_balancing_strategy: super::ProcessorStrategy) -> Self;
        fn with_max_partition_count(self, max_partition_count: usize) -> Self;
        fn with_partition_expiration_duration(self, partition_expiration_duration: Duration) -> Self;
        fn with_prefetch(self, prefetch: u32) -> Self;
        fn with_start_positions(self, start_positions: StartPositions) -> Self;
        fn with_update_interval(self, update_interval: Duration) -> Self;
    }
    #[derive(Default)]
    pub struct ProducerClientBuilder {
    }
    impl ProducerClientBuilder {
        async fn open(self, fully_qualified_namespace: &str, eventhub: &str, credential: Arc<dyn azure_core::credentials::TokenCredential>) -> Result<ProducerClient>;
        async fn open_with_connection_string(self, connection_string: &str, eventhub: Option<&str>) -> Result<ProducerClient>;
        fn with_application_id(self, application_id: String) -> Self;
        fn with_custom_endpoint(self, endpoint: String) -> Self;
        fn with_retry_options(self, retry_options: RetryOptions) -> Self;
        fn with_transport(self, transport: AmqpTransport) -> Self;
    }
}
pub mod error {
    pub struct EventHubsError {
        pub kind: ErrorKind,
    }
    impl Debug for EventHubsError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl Display for EventHubsError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl Error for EventHubsError {
        fn source(&self) -> Option<&dyn std::error::Error + 'static>;
    }
    impl From<AmqpError> for EventHubsError {
        fn from(e: AmqpError) -> Self;
    }
    impl From<Error> for EventHubsError {
        fn from(e: azure_core::Error) -> Self;
    }
    impl From<EventHubsError> for azure_core::Error {
        fn from(value: EventHubsError) -> Self;
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum ErrorKind {
        SimpleMessage(std::borrow::Cow<'static, str>),
        InvalidManagementResponse,
        SendRejected(Option<azure_core_amqp::AmqpDescribedError>),
        AzureCore(azure_core::Error),
        InvalidBatchSize { requested: u64, max_allowed: u64 },
        AmqpError(azure_core_amqp::AmqpError),
        SendNotAccepted(std::borrow::Cow<'static, str>),
        ConsumerDisconnected(Option<azure_core_amqp::AmqpDescribedError>),
        MissingCheckpointMetadata { partition_id: String },
    }
    impl From<ErrorKind> for EventHubsError {
        fn from(kind: ErrorKind) -> Self;
    }
    pub type Result<T> = std::result::Result<T, EventHubsError>;
}
pub mod models {
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct AmqpMessage {
        pub body: AmqpMessageBody,
        pub header: Option<AmqpMessageHeader>,
        pub application_properties: Option<AmqpApplicationProperties>,
        pub message_annotations: Option<AmqpAnnotations>,
        pub delivery_annotations: Option<AmqpAnnotations>,
        pub properties: Option<AmqpMessageProperties>,
        pub footer: Option<AmqpAnnotations>,
    }
    impl AmqpMessage {
        fn add_message_annotation<impl Into<AmqpValue>: Into<AmqpValue>>(&mut self, name: AmqpSymbol, value: impl Into<AmqpValue>);
        fn builder() -> builders::AmqpMessageBuilder;
        fn serialize(message: &AmqpMessage) -> Result<Vec<u8>>;
        fn set_message_body<impl Into<AmqpMessageBody>: Into<AmqpMessageBody>>(&mut self, body: impl Into<AmqpMessageBody>);
        fn set_message_id<impl Into<AmqpMessageId>: Into<AmqpMessageId>>(&mut self, message_id: impl Into<AmqpMessageId>);
    }
    impl AsRef<AmqpMessage> for AmqpMessage {
        fn as_ref(&self) -> &AmqpMessage;
    }
    #[cfg(feature = "ffi")]
    impl Deserializable<AmqpMessage> for AmqpMessage {
        fn decode(data: &[u8]) -> Result<AmqpMessage>;
    }
    impl From<&AmqpMessage> for fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>> {
        fn from(message: &AmqpMessage) -> Self;
    }
    impl From<&Message<Body<EmptyBody>>> for crate::messaging::AmqpMessage {
        fn from(message: &fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::messaging::message::EmptyBody>>) -> Self;
    }
    impl From<&Message<Body<TransparentVec<Data>>>> for crate::messaging::AmqpMessage {
        fn from(message: &fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<TransparentVec<fe2o3_amqp_types::messaging::Data>>>) -> Self;
    }
    impl From<&Message<Body<Value>>> for crate::messaging::AmqpMessage {
        fn from(message: &fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<Value>>) -> Self;
    }
    impl From<AmqpList> for AmqpMessage {
        fn from(list: AmqpList) -> Self;
    }
    impl From<AmqpMessage> for fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>> {
        fn from(message: AmqpMessage) -> Self;
    }
    impl From<AmqpMessage> for fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::message::EmptyBody> {
        fn from(message: AmqpMessage) -> Self;
    }
    impl From<AmqpMessage> for fe2o3_amqp_types::messaging::Message<serde_amqp::extensions::TransparentVec<fe2o3_amqp_types::messaging::AmqpSequence<fe2o3_amqp_types::primitives::Value>>> {
        fn from(message: AmqpMessage) -> Self;
    }
    impl From<AmqpMessage> for fe2o3_amqp_types::messaging::Message<serde_amqp::extensions::TransparentVec<fe2o3_amqp_types::messaging::Data>> {
        fn from(message: AmqpMessage) -> Self;
    }
    impl From<AmqpValue> for AmqpMessage {
        fn from(value: AmqpValue) -> Self;
    }
    impl From<Message<Body<Value>>> for crate::messaging::AmqpMessage {
        fn from(message: fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<Value>>) -> Self;
    }
    impl From<Vec<u8>> for AmqpMessage {
        fn from(body: Vec<u8>) -> Self;
    }
    #[derive(Clone, Debug, Default)]
    pub struct Checkpoint {
        pub fully_qualified_namespace: String,
        pub event_hub_name: String,
        pub consumer_group: String,
        pub partition_id: String,
        pub offset: Option<String>,
        pub sequence_number: Option<i64>,
    }
    impl Checkpoint {
        fn get_checkpoint_blob_name(fully_qualified_namespace: &str, event_hub_name: &str, consumer_group: &str, partition_id: &str) -> Result<String>;
        fn get_checkpoint_blob_prefix_name(fully_qualified_namespace: &str, event_hub_name: &str, consumer_group: &str) -> Result<String>;
    }
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct EventData {
    }
    impl EventData {
        fn body(&self) -> Option<&[u8]>;
        fn builder() -> builders::EventDataBuilder;
        fn content_type(&self) -> Option<&str>;
        fn correlation_id(&self) -> Option<&MessageId>;
        fn message_id(&self) -> Option<&MessageId>;
        fn properties(&self) -> Option<&HashMap<String, AmqpSimpleValue>>;
    }
    impl From<EventData> for crate::models::AmqpMessage {
        fn from(event_data: EventData) -> Self;
    }
    impl<T> From<T> for EventData where T: Into<Vec<u8>> {
        fn from(body: T) -> Self;
    }
    #[derive(Debug)]
    pub struct EventHubPartitionProperties {
        pub id: String,
        pub eventhub: String,
        pub beginning_sequence_number: i64,
        pub last_enqueued_sequence_number: i64,
        pub last_enqueued_offset: String,
        pub last_enqueued_time_utc: Option<std::time::SystemTime>,
        pub is_empty: bool,
    }
    #[derive(Debug)]
    pub struct EventHubProperties {
        pub name: String,
        pub created_on: Option<std::time::SystemTime>,
        pub partition_ids: Vec<String>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct Ownership {
        pub fully_qualified_namespace: String,
        pub event_hub_name: String,
        pub consumer_group: String,
        pub partition_id: String,
        pub owner_id: Option<String>,
        pub etag: Option<azure_core::http::Etag>,
        pub last_modified_time: Option<azure_core::time::OffsetDateTime>,
    }
    impl Ownership {
        fn get_ownership_name(fully_qualified_namespace: &str, event_hub_name: &str, consumer_group: &str, partition_id: &str) -> Result<String>;
        fn get_ownership_prefix_name(fully_qualified_namespace: &str, event_hub_name: &str, consumer_group: &str) -> Result<String>;
    }
    pub struct ReceivedEventData {
    }
    impl ReceivedEventData {
        fn enqueued_time(&self) -> Option<SystemTime>;
        fn event_data(&self) -> &EventData;
        fn offset(&self) -> &Option<String>;
        fn partition_key(&self) -> &Option<String>;
        fn raw_amqp_message(&self) -> &AmqpMessage;
        fn sequence_number(&self) -> Option<i64>;
        fn system_properties(&self) -> &HashMap<String, AmqpValue>;
    }
    impl Debug for ReceivedEventData {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
    }
    impl From<AmqpMessage> for ReceivedEventData {
        fn from(message: AmqpMessage) -> Self;
    }
    #[derive(Debug, Default)]
    pub struct StartPositions {
        pub per_partition: std::collections::HashMap<String, crate::StartPosition>,
        pub default: crate::StartPosition,
    }
    #[derive(Clone, Debug, Default, PartialEq)]
    pub enum AmqpSimpleValue {
        #[default]
        Null,
        Boolean(bool),
        UByte(u8),
        Byte(i8),
        Char(char),
        UShort(u16),
        Short(i16),
        UInt(u32),
        Int(i32),
        ULong(u64),
        Long(i64),
        Float(f32),
        Double(f64),
        Decimal128([u8; 16]),
        Decimal64([u8; 8]),
        Decimal32([u8; 4]),
        TimeStamp(crate::value::AmqpTimestamp),
        Uuid(azure_core::Uuid),
        String(String),
        Symbol(crate::value::AmqpSymbol),
        Binary(Vec<u8>),
        Described(Box<crate::value::AmqpDescribed>),
    }
    impl From<&AmqpSimpleValue> for Vec<u8> {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for [u8; 16] {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for [u8; 4] {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for [u8; 8] {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for azure_core::Uuid {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for bool {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for char {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for crate::value::AmqpSymbol {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for crate::value::AmqpTimestamp {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for f32 {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for f64 {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for fe2o3_amqp_types::primitives::SimpleValue {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for i16 {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for i32 {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for i64 {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for i8 {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for std::string::String {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for u16 {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for u32 {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for u64 {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&AmqpSimpleValue> for u8 {
        fn from(v: &AmqpSimpleValue) -> Self;
    }
    impl From<&SimpleValue> for crate::simple_value::AmqpSimpleValue {
        fn from(v: &fe2o3_amqp_types::primitives::SimpleValue) -> Self;
    }
    impl From<&str> for AmqpSimpleValue {
        fn from(value: &str) -> Self;
    }
    impl From<AmqpSimpleValue> for Vec<u8> {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for [u8; 16] {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for [u8; 4] {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for [u8; 8] {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for azure_core::Uuid {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for bool {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for char {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for crate::value::AmqpSymbol {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for crate::value::AmqpTimestamp {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for f32 {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for f64 {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for fe2o3_amqp_types::primitives::SimpleValue {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for i16 {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for i32 {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for i64 {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for i8 {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for std::string::String {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for u16 {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for u32 {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for u64 {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSimpleValue> for u8 {
        fn from(v: AmqpSimpleValue) -> Self;
    }
    impl From<AmqpSymbol> for AmqpSimpleValue {
        fn from(v: AmqpSymbol) -> Self;
    }
    impl From<AmqpTimestamp> for AmqpSimpleValue {
        fn from(v: AmqpTimestamp) -> Self;
    }
    impl From<SimpleValue> for crate::simple_value::AmqpSimpleValue {
        fn from(v: fe2o3_amqp_types::primitives::SimpleValue) -> Self;
    }
    impl From<String> for AmqpSimpleValue {
        fn from(v: std::string::String) -> Self;
    }
    impl From<Uuid> for AmqpSimpleValue {
        fn from(v: Uuid) -> Self;
    }
    impl From<Vec<u8>> for AmqpSimpleValue {
        fn from(v: Vec<u8>) -> Self;
    }
    impl From<[u8; 16]> for AmqpSimpleValue {
        fn from(v: [u8; 16]) -> Self;
    }
    impl From<[u8; 4]> for AmqpSimpleValue {
        fn from(v: [u8; 4]) -> Self;
    }
    impl From<[u8; 8]> for AmqpSimpleValue {
        fn from(v: [u8; 8]) -> Self;
    }
    impl From<bool> for AmqpSimpleValue {
        fn from(v: bool) -> Self;
    }
    impl From<char> for AmqpSimpleValue {
        fn from(v: char) -> Self;
    }
    impl From<f32> for AmqpSimpleValue {
        fn from(v: f32) -> Self;
    }
    impl From<f64> for AmqpSimpleValue {
        fn from(v: f64) -> Self;
    }
    impl From<i16> for AmqpSimpleValue {
        fn from(v: i16) -> Self;
    }
    impl From<i32> for AmqpSimpleValue {
        fn from(v: i32) -> Self;
    }
    impl From<i64> for AmqpSimpleValue {
        fn from(v: i64) -> Self;
    }
    impl From<i8> for AmqpSimpleValue {
        fn from(v: i8) -> Self;
    }
    impl From<u16> for AmqpSimpleValue {
        fn from(v: u16) -> Self;
    }
    impl From<u32> for AmqpSimpleValue {
        fn from(v: u32) -> Self;
    }
    impl From<u64> for AmqpSimpleValue {
        fn from(v: u64) -> Self;
    }
    impl From<u8> for AmqpSimpleValue {
        fn from(v: u8) -> Self;
    }
    impl PartialEq<AmqpSimpleValue> for Vec<u8> {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for [u8; 16] {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for [u8; 4] {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for [u8; 8] {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for azure_core::Uuid {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for bool {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for char {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for crate::value::AmqpSymbol {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for crate::value::AmqpTimestamp {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for f32 {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for f64 {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for i16 {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for i32 {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for i64 {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for i8 {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for std::string::String {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for u16 {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for u32 {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for u64 {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSimpleValue> for u8 {
        fn eq(&self, other: &AmqpSimpleValue) -> bool;
    }
    impl PartialEq<AmqpSymbol> for AmqpSimpleValue {
        fn eq(&self, other: &AmqpSymbol) -> bool;
    }
    impl PartialEq<AmqpTimestamp> for AmqpSimpleValue {
        fn eq(&self, other: &AmqpTimestamp) -> bool;
    }
    impl PartialEq<String> for AmqpSimpleValue {
        fn eq(&self, other: &std::string::String) -> bool;
    }
    impl PartialEq<Uuid> for AmqpSimpleValue {
        fn eq(&self, other: &Uuid) -> bool;
    }
    impl PartialEq<Vec<u8>> for AmqpSimpleValue {
        fn eq(&self, other: &Vec<u8>) -> bool;
    }
    impl PartialEq<[u8; 16]> for AmqpSimpleValue {
        fn eq(&self, other: &[u8; 16]) -> bool;
    }
    impl PartialEq<[u8; 4]> for AmqpSimpleValue {
        fn eq(&self, other: &[u8; 4]) -> bool;
    }
    impl PartialEq<[u8; 8]> for AmqpSimpleValue {
        fn eq(&self, other: &[u8; 8]) -> bool;
    }
    impl PartialEq<bool> for AmqpSimpleValue {
        fn eq(&self, other: &bool) -> bool;
    }
    impl PartialEq<char> for AmqpSimpleValue {
        fn eq(&self, other: &char) -> bool;
    }
    impl PartialEq<f32> for AmqpSimpleValue {
        fn eq(&self, other: &f32) -> bool;
    }
    impl PartialEq<f64> for AmqpSimpleValue {
        fn eq(&self, other: &f64) -> bool;
    }
    impl PartialEq<i16> for AmqpSimpleValue {
        fn eq(&self, other: &i16) -> bool;
    }
    impl PartialEq<i32> for AmqpSimpleValue {
        fn eq(&self, other: &i32) -> bool;
    }
    impl PartialEq<i64> for AmqpSimpleValue {
        fn eq(&self, other: &i64) -> bool;
    }
    impl PartialEq<i8> for AmqpSimpleValue {
        fn eq(&self, other: &i8) -> bool;
    }
    impl PartialEq<u16> for AmqpSimpleValue {
        fn eq(&self, other: &u16) -> bool;
    }
    impl PartialEq<u32> for AmqpSimpleValue {
        fn eq(&self, other: &u32) -> bool;
    }
    impl PartialEq<u64> for AmqpSimpleValue {
        fn eq(&self, other: &u64) -> bool;
    }
    impl PartialEq<u8> for AmqpSimpleValue {
        fn eq(&self, other: &u8) -> bool;
    }
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub enum AmqpTransport {
        #[default]
        Tcp,
        WebSocket,
    }
    #[derive(Clone, Debug, Default, PartialEq)]
    pub enum AmqpValue {
        #[default]
        Null,
        Boolean(bool),
        UByte(u8),
        UShort(u16),
        UInt(u32),
        ULong(u64),
        Byte(i8),
        Short(i16),
        Int(i32),
        Long(i64),
        Float(f32),
        Double(f64),
        Char(char),
        TimeStamp(AmqpTimestamp),
        Uuid(azure_core::Uuid),
        Binary(Vec<u8>),
        String(String),
        Symbol(AmqpSymbol),
        Decimal128([u8; 16]),
        Decimal64([u8; 8]),
        Decimal32([u8; 4]),
        List(AmqpList),
        Map(AmqpOrderedMap<AmqpValue, AmqpValue>),
        Array(Vec<AmqpValue>),
        Described(Box<AmqpDescribed>),
        #[cfg(feature = "ffi")]
        Composite(Box<AmqpComposite>),
    }
    #[cfg(feature = "ffi")]
    impl Deserializable<AmqpValue> for AmqpValue {
        #[allow(unused_variables)]
        fn decode(data: &[u8]) -> Result<AmqpValue>;
    }
    impl From<&AmqpValue> for Vec<AmqpValue> {
        fn from(v: &AmqpValue) -> Self;
    }
    impl From<&AmqpValue> for Vec<u8> {
        fn from(v: &AmqpValue) -> Self;
    }
    impl From<&AmqpValue> for azure_core::Uuid {
        fn from(v: &AmqpValue) -> Self;
    }
    impl From<&AmqpValue> for bool {
        fn from(v: &AmqpValue) -> Self;
    }
    impl From<&AmqpValue> for char {
        fn from(v: &AmqpValue) -> Self;
    }
    impl From<&AmqpValue> for f32 {
        fn from(v: &AmqpValue) -> Self;
    }
    impl From<&AmqpValue> for f64 {
        fn from(v: &AmqpValue) -> Self;
    }
    impl From<&AmqpValue> for fe2o3_amqp_types::primitives::Value {
        fn from(value: &AmqpValue) -> Self;
    }
    impl From<&AmqpValue> for i16 {
        fn from(v: &AmqpValue) -> Self;
    }
    impl From<&AmqpValue> for i32 {
        fn from(v: &AmqpValue) -> Self;
    }
    impl From<&AmqpValue> for i64 {
        fn from(v: &AmqpValue) -> Self;
    }
    impl From<&AmqpValue> for i8 {
        fn from(v: &AmqpValue) -> Self;
    }
    impl From<&AmqpValue> for std::string::String {
        fn from(v: &AmqpValue) -> Self;
    }
    impl From<&AmqpValue> for u16 {
        fn from(v: &AmqpValue) -> Self;
    }
    impl From<&AmqpValue> for u32 {
        fn from(v: &AmqpValue) -> Self;
    }
    impl From<&AmqpValue> for u64 {
        fn from(v: &AmqpValue) -> Self;
    }
    impl From<&AmqpValue> for u8 {
        fn from(v: &AmqpValue) -> Self;
    }
    impl From<&Value> for crate::value::AmqpValue {
        fn from(value: &fe2o3_amqp_types::primitives::Value) -> Self;
    }
    impl From<&str> for AmqpValue {
        fn from(b: &str) -> Self;
    }
    impl From<()> for AmqpValue {
        fn from(_: ()) -> Self;
    }
    impl From<AmqpAnnotationKey> for crate::value::AmqpValue {
        fn from(key: AmqpAnnotationKey) -> Self;
    }
    impl From<AmqpMessageId> for crate::value::AmqpValue {
        fn from(message_id: AmqpMessageId) -> Self;
    }
    impl From<AmqpValue> for () {
        fn from(v: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for AmqpAnnotationKey {
        fn from(value: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for AmqpMessageBody {
        fn from(value: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for Vec<AmqpValue> {
        fn from(v: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for Vec<u8> {
        fn from(v: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for azure_core::Uuid {
        fn from(v: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for bool {
        fn from(v: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for char {
        fn from(v: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for f32 {
        fn from(v: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for f64 {
        fn from(v: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for fe2o3_amqp_types::primitives::Value {
        fn from(value: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for i16 {
        fn from(v: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for i32 {
        fn from(v: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for i64 {
        fn from(v: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for i8 {
        fn from(v: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for std::string::String {
        fn from(v: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for u16 {
        fn from(v: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for u32 {
        fn from(v: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for u64 {
        fn from(v: AmqpValue) -> Self;
    }
    impl From<AmqpValue> for u8 {
        fn from(v: AmqpValue) -> Self;
    }
    impl From<Box<AmqpDescribed>> for AmqpValue {
        fn from(v: Box<AmqpDescribed>) -> Self;
    }
    impl From<Box<AmqpValue>> for AmqpValue {
        fn from(b: Box<AmqpValue>) -> Self;
    }
    impl From<String> for AmqpValue {
        fn from(v: std::string::String) -> Self;
    }
    impl From<Uuid> for AmqpValue {
        fn from(v: Uuid) -> Self;
    }
    impl From<Value> for crate::value::AmqpValue {
        fn from(value: fe2o3_amqp_types::primitives::Value) -> Self;
    }
    impl From<Vec<AmqpValue>> for AmqpValue {
        fn from(v: Vec<AmqpValue>) -> Self;
    }
    impl From<Vec<u8>> for AmqpValue {
        fn from(v: Vec<u8>) -> Self;
    }
    impl From<bool> for AmqpValue {
        fn from(v: bool) -> Self;
    }
    impl From<char> for AmqpValue {
        fn from(v: char) -> Self;
    }
    impl From<f32> for AmqpValue {
        fn from(v: f32) -> Self;
    }
    impl From<f64> for AmqpValue {
        fn from(v: f64) -> Self;
    }
    impl From<i16> for AmqpValue {
        fn from(v: i16) -> Self;
    }
    impl From<i32> for AmqpValue {
        fn from(v: i32) -> Self;
    }
    impl From<i64> for AmqpValue {
        fn from(v: i64) -> Self;
    }
    impl From<i8> for AmqpValue {
        fn from(v: i8) -> Self;
    }
    impl From<u16> for AmqpValue {
        fn from(v: u16) -> Self;
    }
    impl From<u32> for AmqpValue {
        fn from(v: u32) -> Self;
    }
    impl From<u64> for AmqpValue {
        fn from(v: u64) -> Self;
    }
    impl From<u8> for AmqpValue {
        fn from(v: u8) -> Self;
    }
    impl PartialEq<()> for AmqpValue {
        fn eq(&self, _: &()) -> bool;
    }
    impl PartialEq<AmqpValue> for () {
        fn eq(&self, other: &AmqpValue) -> bool;
    }
    impl PartialEq<AmqpValue> for Vec<AmqpValue> {
        fn eq(&self, other: &AmqpValue) -> bool;
    }
    impl PartialEq<AmqpValue> for Vec<u8> {
        fn eq(&self, other: &AmqpValue) -> bool;
    }
    impl PartialEq<AmqpValue> for azure_core::Uuid {
        fn eq(&self, other: &AmqpValue) -> bool;
    }
    impl PartialEq<AmqpValue> for bool {
        fn eq(&self, other: &AmqpValue) -> bool;
    }
    impl PartialEq<AmqpValue> for char {
        fn eq(&self, other: &AmqpValue) -> bool;
    }
    impl PartialEq<AmqpValue> for f32 {
        fn eq(&self, other: &AmqpValue) -> bool;
    }
    impl PartialEq<AmqpValue> for f64 {
        fn eq(&self, other: &AmqpValue) -> bool;
    }
    impl PartialEq<AmqpValue> for fe2o3_amqp_types::primitives::Value {
        fn eq(&self, other: &AmqpValue) -> bool;
    }
    impl PartialEq<AmqpValue> for i16 {
        fn eq(&self, other: &AmqpValue) -> bool;
    }
    impl PartialEq<AmqpValue> for i32 {
        fn eq(&self, other: &AmqpValue) -> bool;
    }
    impl PartialEq<AmqpValue> for i64 {
        fn eq(&self, other: &AmqpValue) -> bool;
    }
    impl PartialEq<AmqpValue> for i8 {
        fn eq(&self, other: &AmqpValue) -> bool;
    }
    impl PartialEq<AmqpValue> for std::string::String {
        fn eq(&self, other: &AmqpValue) -> bool;
    }
    impl PartialEq<AmqpValue> for u16 {
        fn eq(&self, other: &AmqpValue) -> bool;
    }
    impl PartialEq<AmqpValue> for u32 {
        fn eq(&self, other: &AmqpValue) -> bool;
    }
    impl PartialEq<AmqpValue> for u64 {
        fn eq(&self, other: &AmqpValue) -> bool;
    }
    impl PartialEq<AmqpValue> for u8 {
        fn eq(&self, other: &AmqpValue) -> bool;
    }
    impl PartialEq<Box<AmqpDescribed>> for AmqpValue {
        fn eq(&self, other: &Box<AmqpDescribed>) -> bool;
    }
    impl PartialEq<String> for AmqpValue {
        fn eq(&self, other: &std::string::String) -> bool;
    }
    impl PartialEq<Uuid> for AmqpValue {
        fn eq(&self, other: &Uuid) -> bool;
    }
    impl PartialEq<Value> for crate::value::AmqpValue {
        fn eq(&self, other: &fe2o3_amqp_types::primitives::Value) -> bool;
    }
    impl PartialEq<Vec<AmqpValue>> for AmqpValue {
        fn eq(&self, other: &Vec<AmqpValue>) -> bool;
    }
    impl PartialEq<Vec<u8>> for AmqpValue {
        fn eq(&self, other: &Vec<u8>) -> bool;
    }
    impl PartialEq<bool> for AmqpValue {
        fn eq(&self, other: &bool) -> bool;
    }
    impl PartialEq<char> for AmqpValue {
        fn eq(&self, other: &char) -> bool;
    }
    impl PartialEq<f32> for AmqpValue {
        fn eq(&self, other: &f32) -> bool;
    }
    impl PartialEq<f64> for AmqpValue {
        fn eq(&self, other: &f64) -> bool;
    }
    impl PartialEq<i16> for AmqpValue {
        fn eq(&self, other: &i16) -> bool;
    }
    impl PartialEq<i32> for AmqpValue {
        fn eq(&self, other: &i32) -> bool;
    }
    impl PartialEq<i64> for AmqpValue {
        fn eq(&self, other: &i64) -> bool;
    }
    impl PartialEq<i8> for AmqpValue {
        fn eq(&self, other: &i8) -> bool;
    }
    impl PartialEq<u16> for AmqpValue {
        fn eq(&self, other: &u16) -> bool;
    }
    impl PartialEq<u32> for AmqpValue {
        fn eq(&self, other: &u32) -> bool;
    }
    impl PartialEq<u64> for AmqpValue {
        fn eq(&self, other: &u64) -> bool;
    }
    impl PartialEq<u8> for AmqpValue {
        fn eq(&self, other: &u8) -> bool;
    }
    #[cfg(feature = "ffi")]
    impl Serializable for AmqpValue {
        fn encoded_size(&self) -> Result<usize>;
        #[allow(unused_variables)]
        fn serialize(&self, buffer: &mut [u8]) -> Result<()>;
    }
    impl TryInto<AmqpValue> for Vec<Vec<serde_amqp::Value>> {
        type Error = Error;
        fn try_into(self) -> Result<AmqpValue, <Self as >::Error>;
    }
    impl TryInto<AmqpValue> for fe2o3_amqp_types::messaging::Data {
        type Error = Error;
        fn try_into(self) -> Result<AmqpValue, <Self as >::Error>;
    }
    impl TryInto<AmqpValue> for fe2o3_amqp_types::messaging::message::EmptyBody {
        type Error = Error;
        fn try_into(self) -> Result<AmqpValue, <Self as >::Error>;
    }
    impl TryInto<AmqpValue> for serde_amqp::extensions::TransparentVec<fe2o3_amqp_types::messaging::Data> {
        type Error = Error;
        fn try_into(self) -> Result<AmqpValue, <Self as >::Error>;
    }
    #[derive(Clone, Debug, PartialEq)]
    pub enum MessageId {
        Binary(Vec<u8>),
        String(String),
        Ulong(u64),
        Uuid(azure_core::Uuid),
    }
    impl From<&str> for MessageId {
        fn from(value: &str) -> Self;
    }
    impl From<AmqpMessageId> for MessageId {
        fn from(message_id: AmqpMessageId) -> Self;
    }
    impl From<MessageId> for String {
        fn from(message_id: MessageId) -> Self;
    }
    impl From<MessageId> for Vec<u8> {
        fn from(message_id: MessageId) -> Self;
    }
    impl From<MessageId> for azure_core::Uuid {
        fn from(message_id: MessageId) -> Self;
    }
    impl From<MessageId> for azure_core_amqp::message::AmqpMessageId {
        fn from(message_id: MessageId) -> Self;
    }
    impl From<MessageId> for u64 {
        fn from(message_id: MessageId) -> Self;
    }
    impl From<String> for MessageId {
        fn from(value: String) -> Self;
    }
    impl From<Uuid> for MessageId {
        fn from(value: Uuid) -> Self;
    }
    impl From<Vec<u8>> for MessageId {
        fn from(value: Vec<u8>) -> Self;
    }
    impl From<u64> for MessageId {
        fn from(value: u64) -> Self;
    }
    pub mod builders {
        #[derive(Default)]
        pub struct EventDataBuilder {
        }
        impl EventDataBuilder {
            fn add_property<impl Into<AmqpSimpleValue>: Into<AmqpSimpleValue>>(self, key: String, value: impl Into<AmqpSimpleValue>) -> Self;
            fn build(self) -> EventData;
            fn with_body<T>(self, body: T) -> Self where T: Into<Vec<u8>>;
            fn with_content_type(self, content_type: String) -> Self;
            fn with_correlation_id<impl Into<MessageId>: Into<MessageId>>(self, correlation_id: impl Into<MessageId>) -> Self;
            fn with_message_id<impl Into<MessageId>: Into<MessageId>>(self, message_id: impl Into<MessageId>) -> Self;
        }
    }
}
pub mod processor {
    pub struct PartitionClient {
    }
    impl PartitionClient {
        async fn close(self) -> Result<()>;
        fn get_partition_id(&self) -> &str;
        fn stream_events(&self) -> impl Stream<Item = Result<ReceivedEventData>> + '_;
        async fn update_checkpoint(&self, event_data: &ReceivedEventData) -> Result<()>;
    }
    impl Drop for PartitionClient {
        fn drop(&mut self);
    }
    impl Send for PartitionClient {
    }
    impl Sync for PartitionClient {
    }
    #[async_trait]
    pub trait CheckpointStore: Send + Sync {
        #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn claim_ownership(&self, ownerships: &[Ownership]) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Vec<Ownership>>> + ::core::marker::Send>>;
        #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn list_checkpoints(&self, namespace: &str, event_hub_name: &str, consumer_group: &str) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Vec<Checkpoint>>> + ::core::marker::Send>>;
        #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn list_ownerships(&self, namespace: &str, event_hub_name: &str, consumer_group: &str) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Vec<Ownership>>> + ::core::marker::Send>>;
        #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn update_checkpoint(&self, checkpoint: Checkpoint) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    }
}
```
