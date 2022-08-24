#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The authentication info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthInfoBase {
    #[doc = "The authentication type."]
    #[serde(rename = "authType")]
    pub auth_type: AuthType,
}
impl AuthInfoBase {
    pub fn new(auth_type: AuthType) -> Self {
        Self { auth_type }
    }
}
#[doc = "The authentication type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AuthType")]
pub enum AuthType {
    #[serde(rename = "systemAssignedIdentity")]
    SystemAssignedIdentity,
    #[serde(rename = "userAssignedIdentity")]
    UserAssignedIdentity,
    #[serde(rename = "servicePrincipalSecret")]
    ServicePrincipalSecret,
    #[serde(rename = "servicePrincipalCertificate")]
    ServicePrincipalCertificate,
    #[serde(rename = "secret")]
    Secret,
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
            Self::SystemAssignedIdentity => serializer.serialize_unit_variant("AuthType", 0u32, "systemAssignedIdentity"),
            Self::UserAssignedIdentity => serializer.serialize_unit_variant("AuthType", 1u32, "userAssignedIdentity"),
            Self::ServicePrincipalSecret => serializer.serialize_unit_variant("AuthType", 2u32, "servicePrincipalSecret"),
            Self::ServicePrincipalCertificate => serializer.serialize_unit_variant("AuthType", 3u32, "servicePrincipalCertificate"),
            Self::Secret => serializer.serialize_unit_variant("AuthType", 4u32, "secret"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource properties when type is Azure Key Vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureKeyVaultProperties {
    #[serde(flatten)]
    pub azure_resource_properties_base: AzureResourcePropertiesBase,
    #[doc = "True if connect via Kubernetes CSI Driver."]
    #[serde(rename = "connectAsKubernetesCsiDriver", default, skip_serializing_if = "Option::is_none")]
    pub connect_as_kubernetes_csi_driver: Option<bool>,
}
impl AzureKeyVaultProperties {
    pub fn new(azure_resource_properties_base: AzureResourcePropertiesBase) -> Self {
        Self {
            azure_resource_properties_base,
            connect_as_kubernetes_csi_driver: None,
        }
    }
}
#[doc = "The azure resource info when target service type is AzureResource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResource {
    #[serde(flatten)]
    pub target_service_base: TargetServiceBase,
    #[doc = "The Id of azure resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The azure resource properties"]
    #[serde(rename = "resourceProperties", default, skip_serializing_if = "Option::is_none")]
    pub resource_properties: Option<AzureResourcePropertiesBase>,
}
impl AzureResource {
    pub fn new(target_service_base: TargetServiceBase) -> Self {
        Self {
            target_service_base,
            id: None,
            resource_properties: None,
        }
    }
}
#[doc = "The azure resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourcePropertiesBase {
    #[doc = "The azure resource type."]
    #[serde(rename = "type")]
    pub type_: AzureResourceType,
}
impl AzureResourcePropertiesBase {
    pub fn new(type_: AzureResourceType) -> Self {
        Self { type_ }
    }
}
#[doc = "The azure resource type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureResourceType")]
pub enum AzureResourceType {
    KeyVault,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureResourceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureResourceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureResourceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::KeyVault => serializer.serialize_unit_variant("AzureResourceType", 0u32, "KeyVault"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The service properties when target service type is ConfluentBootstrapServer"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfluentBootstrapServer {
    #[serde(flatten)]
    pub target_service_base: TargetServiceBase,
    #[doc = "The endpoint of service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl ConfluentBootstrapServer {
    pub fn new(target_service_base: TargetServiceBase) -> Self {
        Self {
            target_service_base,
            endpoint: None,
        }
    }
}
#[doc = "The service properties when target service type is ConfluentSchemaRegistry"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfluentSchemaRegistry {
    #[serde(flatten)]
    pub target_service_base: TargetServiceBase,
    #[doc = "The endpoint of service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl ConfluentSchemaRegistry {
    pub fn new(target_service_base: TargetServiceBase) -> Self {
        Self {
            target_service_base,
            endpoint: None,
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
#[doc = "The secret info when type is keyVaultSecretReference. It's for scenario that user provides a secret stored in user's keyvault and source is Azure Kubernetes. The key Vault's resource id is linked to secretStore.keyVaultId."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultSecretReferenceSecretInfo {
    #[serde(flatten)]
    pub secret_info_base: SecretInfoBase,
    #[doc = "Name of the Key Vault secret."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Version of the Key Vault secret."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl KeyVaultSecretReferenceSecretInfo {
    pub fn new(secret_info_base: SecretInfoBase) -> Self {
        Self {
            secret_info_base,
            name: None,
            version: None,
        }
    }
}
#[doc = "The secret info when type is keyVaultSecretUri. It's for scenario that user provides a secret stored in user's keyvault and source is Web App, Spring Cloud or Container App."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultSecretUriSecretInfo {
    #[serde(flatten)]
    pub secret_info_base: SecretInfoBase,
    #[doc = "URI to the keyvault secret"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl KeyVaultSecretUriSecretInfo {
    pub fn new(secret_info_base: SecretInfoBase) -> Self {
        Self {
            secret_info_base,
            value: None,
        }
    }
}
#[doc = "The list of Linker."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkerList {
    #[doc = "The link used to get the next page of Linker list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of Linkers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LinkerResource>,
}
impl azure_core::Continuable for LinkerList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl LinkerList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A linker to be updated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkerPatch {
    #[doc = "The properties of the linker."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LinkerProperties>,
}
impl LinkerPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the linker."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkerProperties {
    #[doc = "The target service properties"]
    #[serde(rename = "targetService", default, skip_serializing_if = "Option::is_none")]
    pub target_service: Option<TargetServiceBase>,
    #[doc = "The authentication info"]
    #[serde(rename = "authInfo", default, skip_serializing_if = "Option::is_none")]
    pub auth_info: Option<AuthInfoBase>,
    #[doc = "The application client type"]
    #[serde(rename = "clientType", default, skip_serializing_if = "Option::is_none")]
    pub client_type: Option<linker_properties::ClientType>,
    #[doc = "The provisioning state. "]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The VNet solution for linker"]
    #[serde(rename = "vNetSolution", default, skip_serializing_if = "Option::is_none")]
    pub v_net_solution: Option<VNetSolution>,
    #[doc = "An option to store secret value in secure place"]
    #[serde(rename = "secretStore", default, skip_serializing_if = "Option::is_none")]
    pub secret_store: Option<SecretStore>,
    #[doc = "connection scope in source service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
impl LinkerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod linker_properties {
    use super::*;
    #[doc = "The application client type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ClientType")]
    pub enum ClientType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "dotnet")]
        Dotnet,
        #[serde(rename = "java")]
        Java,
        #[serde(rename = "python")]
        Python,
        #[serde(rename = "go")]
        Go,
        #[serde(rename = "php")]
        Php,
        #[serde(rename = "ruby")]
        Ruby,
        #[serde(rename = "django")]
        Django,
        #[serde(rename = "nodejs")]
        Nodejs,
        #[serde(rename = "springBoot")]
        SpringBoot,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ClientType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ClientType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ClientType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ClientType", 0u32, "none"),
                Self::Dotnet => serializer.serialize_unit_variant("ClientType", 1u32, "dotnet"),
                Self::Java => serializer.serialize_unit_variant("ClientType", 2u32, "java"),
                Self::Python => serializer.serialize_unit_variant("ClientType", 3u32, "python"),
                Self::Go => serializer.serialize_unit_variant("ClientType", 4u32, "go"),
                Self::Php => serializer.serialize_unit_variant("ClientType", 5u32, "php"),
                Self::Ruby => serializer.serialize_unit_variant("ClientType", 6u32, "ruby"),
                Self::Django => serializer.serialize_unit_variant("ClientType", 7u32, "django"),
                Self::Nodejs => serializer.serialize_unit_variant("ClientType", 8u32, "nodejs"),
                Self::SpringBoot => serializer.serialize_unit_variant("ClientType", 9u32, "springBoot"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Linker of source and target resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkerResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of the linker."]
    pub properties: LinkerProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl LinkerResource {
    pub fn new(properties: LinkerProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "Details of a REST API operation, returned from the Resource Provider Operations API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation, as per Resource-Based Access Control (RBAC). Examples: \"Microsoft.Compute/virtualMachines/write\", \"Microsoft.Compute/virtualMachines/capture/action\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether the operation applies to data-plane. This is \"true\" for data-plane operations and \"false\" for ARM/control-plane operations."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Localized display information for this particular operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Localized display information for this particular operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The localized friendly form of the resource provider name, e.g. \"Microsoft Monitoring Insights\" or \"Microsoft Compute\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The localized friendly name of the resource type related to this operation. E.g. \"Virtual Machines\" or \"Job Schedule Collections\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The concise, localized friendly name for the operation; suitable for dropdowns. E.g. \"Create or Update Virtual Machine\", \"Restart Virtual Machine\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The short, localized friendly description of the operation; suitable for tool tips and detailed views."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Origin {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Origin {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Origin {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("Origin", 0u32, "user"),
                Self::System => serializer.serialize_unit_variant("Origin", 1u32, "system"),
                Self::UserSystem => serializer.serialize_unit_variant("Origin", 2u32, "user,system"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionType")]
    pub enum ActionType {
        Internal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Internal => serializer.serialize_unit_variant("ActionType", 0u32, "Internal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of REST API operations supported by an Azure Resource Provider. It contains an URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the resource provider"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
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
#[doc = "The authentication info when authType is secret"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretAuthInfo {
    #[serde(flatten)]
    pub auth_info_base: AuthInfoBase,
    #[doc = "Username or account name for secret auth."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The secret info"]
    #[serde(rename = "secretInfo", default, skip_serializing_if = "Option::is_none")]
    pub secret_info: Option<SecretInfoBase>,
}
impl SecretAuthInfo {
    pub fn new(auth_info_base: AuthInfoBase) -> Self {
        Self {
            auth_info_base,
            name: None,
            secret_info: None,
        }
    }
}
#[doc = "The secret info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretInfoBase {
    #[doc = "The secret type."]
    #[serde(rename = "secretType")]
    pub secret_type: SecretType,
}
impl SecretInfoBase {
    pub fn new(secret_type: SecretType) -> Self {
        Self { secret_type }
    }
}
#[doc = "An option to store secret value in secure place"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretStore {
    #[doc = "The key vault id to store secret"]
    #[serde(rename = "keyVaultId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_id: Option<String>,
}
impl SecretStore {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The secret type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SecretType")]
pub enum SecretType {
    #[serde(rename = "rawValue")]
    RawValue,
    #[serde(rename = "keyVaultSecretUri")]
    KeyVaultSecretUri,
    #[serde(rename = "keyVaultSecretReference")]
    KeyVaultSecretReference,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SecretType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SecretType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SecretType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::RawValue => serializer.serialize_unit_variant("SecretType", 0u32, "rawValue"),
            Self::KeyVaultSecretUri => serializer.serialize_unit_variant("SecretType", 1u32, "keyVaultSecretUri"),
            Self::KeyVaultSecretReference => serializer.serialize_unit_variant("SecretType", 2u32, "keyVaultSecretReference"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The authentication info when authType is servicePrincipal certificate"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipalCertificateAuthInfo {
    #[serde(flatten)]
    pub auth_info_base: AuthInfoBase,
    #[doc = "Application clientId for servicePrincipal auth."]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "Principal Id for servicePrincipal auth."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "ServicePrincipal certificate for servicePrincipal auth."]
    pub certificate: String,
}
impl ServicePrincipalCertificateAuthInfo {
    pub fn new(auth_info_base: AuthInfoBase, client_id: String, principal_id: String, certificate: String) -> Self {
        Self {
            auth_info_base,
            client_id,
            principal_id,
            certificate,
        }
    }
}
#[doc = "The authentication info when authType is servicePrincipal secret"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipalSecretAuthInfo {
    #[serde(flatten)]
    pub auth_info_base: AuthInfoBase,
    #[doc = "ServicePrincipal application clientId for servicePrincipal auth."]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "Principal Id for servicePrincipal auth."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "Secret for servicePrincipal auth."]
    pub secret: String,
}
impl ServicePrincipalSecretAuthInfo {
    pub fn new(auth_info_base: AuthInfoBase, client_id: String, principal_id: String, secret: String) -> Self {
        Self {
            auth_info_base,
            client_id,
            principal_id,
            secret,
        }
    }
}
#[doc = "A configuration item for source resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceConfiguration {
    #[doc = "The name of setting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of setting"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SourceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configurations for source resource, include appSettings, connectionString and serviceBindings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceConfigurationResult {
    #[doc = "The configuration properties for source resource."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub configurations: Vec<SourceConfiguration>,
}
impl SourceConfigurationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The authentication info when authType is systemAssignedIdentity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemAssignedIdentityAuthInfo {
    #[serde(flatten)]
    pub auth_info_base: AuthInfoBase,
}
impl SystemAssignedIdentityAuthInfo {
    pub fn new(auth_info_base: AuthInfoBase) -> Self {
        Self { auth_info_base }
    }
}
#[doc = "The target service properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetServiceBase {
    #[doc = "The target service type."]
    #[serde(rename = "type")]
    pub type_: TargetServiceType,
}
impl TargetServiceBase {
    pub fn new(type_: TargetServiceType) -> Self {
        Self { type_ }
    }
}
#[doc = "The target service type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TargetServiceType")]
pub enum TargetServiceType {
    AzureResource,
    ConfluentBootstrapServer,
    ConfluentSchemaRegistry,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TargetServiceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TargetServiceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TargetServiceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AzureResource => serializer.serialize_unit_variant("TargetServiceType", 0u32, "AzureResource"),
            Self::ConfluentBootstrapServer => serializer.serialize_unit_variant("TargetServiceType", 1u32, "ConfluentBootstrapServer"),
            Self::ConfluentSchemaRegistry => serializer.serialize_unit_variant("TargetServiceType", 2u32, "ConfluentSchemaRegistry"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The authentication info when authType is userAssignedIdentity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAssignedIdentityAuthInfo {
    #[serde(flatten)]
    pub auth_info_base: AuthInfoBase,
    #[doc = "Client Id for userAssignedIdentity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "Subscription id for userAssignedIdentity."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}
impl UserAssignedIdentityAuthInfo {
    pub fn new(auth_info_base: AuthInfoBase) -> Self {
        Self {
            auth_info_base,
            client_id: None,
            subscription_id: None,
        }
    }
}
#[doc = "The VNet solution for linker"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VNetSolution {
    #[doc = "Type of VNet solution."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<v_net_solution::Type>,
}
impl VNetSolution {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod v_net_solution {
    use super::*;
    #[doc = "Type of VNet solution."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "serviceEndpoint")]
        ServiceEndpoint,
        #[serde(rename = "privateLink")]
        PrivateLink,
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
                Self::ServiceEndpoint => serializer.serialize_unit_variant("Type", 0u32, "serviceEndpoint"),
                Self::PrivateLink => serializer.serialize_unit_variant("Type", 1u32, "privateLink"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The validation result for a linker."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateResult {
    #[doc = "The linker name."]
    #[serde(rename = "linkerName", default, skip_serializing_if = "Option::is_none")]
    pub linker_name: Option<String>,
    #[doc = "A boolean value indicating whether the connection is available or not"]
    #[serde(rename = "isConnectionAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_connection_available: Option<bool>,
    #[doc = "The start time of the validation report."]
    #[serde(rename = "reportStartTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub report_start_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The end time of the validation report."]
    #[serde(rename = "reportEndTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub report_end_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The resource id of the linker source application."]
    #[serde(rename = "sourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[doc = "The resource Id of target service."]
    #[serde(rename = "targetId", default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[doc = "The authentication type."]
    #[serde(rename = "authType", default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<AuthType>,
    #[doc = "The detail of validation result"]
    #[serde(rename = "validationDetail", default, skip_serializing_if = "Vec::is_empty")]
    pub validation_detail: Vec<ValidationResultItem>,
}
impl ValidateResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The validation item for a linker."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidationResultItem {
    #[doc = "The validation item name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The display name of validation item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The result of validation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<validation_result_item::Result>,
    #[doc = "The error message of validation result"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The error code of validation result"]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}
impl ValidationResultItem {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod validation_result_item {
    use super::*;
    #[doc = "The result of validation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Result")]
    pub enum Result {
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "warning")]
        Warning,
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
                Self::Success => serializer.serialize_unit_variant("Result", 0u32, "success"),
                Self::Failed => serializer.serialize_unit_variant("Result", 1u32, "failed"),
                Self::Warning => serializer.serialize_unit_variant("Result", 2u32, "warning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The secret info when type is rawValue. It's for scenarios that user input the secret."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueSecretInfo {
    #[serde(flatten)]
    pub secret_info_base: SecretInfoBase,
    #[doc = "The actual value of the secret."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ValueSecretInfo {
    pub fn new(secret_info_base: SecretInfoBase) -> Self {
        Self {
            secret_info_base,
            value: None,
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
