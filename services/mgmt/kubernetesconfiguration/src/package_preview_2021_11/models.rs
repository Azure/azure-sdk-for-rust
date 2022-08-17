#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
pub type ChartValues = String;
pub type ChartVersion = String;
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
#[doc = "Compliance Status details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComplianceStatus {
    #[doc = "The compliance state of the configuration."]
    #[serde(rename = "complianceState", default, skip_serializing_if = "Option::is_none")]
    pub compliance_state: Option<compliance_status::ComplianceState>,
    #[doc = "Datetime the configuration was last applied."]
    #[serde(rename = "lastConfigApplied", default, with = "azure_core::date::rfc3339::option")]
    pub last_config_applied: Option<time::OffsetDateTime>,
    #[doc = "Message from when the configuration was applied."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Level of the message."]
    #[serde(rename = "messageLevel", default, skip_serializing_if = "Option::is_none")]
    pub message_level: Option<compliance_status::MessageLevel>,
}
impl ComplianceStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod compliance_status {
    use super::*;
    #[doc = "The compliance state of the configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ComplianceState")]
    pub enum ComplianceState {
        Pending,
        Compliant,
        Noncompliant,
        Installed,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ComplianceState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ComplianceState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ComplianceState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pending => serializer.serialize_unit_variant("ComplianceState", 0u32, "Pending"),
                Self::Compliant => serializer.serialize_unit_variant("ComplianceState", 1u32, "Compliant"),
                Self::Noncompliant => serializer.serialize_unit_variant("ComplianceState", 2u32, "Noncompliant"),
                Self::Installed => serializer.serialize_unit_variant("ComplianceState", 3u32, "Installed"),
                Self::Failed => serializer.serialize_unit_variant("ComplianceState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Level of the message."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MessageLevel")]
    pub enum MessageLevel {
        Error,
        Warning,
        Information,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MessageLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MessageLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MessageLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Error => serializer.serialize_unit_variant("MessageLevel", 0u32, "Error"),
                Self::Warning => serializer.serialize_unit_variant("MessageLevel", 1u32, "Warning"),
                Self::Information => serializer.serialize_unit_variant("MessageLevel", 2u32, "Information"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Name-value pairs of protected configuration settings for the configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationProtectedSettings {}
impl ConfigurationProtectedSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specify which kustomizations must succeed reconciliation on the cluster prior to reconciling this kustomization"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DependsOnDefinition {
    #[doc = "Name of the kustomization to claim dependency on"]
    #[serde(rename = "kustomizationName", default, skip_serializing_if = "Option::is_none")]
    pub kustomization_name: Option<String>,
}
impl DependsOnDefinition {
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
#[doc = "The Extension object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Extension {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an Extension resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<extension::Properties>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Extension {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod extension {
    use super::*;
    #[doc = "Properties of an Extension resource"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Type of the Extension, of which this resource is an instance of.  It must be one of the Extension Types registered with Microsoft.KubernetesConfiguration by the Extension publisher."]
        #[serde(rename = "extensionType", default, skip_serializing_if = "Option::is_none")]
        pub extension_type: Option<String>,
        #[doc = "Flag to note if this extension participates in auto upgrade of minor version, or not."]
        #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
        pub auto_upgrade_minor_version: Option<bool>,
        #[doc = "ReleaseTrain this extension participates in for auto-upgrade (e.g. Stable, Preview, etc.) - only if autoUpgradeMinorVersion is 'true'."]
        #[serde(rename = "releaseTrain", default, skip_serializing_if = "Option::is_none")]
        pub release_train: Option<String>,
        #[doc = "Version of the extension for this extension, if it is 'pinned' to a specific version. autoUpgradeMinorVersion must be 'false'."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
        #[doc = "Scope of the extension. It can be either Cluster or Namespace; but not both."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scope: Option<Scope>,
        #[doc = "Configuration settings, as name-value pairs for configuring this extension."]
        #[serde(rename = "configurationSettings", default, skip_serializing_if = "Option::is_none")]
        pub configuration_settings: Option<serde_json::Value>,
        #[doc = "Configuration settings that are sensitive, as name-value pairs for configuring this extension."]
        #[serde(rename = "configurationProtectedSettings", default, skip_serializing_if = "Option::is_none")]
        pub configuration_protected_settings: Option<serde_json::Value>,
        #[doc = "The provisioning state of the resource."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<ProvisioningStateDefinition>,
        #[doc = "Status from this extension."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub statuses: Vec<ExtensionStatus>,
        #[doc = "The error detail."]
        #[serde(rename = "errorInfo", default, skip_serializing_if = "Option::is_none")]
        pub error_info: Option<ErrorDetail>,
        #[doc = "Custom Location settings properties."]
        #[serde(rename = "customLocationSettings", default, skip_serializing_if = "Option::is_none")]
        pub custom_location_settings: Option<serde_json::Value>,
        #[doc = "Uri of the Helm package"]
        #[serde(rename = "packageUri", default, skip_serializing_if = "Option::is_none")]
        pub package_uri: Option<String>,
        #[doc = "Identity of the Extension resource in an AKS cluster"]
        #[serde(rename = "aksAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
        pub aks_assigned_identity: Option<properties::AksAssignedIdentity>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Identity of the Extension resource in an AKS cluster"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct AksAssignedIdentity {
            #[doc = "The principal ID of resource identity."]
            #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
            pub principal_id: Option<String>,
            #[doc = "The tenant ID of resource."]
            #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
            pub tenant_id: Option<String>,
            #[doc = "The identity type."]
            #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
            pub type_: Option<aks_assigned_identity::Type>,
        }
        impl AksAssignedIdentity {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod aks_assigned_identity {
            use super::*;
            #[doc = "The identity type."]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            pub enum Type {
                SystemAssigned,
            }
        }
    }
}
#[doc = "Status from the extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionStatus {
    #[doc = "Status code provided by the Extension"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Short description of status of the extension."]
    #[serde(rename = "displayStatus", default, skip_serializing_if = "Option::is_none")]
    pub display_status: Option<String>,
    #[doc = "Level of the status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<extension_status::Level>,
    #[doc = "Detailed message of the status from the Extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "DateLiteral (per ISO8601) noting the time of installation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}
impl ExtensionStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod extension_status {
    use super::*;
    #[doc = "Level of the status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Level")]
    pub enum Level {
        Error,
        Warning,
        Information,
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
                Self::Error => serializer.serialize_unit_variant("Level", 0u32, "Error"),
                Self::Warning => serializer.serialize_unit_variant("Level", 1u32, "Warning"),
                Self::Information => serializer.serialize_unit_variant("Level", 2u32, "Information"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Level {
        fn default() -> Self {
            Self::Information
        }
    }
}
#[doc = "Represents an Extension Type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionType {
    #[doc = "Properties of the connected cluster."]
    pub properties: ExtensionTypeProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ExtensionType {
    pub fn new(properties: ExtensionTypeProperties) -> Self {
        Self {
            properties,
            system_data: None,
        }
    }
}
#[doc = "List Extension Types"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionTypeList {
    #[doc = "The list of Extension Types"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ExtensionType>,
    #[doc = "The link to fetch the next page of Extension Types"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExtensionTypeList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ExtensionTypeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the connected cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionTypeProperties {
    #[doc = "Extension release train: preview or stable"]
    #[serde(rename = "releaseTrains", default, skip_serializing_if = "Vec::is_empty")]
    pub release_trains: Vec<String>,
    #[doc = "Cluster types"]
    #[serde(rename = "clusterTypes", default, skip_serializing_if = "Option::is_none")]
    pub cluster_types: Option<extension_type_properties::ClusterTypes>,
    #[doc = "Extension scopes"]
    #[serde(rename = "supportedScopes", default, skip_serializing_if = "Option::is_none")]
    pub supported_scopes: Option<SupportedScopes>,
}
impl ExtensionTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod extension_type_properties {
    use super::*;
    #[doc = "Cluster types"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ClusterTypes {
        #[serde(rename = "connectedClusters")]
        ConnectedClusters,
        #[serde(rename = "managedClusters")]
        ManagedClusters,
    }
}
#[doc = "List versions for an Extension"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionVersionList {
    #[doc = "Versions available for this Extension Type"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub versions: Vec<serde_json::Value>,
    #[doc = "The link to fetch the next page of Extension Types"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl azure_core::Continuable for ExtensionVersionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ExtensionVersionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list Extensions.  It contains a list of Extension objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionsList {
    #[doc = "List of Extensions within a Kubernetes cluster."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Extension>,
    #[doc = "URL to get the next set of extension objects, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExtensionsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ExtensionsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Compliance state of the cluster object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FluxComplianceStateDefinition")]
pub enum FluxComplianceStateDefinition {
    Compliant,
    #[serde(rename = "Non-Compliant")]
    NonCompliant,
    Pending,
    Suspended,
    Unknown,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FluxComplianceStateDefinition {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FluxComplianceStateDefinition {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FluxComplianceStateDefinition {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Compliant => serializer.serialize_unit_variant("FluxComplianceStateDefinition", 0u32, "Compliant"),
            Self::NonCompliant => serializer.serialize_unit_variant("FluxComplianceStateDefinition", 1u32, "Non-Compliant"),
            Self::Pending => serializer.serialize_unit_variant("FluxComplianceStateDefinition", 2u32, "Pending"),
            Self::Suspended => serializer.serialize_unit_variant("FluxComplianceStateDefinition", 3u32, "Suspended"),
            Self::Unknown => serializer.serialize_unit_variant("FluxComplianceStateDefinition", 4u32, "Unknown"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for FluxComplianceStateDefinition {
    fn default() -> Self {
        Self::Unknown
    }
}
#[doc = "The Flux Configuration object returned in Get & Put response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FluxConfiguration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties to create a Flux Configuration resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<flux_configuration::Properties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl FluxConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod flux_configuration {
    use super::*;
    #[doc = "Properties to create a Flux Configuration resource"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Scope at which the configuration will be installed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scope: Option<ScopeDefinition>,
        #[doc = "The namespace to which this configuration is installed to. Maximum of 253 lower case alphanumeric characters, hyphen and period only."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[doc = "Source Kind to pull the configuration data from."]
        #[serde(rename = "sourceKind", default, skip_serializing_if = "Option::is_none")]
        pub source_kind: Option<SourceKindDefinition>,
        #[doc = "Whether this configuration should suspend its reconciliation of its kustomizations and sources."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub suspend: Option<bool>,
        #[doc = "Parameters to reconcile to the GitRepository source kind type."]
        #[serde(rename = "gitRepository", default, skip_serializing_if = "Option::is_none")]
        pub git_repository: Option<GitRepositoryDefinition>,
        #[doc = "Array of kustomizations used to reconcile the artifact pulled by the source type on the cluster."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kustomizations: Option<serde_json::Value>,
        #[doc = "Key-value pairs of protected configuration settings for the configuration"]
        #[serde(rename = "configurationProtectedSettings", default, skip_serializing_if = "Option::is_none")]
        pub configuration_protected_settings: Option<serde_json::Value>,
        #[doc = "Statuses of the Flux Kubernetes resources created by the fluxConfiguration or created by the managed objects provisioned by the fluxConfiguration."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub statuses: Vec<ObjectStatusDefinition>,
        #[doc = "Public Key associated with this fluxConfiguration (either generated within the cluster or provided by the user)."]
        #[serde(rename = "repositoryPublicKey", default, skip_serializing_if = "Option::is_none")]
        pub repository_public_key: Option<String>,
        #[doc = "Branch and SHA of the last source commit synced with the cluster."]
        #[serde(rename = "lastSourceSyncedCommitId", default, skip_serializing_if = "Option::is_none")]
        pub last_source_synced_commit_id: Option<String>,
        #[doc = "Datetime the fluxConfiguration last synced its source on the cluster."]
        #[serde(rename = "lastSourceSyncedAt", default, with = "azure_core::date::rfc3339::option")]
        pub last_source_synced_at: Option<time::OffsetDateTime>,
        #[doc = "Compliance state of the cluster object."]
        #[serde(rename = "complianceState", default, skip_serializing_if = "Option::is_none")]
        pub compliance_state: Option<FluxComplianceStateDefinition>,
        #[doc = "The provisioning state of the resource."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<ProvisioningStateDefinition>,
        #[doc = "Error message returned to the user in the case of provisioning failure."]
        #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
        pub error_message: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The Flux Configuration Patch Request object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FluxConfigurationPatch {
    #[doc = "Updatable properties of an Flux Configuration Patch Request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<flux_configuration_patch::Properties>,
}
impl FluxConfigurationPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod flux_configuration_patch {
    use super::*;
    #[doc = "Updatable properties of an Flux Configuration Patch Request"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Source Kind to pull the configuration data from."]
        #[serde(rename = "sourceKind", default, skip_serializing_if = "Option::is_none")]
        pub source_kind: Option<SourceKindDefinition>,
        #[doc = "Whether this configuration should suspend its reconciliation of its kustomizations and sources."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub suspend: Option<bool>,
        #[doc = "Parameters to reconcile to the GitRepository source kind type."]
        #[serde(rename = "gitRepository", default, skip_serializing_if = "Option::is_none")]
        pub git_repository: Option<GitRepositoryDefinition>,
        #[doc = "Array of kustomizations used to reconcile the artifact pulled by the source type on the cluster."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kustomizations: Option<serde_json::Value>,
        #[doc = "Key-value pairs of protected configuration settings for the configuration"]
        #[serde(rename = "configurationProtectedSettings", default, skip_serializing_if = "Option::is_none")]
        pub configuration_protected_settings: Option<serde_json::Value>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Flux Configurations.  It contains a list of FluxConfiguration objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FluxConfigurationsList {
    #[doc = "List of Flux Configurations within a Kubernetes cluster."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FluxConfiguration>,
    #[doc = "URL to get the next set of configuration objects, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FluxConfigurationsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FluxConfigurationsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters to reconcile to the GitRepository source kind type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitRepositoryDefinition {
    #[doc = "The URL to sync for the flux configuration git repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The maximum time to attempt to reconcile the cluster git repository source with the remote."]
    #[serde(rename = "timeoutInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i64>,
    #[doc = "The interval at which to re-reconcile the cluster git repository source with the remote."]
    #[serde(rename = "syncIntervalInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub sync_interval_in_seconds: Option<i64>,
    #[doc = "The source reference for the GitRepository object."]
    #[serde(rename = "repositoryRef", default, skip_serializing_if = "Option::is_none")]
    pub repository_ref: Option<RepositoryRefDefinition>,
    #[doc = "Base64-encoded known_hosts value containing public SSH keys required to access private git repositories over SSH"]
    #[serde(rename = "sshKnownHosts", default, skip_serializing_if = "Option::is_none")]
    pub ssh_known_hosts: Option<String>,
    #[doc = "Base64-encoded HTTPS username used to access private git repositories over HTTPS"]
    #[serde(rename = "httpsUser", default, skip_serializing_if = "Option::is_none")]
    pub https_user: Option<String>,
    #[doc = "Base64-encoded HTTPS certificate authority contents used to access git private git repositories over HTTPS"]
    #[serde(rename = "httpsCAFile", default, skip_serializing_if = "Option::is_none")]
    pub https_ca_file: Option<String>,
    #[doc = "Name of a local secret on the Kubernetes cluster to use as the authentication secret rather than the managed or user-provided configuration secrets."]
    #[serde(rename = "localAuthRef", default, skip_serializing_if = "Option::is_none")]
    pub local_auth_ref: Option<String>,
}
impl GitRepositoryDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for Helm operator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HelmOperatorProperties {
    #[doc = "Version of the operator Helm chart."]
    #[serde(rename = "chartVersion", default, skip_serializing_if = "Option::is_none")]
    pub chart_version: Option<ChartVersion>,
    #[doc = "Values override for the operator Helm chart."]
    #[serde(rename = "chartValues", default, skip_serializing_if = "Option::is_none")]
    pub chart_values: Option<ChartValues>,
}
impl HelmOperatorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HelmReleasePropertiesDefinition {
    #[doc = "The revision number of the last released object change"]
    #[serde(rename = "lastRevisionApplied", default, skip_serializing_if = "Option::is_none")]
    pub last_revision_applied: Option<i64>,
    #[doc = "Object reference to a Kubernetes object on a cluster"]
    #[serde(rename = "helmChartRef", default, skip_serializing_if = "Option::is_none")]
    pub helm_chart_ref: Option<ObjectReferenceDefinition>,
    #[doc = "Total number of times that the HelmRelease failed to install or upgrade"]
    #[serde(rename = "failureCount", default, skip_serializing_if = "Option::is_none")]
    pub failure_count: Option<i64>,
    #[doc = "Number of times that the HelmRelease failed to install"]
    #[serde(rename = "installFailureCount", default, skip_serializing_if = "Option::is_none")]
    pub install_failure_count: Option<i64>,
    #[doc = "Number of times that the HelmRelease failed to upgrade"]
    #[serde(rename = "upgradeFailureCount", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_failure_count: Option<i64>,
}
impl HelmReleasePropertiesDefinition {
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
#[doc = "The Kustomization defining how to reconcile the artifact pulled by the source type on the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KustomizationDefinition {
    #[doc = "The path in the source reference to reconcile on the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Specifies other Kustomizations that this Kustomization depends on. This Kustomization will not reconcile until all dependencies have completed their reconciliation."]
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<DependsOnDefinition>,
    #[doc = "The maximum time to attempt to reconcile the Kustomization on the cluster."]
    #[serde(rename = "timeoutInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i64>,
    #[doc = "The interval at which to re-reconcile the Kustomization on the cluster."]
    #[serde(rename = "syncIntervalInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub sync_interval_in_seconds: Option<i64>,
    #[doc = "The interval at which to re-reconcile the Kustomization on the cluster in the event of failure on reconciliation."]
    #[serde(rename = "retryIntervalInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub retry_interval_in_seconds: Option<i64>,
    #[doc = "Enable/disable garbage collections of Kubernetes objects created by this Kustomization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prune: Option<bool>,
    #[doc = "Specify whether to validate the Kubernetes objects referenced in the Kustomization before applying them to the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<KustomizationValidationDefinition>,
    #[doc = "Enable/disable re-creating Kubernetes resources on the cluster when patching fails due to an immutable field change."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}
impl KustomizationDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specify whether to validate the Kubernetes objects referenced in the Kustomization before applying them to the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "KustomizationValidationDefinition")]
pub enum KustomizationValidationDefinition {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "client")]
    Client,
    #[serde(rename = "server")]
    Server,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for KustomizationValidationDefinition {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for KustomizationValidationDefinition {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for KustomizationValidationDefinition {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("KustomizationValidationDefinition", 0u32, "none"),
            Self::Client => serializer.serialize_unit_variant("KustomizationValidationDefinition", 1u32, "client"),
            Self::Server => serializer.serialize_unit_variant("KustomizationValidationDefinition", 2u32, "server"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for KustomizationValidationDefinition {
    fn default() -> Self {
        Self::None
    }
}
#[doc = "Object reference to a Kubernetes object on a cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ObjectReferenceDefinition {
    #[doc = "Name of the object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Namespace of the object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
impl ObjectReferenceDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status condition of Kubernetes object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ObjectStatusConditionDefinition {
    #[doc = "Last time this status condition has changed"]
    #[serde(rename = "lastTransitionTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_transition_time: Option<time::OffsetDateTime>,
    #[doc = "A more verbose description of the object status condition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Reason for the specified status condition type status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Status of the Kubernetes object condition type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Object status condition type for this object"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ObjectStatusConditionDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Statuses of objects deployed by the user-specified kustomizations from the git repository."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ObjectStatusDefinition {
    #[doc = "Name of the applied object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Namespace of the applied object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "Kind of the applied object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Compliance state of the cluster object."]
    #[serde(rename = "complianceState", default, skip_serializing_if = "Option::is_none")]
    pub compliance_state: Option<FluxComplianceStateDefinition>,
    #[doc = "Object reference to a Kubernetes object on a cluster"]
    #[serde(rename = "appliedBy", default, skip_serializing_if = "Option::is_none")]
    pub applied_by: Option<ObjectReferenceDefinition>,
    #[doc = "List of Kubernetes object status conditions present on the cluster"]
    #[serde(rename = "statusConditions", default, skip_serializing_if = "Vec::is_empty")]
    pub status_conditions: Vec<ObjectStatusConditionDefinition>,
    #[serde(rename = "helmReleaseProperties", default, skip_serializing_if = "Option::is_none")]
    pub helm_release_properties: Option<HelmReleasePropertiesDefinition>,
}
impl ObjectStatusDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The async operations in progress, in the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatusList {
    #[doc = "List of async operations in progress, in the cluster."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "Scope at which the operator will be installed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperatorScopeDefinition")]
pub enum OperatorScopeDefinition {
    #[serde(rename = "cluster")]
    Cluster,
    #[serde(rename = "namespace")]
    Namespace,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperatorScopeDefinition {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperatorScopeDefinition {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperatorScopeDefinition {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Cluster => serializer.serialize_unit_variant("OperatorScopeDefinition", 0u32, "cluster"),
            Self::Namespace => serializer.serialize_unit_variant("OperatorScopeDefinition", 1u32, "namespace"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for OperatorScopeDefinition {
    fn default() -> Self {
        Self::Cluster
    }
}
#[doc = "Type of the operator"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperatorTypeDefinition")]
pub enum OperatorTypeDefinition {
    Flux,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperatorTypeDefinition {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperatorTypeDefinition {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperatorTypeDefinition {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Flux => serializer.serialize_unit_variant("OperatorTypeDefinition", 0u32, "Flux"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
#[doc = "The source reference for the GitRepository object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RepositoryRefDefinition {
    #[doc = "The git repository branch name to checkout."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[doc = "The git repository tag name to checkout. This takes precedence over branch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "The semver range used to match against git repository tags. This takes precedence over tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semver: Option<String>,
    #[doc = "The commit SHA to checkout. This value must be combined with the branch name to be valid. This takes precedence over semver."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
}
impl RepositoryRefDefinition {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "Scope of the extension. It can be either Cluster or Namespace; but not both."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Scope {
    #[doc = "Specifies that the scope of the extension is Cluster"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<ScopeCluster>,
    #[doc = "Specifies that the scope of the extension is Namespace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<ScopeNamespace>,
}
impl Scope {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies that the scope of the extension is Cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopeCluster {
    #[doc = "Namespace where the extension Release must be placed, for a Cluster scoped extension.  If this namespace does not exist, it will be created"]
    #[serde(rename = "releaseNamespace", default, skip_serializing_if = "Option::is_none")]
    pub release_namespace: Option<String>,
}
impl ScopeCluster {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Scope at which the configuration will be installed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScopeDefinition")]
pub enum ScopeDefinition {
    #[serde(rename = "cluster")]
    Cluster,
    #[serde(rename = "namespace")]
    Namespace,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScopeDefinition {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScopeDefinition {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScopeDefinition {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Cluster => serializer.serialize_unit_variant("ScopeDefinition", 0u32, "cluster"),
            Self::Namespace => serializer.serialize_unit_variant("ScopeDefinition", 1u32, "namespace"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for ScopeDefinition {
    fn default() -> Self {
        Self::Cluster
    }
}
#[doc = "Specifies that the scope of the extension is Namespace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopeNamespace {
    #[doc = "Namespace where the extension will be created for an Namespace scoped extension.  If this namespace does not exist, it will be created"]
    #[serde(rename = "targetNamespace", default, skip_serializing_if = "Option::is_none")]
    pub target_namespace: Option<String>,
}
impl ScopeNamespace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SourceControl Configuration object returned in Get & Put response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControlConfiguration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties to create a Source Control Configuration resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<source_control_configuration::Properties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl SourceControlConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod source_control_configuration {
    use super::*;
    #[doc = "Properties to create a Source Control Configuration resource"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Url of the SourceControl Repository."]
        #[serde(rename = "repositoryUrl", default, skip_serializing_if = "Option::is_none")]
        pub repository_url: Option<String>,
        #[doc = "The namespace to which this operator is installed to. Maximum of 253 lower case alphanumeric characters, hyphen and period only."]
        #[serde(rename = "operatorNamespace", default, skip_serializing_if = "Option::is_none")]
        pub operator_namespace: Option<String>,
        #[doc = "Instance name of the operator - identifying the specific configuration."]
        #[serde(rename = "operatorInstanceName", default, skip_serializing_if = "Option::is_none")]
        pub operator_instance_name: Option<String>,
        #[doc = "Type of the operator"]
        #[serde(rename = "operatorType", default, skip_serializing_if = "Option::is_none")]
        pub operator_type: Option<OperatorTypeDefinition>,
        #[doc = "Any Parameters for the Operator instance in string format."]
        #[serde(rename = "operatorParams", default, skip_serializing_if = "Option::is_none")]
        pub operator_params: Option<String>,
        #[doc = "Name-value pairs of protected configuration settings for the configuration"]
        #[serde(rename = "configurationProtectedSettings", default, skip_serializing_if = "Option::is_none")]
        pub configuration_protected_settings: Option<ConfigurationProtectedSettings>,
        #[doc = "Scope at which the operator will be installed."]
        #[serde(rename = "operatorScope", default, skip_serializing_if = "Option::is_none")]
        pub operator_scope: Option<OperatorScopeDefinition>,
        #[doc = "Public Key associated with this SourceControl configuration (either generated within the cluster or provided by the user)."]
        #[serde(rename = "repositoryPublicKey", default, skip_serializing_if = "Option::is_none")]
        pub repository_public_key: Option<String>,
        #[doc = "Base64-encoded known_hosts contents containing public SSH keys required to access private Git instances"]
        #[serde(rename = "sshKnownHostsContents", default, skip_serializing_if = "Option::is_none")]
        pub ssh_known_hosts_contents: Option<String>,
        #[doc = "Option to enable Helm Operator for this git configuration."]
        #[serde(rename = "enableHelmOperator", default, skip_serializing_if = "Option::is_none")]
        pub enable_helm_operator: Option<bool>,
        #[doc = "Properties for Helm operator."]
        #[serde(rename = "helmOperatorProperties", default, skip_serializing_if = "Option::is_none")]
        pub helm_operator_properties: Option<HelmOperatorProperties>,
        #[doc = "The provisioning state of the resource provider."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "Compliance Status details"]
        #[serde(rename = "complianceStatus", default, skip_serializing_if = "Option::is_none")]
        pub compliance_status: Option<ComplianceStatus>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "The provisioning state of the resource provider."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "ProvisioningState")]
        pub enum ProvisioningState {
            Accepted,
            Deleting,
            Running,
            Succeeded,
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
                    Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Accepted"),
                    Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Deleting"),
                    Self::Running => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Running"),
                    Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                    Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Result of the request to list Source Control Configurations.  It contains a list of SourceControlConfiguration objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControlConfigurationList {
    #[doc = "List of Source Control Configurations within a Kubernetes cluster."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SourceControlConfiguration>,
    #[doc = "URL to get the next set of configuration objects, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SourceControlConfigurationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SourceControlConfigurationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Source Kind to pull the configuration data from."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SourceKindDefinition")]
pub enum SourceKindDefinition {
    GitRepository,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SourceKindDefinition {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SourceKindDefinition {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SourceKindDefinition {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::GitRepository => serializer.serialize_unit_variant("SourceKindDefinition", 0u32, "GitRepository"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Extension scopes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedScopes {
    #[doc = "Default extension scopes: cluster or namespace"]
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
#[doc = "The Extension Patch Request object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchExtension {
    #[doc = "Updatable properties of an Extension Patch Request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<patch_extension::Properties>,
}
impl PatchExtension {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod patch_extension {
    use super::*;
    #[doc = "Updatable properties of an Extension Patch Request"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Flag to note if this extension participates in auto upgrade of minor version, or not."]
        #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
        pub auto_upgrade_minor_version: Option<bool>,
        #[doc = "ReleaseTrain this extension participates in for auto-upgrade (e.g. Stable, Preview, etc.) - only if autoUpgradeMinorVersion is 'true'."]
        #[serde(rename = "releaseTrain", default, skip_serializing_if = "Option::is_none")]
        pub release_train: Option<String>,
        #[doc = "Version of the extension for this extension, if it is 'pinned' to a specific version. autoUpgradeMinorVersion must be 'false'."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
        #[doc = "Configuration settings, as name-value pairs for configuring this extension."]
        #[serde(rename = "configurationSettings", default, skip_serializing_if = "Option::is_none")]
        pub configuration_settings: Option<serde_json::Value>,
        #[doc = "Configuration settings that are sensitive, as name-value pairs for configuring this extension."]
        #[serde(rename = "configurationProtectedSettings", default, skip_serializing_if = "Option::is_none")]
        pub configuration_protected_settings: Option<serde_json::Value>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
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
