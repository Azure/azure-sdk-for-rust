#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The access key directly from target resource properties, which target service is Azure Resource, such as Microsoft.Storage"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessKeyInfoBase {
    #[doc = "Permissions of the accessKey. `Read` and `Write` are for Azure Cosmos DB and Azure App Configuration, `Listen`, `Send` and `Manage` are for Azure Event Hub and Azure Service Bus."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub permissions: Vec<String>,
}
impl AccessKeyInfoBase {
    pub fn new() -> Self {
        Self { permissions: Vec::new() }
    }
}
#[doc = "Indicates how to apply the connector operations, such as opt out network configuration, opt in configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ActionType")]
pub enum ActionType {
    #[serde(rename = "enable")]
    Enable,
    #[serde(rename = "optOut")]
    OptOut,
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
            Self::Enable => serializer.serialize_unit_variant("ActionType", 0u32, "enable"),
            Self::OptOut => serializer.serialize_unit_variant("ActionType", 1u32, "optOut"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Whether to allow firewall rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AllowType")]
pub enum AllowType {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AllowType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AllowType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AllowType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::True => serializer.serialize_unit_variant("AllowType", 0u32, "true"),
            Self::False => serializer.serialize_unit_variant("AllowType", 1u32, "false"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The authentication type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "authType")]
pub enum AuthInfoBaseUnion {
    #[serde(rename = "accessKey")]
    AccessKey(AccessKeyInfoBase),
    #[serde(rename = "secret")]
    Secret(SecretAuthInfo),
    #[serde(rename = "servicePrincipalCertificate")]
    ServicePrincipalCertificate(ServicePrincipalCertificateAuthInfo),
    #[serde(rename = "servicePrincipalSecret")]
    ServicePrincipalSecret(ServicePrincipalSecretAuthInfo),
    #[serde(rename = "systemAssignedIdentity")]
    SystemAssignedIdentity(SystemAssignedIdentityAuthInfo),
    #[serde(rename = "userAccount")]
    UserAccount(UserAccountAuthInfo),
    #[serde(rename = "userAssignedIdentity")]
    UserAssignedIdentity(UserAssignedIdentityAuthInfo),
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
    #[serde(rename = "accessKey")]
    AccessKey,
    #[serde(rename = "userAccount")]
    UserAccount,
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
            Self::AccessKey => serializer.serialize_unit_variant("AuthType", 5u32, "accessKey"),
            Self::UserAccount => serializer.serialize_unit_variant("AuthType", 6u32, "userAccount"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource properties when type is Azure Key Vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureKeyVaultProperties {
    #[doc = "True if connect via Kubernetes CSI Driver."]
    #[serde(rename = "connectAsKubernetesCsiDriver", default, skip_serializing_if = "Option::is_none")]
    pub connect_as_kubernetes_csi_driver: Option<bool>,
}
impl AzureKeyVaultProperties {
    pub fn new() -> Self {
        Self {
            connect_as_kubernetes_csi_driver: None,
        }
    }
}
#[doc = "The azure resource info when target service type is AzureResource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResource {
    #[doc = "The Id of azure resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The azure resource properties"]
    #[serde(rename = "resourceProperties", default, skip_serializing_if = "Option::is_none")]
    pub resource_properties: Option<AzureResourcePropertiesBaseUnion>,
}
impl AzureResource {
    pub fn new() -> Self {
        Self {
            id: None,
            resource_properties: None,
        }
    }
}
#[doc = "The azure resource type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AzureResourcePropertiesBaseUnion {
    KeyVault(AzureKeyVaultProperties),
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
#[doc = "The represent of basic error"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BasicErrorDryrunPrerequisiteResult {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl BasicErrorDryrunPrerequisiteResult {
    pub fn new() -> Self {
        Self { code: None, message: None }
    }
}
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
    #[serde(rename = "kafka-springBoot")]
    KafkaSpringBoot,
    #[serde(rename = "dapr")]
    Dapr,
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
            Self::KafkaSpringBoot => serializer.serialize_unit_variant("ClientType", 10u32, "kafka-springBoot"),
            Self::Dapr => serializer.serialize_unit_variant("ClientType", 11u32, "dapr"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The configuration information, used to generate configurations or save to applications"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationInfo {
    #[doc = "The cleanup behavior to indicate whether clean up operation when resource is deleted or updated"]
    #[serde(rename = "deleteOrUpdateBehavior", default, skip_serializing_if = "Option::is_none")]
    pub delete_or_update_behavior: Option<DeleteOrUpdateBehavior>,
    #[doc = "Indicates how to apply the connector operations, such as opt out network configuration, opt in configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionType>,
    #[doc = "Optional. A dictionary of default key name and customized key name mapping. If not specified, default key name will be used for generate configurations"]
    #[serde(rename = "customizedKeys", default, skip_serializing_if = "Option::is_none")]
    pub customized_keys: Option<serde_json::Value>,
    #[doc = "Indicates some additional properties for dapr client type"]
    #[serde(rename = "daprProperties", default, skip_serializing_if = "Option::is_none")]
    pub dapr_properties: Option<DaprProperties>,
    #[doc = "A dictionary of additional configurations to be added. Service will auto generate a set of basic configurations and this property is to full fill more customized configurations"]
    #[serde(rename = "additionalConfigurations", default, skip_serializing_if = "Option::is_none")]
    pub additional_configurations: Option<serde_json::Value>,
}
impl ConfigurationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration names."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Description for the configuration name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Represent the configuration is required or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}
impl ConfigurationName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationNameItem {
    #[doc = "The configuration names which will be set based on specific target resource, client type, auth type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationNames>,
}
impl ConfigurationNameItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration Name list which will be set based on different target resource, client type, auth type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationNameResult {
    #[doc = "Expected configuration names for each target service."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ConfigurationNameItem>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConfigurationNameResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ConfigurationNameResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration names which will be set based on specific target resource, client type, auth type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationNames {
    #[doc = "The target service provider name and resource name."]
    #[serde(rename = "targetService", default, skip_serializing_if = "Option::is_none")]
    pub target_service: Option<String>,
    #[doc = "The application client type"]
    #[serde(rename = "clientType", default, skip_serializing_if = "Option::is_none")]
    pub client_type: Option<ClientType>,
    #[doc = "The authentication type."]
    #[serde(rename = "authType", default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<AuthType>,
    #[doc = "Indicates some additional properties for dapr client type"]
    #[serde(rename = "daprProperties", default, skip_serializing_if = "Option::is_none")]
    pub dapr_properties: Option<DaprProperties>,
    #[doc = "The configuration names to be set in compute service environment."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub names: Vec<ConfigurationName>,
}
impl ConfigurationNames {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configurations for source resource, include appSettings, connectionString and serviceBindings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationResult {
    #[doc = "The configuration properties for source resource."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub configurations: Vec<SourceConfiguration>,
}
impl ConfigurationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The service properties when target service type is ConfluentBootstrapServer"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfluentBootstrapServer {
    #[doc = "The endpoint of service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl ConfluentBootstrapServer {
    pub fn new() -> Self {
        Self { endpoint: None }
    }
}
#[doc = "The service properties when target service type is ConfluentSchemaRegistry"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfluentSchemaRegistry {
    #[doc = "The endpoint of service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl ConfluentSchemaRegistry {
    pub fn new() -> Self {
        Self { endpoint: None }
    }
}
#[doc = "The dryrun parameters for creation or update a linker"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateDryrunParameters {
    #[serde(flatten)]
    pub linker_properties: LinkerProperties,
}
impl CreateOrUpdateDryrunParameters {
    pub fn new() -> Self {
        Self {
            linker_properties: LinkerProperties::default(),
        }
    }
}
#[doc = "The dapr component metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DaprMetadata {
    #[doc = "Metadata property name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Metadata property value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The secret name where dapr could get value"]
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<String>,
}
impl DaprMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates some additional properties for dapr client type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DaprProperties {
    #[doc = "The dapr component version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The dapr component type"]
    #[serde(rename = "componentType", default, skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    #[doc = "The name of a secret store dapr to retrieve secret"]
    #[serde(rename = "secretStoreComponent", default, skip_serializing_if = "Option::is_none")]
    pub secret_store_component: Option<String>,
    #[doc = "Additional dapr metadata"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metadata: Vec<DaprMetadata>,
    #[doc = "The dapr component scopes"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scopes: Vec<String>,
}
impl DaprProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The extra auth info required by Database AAD authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseAadAuthInfo {
    #[doc = "Username created in the database which is mapped to a user in AAD."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}
impl DatabaseAadAuthInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The cleanup behavior to indicate whether clean up operation when resource is deleted or updated"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeleteOrUpdateBehavior")]
pub enum DeleteOrUpdateBehavior {
    Default,
    ForcedCleanup,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeleteOrUpdateBehavior {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeleteOrUpdateBehavior {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeleteOrUpdateBehavior {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Default => serializer.serialize_unit_variant("DeleteOrUpdateBehavior", 0u32, "Default"),
            Self::ForcedCleanup => serializer.serialize_unit_variant("DeleteOrUpdateBehavior", 1u32, "ForcedCleanup"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The name of action for you dryrun job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DryrunActionName")]
pub enum DryrunActionName {
    #[serde(rename = "createOrUpdate")]
    CreateOrUpdate,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DryrunActionName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DryrunActionName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DryrunActionName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::CreateOrUpdate => serializer.serialize_unit_variant("DryrunActionName", 0u32, "createOrUpdate"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The list of dryrun."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DryrunList {
    #[doc = "The link used to get the next page of dryrun list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of dryrun."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DryrunResource>,
}
impl azure_core::Continuable for DryrunList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DryrunList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The preview of the operations for creation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DryrunOperationPreview {
    #[doc = "The operation name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The operation type"]
    #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<dryrun_operation_preview::OperationType>,
    #[doc = "The description of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The action defined by RBAC, refer https://docs.microsoft.com/azure/role-based-access-control/role-definitions#actions-format"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "The scope of the operation, refer https://docs.microsoft.com/azure/role-based-access-control/scope-overview"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
impl DryrunOperationPreview {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod dryrun_operation_preview {
    use super::*;
    #[doc = "The operation type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OperationType")]
    pub enum OperationType {
        #[serde(rename = "configConnection")]
        ConfigConnection,
        #[serde(rename = "configNetwork")]
        ConfigNetwork,
        #[serde(rename = "configAuth")]
        ConfigAuth,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OperationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OperationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OperationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ConfigConnection => serializer.serialize_unit_variant("OperationType", 0u32, "configConnection"),
                Self::ConfigNetwork => serializer.serialize_unit_variant("OperationType", 1u32, "configNetwork"),
                Self::ConfigAuth => serializer.serialize_unit_variant("OperationType", 2u32, "configAuth"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The name of action for you dryrun job."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "actionName")]
pub enum DryrunParametersUnion {
    #[serde(rename = "createOrUpdate")]
    CreateOrUpdate(CreateOrUpdateDryrunParameters),
}
#[doc = "a dryrun job to be updated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DryrunPatch {
    #[doc = "The properties of the dryrun job"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DryrunProperties>,
}
impl DryrunPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of dryrun result."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DryrunPrerequisiteResultUnion {
    #[serde(rename = "basicError")]
    BasicError(BasicErrorDryrunPrerequisiteResult),
    #[serde(rename = "permissionsMissing")]
    PermissionsMissing(PermissionsMissingDryrunPrerequisiteResult),
}
#[doc = "The type of dryrun result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DryrunPrerequisiteResultType")]
pub enum DryrunPrerequisiteResultType {
    #[serde(rename = "basicError")]
    BasicError,
    #[serde(rename = "permissionsMissing")]
    PermissionsMissing,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DryrunPrerequisiteResultType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DryrunPrerequisiteResultType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DryrunPrerequisiteResultType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::BasicError => serializer.serialize_unit_variant("DryrunPrerequisiteResultType", 0u32, "basicError"),
            Self::PermissionsMissing => serializer.serialize_unit_variant("DryrunPrerequisiteResultType", 1u32, "permissionsMissing"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The properties of the dryrun job"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DryrunProperties {
    #[doc = "The parameters of the dryrun"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<DryrunParametersUnion>,
    #[doc = "the result of the dryrun"]
    #[serde(
        rename = "prerequisiteResults",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub prerequisite_results: Vec<DryrunPrerequisiteResultUnion>,
    #[doc = "the preview of the operations for creation"]
    #[serde(
        rename = "operationPreviews",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operation_previews: Vec<DryrunOperationPreview>,
    #[doc = "The provisioning state. "]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl DryrunProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "a dryrun job resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DryrunResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of the dryrun job"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DryrunProperties>,
}
impl DryrunResource {
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
#[doc = "Target service's firewall rules. to allow connections from source service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallRules {
    #[doc = "This value specifies the set of IP addresses or IP address ranges in CIDR form to be included as the allowed list of client IPs for a given database account."]
    #[serde(
        rename = "ipRanges",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_ranges: Vec<String>,
    #[doc = "Whether to allow firewall rules."]
    #[serde(rename = "azureServices", default, skip_serializing_if = "Option::is_none")]
    pub azure_services: Option<AllowType>,
    #[doc = "Whether to allow firewall rules."]
    #[serde(rename = "callerClientIP", default, skip_serializing_if = "Option::is_none")]
    pub caller_client_ip: Option<AllowType>,
}
impl FirewallRules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The secret info when type is keyVaultSecretReference. It's for scenario that user provides a secret stored in user's keyvault and source is Azure Kubernetes. The key Vault's resource id is linked to secretStore.keyVaultId."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultSecretReferenceSecretInfo {
    #[doc = "Name of the Key Vault secret."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Version of the Key Vault secret."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl KeyVaultSecretReferenceSecretInfo {
    pub fn new() -> Self {
        Self { name: None, version: None }
    }
}
#[doc = "The secret info when type is keyVaultSecretUri. It's for scenario that user provides a secret stored in user's keyvault and source is Web App, Spring Cloud or Container App."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultSecretUriSecretInfo {
    #[doc = "URI to the keyvault secret"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl KeyVaultSecretUriSecretInfo {
    pub fn new() -> Self {
        Self { value: None }
    }
}
#[doc = "A Linker to be updated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkerPatch {
    #[doc = "The properties of the Linker."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LinkerProperties>,
}
impl LinkerPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the Linker."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkerProperties {
    #[doc = "The target service properties"]
    #[serde(rename = "targetService", default, skip_serializing_if = "Option::is_none")]
    pub target_service: Option<TargetServiceBaseUnion>,
    #[doc = "The authentication info"]
    #[serde(rename = "authInfo", default, skip_serializing_if = "Option::is_none")]
    pub auth_info: Option<AuthInfoBaseUnion>,
    #[doc = "The application client type"]
    #[serde(rename = "clientType", default, skip_serializing_if = "Option::is_none")]
    pub client_type: Option<ClientType>,
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
    #[doc = "Indicates public network solution, include firewall rules"]
    #[serde(rename = "publicNetworkSolution", default, skip_serializing_if = "Option::is_none")]
    pub public_network_solution: Option<PublicNetworkSolution>,
    #[doc = "The configuration information, used to generate configurations or save to applications"]
    #[serde(rename = "configurationInfo", default, skip_serializing_if = "Option::is_none")]
    pub configuration_info: Option<ConfigurationInfo>,
}
impl LinkerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Linker of source and target resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkerResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of the Linker."]
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The represent of missing permissions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionsMissingDryrunPrerequisiteResult {
    #[doc = "The permission scope"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The permission list"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub permissions: Vec<String>,
    #[doc = "The recommended role to resolve permissions missing"]
    #[serde(rename = "recommendedRole", default, skip_serializing_if = "Option::is_none")]
    pub recommended_role: Option<String>,
}
impl PermissionsMissingDryrunPrerequisiteResult {
    pub fn new() -> Self {
        Self {
            scope: None,
            permissions: Vec::new(),
            recommended_role: None,
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
#[doc = "Indicates public network solution, include firewall rules"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicNetworkSolution {
    #[doc = "The cleanup behavior to indicate whether clean up operation when resource is deleted or updated"]
    #[serde(rename = "deleteOrUpdateBehavior", default, skip_serializing_if = "Option::is_none")]
    pub delete_or_update_behavior: Option<DeleteOrUpdateBehavior>,
    #[doc = "Indicates how to apply the connector operations, such as opt out network configuration, opt in configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionType>,
    #[doc = "Target service's firewall rules. to allow connections from source service."]
    #[serde(rename = "firewallRules", default, skip_serializing_if = "Option::is_none")]
    pub firewall_rules: Option<FirewallRules>,
}
impl PublicNetworkSolution {
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of Linker."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceList {
    #[doc = "The Linker used to get the next page of Linker list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of Linkers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<LinkerResource>,
}
impl azure_core::Continuable for ResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The authentication info when authType is secret"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretAuthInfo {
    #[doc = "Username or account name for secret auth."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The secret info"]
    #[serde(rename = "secretInfo", default, skip_serializing_if = "Option::is_none")]
    pub secret_info: Option<SecretInfoBaseUnion>,
}
impl SecretAuthInfo {
    pub fn new() -> Self {
        Self {
            name: None,
            secret_info: None,
        }
    }
}
#[doc = "The secret type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "secretType")]
pub enum SecretInfoBaseUnion {
    #[serde(rename = "keyVaultSecretReference")]
    KeyVaultSecretReference(KeyVaultSecretReferenceSecretInfo),
    #[serde(rename = "keyVaultSecretUri")]
    KeyVaultSecretUri(KeyVaultSecretUriSecretInfo),
    #[serde(rename = "rawValue")]
    RawValue(ValueSecretInfo),
}
#[doc = "An option to store secret value in secure place"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretStore {
    #[doc = "The key vault id to store secret"]
    #[serde(rename = "keyVaultId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_id: Option<String>,
    #[doc = "The key vault secret name to store secret, only valid when storing one secret"]
    #[serde(rename = "keyVaultSecretName", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_secret_name: Option<String>,
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
#[doc = "The service properties when target service type is SelfHostedServer"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelfHostedServer {
    #[doc = "The endpoint of service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl SelfHostedServer {
    pub fn new() -> Self {
        Self { endpoint: None }
    }
}
#[doc = "The authentication info when authType is servicePrincipal certificate"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipalCertificateAuthInfo {
    #[doc = "Application clientId for servicePrincipal auth."]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "Principal Id for servicePrincipal auth."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "ServicePrincipal certificate for servicePrincipal auth."]
    pub certificate: String,
    #[doc = "The cleanup behavior to indicate whether clean up operation when resource is deleted or updated"]
    #[serde(rename = "deleteOrUpdateBehavior", default, skip_serializing_if = "Option::is_none")]
    pub delete_or_update_behavior: Option<DeleteOrUpdateBehavior>,
    #[doc = "Optional, this value specifies the Azure roles to be assigned. Automatically "]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub roles: Vec<String>,
}
impl ServicePrincipalCertificateAuthInfo {
    pub fn new(client_id: String, principal_id: String, certificate: String) -> Self {
        Self {
            client_id,
            principal_id,
            certificate,
            delete_or_update_behavior: None,
            roles: Vec::new(),
        }
    }
}
#[doc = "The authentication info when authType is servicePrincipal secret"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipalSecretAuthInfo {
    #[serde(flatten)]
    pub database_aad_auth_info: DatabaseAadAuthInfo,
    #[doc = "ServicePrincipal application clientId for servicePrincipal auth."]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "Principal Id for servicePrincipal auth."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "Secret for servicePrincipal auth."]
    pub secret: String,
    #[doc = "The cleanup behavior to indicate whether clean up operation when resource is deleted or updated"]
    #[serde(rename = "deleteOrUpdateBehavior", default, skip_serializing_if = "Option::is_none")]
    pub delete_or_update_behavior: Option<DeleteOrUpdateBehavior>,
    #[doc = "Optional, this value specifies the Azure roles to be assigned. Automatically "]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub roles: Vec<String>,
}
impl ServicePrincipalSecretAuthInfo {
    pub fn new(client_id: String, principal_id: String, secret: String) -> Self {
        Self {
            database_aad_auth_info: DatabaseAadAuthInfo::default(),
            client_id,
            principal_id,
            secret,
            delete_or_update_behavior: None,
            roles: Vec::new(),
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
#[doc = "The authentication info when authType is systemAssignedIdentity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemAssignedIdentityAuthInfo {
    #[serde(flatten)]
    pub database_aad_auth_info: DatabaseAadAuthInfo,
    #[doc = "The cleanup behavior to indicate whether clean up operation when resource is deleted or updated"]
    #[serde(rename = "deleteOrUpdateBehavior", default, skip_serializing_if = "Option::is_none")]
    pub delete_or_update_behavior: Option<DeleteOrUpdateBehavior>,
    #[doc = "Optional, this value specifies the Azure role to be assigned"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub roles: Vec<String>,
}
impl SystemAssignedIdentityAuthInfo {
    pub fn new() -> Self {
        Self {
            database_aad_auth_info: DatabaseAadAuthInfo::default(),
            delete_or_update_behavior: None,
            roles: Vec::new(),
        }
    }
}
#[doc = "The target service type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TargetServiceBaseUnion {
    AzureResource(AzureResource),
    ConfluentBootstrapServer(ConfluentBootstrapServer),
    ConfluentSchemaRegistry(ConfluentSchemaRegistry),
    SelfHostedServer(SelfHostedServer),
}
#[doc = "The target service type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TargetServiceType")]
pub enum TargetServiceType {
    AzureResource,
    ConfluentBootstrapServer,
    ConfluentSchemaRegistry,
    SelfHostedServer,
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
            Self::SelfHostedServer => serializer.serialize_unit_variant("TargetServiceType", 3u32, "SelfHostedServer"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The authentication info when authType is user account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAccountAuthInfo {
    #[serde(flatten)]
    pub database_aad_auth_info: DatabaseAadAuthInfo,
    #[doc = "Principal Id for user account."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The cleanup behavior to indicate whether clean up operation when resource is deleted or updated"]
    #[serde(rename = "deleteOrUpdateBehavior", default, skip_serializing_if = "Option::is_none")]
    pub delete_or_update_behavior: Option<DeleteOrUpdateBehavior>,
    #[doc = "Optional, this value specifies the Azure roles to be assigned. Automatically "]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub roles: Vec<String>,
}
impl UserAccountAuthInfo {
    pub fn new() -> Self {
        Self {
            database_aad_auth_info: DatabaseAadAuthInfo::default(),
            principal_id: None,
            delete_or_update_behavior: None,
            roles: Vec::new(),
        }
    }
}
#[doc = "The authentication info when authType is userAssignedIdentity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAssignedIdentityAuthInfo {
    #[serde(flatten)]
    pub database_aad_auth_info: DatabaseAadAuthInfo,
    #[doc = "Client Id for userAssignedIdentity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "Subscription id for userAssignedIdentity."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The cleanup behavior to indicate whether clean up operation when resource is deleted or updated"]
    #[serde(rename = "deleteOrUpdateBehavior", default, skip_serializing_if = "Option::is_none")]
    pub delete_or_update_behavior: Option<DeleteOrUpdateBehavior>,
    #[doc = "Optional, this value specifies the Azure role to be assigned"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub roles: Vec<String>,
}
impl UserAssignedIdentityAuthInfo {
    pub fn new() -> Self {
        Self {
            database_aad_auth_info: DatabaseAadAuthInfo::default(),
            client_id: None,
            subscription_id: None,
            delete_or_update_behavior: None,
            roles: Vec::new(),
        }
    }
}
#[doc = "The VNet solution for linker"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VNetSolution {
    #[doc = "Type of VNet solution."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<v_net_solution::Type>,
    #[doc = "The cleanup behavior to indicate whether clean up operation when resource is deleted or updated"]
    #[serde(rename = "deleteOrUpdateBehavior", default, skip_serializing_if = "Option::is_none")]
    pub delete_or_update_behavior: Option<DeleteOrUpdateBehavior>,
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
#[doc = "The validation operation result for a Linker."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateOperationResult {
    #[doc = "The validation result for a Linker."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ValidateResult>,
    #[doc = "Validated Linker id."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Validation operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl ValidateOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The validation result for a Linker."]
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
    #[doc = "The resource id of the Linker source application."]
    #[serde(rename = "sourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[doc = "The resource Id of target service."]
    #[serde(rename = "targetId", default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[doc = "The authentication type."]
    #[serde(rename = "authType", default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<AuthType>,
    #[doc = "The detail of validation result"]
    #[serde(
        rename = "validationDetail",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub validation_detail: Vec<ValidationResultItem>,
}
impl ValidateResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The validation item for a Linker."]
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
        #[serde(rename = "failure")]
        Failure,
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
                Self::Failure => serializer.serialize_unit_variant("Result", 1u32, "failure"),
                Self::Warning => serializer.serialize_unit_variant("Result", 2u32, "warning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The secret info when type is rawValue. It's for scenarios that user input the secret."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueSecretInfo {
    #[doc = "The actual value of the secret."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ValueSecretInfo {
    pub fn new() -> Self {
        Self { value: None }
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
