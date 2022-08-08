#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The alias type. "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Alias {
    #[doc = "The alias name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The paths for an alias."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<AliasPath>,
    #[doc = "The type of the alias."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<alias::Type>,
    #[doc = "The default path for an alias."]
    #[serde(rename = "defaultPath", default, skip_serializing_if = "Option::is_none")]
    pub default_path: Option<String>,
    #[doc = "The type of the pattern for an alias path."]
    #[serde(rename = "defaultPattern", default, skip_serializing_if = "Option::is_none")]
    pub default_pattern: Option<AliasPattern>,
    #[serde(rename = "defaultMetadata", default, skip_serializing_if = "Option::is_none")]
    pub default_metadata: Option<AliasPathMetadata>,
}
impl Alias {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod alias {
    use super::*;
    #[doc = "The type of the alias."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        NotSpecified,
        PlainText,
        Mask,
    }
}
#[doc = "The type of the paths for alias."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AliasPath {
    #[doc = "The path of an alias."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The API versions."]
    #[serde(rename = "apiVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,
    #[doc = "The type of the pattern for an alias path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<AliasPattern>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AliasPathMetadata>,
}
impl AliasPath {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AliasPathMetadata {
    #[doc = "The type of the token that the alias path is referring to."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<alias_path_metadata::Type>,
    #[doc = "The attributes of the token that the alias path is referring to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<alias_path_metadata::Attributes>,
}
impl AliasPathMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod alias_path_metadata {
    use super::*;
    #[doc = "The type of the token that the alias path is referring to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        NotSpecified,
        Any,
        String,
        Object,
        Array,
        Integer,
        Number,
        Boolean,
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
                Self::NotSpecified => serializer.serialize_unit_variant("Type", 0u32, "NotSpecified"),
                Self::Any => serializer.serialize_unit_variant("Type", 1u32, "Any"),
                Self::String => serializer.serialize_unit_variant("Type", 2u32, "String"),
                Self::Object => serializer.serialize_unit_variant("Type", 3u32, "Object"),
                Self::Array => serializer.serialize_unit_variant("Type", 4u32, "Array"),
                Self::Integer => serializer.serialize_unit_variant("Type", 5u32, "Integer"),
                Self::Number => serializer.serialize_unit_variant("Type", 6u32, "Number"),
                Self::Boolean => serializer.serialize_unit_variant("Type", 7u32, "Boolean"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The attributes of the token that the alias path is referring to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Attributes")]
    pub enum Attributes {
        None,
        Modifiable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Attributes {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Attributes {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Attributes {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Attributes", 0u32, "None"),
                Self::Modifiable => serializer.serialize_unit_variant("Attributes", 1u32, "Modifiable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The type of the pattern for an alias path."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AliasPattern {
    #[doc = "The alias pattern phrase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phrase: Option<String>,
    #[doc = "The alias pattern variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable: Option<String>,
    #[doc = "The type of alias pattern"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<alias_pattern::Type>,
}
impl AliasPattern {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod alias_pattern {
    use super::*;
    #[doc = "The type of alias pattern"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        NotSpecified,
        Extract,
    }
}
#[doc = "An error response from a policy operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
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
#[doc = "The data effect definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataEffect {
    #[doc = "The data effect name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The data effect details schema."]
    #[serde(rename = "detailsSchema", default, skip_serializing_if = "Option::is_none")]
    pub details_schema: Option<serde_json::Value>,
}
impl DataEffect {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The custom resource function definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataManifestCustomResourceFunctionDefinition {
    #[doc = "The function name as it will appear in the policy rule. eg - 'vault'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The fully qualified control plane resource type that this function represents. eg - 'Microsoft.KeyVault/vaults'."]
    #[serde(rename = "fullyQualifiedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_resource_type: Option<String>,
    #[doc = "The top-level properties that can be selected on the function's output. eg - [ \"name\", \"location\" ] if vault().name and vault().location are supported"]
    #[serde(rename = "defaultProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub default_properties: Vec<String>,
    #[doc = "A value indicating whether the custom properties within the property bag are allowed. Needs api-version to be specified in the policy rule eg - vault('2019-06-01')."]
    #[serde(rename = "allowCustomProperties", default, skip_serializing_if = "Option::is_none")]
    pub allow_custom_properties: Option<bool>,
}
impl DataManifestCustomResourceFunctionDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource functions supported by a manifest"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataManifestResourceFunctionsDefinition {
    #[doc = "The standard resource functions (subscription and/or resourceGroup)."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub standard: Vec<String>,
    #[doc = "An array of data manifest custom resource definition."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub custom: Vec<DataManifestCustomResourceFunctionDefinition>,
}
impl DataManifestResourceFunctionsDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The data policy manifest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataPolicyManifest {
    #[doc = "The properties of the data policy manifest."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataPolicyManifestProperties>,
    #[doc = "The ID of the data policy manifest."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the data policy manifest (it's the same as the Policy Mode)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource (Microsoft.Authorization/dataPolicyManifests)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl DataPolicyManifest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of data policy manifests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataPolicyManifestListResult {
    #[doc = "An array of data policy manifests."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataPolicyManifest>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataPolicyManifestListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataPolicyManifestListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the data policy manifest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataPolicyManifestProperties {
    #[doc = "The list of namespaces for the data policy manifest."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub namespaces: Vec<String>,
    #[doc = "The policy mode of the data policy manifest."]
    #[serde(rename = "policyMode", default, skip_serializing_if = "Option::is_none")]
    pub policy_mode: Option<String>,
    #[doc = "A value indicating whether policy mode is allowed only in built-in definitions."]
    #[serde(rename = "isBuiltInOnly", default, skip_serializing_if = "Option::is_none")]
    pub is_built_in_only: Option<bool>,
    #[doc = "An array of resource type aliases."]
    #[serde(rename = "resourceTypeAliases", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_type_aliases: Vec<ResourceTypeAliases>,
    #[doc = "The effect definition."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<DataEffect>,
    #[doc = "The non-alias field accessor values that can be used in the policy rule."]
    #[serde(rename = "fieldValues", default, skip_serializing_if = "Vec::is_empty")]
    pub field_values: Vec<String>,
    #[doc = "The resource functions supported by a manifest"]
    #[serde(rename = "resourceFunctions", default, skip_serializing_if = "Option::is_none")]
    pub resource_functions: Option<DataManifestResourceFunctionsDefinition>,
}
impl DataPolicyManifestProperties {
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
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
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
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the resource.  Policy assignments support a maximum of one identity.  That is either a system assigned identity or a single user assigned identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[doc = "The principal ID of the resource identity.  This property will only be provided for a system assigned identity"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the resource identity.  This property will only be provided for a system assigned identity"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The identity type. This is the only required field when adding a system or user assigned identity to a resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
    #[doc = "The user identity associated with the policy. The user identity dictionary key references will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity {
    use super::*;
    #[doc = "The identity type. This is the only required field when adding a system or user assigned identity to a resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        None,
    }
}
#[doc = "A message that describes why a resource is non-compliant with the policy. This is shown in 'deny' error messages and on resource's non-compliant compliance results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NonComplianceMessage {
    #[doc = "A message that describes why a resource is non-compliant with the policy. This is shown in 'deny' error messages and on resource's non-compliant compliance results."]
    pub message: String,
    #[doc = "The policy definition reference ID within a policy set definition the message is intended for. This is only applicable if the policy assignment assigns a policy set definition. If this is not provided the message applies to all policies assigned by this policy assignment."]
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
}
impl NonComplianceMessage {
    pub fn new(message: String) -> Self {
        Self {
            message,
            policy_definition_reference_id: None,
        }
    }
}
#[doc = "The parameter definitions for parameters used in the policy. The keys are the parameter names."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterDefinitions {}
impl ParameterDefinitions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition of a parameter that can be provided to the policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterDefinitionsValue {
    #[doc = "The data type of the parameter."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<parameter_definitions_value::Type>,
    #[doc = "The allowed values for the parameter."]
    #[serde(rename = "allowedValues", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_values: Vec<serde_json::Value>,
    #[doc = "The default value for the parameter if no value is provided."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    #[doc = "General metadata for the parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<parameter_definitions_value::Metadata>,
}
impl ParameterDefinitionsValue {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod parameter_definitions_value {
    use super::*;
    #[doc = "The data type of the parameter."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        String,
        Array,
        Object,
        Boolean,
        Integer,
        Float,
        DateTime,
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
                Self::String => serializer.serialize_unit_variant("Type", 0u32, "String"),
                Self::Array => serializer.serialize_unit_variant("Type", 1u32, "Array"),
                Self::Object => serializer.serialize_unit_variant("Type", 2u32, "Object"),
                Self::Boolean => serializer.serialize_unit_variant("Type", 3u32, "Boolean"),
                Self::Integer => serializer.serialize_unit_variant("Type", 4u32, "Integer"),
                Self::Float => serializer.serialize_unit_variant("Type", 5u32, "Float"),
                Self::DateTime => serializer.serialize_unit_variant("Type", 6u32, "DateTime"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "General metadata for the parameter."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Metadata {
        #[doc = "The display name for the parameter."]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "The description of the parameter."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "Used when assigning the policy definition through the portal. Provides a context aware list of values for the user to choose from."]
        #[serde(rename = "strongType", default, skip_serializing_if = "Option::is_none")]
        pub strong_type: Option<String>,
        #[doc = "Set to true to have Azure portal create role assignments on the resource ID or resource scope value of this parameter during policy assignment. This property is useful in case you wish to assign permissions outside the assignment scope."]
        #[serde(rename = "assignPermissions", default, skip_serializing_if = "Option::is_none")]
        pub assign_permissions: Option<bool>,
    }
    impl Metadata {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The parameter values for the policy rule. The keys are the parameter names."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterValues {}
impl ParameterValues {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The value of a parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterValuesValue {
    #[doc = "The value of the parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
impl ParameterValuesValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The policy assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyAssignment {
    #[doc = "The policy assignment properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyAssignmentProperties>,
    #[doc = "The ID of the policy assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the policy assignment."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the policy assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The location of the policy assignment. Only required when utilizing managed identity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Identity for the resource.  Policy assignments support a maximum of one identity.  That is either a system assigned identity or a single user assigned identity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PolicyAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of policy assignments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyAssignmentListResult {
    #[doc = "An array of policy assignments."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicyAssignment>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PolicyAssignmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PolicyAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The policy assignment properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyAssignmentProperties {
    #[doc = "The display name of the policy assignment."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The ID of the policy definition or policy set definition being assigned."]
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[doc = "The scope for the policy assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The policy's excluded scopes."]
    #[serde(rename = "notScopes", default, skip_serializing_if = "Vec::is_empty")]
    pub not_scopes: Vec<String>,
    #[doc = "The parameter values for the policy rule. The keys are the parameter names."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterValues>,
    #[doc = "This message will be part of response in case of policy violation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The policy assignment metadata. Metadata is an open ended object and is typically a collection of key value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The policy assignment enforcement mode. Possible values are Default and DoNotEnforce."]
    #[serde(rename = "enforcementMode", default, skip_serializing_if = "Option::is_none")]
    pub enforcement_mode: Option<policy_assignment_properties::EnforcementMode>,
    #[doc = "The messages that describe why a resource is non-compliant with the policy."]
    #[serde(rename = "nonComplianceMessages", default, skip_serializing_if = "Vec::is_empty")]
    pub non_compliance_messages: Vec<NonComplianceMessage>,
}
impl PolicyAssignmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod policy_assignment_properties {
    use super::*;
    #[doc = "The policy assignment enforcement mode. Possible values are Default and DoNotEnforce."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnforcementMode")]
    pub enum EnforcementMode {
        Default,
        DoNotEnforce,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnforcementMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnforcementMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnforcementMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("EnforcementMode", 0u32, "Default"),
                Self::DoNotEnforce => serializer.serialize_unit_variant("EnforcementMode", 1u32, "DoNotEnforce"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for EnforcementMode {
        fn default() -> Self {
            Self::Default
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyAssignmentUpdate {
    #[doc = "The location of the policy assignment. Only required when utilizing managed identity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Identity for the resource.  Policy assignments support a maximum of one identity.  That is either a system assigned identity or a single user assigned identity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
impl PolicyAssignmentUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The policy definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDefinition {
    #[doc = "The policy definition properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyDefinitionProperties>,
    #[doc = "The ID of the policy definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the policy definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource (Microsoft.Authorization/policyDefinitions)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PolicyDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The policy definition group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyDefinitionGroup {
    #[doc = "The name of the group."]
    pub name: String,
    #[doc = "The group's display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The group's category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The group's description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A resource ID of a resource that contains additional metadata about the group."]
    #[serde(rename = "additionalMetadataId", default, skip_serializing_if = "Option::is_none")]
    pub additional_metadata_id: Option<String>,
}
impl PolicyDefinitionGroup {
    pub fn new(name: String) -> Self {
        Self {
            name,
            display_name: None,
            category: None,
            description: None,
            additional_metadata_id: None,
        }
    }
}
#[doc = "List of policy definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDefinitionListResult {
    #[doc = "An array of policy definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicyDefinition>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PolicyDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PolicyDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The policy definition properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDefinitionProperties {
    #[doc = "The type of policy definition. Possible values are NotSpecified, BuiltIn, Custom, and Static."]
    #[serde(rename = "policyType", default, skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<policy_definition_properties::PolicyType>,
    #[doc = "The policy definition mode. Some examples are All, Indexed, Microsoft.KeyVault.Data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[doc = "The display name of the policy definition."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The policy definition description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The policy rule."]
    #[serde(rename = "policyRule", default, skip_serializing_if = "Option::is_none")]
    pub policy_rule: Option<serde_json::Value>,
    #[doc = "The policy definition metadata.  Metadata is an open ended object and is typically a collection of key value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The parameter definitions for parameters used in the policy. The keys are the parameter names."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterDefinitions>,
}
impl PolicyDefinitionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod policy_definition_properties {
    use super::*;
    #[doc = "The type of policy definition. Possible values are NotSpecified, BuiltIn, Custom, and Static."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PolicyType")]
    pub enum PolicyType {
        NotSpecified,
        BuiltIn,
        Custom,
        Static,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PolicyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PolicyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PolicyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("PolicyType", 0u32, "NotSpecified"),
                Self::BuiltIn => serializer.serialize_unit_variant("PolicyType", 1u32, "BuiltIn"),
                Self::Custom => serializer.serialize_unit_variant("PolicyType", 2u32, "Custom"),
                Self::Static => serializer.serialize_unit_variant("PolicyType", 3u32, "Static"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The policy definition reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyDefinitionReference {
    #[doc = "The ID of the policy definition or policy set definition."]
    #[serde(rename = "policyDefinitionId")]
    pub policy_definition_id: String,
    #[doc = "The parameter values for the policy rule. The keys are the parameter names."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterValues>,
    #[doc = "A unique id (within the policy set definition) for this policy definition reference."]
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[doc = "The name of the groups that this policy definition reference belongs to."]
    #[serde(rename = "groupNames", default, skip_serializing_if = "Vec::is_empty")]
    pub group_names: Vec<String>,
}
impl PolicyDefinitionReference {
    pub fn new(policy_definition_id: String) -> Self {
        Self {
            policy_definition_id,
            parameters: None,
            policy_definition_reference_id: None,
            group_names: Vec::new(),
        }
    }
}
#[doc = "The policy exemption."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyExemption {
    #[doc = "The policy exemption properties."]
    pub properties: PolicyExemptionProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The ID of the policy exemption."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the policy exemption."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource (Microsoft.Authorization/policyExemptions)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl PolicyExemption {
    pub fn new(properties: PolicyExemptionProperties) -> Self {
        Self {
            properties,
            system_data: None,
            id: None,
            name: None,
            type_: None,
        }
    }
}
#[doc = "List of policy exemptions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyExemptionListResult {
    #[doc = "An array of policy exemptions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicyExemption>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PolicyExemptionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PolicyExemptionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The policy exemption properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyExemptionProperties {
    #[doc = "The ID of the policy assignment that is being exempted."]
    #[serde(rename = "policyAssignmentId")]
    pub policy_assignment_id: String,
    #[doc = "The policy definition reference ID list when the associated policy assignment is an assignment of a policy set definition."]
    #[serde(rename = "policyDefinitionReferenceIds", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_definition_reference_ids: Vec<String>,
    #[doc = "The policy exemption category. Possible values are Waiver and Mitigated."]
    #[serde(rename = "exemptionCategory")]
    pub exemption_category: policy_exemption_properties::ExemptionCategory,
    #[doc = "The expiration date and time (in UTC ISO 8601 format yyyy-MM-ddTHH:mm:ssZ) of the policy exemption."]
    #[serde(rename = "expiresOn", with = "azure_core::date::rfc3339::option")]
    pub expires_on: Option<time::OffsetDateTime>,
    #[doc = "The display name of the policy exemption."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description of the policy exemption."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The policy exemption metadata. Metadata is an open ended object and is typically a collection of key value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
impl PolicyExemptionProperties {
    pub fn new(policy_assignment_id: String, exemption_category: policy_exemption_properties::ExemptionCategory) -> Self {
        Self {
            policy_assignment_id,
            policy_definition_reference_ids: Vec::new(),
            exemption_category,
            expires_on: None,
            display_name: None,
            description: None,
            metadata: None,
        }
    }
}
pub mod policy_exemption_properties {
    use super::*;
    #[doc = "The policy exemption category. Possible values are Waiver and Mitigated."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ExemptionCategory")]
    pub enum ExemptionCategory {
        Waiver,
        Mitigated,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ExemptionCategory {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ExemptionCategory {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ExemptionCategory {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Waiver => serializer.serialize_unit_variant("ExemptionCategory", 0u32, "Waiver"),
                Self::Mitigated => serializer.serialize_unit_variant("ExemptionCategory", 1u32, "Mitigated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The policy set definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicySetDefinition {
    #[doc = "The policy set definition properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicySetDefinitionProperties>,
    #[doc = "The ID of the policy set definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the policy set definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource (Microsoft.Authorization/policySetDefinitions)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PolicySetDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of policy set definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicySetDefinitionListResult {
    #[doc = "An array of policy set definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicySetDefinition>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PolicySetDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PolicySetDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The policy set definition properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicySetDefinitionProperties {
    #[doc = "The type of policy definition. Possible values are NotSpecified, BuiltIn, Custom, and Static."]
    #[serde(rename = "policyType", default, skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<policy_set_definition_properties::PolicyType>,
    #[doc = "The display name of the policy set definition."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The policy set definition description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The policy set definition metadata.  Metadata is an open ended object and is typically a collection of key value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The parameter definitions for parameters used in the policy. The keys are the parameter names."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterDefinitions>,
    #[doc = "An array of policy definition references."]
    #[serde(rename = "policyDefinitions")]
    pub policy_definitions: Vec<PolicyDefinitionReference>,
    #[doc = "The metadata describing groups of policy definition references within the policy set definition."]
    #[serde(rename = "policyDefinitionGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_definition_groups: Vec<PolicyDefinitionGroup>,
}
impl PolicySetDefinitionProperties {
    pub fn new(policy_definitions: Vec<PolicyDefinitionReference>) -> Self {
        Self {
            policy_type: None,
            display_name: None,
            description: None,
            metadata: None,
            parameters: None,
            policy_definitions,
            policy_definition_groups: Vec::new(),
        }
    }
}
pub mod policy_set_definition_properties {
    use super::*;
    #[doc = "The type of policy definition. Possible values are NotSpecified, BuiltIn, Custom, and Static."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PolicyType")]
    pub enum PolicyType {
        NotSpecified,
        BuiltIn,
        Custom,
        Static,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PolicyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PolicyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PolicyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("PolicyType", 0u32, "NotSpecified"),
                Self::BuiltIn => serializer.serialize_unit_variant("PolicyType", 1u32, "BuiltIn"),
                Self::Custom => serializer.serialize_unit_variant("PolicyType", 2u32, "Custom"),
                Self::Static => serializer.serialize_unit_variant("PolicyType", 3u32, "Static"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The resource type aliases definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTypeAliases {
    #[doc = "The resource type name."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The aliases for property names."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<Alias>,
}
impl ResourceTypeAliases {
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
