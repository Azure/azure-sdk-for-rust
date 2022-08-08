#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Gets or sets the application server configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationServerConfiguration {
    #[doc = "The subnet id."]
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[doc = "Defines the virtual machine configuration."]
    #[serde(rename = "virtualMachineConfiguration")]
    pub virtual_machine_configuration: VirtualMachineConfiguration,
    #[doc = "The number of app server instances."]
    #[serde(rename = "instanceCount")]
    pub instance_count: i64,
}
impl ApplicationServerConfiguration {
    pub fn new(subnet_id: String, virtual_machine_configuration: VirtualMachineConfiguration, instance_count: i64) -> Self {
        Self {
            subnet_id,
            virtual_machine_configuration,
            instance_count,
        }
    }
}
#[doc = "Gets or sets the central server configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CentralServerConfiguration {
    #[doc = "The subnet id."]
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[doc = "Defines the virtual machine configuration."]
    #[serde(rename = "virtualMachineConfiguration")]
    pub virtual_machine_configuration: VirtualMachineConfiguration,
    #[doc = "The number of central server VMs."]
    #[serde(rename = "instanceCount")]
    pub instance_count: i64,
}
impl CentralServerConfiguration {
    pub fn new(subnet_id: String, virtual_machine_configuration: VirtualMachineConfiguration, instance_count: i64) -> Self {
        Self {
            subnet_id,
            virtual_machine_configuration,
            instance_count,
        }
    }
}
#[doc = "Defines the type of central server VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CentralServerVirtualMachineType")]
pub enum CentralServerVirtualMachineType {
    Primary,
    Secondary,
    Unknown,
    #[serde(rename = "ASCS")]
    Ascs,
    #[serde(rename = "ERSInactive")]
    ErsInactive,
    #[serde(rename = "ERS")]
    Ers,
    Standby,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CentralServerVirtualMachineType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CentralServerVirtualMachineType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CentralServerVirtualMachineType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Primary => serializer.serialize_unit_variant("CentralServerVirtualMachineType", 0u32, "Primary"),
            Self::Secondary => serializer.serialize_unit_variant("CentralServerVirtualMachineType", 1u32, "Secondary"),
            Self::Unknown => serializer.serialize_unit_variant("CentralServerVirtualMachineType", 2u32, "Unknown"),
            Self::Ascs => serializer.serialize_unit_variant("CentralServerVirtualMachineType", 3u32, "ASCS"),
            Self::ErsInactive => serializer.serialize_unit_variant("CentralServerVirtualMachineType", 4u32, "ERSInactive"),
            Self::Ers => serializer.serialize_unit_variant("CentralServerVirtualMachineType", 5u32, "ERS"),
            Self::Standby => serializer.serialize_unit_variant("CentralServerVirtualMachineType", 6u32, "Standby"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The Central Server VM Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CentralServerVmDetails {
    #[doc = "Defines the type of central server VM."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<CentralServerVirtualMachineType>,
    #[serde(rename = "virtualMachineId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_id: Option<String>,
}
impl CentralServerVmDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration Type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ConfigurationType")]
pub enum ConfigurationType {
    Deployment,
    Discovery,
    #[serde(rename = "DeploymentWithOSConfig")]
    DeploymentWithOsConfig,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ConfigurationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ConfigurationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ConfigurationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Deployment => serializer.serialize_unit_variant("ConfigurationType", 0u32, "Deployment"),
            Self::Discovery => serializer.serialize_unit_variant("ConfigurationType", 1u32, "Discovery"),
            Self::DeploymentWithOsConfig => serializer.serialize_unit_variant("ConfigurationType", 2u32, "DeploymentWithOSConfig"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Gets or sets the DB2 provider properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Db2ProviderInstanceProperties {
    #[serde(flatten)]
    pub provider_specific_properties: ProviderSpecificProperties,
    #[doc = "Gets or sets the target virtual machine name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[doc = "Gets or sets the db2 database name."]
    #[serde(rename = "dbName", default, skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
    #[doc = "Gets or sets the db2 database sql port."]
    #[serde(rename = "dbPort", default, skip_serializing_if = "Option::is_none")]
    pub db_port: Option<String>,
    #[doc = "Gets or sets the db2 database user name."]
    #[serde(rename = "dbUsername", default, skip_serializing_if = "Option::is_none")]
    pub db_username: Option<String>,
    #[doc = "Gets or sets the db2 database password."]
    #[serde(rename = "dbPassword", default, skip_serializing_if = "Option::is_none")]
    pub db_password: Option<String>,
    #[doc = "Gets or sets the key vault URI to secret with the database password."]
    #[serde(rename = "dbPasswordUri", default, skip_serializing_if = "Option::is_none")]
    pub db_password_uri: Option<String>,
    #[doc = "Gets or sets the SAP System Identifier"]
    #[serde(rename = "sapSid", default, skip_serializing_if = "Option::is_none")]
    pub sap_sid: Option<String>,
}
impl Db2ProviderInstanceProperties {
    pub fn new(provider_specific_properties: ProviderSpecificProperties) -> Self {
        Self {
            provider_specific_properties,
            hostname: None,
            db_name: None,
            db_port: None,
            db_username: None,
            db_password: None,
            db_password_uri: None,
            sap_sid: None,
        }
    }
}
#[doc = "Gets or sets the database configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseConfiguration {
    #[doc = "Defines the supported SAP Database types."]
    #[serde(rename = "databaseType", default, skip_serializing_if = "Option::is_none")]
    pub database_type: Option<SapDatabaseType>,
    #[doc = "The subnet id."]
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[doc = "Defines the virtual machine configuration."]
    #[serde(rename = "virtualMachineConfiguration")]
    pub virtual_machine_configuration: VirtualMachineConfiguration,
    #[doc = "The number of database VMs."]
    #[serde(rename = "instanceCount")]
    pub instance_count: i64,
}
impl DatabaseConfiguration {
    pub fn new(subnet_id: String, virtual_machine_configuration: VirtualMachineConfiguration, instance_count: i64) -> Self {
        Self {
            database_type: None,
            subnet_id,
            virtual_machine_configuration,
            instance_count,
        }
    }
}
#[doc = "The database scale method."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DatabaseScaleMethod")]
pub enum DatabaseScaleMethod {
    ScaleUp,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DatabaseScaleMethod {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DatabaseScaleMethod {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DatabaseScaleMethod {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ScaleUp => serializer.serialize_unit_variant("DatabaseScaleMethod", 0u32, "ScaleUp"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The Database VM Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseVmDetails {
    #[serde(rename = "virtualMachineId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_id: Option<String>,
    #[doc = "Defines the SAP Instance status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SapVirtualInstanceStatus>,
}
impl DatabaseVmDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the url and storage account ID where deployer VM packages are uploaded"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployerVmPackages {
    #[doc = "The URL to the deployer VM packages file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The deployer VM packages storage account id"]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
}
impl DeployerVmPackages {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentConfiguration {
    #[serde(flatten)]
    pub sap_configuration: SapConfiguration,
    #[doc = "The geo-location where the SAP system is to be created."]
    #[serde(rename = "appLocation", default, skip_serializing_if = "Option::is_none")]
    pub app_location: Option<String>,
    #[doc = "Deploy SAP Infrastructure Details."]
    #[serde(rename = "infrastructureConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub infrastructure_configuration: Option<InfrastructureConfiguration>,
    #[doc = "The SAP Software configuration Input."]
    #[serde(rename = "softwareConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub software_configuration: Option<SoftwareConfiguration>,
}
impl DeploymentConfiguration {
    pub fn new(sap_configuration: SapConfiguration) -> Self {
        Self {
            sap_configuration,
            app_location: None,
            infrastructure_configuration: None,
            software_configuration: None,
        }
    }
}
#[doc = "The deployment Type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeploymentType")]
pub enum DeploymentType {
    SingleServer,
    ThreeTier,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeploymentType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeploymentType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeploymentType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SingleServer => serializer.serialize_unit_variant("DeploymentType", 0u32, "SingleServer"),
            Self::ThreeTier => serializer.serialize_unit_variant("DeploymentType", 1u32, "ThreeTier"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Deployment along with OS Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentWithOsConfiguration {
    #[serde(flatten)]
    pub sap_configuration: SapConfiguration,
    #[doc = "The geo-location where the SAP system is to be created."]
    #[serde(rename = "appLocation", default, skip_serializing_if = "Option::is_none")]
    pub app_location: Option<String>,
    #[doc = "Deploy SAP Infrastructure Details."]
    #[serde(rename = "infrastructureConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub infrastructure_configuration: Option<InfrastructureConfiguration>,
    #[doc = "The SAP Software configuration Input."]
    #[serde(rename = "softwareConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub software_configuration: Option<SoftwareConfiguration>,
    #[doc = "Defines the OS and SAP Configurations for Deployment"]
    #[serde(rename = "osSapConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub os_sap_configuration: Option<OsSapConfiguration>,
}
impl DeploymentWithOsConfiguration {
    pub fn new(sap_configuration: SapConfiguration) -> Self {
        Self {
            sap_configuration,
            app_location: None,
            infrastructure_configuration: None,
            software_configuration: None,
            os_sap_configuration: None,
        }
    }
}
#[doc = "Discovery Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscoveryConfiguration {
    #[serde(flatten)]
    pub sap_configuration: SapConfiguration,
    #[doc = "The virtual machine ID of the Central Server."]
    #[serde(rename = "centralServerVmId", default, skip_serializing_if = "Option::is_none")]
    pub central_server_vm_id: Option<String>,
    #[doc = "The geo-location where the SAP system exists."]
    #[serde(rename = "appLocation", default, skip_serializing_if = "Option::is_none")]
    pub app_location: Option<String>,
}
impl DiscoveryConfiguration {
    pub fn new(sap_configuration: SapConfiguration) -> Self {
        Self {
            sap_configuration,
            central_server_vm_id: None,
            app_location: None,
        }
    }
}
#[doc = "Defines the SAP ERS Server properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnqueueReplicationServerProperties {
    #[doc = "Defines the type of Enqueue Replication Server."]
    #[serde(rename = "ersVersion", default, skip_serializing_if = "Option::is_none")]
    pub ers_version: Option<EnqueueReplicationServerType>,
    #[doc = "The ERS server instance id."]
    #[serde(rename = "instanceNo", default, skip_serializing_if = "Option::is_none")]
    pub instance_no: Option<String>,
    #[doc = "The ERS server SAP host name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[doc = "The ERS server SAP kernel version."]
    #[serde(rename = "kernelVersion", default, skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    #[doc = "The ERS server SAP kernel patch."]
    #[serde(rename = "kernelPatch", default, skip_serializing_if = "Option::is_none")]
    pub kernel_patch: Option<String>,
    #[doc = "The ERS server SAP IP Address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "Defines the SAP Instance health."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<HealthState>,
}
impl EnqueueReplicationServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the type of Enqueue Replication Server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnqueueReplicationServerType")]
pub enum EnqueueReplicationServerType {
    EnqueueReplicator1,
    EnqueueReplicator2,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnqueueReplicationServerType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnqueueReplicationServerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnqueueReplicationServerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::EnqueueReplicator1 => serializer.serialize_unit_variant("EnqueueReplicationServerType", 0u32, "EnqueueReplicator1"),
            Self::EnqueueReplicator2 => serializer.serialize_unit_variant("EnqueueReplicationServerType", 1u32, "EnqueueReplicator2"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the SAP enqueue server properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnqueueServerProperties {
    #[doc = "The enqueue server SAP host name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[doc = "The enqueue server SAP IP Address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The enqueue server Port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[doc = "Defines the SAP Instance health."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<HealthState>,
}
impl EnqueueServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the environment type - Production/Non Production."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnvironmentType")]
pub enum EnvironmentType {
    NonProd,
    Prod,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnvironmentType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnvironmentType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnvironmentType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NonProd => serializer.serialize_unit_variant("EnvironmentType", 0u32, "NonProd"),
            Self::Prod => serializer.serialize_unit_variant("EnvironmentType", 1u32, "Prod"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Standard error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "Server-defined set of error codes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Human-readable representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Array of details about specific errors that led to this reported error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<Error>,
    #[doc = "Object containing more specific information than  the current object about the error."]
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<error::InnerError>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod error {
    use super::*;
    #[doc = "Object containing more specific information than  the current object about the error."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct InnerError {
        #[doc = "Standard error object."]
        #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
        pub inner_error: Box<Option<Error>>,
    }
    impl InnerError {
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
#[doc = "Error definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDefinition {
    #[doc = "Service specific error code which serves as the substatus for the HTTP error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Internal error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDefinition>,
}
impl ErrorDefinition {
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
#[doc = "Defines the SAP Gateway Server properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayServerProperties {
    #[doc = "The gateway Port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[doc = "Defines the SAP Instance health."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<HealthState>,
}
impl GatewayServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets the provider properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HanaDbProviderInstanceProperties {
    #[serde(flatten)]
    pub provider_specific_properties: ProviderSpecificProperties,
    #[doc = "Gets or sets the target virtual machine size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[doc = "Gets or sets the hana database name."]
    #[serde(rename = "dbName", default, skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
    #[doc = "Gets or sets the database sql port."]
    #[serde(rename = "sqlPort", default, skip_serializing_if = "Option::is_none")]
    pub sql_port: Option<String>,
    #[doc = "Gets or sets the database instance number."]
    #[serde(rename = "instanceNumber", default, skip_serializing_if = "Option::is_none")]
    pub instance_number: Option<String>,
    #[doc = "Gets or sets the database user name."]
    #[serde(rename = "dbUsername", default, skip_serializing_if = "Option::is_none")]
    pub db_username: Option<String>,
    #[doc = "Gets or sets the database password."]
    #[serde(rename = "dbPassword", default, skip_serializing_if = "Option::is_none")]
    pub db_password: Option<String>,
    #[doc = "Gets or sets the key vault URI to secret with the database password."]
    #[serde(rename = "dbPasswordUri", default, skip_serializing_if = "Option::is_none")]
    pub db_password_uri: Option<String>,
    #[doc = "Gets or sets the blob URI to SSL certificate for the DB."]
    #[serde(rename = "dbSslCertificateUri", default, skip_serializing_if = "Option::is_none")]
    pub db_ssl_certificate_uri: Option<String>,
    #[doc = "Gets or sets the hostname(s) in the SSL certificate."]
    #[serde(rename = "sslHostNameInCertificate", default, skip_serializing_if = "Option::is_none")]
    pub ssl_host_name_in_certificate: Option<String>,
}
impl HanaDbProviderInstanceProperties {
    pub fn new(provider_specific_properties: ProviderSpecificProperties) -> Self {
        Self {
            provider_specific_properties,
            hostname: None,
            db_name: None,
            sql_port: None,
            instance_number: None,
            db_username: None,
            db_password: None,
            db_password_uri: None,
            db_ssl_certificate_uri: None,
            ssl_host_name_in_certificate: None,
        }
    }
}
#[doc = "Defines the SAP Instance health."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HealthState")]
pub enum HealthState {
    Unknown,
    Healthy,
    Unhealthy,
    Degraded,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HealthState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HealthState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HealthState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("HealthState", 0u32, "Unknown"),
            Self::Healthy => serializer.serialize_unit_variant("HealthState", 1u32, "Healthy"),
            Self::Unhealthy => serializer.serialize_unit_variant("HealthState", 2u32, "Unhealthy"),
            Self::Degraded => serializer.serialize_unit_variant("HealthState", 3u32, "Degraded"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Gets or sets the high availability configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HighAvailabilityConfiguration {
    #[doc = "The high availability Type. AvailabilitySet guarantees 99.95% availability. Availability Zone guarantees 99.99% availability."]
    #[serde(rename = "highAvailabilityType")]
    pub high_availability_type: HighAvailabilityType,
}
impl HighAvailabilityConfiguration {
    pub fn new(high_availability_type: HighAvailabilityType) -> Self {
        Self { high_availability_type }
    }
}
#[doc = "Gets or sets the HA software configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HighAvailabilitySoftwareConfiguration {
    #[doc = "The fencing client id."]
    #[serde(rename = "fencingClientId")]
    pub fencing_client_id: String,
    #[doc = "The fencing client id secret/password. The secret should never expire. This will be used pacemaker to start/stop the cluster VMs."]
    #[serde(rename = "fencingClientPassword")]
    pub fencing_client_password: String,
}
impl HighAvailabilitySoftwareConfiguration {
    pub fn new(fencing_client_id: String, fencing_client_password: String) -> Self {
        Self {
            fencing_client_id,
            fencing_client_password,
        }
    }
}
#[doc = "The high availability Type. AvailabilitySet guarantees 99.95% availability. Availability Zone guarantees 99.99% availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HighAvailabilityType")]
pub enum HighAvailabilityType {
    AvailabilitySet,
    AvailabilityZone,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HighAvailabilityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HighAvailabilityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HighAvailabilityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AvailabilitySet => serializer.serialize_unit_variant("HighAvailabilityType", 0u32, "AvailabilitySet"),
            Self::AvailabilityZone => serializer.serialize_unit_variant("HighAvailabilityType", 1u32, "AvailabilityZone"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies information about the image to use. You can specify information about platform images, marketplace images, or virtual machine images. This element is required when you want to use a platform image, marketplace image, or virtual machine image, but is not used in other creation operations. NOTE: Image reference publisher and offer can only be set when you create the scale set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageReference {
    #[doc = "The image publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Specifies the offer of the platform image or marketplace image used to create the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "The image SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "Specifies the version of the platform image or marketplace image used to create the virtual machine. The allowed formats are Major.Minor.Build or 'latest'. Major, Minor, and Build are decimal numbers. Specify 'latest' to use the latest version of an image available at deploy time. Even if you use 'latest', the VM image will not automatically update after deploy time even if a new version becomes available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Specifies in decimal numbers, the version of platform image or marketplace image used to create the virtual machine. This readonly field differs from 'version', only if the value specified in 'version' field is 'latest'."]
    #[serde(rename = "exactVersion", default, skip_serializing_if = "Option::is_none")]
    pub exact_version: Option<String>,
    #[doc = "Specified the shared gallery image unique id for vm deployment. This can be fetched from shared gallery image GET call."]
    #[serde(rename = "sharedGalleryImageId", default, skip_serializing_if = "Option::is_none")]
    pub shared_gallery_image_id: Option<String>,
}
impl ImageReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deploy SAP Infrastructure Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfrastructureConfiguration {
    #[doc = "The deployment Type."]
    #[serde(rename = "deploymentType")]
    pub deployment_type: DeploymentType,
    #[doc = "The application resource group where SAP system resources will be deployed."]
    #[serde(rename = "appResourceGroup")]
    pub app_resource_group: String,
}
impl InfrastructureConfiguration {
    pub fn new(deployment_type: DeploymentType, app_resource_group: String) -> Self {
        Self {
            deployment_type,
            app_resource_group,
        }
    }
}
#[doc = "Specifies the Linux operating system settings on the virtual machine. <br><br>For a list of supported Linux distributions, see [Linux on Azure-Endorsed Distributions](https://docs.microsoft.com/azure/virtual-machines/linux/endorsed-distros)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinuxConfiguration {
    #[serde(flatten)]
    pub os_configuration: OsConfiguration,
    #[doc = "Specifies whether password authentication should be disabled."]
    #[serde(rename = "disablePasswordAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub disable_password_authentication: Option<bool>,
    #[doc = "SSH configuration for Linux based VMs running on Azure"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssh: Option<SshConfiguration>,
    #[doc = "The SSH Key-pair used to authenticate with the VM. The key needs to be at least 2048-bit and in ssh-rsa format. <br><br> For creating ssh keys, see [Create SSH keys on Linux and Mac for Linux VMs in Azure](https://docs.microsoft.com/azure/virtual-machines/linux/create-ssh-keys-detailed)."]
    #[serde(rename = "sshKeyPair", default, skip_serializing_if = "Option::is_none")]
    pub ssh_key_pair: Option<SshKeyPair>,
}
impl LinuxConfiguration {
    pub fn new(os_configuration: OsConfiguration) -> Self {
        Self {
            os_configuration,
            disable_password_authentication: None,
            ssh: None,
            ssh_key_pair: None,
        }
    }
}
#[doc = "Managed resource group configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedRgConfiguration {
    #[doc = "Managed resource group name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ManagedRgConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of managed service identity (only None, UserAssigned types are allowed)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedServiceIdentityType")]
pub enum ManagedServiceIdentityType {
    None,
    UserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedServiceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedServiceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedServiceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedServiceIdentityType", 0u32, "None"),
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 1u32, "UserAssigned"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the SAP message server properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MessageServerProperties {
    #[doc = "The message server port."]
    #[serde(rename = "msPort", default, skip_serializing_if = "Option::is_none")]
    pub ms_port: Option<i64>,
    #[doc = "The message server internal MS port."]
    #[serde(rename = "internalMsPort", default, skip_serializing_if = "Option::is_none")]
    pub internal_ms_port: Option<i64>,
    #[doc = "The message server http port."]
    #[serde(rename = "httpPort", default, skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,
    #[doc = "The message server https port."]
    #[serde(rename = "httpsPort", default, skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,
    #[doc = "The message server SAP host name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[doc = "The message server IP Address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "Defines the SAP Instance health."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<HealthState>,
}
impl MessageServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SAP monitor info on Azure (ARM properties and SAP monitor properties)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Monitor {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Managed service identity (user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<UserAssignedServiceIdentity>,
    #[doc = "Describes the properties of a SAP monitor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MonitorProperties>,
}
impl Monitor {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
            properties: None,
        }
    }
}
#[doc = "The response from the List SAP monitors operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorListResult {
    #[doc = "The list of SAP monitors."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Monitor>,
    #[doc = "The URL to get the next set of SAP monitors."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MonitorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MonitorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a SAP monitor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorProperties {
    #[doc = "State of provisioning of the SAP monitor."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<monitor_properties::ProvisioningState>,
    #[doc = "Defines the SAP monitor errors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<serde_json::Value>,
    #[doc = "The SAP monitor resources will be deployed in the SAP monitoring region. The subnet region should be same as the SAP monitoring region."]
    #[serde(rename = "appLocation", default, skip_serializing_if = "Option::is_none")]
    pub app_location: Option<String>,
    #[doc = "Sets the routing preference of the SAP monitor. By default only RFC1918 traffic is routed to the customer VNET."]
    #[serde(rename = "routingPreference", default, skip_serializing_if = "Option::is_none")]
    pub routing_preference: Option<monitor_properties::RoutingPreference>,
    #[doc = "Sets the preference for zone redundancy on resources created for the SAP monitor. By default resources will be created which do not support zone redundancy."]
    #[serde(rename = "zoneRedundancyPreference", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundancy_preference: Option<String>,
    #[doc = "Managed resource group configuration"]
    #[serde(rename = "managedResourceGroupConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_configuration: Option<ManagedRgConfiguration>,
    #[doc = "The ARM ID of the Log Analytics Workspace that is used for SAP monitoring."]
    #[serde(rename = "logAnalyticsWorkspaceArmId", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics_workspace_arm_id: Option<String>,
    #[doc = "The subnet which the SAP monitor will be deployed in"]
    #[serde(rename = "monitorSubnet", default, skip_serializing_if = "Option::is_none")]
    pub monitor_subnet: Option<String>,
    #[doc = "The ARM ID of the MSI used for SAP monitoring."]
    #[serde(rename = "msiArmId", default, skip_serializing_if = "Option::is_none")]
    pub msi_arm_id: Option<String>,
}
impl MonitorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod monitor_properties {
    use super::*;
    #[doc = "State of provisioning of the SAP monitor."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Accepted,
        Creating,
        Updating,
        Failed,
        Succeeded,
        Deleting,
        Migrating,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Succeeded"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
                Self::Migrating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Migrating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Sets the routing preference of the SAP monitor. By default only RFC1918 traffic is routed to the customer VNET."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RoutingPreference")]
    pub enum RoutingPreference {
        Default,
        RouteAll,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RoutingPreference {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RoutingPreference {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RoutingPreference {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("RoutingPreference", 0u32, "Default"),
                Self::RouteAll => serializer.serialize_unit_variant("RoutingPreference", 1u32, "RouteAll"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Gets or sets the SQL server provider properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MsSqlServerProviderInstanceProperties {
    #[serde(flatten)]
    pub provider_specific_properties: ProviderSpecificProperties,
    #[doc = "Gets or sets the SQL server host name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[doc = "Gets or sets the database sql port."]
    #[serde(rename = "dbPort", default, skip_serializing_if = "Option::is_none")]
    pub db_port: Option<String>,
    #[doc = "Gets or sets the database user name."]
    #[serde(rename = "dbUsername", default, skip_serializing_if = "Option::is_none")]
    pub db_username: Option<String>,
    #[doc = "Gets or sets the database password."]
    #[serde(rename = "dbPassword", default, skip_serializing_if = "Option::is_none")]
    pub db_password: Option<String>,
    #[doc = "Gets or sets the key vault URI to secret with the database password."]
    #[serde(rename = "dbPasswordUri", default, skip_serializing_if = "Option::is_none")]
    pub db_password_uri: Option<String>,
    #[doc = "Gets or sets the SAP System Identifier"]
    #[serde(rename = "sapSid", default, skip_serializing_if = "Option::is_none")]
    pub sap_sid: Option<String>,
}
impl MsSqlServerProviderInstanceProperties {
    pub fn new(provider_specific_properties: ProviderSpecificProperties) -> Self {
        Self {
            provider_specific_properties,
            hostname: None,
            db_port: None,
            db_username: None,
            db_password: None,
            db_password_uri: None,
            sap_sid: None,
        }
    }
}
#[doc = "Defines the network configuration for SAP infrastructure"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkConfiguration {
    #[doc = "Specifies whether a secondary IP address should be added to the network interface on all VMs"]
    #[serde(rename = "isSecondaryIpEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_secondary_ip_enabled: Option<bool>,
}
impl NetworkConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the OS configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsConfiguration {
    #[doc = "The OS Type"]
    #[serde(rename = "osType")]
    pub os_type: os_configuration::OsType,
}
impl OsConfiguration {
    pub fn new(os_type: os_configuration::OsType) -> Self {
        Self { os_type }
    }
}
pub mod os_configuration {
    use super::*;
    #[doc = "The OS Type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OsType")]
    pub enum OsType {
        Linux,
        Windows,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OsType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OsType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OsType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Linux => serializer.serialize_unit_variant("OsType", 0u32, "Linux"),
                Self::Windows => serializer.serialize_unit_variant("OsType", 1u32, "Windows"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies the operating system settings for the virtual machine. Some of the settings cannot be changed once VM is provisioned."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsProfile {
    #[doc = "Specifies the name of the administrator account. <br><br> This property cannot be updated after the VM is created. <br><br> **Windows-only restriction:** Cannot end in \".\" <br><br> **Disallowed values:** \"administrator\", \"admin\", \"user\", \"user1\", \"test\", \"user2\", \"test1\", \"user3\", \"admin1\", \"1\", \"123\", \"a\", \"actuser\", \"adm\", \"admin2\", \"aspnet\", \"backup\", \"console\", \"david\", \"guest\", \"john\", \"owner\", \"root\", \"server\", \"sql\", \"support\", \"support_388945a0\", \"sys\", \"test2\", \"test3\", \"user4\", \"user5\". <br><br> **Minimum-length (Linux):** 1  character <br><br> **Max-length (Linux):** 64 characters <br><br> **Max-length (Windows):** 20 characters."]
    #[serde(rename = "adminUsername", default, skip_serializing_if = "Option::is_none")]
    pub admin_username: Option<String>,
    #[doc = "Specifies the password of the administrator account. <br><br> **Minimum-length (Windows):** 8 characters <br><br> **Minimum-length (Linux):** 6 characters <br><br> **Max-length (Windows):** 123 characters <br><br> **Max-length (Linux):** 72 characters <br><br> **Complexity requirements:** 3 out of 4 conditions below need to be fulfilled <br> Has lower characters <br>Has upper characters <br> Has a digit <br> Has a special character (Regex match [\\W_]) <br><br> **Disallowed values:** \"abc@123\", \"P@$$w0rd\", \"P@ssw0rd\", \"P@ssword123\", \"Pa$$word\", \"pass@word1\", \"Password!\", \"Password1\", \"Password22\", \"iloveyou!\" <br><br> For resetting the password, see [How to reset the Remote Desktop service or its login password in a Windows VM](https://docs.microsoft.com/troubleshoot/azure/virtual-machines/reset-rdp) <br><br> For resetting root password, see [Manage users, SSH, and check or repair disks on Azure Linux VMs using the VMAccess Extension](https://docs.microsoft.com/troubleshoot/azure/virtual-machines/troubleshoot-ssh-connection)"]
    #[serde(rename = "adminPassword", default, skip_serializing_if = "Option::is_none")]
    pub admin_password: Option<String>,
    #[doc = "Defines the OS configuration."]
    #[serde(rename = "osConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub os_configuration: Option<OsConfiguration>,
}
impl OsProfile {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "Percent of the operation that is complete."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[doc = "The start time of the operation."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the operation."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The operations list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<OperationStatusResult>,
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
            percent_complete: None,
            start_time: None,
            end_time: None,
            operations: Vec::new(),
            error: None,
        }
    }
}
#[doc = "Defines the workload operation content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsContent {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of an Operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationsDefinition>,
}
impl OperationsContent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsDefinition {
    #[doc = "Name of the operation."]
    pub name: String,
    #[doc = "Indicates whether the operation applies to data-plane."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Defines the workload operation origin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operations_definition::Origin>,
    #[doc = "Display information of the operation."]
    pub display: serde_json::Value,
    #[doc = "Defines the action type of workload operation."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operations_definition::ActionType>,
    #[doc = "Defines the workload operation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OperationsDefinition {
    pub fn new(name: String, display: serde_json::Value) -> Self {
        Self {
            name,
            is_data_action: None,
            origin: None,
            display,
            action_type: None,
            properties: None,
        }
    }
}
pub mod operations_definition {
    use super::*;
    #[doc = "Defines the workload operation origin."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        NotSpecified,
        User,
        System,
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
                Self::NotSpecified => serializer.serialize_unit_variant("Origin", 0u32, "NotSpecified"),
                Self::User => serializer.serialize_unit_variant("Origin", 1u32, "User"),
                Self::System => serializer.serialize_unit_variant("Origin", 2u32, "System"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Defines the action type of workload operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionType")]
    pub enum ActionType {
        NotSpecified,
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
                Self::NotSpecified => serializer.serialize_unit_variant("ActionType", 0u32, "NotSpecified"),
                Self::Internal => serializer.serialize_unit_variant("ActionType", 1u32, "Internal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the workload operation definition response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsDefinitionArrayResponseWithContinuation {
    #[doc = "Defines the workload operation definition response properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationsDefinition>,
    #[doc = "The URL to get to the next set of results, if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationsDefinitionArrayResponseWithContinuation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the workload operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsDisplayDefinition {
    #[doc = "Defines the workload provider."]
    pub provider: String,
    #[doc = "Defines the workload resource."]
    pub resource: String,
    #[doc = "Defines the workload operation."]
    pub operation: String,
    #[doc = "Describes the workload operation."]
    pub description: String,
}
impl OperationsDisplayDefinition {
    pub fn new(provider: String, resource: String, operation: String, description: String) -> Self {
        Self {
            provider,
            resource,
            operation,
            description,
        }
    }
}
#[doc = "Defines the OS and SAP Configurations for Deployment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsSapConfiguration {
    #[doc = "Defines the url and storage account ID where deployer VM packages are uploaded"]
    #[serde(rename = "deployerVmPackages", default, skip_serializing_if = "Option::is_none")]
    pub deployer_vm_packages: Option<DeployerVmPackages>,
    #[doc = "The FQDN to set for the SAP system"]
    #[serde(rename = "sapFqdn", default, skip_serializing_if = "Option::is_none")]
    pub sap_fqdn: Option<String>,
}
impl OsSapConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource patch request body"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchResourceRequestBody {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<serde_json::Value>,
}
impl PatchResourceRequestBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets the PrometheusHaCluster provider properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrometheusHaClusterProviderInstanceProperties {
    #[serde(flatten)]
    pub provider_specific_properties: ProviderSpecificProperties,
    #[doc = "URL of the Node Exporter endpoint."]
    #[serde(rename = "prometheusUrl", default, skip_serializing_if = "Option::is_none")]
    pub prometheus_url: Option<String>,
    #[doc = "Gets or sets the target machine name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[doc = "Gets or sets the cluster sid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[doc = "Gets or sets the clusterName."]
    #[serde(rename = "clusterName", default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
}
impl PrometheusHaClusterProviderInstanceProperties {
    pub fn new(provider_specific_properties: ProviderSpecificProperties) -> Self {
        Self {
            provider_specific_properties,
            prometheus_url: None,
            hostname: None,
            sid: None,
            cluster_name: None,
        }
    }
}
#[doc = "Gets or sets the PrometheusOS provider properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrometheusOsProviderInstanceProperties {
    #[serde(flatten)]
    pub provider_specific_properties: ProviderSpecificProperties,
    #[doc = "URL of the Node Exporter endpoint"]
    #[serde(rename = "prometheusUrl", default, skip_serializing_if = "Option::is_none")]
    pub prometheus_url: Option<String>,
}
impl PrometheusOsProviderInstanceProperties {
    pub fn new(provider_specific_properties: ProviderSpecificProperties) -> Self {
        Self {
            provider_specific_properties,
            prometheus_url: None,
        }
    }
}
#[doc = "A provider instance associated with SAP monitor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderInstance {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Managed service identity (user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<UserAssignedServiceIdentity>,
    #[doc = "Describes the properties of a provider instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProviderInstanceProperties>,
}
impl ProviderInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response from the List provider instances operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderInstanceListResult {
    #[doc = "The list of provider instances."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProviderInstance>,
    #[doc = "The URL to get the next set of provider instances."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProviderInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProviderInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a provider instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderInstanceProperties {
    #[doc = "State of provisioning of the provider instance"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<provider_instance_properties::ProvisioningState>,
    #[doc = "Defines the provider instance errors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<serde_json::Value>,
    #[doc = "Gets or sets the provider specific properties."]
    #[serde(rename = "providerSettings", default, skip_serializing_if = "Option::is_none")]
    pub provider_settings: Option<ProviderSpecificProperties>,
}
impl ProviderInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod provider_instance_properties {
    use super::*;
    #[doc = "State of provisioning of the provider instance"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Accepted,
        Creating,
        Updating,
        Failed,
        Succeeded,
        Deleting,
        Migrating,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Succeeded"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
                Self::Migrating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Migrating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Gets or sets the provider specific properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderSpecificProperties {
    #[doc = "The provider type. For example, the value can be SapHana."]
    #[serde(rename = "providerType")]
    pub provider_type: String,
}
impl ProviderSpecificProperties {
    pub fn new(provider_type: String) -> Self {
        Self { provider_type }
    }
}
#[doc = "Defines the provisioning states."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Updating,
    Creating,
    Failed,
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
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Creating"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SKU restriction information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestrictionInfo {
    #[doc = "The restriction locations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "The restriction zones."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
impl RestrictionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Define the SAP Application Server Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapApplicationServerInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Defines the SAP Application Server properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SapApplicationServerProperties>,
}
impl SapApplicationServerInstance {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Defines the collection of SAP Application Server Instances."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapApplicationServerInstanceList {
    #[doc = "Gets the list of SAP Application Server instances."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SapApplicationServerInstance>,
    #[doc = "Gets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SapApplicationServerInstanceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SapApplicationServerInstanceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the SAP Application Server properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapApplicationServerProperties {
    #[doc = "The application server instance id."]
    #[serde(rename = "instanceNo", default, skip_serializing_if = "Option::is_none")]
    pub instance_no: Option<String>,
    #[doc = "The application server subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    #[doc = "The application server SAP host name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[doc = "The application server SAP kernel version."]
    #[serde(rename = "kernelVersion", default, skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    #[doc = "The application server SAP kernel patch."]
    #[serde(rename = "kernelPatch", default, skip_serializing_if = "Option::is_none")]
    pub kernel_patch: Option<String>,
    #[doc = "The application server SAP IP Address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The application server gateway Port."]
    #[serde(rename = "gatewayPort", default, skip_serializing_if = "Option::is_none")]
    pub gateway_port: Option<i64>,
    #[doc = "The application server ICM HTTP Port."]
    #[serde(rename = "icmHttpPort", default, skip_serializing_if = "Option::is_none")]
    pub icm_http_port: Option<i64>,
    #[doc = "The application server ICM HTTPS Port."]
    #[serde(rename = "icmHttpsPort", default, skip_serializing_if = "Option::is_none")]
    pub icm_https_port: Option<i64>,
    #[doc = "The virtual machine."]
    #[serde(rename = "virtualMachineId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_id: Option<String>,
    #[doc = "Defines the SAP Instance status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SapVirtualInstanceStatus>,
    #[doc = "Defines the SAP Instance health."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<HealthState>,
    #[doc = "Defines the provisioning states."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "An error response from the Virtual Instance for SAP Workload service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<SapVirtualInstanceError>,
}
impl SapApplicationServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SAP request to get list of availability zones."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapAvailabilityZoneDetailsRequest {
    #[doc = "The geo-location where the SAP resources will be created."]
    #[serde(rename = "appLocation")]
    pub app_location: String,
    #[doc = "Defines the SAP Product type."]
    #[serde(rename = "sapProduct")]
    pub sap_product: SapProductType,
    #[doc = "Defines the supported SAP Database types."]
    #[serde(rename = "databaseType")]
    pub database_type: SapDatabaseType,
}
impl SapAvailabilityZoneDetailsRequest {
    pub fn new(app_location: String, sap_product: SapProductType, database_type: SapDatabaseType) -> Self {
        Self {
            app_location,
            sap_product,
            database_type,
        }
    }
}
#[doc = "The list of supported availability zone pairs which are part of SAP HA deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapAvailabilityZoneDetailsResult {
    #[doc = "Gets the list of availability zone pairs."]
    #[serde(rename = "availabilityZonePairs", default, skip_serializing_if = "Vec::is_empty")]
    pub availability_zone_pairs: Vec<SapAvailabilityZonePair>,
}
impl SapAvailabilityZoneDetailsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SAP Availability Zone Pair."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapAvailabilityZonePair {
    #[doc = "The zone A."]
    #[serde(rename = "zoneA", default, skip_serializing_if = "Option::is_none")]
    pub zone_a: Option<i64>,
    #[doc = "The zone B."]
    #[serde(rename = "zoneB", default, skip_serializing_if = "Option::is_none")]
    pub zone_b: Option<i64>,
}
impl SapAvailabilityZonePair {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the collection of SAP Central Instances."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapCentralInstanceList {
    #[doc = "Gets the list of SAP central instances."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SapCentralServerInstance>,
    #[doc = "Gets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SapCentralInstanceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SapCentralInstanceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Define the SAP Central Server Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapCentralServerInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Defines the SAP Central Server properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SapCentralServerProperties>,
}
impl SapCentralServerInstance {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Defines the SAP Central Server properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapCentralServerProperties {
    #[doc = "The central server instance id."]
    #[serde(rename = "instanceNo", default, skip_serializing_if = "Option::is_none")]
    pub instance_no: Option<String>,
    #[doc = "The central server subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    #[doc = "Defines the SAP message server properties."]
    #[serde(rename = "messageServerProperties", default, skip_serializing_if = "Option::is_none")]
    pub message_server_properties: Option<MessageServerProperties>,
    #[doc = "Defines the SAP enqueue server properties."]
    #[serde(rename = "enqueueServerProperties", default, skip_serializing_if = "Option::is_none")]
    pub enqueue_server_properties: Option<EnqueueServerProperties>,
    #[doc = "Defines the SAP Gateway Server properties."]
    #[serde(rename = "gatewayServerProperties", default, skip_serializing_if = "Option::is_none")]
    pub gateway_server_properties: Option<GatewayServerProperties>,
    #[doc = "Defines the SAP ERS Server properties."]
    #[serde(rename = "enqueueReplicationServerProperties", default, skip_serializing_if = "Option::is_none")]
    pub enqueue_replication_server_properties: Option<EnqueueReplicationServerProperties>,
    #[doc = "The central server kernel version."]
    #[serde(rename = "kernelVersion", default, skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    #[doc = "The central server kernel patch."]
    #[serde(rename = "kernelPatch", default, skip_serializing_if = "Option::is_none")]
    pub kernel_patch: Option<String>,
    #[doc = "The list of virtual machines."]
    #[serde(rename = "vmDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_details: Vec<CentralServerVmDetails>,
    #[doc = "Defines the SAP Instance status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SapVirtualInstanceStatus>,
    #[doc = "Defines the SAP Instance health."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<HealthState>,
    #[doc = "Defines the provisioning states."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "An error response from the Virtual Instance for SAP Workload service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<SapVirtualInstanceError>,
}
impl SapCentralServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SAP Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapConfiguration {
    #[doc = "The configuration Type."]
    #[serde(rename = "configurationType")]
    pub configuration_type: ConfigurationType,
}
impl SapConfiguration {
    pub fn new(configuration_type: ConfigurationType) -> Self {
        Self { configuration_type }
    }
}
#[doc = "Define the SAP Database Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapDatabaseInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Defines the SAP Database properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SapDatabaseProperties>,
}
impl SapDatabaseInstance {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Defines the collection of SAP Database Instances."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapDatabaseInstanceList {
    #[doc = "Gets the list of SAP Database instances."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SapDatabaseInstance>,
    #[doc = "Gets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SapDatabaseInstanceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SapDatabaseInstanceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the SAP Database properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapDatabaseProperties {
    #[doc = "The database subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    #[doc = "The database SID."]
    #[serde(rename = "databaseSid", default, skip_serializing_if = "Option::is_none")]
    pub database_sid: Option<String>,
    #[doc = "The SAP database type."]
    #[serde(rename = "databaseType", default, skip_serializing_if = "Option::is_none")]
    pub database_type: Option<String>,
    #[doc = "The database IP Address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The list of virtual machines."]
    #[serde(rename = "vmDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_details: Vec<DatabaseVmDetails>,
    #[doc = "Defines the SAP Instance status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SapVirtualInstanceStatus>,
    #[doc = "Defines the provisioning states."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "An error response from the Virtual Instance for SAP Workload service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<SapVirtualInstanceError>,
}
impl SapDatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the supported SAP Database types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SapDatabaseType")]
pub enum SapDatabaseType {
    #[serde(rename = "HANA")]
    Hana,
    #[serde(rename = "DB2")]
    Db2,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SapDatabaseType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SapDatabaseType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SapDatabaseType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Hana => serializer.serialize_unit_variant("SapDatabaseType", 0u32, "HANA"),
            Self::Db2 => serializer.serialize_unit_variant("SapDatabaseType", 1u32, "DB2"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The SAP Disk Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapDiskConfiguration {
    #[doc = "The volume name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[doc = "The disk type."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
    #[doc = "The disk count."]
    #[serde(rename = "diskCount", default, skip_serializing_if = "Option::is_none")]
    pub disk_count: Option<i64>,
    #[doc = "The disk size in GB."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i64>,
    #[doc = "The disk Iops."]
    #[serde(rename = "diskIopsReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub disk_iops_read_write: Option<i64>,
    #[doc = "The disk provisioned throughput in MBps."]
    #[serde(rename = "diskMBpsReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub disk_m_bps_read_write: Option<i64>,
    #[doc = "The disk storage type"]
    #[serde(rename = "diskStorageType", default, skip_serializing_if = "Option::is_none")]
    pub disk_storage_type: Option<String>,
}
impl SapDiskConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SAP request to get list of disk configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapDiskConfigurationsRequest {
    #[doc = "The geo-location where the SAP resources will be created."]
    #[serde(rename = "appLocation")]
    pub app_location: String,
    #[doc = "Defines the environment type - Production/Non Production."]
    pub environment: EnvironmentType,
    #[doc = "Defines the SAP Product type."]
    #[serde(rename = "sapProduct")]
    pub sap_product: SapProductType,
    #[doc = "Defines the supported SAP Database types."]
    #[serde(rename = "databaseType")]
    pub database_type: SapDatabaseType,
    #[doc = "The deployment Type."]
    #[serde(rename = "deploymentType")]
    pub deployment_type: DeploymentType,
    #[doc = "The VM SKU for database instance."]
    #[serde(rename = "dbVmSku")]
    pub db_vm_sku: String,
}
impl SapDiskConfigurationsRequest {
    pub fn new(
        app_location: String,
        environment: EnvironmentType,
        sap_product: SapProductType,
        database_type: SapDatabaseType,
        deployment_type: DeploymentType,
        db_vm_sku: String,
    ) -> Self {
        Self {
            app_location,
            environment,
            sap_product,
            database_type,
            deployment_type,
            db_vm_sku,
        }
    }
}
#[doc = "The list of disk configuration for vmSku which are part of SAP deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapDiskConfigurationsResult {
    #[doc = "Gets the list of Disk Configurations."]
    #[serde(rename = "diskConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub disk_configurations: Vec<SapDiskConfiguration>,
}
impl SapDiskConfigurationsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SAP Software configuration Input when the software is to be installed by service without OS Configurations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapInstallWithoutOsConfigSoftwareConfiguration {
    #[serde(flatten)]
    pub software_configuration: SoftwareConfiguration,
    #[doc = "The URL to the SAP Build of Materials(BOM) file."]
    #[serde(rename = "bomUrl")]
    pub bom_url: String,
    #[doc = "The SAP bits storage account id."]
    #[serde(rename = "sapBitsStorageAccountId")]
    pub sap_bits_storage_account_id: String,
    #[doc = "The software version to install."]
    #[serde(rename = "softwareVersion")]
    pub software_version: String,
    #[doc = "Gets or sets the HA software configuration."]
    #[serde(rename = "highAvailabilitySoftwareConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub high_availability_software_configuration: Option<HighAvailabilitySoftwareConfiguration>,
}
impl SapInstallWithoutOsConfigSoftwareConfiguration {
    pub fn new(
        software_configuration: SoftwareConfiguration,
        bom_url: String,
        sap_bits_storage_account_id: String,
        software_version: String,
    ) -> Self {
        Self {
            software_configuration,
            bom_url,
            sap_bits_storage_account_id,
            software_version,
            high_availability_software_configuration: None,
        }
    }
}
#[doc = "Defines the SAP Product type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SapProductType")]
pub enum SapProductType {
    #[serde(rename = "ECC")]
    Ecc,
    #[serde(rename = "S4HANA")]
    S4hana,
    Other,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SapProductType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SapProductType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SapProductType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ecc => serializer.serialize_unit_variant("SapProductType", 0u32, "ECC"),
            Self::S4hana => serializer.serialize_unit_variant("SapProductType", 1u32, "S4HANA"),
            Self::Other => serializer.serialize_unit_variant("SapProductType", 2u32, "Other"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The SAP Sizing Recommendation request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapSizingRecommendationRequest {
    #[doc = "The geo-location where the resource is to be created."]
    #[serde(rename = "appLocation")]
    pub app_location: String,
    #[doc = "Defines the environment type - Production/Non Production."]
    pub environment: EnvironmentType,
    #[doc = "Defines the SAP Product type."]
    #[serde(rename = "sapProduct")]
    pub sap_product: SapProductType,
    #[doc = "The deployment Type."]
    #[serde(rename = "deploymentType")]
    pub deployment_type: DeploymentType,
    #[doc = "The SAP Application Performance Standard measurement."]
    pub saps: i64,
    #[doc = "The database memory configuration."]
    #[serde(rename = "dbMemory")]
    pub db_memory: i64,
    #[doc = "Defines the supported SAP Database types."]
    #[serde(rename = "databaseType")]
    pub database_type: SapDatabaseType,
    #[doc = "The database scale method."]
    #[serde(rename = "dbScaleMethod", default, skip_serializing_if = "Option::is_none")]
    pub db_scale_method: Option<DatabaseScaleMethod>,
    #[doc = "The high availability Type. AvailabilitySet guarantees 99.95% availability. Availability Zone guarantees 99.99% availability."]
    #[serde(rename = "highAvailabilityType", default, skip_serializing_if = "Option::is_none")]
    pub high_availability_type: Option<HighAvailabilityType>,
}
impl SapSizingRecommendationRequest {
    pub fn new(
        app_location: String,
        environment: EnvironmentType,
        sap_product: SapProductType,
        deployment_type: DeploymentType,
        saps: i64,
        db_memory: i64,
        database_type: SapDatabaseType,
    ) -> Self {
        Self {
            app_location,
            environment,
            sap_product,
            deployment_type,
            saps,
            db_memory,
            database_type,
            db_scale_method: None,
            high_availability_type: None,
        }
    }
}
#[doc = "The SAP sizing recommendation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapSizingRecommendationResult {
    #[doc = "The deployment Type."]
    #[serde(rename = "deploymentType")]
    pub deployment_type: DeploymentType,
}
impl SapSizingRecommendationResult {
    pub fn new(deployment_type: DeploymentType) -> Self {
        Self { deployment_type }
    }
}
#[doc = "The SAP software installation Type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SapSoftwareInstallationType")]
pub enum SapSoftwareInstallationType {
    ServiceInitiated,
    #[serde(rename = "SAPInstallWithoutOSConfig")]
    SapInstallWithoutOsConfig,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SapSoftwareInstallationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SapSoftwareInstallationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SapSoftwareInstallationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ServiceInitiated => serializer.serialize_unit_variant("SapSoftwareInstallationType", 0u32, "ServiceInitiated"),
            Self::SapInstallWithoutOsConfig => {
                serializer.serialize_unit_variant("SapSoftwareInstallationType", 1u32, "SAPInstallWithoutOSConfig")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The list of supported SKUs for different resources which are part of SAP deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapSupportedResourceSkusResult {
    #[doc = "Gets the list of SAP supported SKUs."]
    #[serde(rename = "supportedSkus", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_skus: Vec<SapSupportedSku>,
}
impl SapSupportedResourceSkusResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SAP supported SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapSupportedSku {
    #[doc = "The VM Sku."]
    #[serde(rename = "vmSku", default, skip_serializing_if = "Option::is_none")]
    pub vm_sku: Option<String>,
    #[doc = "True if the Sku is certified for App server in the SAP system."]
    #[serde(rename = "isAppServerCertified", default, skip_serializing_if = "Option::is_none")]
    pub is_app_server_certified: Option<bool>,
    #[doc = "True if the Sku is certified for Database server in the SAP system."]
    #[serde(rename = "isDatabaseCertified", default, skip_serializing_if = "Option::is_none")]
    pub is_database_certified: Option<bool>,
}
impl SapSupportedSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SAP request to get list of supported SKUs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapSupportedSkusRequest {
    #[doc = "The geo-location where the resource is to be created."]
    #[serde(rename = "appLocation")]
    pub app_location: String,
    #[doc = "Defines the environment type - Production/Non Production."]
    pub environment: EnvironmentType,
    #[doc = "Defines the SAP Product type."]
    #[serde(rename = "sapProduct")]
    pub sap_product: SapProductType,
    #[doc = "The deployment Type."]
    #[serde(rename = "deploymentType")]
    pub deployment_type: DeploymentType,
    #[doc = "Defines the supported SAP Database types."]
    #[serde(rename = "databaseType")]
    pub database_type: SapDatabaseType,
    #[doc = "The high availability Type. AvailabilitySet guarantees 99.95% availability. Availability Zone guarantees 99.99% availability."]
    #[serde(rename = "highAvailabilityType", default, skip_serializing_if = "Option::is_none")]
    pub high_availability_type: Option<HighAvailabilityType>,
}
impl SapSupportedSkusRequest {
    pub fn new(
        app_location: String,
        environment: EnvironmentType,
        sap_product: SapProductType,
        deployment_type: DeploymentType,
        database_type: SapDatabaseType,
    ) -> Self {
        Self {
            app_location,
            environment,
            sap_product,
            deployment_type,
            database_type,
            high_availability_type: None,
        }
    }
}
#[doc = "Define the Virtual Instance for SAP."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapVirtualInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Managed service identity (user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<UserAssignedServiceIdentity>,
    #[doc = "Defines the Virtual Instance for SAP properties."]
    pub properties: SapVirtualInstanceProperties,
}
impl SapVirtualInstance {
    pub fn new(tracked_resource: TrackedResource, properties: SapVirtualInstanceProperties) -> Self {
        Self {
            tracked_resource,
            identity: None,
            properties,
        }
    }
}
#[doc = "An error response from the Virtual Instance for SAP Workload service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapVirtualInstanceError {
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ErrorDefinition>,
}
impl SapVirtualInstanceError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the collection of Virtual Instance for SAP."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapVirtualInstanceList {
    #[doc = "Gets the list of Virtual Instances for SAP."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SapVirtualInstance>,
    #[doc = "Gets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SapVirtualInstanceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SapVirtualInstanceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the Virtual Instance for SAP properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapVirtualInstanceProperties {
    #[doc = "Defines the environment type - Production/Non Production."]
    pub environment: EnvironmentType,
    #[doc = "Defines the SAP Product type."]
    #[serde(rename = "sapProduct")]
    pub sap_product: SapProductType,
    #[doc = "The SAP Configuration."]
    pub configuration: SapConfiguration,
    #[doc = "Managed resource group configuration"]
    #[serde(rename = "managedResourceGroupConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_configuration: Option<ManagedRgConfiguration>,
    #[doc = "Defines the SAP Instance status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SapVirtualInstanceStatus>,
    #[doc = "Defines the SAP Instance health."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<HealthState>,
    #[doc = "Defines the Virtual Instance for SAP state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<SapVirtualInstanceState>,
    #[doc = "Defines the provisioning states."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "An error response from the Virtual Instance for SAP Workload service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<SapVirtualInstanceError>,
}
impl SapVirtualInstanceProperties {
    pub fn new(environment: EnvironmentType, sap_product: SapProductType, configuration: SapConfiguration) -> Self {
        Self {
            environment,
            sap_product,
            configuration,
            managed_resource_group_configuration: None,
            status: None,
            health: None,
            state: None,
            provisioning_state: None,
            errors: None,
        }
    }
}
#[doc = "Defines the Virtual Instance for SAP state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SapVirtualInstanceState")]
pub enum SapVirtualInstanceState {
    InfrastructureDeploymentPending,
    InfrastructureDeploymentInProgress,
    InfrastructureDeploymentFailed,
    SoftwareInstallationPending,
    SoftwareInstallationInProgress,
    SoftwareInstallationFailed,
    DiscoveryPending,
    DiscoveryInProgress,
    DiscoveryFailed,
    RegistrationComplete,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SapVirtualInstanceState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SapVirtualInstanceState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SapVirtualInstanceState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::InfrastructureDeploymentPending => {
                serializer.serialize_unit_variant("SapVirtualInstanceState", 0u32, "InfrastructureDeploymentPending")
            }
            Self::InfrastructureDeploymentInProgress => {
                serializer.serialize_unit_variant("SapVirtualInstanceState", 1u32, "InfrastructureDeploymentInProgress")
            }
            Self::InfrastructureDeploymentFailed => {
                serializer.serialize_unit_variant("SapVirtualInstanceState", 2u32, "InfrastructureDeploymentFailed")
            }
            Self::SoftwareInstallationPending => {
                serializer.serialize_unit_variant("SapVirtualInstanceState", 3u32, "SoftwareInstallationPending")
            }
            Self::SoftwareInstallationInProgress => {
                serializer.serialize_unit_variant("SapVirtualInstanceState", 4u32, "SoftwareInstallationInProgress")
            }
            Self::SoftwareInstallationFailed => {
                serializer.serialize_unit_variant("SapVirtualInstanceState", 5u32, "SoftwareInstallationFailed")
            }
            Self::DiscoveryPending => serializer.serialize_unit_variant("SapVirtualInstanceState", 6u32, "DiscoveryPending"),
            Self::DiscoveryInProgress => serializer.serialize_unit_variant("SapVirtualInstanceState", 7u32, "DiscoveryInProgress"),
            Self::DiscoveryFailed => serializer.serialize_unit_variant("SapVirtualInstanceState", 8u32, "DiscoveryFailed"),
            Self::RegistrationComplete => serializer.serialize_unit_variant("SapVirtualInstanceState", 9u32, "RegistrationComplete"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the SAP Instance status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SapVirtualInstanceStatus")]
pub enum SapVirtualInstanceStatus {
    Starting,
    Running,
    Stopping,
    Offline,
    PartiallyRunning,
    Unavailable,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SapVirtualInstanceStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SapVirtualInstanceStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SapVirtualInstanceStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Starting => serializer.serialize_unit_variant("SapVirtualInstanceStatus", 0u32, "Starting"),
            Self::Running => serializer.serialize_unit_variant("SapVirtualInstanceStatus", 1u32, "Running"),
            Self::Stopping => serializer.serialize_unit_variant("SapVirtualInstanceStatus", 2u32, "Stopping"),
            Self::Offline => serializer.serialize_unit_variant("SapVirtualInstanceStatus", 3u32, "Offline"),
            Self::PartiallyRunning => serializer.serialize_unit_variant("SapVirtualInstanceStatus", 4u32, "PartiallyRunning"),
            Self::Unavailable => serializer.serialize_unit_variant("SapVirtualInstanceStatus", 5u32, "Unavailable"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Gets or sets the provider properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapNetWeaverProviderInstanceProperties {
    #[serde(flatten)]
    pub provider_specific_properties: ProviderSpecificProperties,
    #[doc = "Gets or sets the SAP System Identifier"]
    #[serde(rename = "sapSid", default, skip_serializing_if = "Option::is_none")]
    pub sap_sid: Option<String>,
    #[doc = "Gets or sets the target virtual machine IP Address/FQDN."]
    #[serde(rename = "sapHostname", default, skip_serializing_if = "Option::is_none")]
    pub sap_hostname: Option<String>,
    #[doc = "Gets or sets the instance number of SAP NetWeaver."]
    #[serde(rename = "sapInstanceNr", default, skip_serializing_if = "Option::is_none")]
    pub sap_instance_nr: Option<String>,
    #[doc = "Gets or sets the list of HostFile Entries"]
    #[serde(rename = "sapHostFileEntries", default, skip_serializing_if = "Vec::is_empty")]
    pub sap_host_file_entries: Vec<String>,
    #[doc = "Gets or sets the SAP user name."]
    #[serde(rename = "sapUsername", default, skip_serializing_if = "Option::is_none")]
    pub sap_username: Option<String>,
    #[doc = "Sets the SAP password."]
    #[serde(rename = "sapPassword", default, skip_serializing_if = "Option::is_none")]
    pub sap_password: Option<String>,
    #[doc = "Gets or sets the key vault URI to secret with the SAP password."]
    #[serde(rename = "sapPasswordUri", default, skip_serializing_if = "Option::is_none")]
    pub sap_password_uri: Option<String>,
    #[doc = "Gets or sets the SAP Client ID."]
    #[serde(rename = "sapClientId", default, skip_serializing_if = "Option::is_none")]
    pub sap_client_id: Option<String>,
    #[doc = "Gets or sets the SAP HTTP port number."]
    #[serde(rename = "sapPortNumber", default, skip_serializing_if = "Option::is_none")]
    pub sap_port_number: Option<String>,
    #[doc = "Gets or sets the blob URI to SSL certificate for the SAP system."]
    #[serde(rename = "sapSslCertificateUri", default, skip_serializing_if = "Option::is_none")]
    pub sap_ssl_certificate_uri: Option<String>,
}
impl SapNetWeaverProviderInstanceProperties {
    pub fn new(provider_specific_properties: ProviderSpecificProperties) -> Self {
        Self {
            provider_specific_properties,
            sap_sid: None,
            sap_hostname: None,
            sap_instance_nr: None,
            sap_host_file_entries: Vec::new(),
            sap_username: None,
            sap_password: None,
            sap_password_uri: None,
            sap_client_id: None,
            sap_port_number: None,
            sap_ssl_certificate_uri: None,
        }
    }
}
#[doc = "The SAP Software configuration Input when the software is to be installed by service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceInitiatedSoftwareConfiguration {
    #[serde(flatten)]
    pub software_configuration: SoftwareConfiguration,
    #[doc = "The URL to the SAP Build of Materials(BOM) file."]
    #[serde(rename = "bomUrl")]
    pub bom_url: String,
    #[doc = "The software version to install."]
    #[serde(rename = "softwareVersion")]
    pub software_version: String,
    #[doc = "The SAP bits storage account id."]
    #[serde(rename = "sapBitsStorageAccountId")]
    pub sap_bits_storage_account_id: String,
    #[doc = "The FQDN to set for the SAP system during install."]
    #[serde(rename = "sapFqdn")]
    pub sap_fqdn: String,
    #[doc = "The SSH private key."]
    #[serde(rename = "sshPrivateKey")]
    pub ssh_private_key: String,
    #[doc = "Gets or sets the HA software configuration."]
    #[serde(rename = "highAvailabilitySoftwareConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub high_availability_software_configuration: Option<HighAvailabilitySoftwareConfiguration>,
}
impl ServiceInitiatedSoftwareConfiguration {
    pub fn new(
        software_configuration: SoftwareConfiguration,
        bom_url: String,
        software_version: String,
        sap_bits_storage_account_id: String,
        sap_fqdn: String,
        ssh_private_key: String,
    ) -> Self {
        Self {
            software_configuration,
            bom_url,
            software_version,
            sap_bits_storage_account_id,
            sap_fqdn,
            ssh_private_key,
            high_availability_software_configuration: None,
        }
    }
}
#[doc = "Gets or sets the single server configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingleServerConfiguration {
    #[serde(flatten)]
    pub infrastructure_configuration: InfrastructureConfiguration,
    #[doc = "Defines the network configuration for SAP infrastructure"]
    #[serde(rename = "networkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[doc = "Defines the supported SAP Database types."]
    #[serde(rename = "databaseType", default, skip_serializing_if = "Option::is_none")]
    pub database_type: Option<SapDatabaseType>,
    #[doc = "The subnet id."]
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[doc = "Defines the virtual machine configuration."]
    #[serde(rename = "virtualMachineConfiguration")]
    pub virtual_machine_configuration: VirtualMachineConfiguration,
}
impl SingleServerConfiguration {
    pub fn new(
        infrastructure_configuration: InfrastructureConfiguration,
        subnet_id: String,
        virtual_machine_configuration: VirtualMachineConfiguration,
    ) -> Self {
        Self {
            infrastructure_configuration,
            network_configuration: None,
            database_type: None,
            subnet_id,
            virtual_machine_configuration,
        }
    }
}
#[doc = "The recommended configuration for a single server SAP system."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingleServerRecommendationResult {
    #[serde(flatten)]
    pub sap_sizing_recommendation_result: SapSizingRecommendationResult,
    #[doc = "The recommended VM SKU for single server."]
    #[serde(rename = "vmSku", default, skip_serializing_if = "Option::is_none")]
    pub vm_sku: Option<String>,
}
impl SingleServerRecommendationResult {
    pub fn new(sap_sizing_recommendation_result: SapSizingRecommendationResult) -> Self {
        Self {
            sap_sizing_recommendation_result,
            vm_sku: None,
        }
    }
}
#[doc = "The resource model definition representing SKU"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of the SKU. Ex - P3. It is typically a letter+number code"]
    pub name: String,
    #[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<SkuTier>,
    #[doc = "The SKU size. When the name field is the combination of tier and some other value, this would be the standalone code. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "If the service has different generations of hardware, for the same SKU, then that can be captured here."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "If the SKU supports scale out/in then the capacity integer should be included. If scale out/in is not possible for the resource this may be omitted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
#[doc = "The SKU capability definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCapability {
    #[doc = "The capability name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The capability value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SkuCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SKU capacity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCapacity {
    #[doc = "Minimum capacity value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,
    #[doc = "Maximum capacity value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
    #[doc = "Default capacity value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i32>,
    #[doc = "Scale type of the SKU capacity."]
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<sku_capacity::ScaleType>,
}
impl SkuCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sku_capacity {
    use super::*;
    #[doc = "Scale type of the SKU capacity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScaleType")]
    pub enum ScaleType {
        None,
        Manual,
        Automatic,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScaleType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScaleType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScaleType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ScaleType", 0u32, "None"),
                Self::Manual => serializer.serialize_unit_variant("ScaleType", 1u32, "Manual"),
                Self::Automatic => serializer.serialize_unit_variant("ScaleType", 2u32, "Automatic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The SKU cost definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCost {
    #[doc = "Billing meter id."]
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "The quantity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[doc = "The extended unit."]
    #[serde(rename = "extendedUnit", default, skip_serializing_if = "Option::is_none")]
    pub extended_unit: Option<String>,
}
impl SkuCost {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SKU definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuDefinition {
    #[doc = "The name of the SKU."]
    pub name: String,
    #[doc = "Resource type the SKU applicable for."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The SKU size. When the name field is the combination of tier and some other value, this would be the standalone code. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "If the service has different generations of hardware, for the same SKU, then that can be captured here."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "If the service has different kinds of hardware, for the same SKU, then that can be captured here."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "List of locations where this SKU is available."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "List of locations where this SKU is available."]
    #[serde(rename = "locationInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub location_info: Vec<SkuLocationAndZones>,
    #[doc = "If the SKU supports scale out/in then the capacity integer should be included. If scale out/in is not possible for the resource this may be omitted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<serde_json::Value>,
    #[doc = "The SKU costs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub costs: Vec<SkuCost>,
    #[doc = "The SKU capabilities."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<SkuCapability>,
    #[doc = "The SKU restrictions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<SkuRestriction>,
}
impl SkuDefinition {
    pub fn new(name: String) -> Self {
        Self {
            name,
            resource_type: None,
            tier: None,
            size: None,
            family: None,
            kind: None,
            locations: Vec::new(),
            location_info: Vec::new(),
            capacity: None,
            costs: Vec::new(),
            capabilities: Vec::new(),
            restrictions: Vec::new(),
        }
    }
}
#[doc = "The SKU location and zone."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuLocationAndZones {
    #[doc = "The location of the SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The availability zones of SKU location."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "The availability zone details of the SKU location."]
    #[serde(rename = "zoneDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub zone_details: Vec<SkuZoneDetail>,
    #[doc = "The extended locations of SKU."]
    #[serde(rename = "extendedLocations", default, skip_serializing_if = "Vec::is_empty")]
    pub extended_locations: Vec<String>,
    #[doc = "Type of the extended location."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<sku_location_and_zones::Type>,
}
impl SkuLocationAndZones {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sku_location_and_zones {
    use super::*;
    #[doc = "Type of the extended location."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Region,
        EdgeZone,
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
                Self::Region => serializer.serialize_unit_variant("Type", 0u32, "Region"),
                Self::EdgeZone => serializer.serialize_unit_variant("Type", 1u32, "EdgeZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The SKU restriction definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuRestriction {
    #[doc = "The SKU restriction type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<sku_restriction::Type>,
    #[doc = "Restriction values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[doc = "The restriction information."]
    #[serde(rename = "restrictionInfo", default, skip_serializing_if = "Option::is_none")]
    pub restriction_info: Option<serde_json::Value>,
    #[doc = "The SKU restriction reason code."]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<sku_restriction::ReasonCode>,
}
impl SkuRestriction {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sku_restriction {
    use super::*;
    #[doc = "The SKU restriction type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        NotSpecified,
        Location,
        Zone,
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
                Self::Location => serializer.serialize_unit_variant("Type", 1u32, "Location"),
                Self::Zone => serializer.serialize_unit_variant("Type", 2u32, "Zone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The SKU restriction reason code."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReasonCode")]
    pub enum ReasonCode {
        NotSpecified,
        QuotaId,
        NotAvailableForSubscription,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReasonCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReasonCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReasonCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("ReasonCode", 0u32, "NotSpecified"),
                Self::QuotaId => serializer.serialize_unit_variant("ReasonCode", 1u32, "QuotaId"),
                Self::NotAvailableForSubscription => serializer.serialize_unit_variant("ReasonCode", 2u32, "NotAvailableForSubscription"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SkuTier {
    Free,
    Basic,
    Standard,
    Premium,
}
#[doc = "The SKU zone details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuZoneDetail {
    #[doc = "The physical zones."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "The capabilities."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<SkuCapability>,
}
impl SkuZoneDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of SKUs supported by an Azure Resource Provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkusListResult {
    #[doc = "List of SKUs supported by the resource provider"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuDefinition>,
    #[doc = "URL to get the next set of SKU list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SkusListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SkusListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SAP Software configuration Input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoftwareConfiguration {
    #[doc = "The SAP software installation Type."]
    #[serde(rename = "softwareInstallationType")]
    pub software_installation_type: SapSoftwareInstallationType,
}
impl SoftwareConfiguration {
    pub fn new(software_installation_type: SapSoftwareInstallationType) -> Self {
        Self {
            software_installation_type,
        }
    }
}
#[doc = "SSH configuration for Linux based VMs running on Azure"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SshConfiguration {
    #[doc = "The list of SSH public keys used to authenticate with linux based VMs."]
    #[serde(rename = "publicKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub public_keys: Vec<SshPublicKey>,
}
impl SshConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SSH Key-pair used to authenticate with the VM. The key needs to be at least 2048-bit and in ssh-rsa format. <br><br> For creating ssh keys, see [Create SSH keys on Linux and Mac for Linux VMs in Azure](https://docs.microsoft.com/azure/virtual-machines/linux/create-ssh-keys-detailed)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SshKeyPair {
    #[doc = "SSH public key"]
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[doc = "SSH private key."]
    #[serde(rename = "privateKey", default, skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}
impl SshKeyPair {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains information about SSH certificate public key and the path on the Linux VM where the public key is placed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SshPublicKey {
    #[doc = "SSH public key certificate used to authenticate with the VM through ssh. The key needs to be at least 2048-bit and in ssh-rsa format. <br><br> For creating ssh keys, see [Create SSH keys on Linux and Mac for Linux VMs in Azure](https://docs.microsoft.com/azure/virtual-machines/linux/create-ssh-keys-detailed)."]
    #[serde(rename = "keyData", default, skip_serializing_if = "Option::is_none")]
    pub key_data: Option<String>,
}
impl SshPublicKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Stop SAP Request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StopRequest {
    #[doc = "A boolean to specify if the SAP system should be hard-stopped."]
    #[serde(rename = "hardStop", default, skip_serializing_if = "Option::is_none")]
    pub hard_stop: Option<bool>,
}
impl StopRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tags field of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {
    #[doc = "Tags field of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Tags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets the three tier SAP configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreeTierConfiguration {
    #[serde(flatten)]
    pub infrastructure_configuration: InfrastructureConfiguration,
    #[doc = "Defines the network configuration for SAP infrastructure"]
    #[serde(rename = "networkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[doc = "Gets or sets the central server configuration."]
    #[serde(rename = "centralServer")]
    pub central_server: CentralServerConfiguration,
    #[doc = "Gets or sets the application server configuration."]
    #[serde(rename = "applicationServer")]
    pub application_server: ApplicationServerConfiguration,
    #[doc = "Gets or sets the database configuration."]
    #[serde(rename = "databaseServer")]
    pub database_server: DatabaseConfiguration,
    #[doc = "Gets or sets the high availability configuration."]
    #[serde(rename = "highAvailabilityConfig", default, skip_serializing_if = "Option::is_none")]
    pub high_availability_config: Option<HighAvailabilityConfiguration>,
}
impl ThreeTierConfiguration {
    pub fn new(
        infrastructure_configuration: InfrastructureConfiguration,
        central_server: CentralServerConfiguration,
        application_server: ApplicationServerConfiguration,
        database_server: DatabaseConfiguration,
    ) -> Self {
        Self {
            infrastructure_configuration,
            network_configuration: None,
            central_server,
            application_server,
            database_server,
            high_availability_config: None,
        }
    }
}
#[doc = "The recommended configuration for a three tier SAP system."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreeTierRecommendationResult {
    #[serde(flatten)]
    pub sap_sizing_recommendation_result: SapSizingRecommendationResult,
    #[doc = "The database VM SKU."]
    #[serde(rename = "dbVmSku", default, skip_serializing_if = "Option::is_none")]
    pub db_vm_sku: Option<String>,
    #[doc = "The database server instance count."]
    #[serde(rename = "databaseInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub database_instance_count: Option<i64>,
    #[doc = "The central server VM SKU."]
    #[serde(rename = "centralServerVmSku", default, skip_serializing_if = "Option::is_none")]
    pub central_server_vm_sku: Option<String>,
    #[doc = "The central server instance count."]
    #[serde(rename = "centralServerInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub central_server_instance_count: Option<i64>,
    #[doc = "The application server VM SKU."]
    #[serde(rename = "applicationServerVmSku", default, skip_serializing_if = "Option::is_none")]
    pub application_server_vm_sku: Option<String>,
    #[doc = "The application server instance count."]
    #[serde(rename = "applicationServerInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub application_server_instance_count: Option<i64>,
}
impl ThreeTierRecommendationResult {
    pub fn new(sap_sizing_recommendation_result: SapSizingRecommendationResult) -> Self {
        Self {
            sap_sizing_recommendation_result,
            db_vm_sku: None,
            database_instance_count: None,
            central_server_vm_sku: None,
            central_server_instance_count: None,
            application_server_vm_sku: None,
            application_server_instance_count: None,
        }
    }
}
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
#[doc = "Defines the request body for updating SAP monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateMonitorRequest {
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Managed service identity (user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<UserAssignedServiceIdentity>,
}
impl UpdateMonitorRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the request body for updating SAP Application Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateSapApplicationInstanceRequest {
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl UpdateSapApplicationInstanceRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the request body for updating SAP Central Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateSapCentralInstanceRequest {
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl UpdateSapCentralInstanceRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the request body for updating SAP Database Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateSapDatabaseInstanceRequest {
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl UpdateSapDatabaseInstanceRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the request body for updating Virtual Instance for SAP."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateSapVirtualInstanceRequest {
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Managed service identity (user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<UserAssignedServiceIdentity>,
}
impl UpdateSapVirtualInstanceRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User assigned identity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed service identity (user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAssignedServiceIdentity {
    #[doc = "Type of managed service identity (only None, UserAssigned types are allowed)."]
    #[serde(rename = "type")]
    pub type_: ManagedServiceIdentityType,
    #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl UserAssignedServiceIdentity {
    pub fn new(type_: ManagedServiceIdentityType) -> Self {
        Self {
            type_,
            user_assigned_identities: None,
        }
    }
}
#[doc = "Defines the virtual machine configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineConfiguration {
    #[doc = "The virtual machine size."]
    #[serde(rename = "vmSize")]
    pub vm_size: String,
    #[doc = "Specifies information about the image to use. You can specify information about platform images, marketplace images, or virtual machine images. This element is required when you want to use a platform image, marketplace image, or virtual machine image, but is not used in other creation operations. NOTE: Image reference publisher and offer can only be set when you create the scale set."]
    #[serde(rename = "imageReference")]
    pub image_reference: ImageReference,
    #[doc = "Specifies the operating system settings for the virtual machine. Some of the settings cannot be changed once VM is provisioned."]
    #[serde(rename = "osProfile")]
    pub os_profile: OsProfile,
}
impl VirtualMachineConfiguration {
    pub fn new(vm_size: String, image_reference: ImageReference, os_profile: OsProfile) -> Self {
        Self {
            vm_size,
            image_reference,
            os_profile,
        }
    }
}
#[doc = "Specifies Windows operating system settings on the virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsConfiguration {
    #[serde(flatten)]
    pub os_configuration: OsConfiguration,
}
impl WindowsConfiguration {
    pub fn new(os_configuration: OsConfiguration) -> Self {
        Self { os_configuration }
    }
}
#[doc = "Backup profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupProfile {
    #[doc = "Whether to enable Azure backup for the workload"]
    #[serde(rename = "backupEnabled")]
    pub backup_enabled: backup_profile::BackupEnabled,
    #[doc = "Backup vault resource Id"]
    #[serde(rename = "vaultResourceId", default, skip_serializing_if = "Option::is_none")]
    pub vault_resource_id: Option<String>,
}
impl BackupProfile {
    pub fn new(backup_enabled: backup_profile::BackupEnabled) -> Self {
        Self {
            backup_enabled,
            vault_resource_id: None,
        }
    }
}
pub mod backup_profile {
    use super::*;
    #[doc = "Whether to enable Azure backup for the workload"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupEnabled")]
    pub enum BackupEnabled {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupEnabled {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupEnabled {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupEnabled {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("BackupEnabled", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("BackupEnabled", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Cache profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheProfile {
    #[doc = "Cache name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Cache SKU name"]
    #[serde(rename = "skuName")]
    pub sku_name: String,
    #[doc = "Cache family"]
    pub family: cache_profile::Family,
    #[doc = "Cache capacity"]
    pub capacity: i64,
    #[doc = "Cache resource Id"]
    #[serde(rename = "cacheResourceId", default, skip_serializing_if = "Option::is_none")]
    pub cache_resource_id: Option<String>,
}
impl CacheProfile {
    pub fn new(sku_name: String, family: cache_profile::Family, capacity: i64) -> Self {
        Self {
            name: None,
            sku_name,
            family,
            capacity,
            cache_resource_id: None,
        }
    }
}
pub mod cache_profile {
    use super::*;
    #[doc = "Cache family"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Family")]
    pub enum Family {
        C,
        P,
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
                Self::C => serializer.serialize_unit_variant("Family", 0u32, "C"),
                Self::P => serializer.serialize_unit_variant("Family", 1u32, "P"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Workload database profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseProfile {
    #[doc = "Database type"]
    #[serde(rename = "type")]
    pub type_: database_profile::Type,
    #[doc = "Database server name"]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "Database version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The name of the server SKU, e.g. Standard_D32s_v4"]
    pub sku: String,
    #[doc = "Tier of the server SKU"]
    pub tier: database_profile::Tier,
    #[doc = "Whether to enable HA for the server"]
    #[serde(rename = "haEnabled", default, skip_serializing_if = "Option::is_none")]
    pub ha_enabled: Option<database_profile::HaEnabled>,
    #[doc = "SKU name for database storage"]
    #[serde(rename = "storageSku", default, skip_serializing_if = "Option::is_none")]
    pub storage_sku: Option<String>,
    #[doc = "Database storage size in GB"]
    #[serde(rename = "storageInGB", default, skip_serializing_if = "Option::is_none")]
    pub storage_in_gb: Option<i64>,
    #[doc = "Storage IOPS for the server"]
    #[serde(rename = "storageIops", default, skip_serializing_if = "Option::is_none")]
    pub storage_iops: Option<i64>,
    #[doc = "Backup retention days for the server"]
    #[serde(rename = "backupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub backup_retention_days: Option<i32>,
    #[doc = "Whether to enable SSL enforcement on the database"]
    #[serde(rename = "sslEnforcementEnabled", default, skip_serializing_if = "Option::is_none")]
    pub ssl_enforcement_enabled: Option<database_profile::SslEnforcementEnabled>,
    #[doc = "Azure Database Server resource Id"]
    #[serde(rename = "serverResourceId", default, skip_serializing_if = "Option::is_none")]
    pub server_resource_id: Option<String>,
}
impl DatabaseProfile {
    pub fn new(type_: database_profile::Type, sku: String, tier: database_profile::Tier) -> Self {
        Self {
            type_,
            server_name: None,
            version: None,
            sku,
            tier,
            ha_enabled: None,
            storage_sku: None,
            storage_in_gb: None,
            storage_iops: None,
            backup_retention_days: None,
            ssl_enforcement_enabled: None,
            server_resource_id: None,
        }
    }
}
pub mod database_profile {
    use super::*;
    #[doc = "Database type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        MySql,
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
                Self::MySql => serializer.serialize_unit_variant("Type", 0u32, "MySql"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Tier of the server SKU"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Burstable,
        GeneralPurpose,
        MemoryOptimized,
    }
    #[doc = "Whether to enable HA for the server"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HaEnabled")]
    pub enum HaEnabled {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HaEnabled {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HaEnabled {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HaEnabled {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("HaEnabled", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("HaEnabled", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether to enable SSL enforcement on the database"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SslEnforcementEnabled")]
    pub enum SslEnforcementEnabled {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SslEnforcementEnabled {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SslEnforcementEnabled {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SslEnforcementEnabled {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("SslEnforcementEnabled", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("SslEnforcementEnabled", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Disk resource creation details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskInfo {
    #[doc = "Storage type"]
    #[serde(rename = "storageType")]
    pub storage_type: disk_info::StorageType,
    #[doc = "Disk size in GB"]
    #[serde(rename = "sizeInGB", default, skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<i64>,
}
impl DiskInfo {
    pub fn new(storage_type: disk_info::StorageType) -> Self {
        Self {
            storage_type,
            size_in_gb: None,
        }
    }
}
pub mod disk_info {
    use super::*;
    #[doc = "Storage type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StorageType {
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "StandardSSD_LRS")]
        StandardSsdLrs,
    }
}
#[doc = "File share profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileshareProfile {
    #[doc = "Share type"]
    #[serde(rename = "shareType")]
    pub share_type: fileshare_profile::ShareType,
    #[doc = "File share backing storage type"]
    #[serde(rename = "storageType")]
    pub storage_type: fileshare_profile::StorageType,
    #[doc = "File share size in GB"]
    #[serde(rename = "shareSizeInGB", default, skip_serializing_if = "Option::is_none")]
    pub share_size_in_gb: Option<i64>,
    #[doc = "File share storage resource id"]
    #[serde(rename = "storageResourceId", default, skip_serializing_if = "Option::is_none")]
    pub storage_resource_id: Option<String>,
    #[doc = "File share name"]
    #[serde(rename = "shareName", default, skip_serializing_if = "Option::is_none")]
    pub share_name: Option<String>,
}
impl FileshareProfile {
    pub fn new(share_type: fileshare_profile::ShareType, storage_type: fileshare_profile::StorageType) -> Self {
        Self {
            share_type,
            storage_type,
            share_size_in_gb: None,
            storage_resource_id: None,
            share_name: None,
        }
    }
}
pub mod fileshare_profile {
    use super::*;
    #[doc = "Share type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ShareType")]
    pub enum ShareType {
        NfsOnController,
        AzureFiles,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ShareType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ShareType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ShareType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NfsOnController => serializer.serialize_unit_variant("ShareType", 0u32, "NfsOnController"),
                Self::AzureFiles => serializer.serialize_unit_variant("ShareType", 1u32, "AzureFiles"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "File share backing storage type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageType")]
    pub enum StorageType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Standard_GRS")]
        StandardGrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardLrs => serializer.serialize_unit_variant("StorageType", 0u32, "Standard_LRS"),
                Self::StandardGrs => serializer.serialize_unit_variant("StorageType", 1u32, "Standard_GRS"),
                Self::StandardZrs => serializer.serialize_unit_variant("StorageType", 2u32, "Standard_ZRS"),
                Self::PremiumLrs => serializer.serialize_unit_variant("StorageType", 3u32, "Premium_LRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Network profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkProfile {
    #[doc = "Load balancer type"]
    #[serde(rename = "loadBalancerType")]
    pub load_balancer_type: network_profile::LoadBalancerType,
    #[doc = "Load balancer SKU"]
    #[serde(rename = "loadBalancerSku", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_sku: Option<String>,
    #[doc = "Load balancer tier"]
    #[serde(rename = "loadBalancerTier", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_tier: Option<String>,
    #[doc = "Capacity, applicable only for Application Gateway"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[doc = "Whether to enable Azure front door"]
    #[serde(rename = "azureFrontDoorEnabled", default, skip_serializing_if = "Option::is_none")]
    pub azure_front_door_enabled: Option<network_profile::AzureFrontDoorEnabled>,
    #[doc = "Virtual network resource Id"]
    #[serde(rename = "vNetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub v_net_resource_id: Option<String>,
    #[doc = "Azure Loadbalancer or ApplicationGateway resource Id"]
    #[serde(rename = "loadBalancerResourceId", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_resource_id: Option<String>,
    #[doc = "Azure front door resource id"]
    #[serde(rename = "azureFrontDoorResourceId", default, skip_serializing_if = "Option::is_none")]
    pub azure_front_door_resource_id: Option<String>,
    #[doc = "Loadbalancer front-end IP address resource Id"]
    #[serde(rename = "frontEndPublicIpResourceId", default, skip_serializing_if = "Option::is_none")]
    pub front_end_public_ip_resource_id: Option<String>,
    #[doc = "List of outbound public IP resource IDs"]
    #[serde(rename = "outboundPublicIpResourceIds", default, skip_serializing_if = "Vec::is_empty")]
    pub outbound_public_ip_resource_ids: Vec<String>,
}
impl NetworkProfile {
    pub fn new(load_balancer_type: network_profile::LoadBalancerType) -> Self {
        Self {
            load_balancer_type,
            load_balancer_sku: None,
            load_balancer_tier: None,
            capacity: None,
            azure_front_door_enabled: None,
            v_net_resource_id: None,
            load_balancer_resource_id: None,
            azure_front_door_resource_id: None,
            front_end_public_ip_resource_id: None,
            outbound_public_ip_resource_ids: Vec::new(),
        }
    }
}
pub mod network_profile {
    use super::*;
    #[doc = "Load balancer type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LoadBalancerType")]
    pub enum LoadBalancerType {
        ApplicationGateway,
        LoadBalancer,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LoadBalancerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LoadBalancerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LoadBalancerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ApplicationGateway => serializer.serialize_unit_variant("LoadBalancerType", 0u32, "ApplicationGateway"),
                Self::LoadBalancer => serializer.serialize_unit_variant("LoadBalancerType", 1u32, "LoadBalancer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether to enable Azure front door"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AzureFrontDoorEnabled")]
    pub enum AzureFrontDoorEnabled {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AzureFrontDoorEnabled {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AzureFrontDoorEnabled {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AzureFrontDoorEnabled {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("AzureFrontDoorEnabled", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("AzureFrontDoorEnabled", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "VM or VMSS node profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeProfile {
    #[doc = "VM or VMSS name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "VM SKU for node(s)"]
    #[serde(rename = "nodeSku")]
    pub node_sku: String,
    #[doc = "OS image profile"]
    #[serde(rename = "osImage")]
    pub os_image: OsImageProfile,
    #[doc = "Disk resource creation details"]
    #[serde(rename = "osDisk")]
    pub os_disk: DiskInfo,
    #[doc = "Data disks details. This property is not in use right now"]
    #[serde(rename = "dataDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disks: Vec<DiskInfo>,
    #[doc = "VM/VMSS resource ARM Ids"]
    #[serde(rename = "nodeResourceIds", default, skip_serializing_if = "Vec::is_empty")]
    pub node_resource_ids: Vec<String>,
}
impl NodeProfile {
    pub fn new(node_sku: String, os_image: OsImageProfile, os_disk: DiskInfo) -> Self {
        Self {
            name: None,
            node_sku,
            os_image,
            os_disk,
            data_disks: Vec::new(),
            node_resource_ids: Vec::new(),
        }
    }
}
#[doc = "OS image profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsImageProfile {
    #[doc = "OS image publisher"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<os_image_profile::Publisher>,
    #[doc = "OS image offer"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<os_image_profile::Offer>,
    #[doc = "OS image sku"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<os_image_profile::Sku>,
    #[doc = "OS image version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<os_image_profile::Version>,
}
impl OsImageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod os_image_profile {
    use super::*;
    #[doc = "OS image publisher"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Publisher")]
    pub enum Publisher {
        Canonical,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Publisher {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Publisher {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Publisher {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Canonical => serializer.serialize_unit_variant("Publisher", 0u32, "Canonical"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "OS image offer"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Offer")]
    pub enum Offer {
        UbuntuServer,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Offer {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Offer {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Offer {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::UbuntuServer => serializer.serialize_unit_variant("Offer", 0u32, "UbuntuServer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "OS image sku"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Sku")]
    pub enum Sku {
        #[serde(rename = "18.04-LTS")]
        N18_04_LTS,
        #[serde(rename = "16.04-LTS")]
        N16_04_LTS,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Sku {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Sku {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Sku {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N18_04_LTS => serializer.serialize_unit_variant("Sku", 0u32, "18.04-LTS"),
                Self::N16_04_LTS => serializer.serialize_unit_variant("Sku", 1u32, "16.04-LTS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "OS image version"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Version")]
    pub enum Version {
        #[serde(rename = "latest")]
        Latest,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Version {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Version {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Version {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Latest => serializer.serialize_unit_variant("Version", 0u32, "latest"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "PHP profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhpProfile {
    #[doc = "PHP version"]
    pub version: php_profile::Version,
}
impl PhpProfile {
    pub fn new(version: php_profile::Version) -> Self {
        Self { version }
    }
}
pub mod php_profile {
    use super::*;
    #[doc = "PHP version"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Version")]
    pub enum Version {
        #[serde(rename = "7.2")]
        N7_2,
        #[serde(rename = "7.3")]
        N7_3,
        #[serde(rename = "7.4")]
        N7_4,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Version {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Version {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Version {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N7_2 => serializer.serialize_unit_variant("Version", 0u32, "7.2"),
                Self::N7_3 => serializer.serialize_unit_variant("Version", 1u32, "7.3"),
                Self::N7_4 => serializer.serialize_unit_variant("Version", 2u32, "7.4"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Php workload resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhpWorkloadResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "PHP workload resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PhpWorkloadResourceProperties>,
    #[doc = "Indicates which kind of php workload this resource represent e.g WordPress"]
    pub kind: php_workload_resource::Kind,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Identity for the resource. Currently not supported"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<serde_json::Value>,
}
impl PhpWorkloadResource {
    pub fn new(tracked_resource: TrackedResource, kind: php_workload_resource::Kind) -> Self {
        Self {
            tracked_resource,
            properties: None,
            kind,
            sku: None,
            identity: None,
        }
    }
}
pub mod php_workload_resource {
    use super::*;
    #[doc = "Indicates which kind of php workload this resource represent e.g WordPress"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        WordPress,
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
                Self::WordPress => serializer.serialize_unit_variant("Kind", 0u32, "WordPress"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Php workload resource list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PhpWorkloadResourceList {
    #[doc = "List of resources in current page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PhpWorkloadResource>,
    #[doc = "Link to next page of resources"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PhpWorkloadResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PhpWorkloadResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "PHP workload resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhpWorkloadResourceProperties {
    #[doc = "The infra resources for PHP workload will be created in this location"]
    #[serde(rename = "appLocation")]
    pub app_location: String,
    #[doc = "Managed resource group configuration"]
    #[serde(rename = "managedResourceGroupConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_configuration: Option<ManagedRgConfiguration>,
    #[doc = "User profile to configure on a compute resources such as VM, VMSS"]
    #[serde(rename = "adminUserProfile")]
    pub admin_user_profile: UserProfile,
    #[doc = "VMSS profile"]
    #[serde(rename = "webNodesProfile")]
    pub web_nodes_profile: VmssNodesProfile,
    #[doc = "VM or VMSS node profile"]
    #[serde(rename = "controllerProfile")]
    pub controller_profile: NodeProfile,
    #[doc = "Network profile"]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
    #[doc = "Workload database profile"]
    #[serde(rename = "databaseProfile")]
    pub database_profile: DatabaseProfile,
    #[doc = "Workload website profile"]
    #[serde(rename = "siteProfile", default, skip_serializing_if = "Option::is_none")]
    pub site_profile: Option<SiteProfile>,
    #[doc = "File share profile"]
    #[serde(rename = "fileshareProfile", default, skip_serializing_if = "Option::is_none")]
    pub fileshare_profile: Option<FileshareProfile>,
    #[doc = "PHP profile"]
    #[serde(rename = "phpProfile", default, skip_serializing_if = "Option::is_none")]
    pub php_profile: Option<PhpProfile>,
    #[doc = "Search profile"]
    #[serde(rename = "searchProfile", default, skip_serializing_if = "Option::is_none")]
    pub search_profile: Option<SearchProfile>,
    #[doc = "Cache profile"]
    #[serde(rename = "cacheProfile", default, skip_serializing_if = "Option::is_none")]
    pub cache_profile: Option<CacheProfile>,
    #[doc = "Backup profile"]
    #[serde(rename = "backupProfile", default, skip_serializing_if = "Option::is_none")]
    pub backup_profile: Option<BackupProfile>,
    #[doc = "Php workload resource provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<php_workload_resource_properties::ProvisioningState>,
}
impl PhpWorkloadResourceProperties {
    pub fn new(
        app_location: String,
        admin_user_profile: UserProfile,
        web_nodes_profile: VmssNodesProfile,
        controller_profile: NodeProfile,
        database_profile: DatabaseProfile,
    ) -> Self {
        Self {
            app_location,
            managed_resource_group_configuration: None,
            admin_user_profile,
            web_nodes_profile,
            controller_profile,
            network_profile: None,
            database_profile,
            site_profile: None,
            fileshare_profile: None,
            php_profile: None,
            search_profile: None,
            cache_profile: None,
            backup_profile: None,
            provisioning_state: None,
        }
    }
}
pub mod php_workload_resource_properties {
    use super::*;
    #[doc = "Php workload resource provisioning state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        NotSpecified,
        Accepted,
        Created,
        Succeeded,
        Failed,
        Canceled,
        Provisioning,
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
                Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 0u32, "NotSpecified"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Accepted"),
                Self::Created => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Created"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Provisioning"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Search profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchProfile {
    #[serde(flatten)]
    pub node_profile: NodeProfile,
    #[doc = "Search type"]
    #[serde(rename = "searchType")]
    pub search_type: search_profile::SearchType,
}
impl SearchProfile {
    pub fn new(node_profile: NodeProfile, search_type: search_profile::SearchType) -> Self {
        Self { node_profile, search_type }
    }
}
pub mod search_profile {
    use super::*;
    #[doc = "Search type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SearchType")]
    pub enum SearchType {
        Elastic,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SearchType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SearchType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SearchType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Elastic => serializer.serialize_unit_variant("SearchType", 0u32, "Elastic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Workload website profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteProfile {
    #[doc = "Domain name for the application site URL"]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}
impl SiteProfile {
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
#[doc = "User profile to configure on a compute resources such as VM, VMSS"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserProfile {
    #[doc = "User name"]
    #[serde(rename = "userName")]
    pub user_name: String,
    #[doc = "SSH public key data"]
    #[serde(rename = "sshPublicKey")]
    pub ssh_public_key: String,
}
impl UserProfile {
    pub fn new(user_name: String, ssh_public_key: String) -> Self {
        Self { user_name, ssh_public_key }
    }
}
#[doc = "VMSS profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmssNodesProfile {
    #[serde(flatten)]
    pub node_profile: NodeProfile,
    #[doc = "Minimum number of nodes for autoscale"]
    #[serde(rename = "autoScaleMinCount", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale_min_count: Option<i32>,
    #[doc = "Maximum number of nodes for autoscale"]
    #[serde(rename = "autoScaleMaxCount", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale_max_count: Option<i32>,
}
impl VmssNodesProfile {
    pub fn new(node_profile: NodeProfile) -> Self {
        Self {
            node_profile,
            auto_scale_min_count: None,
            auto_scale_max_count: None,
        }
    }
}
#[doc = "WordPress instance resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WordpressInstanceResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "WordPress instance properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WordpressInstanceResourceProperties>,
}
impl WordpressInstanceResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "WordPress instance resource list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WordpressInstanceResourceList {
    #[doc = "List of resources in current page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WordpressInstanceResource>,
    #[doc = "Link to next page of resources"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WordpressInstanceResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WordpressInstanceResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "WordPress instance properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WordpressInstanceResourceProperties {
    #[doc = "Application version"]
    pub version: wordpress_instance_resource_properties::Version,
    #[doc = "Database name used by the application"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "User name used by the application to connect to database"]
    #[serde(rename = "databaseUser", default, skip_serializing_if = "Option::is_none")]
    pub database_user: Option<String>,
    #[doc = "Site Url to access the WordPress application"]
    #[serde(rename = "siteUrl", default, skip_serializing_if = "Option::is_none")]
    pub site_url: Option<String>,
    #[doc = "WordPress instance provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<wordpress_instance_resource_properties::ProvisioningState>,
}
impl WordpressInstanceResourceProperties {
    pub fn new(version: wordpress_instance_resource_properties::Version) -> Self {
        Self {
            version,
            database_name: None,
            database_user: None,
            site_url: None,
            provisioning_state: None,
        }
    }
}
pub mod wordpress_instance_resource_properties {
    use super::*;
    #[doc = "Application version"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Version")]
    pub enum Version {
        #[serde(rename = "5.4.3")]
        N5_4_3,
        #[serde(rename = "5.4.2")]
        N5_4_2,
        #[serde(rename = "5.4.1")]
        N5_4_1,
        #[serde(rename = "5.4")]
        N5_4,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Version {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Version {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Version {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N5_4_3 => serializer.serialize_unit_variant("Version", 0u32, "5.4.3"),
                Self::N5_4_2 => serializer.serialize_unit_variant("Version", 1u32, "5.4.2"),
                Self::N5_4_1 => serializer.serialize_unit_variant("Version", 2u32, "5.4.1"),
                Self::N5_4 => serializer.serialize_unit_variant("Version", 3u32, "5.4"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "WordPress instance provisioning state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        NotSpecified,
        Accepted,
        Created,
        Succeeded,
        Failed,
        Canceled,
        Installing,
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
                Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 0u32, "NotSpecified"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Accepted"),
                Self::Created => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Created"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::Installing => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Installing"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
