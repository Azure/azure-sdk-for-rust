# azure_storage_queue

- **Description**: Microsoft Azure Queue client library for Rust
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`
  - `azure_core/default`

```rust
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(unused_imports)]
pub use azure_storage_queue::generated::clients::queue_client::QueueClient;
pub use azure_storage_queue::generated::clients::queue_client::QueueClientOptions;
pub use azure_storage_queue::generated::clients::queue_service_client::QueueServiceClient;
pub use azure_storage_queue::generated::clients::queue_service_client::QueueServiceClientOptions;
pub mod clients {
    pub struct QueueClient {
    }
    impl QueueClient {
        async fn clear(&self, options: Option<QueueClientClearOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn create(&self, options: Option<QueueClientCreateOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn delete(&self, options: Option<QueueClientDeleteOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn delete_message(&self, message_id: &str, pop_receipt: &str, options: Option<QueueClientDeleteMessageOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn get_access_policy(&self, options: Option<QueueClientGetAccessPolicyOptions<'_>>) -> Result<Response<SignedIdentifiers, XmlFormat>>;
        async fn get_properties(&self, options: Option<QueueClientGetPropertiesOptions<'_>>) -> Result<Response<QueueClientGetPropertiesResult, NoFormat>>;
        async fn peek_messages(&self, options: Option<QueueClientPeekMessagesOptions<'_>>) -> Result<Response<PeekedMessages, XmlFormat>>;
        async fn receive_messages(&self, options: Option<QueueClientReceiveMessagesOptions<'_>>) -> Result<Response<ReceivedMessages, XmlFormat>>;
        async fn send_message(&self, queue_message: RequestContent<QueueMessage, XmlFormat>, options: Option<QueueClientSendMessageOptions<'_>>) -> Result<Response<ListOfSentMessage, XmlFormat>>;
        async fn set_access_policy(&self, queue_acl: RequestContent<SignedIdentifiers, XmlFormat>, options: Option<QueueClientSetAccessPolicyOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn set_metadata(&self, metadata: &HashMap<String, String>, options: Option<QueueClientSetMetadataOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn update_message(&self, message_id: &str, pop_receipt: &str, visibility_timeout: i32, options: Option<QueueClientUpdateMessageOptions<'_>>) -> Result<Response<(), NoFormat>>;
    }
    impl QueueClient {
        async fn exists(&self) -> Result<bool>;
        fn new(queue_url: Url, credential: Option<Arc<dyn TokenCredential>>, options: Option<QueueClientOptions>) -> Result<Self>;
        fn url(&self) -> &Url;
    }
    #[derive(Clone, Debug)]
    pub struct QueueClientOptions {
        pub client_options: azure_core::http::ClientOptions,
        pub version: String,
    }
    impl Default for QueueClientOptions {
        fn default() -> Self;
    }
    pub struct QueueServiceClient {
    }
    impl QueueServiceClient {
        async fn get_properties(&self, options: Option<QueueServiceClientGetPropertiesOptions<'_>>) -> Result<Response<QueueServiceProperties, XmlFormat>>;
        async fn get_statistics(&self, options: Option<QueueServiceClientGetStatisticsOptions<'_>>) -> Result<Response<QueueServiceStats, XmlFormat>>;
        async fn get_user_delegation_key(&self, key_info: RequestContent<KeyInfo, XmlFormat>, options: Option<QueueServiceClientGetUserDelegationKeyOptions<'_>>) -> Result<Response<UserDelegationKey, XmlFormat>>;
        fn list_queues(&self, options: Option<QueueServiceClientListQueuesOptions<'_>>) -> Result<Pager<ListQueuesResponse, XmlFormat>>;
        async fn set_properties(&self, queue_service_properties: RequestContent<QueueServiceProperties, XmlFormat>, options: Option<QueueServiceClientSetPropertiesOptions<'_>>) -> Result<Response<(), NoFormat>>;
    }
    impl QueueServiceClient {
        fn new(service_url: Url, credential: Option<Arc<dyn TokenCredential>>, options: Option<QueueServiceClientOptions>) -> Result<Self>;
        fn queue_client(&self, queue_name: &str) -> Result<QueueClient>;
        fn url(&self) -> &Url;
    }
    #[derive(Clone, Debug)]
    pub struct QueueServiceClientOptions {
        pub client_options: azure_core::http::ClientOptions,
        pub version: String,
    }
    impl Default for QueueServiceClientOptions {
        fn default() -> Self;
    }
}
pub mod models {
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct AccessPolicy {
        #[serde(default, rename = "Expiry", skip_serializing_if = "Option::is_none", with = "models_serde::option_offset_date_time_rfc3339_fixed_width")]
        pub expiry: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "Permission", skip_serializing_if = "Option::is_none")]
        pub permission: Option<String>,
        #[serde(default, rename = "Start", skip_serializing_if = "Option::is_none", with = "models_serde::option_offset_date_time_rfc3339_fixed_width")]
        pub start: Option<azure_core::time::OffsetDateTime>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[serde(rename = "CorsRule")]
    pub struct CorsRule {
        #[serde(rename = "AllowedHeaders", skip_serializing_if = "Option::is_none")]
        pub allowed_headers: Option<String>,
        #[serde(rename = "AllowedMethods", skip_serializing_if = "Option::is_none")]
        pub allowed_methods: Option<String>,
        #[serde(rename = "AllowedOrigins", skip_serializing_if = "Option::is_none")]
        pub allowed_origins: Option<String>,
        #[serde(rename = "ExposedHeaders", skip_serializing_if = "Option::is_none")]
        pub exposed_headers: Option<String>,
        #[serde(rename = "MaxAgeInSeconds", skip_serializing_if = "Option::is_none")]
        pub max_age_in_seconds: Option<i32>,
    }
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct Error {
        #[serde(rename = "Code", skip_serializing_if = "Option::is_none")]
        pub code: Option<super::StorageErrorCode>,
        #[serde(rename = "errorCode", skip_serializing_if = "Option::is_none")]
        pub error_code: Option<String>,
        #[serde(rename = "Message", skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct GeoReplication {
        #[serde(default, rename = "LastSyncTime", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub last_sync_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
        pub status: Option<super::GeoReplicationStatus>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct KeyInfo {
        #[serde(rename = "DelegatedUserTid", skip_serializing_if = "Option::is_none")]
        pub delegated_user_tid: Option<String>,
        #[serde(default, deserialize_with = "models_serde::option_offset_date_time_rfc3339::deserialize", rename = "Expiry", serialize_with = "azure_storage_common::rfc3339::seconds_only::option::serialize", skip_serializing_if = "Option::is_none")]
        pub expiry: Option<azure_core::time::OffsetDateTime>,
        #[serde(default, deserialize_with = "models_serde::option_offset_date_time_rfc3339::deserialize", rename = "Start", serialize_with = "azure_storage_common::rfc3339::seconds_only::option::serialize", skip_serializing_if = "Option::is_none")]
        pub start: Option<azure_core::time::OffsetDateTime>,
    }
    impl TryFrom<KeyInfo> for azure_core::http::RequestContent<super::KeyInfo, azure_core::http::XmlFormat> {
        type Error = Error;
        fn try_from(value: KeyInfo) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename = "QueueMessagesList")]
    pub struct ListOfSentMessage {
        #[serde(rename = "QueueMessage", skip_serializing_if = "Option::is_none")]
        pub items: Option<Vec<SentMessage>>,
    }
    impl ListOfSentMessage {
        fn into_message(self) -> Result<SentMessage>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename = "EnumerationResults")]
    pub struct ListQueuesResponse {
        #[serde(rename = "Marker", skip_serializing_if = "Option::is_none")]
        pub marker: Option<String>,
        #[serde(rename = "MaxResults", skip_serializing_if = "Option::is_none")]
        pub max_results: Option<i32>,
        #[serde(rename = "NextMarker", skip_serializing_if = "Option::is_none")]
        pub next_marker: Option<String>,
        #[serde(rename = "Prefix", skip_serializing_if = "Option::is_none")]
        pub prefix: Option<String>,
        #[serde(default, deserialize_with = "Queue_itemsQueueItem::unwrap", rename = "Queues", serialize_with = "Queue_itemsQueueItem::wrap")]
        pub queue_items: Vec<QueueItem>,
        #[serde(rename = "@ServiceEndpoint", skip_serializing_if = "Option::is_none")]
        pub service_endpoint: Option<String>,
    }
    impl Page for super::ListQueuesResponse {
        type IntoIter = <Vec<QueueItem> as IntoIterator>::IntoIter;
        type Item = QueueItem;
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn into_items(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<<Self as >::IntoIter>> + ::core::marker::Send>>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct Logging {
        #[serde(rename = "Delete", skip_serializing_if = "Option::is_none")]
        pub delete: Option<bool>,
        #[serde(rename = "Read", skip_serializing_if = "Option::is_none")]
        pub read: Option<bool>,
        #[serde(rename = "RetentionPolicy", skip_serializing_if = "Option::is_none")]
        pub retention_policy: Option<RetentionPolicy>,
        #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
        #[serde(rename = "Write", skip_serializing_if = "Option::is_none")]
        pub write: Option<bool>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct Metrics {
        #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(rename = "IncludeAPIs", skip_serializing_if = "Option::is_none")]
        pub include_apis: Option<bool>,
        #[serde(rename = "RetentionPolicy", skip_serializing_if = "Option::is_none")]
        pub retention_policy: Option<RetentionPolicy>,
        #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename = "QueueMessage")]
    pub struct PeekedMessage {
        #[serde(rename = "DequeueCount", skip_serializing_if = "Option::is_none")]
        pub dequeue_count: Option<i64>,
        #[serde(default, rename = "ExpirationTime", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub expiration_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(default, rename = "InsertionTime", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub insertion_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "MessageId", skip_serializing_if = "Option::is_none")]
        pub message_id: Option<String>,
        #[serde(rename = "MessageText", skip_serializing_if = "Option::is_none")]
        pub message_text: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename = "QueueMessagesList")]
    pub struct PeekedMessages {
        #[serde(rename = "QueueMessage", skip_serializing_if = "Option::is_none")]
        pub items: Option<Vec<PeekedMessage>>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct QueueClientClearOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct QueueClientCreateOptions<'a> {
        pub metadata: Option<std::collections::HashMap<String, String>>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct QueueClientDeleteMessageOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct QueueClientDeleteOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct QueueClientGetAccessPolicyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct QueueClientGetPropertiesOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct QueueClientGetPropertiesResult;
    #[derive(Clone, Debug, Default)]
    pub struct QueueClientPeekMessagesOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub number_of_messages: Option<i32>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct QueueClientReceiveMessagesOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub number_of_messages: Option<i32>,
        pub timeout: Option<i32>,
        pub visibility_timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct QueueClientSendMessageOptions<'a> {
        pub message_time_to_live: Option<i32>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
        pub visibility_timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct QueueClientSetAccessPolicyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct QueueClientSetMetadataOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct QueueClientUpdateMessageOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub queue_message: Option<azure_core::http::RequestContent<super::QueueMessage, azure_core::http::XmlFormat>>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename = "Queue")]
    pub struct QueueItem {
        #[serde(rename = "Metadata", skip_serializing_if = "Option::is_none")]
        pub metadata: Option<std::collections::HashMap<String, String>>,
        #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct QueueMessage {
        #[serde(rename = "MessageText", skip_serializing_if = "Option::is_none")]
        pub message_text: Option<String>,
    }
    impl TryFrom<QueueMessage> for azure_core::http::RequestContent<super::QueueMessage, azure_core::http::XmlFormat> {
        type Error = Error;
        fn try_from(value: QueueMessage) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct QueueServiceClientGetPropertiesOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct QueueServiceClientGetStatisticsOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct QueueServiceClientGetUserDelegationKeyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct QueueServiceClientListQueuesOptions<'a> {
        pub include: Option<Vec<super::ListQueuesIncludeType>>,
        pub marker: Option<String>,
        pub maxresults: Option<i32>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub prefix: Option<String>,
        pub timeout: Option<i32>,
    }
    impl QueueServiceClientListQueuesOptions<'_> {
        fn into_owned(self) -> QueueServiceClientListQueuesOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct QueueServiceClientSetPropertiesOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[serde(rename = "StorageServiceProperties")]
    pub struct QueueServiceProperties {
        #[serde(default, deserialize_with = "CorsCorsRule::unwrap", rename = "Cors", serialize_with = "CorsCorsRule::wrap", skip_serializing_if = "Option::is_none")]
        pub cors: Option<Vec<CorsRule>>,
        #[serde(rename = "HourMetrics", skip_serializing_if = "Option::is_none")]
        pub hour_metrics: Option<Metrics>,
        #[serde(rename = "Logging", skip_serializing_if = "Option::is_none")]
        pub logging: Option<Logging>,
        #[serde(rename = "MinuteMetrics", skip_serializing_if = "Option::is_none")]
        pub minute_metrics: Option<Metrics>,
    }
    impl TryFrom<QueueServiceProperties> for azure_core::http::RequestContent<super::QueueServiceProperties, azure_core::http::XmlFormat> {
        type Error = Error;
        fn try_from(value: QueueServiceProperties) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct QueueServiceStats {
        #[serde(rename = "GeoReplication", skip_serializing_if = "Option::is_none")]
        pub geo_replication: Option<GeoReplication>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename = "QueueMessage")]
    pub struct ReceivedMessage {
        #[serde(rename = "DequeueCount", skip_serializing_if = "Option::is_none")]
        pub dequeue_count: Option<i64>,
        #[serde(default, rename = "ExpirationTime", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub expiration_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(default, rename = "InsertionTime", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub insertion_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "MessageId", skip_serializing_if = "Option::is_none")]
        pub message_id: Option<String>,
        #[serde(rename = "MessageText", skip_serializing_if = "Option::is_none")]
        pub message_text: Option<String>,
        #[serde(rename = "PopReceipt", skip_serializing_if = "Option::is_none")]
        pub pop_receipt: Option<String>,
        #[serde(default, rename = "TimeNextVisible", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub time_next_visible: Option<azure_core::time::OffsetDateTime>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename = "QueueMessagesList")]
    pub struct ReceivedMessages {
        #[serde(rename = "QueueMessage", skip_serializing_if = "Option::is_none")]
        pub items: Option<Vec<ReceivedMessage>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct RetentionPolicy {
        #[serde(rename = "Days", skip_serializing_if = "Option::is_none")]
        pub days: Option<i32>,
        #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename = "QueueMessage")]
    pub struct SentMessage {
        #[serde(default, rename = "ExpirationTime", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub expiration_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(default, rename = "InsertionTime", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub insertion_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "MessageId", skip_serializing_if = "Option::is_none")]
        pub message_id: Option<String>,
        #[serde(rename = "PopReceipt", skip_serializing_if = "Option::is_none")]
        pub pop_receipt: Option<String>,
        #[serde(default, rename = "TimeNextVisible", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub time_next_visible: Option<azure_core::time::OffsetDateTime>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct SignedIdentifier {
        #[serde(rename = "AccessPolicy", skip_serializing_if = "Option::is_none")]
        pub access_policy: Option<AccessPolicy>,
        #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[serde(rename = "SignedIdentifiers")]
    pub struct SignedIdentifiers {
        #[serde(rename = "SignedIdentifier", skip_serializing_if = "Option::is_none")]
        pub items: Option<Vec<SignedIdentifier>>,
    }
    impl TryFrom<SignedIdentifiers> for azure_core::http::RequestContent<super::SignedIdentifiers, azure_core::http::XmlFormat> {
        type Error = Error;
        fn try_from(value: SignedIdentifiers) -> Result<Self>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum GeoReplicationStatus {
        Bootstrap,
        Live,
        Unavailable,
        UnknownValue(String),
    }
    impl AsRef<str> for super::GeoReplicationStatus {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::GeoReplicationStatus {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::GeoReplicationStatus {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::GeoReplicationStatus {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a GeoReplicationStatus> for &'a str {
        fn from(e: &'a GeoReplicationStatus) -> Self;
    }
    impl<'de> Deserialize<'de> for super::GeoReplicationStatus {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ListQueuesIncludeType {
        Metadata,
    }
    impl AsRef<str> for super::ListQueuesIncludeType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::ListQueuesIncludeType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::ListQueuesIncludeType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::ListQueuesIncludeType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::ListQueuesIncludeType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum StorageErrorCode {
        AccountAlreadyExists,
        AccountBeingCreated,
        AccountIsDisabled,
        AuthenticationFailed,
        AuthorizationFailure,
        AuthorizationPermissionMismatch,
        AuthorizationProtocolMismatch,
        AuthorizationResourceTypeMismatch,
        AuthorizationServiceMismatch,
        AuthorizationSourceIPMismatch,
        ConditionHeadersNotSupported,
        ConditionNotMet,
        EmptyMetadataKey,
        FeatureVersionMismatch,
        InsufficientAccountPermissions,
        InternalError,
        InvalidAuthenticationInfo,
        InvalidHeaderValue,
        InvalidHttpVerb,
        InvalidInput,
        InvalidMarker,
        InvalidMd5,
        InvalidMetadata,
        InvalidQueryParameterValue,
        InvalidRange,
        InvalidResourceName,
        InvalidUri,
        InvalidXmlDocument,
        InvalidXmlNodeValue,
        Md5Mismatch,
        MessageNotFound,
        MessageTooLarge,
        MetadataTooLarge,
        MissingContentLengthHeader,
        MissingRequiredHeader,
        MissingRequiredQueryParameter,
        MissingRequiredXmlNode,
        MultipleConditionHeadersNotSupported,
        OperationTimedOut,
        OutOfRangeInput,
        OutOfRangeQueryParameterValue,
        PopReceiptMismatch,
        QueueAlreadyExists,
        QueueBeingDeleted,
        QueueDisabled,
        QueueNotEmpty,
        QueueNotFound,
        RequestBodyTooLarge,
        RequestUrlFailedToParse,
        ResourceAlreadyExists,
        ResourceNotFound,
        ResourceTypeMismatch,
        ServerBusy,
        UnsupportedHeader,
        UnsupportedHttpVerb,
        UnsupportedQueryParameter,
        UnsupportedXmlNode,
        UnknownValue(String),
    }
    impl AsRef<str> for super::StorageErrorCode {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::StorageErrorCode {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::StorageErrorCode {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::StorageErrorCode {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a StorageErrorCode> for &'a str {
        fn from(e: &'a StorageErrorCode) -> Self;
    }
    impl<'de> Deserialize<'de> for super::StorageErrorCode {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    pub trait QueueClientGetPropertiesResultHeaders: private::Sealed {
        fn approximate_messages_count(&self) -> Result<Option<i64>>;
        fn metadata(&self) -> Result<HashMap<String, String>>;
    }
}
```
