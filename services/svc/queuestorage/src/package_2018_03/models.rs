#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An Access policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessPolicy {
    #[doc = "the date-time the policy is active"]
    #[serde(rename = "Start", default, with = "azure_core::date::rfc3339::option")]
    pub start: Option<time::OffsetDateTime>,
    #[doc = "the date-time the policy expires"]
    #[serde(rename = "Expiry", default, with = "azure_core::date::rfc3339::option")]
    pub expiry: Option<time::OffsetDateTime>,
    #[doc = "the permissions for the acl policy"]
    #[serde(rename = "Permission", default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}
impl AccessPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CORS is an HTTP feature that enables a web application running under one domain to access resources in another domain. Web browsers implement a security restriction known as same-origin policy that prevents a web page from calling APIs in a different domain; CORS provides a secure way to allow one domain (the origin domain) to call APIs in another domain"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CorsRule {
    #[doc = "The origin domains that are permitted to make a request against the storage service via CORS. The origin domain is the domain from which the request originates. Note that the origin must be an exact case-sensitive match with the origin that the user age sends to the service. You can also use the wildcard character '*' to allow all origin domains to make requests via CORS."]
    #[serde(rename = "AllowedOrigins")]
    pub allowed_origins: String,
    #[doc = "The methods (HTTP request verbs) that the origin domain may use for a CORS request. (comma separated)"]
    #[serde(rename = "AllowedMethods")]
    pub allowed_methods: String,
    #[doc = "the request headers that the origin domain may specify on the CORS request."]
    #[serde(rename = "AllowedHeaders")]
    pub allowed_headers: String,
    #[doc = "The response headers that may be sent in the response to the CORS request and exposed by the browser to the request issuer"]
    #[serde(rename = "ExposedHeaders")]
    pub exposed_headers: String,
    #[doc = "The maximum amount time that a browser should cache the preflight OPTIONS request."]
    #[serde(rename = "MaxAgeInSeconds")]
    pub max_age_in_seconds: i64,
}
impl CorsRule {
    pub fn new(
        allowed_origins: String,
        allowed_methods: String,
        allowed_headers: String,
        exposed_headers: String,
        max_age_in_seconds: i64,
    ) -> Self {
        Self {
            allowed_origins,
            allowed_methods,
            allowed_headers,
            exposed_headers,
            max_age_in_seconds,
        }
    }
}
#[doc = "The object returned in the QueueMessageList array when calling Get Messages on a Queue."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DequeuedMessageItem {
    #[doc = "The Id of the Message."]
    #[serde(rename = "MessageId")]
    pub message_id: String,
    #[doc = "The time the Message was inserted into the Queue."]
    #[serde(rename = "InsertionTime", with = "azure_core::date::rfc1123")]
    pub insertion_time: time::OffsetDateTime,
    #[doc = "The time that the Message will expire and be automatically deleted."]
    #[serde(rename = "ExpirationTime", with = "azure_core::date::rfc1123")]
    pub expiration_time: time::OffsetDateTime,
    #[doc = "This value is required to delete the Message. If deletion fails using this popreceipt then the message has been dequeued by another client."]
    #[serde(rename = "PopReceipt")]
    pub pop_receipt: String,
    #[doc = "The time that the message will again become visible in the Queue."]
    #[serde(rename = "TimeNextVisible", with = "azure_core::date::rfc1123")]
    pub time_next_visible: time::OffsetDateTime,
    #[doc = "The number of times the message has been dequeued."]
    #[serde(rename = "DequeueCount")]
    pub dequeue_count: i64,
    #[doc = "The content of the Message."]
    #[serde(rename = "MessageText")]
    pub message_text: String,
}
impl DequeuedMessageItem {
    pub fn new(
        message_id: String,
        insertion_time: time::OffsetDateTime,
        expiration_time: time::OffsetDateTime,
        pop_receipt: String,
        time_next_visible: time::OffsetDateTime,
        dequeue_count: i64,
        message_text: String,
    ) -> Self {
        Self {
            message_id,
            insertion_time,
            expiration_time,
            pop_receipt,
            time_next_visible,
            dequeue_count,
            message_text,
        }
    }
}
pub type DequeuedMessagesList = Vec<DequeuedMessageItem>;
#[doc = "The object returned in the QueueMessageList array when calling Put Message on a Queue"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnqueuedMessage {
    #[doc = "The Id of the Message."]
    #[serde(rename = "MessageId")]
    pub message_id: String,
    #[doc = "The time the Message was inserted into the Queue."]
    #[serde(rename = "InsertionTime", with = "azure_core::date::rfc1123")]
    pub insertion_time: time::OffsetDateTime,
    #[doc = "The time that the Message will expire and be automatically deleted."]
    #[serde(rename = "ExpirationTime", with = "azure_core::date::rfc1123")]
    pub expiration_time: time::OffsetDateTime,
    #[doc = "This value is required to delete the Message. If deletion fails using this popreceipt then the message has been dequeued by another client."]
    #[serde(rename = "PopReceipt")]
    pub pop_receipt: String,
    #[doc = "The time that the message will again become visible in the Queue."]
    #[serde(rename = "TimeNextVisible", with = "azure_core::date::rfc1123")]
    pub time_next_visible: time::OffsetDateTime,
}
impl EnqueuedMessage {
    pub fn new(
        message_id: String,
        insertion_time: time::OffsetDateTime,
        expiration_time: time::OffsetDateTime,
        pop_receipt: String,
        time_next_visible: time::OffsetDateTime,
    ) -> Self {
        Self {
            message_id,
            insertion_time,
            expiration_time,
            pop_receipt,
            time_next_visible,
        }
    }
}
pub type EnqueuedMessageList = Vec<EnqueuedMessage>;
#[doc = "Error codes returned by the service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ErrorCode")]
pub enum ErrorCode {
    AccountAlreadyExists,
    AccountBeingCreated,
    AccountIsDisabled,
    AuthenticationFailed,
    AuthorizationFailure,
    ConditionHeadersNotSupported,
    ConditionNotMet,
    EmptyMetadataKey,
    InsufficientAccountPermissions,
    InternalError,
    InvalidAuthenticationInfo,
    InvalidHeaderValue,
    InvalidHttpVerb,
    InvalidInput,
    InvalidMd5,
    InvalidMetadata,
    InvalidQueryParameterValue,
    InvalidRange,
    InvalidResourceName,
    InvalidUri,
    InvalidXmlDocument,
    InvalidXmlNodeValue,
    Md5Mismatch,
    MetadataTooLarge,
    MissingContentLengthHeader,
    MissingRequiredQueryParameter,
    MissingRequiredHeader,
    MissingRequiredXmlNode,
    MultipleConditionHeadersNotSupported,
    OperationTimedOut,
    OutOfRangeInput,
    OutOfRangeQueryParameterValue,
    RequestBodyTooLarge,
    ResourceTypeMismatch,
    RequestUrlFailedToParse,
    ResourceAlreadyExists,
    ResourceNotFound,
    ServerBusy,
    UnsupportedHeader,
    UnsupportedXmlNode,
    UnsupportedQueryParameter,
    UnsupportedHttpVerb,
    InvalidMarker,
    MessageNotFound,
    MessageTooLarge,
    PopReceiptMismatch,
    QueueAlreadyExists,
    QueueBeingDeleted,
    QueueDisabled,
    QueueNotEmpty,
    QueueNotFound,
    #[serde(rename = "AuthorizationSourceIPMismatch")]
    AuthorizationSourceIpMismatch,
    AuthorizationProtocolMismatch,
    AuthorizationPermissionMismatch,
    AuthorizationServiceMismatch,
    AuthorizationResourceTypeMismatch,
    FeatureVersionMismatch,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ErrorCode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ErrorCode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ErrorCode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AccountAlreadyExists => serializer.serialize_unit_variant("ErrorCode", 0u32, "AccountAlreadyExists"),
            Self::AccountBeingCreated => serializer.serialize_unit_variant("ErrorCode", 1u32, "AccountBeingCreated"),
            Self::AccountIsDisabled => serializer.serialize_unit_variant("ErrorCode", 2u32, "AccountIsDisabled"),
            Self::AuthenticationFailed => serializer.serialize_unit_variant("ErrorCode", 3u32, "AuthenticationFailed"),
            Self::AuthorizationFailure => serializer.serialize_unit_variant("ErrorCode", 4u32, "AuthorizationFailure"),
            Self::ConditionHeadersNotSupported => serializer.serialize_unit_variant("ErrorCode", 5u32, "ConditionHeadersNotSupported"),
            Self::ConditionNotMet => serializer.serialize_unit_variant("ErrorCode", 6u32, "ConditionNotMet"),
            Self::EmptyMetadataKey => serializer.serialize_unit_variant("ErrorCode", 7u32, "EmptyMetadataKey"),
            Self::InsufficientAccountPermissions => serializer.serialize_unit_variant("ErrorCode", 8u32, "InsufficientAccountPermissions"),
            Self::InternalError => serializer.serialize_unit_variant("ErrorCode", 9u32, "InternalError"),
            Self::InvalidAuthenticationInfo => serializer.serialize_unit_variant("ErrorCode", 10u32, "InvalidAuthenticationInfo"),
            Self::InvalidHeaderValue => serializer.serialize_unit_variant("ErrorCode", 11u32, "InvalidHeaderValue"),
            Self::InvalidHttpVerb => serializer.serialize_unit_variant("ErrorCode", 12u32, "InvalidHttpVerb"),
            Self::InvalidInput => serializer.serialize_unit_variant("ErrorCode", 13u32, "InvalidInput"),
            Self::InvalidMd5 => serializer.serialize_unit_variant("ErrorCode", 14u32, "InvalidMd5"),
            Self::InvalidMetadata => serializer.serialize_unit_variant("ErrorCode", 15u32, "InvalidMetadata"),
            Self::InvalidQueryParameterValue => serializer.serialize_unit_variant("ErrorCode", 16u32, "InvalidQueryParameterValue"),
            Self::InvalidRange => serializer.serialize_unit_variant("ErrorCode", 17u32, "InvalidRange"),
            Self::InvalidResourceName => serializer.serialize_unit_variant("ErrorCode", 18u32, "InvalidResourceName"),
            Self::InvalidUri => serializer.serialize_unit_variant("ErrorCode", 19u32, "InvalidUri"),
            Self::InvalidXmlDocument => serializer.serialize_unit_variant("ErrorCode", 20u32, "InvalidXmlDocument"),
            Self::InvalidXmlNodeValue => serializer.serialize_unit_variant("ErrorCode", 21u32, "InvalidXmlNodeValue"),
            Self::Md5Mismatch => serializer.serialize_unit_variant("ErrorCode", 22u32, "Md5Mismatch"),
            Self::MetadataTooLarge => serializer.serialize_unit_variant("ErrorCode", 23u32, "MetadataTooLarge"),
            Self::MissingContentLengthHeader => serializer.serialize_unit_variant("ErrorCode", 24u32, "MissingContentLengthHeader"),
            Self::MissingRequiredQueryParameter => serializer.serialize_unit_variant("ErrorCode", 25u32, "MissingRequiredQueryParameter"),
            Self::MissingRequiredHeader => serializer.serialize_unit_variant("ErrorCode", 26u32, "MissingRequiredHeader"),
            Self::MissingRequiredXmlNode => serializer.serialize_unit_variant("ErrorCode", 27u32, "MissingRequiredXmlNode"),
            Self::MultipleConditionHeadersNotSupported => {
                serializer.serialize_unit_variant("ErrorCode", 28u32, "MultipleConditionHeadersNotSupported")
            }
            Self::OperationTimedOut => serializer.serialize_unit_variant("ErrorCode", 29u32, "OperationTimedOut"),
            Self::OutOfRangeInput => serializer.serialize_unit_variant("ErrorCode", 30u32, "OutOfRangeInput"),
            Self::OutOfRangeQueryParameterValue => serializer.serialize_unit_variant("ErrorCode", 31u32, "OutOfRangeQueryParameterValue"),
            Self::RequestBodyTooLarge => serializer.serialize_unit_variant("ErrorCode", 32u32, "RequestBodyTooLarge"),
            Self::ResourceTypeMismatch => serializer.serialize_unit_variant("ErrorCode", 33u32, "ResourceTypeMismatch"),
            Self::RequestUrlFailedToParse => serializer.serialize_unit_variant("ErrorCode", 34u32, "RequestUrlFailedToParse"),
            Self::ResourceAlreadyExists => serializer.serialize_unit_variant("ErrorCode", 35u32, "ResourceAlreadyExists"),
            Self::ResourceNotFound => serializer.serialize_unit_variant("ErrorCode", 36u32, "ResourceNotFound"),
            Self::ServerBusy => serializer.serialize_unit_variant("ErrorCode", 37u32, "ServerBusy"),
            Self::UnsupportedHeader => serializer.serialize_unit_variant("ErrorCode", 38u32, "UnsupportedHeader"),
            Self::UnsupportedXmlNode => serializer.serialize_unit_variant("ErrorCode", 39u32, "UnsupportedXmlNode"),
            Self::UnsupportedQueryParameter => serializer.serialize_unit_variant("ErrorCode", 40u32, "UnsupportedQueryParameter"),
            Self::UnsupportedHttpVerb => serializer.serialize_unit_variant("ErrorCode", 41u32, "UnsupportedHttpVerb"),
            Self::InvalidMarker => serializer.serialize_unit_variant("ErrorCode", 42u32, "InvalidMarker"),
            Self::MessageNotFound => serializer.serialize_unit_variant("ErrorCode", 43u32, "MessageNotFound"),
            Self::MessageTooLarge => serializer.serialize_unit_variant("ErrorCode", 44u32, "MessageTooLarge"),
            Self::PopReceiptMismatch => serializer.serialize_unit_variant("ErrorCode", 45u32, "PopReceiptMismatch"),
            Self::QueueAlreadyExists => serializer.serialize_unit_variant("ErrorCode", 46u32, "QueueAlreadyExists"),
            Self::QueueBeingDeleted => serializer.serialize_unit_variant("ErrorCode", 47u32, "QueueBeingDeleted"),
            Self::QueueDisabled => serializer.serialize_unit_variant("ErrorCode", 48u32, "QueueDisabled"),
            Self::QueueNotEmpty => serializer.serialize_unit_variant("ErrorCode", 49u32, "QueueNotEmpty"),
            Self::QueueNotFound => serializer.serialize_unit_variant("ErrorCode", 50u32, "QueueNotFound"),
            Self::AuthorizationSourceIpMismatch => serializer.serialize_unit_variant("ErrorCode", 51u32, "AuthorizationSourceIPMismatch"),
            Self::AuthorizationProtocolMismatch => serializer.serialize_unit_variant("ErrorCode", 52u32, "AuthorizationProtocolMismatch"),
            Self::AuthorizationPermissionMismatch => {
                serializer.serialize_unit_variant("ErrorCode", 53u32, "AuthorizationPermissionMismatch")
            }
            Self::AuthorizationServiceMismatch => serializer.serialize_unit_variant("ErrorCode", 54u32, "AuthorizationServiceMismatch"),
            Self::AuthorizationResourceTypeMismatch => {
                serializer.serialize_unit_variant("ErrorCode", 55u32, "AuthorizationResourceTypeMismatch")
            }
            Self::FeatureVersionMismatch => serializer.serialize_unit_variant("ErrorCode", 56u32, "FeatureVersionMismatch"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoReplication {
    #[doc = "The status of the secondary location"]
    #[serde(rename = "Status")]
    pub status: geo_replication::Status,
    #[doc = "A GMT date/time value, to the second. All primary writes preceding this value are guaranteed to be available for read operations at the secondary. Primary writes after this point in time may or may not be available for reads."]
    #[serde(rename = "LastSyncTime", with = "azure_core::date::rfc1123")]
    pub last_sync_time: time::OffsetDateTime,
}
impl GeoReplication {
    pub fn new(status: geo_replication::Status, last_sync_time: time::OffsetDateTime) -> Self {
        Self { status, last_sync_time }
    }
}
pub mod geo_replication {
    use super::*;
    #[doc = "The status of the secondary location"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "live")]
        Live,
        #[serde(rename = "bootstrap")]
        Bootstrap,
        #[serde(rename = "unavailable")]
        Unavailable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Status {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Status {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Status {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Live => serializer.serialize_unit_variant("Status", 0u32, "live"),
                Self::Bootstrap => serializer.serialize_unit_variant("Status", 1u32, "bootstrap"),
                Self::Unavailable => serializer.serialize_unit_variant("Status", 2u32, "unavailable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The object returned when calling List Queues on a Queue Service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListQueuesSegmentResponse {
    #[serde(rename = "ServiceEndpoint")]
    pub service_endpoint: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxResults")]
    pub max_results: i64,
    #[serde(rename = "QueueItems", default, skip_serializing_if = "Vec::is_empty")]
    pub queue_items: Vec<QueueItem>,
    #[serde(rename = "NextMarker")]
    pub next_marker: String,
}
impl azure_core::Continuable for ListQueuesSegmentResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        if self.next_marker.is_empty() {
            None
        } else {
            Some(self.next_marker.clone())
        }
    }
}
impl ListQueuesSegmentResponse {
    pub fn new(service_endpoint: String, prefix: String, max_results: i64, next_marker: String) -> Self {
        Self {
            service_endpoint,
            prefix,
            marker: None,
            max_results,
            queue_items: Vec::new(),
            next_marker,
        }
    }
}
#[doc = "Azure Analytics Logging settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Logging {
    #[doc = "The version of Storage Analytics to configure."]
    #[serde(rename = "Version")]
    pub version: String,
    #[doc = "Indicates whether all delete requests should be logged."]
    #[serde(rename = "Delete")]
    pub delete: bool,
    #[doc = "Indicates whether all read requests should be logged."]
    #[serde(rename = "Read")]
    pub read: bool,
    #[doc = "Indicates whether all write requests should be logged."]
    #[serde(rename = "Write")]
    pub write: bool,
    #[doc = "the retention policy"]
    #[serde(rename = "RetentionPolicy")]
    pub retention_policy: RetentionPolicy,
}
impl Logging {
    pub fn new(version: String, delete: bool, read: bool, write: bool, retention_policy: RetentionPolicy) -> Self {
        Self {
            version,
            delete,
            read,
            write,
            retention_policy,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Metadata {}
impl Metadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "a summary of request statistics grouped by API in hour or minute aggregates for queues"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    #[doc = "The version of Storage Analytics to configure."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Indicates whether metrics are enabled for the Queue service."]
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[doc = "Indicates whether metrics should generate summary statistics for called API operations."]
    #[serde(rename = "IncludeAPIs", default, skip_serializing_if = "Option::is_none")]
    pub include_ap_is: Option<bool>,
    #[doc = "the retention policy"]
    #[serde(rename = "RetentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
impl Metrics {
    pub fn new(enabled: bool) -> Self {
        Self {
            version: None,
            enabled,
            include_ap_is: None,
            retention_policy: None,
        }
    }
}
#[doc = "The object returned in the QueueMessageList array when calling Peek Messages on a Queue"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeekedMessageItem {
    #[doc = "The Id of the Message."]
    #[serde(rename = "MessageId")]
    pub message_id: String,
    #[doc = "The time the Message was inserted into the Queue."]
    #[serde(rename = "InsertionTime", with = "azure_core::date::rfc1123")]
    pub insertion_time: time::OffsetDateTime,
    #[doc = "The time that the Message will expire and be automatically deleted."]
    #[serde(rename = "ExpirationTime", with = "azure_core::date::rfc1123")]
    pub expiration_time: time::OffsetDateTime,
    #[doc = "The number of times the message has been dequeued."]
    #[serde(rename = "DequeueCount")]
    pub dequeue_count: i64,
    #[doc = "The content of the Message."]
    #[serde(rename = "MessageText")]
    pub message_text: String,
}
impl PeekedMessageItem {
    pub fn new(
        message_id: String,
        insertion_time: time::OffsetDateTime,
        expiration_time: time::OffsetDateTime,
        dequeue_count: i64,
        message_text: String,
    ) -> Self {
        Self {
            message_id,
            insertion_time,
            expiration_time,
            dequeue_count,
            message_text,
        }
    }
}
pub type PeekedMessagesList = Vec<PeekedMessageItem>;
#[doc = "An Azure Storage Queue."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueueItem {
    #[doc = "The name of the Queue."]
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
impl QueueItem {
    pub fn new(name: String) -> Self {
        Self { name, metadata: None }
    }
}
#[doc = "A Message object which can be stored in a Queue"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueueMessage {
    #[doc = "The content of the message"]
    #[serde(rename = "MessageText")]
    pub message_text: String,
}
impl QueueMessage {
    pub fn new(message_text: String) -> Self {
        Self { message_text }
    }
}
#[doc = "the retention policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionPolicy {
    #[doc = "Indicates whether a retention policy is enabled for the storage service"]
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[doc = "Indicates the number of days that metrics or logging or soft-deleted data should be retained. All data older than this value will be deleted"]
    #[serde(rename = "Days", default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
}
impl RetentionPolicy {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, days: None }
    }
}
#[doc = "signed identifier"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignedIdentifier {
    #[doc = "a unique id"]
    #[serde(rename = "Id")]
    pub id: String,
    #[doc = "An Access policy"]
    #[serde(rename = "AccessPolicy")]
    pub access_policy: AccessPolicy,
}
impl SignedIdentifier {
    pub fn new(id: String, access_policy: AccessPolicy) -> Self {
        Self { id, access_policy }
    }
}
pub type SignedIdentifiers = Vec<SignedIdentifier>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageError {
    #[serde(rename = "Message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl azure_core::Continuable for StorageError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl StorageError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage Service Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageServiceProperties {
    #[doc = "Azure Analytics Logging settings."]
    #[serde(rename = "Logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
    #[doc = "a summary of request statistics grouped by API in hour or minute aggregates for queues"]
    #[serde(rename = "HourMetrics", default, skip_serializing_if = "Option::is_none")]
    pub hour_metrics: Option<Metrics>,
    #[doc = "a summary of request statistics grouped by API in hour or minute aggregates for queues"]
    #[serde(rename = "MinuteMetrics", default, skip_serializing_if = "Option::is_none")]
    pub minute_metrics: Option<Metrics>,
    #[doc = "The set of CORS rules."]
    #[serde(rename = "Cors", default, skip_serializing_if = "Vec::is_empty")]
    pub cors: Vec<CorsRule>,
}
impl StorageServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Stats for the storage service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageServiceStats {
    #[serde(rename = "GeoReplication", default, skip_serializing_if = "Option::is_none")]
    pub geo_replication: Option<GeoReplication>,
}
impl StorageServiceStats {
    pub fn new() -> Self {
        Self::default()
    }
}
