#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmIdentity {
    #[doc = "Principal Id"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Tenant Id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of identity used for the resource. The type 'SystemAssigned,UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the service."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<arm_identity::Type>,
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl ArmIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod arm_identity {
    use super::*;
    #[doc = "The type of identity used for the resource. The type 'SystemAssigned,UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmUserIdentity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl ArmUserIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON-serialized X509 Certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateBodyDescription {
    #[doc = "base-64 representation of the X509 leaf certificate .cer file or just .pem file content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}
impl CertificateBodyDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The X509 Certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateDescription {
    #[doc = "The description of an X509 CA Certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CertificateProperties>,
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The entity tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CertificateDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON-serialized array of Certificate objects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateListDescription {
    #[doc = "The array of Certificate objects."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CertificateDescription>,
}
impl CertificateListDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The description of an X509 CA Certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateProperties {
    #[doc = "The certificate's subject name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "The certificate's expiration date and time."]
    #[serde(with = "azure_core::date::rfc1123::option")]
    pub expiry: Option<time::OffsetDateTime>,
    #[doc = "The certificate's thumbprint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "Determines whether certificate has been verified."]
    #[serde(rename = "isVerified", default, skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[doc = "The certificate's create date and time."]
    #[serde(with = "azure_core::date::rfc1123::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "The certificate's last update date and time."]
    #[serde(with = "azure_core::date::rfc1123::option")]
    pub updated: Option<time::OffsetDateTime>,
    #[doc = "The certificate content"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}
impl CertificateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The description of an X509 CA Certificate including the challenge nonce issued for the Proof-Of-Possession flow."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificatePropertiesWithNonce {
    #[doc = "The certificate's subject name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "The certificate's expiration date and time."]
    #[serde(with = "azure_core::date::rfc1123::option")]
    pub expiry: Option<time::OffsetDateTime>,
    #[doc = "The certificate's thumbprint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "Determines whether certificate has been verified."]
    #[serde(rename = "isVerified", default, skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[doc = "The certificate's create date and time."]
    #[serde(with = "azure_core::date::rfc1123::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "The certificate's last update date and time."]
    #[serde(with = "azure_core::date::rfc1123::option")]
    pub updated: Option<time::OffsetDateTime>,
    #[doc = "The certificate's verification code that will be used for proof of possession."]
    #[serde(rename = "verificationCode", default, skip_serializing_if = "Option::is_none")]
    pub verification_code: Option<String>,
    #[doc = "The certificate content"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}
impl CertificatePropertiesWithNonce {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON-serialized leaf certificate"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateVerificationDescription {
    #[doc = "base-64 representation of X509 certificate .cer file or just .pem file content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}
impl CertificateVerificationDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The X509 Certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateWithNonceDescription {
    #[doc = "The description of an X509 CA Certificate including the challenge nonce issued for the Proof-Of-Possession flow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CertificatePropertiesWithNonce>,
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The entity tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CertificateWithNonceDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The IoT hub cloud-to-device messaging properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudToDeviceProperties {
    #[doc = "The max delivery count for cloud-to-device messages in the device queue. See: https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-messaging#cloud-to-device-messages."]
    #[serde(rename = "maxDeliveryCount", default, skip_serializing_if = "Option::is_none")]
    pub max_delivery_count: Option<i32>,
    #[doc = "The default time to live for cloud-to-device messages in the device queue. See: https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-messaging#cloud-to-device-messages."]
    #[serde(rename = "defaultTtlAsIso8601", default, skip_serializing_if = "Option::is_none")]
    pub default_ttl_as_iso8601: Option<String>,
    #[doc = "The properties of the feedback queue for cloud-to-device messages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback: Option<FeedbackProperties>,
}
impl CloudToDeviceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The encryption properties for the IoT hub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionPropertiesDescription {
    #[doc = "The source of the key."]
    #[serde(rename = "keySource", default, skip_serializing_if = "Option::is_none")]
    pub key_source: Option<String>,
    #[doc = "The properties of the KeyVault key."]
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub key_vault_properties: Vec<KeyVaultKeyProperties>,
}
impl EncryptionPropertiesDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The health data for an endpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointHealthData {
    #[doc = "Id of the endpoint"]
    #[serde(rename = "endpointId", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[doc = "Health statuses have following meanings. The 'healthy' status shows that the endpoint is accepting messages as expected. The 'unhealthy' status shows that the endpoint is not accepting messages as expected and IoT Hub is retrying to send data to this endpoint. The status of an unhealthy endpoint will be updated to healthy when IoT Hub has established an eventually consistent state of health. The 'dead' status shows that the endpoint is not accepting messages, after IoT Hub retried sending messages for the retrial period. See IoT Hub metrics to identify errors and monitor issues with endpoints. The 'unknown' status shows that the IoT Hub has not established a connection with the endpoint. No messages have been delivered to or rejected from this endpoint"]
    #[serde(rename = "healthStatus", default, skip_serializing_if = "Option::is_none")]
    pub health_status: Option<endpoint_health_data::HealthStatus>,
    #[doc = "Last error obtained when a message failed to be delivered to iot hub"]
    #[serde(rename = "lastKnownError", default, skip_serializing_if = "Option::is_none")]
    pub last_known_error: Option<String>,
    #[doc = "Time at which the last known error occurred"]
    #[serde(rename = "lastKnownErrorTime", with = "azure_core::date::rfc1123::option")]
    pub last_known_error_time: Option<time::OffsetDateTime>,
    #[doc = "Last time iot hub successfully sent a message to the endpoint"]
    #[serde(rename = "lastSuccessfulSendAttemptTime", with = "azure_core::date::rfc1123::option")]
    pub last_successful_send_attempt_time: Option<time::OffsetDateTime>,
    #[doc = "Last time iot hub tried to send a message to the endpoint"]
    #[serde(rename = "lastSendAttemptTime", with = "azure_core::date::rfc1123::option")]
    pub last_send_attempt_time: Option<time::OffsetDateTime>,
}
impl EndpointHealthData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod endpoint_health_data {
    use super::*;
    #[doc = "Health statuses have following meanings. The 'healthy' status shows that the endpoint is accepting messages as expected. The 'unhealthy' status shows that the endpoint is not accepting messages as expected and IoT Hub is retrying to send data to this endpoint. The status of an unhealthy endpoint will be updated to healthy when IoT Hub has established an eventually consistent state of health. The 'dead' status shows that the endpoint is not accepting messages, after IoT Hub retried sending messages for the retrial period. See IoT Hub metrics to identify errors and monitor issues with endpoints. The 'unknown' status shows that the IoT Hub has not established a connection with the endpoint. No messages have been delivered to or rejected from this endpoint"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HealthStatus")]
    pub enum HealthStatus {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "healthy")]
        Healthy,
        #[serde(rename = "degraded")]
        Degraded,
        #[serde(rename = "unhealthy")]
        Unhealthy,
        #[serde(rename = "dead")]
        Dead,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HealthStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HealthStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HealthStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("HealthStatus", 0u32, "unknown"),
                Self::Healthy => serializer.serialize_unit_variant("HealthStatus", 1u32, "healthy"),
                Self::Degraded => serializer.serialize_unit_variant("HealthStatus", 2u32, "degraded"),
                Self::Unhealthy => serializer.serialize_unit_variant("HealthStatus", 3u32, "unhealthy"),
                Self::Dead => serializer.serialize_unit_variant("HealthStatus", 4u32, "dead"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The JSON-serialized array of EndpointHealthData objects with a next link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointHealthDataListResult {
    #[doc = "JSON-serialized array of Endpoint health data"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EndpointHealthData>,
    #[doc = "Link to more results"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EndpointHealthDataListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EndpointHealthDataListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an enrichment that your IoT hub applies to messages delivered to endpoints."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentProperties {
    #[doc = "The key or name for the enrichment property."]
    pub key: String,
    #[doc = "The value for the enrichment property."]
    pub value: String,
    #[doc = "The list of endpoints for which the enrichment is applied to the message."]
    #[serde(rename = "endpointNames")]
    pub endpoint_names: Vec<String>,
}
impl EnrichmentProperties {
    pub fn new(key: String, value: String, endpoint_names: Vec<String>) -> Self {
        Self {
            key,
            value,
            endpoint_names,
        }
    }
}
#[doc = "Error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The HTTP status code."]
    #[serde(rename = "httpStatusCode", default, skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
impl azure_core::Continuable for ErrorDetails {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The EventHub consumer group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubConsumerGroupBodyDescription {
    #[doc = "The EventHub consumer group name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubConsumerGroupName>,
}
impl EventHubConsumerGroupBodyDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the EventHubConsumerGroupInfo object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubConsumerGroupInfo {
    #[doc = "The tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The Event Hub-compatible consumer group identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The Event Hub-compatible consumer group name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "the resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The etag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl EventHubConsumerGroupInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The EventHub consumer group name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubConsumerGroupName {
    #[doc = "EventHub consumer group name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl EventHubConsumerGroupName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON-serialized array of Event Hub-compatible consumer group names with a next link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubConsumerGroupsListResult {
    #[doc = "List of consumer groups objects"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EventHubConsumerGroupInfo>,
    #[doc = "The next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EventHubConsumerGroupsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EventHubConsumerGroupsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the provisioned Event Hub-compatible endpoint used by the IoT hub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubProperties {
    #[doc = "The retention time for device-to-cloud messages in days. See: https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-messaging#device-to-cloud-messages"]
    #[serde(rename = "retentionTimeInDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_time_in_days: Option<i64>,
    #[doc = "The number of partitions for receiving device-to-cloud messages in the Event Hub-compatible endpoint. See: https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-messaging#device-to-cloud-messages."]
    #[serde(rename = "partitionCount", default, skip_serializing_if = "Option::is_none")]
    pub partition_count: Option<i32>,
    #[doc = "The partition ids in the Event Hub-compatible endpoint."]
    #[serde(rename = "partitionIds", default, skip_serializing_if = "Vec::is_empty")]
    pub partition_ids: Vec<String>,
    #[doc = "The Event Hub-compatible name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The Event Hub-compatible endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl EventHubProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Use to provide parameters when requesting an export of all devices in the IoT hub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportDevicesRequest {
    #[doc = "The export blob container URI."]
    #[serde(rename = "exportBlobContainerUri")]
    pub export_blob_container_uri: String,
    #[doc = "The value indicating whether keys should be excluded during export."]
    #[serde(rename = "excludeKeys")]
    pub exclude_keys: bool,
    #[doc = "The name of the blob that will be created in the provided output blob container. This blob will contain the exported device registry information for the IoT Hub."]
    #[serde(rename = "exportBlobName", default, skip_serializing_if = "Option::is_none")]
    pub export_blob_name: Option<String>,
    #[doc = "Specifies authentication type being used for connecting to the storage account."]
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<export_devices_request::AuthenticationType>,
}
impl ExportDevicesRequest {
    pub fn new(export_blob_container_uri: String, exclude_keys: bool) -> Self {
        Self {
            export_blob_container_uri,
            exclude_keys,
            export_blob_name: None,
            authentication_type: None,
        }
    }
}
pub mod export_devices_request {
    use super::*;
    #[doc = "Specifies authentication type being used for connecting to the storage account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthenticationType")]
    pub enum AuthenticationType {
        #[serde(rename = "keyBased")]
        KeyBased,
        #[serde(rename = "identityBased")]
        IdentityBased,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuthenticationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuthenticationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuthenticationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::KeyBased => serializer.serialize_unit_variant("AuthenticationType", 0u32, "keyBased"),
                Self::IdentityBased => serializer.serialize_unit_variant("AuthenticationType", 1u32, "identityBased"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Use to provide failover region when requesting manual Failover for a hub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailoverInput {
    #[doc = "Region the hub will be failed over to"]
    #[serde(rename = "failoverRegion")]
    pub failover_region: String,
}
impl FailoverInput {
    pub fn new(failover_region: String) -> Self {
        Self { failover_region }
    }
}
#[doc = "The properties of the fallback route. IoT Hub uses these properties when it routes messages to the fallback endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FallbackRouteProperties {
    #[doc = "The name of the route. The name can only include alphanumeric characters, periods, underscores, hyphens, has a maximum length of 64 characters, and must be unique."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The source to which the routing rule is to be applied to. For example, DeviceMessages"]
    pub source: fallback_route_properties::Source,
    #[doc = "The condition which is evaluated in order to apply the fallback route. If the condition is not provided it will evaluate to true by default. For grammar, See: https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-query-language"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "The list of endpoints to which the messages that satisfy the condition are routed to. Currently only 1 endpoint is allowed."]
    #[serde(rename = "endpointNames")]
    pub endpoint_names: Vec<String>,
    #[doc = "Used to specify whether the fallback route is enabled."]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
impl FallbackRouteProperties {
    pub fn new(source: fallback_route_properties::Source, endpoint_names: Vec<String>, is_enabled: bool) -> Self {
        Self {
            name: None,
            source,
            condition: None,
            endpoint_names,
            is_enabled,
        }
    }
}
pub mod fallback_route_properties {
    use super::*;
    #[doc = "The source to which the routing rule is to be applied to. For example, DeviceMessages"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Source")]
    pub enum Source {
        DeviceMessages,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Source {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Source {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Source {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::DeviceMessages => serializer.serialize_unit_variant("Source", 0u32, "DeviceMessages"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of the feedback queue for cloud-to-device messages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeedbackProperties {
    #[doc = "The lock duration for the feedback queue. See: https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-messaging#cloud-to-device-messages."]
    #[serde(rename = "lockDurationAsIso8601", default, skip_serializing_if = "Option::is_none")]
    pub lock_duration_as_iso8601: Option<String>,
    #[doc = "The period of time for which a message is available to consume before it is expired by the IoT hub. See: https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-messaging#cloud-to-device-messages."]
    #[serde(rename = "ttlAsIso8601", default, skip_serializing_if = "Option::is_none")]
    pub ttl_as_iso8601: Option<String>,
    #[doc = "The number of times the IoT hub attempts to deliver a message on the feedback queue. See: https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-messaging#cloud-to-device-messages."]
    #[serde(rename = "maxDeliveryCount", default, skip_serializing_if = "Option::is_none")]
    pub max_delivery_count: Option<i32>,
}
impl FeedbackProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The group information for creating a private endpoint on an IotHub"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupIdInformation {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The properties for a group information object"]
    pub properties: GroupIdInformationProperties,
}
impl GroupIdInformation {
    pub fn new(properties: GroupIdInformationProperties) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            properties,
        }
    }
}
#[doc = "The properties for a group information object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupIdInformationProperties {
    #[doc = "The group id"]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The required members for a specific group id"]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "The required DNS zones for a specific group id"]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl GroupIdInformationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Use to provide parameters when requesting an import of all devices in the hub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportDevicesRequest {
    #[doc = "The input blob container URI."]
    #[serde(rename = "inputBlobContainerUri")]
    pub input_blob_container_uri: String,
    #[doc = "The output blob container URI."]
    #[serde(rename = "outputBlobContainerUri")]
    pub output_blob_container_uri: String,
    #[doc = "The blob name to be used when importing from the provided input blob container."]
    #[serde(rename = "inputBlobName", default, skip_serializing_if = "Option::is_none")]
    pub input_blob_name: Option<String>,
    #[doc = "The blob name to use for storing the status of the import job."]
    #[serde(rename = "outputBlobName", default, skip_serializing_if = "Option::is_none")]
    pub output_blob_name: Option<String>,
    #[doc = "Specifies authentication type being used for connecting to the storage account."]
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<import_devices_request::AuthenticationType>,
}
impl ImportDevicesRequest {
    pub fn new(input_blob_container_uri: String, output_blob_container_uri: String) -> Self {
        Self {
            input_blob_container_uri,
            output_blob_container_uri,
            input_blob_name: None,
            output_blob_name: None,
            authentication_type: None,
        }
    }
}
pub mod import_devices_request {
    use super::*;
    #[doc = "Specifies authentication type being used for connecting to the storage account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthenticationType")]
    pub enum AuthenticationType {
        #[serde(rename = "keyBased")]
        KeyBased,
        #[serde(rename = "identityBased")]
        IdentityBased,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuthenticationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuthenticationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuthenticationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::KeyBased => serializer.serialize_unit_variant("AuthenticationType", 0u32, "keyBased"),
                Self::IdentityBased => serializer.serialize_unit_variant("AuthenticationType", 1u32, "identityBased"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "IoT Hub capacity information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotHubCapacity {
    #[doc = "The minimum number of units."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[doc = "The maximum number of units."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[doc = "The default number of units."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i64>,
    #[doc = "The type of the scaling enabled."]
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<iot_hub_capacity::ScaleType>,
}
impl IotHubCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod iot_hub_capacity {
    use super::*;
    #[doc = "The type of the scaling enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScaleType {
        Automatic,
        Manual,
        None,
    }
}
#[doc = "The description of the IoT hub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubDescription {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The Etag field is *not* required. If it is provided in the response body, it must also be provided as a header per the normal ETag convention."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "The properties of an IoT hub."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IotHubProperties>,
    #[doc = "Information about the SKU of the IoT hub."]
    pub sku: IotHubSkuInfo,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ArmIdentity>,
}
impl IotHubDescription {
    pub fn new(resource: Resource, sku: IotHubSkuInfo) -> Self {
        Self {
            resource,
            etag: None,
            properties: None,
            sku,
            identity: None,
        }
    }
}
#[doc = "The JSON-serialized array of IotHubDescription objects with a next link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotHubDescriptionListResult {
    #[doc = "The array of IotHubDescription objects."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IotHubDescription>,
    #[doc = "The next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IotHubDescriptionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IotHubDescriptionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Public representation of one of the locations where a resource is provisioned."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotHubLocationDescription {
    #[doc = "The name of the Azure region"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The role of the region, can be either primary or secondary. The primary region is where the IoT hub is currently provisioned. The secondary region is the Azure disaster recovery (DR) paired region and also the region where the IoT hub can failover to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<iot_hub_location_description::Role>,
}
impl IotHubLocationDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod iot_hub_location_description {
    use super::*;
    #[doc = "The role of the region, can be either primary or secondary. The primary region is where the IoT hub is currently provisioned. The secondary region is the Azure disaster recovery (DR) paired region and also the region where the IoT hub can failover to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Role")]
    pub enum Role {
        #[serde(rename = "primary")]
        Primary,
        #[serde(rename = "secondary")]
        Secondary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Role {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Role {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Role {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("Role", 0u32, "primary"),
                Self::Secondary => serializer.serialize_unit_variant("Role", 1u32, "secondary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties indicating whether a given IoT hub name is available."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotHubNameAvailabilityInfo {
    #[doc = "The value which indicates whether the provided name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason for unavailability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<iot_hub_name_availability_info::Reason>,
    #[doc = "The detailed reason message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl IotHubNameAvailabilityInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod iot_hub_name_availability_info {
    use super::*;
    #[doc = "The reason for unavailability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[doc = "The properties of an IoT hub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotHubProperties {
    #[doc = "The shared access policies you can use to secure a connection to the IoT hub."]
    #[serde(rename = "authorizationPolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub authorization_policies: Vec<SharedAccessSignatureAuthorizationRule>,
    #[doc = "Whether requests from Public Network are allowed"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<iot_hub_properties::PublicNetworkAccess>,
    #[doc = "The IP filter rules."]
    #[serde(rename = "ipFilterRules", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_filter_rules: Vec<IpFilterRule>,
    #[doc = "Network Rule Set Properties of IotHub"]
    #[serde(rename = "networkRuleSets", default, skip_serializing_if = "Option::is_none")]
    pub network_rule_sets: Option<NetworkRuleSetProperties>,
    #[doc = "Specifies the minimum TLS version to support for this hub. Can be set to \"1.2\" to have clients that use a TLS version below 1.2 to be rejected."]
    #[serde(rename = "minTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub min_tls_version: Option<String>,
    #[doc = "Private endpoint connections created on this IotHub"]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "The provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The hub state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The name of the host."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "The Event Hub-compatible endpoint properties. The only possible keys to this dictionary is events. This key has to be present in the dictionary while making create or update calls for the IoT hub."]
    #[serde(rename = "eventHubEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_endpoints: Option<serde_json::Value>,
    #[doc = "The routing related properties of the IoT hub. See: https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-messaging"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<RoutingProperties>,
    #[doc = "The list of Azure Storage endpoints where you can upload files. Currently you can configure only one Azure Storage account and that MUST have its key as $default. Specifying more than one storage account causes an error to be thrown. Not specifying a value for this property when the enableFileUploadNotifications property is set to True, causes an error to be thrown."]
    #[serde(rename = "storageEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoints: Option<serde_json::Value>,
    #[doc = "The messaging endpoint properties for the file upload notification queue."]
    #[serde(rename = "messagingEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub messaging_endpoints: Option<serde_json::Value>,
    #[doc = "If True, file upload notifications are enabled."]
    #[serde(rename = "enableFileUploadNotifications", default, skip_serializing_if = "Option::is_none")]
    pub enable_file_upload_notifications: Option<bool>,
    #[doc = "The IoT hub cloud-to-device messaging properties."]
    #[serde(rename = "cloudToDevice", default, skip_serializing_if = "Option::is_none")]
    pub cloud_to_device: Option<CloudToDeviceProperties>,
    #[doc = "IoT hub comments."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[doc = "The device streams properties of iothub."]
    #[serde(rename = "deviceStreams", default, skip_serializing_if = "Option::is_none")]
    pub device_streams: Option<iot_hub_properties::DeviceStreams>,
    #[doc = "The capabilities and features enabled for the IoT hub."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<iot_hub_properties::Features>,
    #[doc = "The encryption properties for the IoT hub."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionPropertiesDescription>,
    #[doc = "Primary and secondary location for iot hub"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<IotHubLocationDescription>,
}
impl IotHubProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod iot_hub_properties {
    use super::*;
    #[doc = "Whether requests from Public Network are allowed"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The device streams properties of iothub."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DeviceStreams {
        #[doc = "List of Device Streams Endpoints."]
        #[serde(rename = "streamingEndpoints", default, skip_serializing_if = "Vec::is_empty")]
        pub streaming_endpoints: Vec<String>,
    }
    impl DeviceStreams {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The capabilities and features enabled for the IoT hub."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Features")]
    pub enum Features {
        None,
        DeviceManagement,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Features {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Features {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Features {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Features", 0u32, "None"),
                Self::DeviceManagement => serializer.serialize_unit_variant("Features", 1u32, "DeviceManagement"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Quota metrics properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotHubQuotaMetricInfo {
    #[doc = "The name of the quota metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The current value for the quota metric."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[doc = "The maximum value of the quota metric."]
    #[serde(rename = "maxValue", default, skip_serializing_if = "Option::is_none")]
    pub max_value: Option<i64>,
}
impl IotHubQuotaMetricInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON-serialized array of IotHubQuotaMetricInfo objects with a next link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotHubQuotaMetricInfoListResult {
    #[doc = "The array of quota metrics objects."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IotHubQuotaMetricInfo>,
    #[doc = "The next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IotHubQuotaMetricInfoListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IotHubQuotaMetricInfoListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SKU properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubSkuDescription {
    #[doc = "The type of the resource."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Information about the SKU of the IoT hub."]
    pub sku: IotHubSkuInfo,
    #[doc = "IoT Hub capacity information."]
    pub capacity: IotHubCapacity,
}
impl IotHubSkuDescription {
    pub fn new(sku: IotHubSkuInfo, capacity: IotHubCapacity) -> Self {
        Self {
            resource_type: None,
            sku,
            capacity,
        }
    }
}
#[doc = "The JSON-serialized array of IotHubSkuDescription objects with a next link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotHubSkuDescriptionListResult {
    #[doc = "The array of IotHubSkuDescription."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IotHubSkuDescription>,
    #[doc = "The next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IotHubSkuDescriptionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IotHubSkuDescriptionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the SKU of the IoT hub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubSkuInfo {
    #[doc = "The name of the SKU."]
    pub name: iot_hub_sku_info::Name,
    #[doc = "The billing tier for the IoT hub."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<iot_hub_sku_info::Tier>,
    #[doc = "The number of provisioned IoT Hub units. See: https://docs.microsoft.com/azure/azure-subscription-service-limits#iot-hub-limits."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
}
impl IotHubSkuInfo {
    pub fn new(name: iot_hub_sku_info::Name) -> Self {
        Self {
            name,
            tier: None,
            capacity: None,
        }
    }
}
pub mod iot_hub_sku_info {
    use super::*;
    #[doc = "The name of the SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        F1,
        S1,
        S2,
        S3,
        B1,
        B2,
        B3,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::F1 => serializer.serialize_unit_variant("Name", 0u32, "F1"),
                Self::S1 => serializer.serialize_unit_variant("Name", 1u32, "S1"),
                Self::S2 => serializer.serialize_unit_variant("Name", 2u32, "S2"),
                Self::S3 => serializer.serialize_unit_variant("Name", 3u32, "S3"),
                Self::B1 => serializer.serialize_unit_variant("Name", 4u32, "B1"),
                Self::B2 => serializer.serialize_unit_variant("Name", 5u32, "B2"),
                Self::B3 => serializer.serialize_unit_variant("Name", 6u32, "B3"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The billing tier for the IoT hub."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Free,
        Standard,
        Basic,
    }
}
#[doc = "The IP filter rules for the IoT hub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpFilterRule {
    #[doc = "The name of the IP filter rule."]
    #[serde(rename = "filterName")]
    pub filter_name: String,
    #[doc = "The desired action for requests captured by this rule."]
    pub action: ip_filter_rule::Action,
    #[doc = "A string that contains the IP address range in CIDR notation for the rule."]
    #[serde(rename = "ipMask")]
    pub ip_mask: String,
}
impl IpFilterRule {
    pub fn new(filter_name: String, action: ip_filter_rule::Action, ip_mask: String) -> Self {
        Self {
            filter_name,
            action,
            ip_mask,
        }
    }
}
pub mod ip_filter_rule {
    use super::*;
    #[doc = "The desired action for requests captured by this rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Accept,
        Reject,
    }
}
#[doc = "The properties of the Job Response object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobResponse {
    #[doc = "The job identifier."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "The start time of the job."]
    #[serde(rename = "startTimeUtc", with = "azure_core::date::rfc1123::option")]
    pub start_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The time the job stopped processing."]
    #[serde(rename = "endTimeUtc", with = "azure_core::date::rfc1123::option")]
    pub end_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The type of the job."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<job_response::Type>,
    #[doc = "The status of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<job_response::Status>,
    #[doc = "If status == failed, this string containing the reason for the failure."]
    #[serde(rename = "failureReason", default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[doc = "The status message for the job."]
    #[serde(rename = "statusMessage", default, skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[doc = "The job identifier of the parent job, if any."]
    #[serde(rename = "parentJobId", default, skip_serializing_if = "Option::is_none")]
    pub parent_job_id: Option<String>,
}
impl JobResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_response {
    use super::*;
    #[doc = "The type of the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "export")]
        Export,
        #[serde(rename = "import")]
        Import,
        #[serde(rename = "backup")]
        Backup,
        #[serde(rename = "readDeviceProperties")]
        ReadDeviceProperties,
        #[serde(rename = "writeDeviceProperties")]
        WriteDeviceProperties,
        #[serde(rename = "updateDeviceConfiguration")]
        UpdateDeviceConfiguration,
        #[serde(rename = "rebootDevice")]
        RebootDevice,
        #[serde(rename = "factoryResetDevice")]
        FactoryResetDevice,
        #[serde(rename = "firmwareUpdate")]
        FirmwareUpdate,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("Type", 0u32, "unknown"),
                Self::Export => serializer.serialize_unit_variant("Type", 1u32, "export"),
                Self::Import => serializer.serialize_unit_variant("Type", 2u32, "import"),
                Self::Backup => serializer.serialize_unit_variant("Type", 3u32, "backup"),
                Self::ReadDeviceProperties => serializer.serialize_unit_variant("Type", 4u32, "readDeviceProperties"),
                Self::WriteDeviceProperties => serializer.serialize_unit_variant("Type", 5u32, "writeDeviceProperties"),
                Self::UpdateDeviceConfiguration => serializer.serialize_unit_variant("Type", 6u32, "updateDeviceConfiguration"),
                Self::RebootDevice => serializer.serialize_unit_variant("Type", 7u32, "rebootDevice"),
                Self::FactoryResetDevice => serializer.serialize_unit_variant("Type", 8u32, "factoryResetDevice"),
                Self::FirmwareUpdate => serializer.serialize_unit_variant("Type", 9u32, "firmwareUpdate"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "enqueued")]
        Enqueued,
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "cancelled")]
        Cancelled,
    }
}
#[doc = "The JSON-serialized array of JobResponse objects with a next link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobResponseListResult {
    #[doc = "The array of JobResponse objects."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobResponse>,
    #[doc = "The next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobResponseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobResponseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the KeyVault identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KekIdentity {
    #[doc = "The user assigned identity."]
    #[serde(rename = "userAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identity: Option<String>,
}
impl KekIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the KeyVault key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultKeyProperties {
    #[doc = "The identifier of the key."]
    #[serde(rename = "keyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub key_identifier: Option<String>,
    #[doc = "The properties of the KeyVault identity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<KekIdentity>,
}
impl KeyVaultKeyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Routes that matched"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MatchedRoute {
    #[doc = "The properties of a routing rule that your IoT hub uses to route messages to endpoints."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RouteProperties>,
}
impl MatchedRoute {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the messaging endpoints used by this IoT hub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MessagingEndpointProperties {
    #[doc = "The lock duration. See: https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-file-upload."]
    #[serde(rename = "lockDurationAsIso8601", default, skip_serializing_if = "Option::is_none")]
    pub lock_duration_as_iso8601: Option<String>,
    #[doc = "The period of time for which a message is available to consume before it is expired by the IoT hub. See: https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-file-upload."]
    #[serde(rename = "ttlAsIso8601", default, skip_serializing_if = "Option::is_none")]
    pub ttl_as_iso8601: Option<String>,
    #[doc = "The number of times the IoT hub attempts to deliver a message. See: https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-file-upload."]
    #[serde(rename = "maxDeliveryCount", default, skip_serializing_if = "Option::is_none")]
    pub max_delivery_count: Option<i32>,
}
impl MessagingEndpointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Name of Iot Hub type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Name {
    #[doc = "IotHub type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Localized value of name"]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl Name {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IP Rule to be applied as part of Network Rule Set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRuleSetIpRule {
    #[doc = "Name of the IP filter rule."]
    #[serde(rename = "filterName")]
    pub filter_name: String,
    #[doc = "IP Filter Action"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<network_rule_set_ip_rule::Action>,
    #[doc = "A string that contains the IP address range in CIDR notation for the rule."]
    #[serde(rename = "ipMask")]
    pub ip_mask: String,
}
impl NetworkRuleSetIpRule {
    pub fn new(filter_name: String, ip_mask: String) -> Self {
        Self {
            filter_name,
            action: None,
            ip_mask,
        }
    }
}
pub mod network_rule_set_ip_rule {
    use super::*;
    #[doc = "IP Filter Action"]
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
#[doc = "Network Rule Set Properties of IotHub"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRuleSetProperties {
    #[doc = "Default Action for Network Rule Set"]
    #[serde(rename = "defaultAction", default, skip_serializing_if = "Option::is_none")]
    pub default_action: Option<network_rule_set_properties::DefaultAction>,
    #[doc = "If True, then Network Rule Set is also applied to BuiltIn EventHub EndPoint of IotHub"]
    #[serde(rename = "applyToBuiltInEventHubEndpoint")]
    pub apply_to_built_in_event_hub_endpoint: bool,
    #[doc = "List of IP Rules"]
    #[serde(rename = "ipRules")]
    pub ip_rules: Vec<NetworkRuleSetIpRule>,
}
impl NetworkRuleSetProperties {
    pub fn new(apply_to_built_in_event_hub_endpoint: bool, ip_rules: Vec<NetworkRuleSetIpRule>) -> Self {
        Self {
            default_action: None,
            apply_to_built_in_event_hub_endpoint,
            ip_rules,
        }
    }
}
pub mod network_rule_set_properties {
    use super::*;
    #[doc = "Default Action for Network Rule Set"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DefaultAction")]
    pub enum DefaultAction {
        Deny,
        Allow,
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
                Self::Deny => serializer.serialize_unit_variant("DefaultAction", 0u32, "Deny"),
                Self::Allow => serializer.serialize_unit_variant("DefaultAction", 1u32, "Allow"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for DefaultAction {
        fn default() -> Self {
            Self::Deny
        }
    }
}
#[doc = "IoT Hub REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{read | write | action | delete}"]
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
        #[doc = "Service provider: Microsoft Devices"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource Type: IotHubs"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Name of the operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of the operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Input values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationInputs {
    #[doc = "The name of the IoT hub to check."]
    pub name: String,
}
impl OperationInputs {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "Result of the request to list IoT Hub operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of IoT Hub operations supported by the Microsoft.Devices resource provider."]
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
#[doc = "The private endpoint property of a private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The private endpoint connection of an IotHub"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnection {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The properties of a private endpoint connection"]
    pub properties: PrivateEndpointConnectionProperties,
}
impl PrivateEndpointConnection {
    pub fn new(properties: PrivateEndpointConnectionProperties) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            properties,
        }
    }
}
#[doc = "The properties of a private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The private endpoint property of a private endpoint connection"]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "The current state of a private endpoint connection"]
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
}
impl PrivateEndpointConnectionProperties {
    pub fn new(private_link_service_connection_state: PrivateLinkServiceConnectionState) -> Self {
        Self {
            private_endpoint: None,
            private_link_service_connection_state,
        }
    }
}
pub type PrivateEndpointConnectionsList = Vec<PrivateEndpointConnection>;
#[doc = "The available private link resources for an IotHub"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResources {
    #[doc = "The list of available private link resources for an IotHub"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GroupIdInformation>,
}
impl PrivateLinkResources {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current state of a private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The status of a private endpoint connection"]
    pub status: private_link_service_connection_state::Status,
    #[doc = "The description for the current state of a private endpoint connection"]
    pub description: String,
    #[doc = "Actions required for a private endpoint connection"]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new(status: private_link_service_connection_state::Status, description: String) -> Self {
        Self {
            status,
            description,
            actions_required: None,
        }
    }
}
pub mod private_link_service_connection_state {
    use super::*;
    #[doc = "The status of a private endpoint connection"]
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
#[doc = "Identity registry statistics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryStatistics {
    #[doc = "The total count of devices in the identity registry."]
    #[serde(rename = "totalDeviceCount", default, skip_serializing_if = "Option::is_none")]
    pub total_device_count: Option<i64>,
    #[doc = "The count of enabled devices in the identity registry."]
    #[serde(rename = "enabledDeviceCount", default, skip_serializing_if = "Option::is_none")]
    pub enabled_device_count: Option<i64>,
    #[doc = "The count of disabled devices in the identity registry."]
    #[serde(rename = "disabledDeviceCount", default, skip_serializing_if = "Option::is_none")]
    pub disabled_device_count: Option<i64>,
}
impl RegistryStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The common properties of an Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource location."]
    pub location: String,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[doc = "Compilation error when evaluating route"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RouteCompilationError {
    #[doc = "Route error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Severity of the route error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<route_compilation_error::Severity>,
    #[doc = "Range of route errors"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<RouteErrorRange>,
}
impl RouteCompilationError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod route_compilation_error {
    use super::*;
    #[doc = "Severity of the route error"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Severity")]
    pub enum Severity {
        #[serde(rename = "error")]
        Error,
        #[serde(rename = "warning")]
        Warning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Severity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Severity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Severity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Error => serializer.serialize_unit_variant("Severity", 0u32, "error"),
                Self::Warning => serializer.serialize_unit_variant("Severity", 1u32, "warning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Position where the route error happened"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RouteErrorPosition {
    #[doc = "Line where the route error happened"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,
    #[doc = "Column where the route error happened"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,
}
impl RouteErrorPosition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Range of route errors"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RouteErrorRange {
    #[doc = "Position where the route error happened"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<RouteErrorPosition>,
    #[doc = "Position where the route error happened"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<RouteErrorPosition>,
}
impl RouteErrorRange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a routing rule that your IoT hub uses to route messages to endpoints."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteProperties {
    #[doc = "The name of the route. The name can only include alphanumeric characters, periods, underscores, hyphens, has a maximum length of 64 characters, and must be unique."]
    pub name: String,
    #[doc = "The source that the routing rule is to be applied to, such as DeviceMessages."]
    pub source: route_properties::Source,
    #[doc = "The condition that is evaluated to apply the routing rule. If no condition is provided, it evaluates to true by default. For grammar, see: https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-query-language"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "The list of endpoints to which messages that satisfy the condition are routed. Currently only one endpoint is allowed."]
    #[serde(rename = "endpointNames")]
    pub endpoint_names: Vec<String>,
    #[doc = "Used to specify whether a route is enabled."]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
impl RouteProperties {
    pub fn new(name: String, source: route_properties::Source, endpoint_names: Vec<String>, is_enabled: bool) -> Self {
        Self {
            name,
            source,
            condition: None,
            endpoint_names,
            is_enabled,
        }
    }
}
pub mod route_properties {
    use super::*;
    #[doc = "The source that the routing rule is to be applied to, such as DeviceMessages."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Source")]
    pub enum Source {
        Invalid,
        DeviceMessages,
        TwinChangeEvents,
        DeviceLifecycleEvents,
        DeviceJobLifecycleEvents,
        DigitalTwinChangeEvents,
        DeviceConnectionStateEvents,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Source {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Source {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Source {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("Source", 0u32, "Invalid"),
                Self::DeviceMessages => serializer.serialize_unit_variant("Source", 1u32, "DeviceMessages"),
                Self::TwinChangeEvents => serializer.serialize_unit_variant("Source", 2u32, "TwinChangeEvents"),
                Self::DeviceLifecycleEvents => serializer.serialize_unit_variant("Source", 3u32, "DeviceLifecycleEvents"),
                Self::DeviceJobLifecycleEvents => serializer.serialize_unit_variant("Source", 4u32, "DeviceJobLifecycleEvents"),
                Self::DigitalTwinChangeEvents => serializer.serialize_unit_variant("Source", 5u32, "DigitalTwinChangeEvents"),
                Self::DeviceConnectionStateEvents => serializer.serialize_unit_variant("Source", 6u32, "DeviceConnectionStateEvents"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties related to the custom endpoints to which your IoT hub routes messages based on the routing rules. A maximum of 10 custom endpoints are allowed across all endpoint types for paid hubs and only 1 custom endpoint is allowed across all endpoint types for free hubs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoutingEndpoints {
    #[doc = "The list of Service Bus queue endpoints that IoT hub routes the messages to, based on the routing rules."]
    #[serde(rename = "serviceBusQueues", default, skip_serializing_if = "Vec::is_empty")]
    pub service_bus_queues: Vec<RoutingServiceBusQueueEndpointProperties>,
    #[doc = "The list of Service Bus topic endpoints that the IoT hub routes the messages to, based on the routing rules."]
    #[serde(rename = "serviceBusTopics", default, skip_serializing_if = "Vec::is_empty")]
    pub service_bus_topics: Vec<RoutingServiceBusTopicEndpointProperties>,
    #[doc = "The list of Event Hubs endpoints that IoT hub routes messages to, based on the routing rules. This list does not include the built-in Event Hubs endpoint."]
    #[serde(rename = "eventHubs", default, skip_serializing_if = "Vec::is_empty")]
    pub event_hubs: Vec<RoutingEventHubProperties>,
    #[doc = "The list of storage container endpoints that IoT hub routes messages to, based on the routing rules."]
    #[serde(rename = "storageContainers", default, skip_serializing_if = "Vec::is_empty")]
    pub storage_containers: Vec<RoutingStorageContainerProperties>,
}
impl RoutingEndpoints {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties related to an event hub endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingEventHubProperties {
    #[doc = "Id of the event hub endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The connection string of the event hub endpoint. "]
    #[serde(rename = "connectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    #[doc = "The url of the event hub endpoint. It must include the protocol sb://"]
    #[serde(rename = "endpointUri", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_uri: Option<String>,
    #[doc = "Event hub name on the event hub namespace"]
    #[serde(rename = "entityPath", default, skip_serializing_if = "Option::is_none")]
    pub entity_path: Option<String>,
    #[doc = "Method used to authenticate against the event hub endpoint"]
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<routing_event_hub_properties::AuthenticationType>,
    #[doc = "The name that identifies this endpoint. The name can only include alphanumeric characters, periods, underscores, hyphens and has a maximum length of 64 characters. The following names are reserved:  events, fileNotifications, $default. Endpoint names must be unique across endpoint types."]
    pub name: String,
    #[doc = "The subscription identifier of the event hub endpoint."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The name of the resource group of the event hub endpoint."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl RoutingEventHubProperties {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            connection_string: None,
            endpoint_uri: None,
            entity_path: None,
            authentication_type: None,
            name,
            subscription_id: None,
            resource_group: None,
        }
    }
}
pub mod routing_event_hub_properties {
    use super::*;
    #[doc = "Method used to authenticate against the event hub endpoint"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthenticationType")]
    pub enum AuthenticationType {
        #[serde(rename = "keyBased")]
        KeyBased,
        #[serde(rename = "identityBased")]
        IdentityBased,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuthenticationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuthenticationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuthenticationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::KeyBased => serializer.serialize_unit_variant("AuthenticationType", 0u32, "keyBased"),
                Self::IdentityBased => serializer.serialize_unit_variant("AuthenticationType", 1u32, "identityBased"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Routing message"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoutingMessage {
    #[doc = "Body of routing message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[doc = "App properties"]
    #[serde(rename = "appProperties", default, skip_serializing_if = "Option::is_none")]
    pub app_properties: Option<serde_json::Value>,
    #[doc = "System properties"]
    #[serde(rename = "systemProperties", default, skip_serializing_if = "Option::is_none")]
    pub system_properties: Option<serde_json::Value>,
}
impl RoutingMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The routing related properties of the IoT hub. See: https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-messaging"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoutingProperties {
    #[doc = "The properties related to the custom endpoints to which your IoT hub routes messages based on the routing rules. A maximum of 10 custom endpoints are allowed across all endpoint types for paid hubs and only 1 custom endpoint is allowed across all endpoint types for free hubs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<RoutingEndpoints>,
    #[doc = "The list of user-provided routing rules that the IoT hub uses to route messages to built-in and custom endpoints. A maximum of 100 routing rules are allowed for paid hubs and a maximum of 5 routing rules are allowed for free hubs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub routes: Vec<RouteProperties>,
    #[doc = "The properties of the fallback route. IoT Hub uses these properties when it routes messages to the fallback endpoint."]
    #[serde(rename = "fallbackRoute", default, skip_serializing_if = "Option::is_none")]
    pub fallback_route: Option<FallbackRouteProperties>,
    #[doc = "The list of user-provided enrichments that the IoT hub applies to messages to be delivered to built-in and custom endpoints. See: https://aka.ms/telemetryoneventgrid"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enrichments: Vec<EnrichmentProperties>,
}
impl RoutingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties related to service bus queue endpoint types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingServiceBusQueueEndpointProperties {
    #[doc = "Id of the service bus queue endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The connection string of the service bus queue endpoint."]
    #[serde(rename = "connectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    #[doc = "The url of the service bus queue endpoint. It must include the protocol sb://"]
    #[serde(rename = "endpointUri", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_uri: Option<String>,
    #[doc = "Queue name on the service bus namespace"]
    #[serde(rename = "entityPath", default, skip_serializing_if = "Option::is_none")]
    pub entity_path: Option<String>,
    #[doc = "Method used to authenticate against the service bus queue endpoint"]
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<routing_service_bus_queue_endpoint_properties::AuthenticationType>,
    #[doc = "The name that identifies this endpoint. The name can only include alphanumeric characters, periods, underscores, hyphens and has a maximum length of 64 characters. The following names are reserved:  events, fileNotifications, $default. Endpoint names must be unique across endpoint types. The name need not be the same as the actual queue name."]
    pub name: String,
    #[doc = "The subscription identifier of the service bus queue endpoint."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The name of the resource group of the service bus queue endpoint."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl RoutingServiceBusQueueEndpointProperties {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            connection_string: None,
            endpoint_uri: None,
            entity_path: None,
            authentication_type: None,
            name,
            subscription_id: None,
            resource_group: None,
        }
    }
}
pub mod routing_service_bus_queue_endpoint_properties {
    use super::*;
    #[doc = "Method used to authenticate against the service bus queue endpoint"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthenticationType")]
    pub enum AuthenticationType {
        #[serde(rename = "keyBased")]
        KeyBased,
        #[serde(rename = "identityBased")]
        IdentityBased,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuthenticationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuthenticationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuthenticationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::KeyBased => serializer.serialize_unit_variant("AuthenticationType", 0u32, "keyBased"),
                Self::IdentityBased => serializer.serialize_unit_variant("AuthenticationType", 1u32, "identityBased"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties related to service bus topic endpoint types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingServiceBusTopicEndpointProperties {
    #[doc = "Id of the service bus topic endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The connection string of the service bus topic endpoint."]
    #[serde(rename = "connectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    #[doc = "The url of the service bus topic endpoint. It must include the protocol sb://"]
    #[serde(rename = "endpointUri", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_uri: Option<String>,
    #[doc = "Queue name on the service bus topic"]
    #[serde(rename = "entityPath", default, skip_serializing_if = "Option::is_none")]
    pub entity_path: Option<String>,
    #[doc = "Method used to authenticate against the service bus topic endpoint"]
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<routing_service_bus_topic_endpoint_properties::AuthenticationType>,
    #[doc = "The name that identifies this endpoint. The name can only include alphanumeric characters, periods, underscores, hyphens and has a maximum length of 64 characters. The following names are reserved:  events, fileNotifications, $default. Endpoint names must be unique across endpoint types.  The name need not be the same as the actual topic name."]
    pub name: String,
    #[doc = "The subscription identifier of the service bus topic endpoint."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The name of the resource group of the service bus topic endpoint."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl RoutingServiceBusTopicEndpointProperties {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            connection_string: None,
            endpoint_uri: None,
            entity_path: None,
            authentication_type: None,
            name,
            subscription_id: None,
            resource_group: None,
        }
    }
}
pub mod routing_service_bus_topic_endpoint_properties {
    use super::*;
    #[doc = "Method used to authenticate against the service bus topic endpoint"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthenticationType")]
    pub enum AuthenticationType {
        #[serde(rename = "keyBased")]
        KeyBased,
        #[serde(rename = "identityBased")]
        IdentityBased,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuthenticationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuthenticationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuthenticationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::KeyBased => serializer.serialize_unit_variant("AuthenticationType", 0u32, "keyBased"),
                Self::IdentityBased => serializer.serialize_unit_variant("AuthenticationType", 1u32, "identityBased"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties related to a storage container endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingStorageContainerProperties {
    #[doc = "Id of the storage container endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The connection string of the storage account."]
    #[serde(rename = "connectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    #[doc = "The url of the storage endpoint. It must include the protocol https://"]
    #[serde(rename = "endpointUri", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_uri: Option<String>,
    #[doc = "Method used to authenticate against the storage endpoint"]
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<routing_storage_container_properties::AuthenticationType>,
    #[doc = "The name that identifies this endpoint. The name can only include alphanumeric characters, periods, underscores, hyphens and has a maximum length of 64 characters. The following names are reserved:  events, fileNotifications, $default. Endpoint names must be unique across endpoint types."]
    pub name: String,
    #[doc = "The subscription identifier of the storage account."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The name of the resource group of the storage account."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "The name of storage container in the storage account."]
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "File name format for the blob. Default format is {iothub}/{partition}/{YYYY}/{MM}/{DD}/{HH}/{mm}. All parameters are mandatory but can be reordered."]
    #[serde(rename = "fileNameFormat", default, skip_serializing_if = "Option::is_none")]
    pub file_name_format: Option<String>,
    #[doc = "Time interval at which blobs are written to storage. Value should be between 60 and 720 seconds. Default value is 300 seconds."]
    #[serde(rename = "batchFrequencyInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub batch_frequency_in_seconds: Option<i32>,
    #[doc = "Maximum number of bytes for each blob written to storage. Value should be between 10485760(10MB) and 524288000(500MB). Default value is 314572800(300MB)."]
    #[serde(rename = "maxChunkSizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_chunk_size_in_bytes: Option<i32>,
    #[doc = "Encoding that is used to serialize messages to blobs. Supported values are 'avro', 'avrodeflate', and 'JSON'. Default value is 'avro'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<routing_storage_container_properties::Encoding>,
}
impl RoutingStorageContainerProperties {
    pub fn new(name: String, container_name: String) -> Self {
        Self {
            id: None,
            connection_string: None,
            endpoint_uri: None,
            authentication_type: None,
            name,
            subscription_id: None,
            resource_group: None,
            container_name,
            file_name_format: None,
            batch_frequency_in_seconds: None,
            max_chunk_size_in_bytes: None,
            encoding: None,
        }
    }
}
pub mod routing_storage_container_properties {
    use super::*;
    #[doc = "Method used to authenticate against the storage endpoint"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthenticationType")]
    pub enum AuthenticationType {
        #[serde(rename = "keyBased")]
        KeyBased,
        #[serde(rename = "identityBased")]
        IdentityBased,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuthenticationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuthenticationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuthenticationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::KeyBased => serializer.serialize_unit_variant("AuthenticationType", 0u32, "keyBased"),
                Self::IdentityBased => serializer.serialize_unit_variant("AuthenticationType", 1u32, "identityBased"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Encoding that is used to serialize messages to blobs. Supported values are 'avro', 'avrodeflate', and 'JSON'. Default value is 'avro'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Encoding {
        Avro,
        AvroDeflate,
        #[serde(rename = "JSON")]
        Json,
    }
}
#[doc = "Twin reference input parameter. This is an optional parameter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoutingTwin {
    #[doc = "Twin Tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<routing_twin::Properties>,
}
impl RoutingTwin {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod routing_twin {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Twin desired properties"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub desired: Option<serde_json::Value>,
        #[doc = "Twin desired properties"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub reported: Option<serde_json::Value>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The properties of an IoT hub shared access policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessSignatureAuthorizationRule {
    #[doc = "The name of the shared access policy."]
    #[serde(rename = "keyName")]
    pub key_name: String,
    #[doc = "The primary key."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "The secondary key."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "The permissions assigned to the shared access policy."]
    pub rights: shared_access_signature_authorization_rule::Rights,
}
impl SharedAccessSignatureAuthorizationRule {
    pub fn new(key_name: String, rights: shared_access_signature_authorization_rule::Rights) -> Self {
        Self {
            key_name,
            primary_key: None,
            secondary_key: None,
            rights,
        }
    }
}
pub mod shared_access_signature_authorization_rule {
    use super::*;
    #[doc = "The permissions assigned to the shared access policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Rights {
        RegistryRead,
        RegistryWrite,
        ServiceConnect,
        DeviceConnect,
        #[serde(rename = "RegistryRead, RegistryWrite")]
        RegistryReadRegistryWrite,
        #[serde(rename = "RegistryRead, ServiceConnect")]
        RegistryReadServiceConnect,
        #[serde(rename = "RegistryRead, DeviceConnect")]
        RegistryReadDeviceConnect,
        #[serde(rename = "RegistryWrite, ServiceConnect")]
        RegistryWriteServiceConnect,
        #[serde(rename = "RegistryWrite, DeviceConnect")]
        RegistryWriteDeviceConnect,
        #[serde(rename = "ServiceConnect, DeviceConnect")]
        ServiceConnectDeviceConnect,
        #[serde(rename = "RegistryRead, RegistryWrite, ServiceConnect")]
        RegistryReadRegistryWriteServiceConnect,
        #[serde(rename = "RegistryRead, RegistryWrite, DeviceConnect")]
        RegistryReadRegistryWriteDeviceConnect,
        #[serde(rename = "RegistryRead, ServiceConnect, DeviceConnect")]
        RegistryReadServiceConnectDeviceConnect,
        #[serde(rename = "RegistryWrite, ServiceConnect, DeviceConnect")]
        RegistryWriteServiceConnectDeviceConnect,
        #[serde(rename = "RegistryRead, RegistryWrite, ServiceConnect, DeviceConnect")]
        RegistryReadRegistryWriteServiceConnectDeviceConnect,
    }
}
#[doc = "The list of shared access policies with a next link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedAccessSignatureAuthorizationRuleListResult {
    #[doc = "The list of shared access policies."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SharedAccessSignatureAuthorizationRule>,
    #[doc = "The next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SharedAccessSignatureAuthorizationRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SharedAccessSignatureAuthorizationRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the Azure Storage endpoint for file upload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageEndpointProperties {
    #[doc = "The period of time for which the SAS URI generated by IoT Hub for file upload is valid. See: https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-file-upload#file-upload-notification-configuration-options."]
    #[serde(rename = "sasTtlAsIso8601", default, skip_serializing_if = "Option::is_none")]
    pub sas_ttl_as_iso8601: Option<String>,
    #[doc = "The connection string for the Azure Storage account to which files are uploaded."]
    #[serde(rename = "connectionString")]
    pub connection_string: String,
    #[doc = "The name of the root container where you upload files. The container need not exist but should be creatable using the connectionString specified."]
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "Specifies authentication type being used for connecting to the storage account."]
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<storage_endpoint_properties::AuthenticationType>,
}
impl StorageEndpointProperties {
    pub fn new(connection_string: String, container_name: String) -> Self {
        Self {
            sas_ttl_as_iso8601: None,
            connection_string,
            container_name,
            authentication_type: None,
        }
    }
}
pub mod storage_endpoint_properties {
    use super::*;
    #[doc = "Specifies authentication type being used for connecting to the storage account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthenticationType")]
    pub enum AuthenticationType {
        #[serde(rename = "keyBased")]
        KeyBased,
        #[serde(rename = "identityBased")]
        IdentityBased,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuthenticationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuthenticationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuthenticationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::KeyBased => serializer.serialize_unit_variant("AuthenticationType", 0u32, "keyBased"),
                Self::IdentityBased => serializer.serialize_unit_variant("AuthenticationType", 1u32, "identityBased"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A container holding only the Tags for a resource, allowing the user to update the tags on an IoT Hub instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsResource {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TagsResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input for testing all routes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestAllRoutesInput {
    #[doc = "Routing source"]
    #[serde(rename = "routingSource", default, skip_serializing_if = "Option::is_none")]
    pub routing_source: Option<test_all_routes_input::RoutingSource>,
    #[doc = "Routing message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<RoutingMessage>,
    #[doc = "Twin reference input parameter. This is an optional parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twin: Option<RoutingTwin>,
}
impl TestAllRoutesInput {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_all_routes_input {
    use super::*;
    #[doc = "Routing source"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RoutingSource")]
    pub enum RoutingSource {
        Invalid,
        DeviceMessages,
        TwinChangeEvents,
        DeviceLifecycleEvents,
        DeviceJobLifecycleEvents,
        DigitalTwinChangeEvents,
        DeviceConnectionStateEvents,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RoutingSource {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RoutingSource {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RoutingSource {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("RoutingSource", 0u32, "Invalid"),
                Self::DeviceMessages => serializer.serialize_unit_variant("RoutingSource", 1u32, "DeviceMessages"),
                Self::TwinChangeEvents => serializer.serialize_unit_variant("RoutingSource", 2u32, "TwinChangeEvents"),
                Self::DeviceLifecycleEvents => serializer.serialize_unit_variant("RoutingSource", 3u32, "DeviceLifecycleEvents"),
                Self::DeviceJobLifecycleEvents => serializer.serialize_unit_variant("RoutingSource", 4u32, "DeviceJobLifecycleEvents"),
                Self::DigitalTwinChangeEvents => serializer.serialize_unit_variant("RoutingSource", 5u32, "DigitalTwinChangeEvents"),
                Self::DeviceConnectionStateEvents => {
                    serializer.serialize_unit_variant("RoutingSource", 6u32, "DeviceConnectionStateEvents")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result of testing all routes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestAllRoutesResult {
    #[doc = "JSON-serialized array of matched routes"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub routes: Vec<MatchedRoute>,
}
impl TestAllRoutesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input for testing route"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestRouteInput {
    #[doc = "Routing message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<RoutingMessage>,
    #[doc = "The properties of a routing rule that your IoT hub uses to route messages to endpoints."]
    pub route: RouteProperties,
    #[doc = "Twin reference input parameter. This is an optional parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twin: Option<RoutingTwin>,
}
impl TestRouteInput {
    pub fn new(route: RouteProperties) -> Self {
        Self {
            message: None,
            route,
            twin: None,
        }
    }
}
#[doc = "Result of testing one route"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRouteResult {
    #[doc = "Result of testing route"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<test_route_result::Result>,
    #[doc = "Detailed result of testing a route"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<TestRouteResultDetails>,
}
impl TestRouteResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_route_result {
    use super::*;
    #[doc = "Result of testing route"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Result")]
    pub enum Result {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "false")]
        False,
        #[serde(rename = "true")]
        True,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Result {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Result {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Result {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Undefined => serializer.serialize_unit_variant("Result", 0u32, "undefined"),
                Self::False => serializer.serialize_unit_variant("Result", 1u32, "false"),
                Self::True => serializer.serialize_unit_variant("Result", 2u32, "true"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Detailed result of testing a route"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRouteResultDetails {
    #[doc = "JSON-serialized list of route compilation errors"]
    #[serde(rename = "compilationErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub compilation_errors: Vec<RouteCompilationError>,
}
impl TestRouteResultDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User subscription quota response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserSubscriptionQuota {
    #[doc = "IotHub type id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Response type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Unit of IotHub type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Current number of IotHub type"]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i32>,
    #[doc = "Numerical limit on IotHub type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "Name of Iot Hub type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
}
impl UserSubscriptionQuota {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Json-serialized array of User subscription quota response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserSubscriptionQuotaListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UserSubscriptionQuota>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl UserSubscriptionQuotaListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
