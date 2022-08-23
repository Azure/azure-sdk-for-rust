#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Namespace/ServiceBus Connection String"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessKeys {
    #[doc = "Primary connection string of the created namespace authorization rule."]
    #[serde(rename = "primaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[doc = "Secondary connection string of the created namespace authorization rule."]
    #[serde(rename = "secondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
    #[doc = "Primary connection string of the alias if GEO DR is enabled"]
    #[serde(rename = "aliasPrimaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub alias_primary_connection_string: Option<String>,
    #[doc = "Secondary  connection string of the alias if GEO DR is enabled"]
    #[serde(rename = "aliasSecondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub alias_secondary_connection_string: Option<String>,
    #[doc = "A base64-encoded 256-bit primary key for signing and validating the SAS token."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "A base64-encoded 256-bit primary key for signing and validating the SAS token."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "A string that describes the authorization rule."]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
}
impl AccessKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the filter actions which are allowed for the transformation of a message that have been matched by a filter expression."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Action {
    #[doc = "SQL expression. e.g. MyProperty='ABC'"]
    #[serde(rename = "sqlExpression", default, skip_serializing_if = "Option::is_none")]
    pub sql_expression: Option<String>,
    #[doc = "This property is reserved for future use. An integer value showing the compatibility level, currently hard-coded to 20."]
    #[serde(rename = "compatibilityLevel", default, skip_serializing_if = "Option::is_none")]
    pub compatibility_level: Option<i32>,
    #[doc = "Value that indicates whether the rule action requires preprocessing."]
    #[serde(rename = "requiresPreprocessing", default, skip_serializing_if = "Option::is_none")]
    pub requires_preprocessing: Option<bool>,
}
impl Action {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Single item in List or Get Alias(Disaster Recovery configuration) operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmDisasterRecovery {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties required to the Create Or Update Alias(Disaster Recovery configurations)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<arm_disaster_recovery::Properties>,
}
impl ArmDisasterRecovery {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod arm_disaster_recovery {
    use super::*;
    #[doc = "Properties required to the Create Or Update Alias(Disaster Recovery configurations)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Provisioning state of the Alias(Disaster Recovery configuration) - possible values 'Accepted' or 'Succeeded' or 'Failed'"]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "Number of entities pending to be replicated."]
        #[serde(rename = "pendingReplicationOperationsCount", default, skip_serializing_if = "Option::is_none")]
        pub pending_replication_operations_count: Option<i64>,
        #[doc = "ARM Id of the Primary/Secondary eventhub namespace name, which is part of GEO DR pairing"]
        #[serde(rename = "partnerNamespace", default, skip_serializing_if = "Option::is_none")]
        pub partner_namespace: Option<String>,
        #[doc = "Primary/Secondary eventhub namespace name, which is part of GEO DR pairing"]
        #[serde(rename = "alternateName", default, skip_serializing_if = "Option::is_none")]
        pub alternate_name: Option<String>,
        #[doc = "role of namespace in GEO DR - possible values 'Primary' or 'PrimaryNotReplicating' or 'Secondary'"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub role: Option<properties::Role>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Provisioning state of the Alias(Disaster Recovery configuration) - possible values 'Accepted' or 'Succeeded' or 'Failed'"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Accepted,
            Succeeded,
            Failed,
        }
        #[doc = "role of namespace in GEO DR - possible values 'Primary' or 'PrimaryNotReplicating' or 'Secondary'"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Role {
            Primary,
            PrimaryNotReplicating,
            Secondary,
        }
    }
}
#[doc = "The result of the List Alias(Disaster Recovery configuration) operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmDisasterRecoveryListResult {
    #[doc = "List of Alias(Disaster Recovery configurations)"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ArmDisasterRecovery>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of Alias(Disaster Recovery configuration)"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ArmDisasterRecoveryListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ArmDisasterRecoveryListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties to configure capture description for eventhub"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CaptureDescription {
    #[doc = "A value that indicates whether capture description is enabled. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Enumerates the possible values for the encoding format of capture description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<capture_description::Encoding>,
    #[doc = "The time window allows you to set the frequency with which the capture to Azure Blobs will happen, value should between 60 to 900 seconds"]
    #[serde(rename = "intervalInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i32>,
    #[doc = "The size window defines the amount of data built up in your Event Hub before an capture operation, value should be between 10485760 and 524288000 bytes"]
    #[serde(rename = "sizeLimitInBytes", default, skip_serializing_if = "Option::is_none")]
    pub size_limit_in_bytes: Option<i32>,
    #[doc = "Capture storage details for capture description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
}
impl CaptureDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod capture_description {
    use super::*;
    #[doc = "Enumerates the possible values for the encoding format of capture description."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Encoding {
        Avro,
        AvroDeflate,
    }
}
#[doc = "Description of a Check Name availability request properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailability {
    #[doc = "The Name to check the namespace name availability and The namespace name can contain only letters, numbers, and hyphens. The namespace must start with a letter, and it must end with a letter or number."]
    pub name: String,
}
impl CheckNameAvailability {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "Description of a Check Name availability request properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "The detailed info regarding the reason associated with the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Value indicating namespace is availability, true if the namespace is available; otherwise, false."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Specifies the reason for the unavailability of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<UnavailableReason>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ConnectionState information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionState {
    #[doc = "Status of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<connection_state::Status>,
    #[doc = "Description of the connection state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod connection_state {
    use super::*;
    #[doc = "Status of the connection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Pending,
        Approved,
        Rejected,
        Disconnected,
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
                Self::Pending => serializer.serialize_unit_variant("Status", 0u32, "Pending"),
                Self::Approved => serializer.serialize_unit_variant("Status", 1u32, "Approved"),
                Self::Rejected => serializer.serialize_unit_variant("Status", 2u32, "Rejected"),
                Self::Disconnected => serializer.serialize_unit_variant("Status", 3u32, "Disconnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents the correlation filter expression."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CorrelationFilter {
    #[doc = "dictionary object for custom filters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Identifier of the correlation."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "Identifier of the message."]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[doc = "Address to send to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[doc = "Address of the queue to reply to."]
    #[serde(rename = "replyTo", default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[doc = "Application specific label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Session identifier."]
    #[serde(rename = "sessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[doc = "Session identifier to reply to."]
    #[serde(rename = "replyToSessionId", default, skip_serializing_if = "Option::is_none")]
    pub reply_to_session_id: Option<String>,
    #[doc = "Content type of the message."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "Value that indicates whether the rule action requires preprocessing."]
    #[serde(rename = "requiresPreprocessing", default, skip_serializing_if = "Option::is_none")]
    pub requires_preprocessing: Option<bool>,
}
impl CorrelationFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Capture storage details for capture description"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Destination {
    #[doc = "Name for capture destination"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties describing the storage account, blob container and archive name format for capture destination"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<destination::Properties>,
}
impl Destination {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod destination {
    use super::*;
    #[doc = "Properties describing the storage account, blob container and archive name format for capture destination"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Resource id of the storage account to be used to create the blobs"]
        #[serde(rename = "storageAccountResourceId", default, skip_serializing_if = "Option::is_none")]
        pub storage_account_resource_id: Option<String>,
        #[doc = "Blob container Name"]
        #[serde(rename = "blobContainer", default, skip_serializing_if = "Option::is_none")]
        pub blob_container: Option<String>,
        #[doc = "Blob naming convention for archive, e.g. {Namespace}/{EventHub}/{PartitionId}/{Year}/{Month}/{Day}/{Hour}/{Minute}/{Second}. Here all the parameters (Namespace,EventHub .. etc) are mandatory irrespective of order"]
        #[serde(rename = "archiveNameFormat", default, skip_serializing_if = "Option::is_none")]
        pub archive_name_format: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Properties to configure Encryption"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Encryption {
    #[doc = "Properties to configure keyVault Properties"]
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
    #[doc = "Enumerates the possible value of keySource for Encryption"]
    #[serde(rename = "keySource", default, skip_serializing_if = "Option::is_none")]
    pub key_source: Option<encryption::KeySource>,
}
impl Encryption {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption {
    use super::*;
    #[doc = "Enumerates the possible value of keySource for Encryption"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeySource {
        #[serde(rename = "Microsoft.KeyVault")]
        MicrosoftKeyVault,
    }
    impl Default for KeySource {
        fn default() -> Self {
            Self::MicrosoftKeyVault
        }
    }
}
#[doc = "Entity status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EntityStatus {
    Active,
    Disabled,
    Restoring,
    SendDisabled,
    ReceiveDisabled,
    Creating,
    Deleting,
    Renaming,
    Unknown,
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource management error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response::Error>,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod error_response {
    use super::*;
    #[doc = "The error object."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "The error code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "The error message."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[doc = "The error target."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,
        #[doc = "The error details."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub details: Vec<ErrorResponse>,
        #[doc = "The error additional info."]
        #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
        pub additional_info: Vec<ErrorAdditionalInfo>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The result of the List EventHubs operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubListResult {
    #[doc = "Result of the List EventHubs operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Eventhub>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of EventHubs."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EventHubListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EventHubListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Single item in List or Get Event Hub operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Eventhub {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties supplied to the Create Or Update Event Hub operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<eventhub::Properties>,
}
impl Eventhub {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod eventhub {
    use super::*;
    #[doc = "Properties supplied to the Create Or Update Event Hub operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Current number of shards on the Event Hub."]
        #[serde(rename = "partitionIds", default, skip_serializing_if = "Vec::is_empty")]
        pub partition_ids: Vec<String>,
        #[doc = "Exact time the Event Hub was created."]
        #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        #[doc = "The exact time the message was updated."]
        #[serde(rename = "updatedAt", default, with = "azure_core::date::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
        #[doc = "Number of days to retain the events for this Event Hub, value should be 1 to 7 days"]
        #[serde(rename = "messageRetentionInDays", default, skip_serializing_if = "Option::is_none")]
        pub message_retention_in_days: Option<i64>,
        #[doc = "Number of partitions created for the Event Hub, allowed values are from 1 to 32 partitions."]
        #[serde(rename = "partitionCount", default, skip_serializing_if = "Option::is_none")]
        pub partition_count: Option<i64>,
        #[doc = "Entity status."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<EntityStatus>,
        #[doc = "Properties to configure capture description for eventhub"]
        #[serde(rename = "captureDescription", default, skip_serializing_if = "Option::is_none")]
        pub capture_description: Option<CaptureDescription>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Safe failover is to indicate the service should wait for pending replication to finish before switching to the secondary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverProperties {
    #[doc = "Safe failover is to indicate the service should wait for pending replication to finish before switching to the secondary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<failover_properties::Properties>,
}
impl FailoverProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod failover_properties {
    use super::*;
    #[doc = "Safe failover is to indicate the service should wait for pending replication to finish before switching to the secondary."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Safe failover is to indicate the service should wait for pending replication to finish before switching to the secondary."]
        #[serde(rename = "IsSafeFailover", default, skip_serializing_if = "Option::is_none")]
        pub is_safe_failover: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Rule filter types"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FilterType {
    SqlFilter,
    CorrelationFilter,
}
#[doc = "Properties to configure Identity for Bring your Own Keys"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[doc = "ObjectId from the KeyVault"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "TenantId from the KeyVault"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Enumerates the possible value Identity type, which currently supports only 'SystemAssigned'"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity {
    use super::*;
    #[doc = "Enumerates the possible value Identity type, which currently supports only 'SystemAssigned'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
    impl Default for Type {
        fn default() -> Self {
            Self::SystemAssigned
        }
    }
}
#[doc = "Single item in a List or Get IpFilterRules operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpFilterRule {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties supplied to create or update IpFilterRules"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ip_filter_rule::Properties>,
}
impl IpFilterRule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ip_filter_rule {
    use super::*;
    #[doc = "Properties supplied to create or update IpFilterRules"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "IP Mask"]
        #[serde(rename = "ipMask", default, skip_serializing_if = "Option::is_none")]
        pub ip_mask: Option<String>,
        #[doc = "The IP Filter Action"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub action: Option<properties::Action>,
        #[doc = "IP Filter name"]
        #[serde(rename = "filterName", default, skip_serializing_if = "Option::is_none")]
        pub filter_name: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "The IP Filter Action"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Action")]
        pub enum Action {
            Accept,
            Reject,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for Action {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for Action {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for Action {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Accept => serializer.serialize_unit_variant("Action", 0u32, "Accept"),
                    Self::Reject => serializer.serialize_unit_variant("Action", 1u32, "Reject"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "The response from the List namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpFilterRuleListResult {
    #[doc = "Result of the List IpFilter Rules operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IpFilterRule>,
    #[doc = "Link to the next set of results. Not empty if Value contains an incomplete list of IpFilter Rules"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IpFilterRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IpFilterRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties to configure keyVault Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultProperties {
    #[doc = "Name of the Key from KeyVault"]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[doc = "Uri of KeyVault"]
    #[serde(rename = "keyVaultUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_uri: Option<String>,
}
impl KeyVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Message Count Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MessageCountDetails {
    #[doc = "Number of active messages in the queue, topic, or subscription."]
    #[serde(rename = "activeMessageCount", default, skip_serializing_if = "Option::is_none")]
    pub active_message_count: Option<i64>,
    #[doc = "Number of messages that are dead lettered."]
    #[serde(rename = "deadLetterMessageCount", default, skip_serializing_if = "Option::is_none")]
    pub dead_letter_message_count: Option<i64>,
    #[doc = "Number of scheduled messages."]
    #[serde(rename = "scheduledMessageCount", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_message_count: Option<i64>,
    #[doc = "Number of messages transferred to another queue, topic, or subscription."]
    #[serde(rename = "transferMessageCount", default, skip_serializing_if = "Option::is_none")]
    pub transfer_message_count: Option<i64>,
    #[doc = "Number of messages transferred into dead letters."]
    #[serde(rename = "transferDeadLetterMessageCount", default, skip_serializing_if = "Option::is_none")]
    pub transfer_dead_letter_message_count: Option<i64>,
}
impl MessageCountDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of the List migrationConfigurations operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationConfigListResult {
    #[doc = "List of Migration Configs"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MigrationConfigProperties>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of migrationConfigurations"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MigrationConfigListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MigrationConfigListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Single item in List or Get Migration Config operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationConfigProperties {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties required to the Create Migration Configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<migration_config_properties::Properties>,
}
impl MigrationConfigProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod migration_config_properties {
    use super::*;
    #[doc = "Properties required to the Create Migration Configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "Provisioning state of Migration Configuration "]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<String>,
        #[doc = "Number of entities pending to be replicated."]
        #[serde(rename = "pendingReplicationOperationsCount", default, skip_serializing_if = "Option::is_none")]
        pub pending_replication_operations_count: Option<i64>,
        #[doc = "Existing premium Namespace ARM Id name which has no entities, will be used for migration"]
        #[serde(rename = "targetNamespace")]
        pub target_namespace: String,
        #[doc = "Name to access Standard Namespace after migration"]
        #[serde(rename = "postMigrationName")]
        pub post_migration_name: String,
        #[doc = "State in which Standard to Premium Migration is, possible values : Unknown, Reverting, Completing, Initiating, Syncing, Active"]
        #[serde(rename = "migrationState", default, skip_serializing_if = "Option::is_none")]
        pub migration_state: Option<String>,
    }
    impl Properties {
        pub fn new(target_namespace: String, post_migration_name: String) -> Self {
            Self {
                provisioning_state: None,
                pending_replication_operations_count: None,
                target_namespace,
                post_migration_name,
                migration_state: None,
            }
        }
    }
}
#[doc = "Description of NetWorkRuleSet - IpRules resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NwRuleSetIpRules {
    #[doc = "IP Mask"]
    #[serde(rename = "ipMask", default, skip_serializing_if = "Option::is_none")]
    pub ip_mask: Option<String>,
    #[doc = "The IP Filter Action"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<nw_rule_set_ip_rules::Action>,
}
impl NwRuleSetIpRules {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod nw_rule_set_ip_rules {
    use super::*;
    #[doc = "The IP Filter Action"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Action")]
    pub enum Action {
        Allow,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Action {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Action {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Action {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allow => serializer.serialize_unit_variant("Action", 0u32, "Allow"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Action {
        fn default() -> Self {
            Self::Allow
        }
    }
}
#[doc = "Description of VirtualNetworkRules - NetworkRules resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NwRuleSetVirtualNetworkRules {
    #[doc = "Properties supplied for Subnet"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<Subnet>,
    #[doc = "Value that indicates whether to ignore missing VNet Service Endpoint"]
    #[serde(rename = "ignoreMissingVnetServiceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing_vnet_service_endpoint: Option<bool>,
}
impl NwRuleSetVirtualNetworkRules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of NetworkRuleSet resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRuleSet {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "NetworkRuleSet properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<network_rule_set::Properties>,
}
impl NetworkRuleSet {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_rule_set {
    use super::*;
    #[doc = "NetworkRuleSet properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Default Action for Network Rule Set"]
        #[serde(rename = "defaultAction", default, skip_serializing_if = "Option::is_none")]
        pub default_action: Option<properties::DefaultAction>,
        #[doc = "List VirtualNetwork Rules"]
        #[serde(rename = "virtualNetworkRules", default, skip_serializing_if = "Vec::is_empty")]
        pub virtual_network_rules: Vec<NwRuleSetVirtualNetworkRules>,
        #[doc = "List of IpRules"]
        #[serde(rename = "ipRules", default, skip_serializing_if = "Vec::is_empty")]
        pub ip_rules: Vec<NwRuleSetIpRules>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Default Action for Network Rule Set"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "DefaultAction")]
        pub enum DefaultAction {
            Allow,
            Deny,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for DefaultAction {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for DefaultAction {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for DefaultAction {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Allow => serializer.serialize_unit_variant("DefaultAction", 0u32, "Allow"),
                    Self::Deny => serializer.serialize_unit_variant("DefaultAction", 1u32, "Deny"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "The response of the List NetworkRuleSet operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRuleSetListResult {
    #[doc = "Result of the List NetworkRuleSet operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NetworkRuleSet>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of NetworkRuleSet."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkRuleSetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkRuleSetListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A ServiceBus REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.ServiceBus"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Invoice, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list ServiceBus operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of ServiceBus operations supported by the Microsoft.ServiceBus resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Premium Messaging Region"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PremiumMessagingRegions {
    #[serde(flatten)]
    pub resource_namespace_patch: ResourceNamespacePatch,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<premium_messaging_regions::Properties>,
}
impl PremiumMessagingRegions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod premium_messaging_regions {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Region code"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Full name of the region"]
        #[serde(rename = "fullName", default, skip_serializing_if = "Option::is_none")]
        pub full_name: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The response of the List PremiumMessagingRegions operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PremiumMessagingRegionsListResult {
    #[doc = "Result of the List PremiumMessagingRegions type."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PremiumMessagingRegions>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of PremiumMessagingRegions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PremiumMessagingRegionsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PremiumMessagingRegionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "PrivateEndpoint information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The ARM identifier for Private Endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the PrivateEndpointConnection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the private endpoint connection resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the list of all private endpoint connections operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "A collection of private endpoint connection resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "A link for the next page of private endpoint connection resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the private endpoint connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "PrivateEndpoint information."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "ConnectionState information."]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<ConnectionState>,
    #[doc = "Provisioning state of the Private Endpoint Connection."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<private_endpoint_connection_properties::ProvisioningState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_endpoint_connection_properties {
    use super::*;
    #[doc = "Provisioning state of the Private Endpoint Connection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Canceled,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Information of the private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[doc = "Properties of PrivateLinkResource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
    #[doc = "Fully qualified identifier of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the resource"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of PrivateLinkResource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "Required Members"]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "Required Zone Names"]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the List private link resources operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourcesListResult {
    #[doc = "A collection of private link resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "A link for the next page of private link resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PrivateLinkResourcesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Regenerate Authorization Rule operation, specifies which key needs to be reset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateAccessKeyParameters {
    #[doc = "The access key to regenerate."]
    #[serde(rename = "keyType")]
    pub key_type: regenerate_access_key_parameters::KeyType,
    #[doc = "Optional, if the key value provided, is reset for KeyType value or autogenerate Key value set for keyType"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
impl RegenerateAccessKeyParameters {
    pub fn new(key_type: regenerate_access_key_parameters::KeyType) -> Self {
        Self { key_type, key: None }
    }
}
pub mod regenerate_access_key_parameters {
    use super::*;
    #[doc = "The access key to regenerate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyType {
        PrimaryKey,
        SecondaryKey,
    }
}
#[doc = "The Resource definition for other than namespace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceNamespacePatch {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceNamespacePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of Rule Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Rule {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Description of Rule Resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Ruleproperties>,
}
impl Rule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of the List rule operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleListResult {
    #[doc = "Result of the List Rules operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Rule>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of rules"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of Rule Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Ruleproperties {
    #[doc = "Represents the filter actions which are allowed for the transformation of a message that have been matched by a filter expression."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    #[doc = "Rule filter types"]
    #[serde(rename = "filterType", default, skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<FilterType>,
    #[doc = "Represents a filter which is a composition of an expression and an action that is executed in the pub/sub pipeline."]
    #[serde(rename = "sqlFilter", default, skip_serializing_if = "Option::is_none")]
    pub sql_filter: Option<SqlFilter>,
    #[doc = "Represents the correlation filter expression."]
    #[serde(rename = "correlationFilter", default, skip_serializing_if = "Option::is_none")]
    pub correlation_filter: Option<CorrelationFilter>,
}
impl Ruleproperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a namespace authorization rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SbAuthorizationRule {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "AuthorizationRule properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<sb_authorization_rule::Properties>,
}
impl SbAuthorizationRule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sb_authorization_rule {
    use super::*;
    #[doc = "AuthorizationRule properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "The rights associated with the rule."]
        pub rights: Vec<String>,
    }
    impl Properties {
        pub fn new(rights: Vec<String>) -> Self {
            Self { rights }
        }
    }
}
#[doc = "The response to the List Namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SbAuthorizationRuleListResult {
    #[doc = "Result of the List Authorization Rules operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SbAuthorizationRule>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of Authorization Rules."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SbAuthorizationRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SbAuthorizationRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a namespace resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbNamespace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "SKU of the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SbSku>,
    #[doc = "Properties to configure Identity for Bring your Own Keys"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Properties of the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SbNamespaceProperties>,
}
impl SbNamespace {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            sku: None,
            identity: None,
            properties: None,
        }
    }
}
#[doc = "The response of the List Namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SbNamespaceListResult {
    #[doc = "Result of the List Namespace operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SbNamespace>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of Namespaces."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SbNamespaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SbNamespaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Namespace Migrate Object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbNamespaceMigrate {
    #[doc = "Type of namespaces"]
    #[serde(rename = "targetNamespaceType")]
    pub target_namespace_type: sb_namespace_migrate::TargetNamespaceType,
}
impl SbNamespaceMigrate {
    pub fn new(target_namespace_type: sb_namespace_migrate::TargetNamespaceType) -> Self {
        Self { target_namespace_type }
    }
}
pub mod sb_namespace_migrate {
    use super::*;
    #[doc = "Type of namespaces"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TargetNamespaceType {
        Messaging,
        NotificationHub,
        Mixed,
        EventHub,
        Relay,
    }
}
#[doc = "Properties of the namespace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SbNamespaceProperties {
    #[doc = "Provisioning state of the namespace."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Status of the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The time the namespace was created"]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The time the namespace was updated."]
    #[serde(rename = "updatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub updated_at: Option<time::OffsetDateTime>,
    #[doc = "Endpoint you can use to perform Service Bus operations."]
    #[serde(rename = "serviceBusEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_endpoint: Option<String>,
    #[doc = "Identifier for Azure Insights metrics"]
    #[serde(rename = "metricId", default, skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<String>,
    #[doc = "Enabling this property creates a Premium Service Bus Namespace in regions supported availability zones."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
    #[doc = "Properties to configure Encryption"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
}
impl SbNamespaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a namespace resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SbNamespaceUpdateParameters {
    #[serde(flatten)]
    pub resource_namespace_patch: ResourceNamespacePatch,
    #[doc = "SKU of the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SbSku>,
    #[doc = "Properties of the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SbNamespaceProperties>,
    #[doc = "Properties to configure Identity for Bring your Own Keys"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
impl SbNamespaceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of queue Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SbQueue {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The Queue Properties definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SbQueueProperties>,
}
impl SbQueue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to the List Queues operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SbQueueListResult {
    #[doc = "Result of the List Queues operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SbQueue>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of queues."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SbQueueListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SbQueueListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Queue Properties definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SbQueueProperties {
    #[doc = "Message Count Details."]
    #[serde(rename = "countDetails", default, skip_serializing_if = "Option::is_none")]
    pub count_details: Option<MessageCountDetails>,
    #[doc = "The exact time the message was created."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The exact time the message was updated."]
    #[serde(rename = "updatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub updated_at: Option<time::OffsetDateTime>,
    #[doc = "Last time a message was sent, or the last time there was a receive request to this queue."]
    #[serde(rename = "accessedAt", default, with = "azure_core::date::rfc3339::option")]
    pub accessed_at: Option<time::OffsetDateTime>,
    #[doc = "The size of the queue, in bytes."]
    #[serde(rename = "sizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    #[doc = "The number of messages in the queue."]
    #[serde(rename = "messageCount", default, skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i64>,
    #[doc = "ISO 8601 timespan duration of a peek-lock; that is, the amount of time that the message is locked for other receivers. The maximum value for LockDuration is 5 minutes; the default value is 1 minute."]
    #[serde(rename = "lockDuration", default, skip_serializing_if = "Option::is_none")]
    pub lock_duration: Option<String>,
    #[doc = "The maximum size of the queue in megabytes, which is the size of memory allocated for the queue. Default is 1024."]
    #[serde(rename = "maxSizeInMegabytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_in_megabytes: Option<i32>,
    #[doc = "A value indicating if this queue requires duplicate detection."]
    #[serde(rename = "requiresDuplicateDetection", default, skip_serializing_if = "Option::is_none")]
    pub requires_duplicate_detection: Option<bool>,
    #[doc = "A value that indicates whether the queue supports the concept of sessions."]
    #[serde(rename = "requiresSession", default, skip_serializing_if = "Option::is_none")]
    pub requires_session: Option<bool>,
    #[doc = "ISO 8601 default message timespan to live value. This is the duration after which the message expires, starting from when the message is sent to Service Bus. This is the default value used when TimeToLive is not set on a message itself."]
    #[serde(rename = "defaultMessageTimeToLive", default, skip_serializing_if = "Option::is_none")]
    pub default_message_time_to_live: Option<String>,
    #[doc = "A value that indicates whether this queue has dead letter support when a message expires."]
    #[serde(rename = "deadLetteringOnMessageExpiration", default, skip_serializing_if = "Option::is_none")]
    pub dead_lettering_on_message_expiration: Option<bool>,
    #[doc = "ISO 8601 timeSpan structure that defines the duration of the duplicate detection history. The default value is 10 minutes."]
    #[serde(rename = "duplicateDetectionHistoryTimeWindow", default, skip_serializing_if = "Option::is_none")]
    pub duplicate_detection_history_time_window: Option<String>,
    #[doc = "The maximum delivery count. A message is automatically deadlettered after this number of deliveries. default value is 10."]
    #[serde(rename = "maxDeliveryCount", default, skip_serializing_if = "Option::is_none")]
    pub max_delivery_count: Option<i32>,
    #[doc = "Entity status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EntityStatus>,
    #[doc = "Value that indicates whether server-side batched operations are enabled."]
    #[serde(rename = "enableBatchedOperations", default, skip_serializing_if = "Option::is_none")]
    pub enable_batched_operations: Option<bool>,
    #[doc = "ISO 8061 timeSpan idle interval after which the queue is automatically deleted. The minimum duration is 5 minutes."]
    #[serde(rename = "autoDeleteOnIdle", default, skip_serializing_if = "Option::is_none")]
    pub auto_delete_on_idle: Option<String>,
    #[doc = "A value that indicates whether the queue is to be partitioned across multiple message brokers."]
    #[serde(rename = "enablePartitioning", default, skip_serializing_if = "Option::is_none")]
    pub enable_partitioning: Option<bool>,
    #[doc = "A value that indicates whether Express Entities are enabled. An express queue holds a message in memory temporarily before writing it to persistent storage."]
    #[serde(rename = "enableExpress", default, skip_serializing_if = "Option::is_none")]
    pub enable_express: Option<bool>,
    #[doc = "Queue/Topic name to forward the messages"]
    #[serde(rename = "forwardTo", default, skip_serializing_if = "Option::is_none")]
    pub forward_to: Option<String>,
    #[doc = "Queue/Topic name to forward the Dead Letter message"]
    #[serde(rename = "forwardDeadLetteredMessagesTo", default, skip_serializing_if = "Option::is_none")]
    pub forward_dead_lettered_messages_to: Option<String>,
}
impl SbQueueProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SKU of the namespace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbSku {
    #[doc = "Name of this SKU."]
    pub name: sb_sku::Name,
    #[doc = "The billing tier of this particular SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sb_sku::Tier>,
    #[doc = "The specified messaging units for the tier. For Premium tier, capacity are 1,2 and 4."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl SbSku {
    pub fn new(name: sb_sku::Name) -> Self {
        Self {
            name,
            tier: None,
            capacity: None,
        }
    }
}
pub mod sb_sku {
    use super::*;
    #[doc = "Name of this SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Basic,
        Standard,
        Premium,
    }
    #[doc = "The billing tier of this particular SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Basic,
        Standard,
        Premium,
    }
}
#[doc = "Description of subscription resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SbSubscription {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Description of Subscription Resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SbSubscriptionProperties>,
}
impl SbSubscription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to the List Subscriptions operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SbSubscriptionListResult {
    #[doc = "Result of the List Subscriptions operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SbSubscription>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of subscriptions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SbSubscriptionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SbSubscriptionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of Subscription Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SbSubscriptionProperties {
    #[doc = "Number of messages."]
    #[serde(rename = "messageCount", default, skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i64>,
    #[doc = "Exact time the message was created."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "Last time there was a receive request to this subscription."]
    #[serde(rename = "accessedAt", default, with = "azure_core::date::rfc3339::option")]
    pub accessed_at: Option<time::OffsetDateTime>,
    #[doc = "The exact time the message was updated."]
    #[serde(rename = "updatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub updated_at: Option<time::OffsetDateTime>,
    #[doc = "Message Count Details."]
    #[serde(rename = "countDetails", default, skip_serializing_if = "Option::is_none")]
    pub count_details: Option<MessageCountDetails>,
    #[doc = "ISO 8061 lock duration timespan for the subscription. The default value is 1 minute."]
    #[serde(rename = "lockDuration", default, skip_serializing_if = "Option::is_none")]
    pub lock_duration: Option<String>,
    #[doc = "Value indicating if a subscription supports the concept of sessions."]
    #[serde(rename = "requiresSession", default, skip_serializing_if = "Option::is_none")]
    pub requires_session: Option<bool>,
    #[doc = "ISO 8061 Default message timespan to live value. This is the duration after which the message expires, starting from when the message is sent to Service Bus. This is the default value used when TimeToLive is not set on a message itself."]
    #[serde(rename = "defaultMessageTimeToLive", default, skip_serializing_if = "Option::is_none")]
    pub default_message_time_to_live: Option<String>,
    #[doc = "Value that indicates whether a subscription has dead letter support on filter evaluation exceptions."]
    #[serde(
        rename = "deadLetteringOnFilterEvaluationExceptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dead_lettering_on_filter_evaluation_exceptions: Option<bool>,
    #[doc = "Value that indicates whether a subscription has dead letter support when a message expires."]
    #[serde(rename = "deadLetteringOnMessageExpiration", default, skip_serializing_if = "Option::is_none")]
    pub dead_lettering_on_message_expiration: Option<bool>,
    #[doc = "ISO 8601 timeSpan structure that defines the duration of the duplicate detection history. The default value is 10 minutes."]
    #[serde(rename = "duplicateDetectionHistoryTimeWindow", default, skip_serializing_if = "Option::is_none")]
    pub duplicate_detection_history_time_window: Option<String>,
    #[doc = "Number of maximum deliveries."]
    #[serde(rename = "maxDeliveryCount", default, skip_serializing_if = "Option::is_none")]
    pub max_delivery_count: Option<i32>,
    #[doc = "Entity status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EntityStatus>,
    #[doc = "Value that indicates whether server-side batched operations are enabled."]
    #[serde(rename = "enableBatchedOperations", default, skip_serializing_if = "Option::is_none")]
    pub enable_batched_operations: Option<bool>,
    #[doc = "ISO 8061 timeSpan idle interval after which the topic is automatically deleted. The minimum duration is 5 minutes."]
    #[serde(rename = "autoDeleteOnIdle", default, skip_serializing_if = "Option::is_none")]
    pub auto_delete_on_idle: Option<String>,
    #[doc = "Queue/Topic name to forward the messages"]
    #[serde(rename = "forwardTo", default, skip_serializing_if = "Option::is_none")]
    pub forward_to: Option<String>,
    #[doc = "Queue/Topic name to forward the Dead Letter message"]
    #[serde(rename = "forwardDeadLetteredMessagesTo", default, skip_serializing_if = "Option::is_none")]
    pub forward_dead_lettered_messages_to: Option<String>,
}
impl SbSubscriptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of topic resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SbTopic {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The Topic Properties definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SbTopicProperties>,
}
impl SbTopic {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to the List Topics operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SbTopicListResult {
    #[doc = "Result of the List Topics operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SbTopic>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of topics."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SbTopicListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SbTopicListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Topic Properties definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SbTopicProperties {
    #[doc = "Size of the topic, in bytes."]
    #[serde(rename = "sizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    #[doc = "Exact time the message was created."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The exact time the message was updated."]
    #[serde(rename = "updatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub updated_at: Option<time::OffsetDateTime>,
    #[doc = "Last time the message was sent, or a request was received, for this topic."]
    #[serde(rename = "accessedAt", default, with = "azure_core::date::rfc3339::option")]
    pub accessed_at: Option<time::OffsetDateTime>,
    #[doc = "Number of subscriptions."]
    #[serde(rename = "subscriptionCount", default, skip_serializing_if = "Option::is_none")]
    pub subscription_count: Option<i32>,
    #[doc = "Message Count Details."]
    #[serde(rename = "countDetails", default, skip_serializing_if = "Option::is_none")]
    pub count_details: Option<MessageCountDetails>,
    #[doc = "ISO 8601 Default message timespan to live value. This is the duration after which the message expires, starting from when the message is sent to Service Bus. This is the default value used when TimeToLive is not set on a message itself."]
    #[serde(rename = "defaultMessageTimeToLive", default, skip_serializing_if = "Option::is_none")]
    pub default_message_time_to_live: Option<String>,
    #[doc = "Maximum size of the topic in megabytes, which is the size of the memory allocated for the topic. Default is 1024."]
    #[serde(rename = "maxSizeInMegabytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_in_megabytes: Option<i32>,
    #[doc = "Value indicating if this topic requires duplicate detection."]
    #[serde(rename = "requiresDuplicateDetection", default, skip_serializing_if = "Option::is_none")]
    pub requires_duplicate_detection: Option<bool>,
    #[doc = "ISO8601 timespan structure that defines the duration of the duplicate detection history. The default value is 10 minutes."]
    #[serde(rename = "duplicateDetectionHistoryTimeWindow", default, skip_serializing_if = "Option::is_none")]
    pub duplicate_detection_history_time_window: Option<String>,
    #[doc = "Value that indicates whether server-side batched operations are enabled."]
    #[serde(rename = "enableBatchedOperations", default, skip_serializing_if = "Option::is_none")]
    pub enable_batched_operations: Option<bool>,
    #[doc = "Entity status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EntityStatus>,
    #[doc = "Value that indicates whether the topic supports ordering."]
    #[serde(rename = "supportOrdering", default, skip_serializing_if = "Option::is_none")]
    pub support_ordering: Option<bool>,
    #[doc = "ISO 8601 timespan idle interval after which the topic is automatically deleted. The minimum duration is 5 minutes."]
    #[serde(rename = "autoDeleteOnIdle", default, skip_serializing_if = "Option::is_none")]
    pub auto_delete_on_idle: Option<String>,
    #[doc = "Value that indicates whether the topic to be partitioned across multiple message brokers is enabled."]
    #[serde(rename = "enablePartitioning", default, skip_serializing_if = "Option::is_none")]
    pub enable_partitioning: Option<bool>,
    #[doc = "Value that indicates whether Express Entities are enabled. An express topic holds a message in memory temporarily before writing it to persistent storage."]
    #[serde(rename = "enableExpress", default, skip_serializing_if = "Option::is_none")]
    pub enable_express: Option<bool>,
}
impl SbTopicProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a filter which is a composition of an expression and an action that is executed in the pub/sub pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlFilter {
    #[doc = "The SQL expression. e.g. MyProperty='ABC'"]
    #[serde(rename = "sqlExpression", default, skip_serializing_if = "Option::is_none")]
    pub sql_expression: Option<String>,
    #[doc = "This property is reserved for future use. An integer value showing the compatibility level, currently hard-coded to 20."]
    #[serde(rename = "compatibilityLevel", default, skip_serializing_if = "Option::is_none")]
    pub compatibility_level: Option<i32>,
    #[doc = "Value that indicates whether the rule action requires preprocessing."]
    #[serde(rename = "requiresPreprocessing", default, skip_serializing_if = "Option::is_none")]
    pub requires_preprocessing: Option<bool>,
}
impl SqlFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents set of actions written in SQL language-based syntax that is performed against a ServiceBus.Messaging.BrokeredMessage "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlRuleAction {
    #[serde(flatten)]
    pub action: Action,
    #[doc = "SQL expression. e.g. MyProperty='ABC'"]
    #[serde(rename = "sqlExpression", default, skip_serializing_if = "Option::is_none")]
    pub sql_expression: Option<String>,
    #[doc = "This property is reserved for future use. An integer value showing the compatibility level, currently hard-coded to 20."]
    #[serde(rename = "compatibilityLevel", default, skip_serializing_if = "Option::is_none")]
    pub compatibility_level: Option<i32>,
    #[doc = "Value that indicates whether the rule action requires preprocessing."]
    #[serde(rename = "requiresPreprocessing", default, skip_serializing_if = "Option::is_none")]
    pub requires_preprocessing: Option<bool>,
}
impl SqlRuleAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties supplied for Subnet"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Subnet {
    #[doc = "Resource ID of Virtual Network Subnet"]
    pub id: String,
}
impl Subnet {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "The Resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The Geo-location where the resource lives"]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            location,
            tags: None,
        }
    }
}
#[doc = "Specifies the reason for the unavailability of the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UnavailableReason {
    None,
    InvalidName,
    SubscriptionIsDisabled,
    NameInUse,
    NameInLockdown,
    TooManyNamespaceInCurrentSubscription,
}
#[doc = "Single item in a List or Get VirtualNetworkRules operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkRule {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties supplied to create or update VirtualNetworkRules"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<virtual_network_rule::Properties>,
}
impl VirtualNetworkRule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_network_rule {
    use super::*;
    #[doc = "Properties supplied to create or update VirtualNetworkRules"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Resource ID of Virtual Network Subnet"]
        #[serde(rename = "virtualNetworkSubnetId", default, skip_serializing_if = "Option::is_none")]
        pub virtual_network_subnet_id: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The response from the List namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkRuleListResult {
    #[doc = "Result of the List VirtualNetwork Rules operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetworkRule>,
    #[doc = "Link to the next set of results. Not empty if Value contains an incomplete list of VirtualNetwork Rules"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualNetworkRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
