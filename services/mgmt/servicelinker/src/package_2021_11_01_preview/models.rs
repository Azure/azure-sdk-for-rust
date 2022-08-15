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
    #[doc = "The resource Id of target service."]
    #[serde(rename = "targetId", default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
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
    #[doc = "Password or account key for secret auth."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}
impl SecretAuthInfo {
    pub fn new(auth_info_base: AuthInfoBase) -> Self {
        Self {
            auth_info_base,
            name: None,
            secret: None,
        }
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
#[doc = "The authentication info when authType is userAssignedIdentity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAssignedIdentityAuthInfo {
    #[serde(flatten)]
    pub auth_info_base: AuthInfoBase,
    #[doc = "Client Id for userAssignedIdentity."]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "Subscription id for userAssignedIdentity."]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}
impl UserAssignedIdentityAuthInfo {
    pub fn new(auth_info_base: AuthInfoBase, client_id: String, subscription_id: String) -> Self {
        Self {
            auth_info_base,
            client_id,
            subscription_id,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies if the linker is healthy."]
    #[serde(rename = "linkerStatus", default, skip_serializing_if = "Option::is_none")]
    pub linker_status: Option<validate_result::LinkerStatus>,
    #[doc = "The reason of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The start time of the validation report."]
    #[serde(rename = "reportStartTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub report_start_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The end time of the validation report."]
    #[serde(rename = "reportEndTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub report_end_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The resource Id of target service."]
    #[serde(rename = "targetId", default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[doc = "The authentication type."]
    #[serde(rename = "authType", default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<AuthType>,
}
impl ValidateResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod validate_result {
    use super::*;
    #[doc = "Specifies if the linker is healthy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LinkerStatus")]
    pub enum LinkerStatus {
        Healthy,
        #[serde(rename = "Not healthy")]
        NotHealthy,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LinkerStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LinkerStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LinkerStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Healthy => serializer.serialize_unit_variant("LinkerStatus", 0u32, "Healthy"),
                Self::NotHealthy => serializer.serialize_unit_variant("LinkerStatus", 1u32, "Not healthy"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
