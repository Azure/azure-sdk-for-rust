#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Details of the certificate to be uploaded to the vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateRequest {
    #[doc = "Raw certificate data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RawCertificateData>,
}
impl CertificateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource Name availability input parameters - Resource type and resource name"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityParameters {
    #[doc = "Describes the Resource type: Microsoft.RecoveryServices/Vaults"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource name for which availability needs to be checked"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl CheckNameAvailabilityParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for check name availability API. Resource provider will set availability as true | false."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Localized display information of an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryDisplay {
    #[doc = "Name of the provider for display purposes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "ResourceType for which this Operation can be performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operations Name itself."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation having details of what operation is about."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ClientDiscoveryDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to represent shoebox log specification in json client discovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryForLogSpecification {
    #[doc = "Name of the log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Blobs created in customer storage account per hour"]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl ClientDiscoveryForLogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to represent shoebox properties in json client discovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryForProperties {
    #[doc = "Class to represent shoebox service specification in json client discovery."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ClientDiscoveryForServiceSpecification>,
}
impl ClientDiscoveryForProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to represent shoebox service specification in json client discovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryForServiceSpecification {
    #[doc = "List of log specifications of this operation."]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<ClientDiscoveryForLogSpecification>,
}
impl ClientDiscoveryForServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operations List response which contains list of available APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryResponse {
    #[doc = "List of available operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ClientDiscoveryValueForSingleApi>,
    #[doc = "Link to the next chunk of the response"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ClientDiscoveryResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ClientDiscoveryResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Available operation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryValueForSingleApi {
    #[doc = "Name of the Operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized display information of an operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<ClientDiscoveryDisplay>,
    #[doc = "The intended executor of the operation;governs the display of the operation in the RBAC UX and the audit logs UX"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Class to represent shoebox properties in json client discovery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClientDiscoveryForProperties>,
}
impl ClientDiscoveryValueForSingleApi {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from Azure Backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "The resource management error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
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
#[doc = "The details of the identity used for CMK"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CmkKekIdentity {
    #[doc = "Indicate that system assigned identity should be used. Mutually exclusive with 'userAssignedIdentity' field"]
    #[serde(rename = "useSystemAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub use_system_assigned_identity: Option<bool>,
    #[doc = "The user assigned identity to be used to grant permissions in case the type of identity used is UserAssigned"]
    #[serde(rename = "userAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identity: Option<String>,
}
impl CmkKekIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the Key Vault which hosts CMK"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CmkKeyVaultProperties {
    #[doc = "The key uri of the Customer Managed Key"]
    #[serde(rename = "keyUri", default, skip_serializing_if = "Option::is_none")]
    pub key_uri: Option<String>,
}
impl CmkKeyVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource management error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<Error>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityData {
    #[doc = "The principal ID of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of managed identity used. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user-assigned identities. The type 'None' will remove any identities."]
    #[serde(rename = "type")]
    pub type_: identity_data::Type,
    #[doc = "The list of user-assigned identities associated with the resource. The user-assigned identity dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl IdentityData {
    pub fn new(type_: identity_data::Type) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
pub mod identity_data {
    use super::*;
    #[doc = "The type of managed identity used. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user-assigned identities. The type 'None' will remove any identities."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        SystemAssigned,
        None,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
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
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 0u32, "SystemAssigned"),
                Self::None => serializer.serialize_unit_variant("Type", 1u32, "None"),
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 2u32, "UserAssigned"),
                Self::SystemAssignedUserAssigned => serializer.serialize_unit_variant("Type", 3u32, "SystemAssigned, UserAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Summary of the replication job data for this vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobsSummary {
    #[doc = "Count of failed jobs."]
    #[serde(rename = "failedJobs", default, skip_serializing_if = "Option::is_none")]
    pub failed_jobs: Option<i64>,
    #[doc = "Count of suspended jobs."]
    #[serde(rename = "suspendedJobs", default, skip_serializing_if = "Option::is_none")]
    pub suspended_jobs: Option<i64>,
    #[doc = "Count of in-progress jobs."]
    #[serde(rename = "inProgressJobs", default, skip_serializing_if = "Option::is_none")]
    pub in_progress_jobs: Option<i64>,
}
impl JobsSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Summary of the replication monitoring data for this vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringSummary {
    #[doc = "Count of unhealthy VMs."]
    #[serde(rename = "unHealthyVmCount", default, skip_serializing_if = "Option::is_none")]
    pub un_healthy_vm_count: Option<i64>,
    #[doc = "Count of unhealthy replication providers."]
    #[serde(rename = "unHealthyProviderCount", default, skip_serializing_if = "Option::is_none")]
    pub un_healthy_provider_count: Option<i64>,
    #[doc = "Count of all critical warnings."]
    #[serde(rename = "eventsCount", default, skip_serializing_if = "Option::is_none")]
    pub events_count: Option<i64>,
    #[doc = "Count of all deprecated recovery service providers."]
    #[serde(rename = "deprecatedProviderCount", default, skip_serializing_if = "Option::is_none")]
    pub deprecated_provider_count: Option<i64>,
    #[doc = "Count of all the supported recovery service providers."]
    #[serde(rename = "supportedProviderCount", default, skip_serializing_if = "Option::is_none")]
    pub supported_provider_count: Option<i64>,
    #[doc = "Count of all the unsupported recovery service providers."]
    #[serde(rename = "unsupportedProviderCount", default, skip_serializing_if = "Option::is_none")]
    pub unsupported_provider_count: Option<i64>,
}
impl MonitoringSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The name of usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameInfo {
    #[doc = "Value of usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Localized value of usage."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl NameInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResource {
    #[doc = "End time of the operation"]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The resource management error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
    #[doc = "It should match what is used to GET the operation result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "It must match the last segment of the \"id\" field, and will typically be a GUID / system generated value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of the operation. (InProgress/Success/Failed/Cancelled)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Start time of the operation"]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
}
impl OperationResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tracked resource with location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchTrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PatchTrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Patch Resource information, as returned by the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchVault {
    #[serde(flatten)]
    pub patch_tracked_resource: PatchTrackedResource,
    #[doc = "Properties of the vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VaultProperties>,
    #[doc = "Identifies the unique system identifier for each Azure resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityData>,
}
impl PatchVault {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint network resource that is linked to the Private Endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "Gets or sets id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private Endpoint Connection Response Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[doc = "Gets or sets provisioning state of the private endpoint connection."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<private_endpoint_connection::ProvisioningState>,
    #[doc = "The Private Endpoint network resource that is linked to the Private Endpoint connection."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "Gets or sets private link service connection state."]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_endpoint_connection {
    use super::*;
    #[doc = "Gets or sets provisioning state of the private endpoint connection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Deleting,
        Failed,
        Pending,
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
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Deleting"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Pending"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Information to be stored in Vault properties as an element of privateEndpointConnections List."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionVaultProperties {
    #[doc = "Format of id subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.[Service]/{resource}/{resourceName}/privateEndpointConnections/{connectionName}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Private Endpoint Connection Response Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnection>,
    #[doc = "The name of the private Endpoint Connection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type, which will be of the format, Microsoft.RecoveryServices/vaults/privateEndpointConnections"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The location of the private Endpoint connection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl PrivateEndpointConnectionVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information of the private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[doc = "Properties of the private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
    #[doc = "Fully qualified identifier of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "e.g. Microsoft.RecoveryServices/vaults/privateLinkResources"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "e.g. f9ad6492-33d4-4690-9999-6bfd52a0d081 (Backup) or f9ad6492-33d4-4690-9999-6bfd52a0d082 (SiteRecovery)"]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "[backup-ecs1, backup-prot1, backup-prot1b, backup-prot1c, backup-id1]"]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "The private link resource Private link DNS zone name."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class which represent the stamps associated with the vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResources {
    #[doc = "A collection of private link resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "Link to the next chunk of the response"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateLinkResources {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateLinkResources {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets private link service connection state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "Gets or sets the status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<private_link_service_connection_state::Status>,
    #[doc = "Gets or sets description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets actions required."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_link_service_connection_state {
    use super::*;
    #[doc = "Gets or sets the status."]
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
#[doc = "Raw certificate data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RawCertificateData {
    #[doc = "Specifies the authentication type."]
    #[serde(rename = "authType", default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<raw_certificate_data::AuthType>,
    #[doc = "The base64 encoded certificate raw data string"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}
impl RawCertificateData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod raw_certificate_data {
    use super::*;
    #[doc = "Specifies the authentication type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthType")]
    pub enum AuthType {
        Invalid,
        #[serde(rename = "ACS")]
        Acs,
        #[serde(rename = "AAD")]
        Aad,
        AccessControlService,
        AzureActiveDirectory,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuthType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuthType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuthType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("AuthType", 0u32, "Invalid"),
                Self::Acs => serializer.serialize_unit_variant("AuthType", 1u32, "ACS"),
                Self::Aad => serializer.serialize_unit_variant("AuthType", 2u32, "AAD"),
                Self::AccessControlService => serializer.serialize_unit_variant("AuthType", 3u32, "AccessControlService"),
                Self::AzureActiveDirectory => serializer.serialize_unit_variant("AuthType", 4u32, "AzureActiveDirectory"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Replication usages of a vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationUsage {
    #[doc = "Summary of the replication monitoring data for this vault."]
    #[serde(rename = "monitoringSummary", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_summary: Option<MonitoringSummary>,
    #[doc = "Summary of the replication job data for this vault."]
    #[serde(rename = "jobsSummary", default, skip_serializing_if = "Option::is_none")]
    pub jobs_summary: Option<JobsSummary>,
    #[doc = "Number of replication protected items for this vault."]
    #[serde(rename = "protectedItemCount", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_count: Option<i64>,
    #[doc = "Number of replication recovery plans for this vault."]
    #[serde(rename = "recoveryPlanCount", default, skip_serializing_if = "Option::is_none")]
    pub recovery_plan_count: Option<i64>,
    #[doc = "Number of servers registered to this vault."]
    #[serde(rename = "registeredServersCount", default, skip_serializing_if = "Option::is_none")]
    pub registered_servers_count: Option<i64>,
    #[doc = "The authentication type of recovery service providers in the vault."]
    #[serde(rename = "recoveryServicesProviderAuthType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_services_provider_auth_type: Option<i64>,
}
impl ReplicationUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Replication usages for vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationUsageList {
    #[doc = "The list of replication usages for the given vault."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReplicationUsage>,
}
impl azure_core::Continuable for ReplicationUsageList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ReplicationUsageList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ARM Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id represents the complete path to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name associated with the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type represents the complete path of the form Namespace/ResourceType/ResourceType/..."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Optional ETag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Certificate details representing the Vault credentials for AAD."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceCertificateAndAadDetails {
    #[serde(flatten)]
    pub resource_certificate_details: ResourceCertificateDetails,
    #[doc = "AAD tenant authority."]
    #[serde(rename = "aadAuthority")]
    pub aad_authority: String,
    #[doc = "AAD tenant Id."]
    #[serde(rename = "aadTenantId")]
    pub aad_tenant_id: String,
    #[doc = "AAD service principal clientId."]
    #[serde(rename = "servicePrincipalClientId")]
    pub service_principal_client_id: String,
    #[doc = "AAD service principal ObjectId."]
    #[serde(rename = "servicePrincipalObjectId")]
    pub service_principal_object_id: String,
    #[doc = "Azure Management Endpoint Audience."]
    #[serde(rename = "azureManagementEndpointAudience")]
    pub azure_management_endpoint_audience: String,
    #[doc = "Service Resource Id."]
    #[serde(rename = "serviceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub service_resource_id: Option<String>,
}
impl ResourceCertificateAndAadDetails {
    pub fn new(
        resource_certificate_details: ResourceCertificateDetails,
        aad_authority: String,
        aad_tenant_id: String,
        service_principal_client_id: String,
        service_principal_object_id: String,
        azure_management_endpoint_audience: String,
    ) -> Self {
        Self {
            resource_certificate_details,
            aad_authority,
            aad_tenant_id,
            service_principal_client_id,
            service_principal_object_id,
            azure_management_endpoint_audience,
            service_resource_id: None,
        }
    }
}
#[doc = "Certificate details representing the Vault credentials for ACS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceCertificateAndAcsDetails {
    #[serde(flatten)]
    pub resource_certificate_details: ResourceCertificateDetails,
    #[doc = "ACS namespace name - tenant for our service."]
    #[serde(rename = "globalAcsNamespace")]
    pub global_acs_namespace: String,
    #[doc = "Acs mgmt host name to connect to."]
    #[serde(rename = "globalAcsHostName")]
    pub global_acs_host_name: String,
    #[doc = "Global ACS namespace RP realm."]
    #[serde(rename = "globalAcsRPRealm")]
    pub global_acs_rp_realm: String,
}
impl ResourceCertificateAndAcsDetails {
    pub fn new(
        resource_certificate_details: ResourceCertificateDetails,
        global_acs_namespace: String,
        global_acs_host_name: String,
        global_acs_rp_realm: String,
    ) -> Self {
        Self {
            resource_certificate_details,
            global_acs_namespace,
            global_acs_host_name,
            global_acs_rp_realm,
        }
    }
}
#[doc = "Certificate details representing the Vault credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceCertificateDetails {
    #[doc = "This property will be used as the discriminator for deciding the specific types in the polymorphic chain of types."]
    #[serde(rename = "authType")]
    pub auth_type: String,
    #[doc = "The base64 encoded certificate raw data string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[doc = "Certificate friendly name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Certificate issuer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[doc = "Resource ID of the vault."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<i64>,
    #[doc = "Certificate Subject Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Certificate thumbprint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "Certificate Validity start Date time."]
    #[serde(rename = "validFrom", with = "azure_core::date::rfc3339::option")]
    pub valid_from: Option<time::OffsetDateTime>,
    #[doc = "Certificate Validity End Date time."]
    #[serde(rename = "validTo", with = "azure_core::date::rfc3339::option")]
    pub valid_to: Option<time::OffsetDateTime>,
}
impl ResourceCertificateDetails {
    pub fn new(auth_type: String) -> Self {
        Self {
            auth_type,
            certificate: None,
            friendly_name: None,
            issuer: None,
            resource_id: None,
            subject: None,
            thumbprint: None,
            valid_from: None,
            valid_to: None,
        }
    }
}
#[doc = "Identifies the unique system identifier for each Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The Sku name."]
    pub name: sku::Name,
    #[doc = "The Sku tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The sku family"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The sku size"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "The sku capacity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
        Self {
            name,
            tier: None,
            family: None,
            size: None,
            capacity: None,
        }
    }
}
pub mod sku {
    use super::*;
    #[doc = "The Sku name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Standard,
        #[serde(rename = "RS0")]
        Rs0,
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
                Self::Standard => serializer.serialize_unit_variant("Name", 0u32, "Standard"),
                Self::Rs0 => serializer.serialize_unit_variant("Name", 1u32, "RS0"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Tracked resource with location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource location."]
    pub location: String,
    #[doc = "Resource tags."]
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
#[doc = "Details for upgrading vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpgradeDetails {
    #[doc = "ID of the vault upgrade operation."]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "UTC time at which the upgrade operation has started."]
    #[serde(rename = "startTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub start_time_utc: Option<time::OffsetDateTime>,
    #[doc = "UTC time at which the upgrade operation status was last updated."]
    #[serde(rename = "lastUpdatedTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub last_updated_time_utc: Option<time::OffsetDateTime>,
    #[doc = "UTC time at which the upgrade operation has ended."]
    #[serde(rename = "endTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub end_time_utc: Option<time::OffsetDateTime>,
    #[doc = "Status of the vault upgrade operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<upgrade_details::Status>,
    #[doc = "Message to the user containing information about the upgrade operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The way the vault upgrade was triggered."]
    #[serde(rename = "triggerType", default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<upgrade_details::TriggerType>,
    #[doc = "Resource ID of the upgraded vault."]
    #[serde(rename = "upgradedResourceId", default, skip_serializing_if = "Option::is_none")]
    pub upgraded_resource_id: Option<String>,
    #[doc = "Resource ID of the vault before the upgrade."]
    #[serde(rename = "previousResourceId", default, skip_serializing_if = "Option::is_none")]
    pub previous_resource_id: Option<String>,
}
impl UpgradeDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod upgrade_details {
    use super::*;
    #[doc = "Status of the vault upgrade operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Unknown,
        InProgress,
        Upgraded,
        Failed,
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
                Self::Unknown => serializer.serialize_unit_variant("Status", 0u32, "Unknown"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Upgraded => serializer.serialize_unit_variant("Status", 2u32, "Upgraded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The way the vault upgrade was triggered."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TriggerType")]
    pub enum TriggerType {
        UserTriggered,
        ForcedUpgrade,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TriggerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TriggerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TriggerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::UserTriggered => serializer.serialize_unit_variant("TriggerType", 0u32, "UserTriggered"),
                Self::ForcedUpgrade => serializer.serialize_unit_variant("TriggerType", 1u32, "ForcedUpgrade"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A resource identity that is managed by the user of the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserIdentity {
    #[doc = "The principal ID of the user-assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the user-assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource information, as returned by the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vault {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityData>,
    #[doc = "Properties of the vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VaultProperties>,
    #[doc = "Identifies the unique system identifier for each Azure resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Vault {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
            properties: None,
            sku: None,
            system_data: None,
        }
    }
}
#[doc = "Certificate corresponding to a vault that can be used by clients to register themselves with the vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultCertificateResponse {
    #[doc = "Resource name associated with the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type represents the complete path of the form Namespace/ResourceType/ResourceType/..."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource Id represents the complete path to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Certificate details representing the Vault credentials."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourceCertificateDetails>,
}
impl VaultCertificateResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Vault extended information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultExtendedInfo {
    #[doc = "Integrity key."]
    #[serde(rename = "integrityKey", default, skip_serializing_if = "Option::is_none")]
    pub integrity_key: Option<String>,
    #[doc = "Encryption key."]
    #[serde(rename = "encryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[doc = "Encryption key thumbprint."]
    #[serde(rename = "encryptionKeyThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key_thumbprint: Option<String>,
    #[doc = "Algorithm for Vault ExtendedInfo"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
}
impl VaultExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Vault extended information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultExtendedInfoResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Vault extended information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VaultExtendedInfo>,
}
impl VaultExtendedInfoResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model for a list of Vaults."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Vault>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VaultList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VaultList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultProperties {
    #[doc = "Provisioning State."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Details for upgrading vault."]
    #[serde(rename = "upgradeDetails", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_details: Option<UpgradeDetails>,
    #[doc = "List of private endpoint connection."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnectionVaultProperties>,
    #[doc = "Private endpoint state for backup."]
    #[serde(rename = "privateEndpointStateForBackup", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint_state_for_backup: Option<vault_properties::PrivateEndpointStateForBackup>,
    #[doc = "Private endpoint state for site recovery."]
    #[serde(rename = "privateEndpointStateForSiteRecovery", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint_state_for_site_recovery: Option<vault_properties::PrivateEndpointStateForSiteRecovery>,
    #[doc = "Customer Managed Key details of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<vault_properties::Encryption>,
    #[doc = "The details of the latest move operation performed on the Azure Resource"]
    #[serde(rename = "moveDetails", default, skip_serializing_if = "Option::is_none")]
    pub move_details: Option<vault_properties::MoveDetails>,
    #[doc = "The State of the Resource after the move operation"]
    #[serde(rename = "moveState", default, skip_serializing_if = "Option::is_none")]
    pub move_state: Option<vault_properties::MoveState>,
    #[doc = "Backup storage version"]
    #[serde(rename = "backupStorageVersion", default, skip_serializing_if = "Option::is_none")]
    pub backup_storage_version: Option<vault_properties::BackupStorageVersion>,
}
impl VaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vault_properties {
    use super::*;
    #[doc = "Private endpoint state for backup."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrivateEndpointStateForBackup")]
    pub enum PrivateEndpointStateForBackup {
        None,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrivateEndpointStateForBackup {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrivateEndpointStateForBackup {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrivateEndpointStateForBackup {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("PrivateEndpointStateForBackup", 0u32, "None"),
                Self::Enabled => serializer.serialize_unit_variant("PrivateEndpointStateForBackup", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Private endpoint state for site recovery."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrivateEndpointStateForSiteRecovery")]
    pub enum PrivateEndpointStateForSiteRecovery {
        None,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrivateEndpointStateForSiteRecovery {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrivateEndpointStateForSiteRecovery {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrivateEndpointStateForSiteRecovery {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("PrivateEndpointStateForSiteRecovery", 0u32, "None"),
                Self::Enabled => serializer.serialize_unit_variant("PrivateEndpointStateForSiteRecovery", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Customer Managed Key details of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Encryption {
        #[doc = "The properties of the Key Vault which hosts CMK"]
        #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
        pub key_vault_properties: Option<CmkKeyVaultProperties>,
        #[doc = "The details of the identity used for CMK"]
        #[serde(rename = "kekIdentity", default, skip_serializing_if = "Option::is_none")]
        pub kek_identity: Option<CmkKekIdentity>,
        #[doc = "Enabling/Disabling the Double Encryption state"]
        #[serde(rename = "infrastructureEncryption", default, skip_serializing_if = "Option::is_none")]
        pub infrastructure_encryption: Option<encryption::InfrastructureEncryption>,
    }
    impl Encryption {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod encryption {
        use super::*;
        #[doc = "Enabling/Disabling the Double Encryption state"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "InfrastructureEncryption")]
        pub enum InfrastructureEncryption {
            Enabled,
            Disabled,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for InfrastructureEncryption {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for InfrastructureEncryption {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for InfrastructureEncryption {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Enabled => serializer.serialize_unit_variant("InfrastructureEncryption", 0u32, "Enabled"),
                    Self::Disabled => serializer.serialize_unit_variant("InfrastructureEncryption", 1u32, "Disabled"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
    #[doc = "The details of the latest move operation performed on the Azure Resource"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct MoveDetails {
        #[doc = "OperationId of the Resource Move Operation"]
        #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
        pub operation_id: Option<String>,
        #[doc = "Start Time of the Resource Move Operation"]
        #[serde(rename = "startTimeUtc", with = "azure_core::date::rfc3339::option")]
        pub start_time_utc: Option<time::OffsetDateTime>,
        #[doc = "End Time of the Resource Move Operation"]
        #[serde(rename = "completionTimeUtc", with = "azure_core::date::rfc3339::option")]
        pub completion_time_utc: Option<time::OffsetDateTime>,
        #[doc = "Source Resource of the Resource Move Operation"]
        #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
        pub source_resource_id: Option<String>,
        #[doc = "Target Resource of the Resource Move Operation"]
        #[serde(rename = "targetResourceId", default, skip_serializing_if = "Option::is_none")]
        pub target_resource_id: Option<String>,
    }
    impl MoveDetails {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The State of the Resource after the move operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MoveState")]
    pub enum MoveState {
        Unknown,
        InProgress,
        PrepareFailed,
        CommitFailed,
        PrepareTimedout,
        CommitTimedout,
        MoveSucceeded,
        Failure,
        CriticalFailure,
        PartialSuccess,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MoveState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MoveState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MoveState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("MoveState", 0u32, "Unknown"),
                Self::InProgress => serializer.serialize_unit_variant("MoveState", 1u32, "InProgress"),
                Self::PrepareFailed => serializer.serialize_unit_variant("MoveState", 2u32, "PrepareFailed"),
                Self::CommitFailed => serializer.serialize_unit_variant("MoveState", 3u32, "CommitFailed"),
                Self::PrepareTimedout => serializer.serialize_unit_variant("MoveState", 4u32, "PrepareTimedout"),
                Self::CommitTimedout => serializer.serialize_unit_variant("MoveState", 5u32, "CommitTimedout"),
                Self::MoveSucceeded => serializer.serialize_unit_variant("MoveState", 6u32, "MoveSucceeded"),
                Self::Failure => serializer.serialize_unit_variant("MoveState", 7u32, "Failure"),
                Self::CriticalFailure => serializer.serialize_unit_variant("MoveState", 8u32, "CriticalFailure"),
                Self::PartialSuccess => serializer.serialize_unit_variant("MoveState", 9u32, "PartialSuccess"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Backup storage version"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupStorageVersion")]
    pub enum BackupStorageVersion {
        V1,
        V2,
        Unassigned,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupStorageVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupStorageVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupStorageVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::V1 => serializer.serialize_unit_variant("BackupStorageVersion", 0u32, "V1"),
                Self::V2 => serializer.serialize_unit_variant("BackupStorageVersion", 1u32, "V2"),
                Self::Unassigned => serializer.serialize_unit_variant("BackupStorageVersion", 2u32, "Unassigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Usages of a vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultUsage {
    #[doc = "Unit of the usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<vault_usage::Unit>,
    #[doc = "Quota period of usage."]
    #[serde(rename = "quotaPeriod", default, skip_serializing_if = "Option::is_none")]
    pub quota_period: Option<String>,
    #[doc = "Next reset time of usage."]
    #[serde(rename = "nextResetTime", with = "azure_core::date::rfc3339::option")]
    pub next_reset_time: Option<time::OffsetDateTime>,
    #[doc = "Current value of usage."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[doc = "Limit of usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "The name of usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<NameInfo>,
}
impl VaultUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vault_usage {
    use super::*;
    #[doc = "Unit of the usage."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        Count,
        Bytes,
        Seconds,
        Percent,
        CountPerSecond,
        BytesPerSecond,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Count => serializer.serialize_unit_variant("Unit", 0u32, "Count"),
                Self::Bytes => serializer.serialize_unit_variant("Unit", 1u32, "Bytes"),
                Self::Seconds => serializer.serialize_unit_variant("Unit", 2u32, "Seconds"),
                Self::Percent => serializer.serialize_unit_variant("Unit", 3u32, "Percent"),
                Self::CountPerSecond => serializer.serialize_unit_variant("Unit", 4u32, "CountPerSecond"),
                Self::BytesPerSecond => serializer.serialize_unit_variant("Unit", 5u32, "BytesPerSecond"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Usage for vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultUsageList {
    #[doc = "The list of usages for the given vault."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VaultUsage>,
}
impl azure_core::Continuable for VaultUsageList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl VaultUsageList {
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
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
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
