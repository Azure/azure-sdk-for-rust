#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An identity that have access to the key vault. All identities in the array must use the same tenant ID as the key vault's tenant ID."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyEntry {
    #[doc = "The Azure Active Directory tenant ID that should be used for authenticating requests to the key vault."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[doc = "The object ID of a user, service principal or security group in the Azure Active Directory tenant for the vault. The object ID must be unique for the list of access policies."]
    #[serde(rename = "objectId")]
    pub object_id: String,
    #[doc = " Application ID of the client making request on behalf of a principal"]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[doc = "Permissions the identity has for keys, secrets, certificates and storage."]
    pub permissions: Permissions,
}
impl AccessPolicyEntry {
    pub fn new(tenant_id: String, object_id: String, permissions: Permissions) -> Self {
        Self {
            tenant_id,
            object_id,
            application_id: None,
            permissions,
        }
    }
}
#[doc = "The object attributes managed by the KeyVault service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Attributes {
    #[doc = "Determines whether the object is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Not before date in seconds since 1970-01-01T00:00:00Z."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nbf: Option<i64>,
    #[doc = "Expiry date in seconds since 1970-01-01T00:00:00Z."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    #[doc = "Creation time in seconds since 1970-01-01T00:00:00Z."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[doc = "Last updated time in seconds since 1970-01-01T00:00:00Z."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,
}
impl Attributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The CheckNameAvailability operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "A boolean value that indicates whether the name is available for you to use. If true, the name is available. If false, the name has already been taken or is invalid and cannot be used."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason that a vault name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_result::Reason>,
    #[doc = "An error message explaining the Reason value in more detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_result {
    use super::*;
    #[doc = "The reason that a vault name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        AccountNameInvalid,
        AlreadyExists,
    }
}
#[doc = "An error response from Key Vault resource provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from Key Vault resource provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl azure_core::Continuable for CloudError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from Key Vault resource provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "Error code. This is a mnemonic that can be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "User friendly error message. The message is typically localized and may vary with service version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deleted vault information with extended details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedVault {
    #[doc = "The resource ID for the deleted key vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the key vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type of the key vault."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of the deleted vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeletedVaultProperties>,
}
impl DeletedVault {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of vaults"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedVaultListResult {
    #[doc = "The list of deleted vaults."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeletedVault>,
    #[doc = "The URL to get the next set of deleted vaults."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeletedVaultListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeletedVaultListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the deleted vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedVaultProperties {
    #[doc = "The resource id of the original vault."]
    #[serde(rename = "vaultId", default, skip_serializing_if = "Option::is_none")]
    pub vault_id: Option<String>,
    #[doc = "The location of the original vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The deleted date."]
    #[serde(rename = "deletionDate", default, with = "azure_core::date::rfc3339::option")]
    pub deletion_date: Option<time::OffsetDateTime>,
    #[doc = "The scheduled purged date."]
    #[serde(rename = "scheduledPurgeDate", default, with = "azure_core::date::rfc3339::option")]
    pub scheduled_purge_date: Option<time::OffsetDateTime>,
    #[doc = "Tags of the original vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Purge protection status of the original vault."]
    #[serde(rename = "purgeProtectionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub purge_protection_enabled: Option<bool>,
}
impl DeletedVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of operation: get, read, delete, etc."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DimensionProperties {
    #[doc = "Name of dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of dimension."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Property to specify whether the dimension should be exported for shoebox."]
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
impl DimensionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A rule governing the accessibility of a vault from a specific ip address or ip range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpRule {
    #[doc = "An IPv4 address range in CIDR notation, such as '124.56.78.91' (simple IP address) or '124.56.78.0/24' (all addresses that start with 124.56.78)."]
    pub value: String,
}
impl IpRule {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
#[doc = "Log specification of operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[doc = "Name of log specification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of log specification."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Blob duration of specification."]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metric specification of operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[doc = "Name of metric specification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of Metric specification."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Display description of Metric specification."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "The metric unit. Possible values include: 'Bytes', 'Count', 'Milliseconds'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The metric aggregation type. Possible values include: 'Average', 'Count', 'Total'."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "The supported aggregation types for the metrics."]
    #[serde(rename = "supportedAggregationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<String>,
    #[doc = "The supported time grain types for the metrics."]
    #[serde(rename = "supportedTimeGrainTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_time_grain_types: Vec<String>,
    #[doc = "The metric lock aggregation type."]
    #[serde(rename = "lockAggregationType", default, skip_serializing_if = "Option::is_none")]
    pub lock_aggregation_type: Option<String>,
    #[doc = "The dimensions of metric"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<DimensionProperties>,
    #[doc = "Property to specify whether to fill gap with zero."]
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[doc = "The internal metric name."]
    #[serde(rename = "internalMetricName", default, skip_serializing_if = "Option::is_none")]
    pub internal_metric_name: Option<String>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A set of rules governing the network accessibility of a vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRuleSet {
    #[doc = "Tells what traffic can bypass network rules. This can be 'AzureServices' or 'None'.  If not specified the default is 'AzureServices'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bypass: Option<network_rule_set::Bypass>,
    #[doc = "The default action when no rule from ipRules and from virtualNetworkRules match. This is only used after the bypass property has been evaluated."]
    #[serde(rename = "defaultAction", default, skip_serializing_if = "Option::is_none")]
    pub default_action: Option<network_rule_set::DefaultAction>,
    #[doc = "The list of IP address rules."]
    #[serde(rename = "ipRules", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_rules: Vec<IpRule>,
    #[doc = "The list of virtual network rules."]
    #[serde(rename = "virtualNetworkRules", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
}
impl NetworkRuleSet {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_rule_set {
    use super::*;
    #[doc = "Tells what traffic can bypass network rules. This can be 'AzureServices' or 'None'.  If not specified the default is 'AzureServices'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Bypass")]
    pub enum Bypass {
        AzureServices,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Bypass {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Bypass {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Bypass {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureServices => serializer.serialize_unit_variant("Bypass", 0u32, "AzureServices"),
                Self::None => serializer.serialize_unit_variant("Bypass", 1u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The default action when no rule from ipRules and from virtualNetworkRules match. This is only used after the bypass property has been evaluated."]
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
#[doc = "Key Vault REST API operation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The origin of operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Properties of operation, include metric specifications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
    #[doc = "Property to specify whether the action is a data action."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft Key Vault."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of operation: get, read, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Storage operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Storage operations supported by the Storage resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The URL to get the next set of operations."]
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
#[doc = "Properties of operation, include metric specifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "One property of operation, include log specifications."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Permissions the identity has for keys, secrets, certificates and storage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Permissions {
    #[doc = "Permissions to keys"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<String>,
    #[doc = "Permissions to secrets"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<String>,
    #[doc = "Permissions to certificates"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certificates: Vec<String>,
    #[doc = "Permissions to storage accounts"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub storage: Vec<String>,
}
impl Permissions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint object properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "Full identifier of the private endpoint resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the private endpoint connection resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
    #[doc = "Modified whenever there is a change in the state of private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint connection item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionItem {
    #[doc = "Id of private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Modified whenever there is a change in the state of private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Properties of the private endpoint connection resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnectionItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the private endpoint connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "Private endpoint object properties."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "An object that represents the approval state of the private link connection."]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointConnectionProvisioningState")]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Updating,
    Deleting,
    Failed,
    Disconnected,
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
            Self::Succeeded => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 0u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 1u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 2u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 3u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 4u32, "Failed"),
            Self::Disconnected => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 5u32, "Disconnected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The private endpoint connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointServiceConnectionStatus")]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    Disconnected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointServiceConnectionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointServiceConnectionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointServiceConnectionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 0u32, "Pending"),
            Self::Approved => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 1u32, "Approved"),
            Self::Rejected => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 2u32, "Rejected"),
            Self::Disconnected => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 3u32, "Disconnected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[doc = "Array of private link resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
}
impl PrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "Group identifier of private link resource."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "Required member names of private link resource."]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "Required DNS zone names of the the private link resource."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents the approval state of the private link connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[doc = "The reason for approval or rejection."]
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
#[doc = "Key Vault resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified identifier of the key vault resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the key vault resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type of the key vault resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Azure location of the key vault resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Tags assigned to the key vault resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of vault resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceListResult {
    #[doc = "The list of vault resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Resource>,
    #[doc = "The URL to get the next set of vault resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource information with extended details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Secret {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the secret"]
    pub properties: SecretProperties,
}
impl Secret {
    pub fn new(properties: SecretProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "The secret management attributes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretAttributes {
    #[serde(flatten)]
    pub attributes: Attributes,
}
impl SecretAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for creating or updating a secret"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretCreateOrUpdateParameters {
    #[doc = "The tags that will be assigned to the secret. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of the secret"]
    pub properties: SecretProperties,
}
impl SecretCreateOrUpdateParameters {
    pub fn new(properties: SecretProperties) -> Self {
        Self { tags: None, properties }
    }
}
#[doc = "List of secrets"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretListResult {
    #[doc = "The list of secrets."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Secret>,
    #[doc = "The URL to get the next set of secrets."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecretListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SecretListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for patching a secret"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretPatchParameters {
    #[doc = "The tags that will be assigned to the secret. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of the secret"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecretPatchProperties>,
}
impl SecretPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the secret"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretPatchProperties {
    #[doc = "The value of the secret."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The content type of the secret."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "The secret management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SecretAttributes>,
}
impl SecretPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the secret"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretProperties {
    #[doc = "The value of the secret. NOTE: 'value' will never be returned from the service, as APIs using this model are is intended for internal use in ARM deployments. Users should use the data-plane REST service for interaction with vault secrets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The content type of the secret."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "The secret management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SecretAttributes>,
    #[doc = "The URI to retrieve the current version of the secret."]
    #[serde(rename = "secretUri", default, skip_serializing_if = "Option::is_none")]
    pub secret_uri: Option<String>,
    #[doc = "The URI to retrieve the specific version of the secret."]
    #[serde(rename = "secretUriWithVersion", default, skip_serializing_if = "Option::is_none")]
    pub secret_uri_with_version: Option<String>,
}
impl SecretProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "One property of operation, include log specifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "Log specifications of operation."]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
    #[doc = "Metric specifications of operation."]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SKU details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "SKU family name"]
    pub family: sku::Family,
    #[doc = "SKU name to specify whether the key vault is a standard vault or a premium vault."]
    pub name: sku::Name,
}
impl Sku {
    pub fn new(family: sku::Family, name: sku::Name) -> Self {
        Self { family, name }
    }
}
pub mod sku {
    use super::*;
    #[doc = "SKU family name"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Family")]
    pub enum Family {
        A,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Family {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Family {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Family {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::A => serializer.serialize_unit_variant("Family", 0u32, "A"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "SKU name to specify whether the key vault is a standard vault or a premium vault."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "standard")]
        Standard,
        #[serde(rename = "premium")]
        Premium,
    }
}
#[doc = "Resource information with extended details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vault {
    #[doc = "Fully qualified identifier of the key vault resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the key vault resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type of the key vault resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Azure location of the key vault resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Tags assigned to the key vault resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of the vault"]
    pub properties: VaultProperties,
}
impl Vault {
    pub fn new(properties: VaultProperties) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location: None,
            tags: None,
            properties,
        }
    }
}
#[doc = "Parameters for updating the access policy in a vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultAccessPolicyParameters {
    #[doc = "The resource id of the access policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name of the access policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource name of the access policy."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource type of the access policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties of the vault access policy"]
    pub properties: VaultAccessPolicyProperties,
}
impl VaultAccessPolicyParameters {
    pub fn new(properties: VaultAccessPolicyProperties) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location: None,
            properties,
        }
    }
}
#[doc = "Properties of the vault access policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultAccessPolicyProperties {
    #[doc = "An array of 0 to 16 identities that have access to the key vault. All identities in the array must use the same tenant ID as the key vault's tenant ID."]
    #[serde(rename = "accessPolicies")]
    pub access_policies: Vec<AccessPolicyEntry>,
}
impl VaultAccessPolicyProperties {
    pub fn new(access_policies: Vec<AccessPolicyEntry>) -> Self {
        Self { access_policies }
    }
}
#[doc = "The parameters used to check the availability of the vault name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultCheckNameAvailabilityParameters {
    #[doc = "The vault name."]
    pub name: String,
    #[doc = "The type of resource, Microsoft.KeyVault/vaults"]
    #[serde(rename = "type")]
    pub type_: vault_check_name_availability_parameters::Type,
}
impl VaultCheckNameAvailabilityParameters {
    pub fn new(name: String, type_: vault_check_name_availability_parameters::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod vault_check_name_availability_parameters {
    use super::*;
    #[doc = "The type of resource, Microsoft.KeyVault/vaults"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.KeyVault/vaults")]
        MicrosoftKeyVaultVaults,
    }
}
#[doc = "Parameters for creating or updating a vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultCreateOrUpdateParameters {
    #[doc = "The supported Azure location where the key vault should be created."]
    pub location: String,
    #[doc = "The tags that will be assigned to the key vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of the vault"]
    pub properties: VaultProperties,
}
impl VaultCreateOrUpdateParameters {
    pub fn new(location: String, properties: VaultProperties) -> Self {
        Self {
            location,
            tags: None,
            properties,
        }
    }
}
#[doc = "List of vaults"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultListResult {
    #[doc = "The list of vaults."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Vault>,
    #[doc = "The URL to get the next set of vaults."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VaultListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VaultListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for creating or updating a vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultPatchParameters {
    #[doc = "The tags that will be assigned to the key vault. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of the vault"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VaultPatchProperties>,
}
impl VaultPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultPatchProperties {
    #[doc = "The Azure Active Directory tenant ID that should be used for authenticating requests to the key vault."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "SKU details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "An array of 0 to 16 identities that have access to the key vault. All identities in the array must use the same tenant ID as the key vault's tenant ID."]
    #[serde(rename = "accessPolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub access_policies: Vec<AccessPolicyEntry>,
    #[doc = "Property to specify whether Azure Virtual Machines are permitted to retrieve certificates stored as secrets from the key vault."]
    #[serde(rename = "enabledForDeployment", default, skip_serializing_if = "Option::is_none")]
    pub enabled_for_deployment: Option<bool>,
    #[doc = "Property to specify whether Azure Disk Encryption is permitted to retrieve secrets from the vault and unwrap keys."]
    #[serde(rename = "enabledForDiskEncryption", default, skip_serializing_if = "Option::is_none")]
    pub enabled_for_disk_encryption: Option<bool>,
    #[doc = "Property to specify whether Azure Resource Manager is permitted to retrieve secrets from the key vault."]
    #[serde(rename = "enabledForTemplateDeployment", default, skip_serializing_if = "Option::is_none")]
    pub enabled_for_template_deployment: Option<bool>,
    #[doc = "Property to specify whether the 'soft delete' functionality is enabled for this key vault. Once set to true, it cannot be reverted to false."]
    #[serde(rename = "enableSoftDelete", default, skip_serializing_if = "Option::is_none")]
    pub enable_soft_delete: Option<bool>,
    #[doc = "Property that controls how data actions are authorized. When true, the key vault will use Role Based Access Control (RBAC) for authorization of data actions, and the access policies specified in vault properties will be  ignored. When false, the key vault will use the access policies specified in vault properties, and any policy stored on Azure Resource Manager will be ignored. If null or not specified, the value of this property will not change."]
    #[serde(rename = "enableRbacAuthorization", default, skip_serializing_if = "Option::is_none")]
    pub enable_rbac_authorization: Option<bool>,
    #[doc = "softDelete data retention days. It accepts >=7 and <=90."]
    #[serde(rename = "softDeleteRetentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub soft_delete_retention_in_days: Option<i32>,
    #[doc = "The vault's create mode to indicate whether the vault need to be recovered or not."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<vault_patch_properties::CreateMode>,
    #[doc = "Property specifying whether protection against purge is enabled for this vault. Setting this property to true activates protection against purge for this vault and its content - only the Key Vault service may initiate a hard, irrecoverable deletion. The setting is effective only if soft delete is also enabled. Enabling this functionality is irreversible - that is, the property does not accept false as its value."]
    #[serde(rename = "enablePurgeProtection", default, skip_serializing_if = "Option::is_none")]
    pub enable_purge_protection: Option<bool>,
    #[doc = "A set of rules governing the network accessibility of a vault."]
    #[serde(rename = "networkAcls", default, skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,
}
impl VaultPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vault_patch_properties {
    use super::*;
    #[doc = "The vault's create mode to indicate whether the vault need to be recovered or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateMode {
        #[serde(rename = "recover")]
        Recover,
        #[serde(rename = "default")]
        Default,
    }
}
#[doc = "Properties of the vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultProperties {
    #[doc = "The Azure Active Directory tenant ID that should be used for authenticating requests to the key vault."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[doc = "SKU details"]
    pub sku: Sku,
    #[doc = "An array of 0 to 1024 identities that have access to the key vault. All identities in the array must use the same tenant ID as the key vault's tenant ID. When `createMode` is set to `recover`, access policies are not required. Otherwise, access policies are required."]
    #[serde(rename = "accessPolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub access_policies: Vec<AccessPolicyEntry>,
    #[doc = "The URI of the vault for performing operations on keys and secrets. This property is readonly"]
    #[serde(rename = "vaultUri", default, skip_serializing_if = "Option::is_none")]
    pub vault_uri: Option<String>,
    #[doc = "The resource id of HSM Pool."]
    #[serde(rename = "hsmPoolResourceId", default, skip_serializing_if = "Option::is_none")]
    pub hsm_pool_resource_id: Option<String>,
    #[doc = "Property to specify whether Azure Virtual Machines are permitted to retrieve certificates stored as secrets from the key vault."]
    #[serde(rename = "enabledForDeployment", default, skip_serializing_if = "Option::is_none")]
    pub enabled_for_deployment: Option<bool>,
    #[doc = "Property to specify whether Azure Disk Encryption is permitted to retrieve secrets from the vault and unwrap keys."]
    #[serde(rename = "enabledForDiskEncryption", default, skip_serializing_if = "Option::is_none")]
    pub enabled_for_disk_encryption: Option<bool>,
    #[doc = "Property to specify whether Azure Resource Manager is permitted to retrieve secrets from the key vault."]
    #[serde(rename = "enabledForTemplateDeployment", default, skip_serializing_if = "Option::is_none")]
    pub enabled_for_template_deployment: Option<bool>,
    #[doc = "Property to specify whether the 'soft delete' functionality is enabled for this key vault. If it's not set to any value(true or false) when creating new key vault, it will be set to true by default. Once set to true, it cannot be reverted to false."]
    #[serde(rename = "enableSoftDelete", default, skip_serializing_if = "Option::is_none")]
    pub enable_soft_delete: Option<bool>,
    #[doc = "softDelete data retention days. It accepts >=7 and <=90."]
    #[serde(rename = "softDeleteRetentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub soft_delete_retention_in_days: Option<i32>,
    #[doc = "Property that controls how data actions are authorized. When true, the key vault will use Role Based Access Control (RBAC) for authorization of data actions, and the access policies specified in vault properties will be  ignored. When false, the key vault will use the access policies specified in vault properties, and any policy stored on Azure Resource Manager will be ignored. If null or not specified, the vault is created with the default value of false. Note that management actions are always authorized with RBAC."]
    #[serde(rename = "enableRbacAuthorization", default, skip_serializing_if = "Option::is_none")]
    pub enable_rbac_authorization: Option<bool>,
    #[doc = "The vault's create mode to indicate whether the vault need to be recovered or not."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<vault_properties::CreateMode>,
    #[doc = "Property specifying whether protection against purge is enabled for this vault. Setting this property to true activates protection against purge for this vault and its content - only the Key Vault service may initiate a hard, irrecoverable deletion. The setting is effective only if soft delete is also enabled. Enabling this functionality is irreversible - that is, the property does not accept false as its value."]
    #[serde(rename = "enablePurgeProtection", default, skip_serializing_if = "Option::is_none")]
    pub enable_purge_protection: Option<bool>,
    #[doc = "A set of rules governing the network accessibility of a vault."]
    #[serde(rename = "networkAcls", default, skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,
    #[doc = "Provisioning state of the vault."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<vault_properties::ProvisioningState>,
    #[doc = "List of private endpoint connections associated with the key vault."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnectionItem>,
}
impl VaultProperties {
    pub fn new(tenant_id: String, sku: Sku) -> Self {
        Self {
            tenant_id,
            sku,
            access_policies: Vec::new(),
            vault_uri: None,
            hsm_pool_resource_id: None,
            enabled_for_deployment: None,
            enabled_for_disk_encryption: None,
            enabled_for_template_deployment: None,
            enable_soft_delete: None,
            soft_delete_retention_in_days: None,
            enable_rbac_authorization: None,
            create_mode: None,
            enable_purge_protection: None,
            network_acls: None,
            provisioning_state: None,
            private_endpoint_connections: Vec::new(),
        }
    }
}
pub mod vault_properties {
    use super::*;
    #[doc = "The vault's create mode to indicate whether the vault need to be recovered or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateMode {
        #[serde(rename = "recover")]
        Recover,
        #[serde(rename = "default")]
        Default,
    }
    #[doc = "Provisioning state of the vault."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        RegisteringDns,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::RegisteringDns => serializer.serialize_unit_variant("ProvisioningState", 1u32, "RegisteringDns"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A rule governing the accessibility of a vault from a specific virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    #[doc = "Full resource id of a vnet subnet, such as '/subscriptions/subid/resourceGroups/rg1/providers/Microsoft.Network/virtualNetworks/test-vnet/subnets/subnet1'."]
    pub id: String,
    #[doc = "Property to specify whether NRP will ignore the check if parent subnet has serviceEndpoints configured."]
    #[serde(rename = "ignoreMissingVnetServiceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing_vnet_service_endpoint: Option<bool>,
}
impl VirtualNetworkRule {
    pub fn new(id: String) -> Self {
        Self {
            id,
            ignore_missing_vnet_service_endpoint: None,
        }
    }
}
