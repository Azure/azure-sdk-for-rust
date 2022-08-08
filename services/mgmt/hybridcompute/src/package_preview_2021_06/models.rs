#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionDetail {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The private endpoint connection private ip address"]
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "The private endpoint connection link identifier"]
    #[serde(rename = "linkIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub link_identifier: Option<String>,
    #[doc = "The private endpoint connection group id"]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private endpoint connection member name"]
    #[serde(rename = "memberName", default, skip_serializing_if = "Option::is_none")]
    pub member_name: Option<String>,
}
impl ConnectionDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Detected properties from the machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DetectedProperties {}
impl DetectedProperties {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "Describes the Machine Extension Target Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionTarget {}
impl ExtensionTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the Machine Extension Target Version Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionTargetProperties {
    #[doc = "Extension Upgrade Target Version."]
    #[serde(rename = "targetVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_version: Option<TargetVersion>,
}
impl ExtensionTargetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Arc PrivateLinkScope definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridComputePrivateLinkScope {
    #[serde(flatten)]
    pub private_link_scopes_resource: PrivateLinkScopesResource,
    #[doc = "Properties that define a Azure Arc PrivateLinkScope resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HybridComputePrivateLinkScopeProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl HybridComputePrivateLinkScope {
    pub fn new(private_link_scopes_resource: PrivateLinkScopesResource) -> Self {
        Self {
            private_link_scopes_resource,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "Describes the list of Azure Arc PrivateLinkScope resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridComputePrivateLinkScopeListResult {
    #[doc = "List of Azure Arc PrivateLinkScope definitions."]
    pub value: Vec<HybridComputePrivateLinkScope>,
    #[doc = "The URI to get the next set of Azure Arc PrivateLinkScope definitions if too many PrivateLinkScopes where returned in the result set."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HybridComputePrivateLinkScopeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HybridComputePrivateLinkScopeListResult {
    pub fn new(value: Vec<HybridComputePrivateLinkScope>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties that define a Azure Arc PrivateLinkScope resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridComputePrivateLinkScopeProperties {
    #[doc = "The network access policy to determine if Azure Arc agents can use public Azure Arc service endpoints. Defaults to disabled (access to Azure Arc services only via private link)."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccessType>,
    #[doc = "Current state of this PrivateLinkScope: whether or not is has been provisioned within the resource group it is defined. Users cannot change this value but are able to read from it. Values will include Provisioning ,Succeeded, Canceled and Failed."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The Guid id of the private link scope."]
    #[serde(rename = "privateLinkScopeId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_scope_id: Option<String>,
    #[doc = "The collection of associated Private Endpoint Connections."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnectionDataModel>,
}
impl HybridComputePrivateLinkScopeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[doc = "The principal ID of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The identity type."]
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
    #[doc = "The identity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[doc = "Describes a hybrid machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Machine {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Describes the properties of a hybrid machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineProperties>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Machine {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
            system_data: None,
        }
    }
}
#[doc = "Describes a Machine Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineExtension {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Describes the properties of a Machine Extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineExtensionProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl MachineExtension {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "Describes the Machine Extension Instance View."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionInstanceView {
    #[doc = "The machine extension name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the type of the extension; an example is \"CustomScriptExtension\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the version of the script handler."]
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[doc = "Instance view status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<machine_extension_instance_view::Status>,
}
impl MachineExtensionInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod machine_extension_instance_view {
    use super::*;
    #[doc = "Instance view status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Status {
        #[doc = "The status code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "The level code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub level: Option<status::Level>,
        #[doc = "The short localizable label for the status."]
        #[serde(rename = "displayStatus", default, skip_serializing_if = "Option::is_none")]
        pub display_status: Option<String>,
        #[doc = "The detailed status message, including for alerts and error messages."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[doc = "The time of the status."]
        #[serde(with = "azure_core::date::rfc3339::option")]
        pub time: Option<time::OffsetDateTime>,
    }
    impl Status {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod status {
        use super::*;
        #[doc = "The level code."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Level")]
        pub enum Level {
            Info,
            Warning,
            Error,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for Level {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for Level {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for Level {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Info => serializer.serialize_unit_variant("Level", 0u32, "Info"),
                    Self::Warning => serializer.serialize_unit_variant("Level", 1u32, "Warning"),
                    Self::Error => serializer.serialize_unit_variant("Level", 2u32, "Error"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Describes the properties of a Machine Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionProperties {
    #[doc = "How the extension handler should be forced to update even if the extension configuration has not changed."]
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
    #[doc = "The name of the extension handler publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Specifies the type of the extension; an example is \"CustomScriptExtension\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the version of the script handler."]
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[doc = "Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true."]
    #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_minor_version: Option<bool>,
    #[doc = "Json formatted public settings for the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[doc = "The extension can contain either protectedSettings or protectedSettingsFromKeyVault or no protected settings at all."]
    #[serde(rename = "protectedSettings", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings: Option<serde_json::Value>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Describes the Machine Extension Instance View."]
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<MachineExtensionInstanceView>,
}
impl MachineExtensionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Machine Extension Update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionUpdate {
    #[serde(flatten)]
    pub resource_update: ResourceUpdate,
    #[doc = "Describes the properties of a Machine Extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineExtensionUpdateProperties>,
}
impl MachineExtensionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Machine Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionUpdateProperties {
    #[doc = "How the extension handler should be forced to update even if the extension configuration has not changed."]
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
    #[doc = "The name of the extension handler publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Specifies the type of the extension; an example is \"CustomScriptExtension\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the version of the script handler."]
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[doc = "Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true."]
    #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_minor_version: Option<bool>,
    #[doc = "Json formatted public settings for the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[doc = "The extension can contain either protectedSettings or protectedSettingsFromKeyVault or no protected settings at all."]
    #[serde(rename = "protectedSettings", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings: Option<serde_json::Value>,
}
impl MachineExtensionUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the Machine Extension Upgrade Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionUpgrade {
    #[doc = "Describes the Machine Extension Target Properties"]
    #[serde(rename = "extensionTargets", default, skip_serializing_if = "Option::is_none")]
    pub extension_targets: Option<ExtensionTarget>,
}
impl MachineExtensionUpgrade {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the Machine Extensions List Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionsListResult {
    #[doc = "The list of extensions"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MachineExtension>,
    #[doc = "The uri to fetch the next page of machine extensions. Call ListNext() with this to fetch the next page of extensions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MachineExtensionsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MachineExtensionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List hybrid machine operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineListResult {
    #[doc = "The list of hybrid machines."]
    pub value: Vec<Machine>,
    #[doc = "The URI to fetch the next page of Machines. Call ListNext() with this URI to fetch the next page of hybrid machines."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MachineListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MachineListResult {
    pub fn new(value: Vec<Machine>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the properties of a hybrid machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineProperties {
    #[doc = "Metadata pertaining to the geographic location of the resource."]
    #[serde(rename = "locationData", default, skip_serializing_if = "Option::is_none")]
    pub location_data: Option<LocationData>,
    #[doc = "Specifies the operating system settings for the hybrid machine."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The status of the hybrid machine agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<machine_properties::Status>,
    #[doc = "The time of the last status change."]
    #[serde(rename = "lastStatusChange", with = "azure_core::date::rfc3339::option")]
    pub last_status_change: Option<time::OffsetDateTime>,
    #[doc = "Details about the error state."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<ErrorDetail>,
    #[doc = "The hybrid machine agent full version."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "Specifies the hybrid machine unique ID."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "Specifies the hybrid machine display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Specifies the hybrid machine FQDN."]
    #[serde(rename = "machineFqdn", default, skip_serializing_if = "Option::is_none")]
    pub machine_fqdn: Option<String>,
    #[doc = "Public Key that the client provides to be used during initial resource onboarding"]
    #[serde(rename = "clientPublicKey", default, skip_serializing_if = "Option::is_none")]
    pub client_public_key: Option<String>,
    #[doc = "The Operating System running on the hybrid machine."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "The version of Operating System running on the hybrid machine."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[doc = "The type of Operating System (windows/linux)."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "Specifies the Arc Machine's unique SMBIOS ID"]
    #[serde(rename = "vmUuid", default, skip_serializing_if = "Option::is_none")]
    pub vm_uuid: Option<String>,
    #[doc = "Machine Extensions information"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<MachineExtensionInstanceView>,
    #[doc = "Specifies the Operating System product SKU."]
    #[serde(rename = "osSku", default, skip_serializing_if = "Option::is_none")]
    pub os_sku: Option<String>,
    #[doc = "Specifies the Windows domain name."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "Specifies the AD fully qualified display name."]
    #[serde(rename = "adFqdn", default, skip_serializing_if = "Option::is_none")]
    pub ad_fqdn: Option<String>,
    #[doc = "Specifies the DNS fully qualified display name."]
    #[serde(rename = "dnsFqdn", default, skip_serializing_if = "Option::is_none")]
    pub dns_fqdn: Option<String>,
    #[doc = "The resource id of the private link scope this machine is assigned to, if any."]
    #[serde(rename = "privateLinkScopeResourceId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_scope_resource_id: Option<String>,
    #[doc = "The resource id of the parent cluster (Azure HCI) this machine is assigned to, if any."]
    #[serde(rename = "parentClusterResourceId", default, skip_serializing_if = "Option::is_none")]
    pub parent_cluster_resource_id: Option<String>,
    #[doc = "Specifies whether any MS SQL instance is discovered on the machine."]
    #[serde(rename = "mssqlDiscovered", default, skip_serializing_if = "Option::is_none")]
    pub mssql_discovered: Option<String>,
    #[doc = "Detected properties from the machine."]
    #[serde(rename = "detectedProperties", default, skip_serializing_if = "Option::is_none")]
    pub detected_properties: Option<DetectedProperties>,
}
impl MachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod machine_properties {
    use super::*;
    #[doc = "The status of the hybrid machine agent."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Connected,
        Disconnected,
        Error,
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
                Self::Connected => serializer.serialize_unit_variant("Status", 0u32, "Connected"),
                Self::Disconnected => serializer.serialize_unit_variant("Status", 1u32, "Disconnected"),
                Self::Error => serializer.serialize_unit_variant("Status", 2u32, "Error"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a hybrid machine Update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineUpdate {
    #[serde(flatten)]
    pub resource_update: ResourceUpdate,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Describes the ARM updatable properties of a hybrid machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineUpdateProperties>,
}
impl MachineUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the ARM updatable properties of a hybrid machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineUpdateProperties {
    #[doc = "Metadata pertaining to the geographic location of the resource."]
    #[serde(rename = "locationData", default, skip_serializing_if = "Option::is_none")]
    pub location_data: Option<LocationData>,
    #[doc = "Specifies the operating system settings for the hybrid machine."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[doc = "The resource id of the parent cluster (Azure HCI) this machine is assigned to, if any."]
    #[serde(rename = "parentClusterResourceId", default, skip_serializing_if = "Option::is_none")]
    pub parent_cluster_resource_id: Option<String>,
    #[doc = "The resource id of the private link scope this machine is assigned to, if any."]
    #[serde(rename = "privateLinkScopeResourceId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_scope_resource_id: Option<String>,
}
impl MachineUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the operating system settings for the hybrid machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsProfile {
    #[doc = "Specifies the host OS name of the hybrid machine."]
    #[serde(rename = "computerName", default, skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[doc = "Specifies the windows configuration for update management."]
    #[serde(rename = "windowsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<os_profile::WindowsConfiguration>,
    #[doc = "Specifies the linux configuration for update management."]
    #[serde(rename = "linuxConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub linux_configuration: Option<os_profile::LinuxConfiguration>,
}
impl OsProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod os_profile {
    use super::*;
    #[doc = "Specifies the windows configuration for update management."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct WindowsConfiguration {
        #[doc = "Specifies the patch settings."]
        #[serde(rename = "patchSettings", default, skip_serializing_if = "Option::is_none")]
        pub patch_settings: Option<PatchSettings>,
    }
    impl WindowsConfiguration {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Specifies the linux configuration for update management."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct LinuxConfiguration {
        #[doc = "Specifies the patch settings."]
        #[serde(rename = "patchSettings", default, skip_serializing_if = "Option::is_none")]
        pub patch_settings: Option<PatchSettings>,
    }
    impl LinuxConfiguration {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The List Compute Operation operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The list of compute operations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationValue>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Compute Operation value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationValue {
    #[doc = "The origin of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The name of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Describes the properties of a Hybrid Compute Operation Value Display."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationValueDisplay>,
}
impl OperationValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Hybrid Compute Operation Value Display."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationValueDisplay {
    #[doc = "The display name of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The display name of the resource the operation applies to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The resource provider for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
impl OperationValueDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the patch settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchSettings {
    #[doc = "Specifies the assessment mode."]
    #[serde(rename = "assessmentMode", default, skip_serializing_if = "Option::is_none")]
    pub assessment_mode: Option<String>,
}
impl PatchSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Data Model for a Private Endpoint Connection associated with a Private Link Scope"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionDataModel {
    #[doc = "The ARM Resource Id of the Private Endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The Name of the Private Endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of a private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnectionDataModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private endpoint connections."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "Link to retrieve next page of results."]
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
#[doc = "Properties of a private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "Private endpoint which the connection belongs to."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpointProperty>,
    #[doc = "State of the private endpoint connection."]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionStateProperty>,
    #[doc = "State of the private endpoint connection."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint which the connection belongs to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointProperty {
    #[doc = "Resource id of the private endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpointProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateLinkResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource required member names."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkScopeValidationDetails {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The network access policy to determine if Azure Arc agents can use public Azure Arc service endpoints. Defaults to disabled (access to Azure Arc services only via private link)."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccessType>,
    #[doc = "List of Private Endpoint Connection details."]
    #[serde(rename = "connectionDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub connection_details: Vec<ConnectionDetail>,
}
impl PrivateLinkScopeValidationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkScopesResource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PrivateLinkScopesResource {
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
#[doc = "State of the private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionStateProperty {
    #[doc = "The private link service connection status."]
    pub status: String,
    #[doc = "The private link service connection description."]
    pub description: String,
    #[doc = "The actions required for private link service connection."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionStateProperty {
    pub fn new(status: String, description: String) -> Self {
        Self {
            status,
            description,
            actions_required: None,
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
#[doc = "The network access policy to determine if Azure Arc agents can use public Azure Arc service endpoints. Defaults to disabled (access to Azure Arc services only via private link)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PublicNetworkAccessType")]
pub enum PublicNetworkAccessType {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PublicNetworkAccessType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PublicNetworkAccessType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PublicNetworkAccessType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccessType", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccessType", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for PublicNetworkAccessType {
    fn default() -> Self {
        Self::Disabled
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Update Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceUpdate {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container holding only the Tags for a resource, allowing the user to update the tags on a PrivateLinkScope instance."]
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
pub type TargetVersion = String;
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
#[doc = "Metadata pertaining to the geographic location of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationData {
    #[doc = "A canonical name for the geographic or physical location."]
    pub name: String,
    #[doc = "The city or locality where the resource is located."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "The district, state, or province where the resource is located."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[doc = "The country or region where the resource is located"]
    #[serde(rename = "countryOrRegion", default, skip_serializing_if = "Option::is_none")]
    pub country_or_region: Option<String>,
}
impl LocationData {
    pub fn new(name: String) -> Self {
        Self {
            name,
            city: None,
            district: None,
            country_or_region: None,
        }
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
    #[doc = "The timestamp of resource last modification (UTC)"]
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
