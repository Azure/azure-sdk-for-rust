#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Represents a Blueprint artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    #[doc = "Specifies the kind of Blueprint artifact."]
    pub kind: artifact::Kind,
}
impl Artifact {
    pub fn new(kind: artifact::Kind) -> Self {
        Self {
            azure_resource_base: AzureResourceBase::default(),
            kind,
        }
    }
}
pub mod artifact {
    use super::*;
    #[doc = "Specifies the kind of Blueprint artifact."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "template")]
        Template,
        #[serde(rename = "roleAssignment")]
        RoleAssignment,
        #[serde(rename = "policyAssignment")]
        PolicyAssignment,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Template => serializer.serialize_unit_variant("Kind", 0u32, "template"),
                Self::RoleAssignment => serializer.serialize_unit_variant("Kind", 1u32, "roleAssignment"),
                Self::PolicyAssignment => serializer.serialize_unit_variant("Kind", 2u32, "policyAssignment"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of Blueprint artifacts"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactList {
    #[doc = "List of Blueprint artifacts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Artifact>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ArtifactList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ArtifactList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common properties shared by different artifacts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactPropertiesBase {
    #[doc = "Artifacts which need to be deployed before the specified artifact."]
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
}
impl ArtifactPropertiesBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Blueprint assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Managed Service Identity"]
    pub identity: ManagedServiceIdentity,
    #[doc = "Detailed properties for Assignment."]
    pub properties: AssignmentProperties,
}
impl Assignment {
    pub fn new(tracked_resource: TrackedResource, identity: ManagedServiceIdentity, properties: AssignmentProperties) -> Self {
        Self {
            tracked_resource,
            identity,
            properties,
        }
    }
}
#[doc = "List of Blueprint assignments"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentList {
    #[doc = "List of Blueprint assignments."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Assignment>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AssignmentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AssignmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines how Blueprint-managed resources will be locked."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentLockSettings {
    #[doc = "Lock mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<assignment_lock_settings::Mode>,
}
impl AssignmentLockSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod assignment_lock_settings {
    use super::*;
    #[doc = "Lock mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        None,
        AllResources,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Mode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Mode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Mode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Mode", 0u32, "None"),
                Self::AllResources => serializer.serialize_unit_variant("Mode", 1u32, "AllResources"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Detailed properties for Assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[doc = "ID of the Blueprint definition resource."]
    #[serde(rename = "blueprintId", default, skip_serializing_if = "Option::is_none")]
    pub blueprint_id: Option<String>,
    #[doc = "A dictionary for parameters and their corresponding values."]
    pub parameters: ParameterValueCollection,
    #[doc = "A dictionary which maps resource group placeholders to the resource groups which will be created."]
    #[serde(rename = "resourceGroups")]
    pub resource_groups: ResourceGroupValueCollection,
    #[doc = "The status of Blueprint assignment. This field is readonly."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AssignmentStatus>,
    #[doc = "Defines how Blueprint-managed resources will be locked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locks: Option<AssignmentLockSettings>,
    #[doc = "State of the assignment."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<assignment_properties::ProvisioningState>,
}
impl AssignmentProperties {
    pub fn new(parameters: ParameterValueCollection, resource_groups: ResourceGroupValueCollection) -> Self {
        Self {
            blueprint_resource_properties_base: BlueprintResourcePropertiesBase::default(),
            blueprint_id: None,
            parameters,
            resource_groups,
            status: None,
            locks: None,
            provisioning_state: None,
        }
    }
}
pub mod assignment_properties {
    use super::*;
    #[doc = "State of the assignment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "validating")]
        Validating,
        #[serde(rename = "waiting")]
        Waiting,
        #[serde(rename = "deploying")]
        Deploying,
        #[serde(rename = "cancelling")]
        Cancelling,
        #[serde(rename = "locking")]
        Locking,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "deleting")]
        Deleting,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "creating"),
                Self::Validating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "validating"),
                Self::Waiting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "waiting"),
                Self::Deploying => serializer.serialize_unit_variant("ProvisioningState", 3u32, "deploying"),
                Self::Cancelling => serializer.serialize_unit_variant("ProvisioningState", 4u32, "cancelling"),
                Self::Locking => serializer.serialize_unit_variant("ProvisioningState", 5u32, "locking"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 6u32, "succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 7u32, "failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 8u32, "canceled"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 9u32, "deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The status of Blueprint assignment. This field is readonly."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentStatus {
    #[serde(flatten)]
    pub blueprint_resource_status_base: BlueprintResourceStatusBase,
}
impl AssignmentStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common properties for all Azure resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourceBase {
    #[doc = "String Id used to locate any resource on Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Type of this resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Name of this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl AzureResourceBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Blueprint definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Blueprint {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    #[doc = "Schema for Blueprint properties."]
    pub properties: BlueprintProperties,
}
impl Blueprint {
    pub fn new(properties: BlueprintProperties) -> Self {
        Self {
            azure_resource_base: AzureResourceBase::default(),
            properties,
        }
    }
}
#[doc = "List of Blueprint definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlueprintList {
    #[doc = "List of Blueprint definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Blueprint>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BlueprintList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BlueprintList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for Blueprint properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlueprintProperties {
    #[serde(flatten)]
    pub shared_blueprint_properties: SharedBlueprintProperties,
    #[doc = "Published versions of this blueprint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<serde_json::Value>,
    #[doc = "Layout view of the blueprint, for UI reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<serde_json::Value>,
}
impl BlueprintProperties {
    pub fn new() -> Self {
        Self {
            shared_blueprint_properties: SharedBlueprintProperties::default(),
            versions: None,
            layout: None,
        }
    }
}
#[doc = "Shared properties between all Blueprint resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlueprintResourcePropertiesBase {
    #[doc = "One-liner string explain this resource."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Multi-line explain this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl BlueprintResourcePropertiesBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Shared status properties between all Blueprint resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlueprintResourceStatusBase {
    #[doc = "Creation time of this blueprint."]
    #[serde(rename = "timeCreated", default, skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    #[doc = "Last modified time of this blueprint."]
    #[serde(rename = "lastModified", default, skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
}
impl BlueprintResourceStatusBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of the blueprint. This field is readonly."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlueprintStatus {
    #[serde(flatten)]
    pub blueprint_resource_status_base: BlueprintResourceStatusBase,
}
impl BlueprintStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed Service Identity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[doc = "Type of the Managed Service Identity."]
    #[serde(rename = "type")]
    pub type_: managed_service_identity::Type,
    #[doc = "Azure Active Directory principal ID associated with this Identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "ID of the Azure Active Directory."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: managed_service_identity::Type) -> Self {
        Self {
            type_,
            principal_id: None,
            tenant_id: None,
        }
    }
}
pub mod managed_service_identity {
    use super::*;
    #[doc = "Type of the Managed Service Identity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
        SystemAssigned,
        UserAssigned,
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
                Self::None => serializer.serialize_unit_variant("Type", 0u32, "None"),
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 1u32, "SystemAssigned"),
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 2u32, "UserAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represent a parameter with constrains and metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterDefinition {
    #[doc = "Allowed data types for Azure Resource Manager template parameters."]
    #[serde(rename = "type")]
    pub type_: parameter_definition::Type,
    #[doc = "User-friendly properties for this parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ParameterDefinitionMetadata>,
    #[doc = "Default Value for this parameter."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    #[doc = "Array of allowed values for this parameter."]
    #[serde(rename = "allowedValues", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_values: Vec<serde_json::Value>,
}
impl ParameterDefinition {
    pub fn new(type_: parameter_definition::Type) -> Self {
        Self {
            type_,
            metadata: None,
            default_value: None,
            allowed_values: Vec::new(),
        }
    }
}
pub mod parameter_definition {
    use super::*;
    #[doc = "Allowed data types for Azure Resource Manager template parameters."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "string")]
        String,
        #[serde(rename = "array")]
        Array,
        #[serde(rename = "bool")]
        Bool,
        #[serde(rename = "int")]
        Int,
        #[serde(rename = "object")]
        Object,
        #[serde(rename = "secureObject")]
        SecureObject,
        #[serde(rename = "secureString")]
        SecureString,
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
                Self::String => serializer.serialize_unit_variant("Type", 0u32, "string"),
                Self::Array => serializer.serialize_unit_variant("Type", 1u32, "array"),
                Self::Bool => serializer.serialize_unit_variant("Type", 2u32, "bool"),
                Self::Int => serializer.serialize_unit_variant("Type", 3u32, "int"),
                Self::Object => serializer.serialize_unit_variant("Type", 4u32, "object"),
                Self::SecureObject => serializer.serialize_unit_variant("Type", 5u32, "secureObject"),
                Self::SecureString => serializer.serialize_unit_variant("Type", 6u32, "secureString"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A dictionary hold parameter name and it's metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterDefinitionCollection {}
impl ParameterDefinitionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User-friendly properties for this parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterDefinitionMetadata {
    #[doc = "DisplayName of this parameter/resourceGroup."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Description of this parameter/resourceGroup."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "StrongType for UI to render rich experience during assignment time."]
    #[serde(rename = "strongType", default, skip_serializing_if = "Option::is_none")]
    pub strong_type: Option<String>,
}
impl ParameterDefinitionMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Value for the specified parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterValue {
    #[serde(flatten)]
    pub parameter_value_base: ParameterValueBase,
    #[doc = "actual value."]
    pub value: serde_json::Value,
}
impl ParameterValue {
    pub fn new(value: serde_json::Value) -> Self {
        Self {
            parameter_value_base: ParameterValueBase::default(),
            value,
        }
    }
}
#[doc = "Base class for ParameterValue."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterValueBase {
    #[doc = "Optional property, just to establish ParameterValueBase as a BaseClass."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ParameterValueBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A dictionary for parameters and their corresponding values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterValueCollection {}
impl ParameterValueCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Blueprint artifact applies Policy assignments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyAssignmentArtifact {
    #[serde(flatten)]
    pub artifact: Artifact,
    #[doc = "PolicyAssignment properties"]
    pub properties: PolicyAssignmentArtifactProperties,
}
impl PolicyAssignmentArtifact {
    pub fn new(artifact: Artifact, properties: PolicyAssignmentArtifactProperties) -> Self {
        Self { artifact, properties }
    }
}
#[doc = "PolicyAssignment properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyAssignmentArtifactProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[serde(flatten)]
    pub artifact_properties_base: ArtifactPropertiesBase,
    #[doc = "Azure resource ID of the policy definition."]
    #[serde(rename = "policyDefinitionId")]
    pub policy_definition_id: String,
    #[doc = "A dictionary for parameters and their corresponding values."]
    pub parameters: ParameterValueCollection,
    #[doc = "Name of the resource group placeholder to which the policy will be assigned."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl PolicyAssignmentArtifactProperties {
    pub fn new(policy_definition_id: String, parameters: ParameterValueCollection) -> Self {
        Self {
            blueprint_resource_properties_base: BlueprintResourcePropertiesBase::default(),
            artifact_properties_base: ArtifactPropertiesBase::default(),
            policy_definition_id,
            parameters,
            resource_group: None,
        }
    }
}
#[doc = "Represents a published Blueprint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublishedBlueprint {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    #[doc = "Schema for published Blueprint properties."]
    pub properties: PublishedBlueprintProperties,
}
impl PublishedBlueprint {
    pub fn new(properties: PublishedBlueprintProperties) -> Self {
        Self {
            azure_resource_base: AzureResourceBase::default(),
            properties,
        }
    }
}
#[doc = "List of published Blueprints"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublishedBlueprintList {
    #[doc = "List of published Blueprints."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PublishedBlueprint>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PublishedBlueprintList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PublishedBlueprintList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for published Blueprint properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublishedBlueprintProperties {
    #[serde(flatten)]
    pub shared_blueprint_properties: SharedBlueprintProperties,
    #[doc = "Name of the Blueprint definition."]
    #[serde(rename = "blueprintName", default, skip_serializing_if = "Option::is_none")]
    pub blueprint_name: Option<String>,
    #[doc = "Version-specific change notes"]
    #[serde(rename = "changeNotes", default, skip_serializing_if = "Option::is_none")]
    pub change_notes: Option<String>,
}
impl PublishedBlueprintProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an Azure resource group in a Blueprint definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGroupDefinition {
    #[doc = "Name of this resourceGroup, leave empty if the resource group name will be specified during the Blueprint assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Location of this resourceGroup, leave empty if the resource group location will be specified during the Blueprint assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "User-friendly properties for this parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ParameterDefinitionMetadata>,
    #[doc = "Artifacts which need to be deployed before this resource group."]
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
}
impl ResourceGroupDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A dictionary which maps resource group placeholders to the resource groups which will be created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGroupDefinitionCollection {}
impl ResourceGroupDefinitionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an Azure resource group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGroupValue {
    #[doc = "Name of the resource group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Location of the resource group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl ResourceGroupValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A dictionary which maps resource group placeholders to the resource groups which will be created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGroupValueCollection {}
impl ResourceGroupValueCollection {
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
        #[doc = "Resource provider: Microsoft Blueprint."]
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
}
impl ResourceProviderOperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Blueprint artifact applies Azure role assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentArtifact {
    #[serde(flatten)]
    pub artifact: Artifact,
    #[doc = "Properties of the Role assignment artifact."]
    pub properties: RoleAssignmentArtifactProperties,
}
impl RoleAssignmentArtifact {
    pub fn new(artifact: Artifact, properties: RoleAssignmentArtifactProperties) -> Self {
        Self { artifact, properties }
    }
}
#[doc = "Properties of the Role assignment artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentArtifactProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[serde(flatten)]
    pub artifact_properties_base: ArtifactPropertiesBase,
    #[doc = "Azure resource ID of the RoleDefinition."]
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
    #[doc = "Array of user or group identities in Azure Active Directory. The roleDefinition will apply to these identity."]
    #[serde(rename = "principalIds")]
    pub principal_ids: serde_json::Value,
    #[doc = "RoleAssignment will be scope to this resourceGroup, if left empty, it would scope to the subscription."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl RoleAssignmentArtifactProperties {
    pub fn new(role_definition_id: String, principal_ids: serde_json::Value) -> Self {
        Self {
            blueprint_resource_properties_base: BlueprintResourcePropertiesBase::default(),
            artifact_properties_base: ArtifactPropertiesBase::default(),
            role_definition_id,
            principal_ids,
            resource_group: None,
        }
    }
}
#[doc = "The reference to a secret, if the parameter should be protected."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretReferenceParameterValue {
    #[serde(flatten)]
    pub parameter_value_base: ParameterValueBase,
    #[doc = "Reference to a KeyVault secret."]
    pub reference: SecretValueReference,
}
impl SecretReferenceParameterValue {
    pub fn new(reference: SecretValueReference) -> Self {
        Self {
            parameter_value_base: ParameterValueBase::default(),
            reference,
        }
    }
}
#[doc = "Reference to a KeyVault secret."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretValueReference {
    #[doc = "Specifies the link to a KeyVault."]
    #[serde(rename = "keyVault")]
    pub key_vault: KeyVaultReference,
    #[doc = "Name of the secret."]
    #[serde(rename = "secretName")]
    pub secret_name: String,
    #[doc = "Version of the secret, (if there are multiple versions)"]
    #[serde(rename = "secretVersion", default, skip_serializing_if = "Option::is_none")]
    pub secret_version: Option<String>,
}
impl SecretValueReference {
    pub fn new(key_vault: KeyVaultReference, secret_name: String) -> Self {
        Self {
            key_vault,
            secret_name,
            secret_version: None,
        }
    }
}
#[doc = "Shared Schema for both blueprintProperties and publishedBlueprintProperties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedBlueprintProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[doc = "The status of the blueprint. This field is readonly."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<BlueprintStatus>,
    #[doc = "The scope where this Blueprint can be applied."]
    #[serde(rename = "targetScope", default, skip_serializing_if = "Option::is_none")]
    pub target_scope: Option<shared_blueprint_properties::TargetScope>,
    #[doc = "A dictionary hold parameter name and it's metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterDefinitionCollection>,
    #[doc = "A dictionary which maps resource group placeholders to the resource groups which will be created."]
    #[serde(rename = "resourceGroups", default, skip_serializing_if = "Option::is_none")]
    pub resource_groups: Option<ResourceGroupDefinitionCollection>,
}
impl SharedBlueprintProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod shared_blueprint_properties {
    use super::*;
    #[doc = "The scope where this Blueprint can be applied."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TargetScope")]
    pub enum TargetScope {
        #[serde(rename = "subscription")]
        Subscription,
        #[serde(rename = "managementGroup")]
        ManagementGroup,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TargetScope {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TargetScope {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TargetScope {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Subscription => serializer.serialize_unit_variant("TargetScope", 0u32, "subscription"),
                Self::ManagementGroup => serializer.serialize_unit_variant("TargetScope", 1u32, "managementGroup"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Blueprint artifact deploys Azure resource manager template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateArtifact {
    #[serde(flatten)]
    pub artifact: Artifact,
    #[doc = "Properties of a Template Artifact."]
    pub properties: TemplateArtifactProperties,
}
impl TemplateArtifact {
    pub fn new(artifact: Artifact, properties: TemplateArtifactProperties) -> Self {
        Self { artifact, properties }
    }
}
#[doc = "Properties of a Template Artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateArtifactProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[serde(flatten)]
    pub artifact_properties_base: ArtifactPropertiesBase,
    #[doc = "The Azure Resource Manager template body."]
    pub template: serde_json::Value,
    #[doc = "If applicable, the name of the resource group placeholder to which the template will be deployed."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "A dictionary for parameters and their corresponding values."]
    pub parameters: ParameterValueCollection,
}
impl TemplateArtifactProperties {
    pub fn new(template: serde_json::Value, parameters: ParameterValueCollection) -> Self {
        Self {
            blueprint_resource_properties_base: BlueprintResourcePropertiesBase::default(),
            artifact_properties_base: ArtifactPropertiesBase::default(),
            template,
            resource_group: None,
            parameters,
        }
    }
}
#[doc = "Common properties for all Azure tracked resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    #[doc = "The location of this Blueprint assignment."]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            azure_resource_base: AzureResourceBase::default(),
            location,
        }
    }
}
#[doc = "Specifies the link to a KeyVault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultReference {
    #[doc = "Azure resource ID of the KeyVault."]
    pub id: String,
}
impl KeyVaultReference {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
