#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Extension scope settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterScopeSettings {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Extension scope settings"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<cluster_scope_settings::Properties>,
}
impl ClusterScopeSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cluster_scope_settings {
    use super::*;
    #[doc = "Extension scope settings"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Describes if multiple instances of the extension are allowed"]
        #[serde(rename = "allowMultipleInstances", default, skip_serializing_if = "Option::is_none")]
        pub allow_multiple_instances: Option<bool>,
        #[doc = "Default extension release namespace"]
        #[serde(rename = "defaultReleaseNamespace", default, skip_serializing_if = "Option::is_none")]
        pub default_release_namespace: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
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
#[doc = "The Extension Type object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionType {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<extension_type::Properties>,
}
impl ExtensionType {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod extension_type {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Is this Extension Type a system extension."]
        #[serde(rename = "isSystemExtension", default, skip_serializing_if = "Option::is_none")]
        pub is_system_extension: Option<bool>,
        #[doc = "Should an identity for this cluster resource be created"]
        #[serde(rename = "isManagedIdentityRequired", default, skip_serializing_if = "Option::is_none")]
        pub is_managed_identity_required: Option<bool>,
        #[doc = "Description of the extension type"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "Name of the publisher for the Extension Type"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub publisher: Option<String>,
        #[doc = "Plan information only for the Marketplace Extension Type."]
        #[serde(rename = "planInfo", default, skip_serializing_if = "Option::is_none")]
        pub plan_info: Option<properties::PlanInfo>,
        #[doc = "Cluster Types supported for this Extension Type."]
        #[serde(
            rename = "supportedClusterTypes",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub supported_cluster_types: Vec<String>,
        #[doc = "Supported Kubernetes Scopes for this Extension Type."]
        #[serde(rename = "supportedScopes", default, skip_serializing_if = "Option::is_none")]
        pub supported_scopes: Option<properties::SupportedScopes>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Plan information only for the Marketplace Extension Type."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct PlanInfo {
            #[doc = "Publisher ID of the Marketplace Extension Type."]
            #[serde(rename = "publisherId", default, skip_serializing_if = "Option::is_none")]
            pub publisher_id: Option<String>,
            #[doc = "Plan ID of the Marketplace Extension Type."]
            #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
            pub plan_id: Option<String>,
            #[doc = "Offer or Product ID of the Marketplace Extension Type."]
            #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
            pub offer_id: Option<String>,
        }
        impl PlanInfo {
            pub fn new() -> Self {
                Self::default()
            }
        }
        #[doc = "Supported Kubernetes Scopes for this Extension Type."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct SupportedScopes {
            #[doc = "The default scope of the extension type. This scope will be used if the user does not provide a scope while creating an extension."]
            #[serde(rename = "defaultScope", default, skip_serializing_if = "Option::is_none")]
            pub default_scope: Option<String>,
            #[doc = "Extension scope settings"]
            #[serde(rename = "clusterScopeSettings", default, skip_serializing_if = "Option::is_none")]
            pub cluster_scope_settings: Option<ClusterScopeSettings>,
        }
        impl SupportedScopes {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}
#[doc = "The Extension Type Version object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionTypeVersionForReleaseTrain {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<extension_type_version_for_release_train::Properties>,
}
impl ExtensionTypeVersionForReleaseTrain {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod extension_type_version_for_release_train {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The version number for the extension type"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
        #[doc = "The list of supported Kubernetes cluster versions for this extension type"]
        #[serde(rename = "unsupportedKubernetesVersions", default, skip_serializing_if = "Option::is_none")]
        pub unsupported_kubernetes_versions: Option<properties::UnsupportedKubernetesVersions>,
        #[doc = "A list of supported cluster types for this version of the Extension Type"]
        #[serde(
            rename = "supportedClusterTypes",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub supported_cluster_types: Vec<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "The list of supported Kubernetes cluster versions for this extension type"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct UnsupportedKubernetesVersions {
            #[serde(rename = "connectedCluster", default, skip_serializing_if = "Option::is_none")]
            pub connected_cluster: Option<ExtensionTypeVersionUnsupportedKubernetesMatrix>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub appliances: Option<ExtensionTypeVersionUnsupportedKubernetesMatrix>,
            #[serde(rename = "provisionedCluster", default, skip_serializing_if = "Option::is_none")]
            pub provisioned_cluster: Option<ExtensionTypeVersionUnsupportedKubernetesMatrix>,
            #[serde(rename = "managedCluster", default, skip_serializing_if = "Option::is_none")]
            pub managed_cluster: Option<ExtensionTypeVersionUnsupportedKubernetesMatrix>,
        }
        impl UnsupportedKubernetesVersions {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}
pub type ExtensionTypeVersionUnsupportedKubernetesMatrix = Vec<serde_json::Value>;
#[doc = "List Extension Type Versions. It contains a list of ExtensionTypeVersionForReleaseTrain objects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionTypeVersionsList {
    #[doc = "List of Extension Type Versions for an Extension Type in a Release Train."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ExtensionTypeVersionForReleaseTrain>,
    #[doc = "URL to get the next set of extension objects, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExtensionTypeVersionsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ExtensionTypeVersionsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List Extension Types. It contains a list of ExtensionType objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionTypesList {
    #[doc = "List of Extension Types."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ExtensionType>,
    #[doc = "URL to get the next set of extension type objects, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExtensionTypesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ExtensionTypesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The async operations in progress, in the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatusList {
    #[doc = "List of async operations in progress, in the cluster."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OperationStatusResult>,
    #[doc = "URL to get the next set of Operation Result objects, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationStatusList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationStatusList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current status of an async operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatusResult {
    #[doc = "Fully qualified ID for the async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Operation status."]
    pub status: String,
    #[doc = "Additional information, if available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl OperationStatusResult {
    pub fn new(status: String) -> Self {
        Self {
            id: None,
            name: None,
            status,
            properties: None,
            error: None,
        }
    }
}
#[doc = "The provisioning state of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningStateDefinition")]
pub enum ProvisioningStateDefinition {
    Succeeded,
    Failed,
    Canceled,
    Creating,
    Updating,
    Deleting,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProvisioningStateDefinition {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProvisioningStateDefinition {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProvisioningStateDefinition {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningStateDefinition", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningStateDefinition", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningStateDefinition", 2u32, "Canceled"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningStateDefinition", 3u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningStateDefinition", 4u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningStateDefinition", 5u32, "Deleting"),
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
#[doc = "Supported operation of this resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperation {
    #[doc = "Operation name, in format of {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<resource_provider_operation::Display>,
    #[doc = "The flag that indicates whether the operation applies to data plane."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl ResourceProviderOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_provider_operation {
    use super::*;
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Resource provider: Microsoft KubernetesConfiguration."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of operation: get, read, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of this operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationList {
    #[doc = "List of operations supported by this resource provider."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ResourceProviderOperation>,
    #[doc = "URL to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceProviderOperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceProviderOperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
