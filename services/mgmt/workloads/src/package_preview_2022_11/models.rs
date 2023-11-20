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
#[doc = "The full resource names object for application layer resources. The number of entries in this list should be equal to the number VMs to be created for application layer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationServerFullResourceNames {
    #[doc = "The list of virtual machine naming details."]
    #[serde(
        rename = "virtualMachines",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub virtual_machines: Vec<VirtualMachineResourceNames>,
    #[doc = "The full name for availability set. In case name is not provided, it will be defaulted to {SID}-App-AvSet."]
    #[serde(rename = "availabilitySetName", default, skip_serializing_if = "Option::is_none")]
    pub availability_set_name: Option<String>,
}
impl ApplicationServerFullResourceNames {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the type of application server VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApplicationServerVirtualMachineType")]
pub enum ApplicationServerVirtualMachineType {
    Active,
    Standby,
    Unknown,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApplicationServerVirtualMachineType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApplicationServerVirtualMachineType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApplicationServerVirtualMachineType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("ApplicationServerVirtualMachineType", 0u32, "Active"),
            Self::Standby => serializer.serialize_unit_variant("ApplicationServerVirtualMachineType", 1u32, "Standby"),
            Self::Unknown => serializer.serialize_unit_variant("ApplicationServerVirtualMachineType", 2u32, "Unknown"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The Application Server VM Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationServerVmDetails {
    #[doc = "Defines the type of application server VM."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ApplicationServerVirtualMachineType>,
    #[serde(rename = "virtualMachineId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_id: Option<String>,
    #[doc = "Storage details of all the Storage Accounts attached to the App Virtual Machine. For e.g. NFS on AFS Shared Storage."]
    #[serde(
        rename = "storageDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub storage_details: Vec<StorageInformation>,
}
impl ApplicationServerVmDetails {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The full resource names object for central server layer resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CentralServerFullResourceNames {
    #[doc = "The list of names for all ASCS virtual machines to be deployed. The number of entries in this list should be equal to the number VMs to be created for ASCS layer. At maximum, there can be two virtual machines at this layer: ASCS and ERS."]
    #[serde(
        rename = "virtualMachines",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub virtual_machines: Vec<VirtualMachineResourceNames>,
    #[doc = "The full name for availability set. In case name is not provided, it will be defaulted to {SID}-ASCS-AvSet."]
    #[serde(rename = "availabilitySetName", default, skip_serializing_if = "Option::is_none")]
    pub availability_set_name: Option<String>,
    #[doc = "The resource names object for load balancer and related resources."]
    #[serde(rename = "loadBalancer", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<LoadBalancerResourceNames>,
}
impl CentralServerFullResourceNames {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The SAP Central Services Instance VM details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CentralServerVmDetails {
    #[doc = "Defines the type of central server VM."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<CentralServerVirtualMachineType>,
    #[serde(rename = "virtualMachineId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_id: Option<String>,
    #[doc = "Storage details of all the Storage Accounts attached to the ASCS Virtual Machine. For e.g. NFS on AFS Shared Storage."]
    #[serde(
        rename = "storageDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub storage_details: Vec<StorageInformation>,
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
#[doc = "Gets or sets the file share configuration for file share created with the VIS case."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAndMountFileShareConfiguration {
    #[doc = "The name of file share resource group. The app rg is used in case of missing input."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "The name of file share storage account name . A custom name is used in case of missing input."]
    #[serde(rename = "storageAccountName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_name: Option<String>,
}
impl CreateAndMountFileShareConfiguration {
    pub fn new() -> Self {
        Self {
            resource_group: None,
            storage_account_name: None,
        }
    }
}
#[doc = "Gets or sets the DB2 provider properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Db2ProviderInstanceProperties {
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
    #[doc = "Gets or sets certificate preference if secure communication is enabled."]
    #[serde(rename = "sslPreference", default, skip_serializing_if = "Option::is_none")]
    pub ssl_preference: Option<SslPreference>,
    #[doc = "Gets or sets the blob URI to SSL certificate for the DB2 Database."]
    #[serde(rename = "sslCertificateUri", default, skip_serializing_if = "Option::is_none")]
    pub ssl_certificate_uri: Option<String>,
}
impl Db2ProviderInstanceProperties {
    pub fn new() -> Self {
        Self {
            hostname: None,
            db_name: None,
            db_port: None,
            db_username: None,
            db_password: None,
            db_password_uri: None,
            sap_sid: None,
            ssl_preference: None,
            ssl_certificate_uri: None,
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
    #[doc = "The Disk Configuration Details."]
    #[serde(rename = "diskConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub disk_configuration: Option<DiskConfiguration>,
}
impl DatabaseConfiguration {
    pub fn new(subnet_id: String, virtual_machine_configuration: VirtualMachineConfiguration, instance_count: i64) -> Self {
        Self {
            database_type: None,
            subnet_id,
            virtual_machine_configuration,
            instance_count,
            disk_configuration: None,
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
#[doc = "The full resource names object for database layer resources. The number of entries in this list should be equal to the number VMs to be created for database layer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseServerFullResourceNames {
    #[doc = "The list of virtual machine naming details."]
    #[serde(
        rename = "virtualMachines",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub virtual_machines: Vec<VirtualMachineResourceNames>,
    #[doc = "The full name for availability set. In case name is not provided, it will be defaulted to {SID}-DB-AvSet."]
    #[serde(rename = "availabilitySetName", default, skip_serializing_if = "Option::is_none")]
    pub availability_set_name: Option<String>,
    #[doc = "The resource names object for load balancer and related resources."]
    #[serde(rename = "loadBalancer", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<LoadBalancerResourceNames>,
}
impl DatabaseServerFullResourceNames {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Database VM details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseVmDetails {
    #[serde(rename = "virtualMachineId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_id: Option<String>,
    #[doc = "Defines the SAP Instance status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SapVirtualInstanceStatus>,
    #[doc = "Storage details of all the Storage Accounts attached to the Database Virtual Machine. For e.g. NFS on AFS Shared Storage."]
    #[serde(
        rename = "storageDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub storage_details: Vec<StorageInformation>,
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
    #[doc = "The geo-location where the SAP system is to be created."]
    #[serde(rename = "appLocation", default, skip_serializing_if = "Option::is_none")]
    pub app_location: Option<String>,
    #[doc = "Deploy SAP Infrastructure Details."]
    #[serde(rename = "infrastructureConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub infrastructure_configuration: Option<InfrastructureConfigurationUnion>,
    #[doc = "The SAP Software configuration Input."]
    #[serde(rename = "softwareConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub software_configuration: Option<SoftwareConfigurationUnion>,
}
impl DeploymentConfiguration {
    pub fn new() -> Self {
        Self {
            app_location: None,
            infrastructure_configuration: None,
            software_configuration: None,
        }
    }
}
#[doc = "The type of SAP deployment, single server or Three tier."]
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
    #[doc = "The geo-location where the SAP system is to be created."]
    #[serde(rename = "appLocation", default, skip_serializing_if = "Option::is_none")]
    pub app_location: Option<String>,
    #[doc = "Deploy SAP Infrastructure Details."]
    #[serde(rename = "infrastructureConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub infrastructure_configuration: Option<InfrastructureConfigurationUnion>,
    #[doc = "The SAP Software configuration Input."]
    #[serde(rename = "softwareConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub software_configuration: Option<SoftwareConfigurationUnion>,
    #[doc = "Defines the OS and SAP Configurations for Deployment"]
    #[serde(rename = "osSapConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub os_sap_configuration: Option<OsSapConfiguration>,
}
impl DeploymentWithOsConfiguration {
    pub fn new() -> Self {
        Self {
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
    #[doc = "The virtual machine ID of the Central Server."]
    #[serde(rename = "centralServerVmId", default, skip_serializing_if = "Option::is_none")]
    pub central_server_vm_id: Option<String>,
    #[doc = "The custom storage account name for the storage account created by the service in the managed resource group created as part of VIS deployment.<br><br>Refer to the storage account naming rules [here](https://learn.microsoft.com/azure/azure-resource-manager/management/resource-name-rules#microsoftstorage).<br><br>If not provided, the service will create the storage account with a random name."]
    #[serde(rename = "managedRgStorageAccountName", default, skip_serializing_if = "Option::is_none")]
    pub managed_rg_storage_account_name: Option<String>,
    #[doc = "The geo-location where the SAP system exists."]
    #[serde(rename = "appLocation", default, skip_serializing_if = "Option::is_none")]
    pub app_location: Option<String>,
}
impl DiscoveryConfiguration {
    pub fn new() -> Self {
        Self {
            central_server_vm_id: None,
            managed_rg_storage_account_name: None,
            app_location: None,
        }
    }
}
#[doc = "The Disk Configuration Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskConfiguration {
    #[doc = "The disk configuration for the db volume. For HANA, Required volumes are: ['hana/data', 'hana/log', hana/shared', 'usr/sap', 'os'], Optional volume : ['backup']."]
    #[serde(rename = "diskVolumeConfigurations", default, skip_serializing_if = "Option::is_none")]
    pub disk_volume_configurations: Option<serde_json::Value>,
}
impl DiskConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The supported disk size details for a disk type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskDetails {
    #[doc = "The disk sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<DiskSku>,
    #[doc = "The disk size in GB."]
    #[serde(rename = "sizeGB", default, skip_serializing_if = "Option::is_none")]
    pub size_gb: Option<i64>,
    #[doc = "The minimum supported disk count."]
    #[serde(rename = "minimumSupportedDiskCount", default, skip_serializing_if = "Option::is_none")]
    pub minimum_supported_disk_count: Option<i64>,
    #[doc = "The maximum supported disk count."]
    #[serde(rename = "maximumSupportedDiskCount", default, skip_serializing_if = "Option::is_none")]
    pub maximum_supported_disk_count: Option<i64>,
    #[doc = "The disk Iops."]
    #[serde(rename = "iopsReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub iops_read_write: Option<i64>,
    #[doc = "The disk provisioned throughput in MBps."]
    #[serde(rename = "mbpsReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub mbps_read_write: Option<i64>,
    #[doc = "The disk tier, e.g. P10, E10."]
    #[serde(rename = "diskTier", default, skip_serializing_if = "Option::is_none")]
    pub disk_tier: Option<String>,
}
impl DiskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The disk sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskSku {
    #[doc = "Defines the disk sku name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<DiskSkuName>,
}
impl DiskSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the disk sku name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DiskSkuName")]
pub enum DiskSkuName {
    #[serde(rename = "Standard_LRS")]
    StandardLrs,
    #[serde(rename = "Premium_LRS")]
    PremiumLrs,
    #[serde(rename = "StandardSSD_LRS")]
    StandardSsdLrs,
    #[serde(rename = "UltraSSD_LRS")]
    UltraSsdLrs,
    #[serde(rename = "Premium_ZRS")]
    PremiumZrs,
    #[serde(rename = "StandardSSD_ZRS")]
    StandardSsdZrs,
    #[serde(rename = "PremiumV2_LRS")]
    PremiumV2Lrs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DiskSkuName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DiskSkuName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DiskSkuName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::StandardLrs => serializer.serialize_unit_variant("DiskSkuName", 0u32, "Standard_LRS"),
            Self::PremiumLrs => serializer.serialize_unit_variant("DiskSkuName", 1u32, "Premium_LRS"),
            Self::StandardSsdLrs => serializer.serialize_unit_variant("DiskSkuName", 2u32, "StandardSSD_LRS"),
            Self::UltraSsdLrs => serializer.serialize_unit_variant("DiskSkuName", 3u32, "UltraSSD_LRS"),
            Self::PremiumZrs => serializer.serialize_unit_variant("DiskSkuName", 4u32, "Premium_ZRS"),
            Self::StandardSsdZrs => serializer.serialize_unit_variant("DiskSkuName", 5u32, "StandardSSD_ZRS"),
            Self::PremiumV2Lrs => serializer.serialize_unit_variant("DiskSkuName", 6u32, "PremiumV2_LRS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The disk configuration required for the selected volume."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskVolumeConfiguration {
    #[doc = "The total number of disks required for the concerned volume."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "The disk size in GB."]
    #[serde(rename = "sizeGB", default, skip_serializing_if = "Option::is_none")]
    pub size_gb: Option<i64>,
    #[doc = "The disk sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<DiskSku>,
}
impl DiskVolumeConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the SAP Enqueue Replication Server (ERS) properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnqueueReplicationServerProperties {
    #[doc = "Defines the type of Enqueue Replication Server."]
    #[serde(rename = "ersVersion", default, skip_serializing_if = "Option::is_none")]
    pub ers_version: Option<EnqueueReplicationServerType>,
    #[doc = "ERS Instance Number."]
    #[serde(rename = "instanceNo", default, skip_serializing_if = "Option::is_none")]
    pub instance_no: Option<String>,
    #[doc = "ERS SAP Hostname."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[doc = "ERS SAP Kernel Version."]
    #[serde(rename = "kernelVersion", default, skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    #[doc = "ERS SAP Kernel Patch level."]
    #[serde(rename = "kernelPatch", default, skip_serializing_if = "Option::is_none")]
    pub kernel_patch: Option<String>,
    #[doc = "ERS SAP IP Address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "Defines the health of SAP Instances."]
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
#[doc = "Defines the SAP Enqueue Server properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnqueueServerProperties {
    #[doc = "Enqueue Server SAP Hostname."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[doc = "Enqueue Server SAP IP Address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "Enqueue Server Port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[doc = "Defines the health of SAP Instances."]
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
        pub inner_error: Option<Box<Error>>,
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "The SAP Software configuration Input when the software is installed externally outside the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalInstallationSoftwareConfiguration {
    #[doc = "The resource ID of the virtual machine containing the central server instance."]
    #[serde(rename = "centralServerVmId", default, skip_serializing_if = "Option::is_none")]
    pub central_server_vm_id: Option<String>,
}
impl ExternalInstallationSoftwareConfiguration {
    pub fn new() -> Self {
        Self {
            central_server_vm_id: None,
        }
    }
}
#[doc = "The type of file share config."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "configurationType")]
pub enum FileShareConfigurationUnion {
    CreateAndMount(CreateAndMountFileShareConfiguration),
    Mount(MountFileShareConfiguration),
    Skip(SkipFileShareConfiguration),
}
#[doc = "The type of file share config."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FileShareConfigurationType")]
pub enum FileShareConfigurationType {
    Skip,
    CreateAndMount,
    Mount,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FileShareConfigurationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FileShareConfigurationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FileShareConfigurationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Skip => serializer.serialize_unit_variant("FileShareConfigurationType", 0u32, "Skip"),
            Self::CreateAndMount => serializer.serialize_unit_variant("FileShareConfigurationType", 1u32, "CreateAndMount"),
            Self::Mount => serializer.serialize_unit_variant("FileShareConfigurationType", 2u32, "Mount"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the SAP Gateway Server properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayServerProperties {
    #[doc = "Gateway Port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[doc = "Defines the health of SAP Instances."]
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
    #[serde(rename = "sslCertificateUri", default, skip_serializing_if = "Option::is_none")]
    pub ssl_certificate_uri: Option<String>,
    #[doc = "Gets or sets the hostname(s) in the SSL certificate."]
    #[serde(rename = "sslHostNameInCertificate", default, skip_serializing_if = "Option::is_none")]
    pub ssl_host_name_in_certificate: Option<String>,
    #[doc = "Gets or sets certificate preference if secure communication is enabled."]
    #[serde(rename = "sslPreference", default, skip_serializing_if = "Option::is_none")]
    pub ssl_preference: Option<SslPreference>,
    #[doc = "Gets or sets the SAP System Identifier."]
    #[serde(rename = "sapSid", default, skip_serializing_if = "Option::is_none")]
    pub sap_sid: Option<String>,
}
impl HanaDbProviderInstanceProperties {
    pub fn new() -> Self {
        Self {
            hostname: None,
            db_name: None,
            sql_port: None,
            instance_number: None,
            db_username: None,
            db_password: None,
            db_password_uri: None,
            ssl_certificate_uri: None,
            ssl_host_name_in_certificate: None,
            ssl_preference: None,
            sap_sid: None,
        }
    }
}
#[doc = "Defines the health of SAP Instances."]
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
    #[doc = "The application resource group where SAP system resources will be deployed."]
    #[serde(rename = "appResourceGroup")]
    pub app_resource_group: String,
}
impl InfrastructureConfiguration {
    pub fn new(app_resource_group: String) -> Self {
        Self { app_resource_group }
    }
}
#[doc = "The type of SAP deployment, single server or Three tier."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "deploymentType")]
pub enum InfrastructureConfigurationUnion {
    SingleServer(SingleServerConfiguration),
    ThreeTier(ThreeTierConfiguration),
}
#[doc = "Specifies the Linux operating system settings on the virtual machine. <br><br>For a list of supported Linux distributions, see [Linux on Azure-Endorsed Distributions](https://docs.microsoft.com/azure/virtual-machines/linux/endorsed-distros)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinuxConfiguration {
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
    pub fn new() -> Self {
        Self {
            disable_password_authentication: None,
            ssh: None,
            ssh_key_pair: None,
        }
    }
}
#[doc = "The Load Balancer details such as Load Balancer ID."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancerDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl LoadBalancerDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource names object for load balancer and related resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancerResourceNames {
    #[doc = "The full resource name for load balancer. If this value is not provided, load balancer will be name as {ASCS/DB}-loadBalancer."]
    #[serde(rename = "loadBalancerName", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    #[doc = "The list of frontend IP configuration names. If provided as input, size of this list should be 2 for cs layer and should be 1 for database layer."]
    #[serde(
        rename = "frontendIpConfigurationNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub frontend_ip_configuration_names: Vec<String>,
    #[doc = "The list of backend pool names. Currently, ACSS deploys only one backend pool and hence, size of this list should be 1"]
    #[serde(
        rename = "backendPoolNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub backend_pool_names: Vec<String>,
    #[doc = "The list of health probe names. If provided as input, size of this list should be 2 for cs layer and should be 1 for database layer."]
    #[serde(
        rename = "healthProbeNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub health_probe_names: Vec<String>,
}
impl LoadBalancerResourceNames {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Defines the SAP Message Server properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MessageServerProperties {
    #[doc = "Message Server port."]
    #[serde(rename = "msPort", default, skip_serializing_if = "Option::is_none")]
    pub ms_port: Option<i64>,
    #[doc = "Message Server internal MS port."]
    #[serde(rename = "internalMsPort", default, skip_serializing_if = "Option::is_none")]
    pub internal_ms_port: Option<i64>,
    #[doc = "Message Server HTTP Port."]
    #[serde(rename = "httpPort", default, skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,
    #[doc = "Message Server HTTPS Port."]
    #[serde(rename = "httpsPort", default, skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,
    #[doc = "Message Server SAP Hostname."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[doc = "Message server IP Address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "Defines the health of SAP Instances."]
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Monitor>,
    #[doc = "The URL to get the next set of SAP monitors."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MonitorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
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
    #[doc = "The ARM ID of the Storage account used for SAP monitoring."]
    #[serde(rename = "storageAccountArmId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_arm_id: Option<String>,
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
#[doc = "Gets or sets the file share configuration for externally mounted cases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MountFileShareConfiguration {
    #[doc = "The fileshare resource ID"]
    pub id: String,
    #[doc = "The private endpoint resource ID"]
    #[serde(rename = "privateEndpointId")]
    pub private_endpoint_id: String,
}
impl MountFileShareConfiguration {
    pub fn new(id: String, private_endpoint_id: String) -> Self {
        Self { id, private_endpoint_id }
    }
}
#[doc = "Gets or sets the SQL server provider properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MsSqlServerProviderInstanceProperties {
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
    #[doc = "Gets or sets certificate preference if secure communication is enabled."]
    #[serde(rename = "sslPreference", default, skip_serializing_if = "Option::is_none")]
    pub ssl_preference: Option<SslPreference>,
    #[doc = "Gets or sets the blob URI to SSL certificate for the SQL Database."]
    #[serde(rename = "sslCertificateUri", default, skip_serializing_if = "Option::is_none")]
    pub ssl_certificate_uri: Option<String>,
}
impl MsSqlServerProviderInstanceProperties {
    pub fn new() -> Self {
        Self {
            hostname: None,
            db_port: None,
            db_username: None,
            db_password: None,
            db_password_uri: None,
            sap_sid: None,
            ssl_preference: None,
            ssl_certificate_uri: None,
        }
    }
}
#[doc = "The pattern type to be used for resource naming."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NamingPatternType")]
pub enum NamingPatternType {
    FullResourceName,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NamingPatternType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NamingPatternType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NamingPatternType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::FullResourceName => serializer.serialize_unit_variant("NamingPatternType", 0u32, "FullResourceName"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the network configuration type for SAP system infrastructure that is being deployed "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkConfiguration {
    #[doc = "Specifies whether a secondary IP address should be added to the network interface on all VMs of the SAP system being deployed"]
    #[serde(rename = "isSecondaryIpEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_secondary_ip_enabled: Option<bool>,
}
impl NetworkConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource names object for network interface and related resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceResourceNames {
    #[doc = "The full name for network interface. If name is not provided, service uses a default name based on the deployment type. For SingleServer, default name is {SID}-Nic. In case of HA-AvZone systems, default name will be {SID}-{App/ASCS/DB}-Zone{A/B}-Nic with an incrementor at the end in case of more than 1 instance per layer. For distributed and HA-AvSet systems, default name will be {SID}-{App/ASCS/DB}-Nic with an incrementor at the end in case of more than 1 instance per layer."]
    #[serde(rename = "networkInterfaceName", default, skip_serializing_if = "Option::is_none")]
    pub network_interface_name: Option<String>,
}
impl NetworkInterfaceResourceNames {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The OS Type"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "osType")]
pub enum OsConfigurationUnion {
    Linux(LinuxConfiguration),
    Windows(WindowsConfiguration),
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
    pub os_configuration: Option<OsConfigurationUnion>,
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
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the operation."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The operations list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "Gets or sets the PrometheusHaCluster provider properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrometheusHaClusterProviderInstanceProperties {
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
    #[doc = "Gets or sets certificate preference if secure communication is enabled."]
    #[serde(rename = "sslPreference", default, skip_serializing_if = "Option::is_none")]
    pub ssl_preference: Option<SslPreference>,
    #[doc = "Gets or sets the blob URI to SSL certificate for the HA cluster exporter."]
    #[serde(rename = "sslCertificateUri", default, skip_serializing_if = "Option::is_none")]
    pub ssl_certificate_uri: Option<String>,
}
impl PrometheusHaClusterProviderInstanceProperties {
    pub fn new() -> Self {
        Self {
            prometheus_url: None,
            hostname: None,
            sid: None,
            cluster_name: None,
            ssl_preference: None,
            ssl_certificate_uri: None,
        }
    }
}
#[doc = "Gets or sets the PrometheusOS provider properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrometheusOsProviderInstanceProperties {
    #[doc = "URL of the Node Exporter endpoint"]
    #[serde(rename = "prometheusUrl", default, skip_serializing_if = "Option::is_none")]
    pub prometheus_url: Option<String>,
    #[doc = "Gets or sets certificate preference if secure communication is enabled."]
    #[serde(rename = "sslPreference", default, skip_serializing_if = "Option::is_none")]
    pub ssl_preference: Option<SslPreference>,
    #[doc = "Gets or sets the blob URI to SSL certificate for the prometheus node exporter."]
    #[serde(rename = "sslCertificateUri", default, skip_serializing_if = "Option::is_none")]
    pub ssl_certificate_uri: Option<String>,
    #[doc = "Gets or sets the SAP System Identifier"]
    #[serde(rename = "sapSid", default, skip_serializing_if = "Option::is_none")]
    pub sap_sid: Option<String>,
}
impl PrometheusOsProviderInstanceProperties {
    pub fn new() -> Self {
        Self {
            prometheus_url: None,
            ssl_preference: None,
            ssl_certificate_uri: None,
            sap_sid: None,
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProviderInstance>,
    #[doc = "The URL to get the next set of provider instances."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProviderInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
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
    pub provider_settings: Option<ProviderSpecificPropertiesUnion>,
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
#[doc = "The provider type. For example, the value can be SapHana."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "providerType")]
pub enum ProviderSpecificPropertiesUnion {
    Db2(Db2ProviderInstanceProperties),
    SapHana(HanaDbProviderInstanceProperties),
    MsSqlServer(MsSqlServerProviderInstanceProperties),
    PrometheusHaCluster(PrometheusHaClusterProviderInstanceProperties),
    #[serde(rename = "PrometheusOS")]
    PrometheusOs(PrometheusOsProviderInstanceProperties),
    SapNetWeaver(SapNetWeaverProviderInstanceProperties),
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
#[doc = "Define the SAP Application Server Instance resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapApplicationServerInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Defines the SAP Application Server instance properties."]
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
#[doc = "Defines the collection of SAP Application Server Instance resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapApplicationServerInstanceList {
    #[doc = "Gets the list of SAP Application Server instance resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SapApplicationServerInstance>,
    #[doc = "Gets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SapApplicationServerInstanceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SapApplicationServerInstanceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the SAP Application Server instance properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapApplicationServerProperties {
    #[doc = "Application server Instance Number."]
    #[serde(rename = "instanceNo", default, skip_serializing_if = "Option::is_none")]
    pub instance_no: Option<String>,
    #[doc = "Application server Subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    #[doc = "Application server instance SAP hostname."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[doc = " Application server instance SAP Kernel Version."]
    #[serde(rename = "kernelVersion", default, skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    #[doc = "Application server instance SAP Kernel Patch level."]
    #[serde(rename = "kernelPatch", default, skip_serializing_if = "Option::is_none")]
    pub kernel_patch: Option<String>,
    #[doc = " Application server instance SAP IP Address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "Application server instance gateway Port."]
    #[serde(rename = "gatewayPort", default, skip_serializing_if = "Option::is_none")]
    pub gateway_port: Option<i64>,
    #[doc = "Application server instance ICM HTTP Port."]
    #[serde(rename = "icmHttpPort", default, skip_serializing_if = "Option::is_none")]
    pub icm_http_port: Option<i64>,
    #[doc = "Application server instance ICM HTTPS Port."]
    #[serde(rename = "icmHttpsPort", default, skip_serializing_if = "Option::is_none")]
    pub icm_https_port: Option<i64>,
    #[doc = "The Load Balancer details such as Load Balancer ID."]
    #[serde(rename = "loadBalancerDetails", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_details: Option<LoadBalancerDetails>,
    #[doc = "The list of virtual machines."]
    #[serde(
        rename = "vmDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vm_details: Vec<ApplicationServerVmDetails>,
    #[doc = "Defines the SAP Instance status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SapVirtualInstanceStatus>,
    #[doc = "Defines the health of SAP Instances."]
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
    #[serde(
        rename = "availabilityZonePairs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "Defines the collection of SAP Central Services Instance resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapCentralInstanceList {
    #[doc = "Gets the list of SAP central services instance resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SapCentralServerInstance>,
    #[doc = "Gets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SapCentralInstanceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SapCentralInstanceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Define the SAP Central Services Instance resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapCentralServerInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Defines the SAP Central Services Instance properties."]
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
#[doc = "Defines the SAP Central Services Instance properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapCentralServerProperties {
    #[doc = "The central services instance number."]
    #[serde(rename = "instanceNo", default, skip_serializing_if = "Option::is_none")]
    pub instance_no: Option<String>,
    #[doc = "The central services instance subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    #[doc = "Defines the SAP Message Server properties."]
    #[serde(rename = "messageServerProperties", default, skip_serializing_if = "Option::is_none")]
    pub message_server_properties: Option<MessageServerProperties>,
    #[doc = "Defines the SAP Enqueue Server properties."]
    #[serde(rename = "enqueueServerProperties", default, skip_serializing_if = "Option::is_none")]
    pub enqueue_server_properties: Option<EnqueueServerProperties>,
    #[doc = "Defines the SAP Gateway Server properties."]
    #[serde(rename = "gatewayServerProperties", default, skip_serializing_if = "Option::is_none")]
    pub gateway_server_properties: Option<GatewayServerProperties>,
    #[doc = "Defines the SAP Enqueue Replication Server (ERS) properties."]
    #[serde(rename = "enqueueReplicationServerProperties", default, skip_serializing_if = "Option::is_none")]
    pub enqueue_replication_server_properties: Option<EnqueueReplicationServerProperties>,
    #[doc = "The central services instance Kernel Version."]
    #[serde(rename = "kernelVersion", default, skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    #[doc = "The central services instance Kernel Patch level."]
    #[serde(rename = "kernelPatch", default, skip_serializing_if = "Option::is_none")]
    pub kernel_patch: Option<String>,
    #[doc = "The Load Balancer details such as Load Balancer ID."]
    #[serde(rename = "loadBalancerDetails", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_details: Option<LoadBalancerDetails>,
    #[doc = "The list of virtual machines corresponding to the Central Services instance."]
    #[serde(
        rename = "vmDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vm_details: Vec<CentralServerVmDetails>,
    #[doc = "Defines the SAP Instance status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SapVirtualInstanceStatus>,
    #[doc = "Defines the health of SAP Instances."]
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
#[doc = "The configuration Type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "configurationType")]
pub enum SapConfigurationUnion {
    Deployment(DeploymentConfiguration),
    #[serde(rename = "DeploymentWithOSConfig")]
    DeploymentWithOsConfig(DeploymentWithOsConfiguration),
    Discovery(DiscoveryConfiguration),
}
#[doc = "Define the Database resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapDatabaseInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Defines the Database properties."]
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SapDatabaseInstance>,
    #[doc = "Gets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SapDatabaseInstanceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SapDatabaseInstanceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the Database properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapDatabaseProperties {
    #[doc = "Database subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    #[doc = "Database SID name."]
    #[serde(rename = "databaseSid", default, skip_serializing_if = "Option::is_none")]
    pub database_sid: Option<String>,
    #[doc = "Database type, that is if the DB is HANA, DB2, Oracle, SAP ASE, Max DB or MS SQL Server."]
    #[serde(rename = "databaseType", default, skip_serializing_if = "Option::is_none")]
    pub database_type: Option<String>,
    #[doc = "Database IP Address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The Load Balancer details such as Load Balancer ID."]
    #[serde(rename = "loadBalancerDetails", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_details: Option<LoadBalancerDetails>,
    #[doc = "The list of virtual machines corresponding to the Database resource."]
    #[serde(
        rename = "vmDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "The SAP Disk Configuration contains 'recommended disk' details and list of supported disks detail for a volume type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapDiskConfiguration {
    #[doc = "The disk configuration required for the selected volume."]
    #[serde(rename = "recommendedConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub recommended_configuration: Option<DiskVolumeConfiguration>,
    #[doc = "The list of supported disks for a given VM Sku."]
    #[serde(
        rename = "supportedConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_configurations: Vec<DiskDetails>,
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
    #[doc = "The type of SAP deployment, single server or Three tier."]
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
    #[doc = "The disk configuration for the db volume. For HANA, Required volumes are: ['hana/data', 'hana/log', hana/shared', 'usr/sap', 'os'], Optional volume : ['backup']."]
    #[serde(rename = "volumeConfigurations", default, skip_serializing_if = "Option::is_none")]
    pub volume_configurations: Option<serde_json::Value>,
}
impl SapDiskConfigurationsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SAP Software configuration Input when the software is to be installed by service without OS Configurations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapInstallWithoutOsConfigSoftwareConfiguration {
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
    pub fn new(bom_url: String, sap_bits_storage_account_id: String, software_version: String) -> Self {
        Self {
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
    #[doc = "The type of SAP deployment, single server or Three tier."]
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
#[doc = "The type of SAP deployment, single server or Three tier."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "deploymentType")]
pub enum SapSizingRecommendationResultUnion {
    SingleServer(SingleServerRecommendationResult),
    ThreeTier(ThreeTierRecommendationResult),
}
#[doc = "The SAP software installation Type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SapSoftwareInstallationType")]
pub enum SapSoftwareInstallationType {
    ServiceInitiated,
    #[serde(rename = "SAPInstallWithoutOSConfig")]
    SapInstallWithoutOsConfig,
    External,
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
            Self::External => serializer.serialize_unit_variant("SapSoftwareInstallationType", 2u32, "External"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The list of supported SKUs for different resources which are part of SAP deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapSupportedResourceSkusResult {
    #[doc = "Gets the list of SAP supported SKUs."]
    #[serde(
        rename = "supportedSkus",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[doc = "The type of SAP deployment, single server or Three tier."]
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
#[doc = "Define the Virtual Instance for SAP solutions resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapVirtualInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Managed service identity (user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<UserAssignedServiceIdentity>,
    #[doc = "Defines the Virtual Instance for SAP solutions resource properties."]
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
#[doc = "Defines the collection of Virtual Instance for SAP solutions resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapVirtualInstanceList {
    #[doc = "Gets the list of Virtual Instances for SAP solutions resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SapVirtualInstance>,
    #[doc = "Gets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SapVirtualInstanceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SapVirtualInstanceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the Virtual Instance for SAP solutions resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapVirtualInstanceProperties {
    #[doc = "Defines the environment type - Production/Non Production."]
    pub environment: EnvironmentType,
    #[doc = "Defines the SAP Product type."]
    #[serde(rename = "sapProduct")]
    pub sap_product: SapProductType,
    #[doc = "The SAP Configuration."]
    pub configuration: SapConfigurationUnion,
    #[doc = "Managed resource group configuration"]
    #[serde(rename = "managedResourceGroupConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_configuration: Option<ManagedRgConfiguration>,
    #[doc = "Defines the SAP Instance status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SapVirtualInstanceStatus>,
    #[doc = "Defines the health of SAP Instances."]
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
    pub fn new(environment: EnvironmentType, sap_product: SapProductType, configuration: SapConfigurationUnion) -> Self {
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
    SoftwareDetectionInProgress,
    SoftwareDetectionFailed,
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
            Self::SoftwareDetectionInProgress => {
                serializer.serialize_unit_variant("SapVirtualInstanceState", 6u32, "SoftwareDetectionInProgress")
            }
            Self::SoftwareDetectionFailed => serializer.serialize_unit_variant("SapVirtualInstanceState", 7u32, "SoftwareDetectionFailed"),
            Self::DiscoveryPending => serializer.serialize_unit_variant("SapVirtualInstanceState", 8u32, "DiscoveryPending"),
            Self::DiscoveryInProgress => serializer.serialize_unit_variant("SapVirtualInstanceState", 9u32, "DiscoveryInProgress"),
            Self::DiscoveryFailed => serializer.serialize_unit_variant("SapVirtualInstanceState", 10u32, "DiscoveryFailed"),
            Self::RegistrationComplete => serializer.serialize_unit_variant("SapVirtualInstanceState", 11u32, "RegistrationComplete"),
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
    SoftShutdown,
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
            Self::SoftShutdown => serializer.serialize_unit_variant("SapVirtualInstanceStatus", 6u32, "SoftShutdown"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "configuration associated with SAP Landscape Monitor Dashboard."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapLandscapeMonitor {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Gets or sets the properties for Sap Landscape Monitor Dashboard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SapLandscapeMonitorProperties>,
}
impl SapLandscapeMonitor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response from the List SAP Landscape Monitor Dashboard operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapLandscapeMonitorListResult {
    #[doc = "The list of Sap Landscape Monitor configuration."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SapLandscapeMonitor>,
    #[doc = "The URL to get the next set of SAP Landscape Monitor Dashboard."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SapLandscapeMonitorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets the Threshold Values for Top Metrics Health."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapLandscapeMonitorMetricThresholds {
    #[doc = "Gets or sets the name of the threshold."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the threshold value for Green."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub green: Option<f64>,
    #[doc = "Gets or sets the threshold value for Yellow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yellow: Option<f64>,
    #[doc = "Gets or sets the threshold value for Red."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub red: Option<f64>,
}
impl SapLandscapeMonitorMetricThresholds {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets the properties for Sap Landscape Monitor Dashboard."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapLandscapeMonitorProperties {
    #[doc = "State of provisioning of the SAP monitor."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<sap_landscape_monitor_properties::ProvisioningState>,
    #[doc = "Gets or sets the SID groupings by landscape and Environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grouping: Option<sap_landscape_monitor_properties::Grouping>,
    #[doc = "Gets or sets the list Top Metric Thresholds for SAP Landscape Monitor Dashboard"]
    #[serde(
        rename = "topMetricsThresholds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub top_metrics_thresholds: Vec<SapLandscapeMonitorMetricThresholds>,
}
impl SapLandscapeMonitorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sap_landscape_monitor_properties {
    use super::*;
    #[doc = "State of provisioning of the SAP monitor."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Accepted,
        Created,
        Failed,
        Succeeded,
        Canceled,
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
                Self::Created => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Created"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets the SID groupings by landscape and Environment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Grouping {
        #[doc = "Gets or sets the list of landscape to SID mappings."]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub landscape: Vec<SapLandscapeMonitorSidMapping>,
        #[doc = "Gets or sets the list of Sap Applications to SID mappings."]
        #[serde(
            rename = "sapApplication",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub sap_application: Vec<SapLandscapeMonitorSidMapping>,
    }
    impl Grouping {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Gets or sets the mapping for SID to Environment/Applications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapLandscapeMonitorSidMapping {
    #[doc = "Gets or sets the name of the grouping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the list of SID's."]
    #[serde(
        rename = "topSid",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub top_sid: Vec<String>,
}
impl SapLandscapeMonitorSidMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets the provider properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapNetWeaverProviderInstanceProperties {
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
    #[serde(
        rename = "sapHostFileEntries",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[serde(rename = "sslCertificateUri", default, skip_serializing_if = "Option::is_none")]
    pub ssl_certificate_uri: Option<String>,
    #[doc = "Gets or sets certificate preference if secure communication is enabled."]
    #[serde(rename = "sslPreference", default, skip_serializing_if = "Option::is_none")]
    pub ssl_preference: Option<SslPreference>,
}
impl SapNetWeaverProviderInstanceProperties {
    pub fn new() -> Self {
        Self {
            sap_sid: None,
            sap_hostname: None,
            sap_instance_nr: None,
            sap_host_file_entries: Vec::new(),
            sap_username: None,
            sap_password: None,
            sap_password_uri: None,
            sap_client_id: None,
            sap_port_number: None,
            ssl_certificate_uri: None,
            ssl_preference: None,
        }
    }
}
#[doc = "The SAP Software configuration Input when the software is to be installed by service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceInitiatedSoftwareConfiguration {
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
        bom_url: String,
        software_version: String,
        sap_bits_storage_account_id: String,
        sap_fqdn: String,
        ssh_private_key: String,
    ) -> Self {
        Self {
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
    #[doc = "Defines the network configuration type for SAP system infrastructure that is being deployed "]
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
    #[doc = "The Disk Configuration Details."]
    #[serde(rename = "dbDiskConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub db_disk_configuration: Option<DiskConfiguration>,
    #[doc = "The resource-names input to specify custom names for underlying azure resources that are part of a single server SAP system."]
    #[serde(rename = "customResourceNames", default, skip_serializing_if = "Option::is_none")]
    pub custom_resource_names: Option<SingleServerCustomResourceNamesUnion>,
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
            db_disk_configuration: None,
            custom_resource_names: None,
        }
    }
}
#[doc = "The pattern type to be used for resource naming."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "namingPatternType")]
pub enum SingleServerCustomResourceNamesUnion {
    FullResourceName(SingleServerFullResourceNames),
}
#[doc = "The resource name object where the specified values will be full resource names of the corresponding resources in a single server SAP system."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingleServerFullResourceNames {
    #[doc = "The resource names object for virtual machine and related resources."]
    #[serde(rename = "virtualMachine", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine: Option<VirtualMachineResourceNames>,
}
impl SingleServerFullResourceNames {
    pub fn new() -> Self {
        Self { virtual_machine: None }
    }
}
#[doc = "The recommended configuration for a single server SAP system."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingleServerRecommendationResult {
    #[doc = "The recommended VM SKU for single server."]
    #[serde(rename = "vmSku", default, skip_serializing_if = "Option::is_none")]
    pub vm_sku: Option<String>,
}
impl SingleServerRecommendationResult {
    pub fn new() -> Self {
        Self { vm_sku: None }
    }
}
#[doc = "Gets or sets the skip file share configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkipFileShareConfiguration {}
impl SkipFileShareConfiguration {
    pub fn new() -> Self {
        Self {}
    }
}
#[doc = "The SAP software installation Type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "softwareInstallationType")]
pub enum SoftwareConfigurationUnion {
    External(ExternalInstallationSoftwareConfiguration),
    #[serde(rename = "SAPInstallWithoutOSConfig")]
    SapInstallWithoutOsConfig(SapInstallWithoutOsConfigSoftwareConfiguration),
    ServiceInitiated(ServiceInitiatedSoftwareConfiguration),
}
#[doc = "SSH configuration for Linux based VMs running on Azure"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SshConfiguration {
    #[doc = "The list of SSH public keys used to authenticate with linux based VMs."]
    #[serde(
        rename = "publicKeys",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "Gets or sets certificate preference if secure communication is enabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SslPreference")]
pub enum SslPreference {
    Disabled,
    RootCertificate,
    ServerCertificate,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SslPreference {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SslPreference {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SslPreference {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("SslPreference", 0u32, "Disabled"),
            Self::RootCertificate => serializer.serialize_unit_variant("SslPreference", 1u32, "RootCertificate"),
            Self::ServerCertificate => serializer.serialize_unit_variant("SslPreference", 2u32, "ServerCertificate"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Stop SAP instance(s) request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StopRequest {
    #[doc = "This parameter defines how long (in seconds) the soft shutdown waits until the RFC/HTTP clients no longer consider the server for calls with load balancing. Value 0 means that the kernel does not wait, but goes directly into the next shutdown state, i.e. hard stop."]
    #[serde(rename = "softStopTimeoutSeconds", default, skip_serializing_if = "Option::is_none")]
    pub soft_stop_timeout_seconds: Option<i64>,
}
impl StopRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets the storage configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageConfiguration {
    #[doc = "File Share configuration details, populated with information on storage configuration mounted on the VIS. The createAndMount option is selected in case of missing input."]
    #[serde(rename = "transportFileShareConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub transport_file_share_configuration: Option<FileShareConfigurationUnion>,
}
impl StorageConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage details of all the Storage accounts attached to the VM. For e.g. NFS on AFS Shared Storage. "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageInformation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl StorageInformation {
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
    #[doc = "Defines the network configuration type for SAP system infrastructure that is being deployed "]
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
    #[doc = "Gets or sets the storage configuration."]
    #[serde(rename = "storageConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub storage_configuration: Option<StorageConfiguration>,
    #[doc = "The resource-names input to specify custom names for underlying azure resources that are part of a three tier SAP system."]
    #[serde(rename = "customResourceNames", default, skip_serializing_if = "Option::is_none")]
    pub custom_resource_names: Option<ThreeTierCustomResourceNamesUnion>,
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
            storage_configuration: None,
            custom_resource_names: None,
        }
    }
}
#[doc = "The pattern type to be used for resource naming."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "namingPatternType")]
pub enum ThreeTierCustomResourceNamesUnion {
    FullResourceName(ThreeTierFullResourceNames),
}
#[doc = "The resource name object where the specified values will be full resource names of the corresponding resources in a three tier SAP system."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreeTierFullResourceNames {
    #[doc = "The full resource names object for central server layer resources."]
    #[serde(rename = "centralServer", default, skip_serializing_if = "Option::is_none")]
    pub central_server: Option<CentralServerFullResourceNames>,
    #[doc = "The full resource names object for application layer resources. The number of entries in this list should be equal to the number VMs to be created for application layer."]
    #[serde(rename = "applicationServer", default, skip_serializing_if = "Option::is_none")]
    pub application_server: Option<ApplicationServerFullResourceNames>,
    #[doc = "The full resource names object for database layer resources. The number of entries in this list should be equal to the number VMs to be created for database layer."]
    #[serde(rename = "databaseServer", default, skip_serializing_if = "Option::is_none")]
    pub database_server: Option<DatabaseServerFullResourceNames>,
    #[doc = "The resource names object for shared storage."]
    #[serde(rename = "sharedStorage", default, skip_serializing_if = "Option::is_none")]
    pub shared_storage: Option<SharedStorageResourceNames>,
}
impl ThreeTierFullResourceNames {
    pub fn new() -> Self {
        Self {
            central_server: None,
            application_server: None,
            database_server: None,
            shared_storage: None,
        }
    }
}
#[doc = "The recommended configuration for a three tier SAP system."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreeTierRecommendationResult {
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
    pub fn new() -> Self {
        Self {
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
#[doc = "The resource names object for virtual machine and related resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineResourceNames {
    #[doc = "The full name for virtual machine. The length of this field can be upto 64 characters. If name is not provided, service uses a default name based on the deployment type. For SingleServer, default name is {SID}vm. In case of HA-AvZone systems, default name will be {SID}{app/ascs/db}z{a/b}vm with an incrementor at the end in case of more than 1 vm per layer. For distributed and HA-AvSet systems, default name will be {SID}{app/ascs/db}vm with an incrementor at the end in case of more than 1 vm per layer."]
    #[serde(rename = "vmName", default, skip_serializing_if = "Option::is_none")]
    pub vm_name: Option<String>,
    #[doc = "The full name for virtual-machine's host (computer name). Currently, ACSS only supports host names which are less than or equal to 13 characters long. If this value is not provided, vmName will be used as host name."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "The list of network interface name objects for the selected virtual machine. Currently, only one network interface is supported per virtual machine."]
    #[serde(
        rename = "networkInterfaces",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_interfaces: Vec<NetworkInterfaceResourceNames>,
    #[doc = "The full name for OS disk attached to the VM. If this value is not provided, it will be named by ARM as per its default naming standards (prefixed with vm name). There is only one OS disk attached per Virtual Machine."]
    #[serde(rename = "osDiskName", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_name: Option<String>,
    #[doc = "The full resource names for virtual machine data disks. This is a dictionary containing list of names of data disks per volume. Currently supported volumes for database layer are ['hana/data', 'hana/log', hana/shared', 'usr/sap', 'os', 'backup']. For application and cs layers, only 'default' volume is supported"]
    #[serde(rename = "dataDiskNames", default, skip_serializing_if = "Option::is_none")]
    pub data_disk_names: Option<serde_json::Value>,
}
impl VirtualMachineResourceNames {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies Windows operating system settings on the virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsConfiguration {}
impl WindowsConfiguration {
    pub fn new() -> Self {
        Self {}
    }
}
#[doc = "The resource names object for shared storage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedStorageResourceNames {
    #[doc = "The full name of the shared storage account. If it is not provided, it will be defaulted to {SID}nfs{guid of 15 chars}."]
    #[serde(rename = "sharedStorageAccountName", default, skip_serializing_if = "Option::is_none")]
    pub shared_storage_account_name: Option<String>,
    #[doc = "The full name of private end point for the shared storage account. If it is not provided, it will be defaulted to {storageAccountName}_pe"]
    #[serde(
        rename = "sharedStorageAccountPrivateEndPointName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_storage_account_private_end_point_name: Option<String>,
}
impl SharedStorageResourceNames {
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
