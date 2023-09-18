#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Defines values for AccessRights."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AccessRights")]
pub enum AccessRights {
    Manage,
    Send,
    Listen,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AccessRights {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AccessRights {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AccessRights {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Manage => serializer.serialize_unit_variant("AccessRights", 0u32, "Manage"),
            Self::Send => serializer.serialize_unit_variant("AccessRights", 1u32, "Send"),
            Self::Listen => serializer.serialize_unit_variant("AccessRights", 2u32, "Listen"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Description of a NotificationHub AdmCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdmCredential {
    #[doc = "Description of a NotificationHub AdmCredential."]
    pub properties: AdmCredentialProperties,
}
impl AdmCredential {
    pub fn new(properties: AdmCredentialProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Description of a NotificationHub AdmCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdmCredentialProperties {
    #[doc = "Gets or sets the client identifier."]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "Gets or sets the credential secret access key."]
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    #[doc = "Gets or sets the URL of the authorization token."]
    #[serde(rename = "authTokenUrl")]
    pub auth_token_url: String,
}
impl AdmCredentialProperties {
    pub fn new(client_id: String, client_secret: String, auth_token_url: String) -> Self {
        Self {
            client_id,
            client_secret,
            auth_token_url,
        }
    }
}
#[doc = "Description of a NotificationHub ApnsCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApnsCredential {
    #[doc = "Description of a NotificationHub ApnsCredential."]
    pub properties: ApnsCredentialProperties,
}
impl ApnsCredential {
    pub fn new(properties: ApnsCredentialProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Description of a NotificationHub ApnsCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApnsCredentialProperties {
    #[doc = "Gets or sets the APNS certificate."]
    #[serde(rename = "apnsCertificate", default, skip_serializing_if = "Option::is_none")]
    pub apns_certificate: Option<String>,
    #[doc = "Gets or sets the certificate key."]
    #[serde(rename = "certificateKey", default, skip_serializing_if = "Option::is_none")]
    pub certificate_key: Option<String>,
    #[doc = "Gets or sets the endpoint of this credential."]
    pub endpoint: String,
    #[doc = "Gets or sets the APNS certificate Thumbprint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "Gets or sets a 10-character key identifier (kid) key, obtained from\r\nyour developer account"]
    #[serde(rename = "keyId", default, skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[doc = "Gets or sets the name of the application"]
    #[serde(rename = "appName", default, skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[doc = "Gets or sets the issuer (iss) registered claim key, whose value is\r\nyour 10-character Team ID, obtained from your developer account"]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "Gets or sets provider Authentication Token, obtained through your\r\ndeveloper account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
impl ApnsCredentialProperties {
    pub fn new(endpoint: String) -> Self {
        Self {
            apns_certificate: None,
            certificate_key: None,
            endpoint,
            thumbprint: None,
            key_id: None,
            app_name: None,
            app_id: None,
            token: None,
        }
    }
}
#[doc = "Represents metric availability (part of RP operation descriptions)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Availability {
    #[doc = "Time grain of the availability."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "Duration of the availability blob."]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl Availability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub BaiduCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaiduCredential {
    #[doc = "Description of a NotificationHub BaiduCredential."]
    pub properties: BaiduCredentialProperties,
}
impl BaiduCredential {
    pub fn new(properties: BaiduCredentialProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Description of a NotificationHub BaiduCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaiduCredentialProperties {
    #[doc = "Gets or sets baidu Api Key."]
    #[serde(rename = "baiduApiKey")]
    pub baidu_api_key: String,
    #[doc = "Gets or sets baidu Endpoint."]
    #[serde(rename = "baiduEndPoint")]
    pub baidu_end_point: String,
    #[doc = "Gets or sets baidu Secret Key"]
    #[serde(rename = "baiduSecretKey")]
    pub baidu_secret_key: String,
}
impl BaiduCredentialProperties {
    pub fn new(baidu_api_key: String, baidu_end_point: String, baidu_secret_key: String) -> Self {
        Self {
            baidu_api_key,
            baidu_end_point,
            baidu_secret_key,
        }
    }
}
#[doc = "Description of a NotificationHub BrowserCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrowserCredential {
    #[doc = "Description of a NotificationHub BrowserCredential."]
    pub properties: BrowserCredentialProperties,
}
impl BrowserCredential {
    pub fn new(properties: BrowserCredentialProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Description of a NotificationHub BrowserCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrowserCredentialProperties {
    #[doc = "Gets or sets web push subject."]
    pub subject: String,
    #[doc = "Gets or sets VAPID private key."]
    #[serde(rename = "vapidPrivateKey")]
    pub vapid_private_key: String,
    #[doc = "Gets or sets VAPID public key."]
    #[serde(rename = "vapidPublicKey")]
    pub vapid_public_key: String,
}
impl BrowserCredentialProperties {
    pub fn new(subject: String, vapid_private_key: String, vapid_public_key: String) -> Self {
        Self {
            subject,
            vapid_private_key,
            vapid_public_key,
        }
    }
}
#[doc = "Parameters supplied to the Check Name Availability for Namespace and\r\nNotificationHubs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckAvailabilityParameters {
    #[doc = "Gets resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets resource name"]
    pub name: String,
    #[doc = "Gets resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets or sets resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Not used and deprecated since API version 2023-09-01"]
    #[serde(rename = "isAvailiable", default, skip_serializing_if = "Option::is_none")]
    pub is_availiable: Option<bool>,
    #[doc = "The Sku description for a namespace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl CheckAvailabilityParameters {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            name,
            type_: None,
            location: None,
            tags: None,
            is_availiable: None,
            sku: None,
        }
    }
}
#[doc = "Description of a CheckAvailability resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckAvailabilityResult {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Gets or sets true if the name is available and can be used to\r\ncreate new Namespace/NotificationHub. Otherwise false."]
    #[serde(rename = "isAvailiable", default, skip_serializing_if = "Option::is_none")]
    pub is_availiable: Option<bool>,
    #[doc = "Deprecated - only for compatibility."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Deprecated - only for compatibility."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The Sku description for a namespace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl CheckAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Part of Private Endpoint description that stores information about a connection between Private Endpoint and Notification Hubs namespace.\r\nThis is internal class, not visible to customers, and we use it only to discover the link identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionDetails {
    #[doc = "A unique ID of the connection. This is not the ARM id, but rather an internal identifier set by the Networking RP. Notification Hubs code\r\ndoes not analyze it."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "IP address of the Private Endpoint. This is not used by Notification Hubs."]
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "Link identifier. This is a string representation of an integer that is also encoded in every IPv6 frame received by Front Door,\r\nand we use it to create implicit authorization rule that allows connection from the associated Private Endpoint."]
    #[serde(rename = "linkIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub link_identifier: Option<String>,
    #[doc = "Group name. Always \"namespace\" for Notification Hubs."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "Member name. Always \"namespace\" for Notification Hubs."]
    #[serde(rename = "memberName", default, skip_serializing_if = "Option::is_none")]
    pub member_name: Option<String>,
}
impl ConnectionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DebugSendResponse {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Result of DebugSend operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DebugSendResult>,
    #[doc = "Deprecated - only for compatibility."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Deprecated - only for compatibility."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DebugSendResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of DebugSend operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DebugSendResult {
    #[doc = "Gets or sets successful send"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<i64>,
    #[doc = "Gets or sets send failure"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure: Option<i64>,
    #[doc = "Gets or sets actual failure description"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<RegistrationResult>,
}
impl DebugSendResult {
    pub fn new() -> Self {
        Self::default()
    }
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
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
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
#[doc = "Description of a NotificationHub GcmCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcmCredential {
    #[doc = "Description of a NotificationHub GcmCredential."]
    pub properties: GcmCredentialProperties,
}
impl GcmCredential {
    pub fn new(properties: GcmCredentialProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Description of a NotificationHub GcmCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcmCredentialProperties {
    #[doc = "Gets or sets the GCM endpoint."]
    #[serde(rename = "gcmEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub gcm_endpoint: Option<String>,
    #[doc = "Gets or sets the Google API key."]
    #[serde(rename = "googleApiKey")]
    pub google_api_key: String,
}
impl GcmCredentialProperties {
    pub fn new(google_api_key: String) -> Self {
        Self {
            gcm_endpoint: None,
            google_api_key,
        }
    }
}
#[doc = "Represents a connectivity information to Notification Hubs namespace. This is part of PrivateLinkService proxy that tell\r\nthe Networking RP how to connect to the Notification Hubs namespace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupConnectivityInformation {
    #[doc = "Group id. Always set to \"namespace\"."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "Member name. Always set to \"namespace\"."]
    #[serde(rename = "memberName", default, skip_serializing_if = "Option::is_none")]
    pub member_name: Option<String>,
    #[doc = "List of customer-visible domain names that point to a Notification Hubs namespace."]
    #[serde(
        rename = "customerVisibleFqdns",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub customer_visible_fqdns: Vec<String>,
    #[doc = "One of the domain name from the customer-visible names; this is used internally by Private Link service to make connection to Notification Hubs\r\nnamespace."]
    #[serde(rename = "internalFqdn", default, skip_serializing_if = "Option::is_none")]
    pub internal_fqdn: Option<String>,
    #[doc = "Not used by Notification Hubs."]
    #[serde(rename = "redirectMapId", default, skip_serializing_if = "Option::is_none")]
    pub redirect_map_id: Option<String>,
    #[doc = "ARM region for Private Link Service. We use the region that contains the connected Notification Hubs namespace."]
    #[serde(rename = "privateLinkServiceArmRegion", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_arm_region: Option<String>,
}
impl GroupConnectivityInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A network authorization rule that filters traffic based on IP address."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpRule {
    #[doc = "IP mask."]
    #[serde(rename = "ipMask")]
    pub ip_mask: String,
    #[doc = "List of access rights."]
    pub rights: Vec<AccessRights>,
}
impl IpRule {
    pub fn new(ip_mask: String, rights: Vec<AccessRights>) -> Self {
        Self { ip_mask, rights }
    }
}
#[doc = "A single log category specification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[doc = "Name of the log category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of the log category."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Duration of data written to a single blob."]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A metric specification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[doc = "Metric name / id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "User-visible metric name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Description of the metric."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "Metric unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Type of the aggregation (Average, Minimum, Maximum, Total or Count)."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "List of availabilities."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub availabilities: Vec<Availability>,
    #[doc = "The matching regex pattern to be applied to the field pointed by the \"metricsFilterPathSelector\" flag in the ARM manifest."]
    #[serde(rename = "metricFilterPattern", default, skip_serializing_if = "Option::is_none")]
    pub metric_filter_pattern: Option<String>,
    #[doc = "Optional property. If set to true, then zero will be returned for time duration where no metric is emitted / published."]
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub MpnsCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpnsCredential {
    #[doc = "Description of a NotificationHub MpnsCredential."]
    pub properties: MpnsCredentialProperties,
}
impl MpnsCredential {
    pub fn new(properties: MpnsCredentialProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Description of a NotificationHub MpnsCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpnsCredentialProperties {
    #[doc = "Gets or sets the MPNS certificate."]
    #[serde(rename = "mpnsCertificate")]
    pub mpns_certificate: String,
    #[doc = "Gets or sets the certificate key for this credential."]
    #[serde(rename = "certificateKey")]
    pub certificate_key: String,
    #[doc = "Gets or sets the MPNS certificate Thumbprint"]
    pub thumbprint: String,
}
impl MpnsCredentialProperties {
    pub fn new(mpns_certificate: String, certificate_key: String, thumbprint: String) -> Self {
        Self {
            mpns_certificate,
            certificate_key,
            thumbprint,
        }
    }
}
#[doc = "The response of the List Namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamespaceListResult {
    #[doc = "Gets or sets result of the List AuthorizationRules operation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NamespaceResource>,
    #[doc = "Gets or sets link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NamespaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NamespaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Patch parameter for NamespaceResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamespacePatchParameters {
    #[doc = "The Sku description for a namespace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Represents namespace properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamespaceProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl NamespacePatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents namespace properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamespaceProperties {
    #[doc = "Name of the Notification Hubs namespace. This is immutable property, set automatically \r\nby the service when the namespace is created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Defines values for OperationProvisioningState."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<OperationProvisioningState>,
    #[doc = "Namespace status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<NamespaceStatus>,
    #[doc = "Gets or sets whether or not the namespace is currently enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Gets or sets whether or not the namespace is set as Critical."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical: Option<bool>,
    #[doc = "Namespace subscription id."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Region. The value is always set to the same value as Namespace.Location, so we are deprecating\r\nthis property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "Azure Insights Metrics id."]
    #[serde(rename = "metricId", default, skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<String>,
    #[doc = "Time when the namespace was created."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "Time when the namespace was updated."]
    #[serde(rename = "updatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub updated_at: Option<time::OffsetDateTime>,
    #[doc = "Defines values for NamespaceType."]
    #[serde(rename = "namespaceType", default, skip_serializing_if = "Option::is_none")]
    pub namespace_type: Option<NamespaceType>,
    #[doc = "Allowed replication region"]
    #[serde(rename = "replicationRegion", default, skip_serializing_if = "Option::is_none")]
    pub replication_region: Option<ReplicationRegion>,
    #[doc = "Namespace SKU name."]
    #[serde(rename = "zoneRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundancy: Option<ZoneRedundancyPreference>,
    #[doc = "A collection of network authorization rules."]
    #[serde(rename = "networkAcls", default, skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkAcls>,
    #[doc = "Collection of Notification Hub or Notification Hub Namespace PNS credentials."]
    #[serde(rename = "pnsCredentials", default, skip_serializing_if = "Option::is_none")]
    pub pns_credentials: Option<PnsCredentials>,
    #[doc = "Gets or sets endpoint you can use to perform NotificationHub\r\noperations."]
    #[serde(rename = "serviceBusEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_endpoint: Option<String>,
    #[doc = "Private Endpoint Connections for namespace"]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<PrivateEndpointConnectionResource>,
    #[doc = "Gets or sets scaleUnit where the namespace gets created"]
    #[serde(rename = "scaleUnit", default, skip_serializing_if = "Option::is_none")]
    pub scale_unit: Option<String>,
    #[doc = "Deprecated."]
    #[serde(rename = "dataCenter", default, skip_serializing_if = "Option::is_none")]
    pub data_center: Option<String>,
    #[doc = "Type of public network access."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccess>,
}
impl NamespaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Notification Hubs Namespace Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The Sku description for a namespace"]
    pub sku: Sku,
    #[doc = "Represents namespace properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamespaceProperties>,
}
impl NamespaceResource {
    pub fn new(tracked_resource: TrackedResource, sku: Sku) -> Self {
        Self {
            tracked_resource,
            sku,
            properties: None,
        }
    }
}
#[doc = "Namespace status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NamespaceStatus")]
pub enum NamespaceStatus {
    Created,
    Creating,
    Suspended,
    Deleting,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NamespaceStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NamespaceStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NamespaceStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Created => serializer.serialize_unit_variant("NamespaceStatus", 0u32, "Created"),
            Self::Creating => serializer.serialize_unit_variant("NamespaceStatus", 1u32, "Creating"),
            Self::Suspended => serializer.serialize_unit_variant("NamespaceStatus", 2u32, "Suspended"),
            Self::Deleting => serializer.serialize_unit_variant("NamespaceStatus", 3u32, "Deleting"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines values for NamespaceType."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NamespaceType")]
pub enum NamespaceType {
    Messaging,
    NotificationHub,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NamespaceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NamespaceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NamespaceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Messaging => serializer.serialize_unit_variant("NamespaceType", 0u32, "Messaging"),
            Self::NotificationHub => serializer.serialize_unit_variant("NamespaceType", 1u32, "NotificationHub"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A collection of network authorization rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkAcls {
    #[doc = "List of IP rules."]
    #[serde(
        rename = "ipRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_rules: Vec<IpRule>,
    #[doc = "A default (public Internet) network authorization rule, which contains rights if no other network rule matches."]
    #[serde(rename = "publicNetworkRule", default, skip_serializing_if = "Option::is_none")]
    pub public_network_rule: Option<PublicInternetAuthorizationRule>,
}
impl NetworkAcls {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of the List NotificationHub operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationHubListResult {
    #[doc = "Gets or sets result of the List AuthorizationRules operation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NotificationHubResource>,
    #[doc = "Gets or sets link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NotificationHubListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NotificationHubListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Patch parameter for NamespaceResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationHubPatchParameters {
    #[doc = "NotificationHub properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NotificationHubProperties>,
    #[doc = "The Sku description for a namespace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl NotificationHubPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NotificationHub properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationHubProperties {
    #[doc = "Gets or sets the NotificationHub name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the RegistrationTtl of the created NotificationHub"]
    #[serde(rename = "registrationTtl", default, skip_serializing_if = "Option::is_none")]
    pub registration_ttl: Option<String>,
    #[doc = "Gets or sets the AuthorizationRules of the created NotificationHub"]
    #[serde(
        rename = "authorizationRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub authorization_rules: Vec<SharedAccessAuthorizationRuleProperties>,
    #[doc = "Description of a NotificationHub ApnsCredential."]
    #[serde(rename = "apnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub apns_credential: Option<ApnsCredential>,
    #[doc = "Description of a NotificationHub WnsCredential."]
    #[serde(rename = "wnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub wns_credential: Option<WnsCredential>,
    #[doc = "Description of a NotificationHub GcmCredential."]
    #[serde(rename = "gcmCredential", default, skip_serializing_if = "Option::is_none")]
    pub gcm_credential: Option<GcmCredential>,
    #[doc = "Description of a NotificationHub MpnsCredential."]
    #[serde(rename = "mpnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub mpns_credential: Option<MpnsCredential>,
    #[doc = "Description of a NotificationHub AdmCredential."]
    #[serde(rename = "admCredential", default, skip_serializing_if = "Option::is_none")]
    pub adm_credential: Option<AdmCredential>,
    #[doc = "Description of a NotificationHub BaiduCredential."]
    #[serde(rename = "baiduCredential", default, skip_serializing_if = "Option::is_none")]
    pub baidu_credential: Option<BaiduCredential>,
    #[doc = "Description of a NotificationHub BrowserCredential."]
    #[serde(rename = "browserCredential", default, skip_serializing_if = "Option::is_none")]
    pub browser_credential: Option<BrowserCredential>,
    #[doc = "Description of a NotificationHub XiaomiCredential."]
    #[serde(rename = "xiaomiCredential", default, skip_serializing_if = "Option::is_none")]
    pub xiaomi_credential: Option<XiaomiCredential>,
    #[serde(rename = "dailyMaxActiveDevices", default, skip_serializing_if = "Option::is_none")]
    pub daily_max_active_devices: Option<i64>,
}
impl NotificationHubProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Notification Hub Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationHubResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "NotificationHub properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NotificationHubProperties>,
    #[doc = "The Sku description for a namespace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl NotificationHubResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            sku: None,
        }
    }
}
#[doc = "A NotificationHubs REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Gets operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Optional operation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
    #[doc = "Gets or sets IsDataAction property. It is used to differentiate management and data plane operations."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object that represents the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Gets service provider: Microsoft.NotificationHubs"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Gets resource on which the operation is performed: Invoice, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Gets operation type: Read, write, delete, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Human-friendly operation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list NotificationHubs operations. It contains\r\na list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "Gets list of NotificationHubs operations supported by the\r\nMicrosoft.NotificationHubs resource provider."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "Gets URL to get the next set of operation list results if there are\r\nany."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Optional operation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "Optional service specification used in Operations API."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines values for OperationProvisioningState."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperationProvisioningState")]
pub enum OperationProvisioningState {
    Unknown,
    InProgress,
    Succeeded,
    Failed,
    Canceled,
    Pending,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperationProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperationProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperationProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("OperationProvisioningState", 0u32, "Unknown"),
            Self::InProgress => serializer.serialize_unit_variant("OperationProvisioningState", 1u32, "InProgress"),
            Self::Succeeded => serializer.serialize_unit_variant("OperationProvisioningState", 2u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("OperationProvisioningState", 3u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("OperationProvisioningState", 4u32, "Canceled"),
            Self::Pending => serializer.serialize_unit_variant("OperationProvisioningState", 5u32, "Pending"),
            Self::Disabled => serializer.serialize_unit_variant("OperationProvisioningState", 6u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Collection of Notification Hub or Notification Hub Namespace PNS credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PnsCredentials {
    #[doc = "Description of a NotificationHub AdmCredential."]
    #[serde(rename = "admCredential", default, skip_serializing_if = "Option::is_none")]
    pub adm_credential: Option<AdmCredential>,
    #[doc = "Description of a NotificationHub ApnsCredential."]
    #[serde(rename = "apnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub apns_credential: Option<ApnsCredential>,
    #[doc = "Description of a NotificationHub BaiduCredential."]
    #[serde(rename = "baiduCredential", default, skip_serializing_if = "Option::is_none")]
    pub baidu_credential: Option<BaiduCredential>,
    #[doc = "Description of a NotificationHub BrowserCredential."]
    #[serde(rename = "browserCredential", default, skip_serializing_if = "Option::is_none")]
    pub browser_credential: Option<BrowserCredential>,
    #[doc = "Description of a NotificationHub GcmCredential."]
    #[serde(rename = "gcmCredential", default, skip_serializing_if = "Option::is_none")]
    pub gcm_credential: Option<GcmCredential>,
    #[doc = "Description of a NotificationHub MpnsCredential."]
    #[serde(rename = "mpnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub mpns_credential: Option<MpnsCredential>,
    #[doc = "Description of a NotificationHub WnsCredential."]
    #[serde(rename = "wnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub wns_credential: Option<WnsCredential>,
    #[doc = "Description of a NotificationHub XiaomiCredential."]
    #[serde(rename = "xiaomiCredential", default, skip_serializing_if = "Option::is_none")]
    pub xiaomi_credential: Option<XiaomiCredential>,
}
impl PnsCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub PNS Credentials. This is a response of the POST requests that return namespace or hubs\r\nPNS credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PnsCredentialsResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Collection of Notification Hub or Notification Hub Namespace PNS credentials."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PnsCredentials>,
    #[doc = "Deprecated - only for compatibility."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Deprecated - only for compatibility."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PnsCredentialsResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Namespace / NotificationHub Regenerate Keys request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyKeyResource {
    #[doc = "Type of Shared Access Policy Key (primary or secondary)."]
    #[serde(rename = "policyKey")]
    pub policy_key: PolicyKeyType,
}
impl PolicyKeyResource {
    pub fn new(policy_key: PolicyKeyType) -> Self {
        Self { policy_key }
    }
}
#[doc = "Type of Shared Access Policy Key (primary or secondary)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PolicyKeyType")]
pub enum PolicyKeyType {
    PrimaryKey,
    SecondaryKey,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PolicyKeyType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PolicyKeyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PolicyKeyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::PrimaryKey => serializer.serialize_unit_variant("PolicyKeyType", 0u32, "PrimaryKey"),
            Self::SecondaryKey => serializer.serialize_unit_variant("PolicyKeyType", 1u32, "SecondaryKey"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Private Endpoint Connection properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "State of Private Endpoint Connection."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
    #[doc = "Represents a Private Endpoint that is connected to Notification Hubs namespace using Private Endpoint Connection."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<RemotePrivateEndpointConnection>,
    #[doc = "List of group ids. For Notification Hubs, it always contains a single \"namespace\" element."]
    #[serde(
        rename = "groupIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_ids: Vec<String>,
    #[doc = "State of the Private Link Service connection."]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<RemotePrivateLinkServiceConnectionState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "State of Private Endpoint Connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointConnectionProvisioningState")]
pub enum PrivateEndpointConnectionProvisioningState {
    Unknown,
    Succeeded,
    Creating,
    Updating,
    UpdatingByProxy,
    Deleting,
    DeletingByProxy,
    Deleted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointConnectionProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointConnectionProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointConnectionProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 0u32, "Unknown"),
            Self::Succeeded => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 1u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 2u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 3u32, "Updating"),
            Self::UpdatingByProxy => {
                serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 4u32, "UpdatingByProxy")
            }
            Self::Deleting => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 5u32, "Deleting"),
            Self::DeletingByProxy => {
                serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 6u32, "DeletingByProxy")
            }
            Self::Deleted => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 7u32, "Deleted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents a Private Endpoint Connection ARM resource - a sub-resource of Notification Hubs namespace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Private Endpoint Connection properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnectionResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of the List Private Endpoint Connections operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionResourceListResult {
    #[doc = "Gets or sets result of the List AuthorizationRules operation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateEndpointConnectionResource>,
    #[doc = "Gets or sets link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PrivateEndpointConnectionResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "State of Private Link Connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateLinkConnectionStatus")]
pub enum PrivateLinkConnectionStatus {
    Disconnected,
    Pending,
    Approved,
    Rejected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateLinkConnectionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateLinkConnectionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateLinkConnectionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disconnected => serializer.serialize_unit_variant("PrivateLinkConnectionStatus", 0u32, "Disconnected"),
            Self::Pending => serializer.serialize_unit_variant("PrivateLinkConnectionStatus", 1u32, "Pending"),
            Self::Approved => serializer.serialize_unit_variant("PrivateLinkConnectionStatus", 2u32, "Approved"),
            Self::Rejected => serializer.serialize_unit_variant("PrivateLinkConnectionStatus", 3u32, "Rejected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A Private Link Arm Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Represents properties of Private Link Resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of the List Private Link Resources operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[doc = "Gets or sets result of the List AuthorizationRules operation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "Gets or sets link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateLinkResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents properties of Private Link Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "A Group Id for Private Link. For Notification Hubs, it is always set to \"namespace\"."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "Required members. For Notification Hubs, it's always a collection with a single \"namespace\" item."]
    #[serde(
        rename = "requiredMembers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_members: Vec<String>,
    #[doc = "Required DNS zone names. For Notification Hubs, it contains two CNames for Service Bus and Notification Hubs zones."]
    #[serde(
        rename = "requiredZoneNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A customer-visible sub-resource of Private Endpoint, which describe the connection between Private Endpoint and Notification Hubs namespace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnection {
    #[doc = "Name of the Private Link Service connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "List of group ids. Always contains a single element - \"namespace\" - for Notification Hub Namespace."]
    #[serde(
        rename = "groupIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_ids: Vec<String>,
    #[doc = "Request message provided by the user that created the connection. This is usually used when the connection requires manual approval."]
    #[serde(rename = "requestMessage", default, skip_serializing_if = "Option::is_none")]
    pub request_message: Option<String>,
}
impl PrivateLinkServiceConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for a Azure Resource Manager proxy resource. It will not have tags and a location"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A default (public Internet) network authorization rule, which contains rights if no other network rule matches."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicInternetAuthorizationRule {
    #[doc = "List of access rights."]
    pub rights: Vec<AccessRights>,
}
impl PublicInternetAuthorizationRule {
    pub fn new(rights: Vec<AccessRights>) -> Self {
        Self { rights }
    }
}
#[doc = "Type of public network access."]
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
impl Default for PublicNetworkAccess {
    fn default() -> Self {
        Self::Enabled
    }
}
#[doc = "Notification result for a single registration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationResult {
    #[doc = "PNS type."]
    #[serde(rename = "applicationPlatform", default, skip_serializing_if = "Option::is_none")]
    pub application_platform: Option<String>,
    #[doc = "PNS handle."]
    #[serde(rename = "pnsHandle", default, skip_serializing_if = "Option::is_none")]
    pub pns_handle: Option<String>,
    #[doc = "Registration id."]
    #[serde(rename = "registrationId", default, skip_serializing_if = "Option::is_none")]
    pub registration_id: Option<String>,
    #[doc = "Notification outcome."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
}
impl RegistrationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Private Endpoint that is connected to Notification Hubs namespace using Private Endpoint Connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemotePrivateEndpointConnection {
    #[doc = "ARM resource ID of the Private Endpoint. This may belong to different subscription and resource group than a Notification Hubs namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl RemotePrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "State of the Private Link Service connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemotePrivateLinkServiceConnectionState {
    #[doc = "State of Private Link Connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateLinkConnectionStatus>,
    #[doc = "Human-friendly description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Human-friendly description of required actions."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl RemotePrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Allowed replication region"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReplicationRegion")]
pub enum ReplicationRegion {
    Default,
    WestUs2,
    NorthEurope,
    AustraliaEast,
    BrazilSouth,
    SouthEastAsia,
    SouthAfricaNorth,
    None,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReplicationRegion {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReplicationRegion {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReplicationRegion {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Default => serializer.serialize_unit_variant("ReplicationRegion", 0u32, "Default"),
            Self::WestUs2 => serializer.serialize_unit_variant("ReplicationRegion", 1u32, "WestUs2"),
            Self::NorthEurope => serializer.serialize_unit_variant("ReplicationRegion", 2u32, "NorthEurope"),
            Self::AustraliaEast => serializer.serialize_unit_variant("ReplicationRegion", 3u32, "AustraliaEast"),
            Self::BrazilSouth => serializer.serialize_unit_variant("ReplicationRegion", 4u32, "BrazilSouth"),
            Self::SouthEastAsia => serializer.serialize_unit_variant("ReplicationRegion", 5u32, "SouthEastAsia"),
            Self::SouthAfricaNorth => serializer.serialize_unit_variant("ReplicationRegion", 6u32, "SouthAfricaNorth"),
            Self::None => serializer.serialize_unit_variant("ReplicationRegion", 7u32, "None"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. E.g. \"/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for the POST request that returns Namespace or NotificationHub access keys (connection strings)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceListKeys {
    #[doc = "Gets or sets primaryConnectionString of the AuthorizationRule."]
    #[serde(rename = "primaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[doc = "Gets or sets secondaryConnectionString of the created\r\nAuthorizationRule"]
    #[serde(rename = "secondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
    #[doc = "Gets or sets primaryKey of the created AuthorizationRule."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "Gets or sets secondaryKey of the created AuthorizationRule"]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "Gets or sets keyName of the created AuthorizationRule"]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
}
impl ResourceListKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Optional service specification used in Operations API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "Log specifications."]
    #[serde(
        rename = "logSpecifications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub log_specifications: Vec<LogSpecification>,
    #[doc = "Metric specification."]
    #[serde(
        rename = "metricSpecifications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metric_specifications: Vec<MetricSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of the List Namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedAccessAuthorizationRuleListResult {
    #[doc = "Gets or sets result of the List AuthorizationRules operation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SharedAccessAuthorizationRuleResource>,
    #[doc = "Gets or sets link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SharedAccessAuthorizationRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SharedAccessAuthorizationRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SharedAccessAuthorizationRule properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessAuthorizationRuleProperties {
    #[doc = "Gets or sets the rights associated with the rule."]
    pub rights: Vec<AccessRights>,
    #[doc = "Gets a base64-encoded 256-bit primary key for signing and\r\nvalidating the SAS token."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "Gets a base64-encoded 256-bit primary key for signing and\r\nvalidating the SAS token."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "Gets a string that describes the authorization rule."]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[doc = "Gets the last modified time for this rule"]
    #[serde(rename = "modifiedTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the created time for this rule"]
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "Gets a string that describes the claim type"]
    #[serde(rename = "claimType", default, skip_serializing_if = "Option::is_none")]
    pub claim_type: Option<String>,
    #[doc = "Gets a string that describes the claim value"]
    #[serde(rename = "claimValue", default, skip_serializing_if = "Option::is_none")]
    pub claim_value: Option<String>,
    #[doc = "Gets the revision number for the rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}
impl SharedAccessAuthorizationRuleProperties {
    pub fn new(rights: Vec<AccessRights>) -> Self {
        Self {
            rights,
            primary_key: None,
            secondary_key: None,
            key_name: None,
            modified_time: None,
            created_time: None,
            claim_type: None,
            claim_value: None,
            revision: None,
        }
    }
}
#[doc = "Response for POST requests that return single SharedAccessAuthorizationRule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedAccessAuthorizationRuleResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "SharedAccessAuthorizationRule properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedAccessAuthorizationRuleProperties>,
    #[doc = "Deprecated - only for compatibility."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Deprecated - only for compatibility."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SharedAccessAuthorizationRuleResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Sku description for a namespace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "Namespace SKU name."]
    pub name: SkuName,
    #[doc = "Gets or sets the tier of particular sku"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "Gets or sets the Sku size"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "Gets or sets the Sku Family"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "Gets or sets the capacity of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new(name: SkuName) -> Self {
        Self {
            name,
            tier: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
#[doc = "Namespace SKU name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SkuName")]
pub enum SkuName {
    Free,
    Basic,
    Standard,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SkuName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SkuName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SkuName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Free => serializer.serialize_unit_variant("SkuName", 0u32, "Free"),
            Self::Basic => serializer.serialize_unit_variant("SkuName", 1u32, "Basic"),
            Self::Standard => serializer.serialize_unit_variant("SkuName", 2u32, "Standard"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource model definition for an Azure Resource Manager tracked top level resource which has 'tags' and a 'location'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
#[doc = "Description of a NotificationHub WnsCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WnsCredential {
    #[doc = "Description of a NotificationHub WnsCredential."]
    pub properties: WnsCredentialProperties,
}
impl WnsCredential {
    pub fn new(properties: WnsCredentialProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Description of a NotificationHub WnsCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WnsCredentialProperties {
    #[doc = "Gets or sets the package ID for this credential."]
    #[serde(rename = "packageSid", default, skip_serializing_if = "Option::is_none")]
    pub package_sid: Option<String>,
    #[doc = "Gets or sets the secret key."]
    #[serde(rename = "secretKey", default, skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    #[doc = "Gets or sets the Windows Live endpoint."]
    #[serde(rename = "windowsLiveEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub windows_live_endpoint: Option<String>,
    #[doc = "Ges or sets the WNS Certificate Key."]
    #[serde(rename = "certificateKey", default, skip_serializing_if = "Option::is_none")]
    pub certificate_key: Option<String>,
    #[doc = "Gets or sets the WNS Certificate."]
    #[serde(rename = "wnsCertificate", default, skip_serializing_if = "Option::is_none")]
    pub wns_certificate: Option<String>,
}
impl WnsCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub XiaomiCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct XiaomiCredential {
    #[doc = "Description of a NotificationHub XiaomiCredentialProperties."]
    pub properties: XiaomiCredentialProperties,
}
impl XiaomiCredential {
    pub fn new(properties: XiaomiCredentialProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Description of a NotificationHub XiaomiCredentialProperties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct XiaomiCredentialProperties {
    #[doc = "Gets or sets app secret."]
    #[serde(rename = "appSecret", default, skip_serializing_if = "Option::is_none")]
    pub app_secret: Option<String>,
    #[doc = "Gets or sets xiaomi service endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl XiaomiCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Namespace SKU name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ZoneRedundancyPreference")]
pub enum ZoneRedundancyPreference {
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ZoneRedundancyPreference {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ZoneRedundancyPreference {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ZoneRedundancyPreference {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("ZoneRedundancyPreference", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("ZoneRedundancyPreference", 1u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for ZoneRedundancyPreference {
    fn default() -> Self {
        Self::Disabled
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreatedByType")]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreatedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreatedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreatedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("CreatedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("CreatedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("CreatedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("CreatedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastModifiedByType")]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastModifiedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastModifiedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastModifiedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("LastModifiedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("LastModifiedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("LastModifiedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("LastModifiedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
