#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Azure Networking ACL Action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AclAction")]
pub enum AclAction {
    Allow,
    Deny,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AclAction {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AclAction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AclAction {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Allow => serializer.serialize_unit_variant("AclAction", 0u32, "Allow"),
            Self::Deny => serializer.serialize_unit_variant("AclAction", 1u32, "Deny"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A custom certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomCertificate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Custom certificate properties."]
    pub properties: CustomCertificateProperties,
}
impl CustomCertificate {
    pub fn new(properties: CustomCertificateProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "Custom certificates list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomCertificateList {
    #[doc = "List of custom certificates of this resource."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CustomCertificate>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CustomCertificateList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CustomCertificateList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Custom certificate properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomCertificateProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Base uri of the KeyVault that stores certificate."]
    #[serde(rename = "keyVaultBaseUri")]
    pub key_vault_base_uri: String,
    #[doc = "Certificate secret name."]
    #[serde(rename = "keyVaultSecretName")]
    pub key_vault_secret_name: String,
    #[doc = "Certificate secret version."]
    #[serde(rename = "keyVaultSecretVersion", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_secret_version: Option<String>,
}
impl CustomCertificateProperties {
    pub fn new(key_vault_base_uri: String, key_vault_secret_name: String) -> Self {
        Self {
            provisioning_state: None,
            key_vault_base_uri,
            key_vault_secret_name,
            key_vault_secret_version: None,
        }
    }
}
#[doc = "A custom domain"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomain {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a custom domain."]
    pub properties: CustomDomainProperties,
}
impl CustomDomain {
    pub fn new(properties: CustomDomainProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "Custom domains list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomDomainList {
    #[doc = "List of custom domains that bind to this resource."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CustomDomain>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CustomDomainList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CustomDomainList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a custom domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomainProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The custom domain name."]
    #[serde(rename = "domainName")]
    pub domain_name: String,
    #[doc = "Reference to a resource."]
    #[serde(rename = "customCertificate")]
    pub custom_certificate: ResourceReference,
}
impl CustomDomainProperties {
    pub fn new(domain_name: String, custom_certificate: ResourceReference) -> Self {
        Self {
            provisioning_state: None,
            domain_name,
            custom_certificate,
        }
    }
}
#[doc = "Specifications of the Dimension of metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimension {
    #[doc = "The public facing name of the dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the dimension."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Name of the dimension as it appears in MDM."]
    #[serde(rename = "internalName", default, skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[doc = "A Boolean flag indicating whether this dimension should be included for the shoebox export scenario."]
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
impl Dimension {
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
#[doc = "Properties of event handler."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHandler {
    #[doc = "Gets or sets the EventHandler URL template. You can use a predefined parameter {hub} and {event} inside the template, the value of the EventHandler URL is dynamically calculated when the client request comes in.\r\nFor example, UrlTemplate can be `http://example.com/api/{hub}/{event}`. The host part can't contains parameters."]
    #[serde(rename = "urlTemplate")]
    pub url_template: String,
    #[doc = "Gets or sets the matching pattern for event names.\r\nThere are 3 kinds of patterns supported:\r\n    1. \"*\", it matches any event name\r\n    2. Combine multiple events with \",\", for example \"event1,event2\", it matches event \"event1\" and \"event2\"\r\n    3. A single event name, for example, \"event1\", it matches \"event1\""]
    #[serde(rename = "userEventPattern", default, skip_serializing_if = "Option::is_none")]
    pub user_event_pattern: Option<String>,
    #[doc = "Gets or sets the list of system events."]
    #[serde(
        rename = "systemEvents",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub system_events: Vec<String>,
    #[doc = "Upstream auth settings. If not set, no auth is used for upstream messages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<UpstreamAuthSettings>,
}
impl EventHandler {
    pub fn new(url_template: String) -> Self {
        Self {
            url_template,
            user_event_pattern: None,
            system_events: Vec::new(),
            auth: None,
        }
    }
}
#[doc = "An Event Hub endpoint. \r\nThe managed identity of Web PubSub service must be enabled, and the identity should have the \"Azure Event Hubs Data sender\" role to access Event Hub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEndpoint {
    #[doc = "The fully qualified namespace name of the Event Hub resource. For example, \"example.servicebus.windows.net\"."]
    #[serde(rename = "fullyQualifiedNamespace")]
    pub fully_qualified_namespace: String,
    #[doc = "The name of the Event Hub."]
    #[serde(rename = "eventHubName")]
    pub event_hub_name: String,
}
impl EventHubEndpoint {
    pub fn new(fully_qualified_namespace: String, event_hub_name: String) -> Self {
        Self {
            fully_qualified_namespace,
            event_hub_name,
        }
    }
}
#[doc = "A setting defines which kinds of events should be sent to which endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventListener {
    #[doc = "A base class for event filter which determines whether an event should be sent to an event listener."]
    pub filter: EventListenerFilterUnion,
    #[doc = "An endpoint specifying where Web PubSub should send events to."]
    pub endpoint: EventListenerEndpointUnion,
}
impl EventListener {
    pub fn new(filter: EventListenerFilterUnion, endpoint: EventListenerEndpointUnion) -> Self {
        Self { filter, endpoint }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EventListenerEndpointUnion {
    EventHub(EventHubEndpoint),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EventListenerFilterUnion {
    EventName(EventNameFilter),
}
#[doc = "Filter events by their name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventNameFilter {
    #[doc = "Gets or sets a list of system events. Supported events: \"connected\" and \"disconnected\". Blocking event \"connect\" is not supported because it requires a response."]
    #[serde(
        rename = "systemEvents",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub system_events: Vec<String>,
    #[doc = "Gets or sets a matching pattern for event names.\r\nThere are 3 kinds of patterns supported:\r\n    1. \"*\", it matches any event name\r\n    2. Combine multiple events with \",\", for example \"event1,event2\", it matches events \"event1\" and \"event2\"\r\n    3. A single event name, for example, \"event1\", it matches \"event1\""]
    #[serde(rename = "userEventPattern", default, skip_serializing_if = "Option::is_none")]
    pub user_event_pattern: Option<String>,
}
impl EventNameFilter {
    pub fn new() -> Self {
        Self {
            system_events: Vec::new(),
            user_event_pattern: None,
        }
    }
}
#[doc = "The type of access key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "KeyType")]
pub enum KeyType {
    Primary,
    Secondary,
    Salt,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for KeyType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for KeyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for KeyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Primary => serializer.serialize_unit_variant("KeyType", 0u32, "Primary"),
            Self::Secondary => serializer.serialize_unit_variant("KeyType", 1u32, "Secondary"),
            Self::Salt => serializer.serialize_unit_variant("KeyType", 2u32, "Salt"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Live trace category configuration of a Microsoft.SignalRService resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveTraceCategory {
    #[doc = "Gets or sets the live trace category's name.\r\nAvailable values: ConnectivityLogs, MessagingLogs.\r\nCase insensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether or the live trace category is enabled.\r\nAvailable values: true, false.\r\nCase insensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
}
impl LiveTraceCategory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Live trace configuration of a Microsoft.SignalRService resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveTraceConfiguration {
    #[doc = "Indicates whether or not enable live trace.\r\nWhen it's set to true, live trace client can connect to the service.\r\nOtherwise, live trace client can't connect to the service, so that you are unable to receive any log, no matter what you configure in \"categories\".\r\nAvailable values: true, false.\r\nCase insensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
    #[doc = "Gets or sets the list of category configurations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub categories: Vec<LiveTraceCategory>,
}
impl LiveTraceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifications of the Logs for Azure Monitoring."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[doc = "Name of the log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the log."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class represent managed identities used for request and response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentity {
    #[doc = "Represents the identity type: systemAssigned, userAssigned, None"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ManagedIdentityType>,
    #[doc = "Get or set the user assigned identities"]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
    #[doc = "Get the principal id for the system assigned identity.\r\nOnly be used in response."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Get the tenant id for the system assigned identity.\r\nOnly be used in response"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ManagedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed identity settings for upstream."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentitySettings {
    #[doc = "The Resource indicating the App ID URI of the target resource.\r\nIt also appears in the aud (audience) claim of the issued token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl ManagedIdentitySettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the identity type: systemAssigned, userAssigned, None"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedIdentityType")]
pub enum ManagedIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedIdentityType", 1u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedIdentityType", 2u32, "UserAssigned"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifications of the Metrics for Azure Monitoring."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[doc = "Name of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the metric."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Localized friendly description of the metric."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "The unit that makes sense for the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Only provide one value for this field. Valid values: Average, Minimum, Maximum, Total, Count."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "Optional. If set to true, then zero will be returned for time duration where no metric is emitted/published. \r\nEx. a metric that returns the number of times a particular error code was emitted. The error code may not appear \r\noften, instead of the RP publishing 0, Shoebox can auto fill in 0s for time periods where nothing was emitted."]
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<String>,
    #[doc = "The name of the metric category that the metric belongs to. A metric can only belong to a single category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The dimensions of the metrics."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimensions: Vec<Dimension>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to check name availability. It contains a flag and possible reason of failure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailability {
    #[doc = "Indicates whether the name is available or not."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason of the availability. Required if name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The message of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl NameAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data POST-ed to the nameAvailability action"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAvailabilityParameters {
    #[doc = "The resource type. Can be \"Microsoft.SignalRService/SignalR\", \"Microsoft.SignalRService/WebPubSub\", \"Microsoft.SignalRService/SignalR/replicas\" or \"Microsoft.SignalRService/WebPubSub/replicas\""]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The resource name to validate. e.g.\"my-resource-name\""]
    pub name: String,
}
impl NameAvailabilityParameters {
    pub fn new(type_: String, name: String) -> Self {
        Self { type_, name }
    }
}
#[doc = "Network ACL"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkAcl {
    #[doc = "Allowed request types. The value can be one or more of: ClientConnection, ServerConnection, RESTAPI."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allow: Vec<WebPubSubRequestType>,
    #[doc = "Denied request types. The value can be one or more of: ClientConnection, ServerConnection, RESTAPI."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub deny: Vec<WebPubSubRequestType>,
}
impl NetworkAcl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "REST API operation supported by resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation with format: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "If the operation is a data action. (for data plane rbac)"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "The object that describes a operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Optional. The intended executor of the operation; governs the display of the operation in the RBAC UX and the audit logs UX."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Extra Operation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object that describes a operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Friendly name of the resource provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource type on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The localized friendly name for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The localized friendly description for the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list REST API operations. It contains a list of operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "List of operations supported by the resource provider."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Extra Operation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "An object that describes a specification."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "Full qualified Id of the private endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ACL for a private endpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointAcl {
    #[serde(flatten)]
    pub network_acl: NetworkAcl,
    #[doc = "Name of the private endpoint connection"]
    pub name: String,
}
impl PrivateEndpointAcl {
    pub fn new(name: String) -> Self {
        Self {
            network_acl: NetworkAcl::default(),
            name,
        }
    }
}
#[doc = "A private endpoint connection to an azure resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Private endpoint connection properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private endpoint connections"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionList {
    #[doc = "The list of the private endpoint connections"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "Request URL that can be used to query next page of private endpoint connections. Returned when the total number of requested private endpoint connections exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PrivateEndpointConnectionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint connection properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Private endpoint"]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "Group IDs"]
    #[serde(
        rename = "groupIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_ids: Vec<String>,
    #[doc = "Connection state of the private endpoint connection"]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Private link resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains a list of PrivateLinkResource and a possible link to query more results"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceList {
    #[doc = "List of PrivateLinkResource"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateLinkResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PrivateLinkResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private link resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "Group Id of the private link resource"]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "Required members of the private link resource"]
    #[serde(
        rename = "requiredMembers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_members: Vec<String>,
    #[doc = "Required private DNS zone names"]
    #[serde(
        rename = "requiredZoneNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_zone_names: Vec<String>,
    #[doc = "The list of resources that are onboarded to private link service"]
    #[serde(
        rename = "shareablePrivateLinkResourceTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub shareable_private_link_resource_types: Vec<ShareablePrivateLinkResourceType>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Connection state of the private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "Indicates whether the connection has been Approved/Rejected/Removed by the owner of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateLinkServiceConnectionStatus>,
    #[doc = "The reason for approval/rejection of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates whether the connection has been Approved/Rejected/Removed by the owner of the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateLinkServiceConnectionStatus")]
pub enum PrivateLinkServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    Disconnected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateLinkServiceConnectionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateLinkServiceConnectionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateLinkServiceConnectionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("PrivateLinkServiceConnectionStatus", 0u32, "Pending"),
            Self::Approved => serializer.serialize_unit_variant("PrivateLinkServiceConnectionStatus", 1u32, "Approved"),
            Self::Rejected => serializer.serialize_unit_variant("PrivateLinkServiceConnectionStatus", 2u32, "Rejected"),
            Self::Disconnected => serializer.serialize_unit_variant("PrivateLinkServiceConnectionStatus", 3u32, "Disconnected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Provisioning state of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Unknown,
    Succeeded,
    Failed,
    Canceled,
    Running,
    Creating,
    Updating,
    Deleting,
    Moving,
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
            Self::Unknown => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Unknown"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
            Self::Running => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Running"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleting"),
            Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Moving"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "Parameters describes the request to regenerate access keys"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegenerateKeyParameters {
    #[doc = "The type of access key."]
    #[serde(rename = "keyType", default, skip_serializing_if = "Option::is_none")]
    pub key_type: Option<KeyType>,
}
impl RegenerateKeyParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class represent a replica resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Replica {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The billing information of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReplicaProperties>,
}
impl Replica {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            sku: None,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicaList {
    #[doc = "List of the replica"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Replica>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReplicaList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ReplicaList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicaProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ReplicaProperties {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Resource log category configuration of a Microsoft.SignalRService resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceLogCategory {
    #[doc = "Gets or sets the resource log category's name.\r\nAvailable values: ConnectivityLogs, MessagingLogs.\r\nCase insensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether or the resource log category is enabled.\r\nAvailable values: true, false.\r\nCase insensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
}
impl ResourceLogCategory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource log configuration of a Microsoft.SignalRService resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceLogConfiguration {
    #[doc = "Gets or sets the list of category configurations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub categories: Vec<ResourceLogCategory>,
}
impl ResourceLogConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceReference {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ResourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The billing information of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSku {
    #[doc = "The name of the SKU. Required.\r\n\r\nAllowed values: Standard_S1, Free_F1, Premium_P1"]
    pub name: String,
    #[doc = "Optional tier of this particular SKU. 'Standard' or 'Free'. \r\n\r\n`Basic` is deprecated, use `Standard` instead."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<WebPubSubSkuTier>,
    #[doc = "Not used. Retained for future use."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "Not used. Retained for future use."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "Optional, integer. The unit count of the resource. 1 by default.\r\n\r\nIf present, following values are allowed:\r\n    Free: 1;\r\n    Standard: 1,2,3,4,5,6,7,8,9,10,20,30,40,50,60,70,80,90,100;\r\n    Premium:  1,2,3,4,5,6,7,8,9,10,20,30,40,50,60,70,80,90,100;"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl ResourceSku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
#[doc = "The scale type applicable to the sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScaleType")]
pub enum ScaleType {
    None,
    Manual,
    Automatic,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScaleType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScaleType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScaleType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ScaleType", 0u32, "None"),
            Self::Manual => serializer.serialize_unit_variant("ScaleType", 1u32, "Manual"),
            Self::Automatic => serializer.serialize_unit_variant("ScaleType", 2u32, "Automatic"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The kind of the service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceKind")]
pub enum ServiceKind {
    WebPubSub,
    #[serde(rename = "SocketIO")]
    SocketIo,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::WebPubSub => serializer.serialize_unit_variant("ServiceKind", 0u32, "WebPubSub"),
            Self::SocketIo => serializer.serialize_unit_variant("ServiceKind", 1u32, "SocketIO"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An object that describes a specification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "Specifications of the Metrics for Azure Monitoring."]
    #[serde(
        rename = "metricSpecifications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metric_specifications: Vec<MetricSpecification>,
    #[doc = "Specifications of the Logs for Azure Monitoring."]
    #[serde(
        rename = "logSpecifications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub log_specifications: Vec<LogSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a resource type that has been onboarded to private link service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShareablePrivateLinkResourceProperties {
    #[doc = "The description of the resource type that has been onboarded to private link service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The resource provider group id for the resource that has been onboarded to private link service"]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The resource provider type for the resource that has been onboarded to private link service"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ShareablePrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a  resource type that has been onboarded to private link service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShareablePrivateLinkResourceType {
    #[doc = "The name of the resource type that has been onboarded to private link service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Describes the properties of a resource type that has been onboarded to private link service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ShareablePrivateLinkResourceProperties>,
}
impl ShareablePrivateLinkResourceType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Shared Private Link Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedPrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Describes the properties of an existing Shared Private Link Resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedPrivateLinkResourceProperties>,
}
impl SharedPrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of shared private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedPrivateLinkResourceList {
    #[doc = "The list of the shared private link resources"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SharedPrivateLinkResource>,
    #[doc = "Request URL that can be used to query next page of private endpoint connections. Returned when the total number of requested private endpoint connections exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SharedPrivateLinkResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SharedPrivateLinkResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of an existing Shared Private Link Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedPrivateLinkResourceProperties {
    #[doc = "The group id from the provider of resource the shared private link resource is for"]
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[doc = "The resource id of the resource the shared private link resource is for"]
    #[serde(rename = "privateLinkResourceId")]
    pub private_link_resource_id: String,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The request message for requesting approval of the shared private link resource"]
    #[serde(rename = "requestMessage", default, skip_serializing_if = "Option::is_none")]
    pub request_message: Option<String>,
    #[doc = "Status of the shared private link resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SharedPrivateLinkResourceStatus>,
}
impl SharedPrivateLinkResourceProperties {
    pub fn new(group_id: String, private_link_resource_id: String) -> Self {
        Self {
            group_id,
            private_link_resource_id,
            provisioning_state: None,
            request_message: None,
            status: None,
        }
    }
}
#[doc = "Status of the shared private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SharedPrivateLinkResourceStatus")]
pub enum SharedPrivateLinkResourceStatus {
    Pending,
    Approved,
    Rejected,
    Disconnected,
    Timeout,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SharedPrivateLinkResourceStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SharedPrivateLinkResourceStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SharedPrivateLinkResourceStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("SharedPrivateLinkResourceStatus", 0u32, "Pending"),
            Self::Approved => serializer.serialize_unit_variant("SharedPrivateLinkResourceStatus", 1u32, "Approved"),
            Self::Rejected => serializer.serialize_unit_variant("SharedPrivateLinkResourceStatus", 2u32, "Rejected"),
            Self::Disconnected => serializer.serialize_unit_variant("SharedPrivateLinkResourceStatus", 3u32, "Disconnected"),
            Self::Timeout => serializer.serialize_unit_variant("SharedPrivateLinkResourceStatus", 4u32, "Timeout"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Object that describes a specific usage of the resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRServiceUsage {
    #[doc = "Fully qualified ARM resource id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Current value for the usage quota."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[doc = "The maximum permitted value for the usage quota. If there is no limit, this value will be -1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "Localizable String object containing the name and a localized value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<SignalRServiceUsageName>,
    #[doc = "Representing the units of the usage quota. Possible values are: Count, Bytes, Seconds, Percent, CountPerSecond, BytesPerSecond."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl SignalRServiceUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of the resource usages and a possible link for next set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRServiceUsageList {
    #[doc = "List of the resource usages"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SignalRServiceUsage>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SignalRServiceUsageList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SignalRServiceUsageList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Localizable String object containing the name and a localized value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRServiceUsageName {
    #[doc = "The identifier of the usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Localized name of the usage."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl SignalRServiceUsageName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an available sku.\""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[doc = "The resource type that this object applies to"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The billing information of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[doc = "Describes scaling information of a sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<SkuCapacity>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes scaling information of a sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCapacity {
    #[doc = "The lowest permitted capacity for this resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,
    #[doc = "The highest permitted capacity for this resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
    #[doc = "The default capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i32>,
    #[doc = "Allows capacity value list."]
    #[serde(
        rename = "allowedValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_values: Vec<i32>,
    #[doc = "The scale type applicable to the sku."]
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<ScaleType>,
}
impl SkuCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list skus operation response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuList {
    #[doc = "The list of skus available for the resource."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Sku>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SkuList {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Upstream auth settings. If not set, no auth is used for upstream messages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpstreamAuthSettings {
    #[doc = "Upstream auth type enum."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<UpstreamAuthType>,
    #[doc = "Managed identity settings for upstream."]
    #[serde(rename = "managedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub managed_identity: Option<ManagedIdentitySettings>,
}
impl UpstreamAuthSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Upstream auth type enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UpstreamAuthType")]
pub enum UpstreamAuthType {
    None,
    ManagedIdentity,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UpstreamAuthType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UpstreamAuthType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UpstreamAuthType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("UpstreamAuthType", 0u32, "None"),
            Self::ManagedIdentity => serializer.serialize_unit_variant("UpstreamAuthType", 1u32, "ManagedIdentity"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Properties of user assigned identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentityProperty {
    #[doc = "Get the principal id for the user assigned identity"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Get the client id for the user assigned identity"]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentityProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A hub setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebPubSubHub {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a hub."]
    pub properties: WebPubSubHubProperties,
}
impl WebPubSubHub {
    pub fn new(properties: WebPubSubHubProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "Hub setting list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebPubSubHubList {
    #[doc = "List of hub settings to this resource."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WebPubSubHub>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WebPubSubHubList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WebPubSubHubList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a hub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebPubSubHubProperties {
    #[doc = "Event handler of a hub."]
    #[serde(
        rename = "eventHandlers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub event_handlers: Vec<EventHandler>,
    #[doc = "Event listener settings for forwarding your client events to listeners.\r\nEvent listener is transparent to Web PubSub clients, and it doesn't return any result to clients nor interrupt the lifetime of clients.\r\nOne event can be sent to multiple listeners, as long as it matches the filters in those listeners. The order of the array elements doesn't matter.\r\nMaximum count of event listeners among all hubs is 10."]
    #[serde(
        rename = "eventListeners",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub event_listeners: Vec<EventListener>,
    #[doc = "The settings for configuring if anonymous connections are allowed for this hub: \"allow\" or \"deny\". Default to \"deny\"."]
    #[serde(rename = "anonymousConnectPolicy", default, skip_serializing_if = "Option::is_none")]
    pub anonymous_connect_policy: Option<String>,
}
impl WebPubSubHubProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class represents the access keys of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebPubSubKeys {
    #[doc = "The primary access key."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "The secondary access key."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "Connection string constructed via the primaryKey"]
    #[serde(rename = "primaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[doc = "Connection string constructed via the secondaryKey"]
    #[serde(rename = "secondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
}
impl WebPubSubKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network ACLs for the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebPubSubNetworkAcLs {
    #[doc = "Azure Networking ACL Action."]
    #[serde(rename = "defaultAction", default, skip_serializing_if = "Option::is_none")]
    pub default_action: Option<AclAction>,
    #[doc = "Network ACL"]
    #[serde(rename = "publicNetwork", default, skip_serializing_if = "Option::is_none")]
    pub public_network: Option<NetworkAcl>,
    #[doc = "ACLs for requests from private endpoints"]
    #[serde(
        rename = "privateEndpoints",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoints: Vec<PrivateEndpointAcl>,
}
impl WebPubSubNetworkAcLs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class that describes the properties of the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebPubSubProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The publicly accessible IP of the resource."]
    #[serde(rename = "externalIP", default, skip_serializing_if = "Option::is_none")]
    pub external_ip: Option<String>,
    #[doc = "FQDN of the service instance."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "The publicly accessible port of the resource which is designed for browser/client side usage."]
    #[serde(rename = "publicPort", default, skip_serializing_if = "Option::is_none")]
    pub public_port: Option<i32>,
    #[doc = "The publicly accessible port of the resource which is designed for customer server side usage."]
    #[serde(rename = "serverPort", default, skip_serializing_if = "Option::is_none")]
    pub server_port: Option<i32>,
    #[doc = "Version of the resource. Probably you need the same or higher version of client SDKs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Private endpoint connections to the resource."]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "The list of shared private link resources."]
    #[serde(
        rename = "sharedPrivateLinkResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub shared_private_link_resources: Vec<SharedPrivateLinkResource>,
    #[doc = "TLS settings for the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<WebPubSubTlsSettings>,
    #[doc = "Deprecated."]
    #[serde(rename = "hostNamePrefix", default, skip_serializing_if = "Option::is_none")]
    pub host_name_prefix: Option<String>,
    #[doc = "Live trace configuration of a Microsoft.SignalRService resource."]
    #[serde(rename = "liveTraceConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub live_trace_configuration: Option<LiveTraceConfiguration>,
    #[doc = "Resource log configuration of a Microsoft.SignalRService resource."]
    #[serde(rename = "resourceLogConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub resource_log_configuration: Option<ResourceLogConfiguration>,
    #[doc = "Network ACLs for the resource"]
    #[serde(rename = "networkACLs", default, skip_serializing_if = "Option::is_none")]
    pub network_ac_ls: Option<WebPubSubNetworkAcLs>,
    #[doc = "Enable or disable public network access. Default to \"Enabled\".\r\nWhen it's Enabled, network ACLs still apply.\r\nWhen it's Disabled, public network access is always disabled no matter what you set in network ACLs."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<String>,
    #[doc = "DisableLocalAuth\r\nEnable or disable local auth with AccessKey\r\nWhen set as true, connection with AccessKey=xxx won't work."]
    #[serde(rename = "disableLocalAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_local_auth: Option<bool>,
    #[doc = "DisableLocalAuth\r\nEnable or disable aad auth\r\nWhen set as true, connection with AuthType=aad won't work."]
    #[serde(rename = "disableAadAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_aad_auth: Option<bool>,
}
impl WebPubSubProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The incoming request type to the service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WebPubSubRequestType")]
pub enum WebPubSubRequestType {
    ClientConnection,
    ServerConnection,
    #[serde(rename = "RESTAPI")]
    Restapi,
    Trace,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WebPubSubRequestType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WebPubSubRequestType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WebPubSubRequestType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ClientConnection => serializer.serialize_unit_variant("WebPubSubRequestType", 0u32, "ClientConnection"),
            Self::ServerConnection => serializer.serialize_unit_variant("WebPubSubRequestType", 1u32, "ServerConnection"),
            Self::Restapi => serializer.serialize_unit_variant("WebPubSubRequestType", 2u32, "RESTAPI"),
            Self::Trace => serializer.serialize_unit_variant("WebPubSubRequestType", 3u32, "Trace"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A class represent a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebPubSubResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The billing information of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[doc = "A class that describes the properties of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebPubSubProperties>,
    #[doc = "The kind of the service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ServiceKind>,
    #[doc = "A class represent managed identities used for request and response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedIdentity>,
}
impl WebPubSubResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            sku: None,
            properties: None,
            kind: None,
            identity: None,
        }
    }
}
#[doc = "Object that includes an array of resources and a possible link for next set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebPubSubResourceList {
    #[doc = "List of the resources"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WebPubSubResource>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WebPubSubResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WebPubSubResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Optional tier of this particular SKU. 'Standard' or 'Free'. \r\n\r\n`Basic` is deprecated, use `Standard` instead."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WebPubSubSkuTier")]
pub enum WebPubSubSkuTier {
    Free,
    Basic,
    Standard,
    Premium,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WebPubSubSkuTier {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WebPubSubSkuTier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WebPubSubSkuTier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Free => serializer.serialize_unit_variant("WebPubSubSkuTier", 0u32, "Free"),
            Self::Basic => serializer.serialize_unit_variant("WebPubSubSkuTier", 1u32, "Basic"),
            Self::Standard => serializer.serialize_unit_variant("WebPubSubSkuTier", 2u32, "Standard"),
            Self::Premium => serializer.serialize_unit_variant("WebPubSubSkuTier", 3u32, "Premium"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "TLS settings for the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebPubSubTlsSettings {
    #[doc = "Request client certificate during TLS handshake if enabled. Not supported for free tier. Any input will be ignored for free tier."]
    #[serde(rename = "clientCertEnabled", default, skip_serializing_if = "Option::is_none")]
    pub client_cert_enabled: Option<bool>,
}
impl WebPubSubTlsSettings {
    pub fn new() -> Self {
        Self::default()
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
