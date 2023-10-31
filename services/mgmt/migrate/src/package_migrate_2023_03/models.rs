#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Common API Versions for Assessment Project Tracked Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApiVersions")]
pub enum ApiVersions {
    #[serde(rename = "2019-10-01")]
    N2019_10_01,
    #[serde(rename = "2020-01-01")]
    N2020_01_01,
    #[serde(rename = "2020-05-01-preview")]
    N2020_05_01_preview,
    #[serde(rename = "2022-02-02-preview")]
    N2022_02_02_preview,
    #[serde(rename = "2023-03-03")]
    N2023_03_03,
    #[serde(rename = "2023-03-15")]
    N2023_03_15,
    #[serde(rename = "2023-04-01-preview")]
    N2023_04_01_preview,
    #[serde(rename = "2023-07-07-preview")]
    N2023_07_07_preview,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApiVersions {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApiVersions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApiVersions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::N2019_10_01 => serializer.serialize_unit_variant("ApiVersions", 0u32, "2019-10-01"),
            Self::N2020_01_01 => serializer.serialize_unit_variant("ApiVersions", 1u32, "2020-01-01"),
            Self::N2020_05_01_preview => serializer.serialize_unit_variant("ApiVersions", 2u32, "2020-05-01-preview"),
            Self::N2022_02_02_preview => serializer.serialize_unit_variant("ApiVersions", 3u32, "2022-02-02-preview"),
            Self::N2023_03_03 => serializer.serialize_unit_variant("ApiVersions", 4u32, "2023-03-03"),
            Self::N2023_03_15 => serializer.serialize_unit_variant("ApiVersions", 5u32, "2023-03-15"),
            Self::N2023_04_01_preview => serializer.serialize_unit_variant("ApiVersions", 6u32, "2023-04-01-preview"),
            Self::N2023_07_07_preview => serializer.serialize_unit_variant("ApiVersions", 7u32, "2023-07-07-preview"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Assessed Disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedDisk {
    #[doc = "Cloud Suitability for Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suitability: Option<CloudSuitability>,
    #[serde(rename = "suitabilityExplanation", default, skip_serializing_if = "Option::is_none")]
    pub suitability_explanation: Option<AzureDiskSuitabilityExplanation>,
    #[serde(rename = "suitabilityDetail", default, skip_serializing_if = "Option::is_none")]
    pub suitability_detail: Option<AzureDiskSuitabilityDetail>,
    #[serde(rename = "recommendedDiskSize", default, skip_serializing_if = "Option::is_none")]
    pub recommended_disk_size: Option<AzureDiskSize>,
    #[serde(rename = "recommendedDiskType", default, skip_serializing_if = "Option::is_none")]
    pub recommended_disk_type: Option<AzureDiskType>,
    #[doc = "Gets the recommended disk size."]
    #[serde(rename = "recommendedDiskSizeGigabytes", default, skip_serializing_if = "Option::is_none")]
    pub recommended_disk_size_gigabytes: Option<i32>,
    #[doc = "Gets the recommended disk throughput."]
    #[serde(rename = "recommendDiskThroughputInMbps", default, skip_serializing_if = "Option::is_none")]
    pub recommend_disk_throughput_in_mbps: Option<f32>,
    #[doc = "Gets the recommended disk iops."]
    #[serde(rename = "recommendedDiskIops", default, skip_serializing_if = "Option::is_none")]
    pub recommended_disk_iops: Option<f32>,
    #[doc = "Gets the monthly storage cost."]
    #[serde(rename = "monthlyStorageCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_storage_cost: Option<f32>,
    #[doc = "Gets the name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the machine display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets the gigabytes provisioned."]
    #[serde(rename = "gigabytesProvisioned", default, skip_serializing_if = "Option::is_none")]
    pub gigabytes_provisioned: Option<f32>,
    #[doc = "Gets the megabytes per second of read."]
    #[serde(rename = "megabytesPerSecondOfRead", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_of_read: Option<f32>,
    #[doc = "Gets the megabytes per second of write."]
    #[serde(rename = "megabytesPerSecondOfWrite", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_of_write: Option<f32>,
    #[doc = "Gets the number of read operations per second."]
    #[serde(rename = "numberOfReadOperationsPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub number_of_read_operations_per_second: Option<f32>,
    #[doc = "Gets the number of write operations per second."]
    #[serde(rename = "numberOfWriteOperationsPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub number_of_write_operations_per_second: Option<f32>,
}
impl AssessedDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Machine assessment Assessed Machine resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedMachine {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Assessed machine properties class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssessedMachineProperties>,
}
impl AssessedMachine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a AssessedMachine list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssessedMachineListResult {
    #[doc = "The AssessedMachine items on this page"]
    pub value: Vec<AssessedMachine>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AssessedMachineListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AssessedMachineListResult {
    pub fn new(value: Vec<AssessedMachine>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Assessed machine properties class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedMachineProperties {
    #[doc = "List of errors for this machine."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<Error>,
    #[doc = "Dictionary of disks attached to the machine. Key is ID of disk. Value is a disk object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disks: Option<serde_json::Value>,
    #[doc = "Monthly ultra storage cost."]
    #[serde(rename = "monthlyUltraStorageCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_ultra_storage_cost: Option<f32>,
    #[doc = "Represents a information \\ details of a processor."]
    #[serde(rename = "hostProcessor", default, skip_serializing_if = "Option::is_none")]
    pub host_processor: Option<ProcessorInfo>,
    #[doc = "Gets or sets the collection of cost components."]
    #[serde(
        rename = "costComponents",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cost_components: Vec<CostComponent>,
    #[doc = "Class to represent the Product Support Status."]
    #[serde(rename = "productSupportStatus", default, skip_serializing_if = "Option::is_none")]
    pub product_support_status: Option<ProductSupportStatus>,
    #[doc = "Monthly networking cost."]
    #[serde(rename = "monthlyBandwidthCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_bandwidth_cost: Option<f32>,
    #[doc = "Monthly storage cost."]
    #[serde(rename = "monthlyStorageCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_storage_cost: Option<f32>,
    #[doc = "Monthly premium storage cost."]
    #[serde(rename = "monthlyPremiumStorageCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_premium_storage_cost: Option<f32>,
    #[doc = "Monthly standard SSD storage cost."]
    #[serde(rename = "monthlyStandardSsdStorageCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_standard_ssd_storage_cost: Option<f32>,
    #[doc = "List of Network Adapters that were assessed as part of this machine's\nassessment."]
    #[serde(rename = "networkAdapters", default, skip_serializing_if = "Option::is_none")]
    pub network_adapters: Option<serde_json::Value>,
    #[serde(rename = "recommendedSize", default, skip_serializing_if = "Option::is_none")]
    pub recommended_size: Option<AzureVmSize>,
    #[doc = "Number of cores for recommended size. Read Only."]
    #[serde(rename = "numberOfCoresForRecommendedSize", default, skip_serializing_if = "Option::is_none")]
    pub number_of_cores_for_recommended_size: Option<i32>,
    #[doc = "Megabytes of memory for recommended size. Read Only."]
    #[serde(rename = "megabytesOfMemoryForRecommendedSize", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_of_memory_for_recommended_size: Option<f32>,
    #[doc = "Monthly Compute cost calculated for Recommended size, for a 31-day month."]
    #[serde(rename = "monthlyComputeCostForRecommendedSize", default, skip_serializing_if = "Option::is_none")]
    pub monthly_compute_cost_for_recommended_size: Option<f32>,
    #[serde(rename = "suitabilityExplanation", default, skip_serializing_if = "Option::is_none")]
    pub suitability_explanation: Option<AzureVmSuitabilityExplanation>,
    #[serde(rename = "suitabilityDetail", default, skip_serializing_if = "Option::is_none")]
    pub suitability_detail: Option<AzureVmSuitabilityDetail>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<AssessedMachineType>,
    #[serde(rename = "bootType", default, skip_serializing_if = "Option::is_none")]
    pub boot_type: Option<MachineBootType>,
    #[doc = "Operating system as reported by datacenter management solution."]
    #[serde(rename = "operatingSystemType", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_type: Option<String>,
    #[doc = "Operating system as reported by datacenter management solution."]
    #[serde(rename = "operatingSystemName", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_name: Option<String>,
    #[doc = "Operating system version as reported by datacenter management solution."]
    #[serde(rename = "operatingSystemVersion", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_version: Option<String>,
    #[serde(rename = "operatingSystemArchitecture", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_architecture: Option<GuestOperatingSystemArchitecture>,
    #[doc = "When was machine first created."]
    #[serde(rename = "createdTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "When was machine last updated."]
    #[serde(rename = "updatedTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Display Name of the Machine."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Description for the machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Data center machine ARM id."]
    #[serde(rename = "datacenterMachineArmId", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_machine_arm_id: Option<String>,
    #[doc = "Data center management server ARM id."]
    #[serde(rename = "datacenterManagementServerArmId", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_management_server_arm_id: Option<String>,
    #[doc = "Data center management server name."]
    #[serde(rename = "datacenterManagementServerName", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_management_server_name: Option<String>,
    #[doc = "Megabytes of memory found allocated for the machine in private data center."]
    #[serde(rename = "megabytesOfMemory", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_of_memory: Option<f32>,
    #[doc = "Number of CPU cores found on the machine."]
    #[serde(rename = "numberOfCores", default, skip_serializing_if = "Option::is_none")]
    pub number_of_cores: Option<i32>,
    #[doc = "Confidence Rating in Percentage."]
    #[serde(rename = "confidenceRatingInPercentage", default, skip_serializing_if = "Option::is_none")]
    pub confidence_rating_in_percentage: Option<f32>,
    #[doc = "Percentile of Percentage of Cores Utilized noted during time period T.\n        \n   Here N and T are settings on Assessment."]
    #[serde(rename = "percentageCoresUtilization", default, skip_serializing_if = "Option::is_none")]
    pub percentage_cores_utilization: Option<f32>,
    #[doc = "Percentile of Percentage of Memory Utilized noted during time period T.\n       \n    Here N and T are settings on Assessment."]
    #[serde(rename = "percentageMemoryUtilization", default, skip_serializing_if = "Option::is_none")]
    pub percentage_memory_utilization: Option<f32>,
    #[doc = "Cloud Suitability for Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suitability: Option<CloudSuitability>,
}
impl AssessedMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AssessedMachineType")]
pub enum AssessedMachineType {
    Unknown,
    AssessedMachine,
    AvsAssessedMachine,
    SqlAssessedMachine,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AssessedMachineType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AssessedMachineType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AssessedMachineType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AssessedMachineType", 0u32, "Unknown"),
            Self::AssessedMachine => serializer.serialize_unit_variant("AssessedMachineType", 1u32, "AssessedMachine"),
            Self::AvsAssessedMachine => serializer.serialize_unit_variant("AssessedMachineType", 2u32, "AvsAssessedMachine"),
            Self::SqlAssessedMachine => serializer.serialize_unit_variant("AssessedMachineType", 3u32, "SqlAssessedMachine"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Assessed Network Adapter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedNetworkAdapter {
    #[doc = "Cloud Suitability for Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suitability: Option<CloudSuitability>,
    #[serde(rename = "suitabilityDetail", default, skip_serializing_if = "Option::is_none")]
    pub suitability_detail: Option<AzureNetworkAdapterSuitabilityDetail>,
    #[serde(rename = "suitabilityExplanation", default, skip_serializing_if = "Option::is_none")]
    pub suitability_explanation: Option<AzureNetworkAdapterSuitabilityExplanation>,
    #[doc = "Gets the monthly bandwidth costs."]
    #[serde(rename = "monthlyBandwidthCosts", default, skip_serializing_if = "Option::is_none")]
    pub monthly_bandwidth_costs: Option<f32>,
    #[doc = "Gets the net gigabytes transmitted per month."]
    #[serde(rename = "netGigabytesTransmittedPerMonth", default, skip_serializing_if = "Option::is_none")]
    pub net_gigabytes_transmitted_per_month: Option<f32>,
    #[doc = "Gets the display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets the mac address."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "Gets the ip addresses."]
    #[serde(
        rename = "ipAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_addresses: Vec<String>,
    #[doc = "Gets the megabytes per second received."]
    #[serde(rename = "megabytesPerSecondReceived", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_received: Option<f32>,
    #[doc = "Gets the megabytes per second transmitted."]
    #[serde(rename = "megabytesPerSecondTransmitted", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_transmitted: Option<f32>,
}
impl AssessedNetworkAdapter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Assessed SQL database web model class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedSqlDatabaseV2 {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Assessed SQL database properties web model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssessedSqlDatabaseV2Properties>,
}
impl AssessedSqlDatabaseV2 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a AssessedSqlDatabaseV2 list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssessedSqlDatabaseV2ListResult {
    #[doc = "The AssessedSqlDatabaseV2 items on this page"]
    pub value: Vec<AssessedSqlDatabaseV2>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AssessedSqlDatabaseV2ListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AssessedSqlDatabaseV2ListResult {
    pub fn new(value: Vec<AssessedSqlDatabaseV2>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Assessed SQL database properties web model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedSqlDatabaseV2Properties {
    #[serde(rename = "recommendedAzureSqlTargetType", default, skip_serializing_if = "Option::is_none")]
    pub recommended_azure_sql_target_type: Option<TargetType>,
    #[serde(rename = "recommendedSuitability", default, skip_serializing_if = "Option::is_none")]
    pub recommended_suitability: Option<RecommendedSuitability>,
    #[doc = "Gets or sets the aggregated cache size of this database. This is a performance\ndata metric for this DB."]
    #[serde(rename = "bufferCacheSizeInMB", default, skip_serializing_if = "Option::is_none")]
    pub buffer_cache_size_in_mb: Option<f32>,
    #[doc = "Class to represent the Product Support Status."]
    #[serde(rename = "productSupportStatus", default, skip_serializing_if = "Option::is_none")]
    pub product_support_status: Option<ProductSupportStatus>,
    #[doc = "Class representing Azure SQL PAAS suitability details."]
    #[serde(rename = "azureSqlMISuitabilityDetails", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_mi_suitability_details: Option<SqlAssessmentV2PaasSuitabilityData>,
    #[doc = "Class representing Azure SQL PAAS suitability details."]
    #[serde(rename = "azureSqlDBSuitabilityDetails", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_db_suitability_details: Option<SqlAssessmentV2PaasSuitabilityData>,
    #[doc = "Gets a value indicating whether the assessed SQL database is highly available\nor not."]
    #[serde(rename = "isDatabaseHighlyAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_database_highly_available: Option<bool>,
    #[doc = "Assessed Sql Availability Group Data Overview."]
    #[serde(rename = "linkedAvailabilityGroupOverview", default, skip_serializing_if = "Option::is_none")]
    pub linked_availability_group_overview: Option<SqlAvailabilityGroupDataOverview>,
    #[doc = "Machine arm id."]
    #[serde(rename = "machineArmId", default, skip_serializing_if = "Option::is_none")]
    pub machine_arm_id: Option<String>,
    #[doc = "Assessed SQL instance arm id."]
    #[serde(rename = "assessedSqlInstanceArmId", default, skip_serializing_if = "Option::is_none")]
    pub assessed_sql_instance_arm_id: Option<String>,
    #[doc = "Machine display name."]
    #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[doc = "SQL instance name."]
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[doc = "SQL database name."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "SQL database size in megabytes."]
    #[serde(rename = "databaseSizeInMB", default, skip_serializing_if = "Option::is_none")]
    pub database_size_in_mb: Option<f32>,
    #[serde(rename = "compatibilityLevel", default, skip_serializing_if = "Option::is_none")]
    pub compatibility_level: Option<CompatibilityLevel>,
    #[doc = "SQL database SDS arm id."]
    #[serde(rename = "sqlDatabaseSdsArmId", default, skip_serializing_if = "Option::is_none")]
    pub sql_database_sds_arm_id: Option<String>,
    #[doc = "The percentage of the total number of cores being utilized by the SQL database."]
    #[serde(rename = "percentageCoresUtilization", default, skip_serializing_if = "Option::is_none")]
    pub percentage_cores_utilization: Option<f32>,
    #[doc = "The read throughput of the SQL database."]
    #[serde(rename = "megabytesPerSecondOfRead", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_of_read: Option<f32>,
    #[doc = "The write throughput of the SQL database."]
    #[serde(rename = "megabytesPerSecondOfWrite", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_of_write: Option<f32>,
    #[doc = "The read operations per second of the SQL database."]
    #[serde(rename = "numberOfReadOperationsPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub number_of_read_operations_per_second: Option<f32>,
    #[doc = "The write operations per second of the SQL database."]
    #[serde(rename = "numberOfWriteOperationsPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub number_of_write_operations_per_second: Option<f32>,
    #[doc = "Confidence Rating in Percentage."]
    #[serde(rename = "confidenceRatingInPercentage", default, skip_serializing_if = "Option::is_none")]
    pub confidence_rating_in_percentage: Option<f32>,
    #[doc = "Assessment Sizing Criteria."]
    #[serde(rename = "sizingCriterion", default, skip_serializing_if = "Option::is_none")]
    pub sizing_criterion: Option<AssessmentSizingCriterion>,
    #[doc = "When was assessed SQL database first created."]
    #[serde(rename = "createdTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "When was assessed SQL database last updated."]
    #[serde(rename = "updatedTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
}
impl AssessedSqlDatabaseV2Properties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Assessed Sql Instance Database Summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedSqlInstanceDatabaseSummary {
    #[doc = "Gets the number of user databases."]
    #[serde(rename = "numberOfUserDatabases", default, skip_serializing_if = "Option::is_none")]
    pub number_of_user_databases: Option<i32>,
    #[doc = "Gets the total database size in MB."]
    #[serde(rename = "totalDatabaseSizeInMB", default, skip_serializing_if = "Option::is_none")]
    pub total_database_size_in_mb: Option<f32>,
    #[doc = "Gets the largest database size in MB."]
    #[serde(rename = "largestDatabaseSizeInMB", default, skip_serializing_if = "Option::is_none")]
    pub largest_database_size_in_mb: Option<f32>,
    #[doc = "Gets the total discovered user databases."]
    #[serde(rename = "totalDiscoveredUserDatabases", default, skip_serializing_if = "Option::is_none")]
    pub total_discovered_user_databases: Option<i32>,
}
impl AssessedSqlInstanceDatabaseSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Assessed Sql Instance Disk Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedSqlInstanceDiskDetails {
    #[doc = "Gets the disk id."]
    #[serde(rename = "diskId", default, skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[doc = "Gets the disk size in mb."]
    #[serde(rename = "diskSizeInMB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_in_mb: Option<f32>,
    #[doc = "Gets the megabytes per second of read."]
    #[serde(rename = "megabytesPerSecondOfRead", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_of_read: Option<f32>,
    #[doc = "Gets the megabytes per second of write."]
    #[serde(rename = "megabytesPerSecondOfWrite", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_of_write: Option<f32>,
    #[doc = "Gets the number of read operations per second."]
    #[serde(rename = "numberOfReadOperationsPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub number_of_read_operations_per_second: Option<f32>,
    #[doc = "Gets the number of write operations per second."]
    #[serde(rename = "numberOfWriteOperationsPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub number_of_write_operations_per_second: Option<f32>,
}
impl AssessedSqlInstanceDiskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Assessed Sql Instance Storage Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedSqlInstanceStorageDetails {
    #[doc = "Gets the storage type."]
    #[serde(rename = "storageType", default, skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[doc = "Gets the disk size in mb."]
    #[serde(rename = "diskSizeInMB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_in_mb: Option<f32>,
    #[doc = "Gets the megabytes per second of read."]
    #[serde(rename = "megabytesPerSecondOfRead", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_of_read: Option<f32>,
    #[doc = "Gets the megabytes per second of read."]
    #[serde(rename = "megabytesPerSecondOfWrite", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_of_write: Option<f32>,
    #[doc = "Gets the number of read operations per second."]
    #[serde(rename = "numberOfReadOperationsPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub number_of_read_operations_per_second: Option<f32>,
    #[doc = "Gets the number of write operations per second."]
    #[serde(rename = "numberOfWriteOperationsPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub number_of_write_operations_per_second: Option<f32>,
}
impl AssessedSqlInstanceStorageDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Assessed Sql Instance Summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedSqlInstanceSummary {
    #[doc = "Gets the instance id."]
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[doc = "Gets the instance name."]
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[doc = "Gets the instance arm id."]
    #[serde(rename = "sqlInstanceSdsArmId", default, skip_serializing_if = "Option::is_none")]
    pub sql_instance_sds_arm_id: Option<String>,
    #[doc = "Gets the instance entity id."]
    #[serde(rename = "sqlInstanceEntityId", default, skip_serializing_if = "Option::is_none")]
    pub sql_instance_entity_id: Option<String>,
    #[doc = "Gets the Sql edition."]
    #[serde(rename = "sqlEdition", default, skip_serializing_if = "Option::is_none")]
    pub sql_edition: Option<String>,
    #[doc = "Gets the Sql version."]
    #[serde(rename = "sqlVersion", default, skip_serializing_if = "Option::is_none")]
    pub sql_version: Option<String>,
    #[doc = "Gets whether Sql is clustered."]
    #[serde(rename = "isClustered", default, skip_serializing_if = "Option::is_none")]
    pub is_clustered: Option<bool>,
    #[doc = "Gets whether Sql is highly available."]
    #[serde(rename = "isHighAvailabilityEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_high_availability_enabled: Option<bool>,
    #[serde(rename = "sqlFciState", default, skip_serializing_if = "Option::is_none")]
    pub sql_fci_state: Option<SqlFciState>,
}
impl AssessedSqlInstanceSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Assessed SQL instance web model class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedSqlInstanceV2 {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Assessed SQL instance properties web model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssessedSqlInstanceV2Properties>,
}
impl AssessedSqlInstanceV2 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a AssessedSqlInstanceV2 list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssessedSqlInstanceV2ListResult {
    #[doc = "The AssessedSqlInstanceV2 items on this page"]
    pub value: Vec<AssessedSqlInstanceV2>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AssessedSqlInstanceV2ListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AssessedSqlInstanceV2ListResult {
    pub fn new(value: Vec<AssessedSqlInstanceV2>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Assessed SQL instance properties web model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedSqlInstanceV2Properties {
    #[doc = "Gets or sets the memory used by SQL instance in megabytes."]
    #[serde(rename = "memoryInUseInMB", default, skip_serializing_if = "Option::is_none")]
    pub memory_in_use_in_mb: Option<f32>,
    #[doc = "Gets or sets a value indicating whether SQL instance has been deep discovered."]
    #[serde(rename = "hasScanOccurred", default, skip_serializing_if = "Option::is_none")]
    pub has_scan_occurred: Option<bool>,
    #[serde(rename = "recommendedAzureSqlTargetType", default, skip_serializing_if = "Option::is_none")]
    pub recommended_azure_sql_target_type: Option<TargetType>,
    #[serde(rename = "recommendedSuitability", default, skip_serializing_if = "Option::is_none")]
    pub recommended_suitability: Option<RecommendedSuitability>,
    #[doc = "Class representing Azure SQL PAAS suitability details."]
    #[serde(rename = "azureSqlMISuitabilityDetails", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_mi_suitability_details: Option<SqlAssessmentV2PaasSuitabilityData>,
    #[doc = "Class representing Azure SQL PAAS suitability details."]
    #[serde(rename = "azureSqlDBSuitabilityDetails", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_db_suitability_details: Option<SqlAssessmentV2PaasSuitabilityData>,
    #[doc = "Class representing Azure SQL IAAS suitability details."]
    #[serde(rename = "azureSqlVMSuitabilityDetails", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_vm_suitability_details: Option<SqlAssessmentV2IaasSuitabilityData>,
    #[doc = "Gets the storage details."]
    #[serde(
        rename = "storageTypeBasedDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub storage_type_based_details: Vec<AssessedSqlInstanceStorageDetails>,
    #[doc = "Class to represent the Product Support Status."]
    #[serde(rename = "productSupportStatus", default, skip_serializing_if = "Option::is_none")]
    pub product_support_status: Option<ProductSupportStatus>,
    #[doc = "Sql fci meta data."]
    #[serde(rename = "fciMetadata", default, skip_serializing_if = "Option::is_none")]
    pub fci_metadata: Option<SqlFciMetadata>,
    #[doc = "Assessed Sql Availability Replica Summary."]
    #[serde(rename = "availabilityReplicaSummary", default, skip_serializing_if = "Option::is_none")]
    pub availability_replica_summary: Option<SqlAvailabilityReplicaSummary>,
    #[doc = "Gets a value indicating whether the SQL instance is clustered or not."]
    #[serde(rename = "isClustered", default, skip_serializing_if = "Option::is_none")]
    pub is_clustered: Option<bool>,
    #[doc = "Gets a value indicating whether the high availability is enabled or not."]
    #[serde(rename = "isHighAvailabilityEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_high_availability_enabled: Option<bool>,
    #[doc = "Gets the list of recommended target reasoning."]
    #[serde(
        rename = "recommendedTargetReasonings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub recommended_target_reasonings: Vec<SqlRecommendationReasoning>,
    #[doc = "Machine arm id."]
    #[serde(rename = "machineArmId", default, skip_serializing_if = "Option::is_none")]
    pub machine_arm_id: Option<String>,
    #[doc = "Machine display name."]
    #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[doc = "SQL instance name."]
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[doc = "SQL instance SDS arm id."]
    #[serde(rename = "sqlInstanceSdsArmId", default, skip_serializing_if = "Option::is_none")]
    pub sql_instance_sds_arm_id: Option<String>,
    #[doc = "SQL instance edition."]
    #[serde(rename = "sqlEdition", default, skip_serializing_if = "Option::is_none")]
    pub sql_edition: Option<String>,
    #[doc = "SQL instance version."]
    #[serde(rename = "sqlVersion", default, skip_serializing_if = "Option::is_none")]
    pub sql_version: Option<String>,
    #[doc = "Number of CPU cores assigned to the SQL instance."]
    #[serde(rename = "numberOfCoresAllocated", default, skip_serializing_if = "Option::is_none")]
    pub number_of_cores_allocated: Option<i32>,
    #[doc = "The percentage of the total number of cores being utilized by the SQL instance."]
    #[serde(rename = "percentageCoresUtilization", default, skip_serializing_if = "Option::is_none")]
    pub percentage_cores_utilization: Option<f32>,
    #[doc = "The logical disk details."]
    #[serde(
        rename = "logicalDisks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub logical_disks: Vec<AssessedSqlInstanceDiskDetails>,
    #[doc = "Assessed Sql Instance Database Summary."]
    #[serde(rename = "databaseSummary", default, skip_serializing_if = "Option::is_none")]
    pub database_summary: Option<AssessedSqlInstanceDatabaseSummary>,
    #[doc = "Confidence Rating in Percentage."]
    #[serde(rename = "confidenceRatingInPercentage", default, skip_serializing_if = "Option::is_none")]
    pub confidence_rating_in_percentage: Option<f32>,
    #[doc = "Assessment Sizing Criteria."]
    #[serde(rename = "sizingCriterion", default, skip_serializing_if = "Option::is_none")]
    pub sizing_criterion: Option<AssessmentSizingCriterion>,
    #[doc = "When was assessed SQL instance first created."]
    #[serde(rename = "createdTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "When was assessed SQL instance last updated."]
    #[serde(rename = "updatedTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
}
impl AssessedSqlInstanceV2Properties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SQL Assessment REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedSqlMachine {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Assessed SQL machine properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssessedSqlMachineProperties>,
}
impl AssessedSqlMachine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a AssessedSqlMachine list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssessedSqlMachineListResult {
    #[doc = "The AssessedSqlMachine items on this page"]
    pub value: Vec<AssessedSqlMachine>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AssessedSqlMachineListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AssessedSqlMachineListResult {
    pub fn new(value: Vec<AssessedSqlMachine>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Assessed SQL machine properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedSqlMachineProperties {
    #[doc = "Gets or sets the BIOS GUID for the machine."]
    #[serde(rename = "biosGuid", default, skip_serializing_if = "Option::is_none")]
    pub bios_guid: Option<String>,
    #[doc = "Gets or sets the FQDN for the machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Gets the list of SQL instances discovered on the machine."]
    #[serde(
        rename = "sqlInstances",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sql_instances: Vec<AssessedSqlInstanceSummary>,
    #[serde(rename = "suitabilityDetail", default, skip_serializing_if = "Option::is_none")]
    pub suitability_detail: Option<AzureVmSuitabilityDetail>,
    #[serde(rename = "suitabilityExplanation", default, skip_serializing_if = "Option::is_none")]
    pub suitability_explanation: Option<AzureVmSuitabilityExplanation>,
    #[serde(rename = "recommendedVmSize", default, skip_serializing_if = "Option::is_none")]
    pub recommended_vm_size: Option<AzureVmSize>,
    #[serde(rename = "recommendedVmFamily", default, skip_serializing_if = "Option::is_none")]
    pub recommended_vm_family: Option<AzureVmFamily>,
    #[doc = "Class to represent the Product Support Status."]
    #[serde(rename = "productSupportStatus", default, skip_serializing_if = "Option::is_none")]
    pub product_support_status: Option<ProductSupportStatus>,
    #[doc = "Gets or sets the Number of cores for recommended size."]
    #[serde(rename = "recommendedVmSizeNumberOfCores", default, skip_serializing_if = "Option::is_none")]
    pub recommended_vm_size_number_of_cores: Option<i32>,
    #[doc = "Gets or sets the Megabytes of memory for recommended size."]
    #[serde(rename = "recommendedVmSizeMegabytesOfMemory", default, skip_serializing_if = "Option::is_none")]
    pub recommended_vm_size_megabytes_of_memory: Option<f32>,
    #[doc = "Gets or sets the monthly compute cost calculated for recommended size."]
    #[serde(rename = "monthlyComputeCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_compute_cost: Option<f32>,
    #[doc = "Gets the list of data disks that were assessed as part of this assessment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disks: Option<serde_json::Value>,
    #[doc = "Gets the list of network adapters that were assessed as part of this assessment."]
    #[serde(rename = "networkAdapters", default, skip_serializing_if = "Option::is_none")]
    pub network_adapters: Option<serde_json::Value>,
    #[doc = "Gets or sets the monthly networking cost."]
    #[serde(rename = "monthlyBandwidthCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_bandwidth_cost: Option<f32>,
    #[doc = "Gets or sets the monthly total storage cost."]
    #[serde(rename = "monthlyStorageCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_storage_cost: Option<f32>,
    #[doc = "Gets the collection of cost components."]
    #[serde(
        rename = "costComponents",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cost_components: Vec<CostComponent>,
    #[doc = "Cloud Suitability for Azure."]
    #[serde(rename = "securitySuitability", default, skip_serializing_if = "Option::is_none")]
    pub security_suitability: Option<CloudSuitability>,
    #[doc = "Gets the list of migration guidelines applicable."]
    #[serde(
        rename = "migrationGuidelines",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub migration_guidelines: Vec<SqlMigrationGuideline>,
    #[serde(rename = "bootType", default, skip_serializing_if = "Option::is_none")]
    pub boot_type: Option<MachineBootType>,
    #[doc = "Operating system as reported by datacenter management solution."]
    #[serde(rename = "operatingSystemType", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_type: Option<String>,
    #[doc = "Operating system as reported by datacenter management solution."]
    #[serde(rename = "operatingSystemName", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_name: Option<String>,
    #[doc = "Operating system version as reported by datacenter management solution."]
    #[serde(rename = "operatingSystemVersion", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_version: Option<String>,
    #[serde(rename = "operatingSystemArchitecture", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_architecture: Option<GuestOperatingSystemArchitecture>,
    #[doc = "When was machine first created."]
    #[serde(rename = "createdTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "When was machine last updated."]
    #[serde(rename = "updatedTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Display Name of the Machine."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<AssessedMachineType>,
    #[doc = "Description for the machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Data center machine ARM id."]
    #[serde(rename = "datacenterMachineArmId", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_machine_arm_id: Option<String>,
    #[doc = "Data center management server ARM id."]
    #[serde(rename = "datacenterManagementServerArmId", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_management_server_arm_id: Option<String>,
    #[doc = "Data center management server name."]
    #[serde(rename = "datacenterManagementServerName", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_management_server_name: Option<String>,
    #[doc = "Megabytes of memory found allocated for the machine in private data center."]
    #[serde(rename = "megabytesOfMemory", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_of_memory: Option<f32>,
    #[doc = "Number of CPU cores found on the machine."]
    #[serde(rename = "numberOfCores", default, skip_serializing_if = "Option::is_none")]
    pub number_of_cores: Option<i32>,
    #[doc = "Confidence Rating in Percentage."]
    #[serde(rename = "confidenceRatingInPercentage", default, skip_serializing_if = "Option::is_none")]
    pub confidence_rating_in_percentage: Option<f32>,
    #[doc = "Percentile of Percentage of Cores Utilized noted during time period T.\n        \n   Here N and T are settings on Assessment."]
    #[serde(rename = "percentageCoresUtilization", default, skip_serializing_if = "Option::is_none")]
    pub percentage_cores_utilization: Option<f32>,
    #[doc = "Percentile of Percentage of Memory Utilized noted during time period T.\n       \n    Here N and T are settings on Assessment."]
    #[serde(rename = "percentageMemoryUtilization", default, skip_serializing_if = "Option::is_none")]
    pub percentage_memory_utilization: Option<f32>,
    #[doc = "Cloud Suitability for Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suitability: Option<CloudSuitability>,
    #[doc = "Assessment Sizing Criteria."]
    #[serde(rename = "sizingCriterion", default, skip_serializing_if = "Option::is_none")]
    pub sizing_criterion: Option<AssessmentSizingCriterion>,
}
impl AssessedSqlMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SQL Assessment REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedSqlRecommendedEntity {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Assessed SQL recommended entity properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssessedSqlRecommendedEntityProperties>,
}
impl AssessedSqlRecommendedEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a AssessedSqlRecommendedEntity list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssessedSqlRecommendedEntityListResult {
    #[doc = "The AssessedSqlRecommendedEntity items on this page"]
    pub value: Vec<AssessedSqlRecommendedEntity>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AssessedSqlRecommendedEntityListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AssessedSqlRecommendedEntityListResult {
    pub fn new(value: Vec<AssessedSqlRecommendedEntity>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Assessed SQL recommended entity properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedSqlRecommendedEntityProperties {
    #[doc = "Gets or sets machine name."]
    #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[doc = "Gets or sets SQL instance name."]
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[doc = "Class to represent the Product Support Status."]
    #[serde(rename = "productSupportStatus", default, skip_serializing_if = "Option::is_none")]
    pub product_support_status: Option<ProductSupportStatus>,
    #[doc = "Gets or sets assessed database count."]
    #[serde(rename = "dbCount", default, skip_serializing_if = "Option::is_none")]
    pub db_count: Option<i32>,
    #[doc = "Gets or sets the total discovered database count."]
    #[serde(rename = "discoveredDBCount", default, skip_serializing_if = "Option::is_none")]
    pub discovered_db_count: Option<i32>,
    #[doc = "Gets or sets a value indicating whether instance deep discovery has occurred or\nnot."]
    #[serde(rename = "hasScanOccurred", default, skip_serializing_if = "Option::is_none")]
    pub has_scan_occurred: Option<bool>,
    #[serde(rename = "recommendedAzureSqlTargetType", default, skip_serializing_if = "Option::is_none")]
    pub recommended_azure_sql_target_type: Option<TargetType>,
    #[serde(rename = "recommendedSuitability", default, skip_serializing_if = "Option::is_none")]
    pub recommended_suitability: Option<RecommendedSuitability>,
    #[doc = "Class representing Azure SQL PAAS suitability details."]
    #[serde(rename = "azureSqlMISuitabilityDetails", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_mi_suitability_details: Option<SqlAssessmentV2PaasSuitabilityData>,
    #[doc = "Class representing Azure SQL PAAS suitability details."]
    #[serde(rename = "azureSqlDBSuitabilityDetails", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_db_suitability_details: Option<SqlAssessmentV2PaasSuitabilityData>,
    #[doc = "Class representing Azure SQL IAAS suitability details."]
    #[serde(rename = "azureSqlVMSuitabilityDetails", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_vm_suitability_details: Option<SqlAssessmentV2IaasSuitabilityData>,
    #[doc = "Gets or sets Arm id of assessed entity."]
    #[serde(rename = "assessedSqlEntityArmId", default, skip_serializing_if = "Option::is_none")]
    pub assessed_sql_entity_arm_id: Option<String>,
    #[doc = "Gets or sets a value indicating whether the SQL instance is clustered or not."]
    #[serde(rename = "isClustered", default, skip_serializing_if = "Option::is_none")]
    pub is_clustered: Option<bool>,
    #[doc = "Gets or sets a value indicating whether the high availability is enabled or not."]
    #[serde(rename = "isHighAvailabilityEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_high_availability_enabled: Option<bool>,
    #[doc = "Gets the SQL edition from the recommended entity if applicable."]
    #[serde(rename = "sqlEdition", default, skip_serializing_if = "Option::is_none")]
    pub sql_edition: Option<String>,
    #[doc = "Gets the SQL version from the recommended entity if applicable."]
    #[serde(rename = "sqlVersion", default, skip_serializing_if = "Option::is_none")]
    pub sql_version: Option<String>,
    #[doc = "Assessment Sizing Criteria."]
    #[serde(rename = "sizingCriterion", default, skip_serializing_if = "Option::is_none")]
    pub sizing_criterion: Option<AssessmentSizingCriterion>,
}
impl AssessedSqlRecommendedEntityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Machine assessment resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Assessment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an assessment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineAssessmentProperties>,
}
impl Assessment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a Assessment list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssessmentListResult {
    #[doc = "The Assessment items on this page"]
    pub value: Vec<Assessment>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AssessmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AssessmentListResult {
    pub fn new(value: Vec<Assessment>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Assessment options resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessmentOptions {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Assessment options properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssessmentOptionsProperties>,
}
impl AssessmentOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a AssessmentOptions list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssessmentOptionsListResult {
    #[doc = "The AssessmentOptions items on this page"]
    pub value: Vec<AssessmentOptions>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AssessmentOptionsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AssessmentOptionsListResult {
    pub fn new(value: Vec<AssessmentOptions>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Assessment options properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessmentOptionsProperties {
    #[doc = "Dictionary of VM families grouped by vm family name describing the targeted\nazure locations of VM family and the category of the family."]
    #[serde(
        rename = "vmFamilies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vm_families: Vec<VmFamilyOptions>,
    #[doc = "List of supported VM Families."]
    #[serde(
        rename = "reservedInstanceVmFamilies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reserved_instance_vm_families: Vec<String>,
    #[doc = "List of supported Azure regions for reserved instances."]
    #[serde(
        rename = "reservedInstanceSupportedLocations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reserved_instance_supported_locations: Vec<String>,
    #[doc = "List of supported currencies for reserved instances."]
    #[serde(
        rename = "reservedInstanceSupportedCurrencies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reserved_instance_supported_currencies: Vec<String>,
    #[doc = "List of supported Azure offer codes for reserved instances."]
    #[serde(
        rename = "reservedInstanceSupportedOffers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reserved_instance_supported_offers: Vec<String>,
    #[doc = "Ultra disk related assessment options."]
    #[serde(
        rename = "ultraDiskVmFamilies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ultra_disk_vm_families: Vec<UltraDiskAssessmentOptions>,
    #[doc = "List of VM Families that support premium disks for assessments."]
    #[serde(
        rename = "premiumDiskVmFamilies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub premium_disk_vm_families: Vec<String>,
    #[doc = "List of VM Families that support Savings plan offer for assessments."]
    #[serde(
        rename = "savingsPlanVmFamilies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub savings_plan_vm_families: Vec<String>,
    #[doc = "List of Azure locations that support Savings plan offer for assessments."]
    #[serde(
        rename = "savingsPlanSupportedLocations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub savings_plan_supported_locations: Vec<String>,
}
impl AssessmentOptionsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Assessment project site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssessmentProject {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of a project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProjectProperties>,
}
impl AssessmentProject {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a AssessmentProject list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssessmentProjectListResult {
    #[doc = "The AssessmentProject items on this page"]
    pub value: Vec<AssessmentProject>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AssessmentProjectListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AssessmentProjectListResult {
    pub fn new(value: Vec<AssessmentProject>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Assessment project summary resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessmentProjectSummary {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Assessment project summary properties class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssessmentProjectSummaryProperties>,
}
impl AssessmentProjectSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a AssessmentProjectSummary list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssessmentProjectSummaryListResult {
    #[doc = "The AssessmentProjectSummary items on this page"]
    pub value: Vec<AssessmentProjectSummary>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AssessmentProjectSummaryListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AssessmentProjectSummaryListResult {
    pub fn new(value: Vec<AssessmentProjectSummary>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Assessment project summary properties class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessmentProjectSummaryProperties {
    #[doc = "Gets the Error summary by feature. Contains number of affected Entities per\nfeature."]
    #[serde(
        rename = "errorSummaryAffectedEntities",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub error_summary_affected_entities: Vec<ErrorSummary>,
    #[doc = "Gets the number of private endpoint connections."]
    #[serde(rename = "numberOfPrivateEndpointConnections", default, skip_serializing_if = "Option::is_none")]
    pub number_of_private_endpoint_connections: Option<i32>,
    #[doc = "Gets the number of groups created in this project."]
    #[serde(rename = "numberOfGroups", default, skip_serializing_if = "Option::is_none")]
    pub number_of_groups: Option<i32>,
    #[doc = "Gets the number of machines part of this project."]
    #[serde(rename = "numberOfMachines", default, skip_serializing_if = "Option::is_none")]
    pub number_of_machines: Option<i32>,
    #[doc = "Gets the number of import machines part of this project."]
    #[serde(rename = "numberOfImportMachines", default, skip_serializing_if = "Option::is_none")]
    pub number_of_import_machines: Option<i32>,
    #[doc = "Gets the number of assessments created in this project."]
    #[serde(rename = "numberOfAssessments", default, skip_serializing_if = "Option::is_none")]
    pub number_of_assessments: Option<i32>,
    #[doc = "Gets the last assessment timestamp."]
    #[serde(rename = "lastAssessmentTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub last_assessment_timestamp: Option<time::OffsetDateTime>,
}
impl AssessmentProjectSummaryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type used for update operations of the AssessmentProject."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessmentProjectUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the AssessmentProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssessmentProjectUpdateProperties>,
}
impl AssessmentProjectUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the AssessmentProject."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessmentProjectUpdateProperties {
    #[doc = "Assessment solution ARM id tracked by Microsoft.Migrate/migrateProjects."]
    #[serde(rename = "assessmentSolutionId", default, skip_serializing_if = "Option::is_none")]
    pub assessment_solution_id: Option<String>,
    #[doc = "Project Status."]
    #[serde(rename = "projectStatus", default, skip_serializing_if = "Option::is_none")]
    pub project_status: Option<ProjectStatus>,
    #[doc = "The ARM id of service map workspace created by customer."]
    #[serde(rename = "customerWorkspaceId", default, skip_serializing_if = "Option::is_none")]
    pub customer_workspace_id: Option<String>,
    #[doc = "Location of service map workspace created by customer."]
    #[serde(rename = "customerWorkspaceLocation", default, skip_serializing_if = "Option::is_none")]
    pub customer_workspace_location: Option<String>,
    #[doc = "This value can be set to 'enabled' to avoid breaking changes on existing\ncustomer resources and templates. If set to 'disabled', traffic over public\ninterface is not allowed, and private endpoint connections would be the\nexclusive access method."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<String>,
    #[doc = "The ARM id of the storage account used for interactions when public access is\ndisabled."]
    #[serde(rename = "customerStorageAccountArmId", default, skip_serializing_if = "Option::is_none")]
    pub customer_storage_account_arm_id: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl AssessmentProjectUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Assessment Sizing Criteria."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AssessmentSizingCriterion")]
pub enum AssessmentSizingCriterion {
    PerformanceBased,
    AsOnPremises,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AssessmentSizingCriterion {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AssessmentSizingCriterion {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AssessmentSizingCriterion {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::PerformanceBased => serializer.serialize_unit_variant("AssessmentSizingCriterion", 0u32, "PerformanceBased"),
            Self::AsOnPremises => serializer.serialize_unit_variant("AssessmentSizingCriterion", 1u32, "AsOnPremises"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AssessmentStage")]
pub enum AssessmentStage {
    InProgress,
    UnderReview,
    Approved,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AssessmentStage {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AssessmentStage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AssessmentStage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::InProgress => serializer.serialize_unit_variant("AssessmentStage", 0u32, "InProgress"),
            Self::UnderReview => serializer.serialize_unit_variant("AssessmentStage", 1u32, "UnderReview"),
            Self::Approved => serializer.serialize_unit_variant("AssessmentStage", 2u32, "Approved"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Assessment Status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AssessmentStatus")]
pub enum AssessmentStatus {
    Created,
    Updated,
    Running,
    Completed,
    Invalid,
    OutOfSync,
    OutDated,
    Deleted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AssessmentStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AssessmentStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AssessmentStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Created => serializer.serialize_unit_variant("AssessmentStatus", 0u32, "Created"),
            Self::Updated => serializer.serialize_unit_variant("AssessmentStatus", 1u32, "Updated"),
            Self::Running => serializer.serialize_unit_variant("AssessmentStatus", 2u32, "Running"),
            Self::Completed => serializer.serialize_unit_variant("AssessmentStatus", 3u32, "Completed"),
            Self::Invalid => serializer.serialize_unit_variant("AssessmentStatus", 4u32, "Invalid"),
            Self::OutOfSync => serializer.serialize_unit_variant("AssessmentStatus", 5u32, "OutOfSync"),
            Self::OutDated => serializer.serialize_unit_variant("AssessmentStatus", 6u32, "OutDated"),
            Self::Deleted => serializer.serialize_unit_variant("AssessmentStatus", 7u32, "Deleted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AssessmentType")]
pub enum AssessmentType {
    Unknown,
    MachineAssessment,
    AvsAssessment,
    SqlAssessment,
    WebAppAssessment,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AssessmentType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AssessmentType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AssessmentType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AssessmentType", 0u32, "Unknown"),
            Self::MachineAssessment => serializer.serialize_unit_variant("AssessmentType", 1u32, "MachineAssessment"),
            Self::AvsAssessment => serializer.serialize_unit_variant("AssessmentType", 2u32, "AvsAssessment"),
            Self::SqlAssessment => serializer.serialize_unit_variant("AssessmentType", 3u32, "SqlAssessment"),
            Self::WebAppAssessment => serializer.serialize_unit_variant("AssessmentType", 4u32, "WebAppAssessment"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AsyncCommitModeIntent")]
pub enum AsyncCommitModeIntent {
    None,
    HighAvailability,
    DisasterRecovery,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AsyncCommitModeIntent {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AsyncCommitModeIntent {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AsyncCommitModeIntent {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("AsyncCommitModeIntent", 0u32, "None"),
            Self::HighAvailability => serializer.serialize_unit_variant("AsyncCommitModeIntent", 1u32, "HighAvailability"),
            Self::DisasterRecovery => serializer.serialize_unit_variant("AsyncCommitModeIntent", 2u32, "DisasterRecovery"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "AVS assessed disk web model class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvsAssessedDisk {
    #[doc = "Gets the ID of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the display name of the disk."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gigabytes Provisioned for a disk in private data center."]
    #[serde(rename = "gigabytesProvisioned", default, skip_serializing_if = "Option::is_none")]
    pub gigabytes_provisioned: Option<f32>,
    #[doc = "Disk Read Throughput in MB/s."]
    #[serde(rename = "megabytesPerSecondOfRead", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_of_read: Option<f32>,
    #[doc = "Disk Write Throughput in MB/s."]
    #[serde(rename = "megabytesPerSecondOfWrite", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_of_write: Option<f32>,
    #[doc = "Read Operations per second."]
    #[serde(rename = "numberOfReadOperationsPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub number_of_read_operations_per_second: Option<f32>,
    #[doc = "Write Operations per second."]
    #[serde(rename = "numberOfWriteOperationsPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub number_of_write_operations_per_second: Option<f32>,
}
impl AvsAssessedDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AVS assessment Assessed Machine resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvsAssessedMachine {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "AVS assessed machine properties web model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvsAssessedMachineProperties>,
}
impl AvsAssessedMachine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a AvsAssessedMachine list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvsAssessedMachineListResult {
    #[doc = "The AvsAssessedMachine items on this page"]
    pub value: Vec<AvsAssessedMachine>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AvsAssessedMachineListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AvsAssessedMachineListResult {
    pub fn new(value: Vec<AvsAssessedMachine>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "AVS assessed machine properties web model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvsAssessedMachineProperties {
    #[doc = "List of errors for this machine."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<Error>,
    #[doc = "List of Disks that were assessed as part of this machine's assessment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disks: Option<serde_json::Value>,
    #[doc = "List of Network Adapters that were assessed as part of this machine's\nassessment."]
    #[serde(rename = "networkAdapters", default, skip_serializing_if = "Option::is_none")]
    pub network_adapters: Option<serde_json::Value>,
    #[doc = "Gets the storage in use."]
    #[serde(rename = "storageInUseGB", default, skip_serializing_if = "Option::is_none")]
    pub storage_in_use_gb: Option<f32>,
    #[serde(rename = "suitabilityExplanation", default, skip_serializing_if = "Option::is_none")]
    pub suitability_explanation: Option<AzureAvsVmSuitabilityExplanation>,
    #[serde(rename = "suitabilityDetail", default, skip_serializing_if = "Option::is_none")]
    pub suitability_detail: Option<AzureAvsVmSuitabilityDetail>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<AssessedMachineType>,
    #[serde(rename = "bootType", default, skip_serializing_if = "Option::is_none")]
    pub boot_type: Option<MachineBootType>,
    #[doc = "Operating system as reported by datacenter management solution."]
    #[serde(rename = "operatingSystemType", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_type: Option<String>,
    #[doc = "Operating system as reported by datacenter management solution."]
    #[serde(rename = "operatingSystemName", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_name: Option<String>,
    #[doc = "Operating system version as reported by datacenter management solution."]
    #[serde(rename = "operatingSystemVersion", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_version: Option<String>,
    #[serde(rename = "operatingSystemArchitecture", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_architecture: Option<GuestOperatingSystemArchitecture>,
    #[doc = "When was machine first created."]
    #[serde(rename = "createdTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "When was machine last updated."]
    #[serde(rename = "updatedTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Display Name of the Machine."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Description for the machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Data center machine ARM id."]
    #[serde(rename = "datacenterMachineArmId", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_machine_arm_id: Option<String>,
    #[doc = "Data center management server ARM id."]
    #[serde(rename = "datacenterManagementServerArmId", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_management_server_arm_id: Option<String>,
    #[doc = "Data center management server name."]
    #[serde(rename = "datacenterManagementServerName", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_management_server_name: Option<String>,
    #[doc = "Megabytes of memory found allocated for the machine in private data center."]
    #[serde(rename = "megabytesOfMemory", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_of_memory: Option<f32>,
    #[doc = "Number of CPU cores found on the machine."]
    #[serde(rename = "numberOfCores", default, skip_serializing_if = "Option::is_none")]
    pub number_of_cores: Option<i32>,
    #[doc = "Confidence Rating in Percentage."]
    #[serde(rename = "confidenceRatingInPercentage", default, skip_serializing_if = "Option::is_none")]
    pub confidence_rating_in_percentage: Option<f32>,
    #[doc = "Percentile of Percentage of Cores Utilized noted during time period T.\n        \n   Here N and T are settings on Assessment."]
    #[serde(rename = "percentageCoresUtilization", default, skip_serializing_if = "Option::is_none")]
    pub percentage_cores_utilization: Option<f32>,
    #[doc = "Percentile of Percentage of Memory Utilized noted during time period T.\n       \n    Here N and T are settings on Assessment."]
    #[serde(rename = "percentageMemoryUtilization", default, skip_serializing_if = "Option::is_none")]
    pub percentage_memory_utilization: Option<f32>,
    #[doc = "Cloud Suitability for Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suitability: Option<CloudSuitability>,
}
impl AvsAssessedMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Second level object returned as part of AVS AssessedMachine REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvsAssessedNetworkAdapter {
    #[doc = "Mac address of the NIC."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "IP V4 addresses for the machine."]
    #[serde(
        rename = "ipAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_addresses: Vec<String>,
    #[doc = "Gets the display name of the network adapter."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets the Received data for Network Adapter in MB/s.\n            This value is\nthe percentile of historical data based on options selected in Assessment."]
    #[serde(rename = "megabytesPerSecondReceived", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_received: Option<f32>,
    #[doc = "Gets the Transmitted data for Network Adapter in MB/s.\n            This value\nis the percentile of historical data based on options selected in Assessment."]
    #[serde(rename = "megabytesPerSecondTransmitted", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_transmitted: Option<f32>,
}
impl AvsAssessedNetworkAdapter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AVS assessment resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvsAssessment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of the AVS assessment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvsAssessmentProperties>,
}
impl AvsAssessment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a AvsAssessment list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvsAssessmentListResult {
    #[doc = "The AvsAssessment items on this page"]
    pub value: Vec<AvsAssessment>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AvsAssessmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AvsAssessmentListResult {
    pub fn new(value: Vec<AvsAssessment>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "AVS Assessment options resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvsAssessmentOptions {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "AVS Assessment options properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvsAssessmentOptionsProperties>,
}
impl AvsAssessmentOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a AvsAssessmentOptions list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvsAssessmentOptionsListResult {
    #[doc = "The AvsAssessmentOptions items on this page"]
    pub value: Vec<AvsAssessmentOptions>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AvsAssessmentOptionsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AvsAssessmentOptionsListResult {
    pub fn new(value: Vec<AvsAssessmentOptions>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "AVS Assessment options properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvsAssessmentOptionsProperties {
    #[doc = "AVS SKU Nodes."]
    #[serde(
        rename = "avsNodes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub avs_nodes: Vec<AvsSkuOptions>,
    #[doc = "FTT and Raid level values."]
    #[serde(
        rename = "failuresToTolerateAndRaidLevelValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub failures_to_tolerate_and_raid_level_values: Vec<FttAndRaidLevel>,
    #[doc = "List of AVS nodes for RI."]
    #[serde(
        rename = "reservedInstanceAvsNodes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reserved_instance_avs_nodes: Vec<AzureAvsNodeType>,
    #[doc = "List of supported Azure regions for reserved instances."]
    #[serde(
        rename = "reservedInstanceSupportedLocations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reserved_instance_supported_locations: Vec<AzureLocation>,
    #[doc = "List of supported currencies for reserved instances."]
    #[serde(
        rename = "reservedInstanceSupportedCurrencies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reserved_instance_supported_currencies: Vec<AzureCurrency>,
    #[doc = "List of supported Azure offer codes for reserved instances."]
    #[serde(
        rename = "reservedInstanceSupportedOffers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reserved_instance_supported_offers: Vec<AzureOfferCode>,
}
impl AvsAssessmentOptionsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the AVS assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvsAssessmentProperties {
    #[serde(flatten)]
    pub azure_resource_properties: AzureResourceProperties,
    #[doc = "Gets the assessment error summary.\n            This is the number of machines\naffected by each type of error in this assessment."]
    #[serde(rename = "assessmentErrorSummary", default, skip_serializing_if = "Option::is_none")]
    pub assessment_error_summary: Option<serde_json::Value>,
    #[serde(rename = "failuresToTolerateAndRaidLevel", default, skip_serializing_if = "Option::is_none")]
    pub failures_to_tolerate_and_raid_level: Option<FttAndRaidLevel>,
    #[doc = "VCPU over subscription."]
    #[serde(rename = "vcpuOversubscription", default, skip_serializing_if = "Option::is_none")]
    pub vcpu_oversubscription: Option<f32>,
    #[serde(rename = "nodeType", default, skip_serializing_if = "Option::is_none")]
    pub node_type: Option<AzureAvsNodeType>,
    #[serde(rename = "reservedInstance", default, skip_serializing_if = "Option::is_none")]
    pub reserved_instance: Option<AzureReservedInstance>,
    #[doc = "Total monthly cost."]
    #[serde(rename = "totalMonthlyCost", default, skip_serializing_if = "Option::is_none")]
    pub total_monthly_cost: Option<f32>,
    #[doc = "Cloud Suitability for Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suitability: Option<CloudSuitability>,
    #[serde(rename = "suitabilityExplanation", default, skip_serializing_if = "Option::is_none")]
    pub suitability_explanation: Option<AzureAvsSuitabilityExplanation>,
    #[doc = "Recommended number of nodes."]
    #[serde(rename = "numberOfNodes", default, skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i32>,
    #[doc = "Predicted CPU utilization."]
    #[serde(rename = "cpuUtilization", default, skip_serializing_if = "Option::is_none")]
    pub cpu_utilization: Option<f32>,
    #[doc = "Predicted RAM utilization."]
    #[serde(rename = "ramUtilization", default, skip_serializing_if = "Option::is_none")]
    pub ram_utilization: Option<f32>,
    #[doc = "Predicted storage utilization."]
    #[serde(rename = "storageUtilization", default, skip_serializing_if = "Option::is_none")]
    pub storage_utilization: Option<f32>,
    #[doc = "Predicted total CPU cores used."]
    #[serde(rename = "totalCpuCores", default, skip_serializing_if = "Option::is_none")]
    pub total_cpu_cores: Option<f32>,
    #[doc = "Predicted total RAM used in GB."]
    #[serde(rename = "totalRamInGB", default, skip_serializing_if = "Option::is_none")]
    pub total_ram_in_gb: Option<f32>,
    #[doc = "Predicted total Storage used in GB."]
    #[serde(rename = "totalStorageInGB", default, skip_serializing_if = "Option::is_none")]
    pub total_storage_in_gb: Option<f32>,
    #[doc = "Number of machines part of the assessment."]
    #[serde(rename = "numberOfMachines", default, skip_serializing_if = "Option::is_none")]
    pub number_of_machines: Option<i32>,
    #[doc = "Cloud suitability summary for all the machines in the assessment."]
    #[serde(rename = "suitabilitySummary", default, skip_serializing_if = "Option::is_none")]
    pub suitability_summary: Option<serde_json::Value>,
    #[doc = "Memory overcommit."]
    #[serde(rename = "memOvercommit", default, skip_serializing_if = "Option::is_none")]
    pub mem_overcommit: Option<f32>,
    #[doc = "De-duplication compression."]
    #[serde(rename = "dedupeCompression", default, skip_serializing_if = "Option::is_none")]
    pub dedupe_compression: Option<f32>,
    #[doc = "Limiting factor."]
    #[serde(rename = "limitingFactor", default, skip_serializing_if = "Option::is_none")]
    pub limiting_factor: Option<String>,
    #[doc = "Is Stretch Cluster Enabled."]
    #[serde(rename = "isStretchClusterEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_stretch_cluster_enabled: Option<bool>,
    #[serde(rename = "groupType", default, skip_serializing_if = "Option::is_none")]
    pub group_type: Option<GroupType>,
    #[serde(rename = "assessmentType", default, skip_serializing_if = "Option::is_none")]
    pub assessment_type: Option<AssessmentType>,
    #[doc = "Location for Azure."]
    #[serde(rename = "azureLocation", default, skip_serializing_if = "Option::is_none")]
    pub azure_location: Option<AzureLocation>,
    #[serde(rename = "azureOfferCode", default, skip_serializing_if = "Option::is_none")]
    pub azure_offer_code: Option<AzureOfferCode>,
    #[doc = "Currency for Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<AzureCurrency>,
    #[doc = "Percentage of buffer that user wants on performance metrics when recommending\nAzure sizes."]
    #[serde(rename = "scalingFactor", default, skip_serializing_if = "Option::is_none")]
    pub scaling_factor: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentile: Option<Percentile>,
    #[serde(rename = "timeRange", default, skip_serializing_if = "Option::is_none")]
    pub time_range: Option<TimeRange>,
    #[doc = "Gets or sets the start time to consider performance data for assessment."]
    #[serde(rename = "perfDataStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub perf_data_start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the end time to consider performance data for assessment."]
    #[serde(rename = "perfDataEndTime", default, with = "azure_core::date::rfc3339::option")]
    pub perf_data_end_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<AssessmentStage>,
    #[doc = "Custom discount percentage."]
    #[serde(rename = "discountPercentage", default, skip_serializing_if = "Option::is_none")]
    pub discount_percentage: Option<f32>,
    #[doc = "Assessment Sizing Criteria."]
    #[serde(rename = "sizingCriterion", default, skip_serializing_if = "Option::is_none")]
    pub sizing_criterion: Option<AssessmentSizingCriterion>,
    #[doc = "Confidence Rating in Percentage."]
    #[serde(rename = "confidenceRatingInPercentage", default, skip_serializing_if = "Option::is_none")]
    pub confidence_rating_in_percentage: Option<f32>,
    #[doc = "Time when the Azure Prices were queried. Date-Time represented in ISO-8601\nformat."]
    #[serde(rename = "pricesTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub prices_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Date and Time when assessment was created."]
    #[serde(rename = "createdTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Date and Time when assessment was last updated."]
    #[serde(rename = "updatedTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Assessment Status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AssessmentStatus>,
    #[doc = "Schema version."]
    #[serde(rename = "schemaVersion", default, skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
}
impl AvsAssessmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AVS SKU specific options."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvsSkuOptions {
    #[serde(rename = "nodeType", default, skip_serializing_if = "Option::is_none")]
    pub node_type: Option<AzureAvsNodeType>,
    #[doc = "List of locations where this node type is available."]
    #[serde(
        rename = "targetLocations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub target_locations: Vec<AzureLocation>,
}
impl AvsSkuOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureAvsNodeType")]
pub enum AzureAvsNodeType {
    Unknown,
    #[serde(rename = "AV36")]
    Av36,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureAvsNodeType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureAvsNodeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureAvsNodeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureAvsNodeType", 0u32, "Unknown"),
            Self::Av36 => serializer.serialize_unit_variant("AzureAvsNodeType", 1u32, "AV36"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureAvsSuitabilityExplanation")]
pub enum AzureAvsSuitabilityExplanation {
    Unknown,
    NotApplicable,
    UnsupportedLocationForSelectedNode,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureAvsSuitabilityExplanation {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureAvsSuitabilityExplanation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureAvsSuitabilityExplanation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureAvsSuitabilityExplanation", 0u32, "Unknown"),
            Self::NotApplicable => serializer.serialize_unit_variant("AzureAvsSuitabilityExplanation", 1u32, "NotApplicable"),
            Self::UnsupportedLocationForSelectedNode => {
                serializer.serialize_unit_variant("AzureAvsSuitabilityExplanation", 2u32, "UnsupportedLocationForSelectedNode")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureAvsVmSuitabilityDetail")]
pub enum AzureAvsVmSuitabilityDetail {
    None,
    PercentageOfCoresUtilizedMissing,
    PercentageOfMemoryUtilizedMissing,
    PercentageOfCoresUtilizedOutOfRange,
    PercentageOfMemoryUtilizedOutOfRange,
    PercentageOfStorageUtilizedOutOfRange,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureAvsVmSuitabilityDetail {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureAvsVmSuitabilityDetail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureAvsVmSuitabilityDetail {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("AzureAvsVmSuitabilityDetail", 0u32, "None"),
            Self::PercentageOfCoresUtilizedMissing => {
                serializer.serialize_unit_variant("AzureAvsVmSuitabilityDetail", 1u32, "PercentageOfCoresUtilizedMissing")
            }
            Self::PercentageOfMemoryUtilizedMissing => {
                serializer.serialize_unit_variant("AzureAvsVmSuitabilityDetail", 2u32, "PercentageOfMemoryUtilizedMissing")
            }
            Self::PercentageOfCoresUtilizedOutOfRange => {
                serializer.serialize_unit_variant("AzureAvsVmSuitabilityDetail", 3u32, "PercentageOfCoresUtilizedOutOfRange")
            }
            Self::PercentageOfMemoryUtilizedOutOfRange => {
                serializer.serialize_unit_variant("AzureAvsVmSuitabilityDetail", 4u32, "PercentageOfMemoryUtilizedOutOfRange")
            }
            Self::PercentageOfStorageUtilizedOutOfRange => {
                serializer.serialize_unit_variant("AzureAvsVmSuitabilityDetail", 5u32, "PercentageOfStorageUtilizedOutOfRange")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureAvsVmSuitabilityExplanation")]
pub enum AzureAvsVmSuitabilityExplanation {
    Unknown,
    NotApplicable,
    IpV6NotSupported,
    UnsupportedOperatingSystem,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureAvsVmSuitabilityExplanation {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureAvsVmSuitabilityExplanation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureAvsVmSuitabilityExplanation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureAvsVmSuitabilityExplanation", 0u32, "Unknown"),
            Self::NotApplicable => serializer.serialize_unit_variant("AzureAvsVmSuitabilityExplanation", 1u32, "NotApplicable"),
            Self::IpV6NotSupported => serializer.serialize_unit_variant("AzureAvsVmSuitabilityExplanation", 2u32, "IpV6NotSupported"),
            Self::UnsupportedOperatingSystem => {
                serializer.serialize_unit_variant("AzureAvsVmSuitabilityExplanation", 3u32, "UnsupportedOperatingSystem")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Currency for Azure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureCurrency")]
pub enum AzureCurrency {
    Unknown,
    #[serde(rename = "USD")]
    Usd,
    #[serde(rename = "DKK")]
    Dkk,
    #[serde(rename = "CAD")]
    Cad,
    #[serde(rename = "IDR")]
    Idr,
    #[serde(rename = "JPY")]
    Jpy,
    #[serde(rename = "KRW")]
    Krw,
    #[serde(rename = "NZD")]
    Nzd,
    #[serde(rename = "NOK")]
    Nok,
    #[serde(rename = "RUB")]
    Rub,
    #[serde(rename = "SAR")]
    Sar,
    #[serde(rename = "ZAR")]
    Zar,
    #[serde(rename = "SEK")]
    Sek,
    #[serde(rename = "TRY")]
    Try,
    #[serde(rename = "GBP")]
    Gbp,
    #[serde(rename = "MXN")]
    Mxn,
    #[serde(rename = "MYR")]
    Myr,
    #[serde(rename = "INR")]
    Inr,
    #[serde(rename = "HKD")]
    Hkd,
    #[serde(rename = "BRL")]
    Brl,
    #[serde(rename = "TWD")]
    Twd,
    #[serde(rename = "EUR")]
    Eur,
    #[serde(rename = "CHF")]
    Chf,
    #[serde(rename = "ARS")]
    Ars,
    #[serde(rename = "AUD")]
    Aud,
    #[serde(rename = "CNY")]
    Cny,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureCurrency {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureCurrency {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureCurrency {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureCurrency", 0u32, "Unknown"),
            Self::Usd => serializer.serialize_unit_variant("AzureCurrency", 1u32, "USD"),
            Self::Dkk => serializer.serialize_unit_variant("AzureCurrency", 2u32, "DKK"),
            Self::Cad => serializer.serialize_unit_variant("AzureCurrency", 3u32, "CAD"),
            Self::Idr => serializer.serialize_unit_variant("AzureCurrency", 4u32, "IDR"),
            Self::Jpy => serializer.serialize_unit_variant("AzureCurrency", 5u32, "JPY"),
            Self::Krw => serializer.serialize_unit_variant("AzureCurrency", 6u32, "KRW"),
            Self::Nzd => serializer.serialize_unit_variant("AzureCurrency", 7u32, "NZD"),
            Self::Nok => serializer.serialize_unit_variant("AzureCurrency", 8u32, "NOK"),
            Self::Rub => serializer.serialize_unit_variant("AzureCurrency", 9u32, "RUB"),
            Self::Sar => serializer.serialize_unit_variant("AzureCurrency", 10u32, "SAR"),
            Self::Zar => serializer.serialize_unit_variant("AzureCurrency", 11u32, "ZAR"),
            Self::Sek => serializer.serialize_unit_variant("AzureCurrency", 12u32, "SEK"),
            Self::Try => serializer.serialize_unit_variant("AzureCurrency", 13u32, "TRY"),
            Self::Gbp => serializer.serialize_unit_variant("AzureCurrency", 14u32, "GBP"),
            Self::Mxn => serializer.serialize_unit_variant("AzureCurrency", 15u32, "MXN"),
            Self::Myr => serializer.serialize_unit_variant("AzureCurrency", 16u32, "MYR"),
            Self::Inr => serializer.serialize_unit_variant("AzureCurrency", 17u32, "INR"),
            Self::Hkd => serializer.serialize_unit_variant("AzureCurrency", 18u32, "HKD"),
            Self::Brl => serializer.serialize_unit_variant("AzureCurrency", 19u32, "BRL"),
            Self::Twd => serializer.serialize_unit_variant("AzureCurrency", 20u32, "TWD"),
            Self::Eur => serializer.serialize_unit_variant("AzureCurrency", 21u32, "EUR"),
            Self::Chf => serializer.serialize_unit_variant("AzureCurrency", 22u32, "CHF"),
            Self::Ars => serializer.serialize_unit_variant("AzureCurrency", 23u32, "ARS"),
            Self::Aud => serializer.serialize_unit_variant("AzureCurrency", 24u32, "AUD"),
            Self::Cny => serializer.serialize_unit_variant("AzureCurrency", 25u32, "CNY"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureDiskSize")]
pub enum AzureDiskSize {
    Unknown,
    #[serde(rename = "Standard_S4")]
    StandardS4,
    #[serde(rename = "Standard_S6")]
    StandardS6,
    #[serde(rename = "Standard_S10")]
    StandardS10,
    #[serde(rename = "Standard_S15")]
    StandardS15,
    #[serde(rename = "Standard_S20")]
    StandardS20,
    #[serde(rename = "Standard_S30")]
    StandardS30,
    #[serde(rename = "Standard_S40")]
    StandardS40,
    #[serde(rename = "Standard_S50")]
    StandardS50,
    #[serde(rename = "Standard_S60")]
    StandardS60,
    #[serde(rename = "Standard_S70")]
    StandardS70,
    #[serde(rename = "Standard_S80")]
    StandardS80,
    #[serde(rename = "Premium_P4")]
    PremiumP4,
    #[serde(rename = "Premium_P6")]
    PremiumP6,
    #[serde(rename = "Premium_P10")]
    PremiumP10,
    #[serde(rename = "Premium_P15")]
    PremiumP15,
    #[serde(rename = "Premium_P20")]
    PremiumP20,
    #[serde(rename = "Premium_P30")]
    PremiumP30,
    #[serde(rename = "Premium_P40")]
    PremiumP40,
    #[serde(rename = "Premium_P50")]
    PremiumP50,
    #[serde(rename = "Premium_P60")]
    PremiumP60,
    #[serde(rename = "Premium_P70")]
    PremiumP70,
    #[serde(rename = "Premium_P80")]
    PremiumP80,
    #[serde(rename = "StandardSSD_E10")]
    StandardSsdE10,
    #[serde(rename = "StandardSSD_E15")]
    StandardSsdE15,
    #[serde(rename = "StandardSSD_E20")]
    StandardSsdE20,
    #[serde(rename = "StandardSSD_E30")]
    StandardSsdE30,
    #[serde(rename = "StandardSSD_E40")]
    StandardSsdE40,
    #[serde(rename = "StandardSSD_E50")]
    StandardSsdE50,
    #[serde(rename = "StandardSSD_E60")]
    StandardSsdE60,
    #[serde(rename = "StandardSSD_E70")]
    StandardSsdE70,
    #[serde(rename = "StandardSSD_E80")]
    StandardSsdE80,
    #[serde(rename = "StandardSSD_E4")]
    StandardSsdE4,
    #[serde(rename = "StandardSSD_E6")]
    StandardSsdE6,
    #[serde(rename = "StandardSSD_E1")]
    StandardSsdE1,
    #[serde(rename = "StandardSSD_E2")]
    StandardSsdE2,
    #[serde(rename = "StandardSSD_E3")]
    StandardSsdE3,
    #[serde(rename = "Premium_P1")]
    PremiumP1,
    #[serde(rename = "Premium_P2")]
    PremiumP2,
    #[serde(rename = "Premium_P3")]
    PremiumP3,
    Ultra,
    PremiumV2,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureDiskSize {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureDiskSize {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureDiskSize {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureDiskSize", 0u32, "Unknown"),
            Self::StandardS4 => serializer.serialize_unit_variant("AzureDiskSize", 1u32, "Standard_S4"),
            Self::StandardS6 => serializer.serialize_unit_variant("AzureDiskSize", 2u32, "Standard_S6"),
            Self::StandardS10 => serializer.serialize_unit_variant("AzureDiskSize", 3u32, "Standard_S10"),
            Self::StandardS15 => serializer.serialize_unit_variant("AzureDiskSize", 4u32, "Standard_S15"),
            Self::StandardS20 => serializer.serialize_unit_variant("AzureDiskSize", 5u32, "Standard_S20"),
            Self::StandardS30 => serializer.serialize_unit_variant("AzureDiskSize", 6u32, "Standard_S30"),
            Self::StandardS40 => serializer.serialize_unit_variant("AzureDiskSize", 7u32, "Standard_S40"),
            Self::StandardS50 => serializer.serialize_unit_variant("AzureDiskSize", 8u32, "Standard_S50"),
            Self::StandardS60 => serializer.serialize_unit_variant("AzureDiskSize", 9u32, "Standard_S60"),
            Self::StandardS70 => serializer.serialize_unit_variant("AzureDiskSize", 10u32, "Standard_S70"),
            Self::StandardS80 => serializer.serialize_unit_variant("AzureDiskSize", 11u32, "Standard_S80"),
            Self::PremiumP4 => serializer.serialize_unit_variant("AzureDiskSize", 12u32, "Premium_P4"),
            Self::PremiumP6 => serializer.serialize_unit_variant("AzureDiskSize", 13u32, "Premium_P6"),
            Self::PremiumP10 => serializer.serialize_unit_variant("AzureDiskSize", 14u32, "Premium_P10"),
            Self::PremiumP15 => serializer.serialize_unit_variant("AzureDiskSize", 15u32, "Premium_P15"),
            Self::PremiumP20 => serializer.serialize_unit_variant("AzureDiskSize", 16u32, "Premium_P20"),
            Self::PremiumP30 => serializer.serialize_unit_variant("AzureDiskSize", 17u32, "Premium_P30"),
            Self::PremiumP40 => serializer.serialize_unit_variant("AzureDiskSize", 18u32, "Premium_P40"),
            Self::PremiumP50 => serializer.serialize_unit_variant("AzureDiskSize", 19u32, "Premium_P50"),
            Self::PremiumP60 => serializer.serialize_unit_variant("AzureDiskSize", 20u32, "Premium_P60"),
            Self::PremiumP70 => serializer.serialize_unit_variant("AzureDiskSize", 21u32, "Premium_P70"),
            Self::PremiumP80 => serializer.serialize_unit_variant("AzureDiskSize", 22u32, "Premium_P80"),
            Self::StandardSsdE10 => serializer.serialize_unit_variant("AzureDiskSize", 23u32, "StandardSSD_E10"),
            Self::StandardSsdE15 => serializer.serialize_unit_variant("AzureDiskSize", 24u32, "StandardSSD_E15"),
            Self::StandardSsdE20 => serializer.serialize_unit_variant("AzureDiskSize", 25u32, "StandardSSD_E20"),
            Self::StandardSsdE30 => serializer.serialize_unit_variant("AzureDiskSize", 26u32, "StandardSSD_E30"),
            Self::StandardSsdE40 => serializer.serialize_unit_variant("AzureDiskSize", 27u32, "StandardSSD_E40"),
            Self::StandardSsdE50 => serializer.serialize_unit_variant("AzureDiskSize", 28u32, "StandardSSD_E50"),
            Self::StandardSsdE60 => serializer.serialize_unit_variant("AzureDiskSize", 29u32, "StandardSSD_E60"),
            Self::StandardSsdE70 => serializer.serialize_unit_variant("AzureDiskSize", 30u32, "StandardSSD_E70"),
            Self::StandardSsdE80 => serializer.serialize_unit_variant("AzureDiskSize", 31u32, "StandardSSD_E80"),
            Self::StandardSsdE4 => serializer.serialize_unit_variant("AzureDiskSize", 32u32, "StandardSSD_E4"),
            Self::StandardSsdE6 => serializer.serialize_unit_variant("AzureDiskSize", 33u32, "StandardSSD_E6"),
            Self::StandardSsdE1 => serializer.serialize_unit_variant("AzureDiskSize", 34u32, "StandardSSD_E1"),
            Self::StandardSsdE2 => serializer.serialize_unit_variant("AzureDiskSize", 35u32, "StandardSSD_E2"),
            Self::StandardSsdE3 => serializer.serialize_unit_variant("AzureDiskSize", 36u32, "StandardSSD_E3"),
            Self::PremiumP1 => serializer.serialize_unit_variant("AzureDiskSize", 37u32, "Premium_P1"),
            Self::PremiumP2 => serializer.serialize_unit_variant("AzureDiskSize", 38u32, "Premium_P2"),
            Self::PremiumP3 => serializer.serialize_unit_variant("AzureDiskSize", 39u32, "Premium_P3"),
            Self::Ultra => serializer.serialize_unit_variant("AzureDiskSize", 40u32, "Ultra"),
            Self::PremiumV2 => serializer.serialize_unit_variant("AzureDiskSize", 41u32, "PremiumV2"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureDiskSuitabilityDetail")]
pub enum AzureDiskSuitabilityDetail {
    None,
    NumberOfReadOperationsPerSecondMissing,
    NumberOfWriteOperationsPerSecondMissing,
    MegabytesPerSecondOfReadMissing,
    MegabytesPerSecondOfWriteMissing,
    DiskGigabytesConsumedMissing,
    DiskGigabytesProvisionedMissing,
    NumberOfReadOperationsPerSecondOutOfRange,
    NumberOfWriteOperationsPerSecondOutOfRange,
    MegabytesPerSecondOfReadOutOfRange,
    MegabytesPerSecondOfWriteOutOfRange,
    DiskGigabytesConsumedOutOfRange,
    DiskGigabytesProvisionedOutOfRange,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureDiskSuitabilityDetail {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureDiskSuitabilityDetail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureDiskSuitabilityDetail {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("AzureDiskSuitabilityDetail", 0u32, "None"),
            Self::NumberOfReadOperationsPerSecondMissing => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityDetail", 1u32, "NumberOfReadOperationsPerSecondMissing")
            }
            Self::NumberOfWriteOperationsPerSecondMissing => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityDetail", 2u32, "NumberOfWriteOperationsPerSecondMissing")
            }
            Self::MegabytesPerSecondOfReadMissing => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityDetail", 3u32, "MegabytesPerSecondOfReadMissing")
            }
            Self::MegabytesPerSecondOfWriteMissing => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityDetail", 4u32, "MegabytesPerSecondOfWriteMissing")
            }
            Self::DiskGigabytesConsumedMissing => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityDetail", 5u32, "DiskGigabytesConsumedMissing")
            }
            Self::DiskGigabytesProvisionedMissing => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityDetail", 6u32, "DiskGigabytesProvisionedMissing")
            }
            Self::NumberOfReadOperationsPerSecondOutOfRange => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityDetail", 7u32, "NumberOfReadOperationsPerSecondOutOfRange")
            }
            Self::NumberOfWriteOperationsPerSecondOutOfRange => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityDetail", 8u32, "NumberOfWriteOperationsPerSecondOutOfRange")
            }
            Self::MegabytesPerSecondOfReadOutOfRange => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityDetail", 9u32, "MegabytesPerSecondOfReadOutOfRange")
            }
            Self::MegabytesPerSecondOfWriteOutOfRange => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityDetail", 10u32, "MegabytesPerSecondOfWriteOutOfRange")
            }
            Self::DiskGigabytesConsumedOutOfRange => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityDetail", 11u32, "DiskGigabytesConsumedOutOfRange")
            }
            Self::DiskGigabytesProvisionedOutOfRange => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityDetail", 12u32, "DiskGigabytesProvisionedOutOfRange")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureDiskSuitabilityExplanation")]
pub enum AzureDiskSuitabilityExplanation {
    Unknown,
    NotApplicable,
    DiskSizeGreaterThanSupported,
    NoSuitableDiskSizeForIops,
    NoSuitableDiskSizeForThroughput,
    NoDiskSizeFoundInSelectedLocation,
    NoDiskSizeFoundForSelectedRedundancy,
    InternalErrorOccurredForDiskEvaluation,
    NoEaPriceFoundForDiskSize,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureDiskSuitabilityExplanation {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureDiskSuitabilityExplanation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureDiskSuitabilityExplanation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureDiskSuitabilityExplanation", 0u32, "Unknown"),
            Self::NotApplicable => serializer.serialize_unit_variant("AzureDiskSuitabilityExplanation", 1u32, "NotApplicable"),
            Self::DiskSizeGreaterThanSupported => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityExplanation", 2u32, "DiskSizeGreaterThanSupported")
            }
            Self::NoSuitableDiskSizeForIops => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityExplanation", 3u32, "NoSuitableDiskSizeForIops")
            }
            Self::NoSuitableDiskSizeForThroughput => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityExplanation", 4u32, "NoSuitableDiskSizeForThroughput")
            }
            Self::NoDiskSizeFoundInSelectedLocation => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityExplanation", 5u32, "NoDiskSizeFoundInSelectedLocation")
            }
            Self::NoDiskSizeFoundForSelectedRedundancy => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityExplanation", 6u32, "NoDiskSizeFoundForSelectedRedundancy")
            }
            Self::InternalErrorOccurredForDiskEvaluation => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityExplanation", 7u32, "InternalErrorOccurredForDiskEvaluation")
            }
            Self::NoEaPriceFoundForDiskSize => {
                serializer.serialize_unit_variant("AzureDiskSuitabilityExplanation", 8u32, "NoEaPriceFoundForDiskSize")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureDiskType")]
pub enum AzureDiskType {
    Unknown,
    Standard,
    #[serde(rename = "StandardSSD")]
    StandardSsd,
    Premium,
    StandardOrPremium,
    Ultra,
    PremiumV2,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureDiskType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureDiskType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureDiskType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureDiskType", 0u32, "Unknown"),
            Self::Standard => serializer.serialize_unit_variant("AzureDiskType", 1u32, "Standard"),
            Self::StandardSsd => serializer.serialize_unit_variant("AzureDiskType", 2u32, "StandardSSD"),
            Self::Premium => serializer.serialize_unit_variant("AzureDiskType", 3u32, "Premium"),
            Self::StandardOrPremium => serializer.serialize_unit_variant("AzureDiskType", 4u32, "StandardOrPremium"),
            Self::Ultra => serializer.serialize_unit_variant("AzureDiskType", 5u32, "Ultra"),
            Self::PremiumV2 => serializer.serialize_unit_variant("AzureDiskType", 6u32, "PremiumV2"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureHybridUseBenefit")]
pub enum AzureHybridUseBenefit {
    Unknown,
    Yes,
    No,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureHybridUseBenefit {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureHybridUseBenefit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureHybridUseBenefit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureHybridUseBenefit", 0u32, "Unknown"),
            Self::Yes => serializer.serialize_unit_variant("AzureHybridUseBenefit", 1u32, "Yes"),
            Self::No => serializer.serialize_unit_variant("AzureHybridUseBenefit", 2u32, "No"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Location for Azure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureLocation")]
pub enum AzureLocation {
    Unknown,
    EastAsia,
    SoutheastAsia,
    AustraliaEast,
    AustraliaSoutheast,
    BrazilSouth,
    CanadaCentral,
    CanadaEast,
    WestEurope,
    NorthEurope,
    CentralIndia,
    SouthIndia,
    WestIndia,
    JapanEast,
    JapanWest,
    KoreaCentral,
    KoreaSouth,
    UkWest,
    UkSouth,
    NorthCentralUs,
    EastUs,
    WestUs2,
    SouthCentralUs,
    CentralUs,
    EastUs2,
    WestUs,
    WestCentralUs,
    GermanyCentral,
    GermanyNortheast,
    ChinaNorth,
    ChinaEast,
    #[serde(rename = "USGovArizona")]
    UsGovArizona,
    #[serde(rename = "USGovTexas")]
    UsGovTexas,
    #[serde(rename = "USGovIowa")]
    UsGovIowa,
    #[serde(rename = "USGovVirginia")]
    UsGovVirginia,
    #[serde(rename = "USDoDCentral")]
    UsDoDCentral,
    #[serde(rename = "USDoDEast")]
    UsDoDEast,
    FranceCentral,
    AustraliaCentral,
    SouthAfricaNorth,
    FranceSouth,
    AustraliaCentral2,
    SouthAfricaWest,
    GermanyNorth,
    GermanyWestCentral,
    NorwayEast,
    NorwayWest,
    ChinaEast2,
    ChinaNorth2,
    SwitzerlandNorth,
    SwitzerlandWest,
    #[serde(rename = "UAENorth")]
    UaeNorth,
    #[serde(rename = "UAECentral")]
    UaeCentral,
    UsNatEast,
    UsNatWest,
    UsSecEast,
    UsSecCentral,
    UsSecWest,
    SwedenCentral,
    QatarCentral,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureLocation {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureLocation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureLocation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureLocation", 0u32, "Unknown"),
            Self::EastAsia => serializer.serialize_unit_variant("AzureLocation", 1u32, "EastAsia"),
            Self::SoutheastAsia => serializer.serialize_unit_variant("AzureLocation", 2u32, "SoutheastAsia"),
            Self::AustraliaEast => serializer.serialize_unit_variant("AzureLocation", 3u32, "AustraliaEast"),
            Self::AustraliaSoutheast => serializer.serialize_unit_variant("AzureLocation", 4u32, "AustraliaSoutheast"),
            Self::BrazilSouth => serializer.serialize_unit_variant("AzureLocation", 5u32, "BrazilSouth"),
            Self::CanadaCentral => serializer.serialize_unit_variant("AzureLocation", 6u32, "CanadaCentral"),
            Self::CanadaEast => serializer.serialize_unit_variant("AzureLocation", 7u32, "CanadaEast"),
            Self::WestEurope => serializer.serialize_unit_variant("AzureLocation", 8u32, "WestEurope"),
            Self::NorthEurope => serializer.serialize_unit_variant("AzureLocation", 9u32, "NorthEurope"),
            Self::CentralIndia => serializer.serialize_unit_variant("AzureLocation", 10u32, "CentralIndia"),
            Self::SouthIndia => serializer.serialize_unit_variant("AzureLocation", 11u32, "SouthIndia"),
            Self::WestIndia => serializer.serialize_unit_variant("AzureLocation", 12u32, "WestIndia"),
            Self::JapanEast => serializer.serialize_unit_variant("AzureLocation", 13u32, "JapanEast"),
            Self::JapanWest => serializer.serialize_unit_variant("AzureLocation", 14u32, "JapanWest"),
            Self::KoreaCentral => serializer.serialize_unit_variant("AzureLocation", 15u32, "KoreaCentral"),
            Self::KoreaSouth => serializer.serialize_unit_variant("AzureLocation", 16u32, "KoreaSouth"),
            Self::UkWest => serializer.serialize_unit_variant("AzureLocation", 17u32, "UkWest"),
            Self::UkSouth => serializer.serialize_unit_variant("AzureLocation", 18u32, "UkSouth"),
            Self::NorthCentralUs => serializer.serialize_unit_variant("AzureLocation", 19u32, "NorthCentralUs"),
            Self::EastUs => serializer.serialize_unit_variant("AzureLocation", 20u32, "EastUs"),
            Self::WestUs2 => serializer.serialize_unit_variant("AzureLocation", 21u32, "WestUs2"),
            Self::SouthCentralUs => serializer.serialize_unit_variant("AzureLocation", 22u32, "SouthCentralUs"),
            Self::CentralUs => serializer.serialize_unit_variant("AzureLocation", 23u32, "CentralUs"),
            Self::EastUs2 => serializer.serialize_unit_variant("AzureLocation", 24u32, "EastUs2"),
            Self::WestUs => serializer.serialize_unit_variant("AzureLocation", 25u32, "WestUs"),
            Self::WestCentralUs => serializer.serialize_unit_variant("AzureLocation", 26u32, "WestCentralUs"),
            Self::GermanyCentral => serializer.serialize_unit_variant("AzureLocation", 27u32, "GermanyCentral"),
            Self::GermanyNortheast => serializer.serialize_unit_variant("AzureLocation", 28u32, "GermanyNortheast"),
            Self::ChinaNorth => serializer.serialize_unit_variant("AzureLocation", 29u32, "ChinaNorth"),
            Self::ChinaEast => serializer.serialize_unit_variant("AzureLocation", 30u32, "ChinaEast"),
            Self::UsGovArizona => serializer.serialize_unit_variant("AzureLocation", 31u32, "USGovArizona"),
            Self::UsGovTexas => serializer.serialize_unit_variant("AzureLocation", 32u32, "USGovTexas"),
            Self::UsGovIowa => serializer.serialize_unit_variant("AzureLocation", 33u32, "USGovIowa"),
            Self::UsGovVirginia => serializer.serialize_unit_variant("AzureLocation", 34u32, "USGovVirginia"),
            Self::UsDoDCentral => serializer.serialize_unit_variant("AzureLocation", 35u32, "USDoDCentral"),
            Self::UsDoDEast => serializer.serialize_unit_variant("AzureLocation", 36u32, "USDoDEast"),
            Self::FranceCentral => serializer.serialize_unit_variant("AzureLocation", 37u32, "FranceCentral"),
            Self::AustraliaCentral => serializer.serialize_unit_variant("AzureLocation", 38u32, "AustraliaCentral"),
            Self::SouthAfricaNorth => serializer.serialize_unit_variant("AzureLocation", 39u32, "SouthAfricaNorth"),
            Self::FranceSouth => serializer.serialize_unit_variant("AzureLocation", 40u32, "FranceSouth"),
            Self::AustraliaCentral2 => serializer.serialize_unit_variant("AzureLocation", 41u32, "AustraliaCentral2"),
            Self::SouthAfricaWest => serializer.serialize_unit_variant("AzureLocation", 42u32, "SouthAfricaWest"),
            Self::GermanyNorth => serializer.serialize_unit_variant("AzureLocation", 43u32, "GermanyNorth"),
            Self::GermanyWestCentral => serializer.serialize_unit_variant("AzureLocation", 44u32, "GermanyWestCentral"),
            Self::NorwayEast => serializer.serialize_unit_variant("AzureLocation", 45u32, "NorwayEast"),
            Self::NorwayWest => serializer.serialize_unit_variant("AzureLocation", 46u32, "NorwayWest"),
            Self::ChinaEast2 => serializer.serialize_unit_variant("AzureLocation", 47u32, "ChinaEast2"),
            Self::ChinaNorth2 => serializer.serialize_unit_variant("AzureLocation", 48u32, "ChinaNorth2"),
            Self::SwitzerlandNorth => serializer.serialize_unit_variant("AzureLocation", 49u32, "SwitzerlandNorth"),
            Self::SwitzerlandWest => serializer.serialize_unit_variant("AzureLocation", 50u32, "SwitzerlandWest"),
            Self::UaeNorth => serializer.serialize_unit_variant("AzureLocation", 51u32, "UAENorth"),
            Self::UaeCentral => serializer.serialize_unit_variant("AzureLocation", 52u32, "UAECentral"),
            Self::UsNatEast => serializer.serialize_unit_variant("AzureLocation", 53u32, "UsNatEast"),
            Self::UsNatWest => serializer.serialize_unit_variant("AzureLocation", 54u32, "UsNatWest"),
            Self::UsSecEast => serializer.serialize_unit_variant("AzureLocation", 55u32, "UsSecEast"),
            Self::UsSecCentral => serializer.serialize_unit_variant("AzureLocation", 56u32, "UsSecCentral"),
            Self::UsSecWest => serializer.serialize_unit_variant("AzureLocation", 57u32, "UsSecWest"),
            Self::SwedenCentral => serializer.serialize_unit_variant("AzureLocation", 58u32, "SwedenCentral"),
            Self::QatarCentral => serializer.serialize_unit_variant("AzureLocation", 59u32, "QatarCentral"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Class representing an Azure Managed Disk SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureManagedDiskSkuDto {
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<AzureManagedDiskSkuDtoDiskType>,
    #[serde(rename = "diskSize", default, skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<AzureDiskSize>,
    #[serde(rename = "diskRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub disk_redundancy: Option<AzureManagedDiskSkuDtoDiskRedundancy>,
    #[doc = "Gets the managed disk storage cost."]
    #[serde(rename = "storageCost", default, skip_serializing_if = "Option::is_none")]
    pub storage_cost: Option<f32>,
    #[doc = "Gets the recommended in GB of the managed disk."]
    #[serde(rename = "recommendedSizeInGib", default, skip_serializing_if = "Option::is_none")]
    pub recommended_size_in_gib: Option<f32>,
    #[doc = "Gets the recommended throughput in MBPS of the managed disk."]
    #[serde(rename = "recommendedThroughputInMbps", default, skip_serializing_if = "Option::is_none")]
    pub recommended_throughput_in_mbps: Option<f32>,
    #[doc = "Gets the recommended IOPS of the managed disk."]
    #[serde(rename = "recommendedIops", default, skip_serializing_if = "Option::is_none")]
    pub recommended_iops: Option<f32>,
}
impl AzureManagedDiskSkuDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureManagedDiskSkuDtoDiskRedundancy")]
pub enum AzureManagedDiskSkuDtoDiskRedundancy {
    Unknown,
    #[serde(rename = "LRS")]
    Lrs,
    #[serde(rename = "ZRS")]
    Zrs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureManagedDiskSkuDtoDiskRedundancy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureManagedDiskSkuDtoDiskRedundancy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureManagedDiskSkuDtoDiskRedundancy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureManagedDiskSkuDtoDiskRedundancy", 0u32, "Unknown"),
            Self::Lrs => serializer.serialize_unit_variant("AzureManagedDiskSkuDtoDiskRedundancy", 1u32, "LRS"),
            Self::Zrs => serializer.serialize_unit_variant("AzureManagedDiskSkuDtoDiskRedundancy", 2u32, "ZRS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureManagedDiskSkuDtoDiskType")]
pub enum AzureManagedDiskSkuDtoDiskType {
    Unknown,
    Standard,
    #[serde(rename = "StandardSSD")]
    StandardSsd,
    Premium,
    StandardOrPremium,
    Ultra,
    PremiumV2,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureManagedDiskSkuDtoDiskType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureManagedDiskSkuDtoDiskType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureManagedDiskSkuDtoDiskType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureManagedDiskSkuDtoDiskType", 0u32, "Unknown"),
            Self::Standard => serializer.serialize_unit_variant("AzureManagedDiskSkuDtoDiskType", 1u32, "Standard"),
            Self::StandardSsd => serializer.serialize_unit_variant("AzureManagedDiskSkuDtoDiskType", 2u32, "StandardSSD"),
            Self::Premium => serializer.serialize_unit_variant("AzureManagedDiskSkuDtoDiskType", 3u32, "Premium"),
            Self::StandardOrPremium => serializer.serialize_unit_variant("AzureManagedDiskSkuDtoDiskType", 4u32, "StandardOrPremium"),
            Self::Ultra => serializer.serialize_unit_variant("AzureManagedDiskSkuDtoDiskType", 5u32, "Ultra"),
            Self::PremiumV2 => serializer.serialize_unit_variant("AzureManagedDiskSkuDtoDiskType", 6u32, "PremiumV2"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureNetworkAdapterSuitabilityDetail")]
pub enum AzureNetworkAdapterSuitabilityDetail {
    None,
    MegabytesOfDataTransmittedMissing,
    MegabytesOfDataTransmittedOutOfRange,
    MegabytesOfDataRecievedMissing,
    MegabytesOfDataRecievedOutOfRange,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureNetworkAdapterSuitabilityDetail {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureNetworkAdapterSuitabilityDetail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureNetworkAdapterSuitabilityDetail {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("AzureNetworkAdapterSuitabilityDetail", 0u32, "None"),
            Self::MegabytesOfDataTransmittedMissing => {
                serializer.serialize_unit_variant("AzureNetworkAdapterSuitabilityDetail", 1u32, "MegabytesOfDataTransmittedMissing")
            }
            Self::MegabytesOfDataTransmittedOutOfRange => {
                serializer.serialize_unit_variant("AzureNetworkAdapterSuitabilityDetail", 2u32, "MegabytesOfDataTransmittedOutOfRange")
            }
            Self::MegabytesOfDataRecievedMissing => {
                serializer.serialize_unit_variant("AzureNetworkAdapterSuitabilityDetail", 3u32, "MegabytesOfDataRecievedMissing")
            }
            Self::MegabytesOfDataRecievedOutOfRange => {
                serializer.serialize_unit_variant("AzureNetworkAdapterSuitabilityDetail", 4u32, "MegabytesOfDataRecievedOutOfRange")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureNetworkAdapterSuitabilityExplanation")]
pub enum AzureNetworkAdapterSuitabilityExplanation {
    Unknown,
    NotApplicable,
    InternalErrorOccurred,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureNetworkAdapterSuitabilityExplanation {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureNetworkAdapterSuitabilityExplanation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureNetworkAdapterSuitabilityExplanation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureNetworkAdapterSuitabilityExplanation", 0u32, "Unknown"),
            Self::NotApplicable => serializer.serialize_unit_variant("AzureNetworkAdapterSuitabilityExplanation", 1u32, "NotApplicable"),
            Self::InternalErrorOccurred => {
                serializer.serialize_unit_variant("AzureNetworkAdapterSuitabilityExplanation", 2u32, "InternalErrorOccurred")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureOfferCode")]
pub enum AzureOfferCode {
    Unknown,
    #[serde(rename = "MSAZR0003P")]
    Msazr0003p,
    #[serde(rename = "MSAZR0044P")]
    Msazr0044p,
    #[serde(rename = "MSAZR0059P")]
    Msazr0059p,
    #[serde(rename = "MSAZR0060P")]
    Msazr0060p,
    #[serde(rename = "MSAZR0062P")]
    Msazr0062p,
    #[serde(rename = "MSAZR0063P")]
    Msazr0063p,
    #[serde(rename = "MSAZR0064P")]
    Msazr0064p,
    #[serde(rename = "MSAZR0029P")]
    Msazr0029p,
    #[serde(rename = "MSAZR0022P")]
    Msazr0022p,
    #[serde(rename = "MSAZR0023P")]
    Msazr0023p,
    #[serde(rename = "MSAZR0148P")]
    Msazr0148p,
    #[serde(rename = "MSAZR0025P")]
    Msazr0025p,
    #[serde(rename = "MSAZR0036P")]
    Msazr0036p,
    #[serde(rename = "MSAZR0120P")]
    Msazr0120p,
    #[serde(rename = "MSAZR0121P")]
    Msazr0121p,
    #[serde(rename = "MSAZR0122P")]
    Msazr0122p,
    #[serde(rename = "MSAZR0123P")]
    Msazr0123p,
    #[serde(rename = "MSAZR0124P")]
    Msazr0124p,
    #[serde(rename = "MSAZR0125P")]
    Msazr0125p,
    #[serde(rename = "MSAZR0126P")]
    Msazr0126p,
    #[serde(rename = "MSAZR0127P")]
    Msazr0127p,
    #[serde(rename = "MSAZR0128P")]
    Msazr0128p,
    #[serde(rename = "MSAZR0129P")]
    Msazr0129p,
    #[serde(rename = "MSAZR0130P")]
    Msazr0130p,
    #[serde(rename = "MSAZR0111P")]
    Msazr0111p,
    #[serde(rename = "MSAZR0144P")]
    Msazr0144p,
    #[serde(rename = "MSAZR0149P")]
    Msazr0149p,
    #[serde(rename = "MSMCAZR0044P")]
    Msmcazr0044p,
    #[serde(rename = "MSMCAZR0059P")]
    Msmcazr0059p,
    #[serde(rename = "MSMCAZR0060P")]
    Msmcazr0060p,
    #[serde(rename = "MSMCAZR0063P")]
    Msmcazr0063p,
    #[serde(rename = "MSMCAZR0120P")]
    Msmcazr0120p,
    #[serde(rename = "MSMCAZR0121P")]
    Msmcazr0121p,
    #[serde(rename = "MSMCAZR0125P")]
    Msmcazr0125p,
    #[serde(rename = "MSMCAZR0128P")]
    Msmcazr0128p,
    #[serde(rename = "MSAZRDE0003P")]
    Msazrde0003p,
    #[serde(rename = "MSAZRDE0044P")]
    Msazrde0044p,
    #[serde(rename = "MSAZRUSGOV0003P")]
    Msazrusgov0003p,
    #[serde(rename = "EA")]
    Ea,
    #[serde(rename = "MSAZR0243P")]
    Msazr0243p,
    SavingsPlan1Year,
    SavingsPlan3Year,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureOfferCode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureOfferCode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureOfferCode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureOfferCode", 0u32, "Unknown"),
            Self::Msazr0003p => serializer.serialize_unit_variant("AzureOfferCode", 1u32, "MSAZR0003P"),
            Self::Msazr0044p => serializer.serialize_unit_variant("AzureOfferCode", 2u32, "MSAZR0044P"),
            Self::Msazr0059p => serializer.serialize_unit_variant("AzureOfferCode", 3u32, "MSAZR0059P"),
            Self::Msazr0060p => serializer.serialize_unit_variant("AzureOfferCode", 4u32, "MSAZR0060P"),
            Self::Msazr0062p => serializer.serialize_unit_variant("AzureOfferCode", 5u32, "MSAZR0062P"),
            Self::Msazr0063p => serializer.serialize_unit_variant("AzureOfferCode", 6u32, "MSAZR0063P"),
            Self::Msazr0064p => serializer.serialize_unit_variant("AzureOfferCode", 7u32, "MSAZR0064P"),
            Self::Msazr0029p => serializer.serialize_unit_variant("AzureOfferCode", 8u32, "MSAZR0029P"),
            Self::Msazr0022p => serializer.serialize_unit_variant("AzureOfferCode", 9u32, "MSAZR0022P"),
            Self::Msazr0023p => serializer.serialize_unit_variant("AzureOfferCode", 10u32, "MSAZR0023P"),
            Self::Msazr0148p => serializer.serialize_unit_variant("AzureOfferCode", 11u32, "MSAZR0148P"),
            Self::Msazr0025p => serializer.serialize_unit_variant("AzureOfferCode", 12u32, "MSAZR0025P"),
            Self::Msazr0036p => serializer.serialize_unit_variant("AzureOfferCode", 13u32, "MSAZR0036P"),
            Self::Msazr0120p => serializer.serialize_unit_variant("AzureOfferCode", 14u32, "MSAZR0120P"),
            Self::Msazr0121p => serializer.serialize_unit_variant("AzureOfferCode", 15u32, "MSAZR0121P"),
            Self::Msazr0122p => serializer.serialize_unit_variant("AzureOfferCode", 16u32, "MSAZR0122P"),
            Self::Msazr0123p => serializer.serialize_unit_variant("AzureOfferCode", 17u32, "MSAZR0123P"),
            Self::Msazr0124p => serializer.serialize_unit_variant("AzureOfferCode", 18u32, "MSAZR0124P"),
            Self::Msazr0125p => serializer.serialize_unit_variant("AzureOfferCode", 19u32, "MSAZR0125P"),
            Self::Msazr0126p => serializer.serialize_unit_variant("AzureOfferCode", 20u32, "MSAZR0126P"),
            Self::Msazr0127p => serializer.serialize_unit_variant("AzureOfferCode", 21u32, "MSAZR0127P"),
            Self::Msazr0128p => serializer.serialize_unit_variant("AzureOfferCode", 22u32, "MSAZR0128P"),
            Self::Msazr0129p => serializer.serialize_unit_variant("AzureOfferCode", 23u32, "MSAZR0129P"),
            Self::Msazr0130p => serializer.serialize_unit_variant("AzureOfferCode", 24u32, "MSAZR0130P"),
            Self::Msazr0111p => serializer.serialize_unit_variant("AzureOfferCode", 25u32, "MSAZR0111P"),
            Self::Msazr0144p => serializer.serialize_unit_variant("AzureOfferCode", 26u32, "MSAZR0144P"),
            Self::Msazr0149p => serializer.serialize_unit_variant("AzureOfferCode", 27u32, "MSAZR0149P"),
            Self::Msmcazr0044p => serializer.serialize_unit_variant("AzureOfferCode", 28u32, "MSMCAZR0044P"),
            Self::Msmcazr0059p => serializer.serialize_unit_variant("AzureOfferCode", 29u32, "MSMCAZR0059P"),
            Self::Msmcazr0060p => serializer.serialize_unit_variant("AzureOfferCode", 30u32, "MSMCAZR0060P"),
            Self::Msmcazr0063p => serializer.serialize_unit_variant("AzureOfferCode", 31u32, "MSMCAZR0063P"),
            Self::Msmcazr0120p => serializer.serialize_unit_variant("AzureOfferCode", 32u32, "MSMCAZR0120P"),
            Self::Msmcazr0121p => serializer.serialize_unit_variant("AzureOfferCode", 33u32, "MSMCAZR0121P"),
            Self::Msmcazr0125p => serializer.serialize_unit_variant("AzureOfferCode", 34u32, "MSMCAZR0125P"),
            Self::Msmcazr0128p => serializer.serialize_unit_variant("AzureOfferCode", 35u32, "MSMCAZR0128P"),
            Self::Msazrde0003p => serializer.serialize_unit_variant("AzureOfferCode", 36u32, "MSAZRDE0003P"),
            Self::Msazrde0044p => serializer.serialize_unit_variant("AzureOfferCode", 37u32, "MSAZRDE0044P"),
            Self::Msazrusgov0003p => serializer.serialize_unit_variant("AzureOfferCode", 38u32, "MSAZRUSGOV0003P"),
            Self::Ea => serializer.serialize_unit_variant("AzureOfferCode", 39u32, "EA"),
            Self::Msazr0243p => serializer.serialize_unit_variant("AzureOfferCode", 40u32, "MSAZR0243P"),
            Self::SavingsPlan1Year => serializer.serialize_unit_variant("AzureOfferCode", 41u32, "SavingsPlan1Year"),
            Self::SavingsPlan3Year => serializer.serialize_unit_variant("AzureOfferCode", 42u32, "SavingsPlan3Year"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzurePricingTier")]
pub enum AzurePricingTier {
    Standard,
    Basic,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzurePricingTier {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzurePricingTier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzurePricingTier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Standard => serializer.serialize_unit_variant("AzurePricingTier", 0u32, "Standard"),
            Self::Basic => serializer.serialize_unit_variant("AzurePricingTier", 1u32, "Basic"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Quorum Witness."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureQuorumWitnessDto {
    #[serde(rename = "quorumWitnessType", default, skip_serializing_if = "Option::is_none")]
    pub quorum_witness_type: Option<AzureQuorumWitnessDtoQuorumWitnessType>,
}
impl AzureQuorumWitnessDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureQuorumWitnessDtoQuorumWitnessType")]
pub enum AzureQuorumWitnessDtoQuorumWitnessType {
    Unknown,
    Cloud,
    Disk,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureQuorumWitnessDtoQuorumWitnessType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureQuorumWitnessDtoQuorumWitnessType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureQuorumWitnessDtoQuorumWitnessType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureQuorumWitnessDtoQuorumWitnessType", 0u32, "Unknown"),
            Self::Cloud => serializer.serialize_unit_variant("AzureQuorumWitnessDtoQuorumWitnessType", 1u32, "Cloud"),
            Self::Disk => serializer.serialize_unit_variant("AzureQuorumWitnessDtoQuorumWitnessType", 2u32, "Disk"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureReservedInstance")]
pub enum AzureReservedInstance {
    None,
    #[serde(rename = "RI1Year")]
    Ri1Year,
    #[serde(rename = "RI3Year")]
    Ri3Year,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureReservedInstance {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureReservedInstance {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureReservedInstance {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("AzureReservedInstance", 0u32, "None"),
            Self::Ri1Year => serializer.serialize_unit_variant("AzureReservedInstance", 1u32, "RI1Year"),
            Self::Ri3Year => serializer.serialize_unit_variant("AzureReservedInstance", 2u32, "RI3Year"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Common properties for all azure tracked and proxy resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourceProperties {
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl AzureResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureSecurityOfferingType")]
pub enum AzureSecurityOfferingType {
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "MDC")]
    Mdc,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureSecurityOfferingType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureSecurityOfferingType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureSecurityOfferingType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::No => serializer.serialize_unit_variant("AzureSecurityOfferingType", 0u32, "NO"),
            Self::Mdc => serializer.serialize_unit_variant("AzureSecurityOfferingType", 1u32, "MDC"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureSqlDataBaseType")]
pub enum AzureSqlDataBaseType {
    Unknown,
    Automatic,
    SingleDatabase,
    ElasticPool,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureSqlDataBaseType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureSqlDataBaseType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureSqlDataBaseType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureSqlDataBaseType", 0u32, "Unknown"),
            Self::Automatic => serializer.serialize_unit_variant("AzureSqlDataBaseType", 1u32, "Automatic"),
            Self::SingleDatabase => serializer.serialize_unit_variant("AzureSqlDataBaseType", 2u32, "SingleDatabase"),
            Self::ElasticPool => serializer.serialize_unit_variant("AzureSqlDataBaseType", 3u32, "ElasticPool"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Class representing Azure SQL IAAS SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlIaasSkuDto {
    #[doc = "Azure Virtual Machine SKU."]
    #[serde(rename = "virtualMachineSize", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_size: Option<AzureVirtualMachineSkuDto>,
    #[doc = "Gets the The list of data disk sizes."]
    #[serde(
        rename = "dataDiskSizes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_disk_sizes: Vec<AzureManagedDiskSkuDto>,
    #[doc = "Gets the The list of log disk sizes."]
    #[serde(
        rename = "logDiskSizes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub log_disk_sizes: Vec<AzureManagedDiskSkuDto>,
    #[serde(rename = "azureSqlTargetType", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_target_type: Option<TargetType>,
}
impl AzureSqlIaasSkuDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureSqlInstanceType")]
pub enum AzureSqlInstanceType {
    Unknown,
    Automatic,
    SingleInstance,
    InstancePools,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureSqlInstanceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureSqlInstanceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureSqlInstanceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureSqlInstanceType", 0u32, "Unknown"),
            Self::Automatic => serializer.serialize_unit_variant("AzureSqlInstanceType", 1u32, "Automatic"),
            Self::SingleInstance => serializer.serialize_unit_variant("AzureSqlInstanceType", 2u32, "SingleInstance"),
            Self::InstancePools => serializer.serialize_unit_variant("AzureSqlInstanceType", 3u32, "InstancePools"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Class representing Azure SQL PAAS SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlPaasSkuDto {
    #[serde(rename = "azureSqlServiceTier", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_service_tier: Option<AzureSqlServiceTier>,
    #[serde(rename = "azureSqlComputeTier", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_compute_tier: Option<ComputeTier>,
    #[serde(rename = "azureSqlHardwareGeneration", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_hardware_generation: Option<HardwareGeneration>,
    #[doc = "Gets the storage maximum size in megabytes."]
    #[serde(rename = "storageMaxSizeInMB", default, skip_serializing_if = "Option::is_none")]
    pub storage_max_size_in_mb: Option<f32>,
    #[doc = "Gets the predicted data size in megabytes in the Azure SQL, will impact the\nbilling cost."]
    #[serde(rename = "predictedDataSizeInMB", default, skip_serializing_if = "Option::is_none")]
    pub predicted_data_size_in_mb: Option<f32>,
    #[doc = "Gets the predicted log size in megabytes in the Azure SQL, will impact the\nbilling cost."]
    #[serde(rename = "predictedLogSizeInMB", default, skip_serializing_if = "Option::is_none")]
    pub predicted_log_size_in_mb: Option<f32>,
    #[doc = "Gets the number of cores."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cores: Option<i32>,
    #[serde(rename = "azureSqlTargetType", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_target_type: Option<TargetType>,
}
impl AzureSqlPaasSkuDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureSqlPurchaseModel")]
pub enum AzureSqlPurchaseModel {
    Unknown,
    VCore,
    #[serde(rename = "DTU")]
    Dtu,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureSqlPurchaseModel {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureSqlPurchaseModel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureSqlPurchaseModel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureSqlPurchaseModel", 0u32, "Unknown"),
            Self::VCore => serializer.serialize_unit_variant("AzureSqlPurchaseModel", 1u32, "VCore"),
            Self::Dtu => serializer.serialize_unit_variant("AzureSqlPurchaseModel", 2u32, "DTU"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureSqlServiceTier")]
pub enum AzureSqlServiceTier {
    Unknown,
    Automatic,
    GeneralPurpose,
    BusinessCritical,
    HyperScale,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureSqlServiceTier {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureSqlServiceTier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureSqlServiceTier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureSqlServiceTier", 0u32, "Unknown"),
            Self::Automatic => serializer.serialize_unit_variant("AzureSqlServiceTier", 1u32, "Automatic"),
            Self::GeneralPurpose => serializer.serialize_unit_variant("AzureSqlServiceTier", 2u32, "GeneralPurpose"),
            Self::BusinessCritical => serializer.serialize_unit_variant("AzureSqlServiceTier", 3u32, "BusinessCritical"),
            Self::HyperScale => serializer.serialize_unit_variant("AzureSqlServiceTier", 4u32, "HyperScale"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureStorageRedundancy")]
pub enum AzureStorageRedundancy {
    Unknown,
    LocallyRedundant,
    ZoneRedundant,
    GeoRedundant,
    ReadAccessGeoRedundant,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureStorageRedundancy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureStorageRedundancy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureStorageRedundancy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureStorageRedundancy", 0u32, "Unknown"),
            Self::LocallyRedundant => serializer.serialize_unit_variant("AzureStorageRedundancy", 1u32, "LocallyRedundant"),
            Self::ZoneRedundant => serializer.serialize_unit_variant("AzureStorageRedundancy", 2u32, "ZoneRedundant"),
            Self::GeoRedundant => serializer.serialize_unit_variant("AzureStorageRedundancy", 3u32, "GeoRedundant"),
            Self::ReadAccessGeoRedundant => serializer.serialize_unit_variant("AzureStorageRedundancy", 4u32, "ReadAccessGeoRedundant"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Azure Virtual Machine SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureVirtualMachineSkuDto {
    #[serde(rename = "azureVmFamily", default, skip_serializing_if = "Option::is_none")]
    pub azure_vm_family: Option<AzureVmFamily>,
    #[doc = "Gets the Compute Size in vCores."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cores: Option<i32>,
    #[serde(rename = "azureSkuName", default, skip_serializing_if = "Option::is_none")]
    pub azure_sku_name: Option<AzureVmSize>,
    #[doc = "Gets the Available vCores. This can be less than the vCores in the Constrained\nvCPU VM Sizes."]
    #[serde(rename = "availableCores", default, skip_serializing_if = "Option::is_none")]
    pub available_cores: Option<i32>,
    #[doc = "Gets the Max network interfaces."]
    #[serde(rename = "maxNetworkInterfaces", default, skip_serializing_if = "Option::is_none")]
    pub max_network_interfaces: Option<i32>,
}
impl AzureVirtualMachineSkuDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureVmFamily")]
pub enum AzureVmFamily {
    Unknown,
    #[serde(rename = "Basic_A0_A4")]
    BasicA0A4,
    #[serde(rename = "Standard_A0_A7")]
    StandardA0A7,
    #[serde(rename = "Standard_A8_A11")]
    StandardA8A11,
    #[serde(rename = "Av2_series")]
    Av2Series,
    #[serde(rename = "D_series")]
    DSeries,
    #[serde(rename = "Dv2_series")]
    Dv2Series,
    #[serde(rename = "DS_series")]
    DsSeries,
    #[serde(rename = "DSv2_series")]
    DSv2Series,
    #[serde(rename = "F_series")]
    FSeries,
    #[serde(rename = "Fs_series")]
    FsSeries,
    #[serde(rename = "G_series")]
    GSeries,
    #[serde(rename = "GS_series")]
    GsSeries,
    #[serde(rename = "H_series")]
    HSeries,
    #[serde(rename = "Ls_series")]
    LsSeries,
    #[serde(rename = "Dsv3_series")]
    Dsv3Series,
    #[serde(rename = "Dv3_series")]
    Dv3Series,
    #[serde(rename = "Fsv2_series")]
    Fsv2Series,
    #[serde(rename = "Ev3_series")]
    Ev3Series,
    #[serde(rename = "Esv3_series")]
    Esv3Series,
    #[serde(rename = "M_series")]
    MSeries,
    #[serde(rename = "DC_Series")]
    DcSeries,
    #[serde(rename = "Lsv2_series")]
    Lsv2Series,
    #[serde(rename = "Ev4_series")]
    Ev4Series,
    #[serde(rename = "Esv4_series")]
    Esv4Series,
    #[serde(rename = "Edv4_series")]
    Edv4Series,
    #[serde(rename = "Edsv4_series")]
    Edsv4Series,
    #[serde(rename = "Dv4_series")]
    Dv4Series,
    #[serde(rename = "Dsv4_series")]
    Dsv4Series,
    #[serde(rename = "Ddv4_series")]
    Ddv4Series,
    #[serde(rename = "Ddsv4_series")]
    Ddsv4Series,
    #[serde(rename = "Easv4_series")]
    Easv4Series,
    #[serde(rename = "Dasv4_series")]
    Dasv4Series,
    #[serde(rename = "Mv2_series")]
    Mv2Series,
    #[serde(rename = "Eav4_series")]
    Eav4Series,
    #[serde(rename = "Dav4_series")]
    Dav4Series,
    #[serde(rename = "Msv2_series")]
    Msv2Series,
    #[serde(rename = "Mdsv2_series")]
    Mdsv2Series,
    #[serde(rename = "Dv5_series")]
    Dv5Series,
    #[serde(rename = "Dsv5_series")]
    Dsv5Series,
    #[serde(rename = "Ddv5_series")]
    Ddv5Series,
    #[serde(rename = "Ddsv5_series")]
    Ddsv5Series,
    #[serde(rename = "Dasv5_series")]
    Dasv5Series,
    #[serde(rename = "Dadsv5_series")]
    Dadsv5Series,
    #[serde(rename = "Ev5_series")]
    Ev5Series,
    #[serde(rename = "Esv5_series")]
    Esv5Series,
    #[serde(rename = "Edv5_series")]
    Edv5Series,
    #[serde(rename = "Edsv5_series")]
    Edsv5Series,
    #[serde(rename = "Easv5_series")]
    Easv5Series,
    #[serde(rename = "Eadsv5_series")]
    Eadsv5Series,
    #[serde(rename = "Ebsv5_series")]
    Ebsv5Series,
    #[serde(rename = "Ebdsv5_series")]
    Ebdsv5Series,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureVmFamily {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureVmFamily {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureVmFamily {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureVmFamily", 0u32, "Unknown"),
            Self::BasicA0A4 => serializer.serialize_unit_variant("AzureVmFamily", 1u32, "Basic_A0_A4"),
            Self::StandardA0A7 => serializer.serialize_unit_variant("AzureVmFamily", 2u32, "Standard_A0_A7"),
            Self::StandardA8A11 => serializer.serialize_unit_variant("AzureVmFamily", 3u32, "Standard_A8_A11"),
            Self::Av2Series => serializer.serialize_unit_variant("AzureVmFamily", 4u32, "Av2_series"),
            Self::DSeries => serializer.serialize_unit_variant("AzureVmFamily", 5u32, "D_series"),
            Self::Dv2Series => serializer.serialize_unit_variant("AzureVmFamily", 6u32, "Dv2_series"),
            Self::DsSeries => serializer.serialize_unit_variant("AzureVmFamily", 7u32, "DS_series"),
            Self::DSv2Series => serializer.serialize_unit_variant("AzureVmFamily", 8u32, "DSv2_series"),
            Self::FSeries => serializer.serialize_unit_variant("AzureVmFamily", 9u32, "F_series"),
            Self::FsSeries => serializer.serialize_unit_variant("AzureVmFamily", 10u32, "Fs_series"),
            Self::GSeries => serializer.serialize_unit_variant("AzureVmFamily", 11u32, "G_series"),
            Self::GsSeries => serializer.serialize_unit_variant("AzureVmFamily", 12u32, "GS_series"),
            Self::HSeries => serializer.serialize_unit_variant("AzureVmFamily", 13u32, "H_series"),
            Self::LsSeries => serializer.serialize_unit_variant("AzureVmFamily", 14u32, "Ls_series"),
            Self::Dsv3Series => serializer.serialize_unit_variant("AzureVmFamily", 15u32, "Dsv3_series"),
            Self::Dv3Series => serializer.serialize_unit_variant("AzureVmFamily", 16u32, "Dv3_series"),
            Self::Fsv2Series => serializer.serialize_unit_variant("AzureVmFamily", 17u32, "Fsv2_series"),
            Self::Ev3Series => serializer.serialize_unit_variant("AzureVmFamily", 18u32, "Ev3_series"),
            Self::Esv3Series => serializer.serialize_unit_variant("AzureVmFamily", 19u32, "Esv3_series"),
            Self::MSeries => serializer.serialize_unit_variant("AzureVmFamily", 20u32, "M_series"),
            Self::DcSeries => serializer.serialize_unit_variant("AzureVmFamily", 21u32, "DC_Series"),
            Self::Lsv2Series => serializer.serialize_unit_variant("AzureVmFamily", 22u32, "Lsv2_series"),
            Self::Ev4Series => serializer.serialize_unit_variant("AzureVmFamily", 23u32, "Ev4_series"),
            Self::Esv4Series => serializer.serialize_unit_variant("AzureVmFamily", 24u32, "Esv4_series"),
            Self::Edv4Series => serializer.serialize_unit_variant("AzureVmFamily", 25u32, "Edv4_series"),
            Self::Edsv4Series => serializer.serialize_unit_variant("AzureVmFamily", 26u32, "Edsv4_series"),
            Self::Dv4Series => serializer.serialize_unit_variant("AzureVmFamily", 27u32, "Dv4_series"),
            Self::Dsv4Series => serializer.serialize_unit_variant("AzureVmFamily", 28u32, "Dsv4_series"),
            Self::Ddv4Series => serializer.serialize_unit_variant("AzureVmFamily", 29u32, "Ddv4_series"),
            Self::Ddsv4Series => serializer.serialize_unit_variant("AzureVmFamily", 30u32, "Ddsv4_series"),
            Self::Easv4Series => serializer.serialize_unit_variant("AzureVmFamily", 31u32, "Easv4_series"),
            Self::Dasv4Series => serializer.serialize_unit_variant("AzureVmFamily", 32u32, "Dasv4_series"),
            Self::Mv2Series => serializer.serialize_unit_variant("AzureVmFamily", 33u32, "Mv2_series"),
            Self::Eav4Series => serializer.serialize_unit_variant("AzureVmFamily", 34u32, "Eav4_series"),
            Self::Dav4Series => serializer.serialize_unit_variant("AzureVmFamily", 35u32, "Dav4_series"),
            Self::Msv2Series => serializer.serialize_unit_variant("AzureVmFamily", 36u32, "Msv2_series"),
            Self::Mdsv2Series => serializer.serialize_unit_variant("AzureVmFamily", 37u32, "Mdsv2_series"),
            Self::Dv5Series => serializer.serialize_unit_variant("AzureVmFamily", 38u32, "Dv5_series"),
            Self::Dsv5Series => serializer.serialize_unit_variant("AzureVmFamily", 39u32, "Dsv5_series"),
            Self::Ddv5Series => serializer.serialize_unit_variant("AzureVmFamily", 40u32, "Ddv5_series"),
            Self::Ddsv5Series => serializer.serialize_unit_variant("AzureVmFamily", 41u32, "Ddsv5_series"),
            Self::Dasv5Series => serializer.serialize_unit_variant("AzureVmFamily", 42u32, "Dasv5_series"),
            Self::Dadsv5Series => serializer.serialize_unit_variant("AzureVmFamily", 43u32, "Dadsv5_series"),
            Self::Ev5Series => serializer.serialize_unit_variant("AzureVmFamily", 44u32, "Ev5_series"),
            Self::Esv5Series => serializer.serialize_unit_variant("AzureVmFamily", 45u32, "Esv5_series"),
            Self::Edv5Series => serializer.serialize_unit_variant("AzureVmFamily", 46u32, "Edv5_series"),
            Self::Edsv5Series => serializer.serialize_unit_variant("AzureVmFamily", 47u32, "Edsv5_series"),
            Self::Easv5Series => serializer.serialize_unit_variant("AzureVmFamily", 48u32, "Easv5_series"),
            Self::Eadsv5Series => serializer.serialize_unit_variant("AzureVmFamily", 49u32, "Eadsv5_series"),
            Self::Ebsv5Series => serializer.serialize_unit_variant("AzureVmFamily", 50u32, "Ebsv5_series"),
            Self::Ebdsv5Series => serializer.serialize_unit_variant("AzureVmFamily", 51u32, "Ebdsv5_series"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureVmSize")]
pub enum AzureVmSize {
    Unknown,
    #[serde(rename = "Basic_A0")]
    BasicA0,
    #[serde(rename = "Basic_A1")]
    BasicA1,
    #[serde(rename = "Basic_A2")]
    BasicA2,
    #[serde(rename = "Basic_A3")]
    BasicA3,
    #[serde(rename = "Basic_A4")]
    BasicA4,
    #[serde(rename = "Standard_A0")]
    StandardA0,
    #[serde(rename = "Standard_A1")]
    StandardA1,
    #[serde(rename = "Standard_A2")]
    StandardA2,
    #[serde(rename = "Standard_A3")]
    StandardA3,
    #[serde(rename = "Standard_A4")]
    StandardA4,
    #[serde(rename = "Standard_A5")]
    StandardA5,
    #[serde(rename = "Standard_A6")]
    StandardA6,
    #[serde(rename = "Standard_A7")]
    StandardA7,
    #[serde(rename = "Standard_A8")]
    StandardA8,
    #[serde(rename = "Standard_A9")]
    StandardA9,
    #[serde(rename = "Standard_A10")]
    StandardA10,
    #[serde(rename = "Standard_A11")]
    StandardA11,
    #[serde(rename = "Standard_A1_v2")]
    StandardA1V2,
    #[serde(rename = "Standard_A2_v2")]
    StandardA2V2,
    #[serde(rename = "Standard_A4_v2")]
    StandardA4V2,
    #[serde(rename = "Standard_A8_v2")]
    StandardA8V2,
    #[serde(rename = "Standard_A2m_v2")]
    StandardA2mV2,
    #[serde(rename = "Standard_A4m_v2")]
    StandardA4mV2,
    #[serde(rename = "Standard_A8m_v2")]
    StandardA8mV2,
    #[serde(rename = "Standard_D1")]
    StandardD1,
    #[serde(rename = "Standard_D2")]
    StandardD2,
    #[serde(rename = "Standard_D3")]
    StandardD3,
    #[serde(rename = "Standard_D4")]
    StandardD4,
    #[serde(rename = "Standard_D11")]
    StandardD11,
    #[serde(rename = "Standard_D12")]
    StandardD12,
    #[serde(rename = "Standard_D13")]
    StandardD13,
    #[serde(rename = "Standard_D14")]
    StandardD14,
    #[serde(rename = "Standard_D1_v2")]
    StandardD1V2,
    #[serde(rename = "Standard_D2_v2")]
    StandardD2V2,
    #[serde(rename = "Standard_D3_v2")]
    StandardD3V2,
    #[serde(rename = "Standard_D4_v2")]
    StandardD4V2,
    #[serde(rename = "Standard_D5_v2")]
    StandardD5V2,
    #[serde(rename = "Standard_D11_v2")]
    StandardD11V2,
    #[serde(rename = "Standard_D12_v2")]
    StandardD12V2,
    #[serde(rename = "Standard_D13_v2")]
    StandardD13V2,
    #[serde(rename = "Standard_D14_v2")]
    StandardD14V2,
    #[serde(rename = "Standard_D15_v2")]
    StandardD15V2,
    #[serde(rename = "Standard_DS1")]
    StandardDs1,
    #[serde(rename = "Standard_DS2")]
    StandardDs2,
    #[serde(rename = "Standard_DS3")]
    StandardDs3,
    #[serde(rename = "Standard_DS4")]
    StandardDs4,
    #[serde(rename = "Standard_DS11")]
    StandardDs11,
    #[serde(rename = "Standard_DS12")]
    StandardDs12,
    #[serde(rename = "Standard_DS13")]
    StandardDs13,
    #[serde(rename = "Standard_DS14")]
    StandardDs14,
    #[serde(rename = "Standard_DS1_v2")]
    StandardDs1V2,
    #[serde(rename = "Standard_DS2_v2")]
    StandardDs2V2,
    #[serde(rename = "Standard_DS3_v2")]
    StandardDs3V2,
    #[serde(rename = "Standard_DS4_v2")]
    StandardDs4V2,
    #[serde(rename = "Standard_DS5_v2")]
    StandardDs5V2,
    #[serde(rename = "Standard_DS11_v2")]
    StandardDs11V2,
    #[serde(rename = "Standard_DS12_v2")]
    StandardDs12V2,
    #[serde(rename = "Standard_DS13_v2")]
    StandardDs13V2,
    #[serde(rename = "Standard_DS14_v2")]
    StandardDs14V2,
    #[serde(rename = "Standard_DS15_v2")]
    StandardDs15V2,
    #[serde(rename = "Standard_F1")]
    StandardF1,
    #[serde(rename = "Standard_F2")]
    StandardF2,
    #[serde(rename = "Standard_F4")]
    StandardF4,
    #[serde(rename = "Standard_F8")]
    StandardF8,
    #[serde(rename = "Standard_F16")]
    StandardF16,
    #[serde(rename = "Standard_F1s")]
    StandardF1s,
    #[serde(rename = "Standard_F2s")]
    StandardF2s,
    #[serde(rename = "Standard_F4s")]
    StandardF4s,
    #[serde(rename = "Standard_F8s")]
    StandardF8s,
    #[serde(rename = "Standard_F16s")]
    StandardF16s,
    #[serde(rename = "Standard_G1")]
    StandardG1,
    #[serde(rename = "Standard_G2")]
    StandardG2,
    #[serde(rename = "Standard_G3")]
    StandardG3,
    #[serde(rename = "Standard_G4")]
    StandardG4,
    #[serde(rename = "Standard_G5")]
    StandardG5,
    #[serde(rename = "Standard_GS1")]
    StandardGs1,
    #[serde(rename = "Standard_GS2")]
    StandardGs2,
    #[serde(rename = "Standard_GS3")]
    StandardGs3,
    #[serde(rename = "Standard_GS4")]
    StandardGs4,
    #[serde(rename = "Standard_GS5")]
    StandardGs5,
    #[serde(rename = "Standard_H8")]
    StandardH8,
    #[serde(rename = "Standard_H16")]
    StandardH16,
    #[serde(rename = "Standard_H8m")]
    StandardH8m,
    #[serde(rename = "Standard_H16m")]
    StandardH16m,
    #[serde(rename = "Standard_H16r")]
    StandardH16r,
    #[serde(rename = "Standard_H16mr")]
    StandardH16mr,
    #[serde(rename = "Standard_L4s")]
    StandardL4s,
    #[serde(rename = "Standard_L8s")]
    StandardL8s,
    #[serde(rename = "Standard_L16s")]
    StandardL16s,
    #[serde(rename = "Standard_L32s")]
    StandardL32s,
    #[serde(rename = "Standard_D2s_v3")]
    StandardD2sV3,
    #[serde(rename = "Standard_D4s_v3")]
    StandardD4sV3,
    #[serde(rename = "Standard_D8s_v3")]
    StandardD8sV3,
    #[serde(rename = "Standard_D16s_v3")]
    StandardD16sV3,
    #[serde(rename = "Standard_D32s_v3")]
    StandardD32sV3,
    #[serde(rename = "Standard_D64s_v3")]
    StandardD64sV3,
    #[serde(rename = "Standard_D2_v3")]
    StandardD2V3,
    #[serde(rename = "Standard_D4_v3")]
    StandardD4V3,
    #[serde(rename = "Standard_D8_v3")]
    StandardD8V3,
    #[serde(rename = "Standard_D16_v3")]
    StandardD16V3,
    #[serde(rename = "Standard_D32_v3")]
    StandardD32V3,
    #[serde(rename = "Standard_D64_v3")]
    StandardD64V3,
    #[serde(rename = "Standard_F2s_v2")]
    StandardF2sV2,
    #[serde(rename = "Standard_F4s_v2")]
    StandardF4sV2,
    #[serde(rename = "Standard_F8s_v2")]
    StandardF8sV2,
    #[serde(rename = "Standard_F16s_v2")]
    StandardF16sV2,
    #[serde(rename = "Standard_F32s_v2")]
    StandardF32sV2,
    #[serde(rename = "Standard_F64s_v2")]
    StandardF64sV2,
    #[serde(rename = "Standard_F72s_v2")]
    StandardF72sV2,
    #[serde(rename = "Standard_E2_v3")]
    StandardE2V3,
    #[serde(rename = "Standard_E4_v3")]
    StandardE4V3,
    #[serde(rename = "Standard_E8_v3")]
    StandardE8V3,
    #[serde(rename = "Standard_E16_v3")]
    StandardE16V3,
    #[serde(rename = "Standard_E32_v3")]
    StandardE32V3,
    #[serde(rename = "Standard_E64_v3")]
    StandardE64V3,
    #[serde(rename = "Standard_E2s_v3")]
    StandardE2sV3,
    #[serde(rename = "Standard_E4s_v3")]
    StandardE4sV3,
    #[serde(rename = "Standard_E8s_v3")]
    StandardE8sV3,
    #[serde(rename = "Standard_E16s_v3")]
    StandardE16sV3,
    #[serde(rename = "Standard_E32s_v3")]
    StandardE32sV3,
    #[serde(rename = "Standard_E64s_v3")]
    StandardE64sV3,
    #[serde(rename = "Standard_M64s")]
    StandardM64s,
    #[serde(rename = "Standard_M64ms")]
    StandardM64ms,
    #[serde(rename = "Standard_M128s")]
    StandardM128s,
    #[serde(rename = "Standard_M128ms")]
    StandardM128ms,
    #[serde(rename = "Standard_DC2s")]
    StandardDc2s,
    #[serde(rename = "Standard_DC4s")]
    StandardDc4s,
    #[serde(rename = "Standard_E20_v3")]
    StandardE20V3,
    #[serde(rename = "Standard_E20s_v3")]
    StandardE20sV3,
    #[serde(rename = "Standard_E64i_v3")]
    StandardE64iV3,
    #[serde(rename = "Standard_E64is_v3")]
    StandardE64isV3,
    #[serde(rename = "Standard_M8ms")]
    StandardM8ms,
    #[serde(rename = "Standard_M16ms")]
    StandardM16ms,
    #[serde(rename = "Standard_M32ls")]
    StandardM32ls,
    #[serde(rename = "Standard_M32ms")]
    StandardM32ms,
    #[serde(rename = "Standard_M32ts")]
    StandardM32ts,
    #[serde(rename = "Standard_M64")]
    StandardM64,
    #[serde(rename = "Standard_M64ls")]
    StandardM64ls,
    #[serde(rename = "Standard_M64m")]
    StandardM64m,
    #[serde(rename = "Standard_M128")]
    StandardM128,
    #[serde(rename = "Standard_M128m")]
    StandardM128m,
    #[serde(rename = "Standard_L8s_v2")]
    StandardL8sV2,
    #[serde(rename = "Standard_L16s_v2")]
    StandardL16sV2,
    #[serde(rename = "Standard_L32s_v2")]
    StandardL32sV2,
    #[serde(rename = "Standard_L48s_v2")]
    StandardL48sV2,
    #[serde(rename = "Standard_L64s_v2")]
    StandardL64sV2,
    #[serde(rename = "Standard_L80s_v2")]
    StandardL80sV2,
    #[serde(rename = "Standard_D2_v4")]
    StandardD2V4,
    #[serde(rename = "Standard_D4_v4")]
    StandardD4V4,
    #[serde(rename = "Standard_D8_v4")]
    StandardD8V4,
    #[serde(rename = "Standard_D16_v4")]
    StandardD16V4,
    #[serde(rename = "Standard_D32_v4")]
    StandardD32V4,
    #[serde(rename = "Standard_D48_v4")]
    StandardD48V4,
    #[serde(rename = "Standard_D64_v4")]
    StandardD64V4,
    #[serde(rename = "Standard_D2s_v4")]
    StandardD2sV4,
    #[serde(rename = "Standard_D4s_v4")]
    StandardD4sV4,
    #[serde(rename = "Standard_D8s_v4")]
    StandardD8sV4,
    #[serde(rename = "Standard_D16s_v4")]
    StandardD16sV4,
    #[serde(rename = "Standard_D32s_v4")]
    StandardD32sV4,
    #[serde(rename = "Standard_D48s_v4")]
    StandardD48sV4,
    #[serde(rename = "Standard_D64s_v4")]
    StandardD64sV4,
    #[serde(rename = "Standard_D2d_v4")]
    StandardD2dV4,
    #[serde(rename = "Standard_D4d_v4")]
    StandardD4dV4,
    #[serde(rename = "Standard_D8d_v4")]
    StandardD8dV4,
    #[serde(rename = "Standard_D16d_v4")]
    StandardD16dV4,
    #[serde(rename = "Standard_D32d_v4")]
    StandardD32dV4,
    #[serde(rename = "Standard_D48d_v4")]
    StandardD48dV4,
    #[serde(rename = "Standard_D64d_v4")]
    StandardD64dV4,
    #[serde(rename = "Standard_D2ds_v4")]
    StandardD2dsV4,
    #[serde(rename = "Standard_D4ds_v4")]
    StandardD4dsV4,
    #[serde(rename = "Standard_D8ds_v4")]
    StandardD8dsV4,
    #[serde(rename = "Standard_D16ds_v4")]
    StandardD16dsV4,
    #[serde(rename = "Standard_D32ds_v4")]
    StandardD32dsV4,
    #[serde(rename = "Standard_D48ds_v4")]
    StandardD48dsV4,
    #[serde(rename = "Standard_D64ds_v4")]
    StandardD64dsV4,
    #[serde(rename = "Standard_E2_v4")]
    StandardE2V4,
    #[serde(rename = "Standard_E4_v4")]
    StandardE4V4,
    #[serde(rename = "Standard_E8_v4")]
    StandardE8V4,
    #[serde(rename = "Standard_E16_v4")]
    StandardE16V4,
    #[serde(rename = "Standard_E20_v4")]
    StandardE20V4,
    #[serde(rename = "Standard_E32_v4")]
    StandardE32V4,
    #[serde(rename = "Standard_E48_v4")]
    StandardE48V4,
    #[serde(rename = "Standard_E64_v4")]
    StandardE64V4,
    #[serde(rename = "Standard_E2s_v4")]
    StandardE2sV4,
    #[serde(rename = "Standard_E4s_v4")]
    StandardE4sV4,
    #[serde(rename = "Standard_E8s_v4")]
    StandardE8sV4,
    #[serde(rename = "Standard_E16s_v4")]
    StandardE16sV4,
    #[serde(rename = "Standard_E20s_v4")]
    StandardE20sV4,
    #[serde(rename = "Standard_E32s_v4")]
    StandardE32sV4,
    #[serde(rename = "Standard_E48s_v4")]
    StandardE48sV4,
    #[serde(rename = "Standard_E64s_v4")]
    StandardE64sV4,
    #[serde(rename = "Standard_E2d_v4")]
    StandardE2dV4,
    #[serde(rename = "Standard_E4d_v4")]
    StandardE4dV4,
    #[serde(rename = "Standard_E8d_v4")]
    StandardE8dV4,
    #[serde(rename = "Standard_E16d_v4")]
    StandardE16dV4,
    #[serde(rename = "Standard_E20d_v4")]
    StandardE20dV4,
    #[serde(rename = "Standard_E32d_v4")]
    StandardE32dV4,
    #[serde(rename = "Standard_E48d_v4")]
    StandardE48dV4,
    #[serde(rename = "Standard_E64d_v4")]
    StandardE64dV4,
    #[serde(rename = "Standard_E2ds_v4")]
    StandardE2dsV4,
    #[serde(rename = "Standard_E4ds_v4")]
    StandardE4dsV4,
    #[serde(rename = "Standard_E8ds_v4")]
    StandardE8dsV4,
    #[serde(rename = "Standard_E16ds_v4")]
    StandardE16dsV4,
    #[serde(rename = "Standard_E20ds_v4")]
    StandardE20dsV4,
    #[serde(rename = "Standard_E32ds_v4")]
    StandardE32dsV4,
    #[serde(rename = "Standard_E48ds_v4")]
    StandardE48dsV4,
    #[serde(rename = "Standard_E64ds_v4")]
    StandardE64dsV4,
    #[serde(rename = "Standard_E2as_v4")]
    StandardE2asV4,
    #[serde(rename = "Standard_E4as_v4")]
    StandardE4asV4,
    #[serde(rename = "Standard_E8as_v4")]
    StandardE8asV4,
    #[serde(rename = "Standard_E16as_v4")]
    StandardE16asV4,
    #[serde(rename = "Standard_E20as_v4")]
    StandardE20asV4,
    #[serde(rename = "Standard_E32as_v4")]
    StandardE32asV4,
    #[serde(rename = "Standard_E48as_v4")]
    StandardE48asV4,
    #[serde(rename = "Standard_E64as_v4")]
    StandardE64asV4,
    #[serde(rename = "Standard_E96as_v4")]
    StandardE96asV4,
    #[serde(rename = "Standard_D2as_v4")]
    StandardD2asV4,
    #[serde(rename = "Standard_D4as_v4")]
    StandardD4asV4,
    #[serde(rename = "Standard_D8as_v4")]
    StandardD8asV4,
    #[serde(rename = "Standard_D16as_v4")]
    StandardD16asV4,
    #[serde(rename = "Standard_D32as_v4")]
    StandardD32asV4,
    #[serde(rename = "Standard_D48as_v4")]
    StandardD48asV4,
    #[serde(rename = "Standard_D64as_v4")]
    StandardD64asV4,
    #[serde(rename = "Standard_D96as_v4")]
    StandardD96asV4,
    #[serde(rename = "Standard_M208ms_v2")]
    StandardM208msV2,
    #[serde(rename = "Standard_M208s_v2")]
    StandardM208sV2,
    #[serde(rename = "Standard_M416ms_v2")]
    StandardM416msV2,
    #[serde(rename = "Standard_M416s_v2")]
    StandardM416sV2,
    #[serde(rename = "Standard_F48s_v2")]
    StandardF48sV2,
    #[serde(rename = "Standard_E48_v3")]
    StandardE48V3,
    #[serde(rename = "Standard_E48s_v3")]
    StandardE48sV3,
    #[serde(rename = "Standard_E80is_v4")]
    StandardE80isV4,
    #[serde(rename = "Standard_E80ids_v4")]
    StandardE80idsV4,
    #[serde(rename = "Standard_E2a_v4")]
    StandardE2aV4,
    #[serde(rename = "Standard_E4a_v4")]
    StandardE4aV4,
    #[serde(rename = "Standard_E8a_v4")]
    StandardE8aV4,
    #[serde(rename = "Standard_E16a_v4")]
    StandardE16aV4,
    #[serde(rename = "Standard_E20a_v4")]
    StandardE20aV4,
    #[serde(rename = "Standard_E32a_v4")]
    StandardE32aV4,
    #[serde(rename = "Standard_E48a_v4")]
    StandardE48aV4,
    #[serde(rename = "Standard_E64a_v4")]
    StandardE64aV4,
    #[serde(rename = "Standard_E96a_v4")]
    StandardE96aV4,
    #[serde(rename = "Standard_D2a_v4")]
    StandardD2aV4,
    #[serde(rename = "Standard_D4a_v4")]
    StandardD4aV4,
    #[serde(rename = "Standard_D8a_v4")]
    StandardD8aV4,
    #[serde(rename = "Standard_D16a_v4")]
    StandardD16aV4,
    #[serde(rename = "Standard_D32a_v4")]
    StandardD32aV4,
    #[serde(rename = "Standard_D48a_v4")]
    StandardD48aV4,
    #[serde(rename = "Standard_D64a_v4")]
    StandardD64aV4,
    #[serde(rename = "Standard_D96a_v4")]
    StandardD96aV4,
    #[serde(rename = "Standard_M32ms_v2")]
    StandardM32msV2,
    #[serde(rename = "Standard_M64s_v2")]
    StandardM64sV2,
    #[serde(rename = "Standard_M64ms_v2")]
    StandardM64msV2,
    #[serde(rename = "Standard_M128s_v2")]
    StandardM128sV2,
    #[serde(rename = "Standard_M128ms_v2")]
    StandardM128msV2,
    #[serde(rename = "Standard_M192is_v2")]
    StandardM192isV2,
    #[serde(rename = "Standard_M192ims_v2")]
    StandardM192imsV2,
    #[serde(rename = "Standard_M32dms_v2")]
    StandardM32dmsV2,
    #[serde(rename = "Standard_M64ds_v2")]
    StandardM64dsV2,
    #[serde(rename = "Standard_M64dms_v2")]
    StandardM64dmsV2,
    #[serde(rename = "Standard_M128ds_v2")]
    StandardM128dsV2,
    #[serde(rename = "Standard_M128dms_v2")]
    StandardM128dmsV2,
    #[serde(rename = "Standard_M192ids_v2")]
    StandardM192idsV2,
    #[serde(rename = "Standard_M192idms_v2")]
    StandardM192idmsV2,
    #[serde(rename = "Standard_D2_v5")]
    StandardD2V5,
    #[serde(rename = "Standard_D4_v5")]
    StandardD4V5,
    #[serde(rename = "Standard_D8_v5")]
    StandardD8V5,
    #[serde(rename = "Standard_D16_v5")]
    StandardD16V5,
    #[serde(rename = "Standard_D32_v5")]
    StandardD32V5,
    #[serde(rename = "Standard_D48_v5")]
    StandardD48V5,
    #[serde(rename = "Standard_D64_v5")]
    StandardD64V5,
    #[serde(rename = "Standard_D96_v5")]
    StandardD96V5,
    #[serde(rename = "Standard_D2s_v5")]
    StandardD2sV5,
    #[serde(rename = "Standard_D4s_v5")]
    StandardD4sV5,
    #[serde(rename = "Standard_D8s_v5")]
    StandardD8sV5,
    #[serde(rename = "Standard_D16s_v5")]
    StandardD16sV5,
    #[serde(rename = "Standard_D32s_v5")]
    StandardD32sV5,
    #[serde(rename = "Standard_D48s_v5")]
    StandardD48sV5,
    #[serde(rename = "Standard_D64s_v5")]
    StandardD64sV5,
    #[serde(rename = "Standard_D96s_v5")]
    StandardD96sV5,
    #[serde(rename = "Standard_D2d_v5")]
    StandardD2dV5,
    #[serde(rename = "Standard_D4d_v5")]
    StandardD4dV5,
    #[serde(rename = "Standard_D8d_v5")]
    StandardD8dV5,
    #[serde(rename = "Standard_D16d_v5")]
    StandardD16dV5,
    #[serde(rename = "Standard_D32d_v5")]
    StandardD32dV5,
    #[serde(rename = "Standard_D48d_v5")]
    StandardD48dV5,
    #[serde(rename = "Standard_D64d_v5")]
    StandardD64dV5,
    #[serde(rename = "Standard_D96d_v5")]
    StandardD96dV5,
    #[serde(rename = "Standard_D2ds_v5")]
    StandardD2dsV5,
    #[serde(rename = "Standard_D4ds_v5")]
    StandardD4dsV5,
    #[serde(rename = "Standard_D8ds_v5")]
    StandardD8dsV5,
    #[serde(rename = "Standard_D16ds_v5")]
    StandardD16dsV5,
    #[serde(rename = "Standard_D32ds_v5")]
    StandardD32dsV5,
    #[serde(rename = "Standard_D48ds_v5")]
    StandardD48dsV5,
    #[serde(rename = "Standard_D64ds_v5")]
    StandardD64dsV5,
    #[serde(rename = "Standard_D96ds_v5")]
    StandardD96dsV5,
    #[serde(rename = "Standard_D2as_v5")]
    StandardD2asV5,
    #[serde(rename = "Standard_D4as_v5")]
    StandardD4asV5,
    #[serde(rename = "Standard_D8as_v5")]
    StandardD8asV5,
    #[serde(rename = "Standard_D16as_v5")]
    StandardD16asV5,
    #[serde(rename = "Standard_D32as_v5")]
    StandardD32asV5,
    #[serde(rename = "Standard_D48as_v5")]
    StandardD48asV5,
    #[serde(rename = "Standard_D64as_v5")]
    StandardD64asV5,
    #[serde(rename = "Standard_D96as_v5")]
    StandardD96asV5,
    #[serde(rename = "Standard_D2ads_v5")]
    StandardD2adsV5,
    #[serde(rename = "Standard_D4ads_v5")]
    StandardD4adsV5,
    #[serde(rename = "Standard_D8ads_v5")]
    StandardD8adsV5,
    #[serde(rename = "Standard_D16ads_v5")]
    StandardD16adsV5,
    #[serde(rename = "Standard_D32ads_v5")]
    StandardD32adsV5,
    #[serde(rename = "Standard_D48ads_v5")]
    StandardD48adsV5,
    #[serde(rename = "Standard_D64ads_v5")]
    StandardD64adsV5,
    #[serde(rename = "Standard_D96ads_v5")]
    StandardD96adsV5,
    #[serde(rename = "Standard_E2_v5")]
    StandardE2V5,
    #[serde(rename = "Standard_E4_v5")]
    StandardE4V5,
    #[serde(rename = "Standard_E8_v5")]
    StandardE8V5,
    #[serde(rename = "Standard_E16_v5")]
    StandardE16V5,
    #[serde(rename = "Standard_E20_v5")]
    StandardE20V5,
    #[serde(rename = "Standard_E32_v5")]
    StandardE32V5,
    #[serde(rename = "Standard_E48_v5")]
    StandardE48V5,
    #[serde(rename = "Standard_E64_v5")]
    StandardE64V5,
    #[serde(rename = "Standard_E96_v5")]
    StandardE96V5,
    #[serde(rename = "Standard_E104i_v5")]
    StandardE104iV5,
    #[serde(rename = "Standard_E2s_v5")]
    StandardE2sV5,
    #[serde(rename = "Standard_E4s_v5")]
    StandardE4sV5,
    #[serde(rename = "Standard_E8s_v5")]
    StandardE8sV5,
    #[serde(rename = "Standard_E16s_v5")]
    StandardE16sV5,
    #[serde(rename = "Standard_E20s_v5")]
    StandardE20sV5,
    #[serde(rename = "Standard_E32s_v5")]
    StandardE32sV5,
    #[serde(rename = "Standard_E48s_v5")]
    StandardE48sV5,
    #[serde(rename = "Standard_E64s_v5")]
    StandardE64sV5,
    #[serde(rename = "Standard_E96s_v5")]
    StandardE96sV5,
    #[serde(rename = "Standard_E104is_v5")]
    StandardE104isV5,
    #[serde(rename = "Standard_E2d_v5")]
    StandardE2dV5,
    #[serde(rename = "Standard_E4d_v5")]
    StandardE4dV5,
    #[serde(rename = "Standard_E8d_v5")]
    StandardE8dV5,
    #[serde(rename = "Standard_E16d_v5")]
    StandardE16dV5,
    #[serde(rename = "Standard_E20d_v5")]
    StandardE20dV5,
    #[serde(rename = "Standard_E32d_v5")]
    StandardE32dV5,
    #[serde(rename = "Standard_E48d_v5")]
    StandardE48dV5,
    #[serde(rename = "Standard_E64d_v5")]
    StandardE64dV5,
    #[serde(rename = "Standard_E96d_v5")]
    StandardE96dV5,
    #[serde(rename = "Standard_E104id_v5")]
    StandardE104idV5,
    #[serde(rename = "Standard_E2ds_v5")]
    StandardE2dsV5,
    #[serde(rename = "Standard_E4ds_v5")]
    StandardE4dsV5,
    #[serde(rename = "Standard_E8ds_v5")]
    StandardE8dsV5,
    #[serde(rename = "Standard_E16ds_v5")]
    StandardE16dsV5,
    #[serde(rename = "Standard_E20ds_v5")]
    StandardE20dsV5,
    #[serde(rename = "Standard_E32ds_v5")]
    StandardE32dsV5,
    #[serde(rename = "Standard_E48ds_v5")]
    StandardE48dsV5,
    #[serde(rename = "Standard_E64ds_v5")]
    StandardE64dsV5,
    #[serde(rename = "Standard_E96ds_v5")]
    StandardE96dsV5,
    #[serde(rename = "Standard_E104ids_v5")]
    StandardE104idsV5,
    #[serde(rename = "Standard_E2as_v5")]
    StandardE2asV5,
    #[serde(rename = "Standard_E4as_v5")]
    StandardE4asV5,
    #[serde(rename = "Standard_E8as_v5")]
    StandardE8asV5,
    #[serde(rename = "Standard_E16as_v5")]
    StandardE16asV5,
    #[serde(rename = "Standard_E20as_v5")]
    StandardE20asV5,
    #[serde(rename = "Standard_E32as_v5")]
    StandardE32asV5,
    #[serde(rename = "Standard_E48as_v5")]
    StandardE48asV5,
    #[serde(rename = "Standard_E64as_v5")]
    StandardE64asV5,
    #[serde(rename = "Standard_E96as_v5")]
    StandardE96asV5,
    #[serde(rename = "Standard_E2ads_v5")]
    StandardE2adsV5,
    #[serde(rename = "Standard_E4ads_v5")]
    StandardE4adsV5,
    #[serde(rename = "Standard_E8ads_v5")]
    StandardE8adsV5,
    #[serde(rename = "Standard_E16ads_v5")]
    StandardE16adsV5,
    #[serde(rename = "Standard_E20ads_v5")]
    StandardE20adsV5,
    #[serde(rename = "Standard_E32ads_v5")]
    StandardE32adsV5,
    #[serde(rename = "Standard_E48ads_v5")]
    StandardE48adsV5,
    #[serde(rename = "Standard_E64ads_v5")]
    StandardE64adsV5,
    #[serde(rename = "Standard_E96ads_v5")]
    StandardE96adsV5,
    #[serde(rename = "Standard_M8_2ms")]
    StandardM82ms,
    #[serde(rename = "Standard_M8_4ms")]
    StandardM84ms,
    #[serde(rename = "Standard_M16_4ms")]
    StandardM164ms,
    #[serde(rename = "Standard_M16_8ms")]
    StandardM168ms,
    #[serde(rename = "Standard_M32_8ms")]
    StandardM328ms,
    #[serde(rename = "Standard_M32_16ms")]
    StandardM3216ms,
    #[serde(rename = "Standard_M64_32ms")]
    StandardM6432ms,
    #[serde(rename = "Standard_M64_16ms")]
    StandardM6416ms,
    #[serde(rename = "Standard_M128_64ms")]
    StandardM12864ms,
    #[serde(rename = "Standard_M128_32ms")]
    StandardM12832ms,
    #[serde(rename = "Standard_E4_2s_v3")]
    StandardE42sV3,
    #[serde(rename = "Standard_E8_4s_v3")]
    StandardE84sV3,
    #[serde(rename = "Standard_E8_2s_v3")]
    StandardE82sV3,
    #[serde(rename = "Standard_E16_8s_v3")]
    StandardE168sV3,
    #[serde(rename = "Standard_E16_4s_v3")]
    StandardE164sV3,
    #[serde(rename = "Standard_E32_16s_v3")]
    StandardE3216sV3,
    #[serde(rename = "Standard_E32_8s_v3")]
    StandardE328sV3,
    #[serde(rename = "Standard_E64_32s_v3")]
    StandardE6432sV3,
    #[serde(rename = "Standard_E64_16s_v3")]
    StandardE6416sV3,
    #[serde(rename = "Standard_E4_2s_v4")]
    StandardE42sV4,
    #[serde(rename = "Standard_E8_4s_v4")]
    StandardE84sV4,
    #[serde(rename = "Standard_E8_2s_v4")]
    StandardE82sV4,
    #[serde(rename = "Standard_E16_8s_v4")]
    StandardE168sV4,
    #[serde(rename = "Standard_E16_4s_v4")]
    StandardE164sV4,
    #[serde(rename = "Standard_E32_16s_v4")]
    StandardE3216sV4,
    #[serde(rename = "Standard_E32_8s_v4")]
    StandardE328sV4,
    #[serde(rename = "Standard_E64_32s_v4")]
    StandardE6432sV4,
    #[serde(rename = "Standard_E64_16s_v4")]
    StandardE6416sV4,
    #[serde(rename = "Standard_E4_2ds_v4")]
    StandardE42dsV4,
    #[serde(rename = "Standard_E8_4ds_v4")]
    StandardE84dsV4,
    #[serde(rename = "Standard_E8_2ds_v4")]
    StandardE82dsV4,
    #[serde(rename = "Standard_E16_8ds_v4")]
    StandardE168dsV4,
    #[serde(rename = "Standard_E16_4ds_v4")]
    StandardE164dsV4,
    #[serde(rename = "Standard_E32_16ds_v4")]
    StandardE3216dsV4,
    #[serde(rename = "Standard_E32_8ds_v4")]
    StandardE328dsV4,
    #[serde(rename = "Standard_E64_32ds_v4")]
    StandardE6432dsV4,
    #[serde(rename = "Standard_E64_16ds_v4")]
    StandardE6416dsV4,
    #[serde(rename = "Standard_E4_2as_v4")]
    StandardE42asV4,
    #[serde(rename = "Standard_E8_4as_v4")]
    StandardE84asV4,
    #[serde(rename = "Standard_E8_2as_v4")]
    StandardE82asV4,
    #[serde(rename = "Standard_E16_8as_v4")]
    StandardE168asV4,
    #[serde(rename = "Standard_E16_4as_v4")]
    StandardE164asV4,
    #[serde(rename = "Standard_E32_16as_v4")]
    StandardE3216asV4,
    #[serde(rename = "Standard_E32_8as_v4")]
    StandardE328asV4,
    #[serde(rename = "Standard_E64_32as_v4")]
    StandardE6432asV4,
    #[serde(rename = "Standard_E64_16as_v4")]
    StandardE6416asV4,
    #[serde(rename = "Standard_E96_48as_v4")]
    StandardE9648asV4,
    #[serde(rename = "Standard_E96_24as_v4")]
    StandardE9624asV4,
    #[serde(rename = "Standard_E4_2ads_v5")]
    StandardE42adsV5,
    #[serde(rename = "Standard_E8_4ads_v5")]
    StandardE84adsV5,
    #[serde(rename = "Standard_E8_2ads_v5")]
    StandardE82adsV5,
    #[serde(rename = "Standard_E16_8ads_v5")]
    StandardE168adsV5,
    #[serde(rename = "Standard_E16_4ads_v5")]
    StandardE164adsV5,
    #[serde(rename = "Standard_E32_16ads_v5")]
    StandardE3216adsV5,
    #[serde(rename = "Standard_E32_8ads_v5")]
    StandardE328adsV5,
    #[serde(rename = "Standard_E64_32ads_v5")]
    StandardE6432adsV5,
    #[serde(rename = "Standard_E64_16ads_v5")]
    StandardE6416adsV5,
    #[serde(rename = "Standard_E96_48ads_v5")]
    StandardE9648adsV5,
    #[serde(rename = "Standard_E96_24ads_v5")]
    StandardE9624adsV5,
    #[serde(rename = "Standard_E4_2s_v5")]
    StandardE42sV5,
    #[serde(rename = "Standard_E8_4s_v5")]
    StandardE84sV5,
    #[serde(rename = "Standard_E8_2s_v5")]
    StandardE82sV5,
    #[serde(rename = "Standard_E16_8s_v5")]
    StandardE168sV5,
    #[serde(rename = "Standard_E16_4s_v5")]
    StandardE164sV5,
    #[serde(rename = "Standard_E32_16s_v5")]
    StandardE3216sV5,
    #[serde(rename = "Standard_E32_8s_v5")]
    StandardE328sV5,
    #[serde(rename = "Standard_E64_32s_v5")]
    StandardE6432sV5,
    #[serde(rename = "Standard_E64_16s_v5")]
    StandardE6416sV5,
    #[serde(rename = "Standard_E96_48s_v5")]
    StandardE9648sV5,
    #[serde(rename = "Standard_E96_24s_v5")]
    StandardE9624sV5,
    #[serde(rename = "Standard_E4_2ds_v5")]
    StandardE42dsV5,
    #[serde(rename = "Standard_E8_4ds_v5")]
    StandardE84dsV5,
    #[serde(rename = "Standard_E8_2ds_v5")]
    StandardE82dsV5,
    #[serde(rename = "Standard_E16_8ds_v5")]
    StandardE168dsV5,
    #[serde(rename = "Standard_E16_4ds_v5")]
    StandardE164dsV5,
    #[serde(rename = "Standard_E32_16ds_v5")]
    StandardE3216dsV5,
    #[serde(rename = "Standard_E32_8ds_v5")]
    StandardE328dsV5,
    #[serde(rename = "Standard_E64_32ds_v5")]
    StandardE6432dsV5,
    #[serde(rename = "Standard_E64_16ds_v5")]
    StandardE6416dsV5,
    #[serde(rename = "Standard_E96_48ds_v5")]
    StandardE9648dsV5,
    #[serde(rename = "Standard_E96_24ds_v5")]
    StandardE9624dsV5,
    #[serde(rename = "Standard_E4_2as_v5")]
    StandardE42asV5,
    #[serde(rename = "Standard_E8_4as_v5")]
    StandardE84asV5,
    #[serde(rename = "Standard_E8_2as_v5")]
    StandardE82asV5,
    #[serde(rename = "Standard_E16_8as_v5")]
    StandardE168asV5,
    #[serde(rename = "Standard_E16_4as_v5")]
    StandardE164asV5,
    #[serde(rename = "Standard_E32_16as_v5")]
    StandardE3216asV5,
    #[serde(rename = "Standard_E32_8as_v5")]
    StandardE328asV5,
    #[serde(rename = "Standard_E64_32as_v5")]
    StandardE6432asV5,
    #[serde(rename = "Standard_E64_16as_v5")]
    StandardE6416asV5,
    #[serde(rename = "Standard_E96_48as_v5")]
    StandardE9648asV5,
    #[serde(rename = "Standard_E96_24as_v5")]
    StandardE9624asV5,
    #[serde(rename = "Standard_GS4_8")]
    StandardGs48,
    #[serde(rename = "Standard_GS4_4")]
    StandardGs44,
    #[serde(rename = "Standard_GS5_16")]
    StandardGs516,
    #[serde(rename = "Standard_GS5_8")]
    StandardGs58,
    #[serde(rename = "Standard_DS11_1_v2")]
    StandardDs111V2,
    #[serde(rename = "Standard_DS12_2_v2")]
    StandardDs122V2,
    #[serde(rename = "Standard_DS12_1_v2")]
    StandardDs121V2,
    #[serde(rename = "Standard_DS13_4_v2")]
    StandardDs134V2,
    #[serde(rename = "Standard_DS13_2_v2")]
    StandardDs132V2,
    #[serde(rename = "Standard_DS14_8_v2")]
    StandardDs148V2,
    #[serde(rename = "Standard_DS14_4_v2")]
    StandardDs144V2,
    #[serde(rename = "Standard_M416_208s_v2")]
    StandardM416208sV2,
    #[serde(rename = "Standard_M416_208ms_v2")]
    StandardM416208msV2,
    #[serde(rename = "Standard_E2bs_v5")]
    StandardE2bsV5,
    #[serde(rename = "Standard_E4bs_v5")]
    StandardE4bsV5,
    #[serde(rename = "Standard_E8bs_v5")]
    StandardE8bsV5,
    #[serde(rename = "Standard_E16bs_v5")]
    StandardE16bsV5,
    #[serde(rename = "Standard_E32bs_v5")]
    StandardE32bsV5,
    #[serde(rename = "Standard_E48bs_v5")]
    StandardE48bsV5,
    #[serde(rename = "Standard_E64bs_v5")]
    StandardE64bsV5,
    #[serde(rename = "Standard_E2bds_v5")]
    StandardE2bdsV5,
    #[serde(rename = "Standard_E4bds_v5")]
    StandardE4bdsV5,
    #[serde(rename = "Standard_E8bds_v5")]
    StandardE8bdsV5,
    #[serde(rename = "Standard_E16bds_v5")]
    StandardE16bdsV5,
    #[serde(rename = "Standard_E32bds_v5")]
    StandardE32bdsV5,
    #[serde(rename = "Standard_E48bds_v5")]
    StandardE48bdsV5,
    #[serde(rename = "Standard_E64bds_v5")]
    StandardE64bdsV5,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureVmSize {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureVmSize {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureVmSize {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureVmSize", 0u32, "Unknown"),
            Self::BasicA0 => serializer.serialize_unit_variant("AzureVmSize", 1u32, "Basic_A0"),
            Self::BasicA1 => serializer.serialize_unit_variant("AzureVmSize", 2u32, "Basic_A1"),
            Self::BasicA2 => serializer.serialize_unit_variant("AzureVmSize", 3u32, "Basic_A2"),
            Self::BasicA3 => serializer.serialize_unit_variant("AzureVmSize", 4u32, "Basic_A3"),
            Self::BasicA4 => serializer.serialize_unit_variant("AzureVmSize", 5u32, "Basic_A4"),
            Self::StandardA0 => serializer.serialize_unit_variant("AzureVmSize", 6u32, "Standard_A0"),
            Self::StandardA1 => serializer.serialize_unit_variant("AzureVmSize", 7u32, "Standard_A1"),
            Self::StandardA2 => serializer.serialize_unit_variant("AzureVmSize", 8u32, "Standard_A2"),
            Self::StandardA3 => serializer.serialize_unit_variant("AzureVmSize", 9u32, "Standard_A3"),
            Self::StandardA4 => serializer.serialize_unit_variant("AzureVmSize", 10u32, "Standard_A4"),
            Self::StandardA5 => serializer.serialize_unit_variant("AzureVmSize", 11u32, "Standard_A5"),
            Self::StandardA6 => serializer.serialize_unit_variant("AzureVmSize", 12u32, "Standard_A6"),
            Self::StandardA7 => serializer.serialize_unit_variant("AzureVmSize", 13u32, "Standard_A7"),
            Self::StandardA8 => serializer.serialize_unit_variant("AzureVmSize", 14u32, "Standard_A8"),
            Self::StandardA9 => serializer.serialize_unit_variant("AzureVmSize", 15u32, "Standard_A9"),
            Self::StandardA10 => serializer.serialize_unit_variant("AzureVmSize", 16u32, "Standard_A10"),
            Self::StandardA11 => serializer.serialize_unit_variant("AzureVmSize", 17u32, "Standard_A11"),
            Self::StandardA1V2 => serializer.serialize_unit_variant("AzureVmSize", 18u32, "Standard_A1_v2"),
            Self::StandardA2V2 => serializer.serialize_unit_variant("AzureVmSize", 19u32, "Standard_A2_v2"),
            Self::StandardA4V2 => serializer.serialize_unit_variant("AzureVmSize", 20u32, "Standard_A4_v2"),
            Self::StandardA8V2 => serializer.serialize_unit_variant("AzureVmSize", 21u32, "Standard_A8_v2"),
            Self::StandardA2mV2 => serializer.serialize_unit_variant("AzureVmSize", 22u32, "Standard_A2m_v2"),
            Self::StandardA4mV2 => serializer.serialize_unit_variant("AzureVmSize", 23u32, "Standard_A4m_v2"),
            Self::StandardA8mV2 => serializer.serialize_unit_variant("AzureVmSize", 24u32, "Standard_A8m_v2"),
            Self::StandardD1 => serializer.serialize_unit_variant("AzureVmSize", 25u32, "Standard_D1"),
            Self::StandardD2 => serializer.serialize_unit_variant("AzureVmSize", 26u32, "Standard_D2"),
            Self::StandardD3 => serializer.serialize_unit_variant("AzureVmSize", 27u32, "Standard_D3"),
            Self::StandardD4 => serializer.serialize_unit_variant("AzureVmSize", 28u32, "Standard_D4"),
            Self::StandardD11 => serializer.serialize_unit_variant("AzureVmSize", 29u32, "Standard_D11"),
            Self::StandardD12 => serializer.serialize_unit_variant("AzureVmSize", 30u32, "Standard_D12"),
            Self::StandardD13 => serializer.serialize_unit_variant("AzureVmSize", 31u32, "Standard_D13"),
            Self::StandardD14 => serializer.serialize_unit_variant("AzureVmSize", 32u32, "Standard_D14"),
            Self::StandardD1V2 => serializer.serialize_unit_variant("AzureVmSize", 33u32, "Standard_D1_v2"),
            Self::StandardD2V2 => serializer.serialize_unit_variant("AzureVmSize", 34u32, "Standard_D2_v2"),
            Self::StandardD3V2 => serializer.serialize_unit_variant("AzureVmSize", 35u32, "Standard_D3_v2"),
            Self::StandardD4V2 => serializer.serialize_unit_variant("AzureVmSize", 36u32, "Standard_D4_v2"),
            Self::StandardD5V2 => serializer.serialize_unit_variant("AzureVmSize", 37u32, "Standard_D5_v2"),
            Self::StandardD11V2 => serializer.serialize_unit_variant("AzureVmSize", 38u32, "Standard_D11_v2"),
            Self::StandardD12V2 => serializer.serialize_unit_variant("AzureVmSize", 39u32, "Standard_D12_v2"),
            Self::StandardD13V2 => serializer.serialize_unit_variant("AzureVmSize", 40u32, "Standard_D13_v2"),
            Self::StandardD14V2 => serializer.serialize_unit_variant("AzureVmSize", 41u32, "Standard_D14_v2"),
            Self::StandardD15V2 => serializer.serialize_unit_variant("AzureVmSize", 42u32, "Standard_D15_v2"),
            Self::StandardDs1 => serializer.serialize_unit_variant("AzureVmSize", 43u32, "Standard_DS1"),
            Self::StandardDs2 => serializer.serialize_unit_variant("AzureVmSize", 44u32, "Standard_DS2"),
            Self::StandardDs3 => serializer.serialize_unit_variant("AzureVmSize", 45u32, "Standard_DS3"),
            Self::StandardDs4 => serializer.serialize_unit_variant("AzureVmSize", 46u32, "Standard_DS4"),
            Self::StandardDs11 => serializer.serialize_unit_variant("AzureVmSize", 47u32, "Standard_DS11"),
            Self::StandardDs12 => serializer.serialize_unit_variant("AzureVmSize", 48u32, "Standard_DS12"),
            Self::StandardDs13 => serializer.serialize_unit_variant("AzureVmSize", 49u32, "Standard_DS13"),
            Self::StandardDs14 => serializer.serialize_unit_variant("AzureVmSize", 50u32, "Standard_DS14"),
            Self::StandardDs1V2 => serializer.serialize_unit_variant("AzureVmSize", 51u32, "Standard_DS1_v2"),
            Self::StandardDs2V2 => serializer.serialize_unit_variant("AzureVmSize", 52u32, "Standard_DS2_v2"),
            Self::StandardDs3V2 => serializer.serialize_unit_variant("AzureVmSize", 53u32, "Standard_DS3_v2"),
            Self::StandardDs4V2 => serializer.serialize_unit_variant("AzureVmSize", 54u32, "Standard_DS4_v2"),
            Self::StandardDs5V2 => serializer.serialize_unit_variant("AzureVmSize", 55u32, "Standard_DS5_v2"),
            Self::StandardDs11V2 => serializer.serialize_unit_variant("AzureVmSize", 56u32, "Standard_DS11_v2"),
            Self::StandardDs12V2 => serializer.serialize_unit_variant("AzureVmSize", 57u32, "Standard_DS12_v2"),
            Self::StandardDs13V2 => serializer.serialize_unit_variant("AzureVmSize", 58u32, "Standard_DS13_v2"),
            Self::StandardDs14V2 => serializer.serialize_unit_variant("AzureVmSize", 59u32, "Standard_DS14_v2"),
            Self::StandardDs15V2 => serializer.serialize_unit_variant("AzureVmSize", 60u32, "Standard_DS15_v2"),
            Self::StandardF1 => serializer.serialize_unit_variant("AzureVmSize", 61u32, "Standard_F1"),
            Self::StandardF2 => serializer.serialize_unit_variant("AzureVmSize", 62u32, "Standard_F2"),
            Self::StandardF4 => serializer.serialize_unit_variant("AzureVmSize", 63u32, "Standard_F4"),
            Self::StandardF8 => serializer.serialize_unit_variant("AzureVmSize", 64u32, "Standard_F8"),
            Self::StandardF16 => serializer.serialize_unit_variant("AzureVmSize", 65u32, "Standard_F16"),
            Self::StandardF1s => serializer.serialize_unit_variant("AzureVmSize", 66u32, "Standard_F1s"),
            Self::StandardF2s => serializer.serialize_unit_variant("AzureVmSize", 67u32, "Standard_F2s"),
            Self::StandardF4s => serializer.serialize_unit_variant("AzureVmSize", 68u32, "Standard_F4s"),
            Self::StandardF8s => serializer.serialize_unit_variant("AzureVmSize", 69u32, "Standard_F8s"),
            Self::StandardF16s => serializer.serialize_unit_variant("AzureVmSize", 70u32, "Standard_F16s"),
            Self::StandardG1 => serializer.serialize_unit_variant("AzureVmSize", 71u32, "Standard_G1"),
            Self::StandardG2 => serializer.serialize_unit_variant("AzureVmSize", 72u32, "Standard_G2"),
            Self::StandardG3 => serializer.serialize_unit_variant("AzureVmSize", 73u32, "Standard_G3"),
            Self::StandardG4 => serializer.serialize_unit_variant("AzureVmSize", 74u32, "Standard_G4"),
            Self::StandardG5 => serializer.serialize_unit_variant("AzureVmSize", 75u32, "Standard_G5"),
            Self::StandardGs1 => serializer.serialize_unit_variant("AzureVmSize", 76u32, "Standard_GS1"),
            Self::StandardGs2 => serializer.serialize_unit_variant("AzureVmSize", 77u32, "Standard_GS2"),
            Self::StandardGs3 => serializer.serialize_unit_variant("AzureVmSize", 78u32, "Standard_GS3"),
            Self::StandardGs4 => serializer.serialize_unit_variant("AzureVmSize", 79u32, "Standard_GS4"),
            Self::StandardGs5 => serializer.serialize_unit_variant("AzureVmSize", 80u32, "Standard_GS5"),
            Self::StandardH8 => serializer.serialize_unit_variant("AzureVmSize", 81u32, "Standard_H8"),
            Self::StandardH16 => serializer.serialize_unit_variant("AzureVmSize", 82u32, "Standard_H16"),
            Self::StandardH8m => serializer.serialize_unit_variant("AzureVmSize", 83u32, "Standard_H8m"),
            Self::StandardH16m => serializer.serialize_unit_variant("AzureVmSize", 84u32, "Standard_H16m"),
            Self::StandardH16r => serializer.serialize_unit_variant("AzureVmSize", 85u32, "Standard_H16r"),
            Self::StandardH16mr => serializer.serialize_unit_variant("AzureVmSize", 86u32, "Standard_H16mr"),
            Self::StandardL4s => serializer.serialize_unit_variant("AzureVmSize", 87u32, "Standard_L4s"),
            Self::StandardL8s => serializer.serialize_unit_variant("AzureVmSize", 88u32, "Standard_L8s"),
            Self::StandardL16s => serializer.serialize_unit_variant("AzureVmSize", 89u32, "Standard_L16s"),
            Self::StandardL32s => serializer.serialize_unit_variant("AzureVmSize", 90u32, "Standard_L32s"),
            Self::StandardD2sV3 => serializer.serialize_unit_variant("AzureVmSize", 91u32, "Standard_D2s_v3"),
            Self::StandardD4sV3 => serializer.serialize_unit_variant("AzureVmSize", 92u32, "Standard_D4s_v3"),
            Self::StandardD8sV3 => serializer.serialize_unit_variant("AzureVmSize", 93u32, "Standard_D8s_v3"),
            Self::StandardD16sV3 => serializer.serialize_unit_variant("AzureVmSize", 94u32, "Standard_D16s_v3"),
            Self::StandardD32sV3 => serializer.serialize_unit_variant("AzureVmSize", 95u32, "Standard_D32s_v3"),
            Self::StandardD64sV3 => serializer.serialize_unit_variant("AzureVmSize", 96u32, "Standard_D64s_v3"),
            Self::StandardD2V3 => serializer.serialize_unit_variant("AzureVmSize", 97u32, "Standard_D2_v3"),
            Self::StandardD4V3 => serializer.serialize_unit_variant("AzureVmSize", 98u32, "Standard_D4_v3"),
            Self::StandardD8V3 => serializer.serialize_unit_variant("AzureVmSize", 99u32, "Standard_D8_v3"),
            Self::StandardD16V3 => serializer.serialize_unit_variant("AzureVmSize", 100u32, "Standard_D16_v3"),
            Self::StandardD32V3 => serializer.serialize_unit_variant("AzureVmSize", 101u32, "Standard_D32_v3"),
            Self::StandardD64V3 => serializer.serialize_unit_variant("AzureVmSize", 102u32, "Standard_D64_v3"),
            Self::StandardF2sV2 => serializer.serialize_unit_variant("AzureVmSize", 103u32, "Standard_F2s_v2"),
            Self::StandardF4sV2 => serializer.serialize_unit_variant("AzureVmSize", 104u32, "Standard_F4s_v2"),
            Self::StandardF8sV2 => serializer.serialize_unit_variant("AzureVmSize", 105u32, "Standard_F8s_v2"),
            Self::StandardF16sV2 => serializer.serialize_unit_variant("AzureVmSize", 106u32, "Standard_F16s_v2"),
            Self::StandardF32sV2 => serializer.serialize_unit_variant("AzureVmSize", 107u32, "Standard_F32s_v2"),
            Self::StandardF64sV2 => serializer.serialize_unit_variant("AzureVmSize", 108u32, "Standard_F64s_v2"),
            Self::StandardF72sV2 => serializer.serialize_unit_variant("AzureVmSize", 109u32, "Standard_F72s_v2"),
            Self::StandardE2V3 => serializer.serialize_unit_variant("AzureVmSize", 110u32, "Standard_E2_v3"),
            Self::StandardE4V3 => serializer.serialize_unit_variant("AzureVmSize", 111u32, "Standard_E4_v3"),
            Self::StandardE8V3 => serializer.serialize_unit_variant("AzureVmSize", 112u32, "Standard_E8_v3"),
            Self::StandardE16V3 => serializer.serialize_unit_variant("AzureVmSize", 113u32, "Standard_E16_v3"),
            Self::StandardE32V3 => serializer.serialize_unit_variant("AzureVmSize", 114u32, "Standard_E32_v3"),
            Self::StandardE64V3 => serializer.serialize_unit_variant("AzureVmSize", 115u32, "Standard_E64_v3"),
            Self::StandardE2sV3 => serializer.serialize_unit_variant("AzureVmSize", 116u32, "Standard_E2s_v3"),
            Self::StandardE4sV3 => serializer.serialize_unit_variant("AzureVmSize", 117u32, "Standard_E4s_v3"),
            Self::StandardE8sV3 => serializer.serialize_unit_variant("AzureVmSize", 118u32, "Standard_E8s_v3"),
            Self::StandardE16sV3 => serializer.serialize_unit_variant("AzureVmSize", 119u32, "Standard_E16s_v3"),
            Self::StandardE32sV3 => serializer.serialize_unit_variant("AzureVmSize", 120u32, "Standard_E32s_v3"),
            Self::StandardE64sV3 => serializer.serialize_unit_variant("AzureVmSize", 121u32, "Standard_E64s_v3"),
            Self::StandardM64s => serializer.serialize_unit_variant("AzureVmSize", 122u32, "Standard_M64s"),
            Self::StandardM64ms => serializer.serialize_unit_variant("AzureVmSize", 123u32, "Standard_M64ms"),
            Self::StandardM128s => serializer.serialize_unit_variant("AzureVmSize", 124u32, "Standard_M128s"),
            Self::StandardM128ms => serializer.serialize_unit_variant("AzureVmSize", 125u32, "Standard_M128ms"),
            Self::StandardDc2s => serializer.serialize_unit_variant("AzureVmSize", 126u32, "Standard_DC2s"),
            Self::StandardDc4s => serializer.serialize_unit_variant("AzureVmSize", 127u32, "Standard_DC4s"),
            Self::StandardE20V3 => serializer.serialize_unit_variant("AzureVmSize", 128u32, "Standard_E20_v3"),
            Self::StandardE20sV3 => serializer.serialize_unit_variant("AzureVmSize", 129u32, "Standard_E20s_v3"),
            Self::StandardE64iV3 => serializer.serialize_unit_variant("AzureVmSize", 130u32, "Standard_E64i_v3"),
            Self::StandardE64isV3 => serializer.serialize_unit_variant("AzureVmSize", 131u32, "Standard_E64is_v3"),
            Self::StandardM8ms => serializer.serialize_unit_variant("AzureVmSize", 132u32, "Standard_M8ms"),
            Self::StandardM16ms => serializer.serialize_unit_variant("AzureVmSize", 133u32, "Standard_M16ms"),
            Self::StandardM32ls => serializer.serialize_unit_variant("AzureVmSize", 134u32, "Standard_M32ls"),
            Self::StandardM32ms => serializer.serialize_unit_variant("AzureVmSize", 135u32, "Standard_M32ms"),
            Self::StandardM32ts => serializer.serialize_unit_variant("AzureVmSize", 136u32, "Standard_M32ts"),
            Self::StandardM64 => serializer.serialize_unit_variant("AzureVmSize", 137u32, "Standard_M64"),
            Self::StandardM64ls => serializer.serialize_unit_variant("AzureVmSize", 138u32, "Standard_M64ls"),
            Self::StandardM64m => serializer.serialize_unit_variant("AzureVmSize", 139u32, "Standard_M64m"),
            Self::StandardM128 => serializer.serialize_unit_variant("AzureVmSize", 140u32, "Standard_M128"),
            Self::StandardM128m => serializer.serialize_unit_variant("AzureVmSize", 141u32, "Standard_M128m"),
            Self::StandardL8sV2 => serializer.serialize_unit_variant("AzureVmSize", 142u32, "Standard_L8s_v2"),
            Self::StandardL16sV2 => serializer.serialize_unit_variant("AzureVmSize", 143u32, "Standard_L16s_v2"),
            Self::StandardL32sV2 => serializer.serialize_unit_variant("AzureVmSize", 144u32, "Standard_L32s_v2"),
            Self::StandardL48sV2 => serializer.serialize_unit_variant("AzureVmSize", 145u32, "Standard_L48s_v2"),
            Self::StandardL64sV2 => serializer.serialize_unit_variant("AzureVmSize", 146u32, "Standard_L64s_v2"),
            Self::StandardL80sV2 => serializer.serialize_unit_variant("AzureVmSize", 147u32, "Standard_L80s_v2"),
            Self::StandardD2V4 => serializer.serialize_unit_variant("AzureVmSize", 148u32, "Standard_D2_v4"),
            Self::StandardD4V4 => serializer.serialize_unit_variant("AzureVmSize", 149u32, "Standard_D4_v4"),
            Self::StandardD8V4 => serializer.serialize_unit_variant("AzureVmSize", 150u32, "Standard_D8_v4"),
            Self::StandardD16V4 => serializer.serialize_unit_variant("AzureVmSize", 151u32, "Standard_D16_v4"),
            Self::StandardD32V4 => serializer.serialize_unit_variant("AzureVmSize", 152u32, "Standard_D32_v4"),
            Self::StandardD48V4 => serializer.serialize_unit_variant("AzureVmSize", 153u32, "Standard_D48_v4"),
            Self::StandardD64V4 => serializer.serialize_unit_variant("AzureVmSize", 154u32, "Standard_D64_v4"),
            Self::StandardD2sV4 => serializer.serialize_unit_variant("AzureVmSize", 155u32, "Standard_D2s_v4"),
            Self::StandardD4sV4 => serializer.serialize_unit_variant("AzureVmSize", 156u32, "Standard_D4s_v4"),
            Self::StandardD8sV4 => serializer.serialize_unit_variant("AzureVmSize", 157u32, "Standard_D8s_v4"),
            Self::StandardD16sV4 => serializer.serialize_unit_variant("AzureVmSize", 158u32, "Standard_D16s_v4"),
            Self::StandardD32sV4 => serializer.serialize_unit_variant("AzureVmSize", 159u32, "Standard_D32s_v4"),
            Self::StandardD48sV4 => serializer.serialize_unit_variant("AzureVmSize", 160u32, "Standard_D48s_v4"),
            Self::StandardD64sV4 => serializer.serialize_unit_variant("AzureVmSize", 161u32, "Standard_D64s_v4"),
            Self::StandardD2dV4 => serializer.serialize_unit_variant("AzureVmSize", 162u32, "Standard_D2d_v4"),
            Self::StandardD4dV4 => serializer.serialize_unit_variant("AzureVmSize", 163u32, "Standard_D4d_v4"),
            Self::StandardD8dV4 => serializer.serialize_unit_variant("AzureVmSize", 164u32, "Standard_D8d_v4"),
            Self::StandardD16dV4 => serializer.serialize_unit_variant("AzureVmSize", 165u32, "Standard_D16d_v4"),
            Self::StandardD32dV4 => serializer.serialize_unit_variant("AzureVmSize", 166u32, "Standard_D32d_v4"),
            Self::StandardD48dV4 => serializer.serialize_unit_variant("AzureVmSize", 167u32, "Standard_D48d_v4"),
            Self::StandardD64dV4 => serializer.serialize_unit_variant("AzureVmSize", 168u32, "Standard_D64d_v4"),
            Self::StandardD2dsV4 => serializer.serialize_unit_variant("AzureVmSize", 169u32, "Standard_D2ds_v4"),
            Self::StandardD4dsV4 => serializer.serialize_unit_variant("AzureVmSize", 170u32, "Standard_D4ds_v4"),
            Self::StandardD8dsV4 => serializer.serialize_unit_variant("AzureVmSize", 171u32, "Standard_D8ds_v4"),
            Self::StandardD16dsV4 => serializer.serialize_unit_variant("AzureVmSize", 172u32, "Standard_D16ds_v4"),
            Self::StandardD32dsV4 => serializer.serialize_unit_variant("AzureVmSize", 173u32, "Standard_D32ds_v4"),
            Self::StandardD48dsV4 => serializer.serialize_unit_variant("AzureVmSize", 174u32, "Standard_D48ds_v4"),
            Self::StandardD64dsV4 => serializer.serialize_unit_variant("AzureVmSize", 175u32, "Standard_D64ds_v4"),
            Self::StandardE2V4 => serializer.serialize_unit_variant("AzureVmSize", 176u32, "Standard_E2_v4"),
            Self::StandardE4V4 => serializer.serialize_unit_variant("AzureVmSize", 177u32, "Standard_E4_v4"),
            Self::StandardE8V4 => serializer.serialize_unit_variant("AzureVmSize", 178u32, "Standard_E8_v4"),
            Self::StandardE16V4 => serializer.serialize_unit_variant("AzureVmSize", 179u32, "Standard_E16_v4"),
            Self::StandardE20V4 => serializer.serialize_unit_variant("AzureVmSize", 180u32, "Standard_E20_v4"),
            Self::StandardE32V4 => serializer.serialize_unit_variant("AzureVmSize", 181u32, "Standard_E32_v4"),
            Self::StandardE48V4 => serializer.serialize_unit_variant("AzureVmSize", 182u32, "Standard_E48_v4"),
            Self::StandardE64V4 => serializer.serialize_unit_variant("AzureVmSize", 183u32, "Standard_E64_v4"),
            Self::StandardE2sV4 => serializer.serialize_unit_variant("AzureVmSize", 184u32, "Standard_E2s_v4"),
            Self::StandardE4sV4 => serializer.serialize_unit_variant("AzureVmSize", 185u32, "Standard_E4s_v4"),
            Self::StandardE8sV4 => serializer.serialize_unit_variant("AzureVmSize", 186u32, "Standard_E8s_v4"),
            Self::StandardE16sV4 => serializer.serialize_unit_variant("AzureVmSize", 187u32, "Standard_E16s_v4"),
            Self::StandardE20sV4 => serializer.serialize_unit_variant("AzureVmSize", 188u32, "Standard_E20s_v4"),
            Self::StandardE32sV4 => serializer.serialize_unit_variant("AzureVmSize", 189u32, "Standard_E32s_v4"),
            Self::StandardE48sV4 => serializer.serialize_unit_variant("AzureVmSize", 190u32, "Standard_E48s_v4"),
            Self::StandardE64sV4 => serializer.serialize_unit_variant("AzureVmSize", 191u32, "Standard_E64s_v4"),
            Self::StandardE2dV4 => serializer.serialize_unit_variant("AzureVmSize", 192u32, "Standard_E2d_v4"),
            Self::StandardE4dV4 => serializer.serialize_unit_variant("AzureVmSize", 193u32, "Standard_E4d_v4"),
            Self::StandardE8dV4 => serializer.serialize_unit_variant("AzureVmSize", 194u32, "Standard_E8d_v4"),
            Self::StandardE16dV4 => serializer.serialize_unit_variant("AzureVmSize", 195u32, "Standard_E16d_v4"),
            Self::StandardE20dV4 => serializer.serialize_unit_variant("AzureVmSize", 196u32, "Standard_E20d_v4"),
            Self::StandardE32dV4 => serializer.serialize_unit_variant("AzureVmSize", 197u32, "Standard_E32d_v4"),
            Self::StandardE48dV4 => serializer.serialize_unit_variant("AzureVmSize", 198u32, "Standard_E48d_v4"),
            Self::StandardE64dV4 => serializer.serialize_unit_variant("AzureVmSize", 199u32, "Standard_E64d_v4"),
            Self::StandardE2dsV4 => serializer.serialize_unit_variant("AzureVmSize", 200u32, "Standard_E2ds_v4"),
            Self::StandardE4dsV4 => serializer.serialize_unit_variant("AzureVmSize", 201u32, "Standard_E4ds_v4"),
            Self::StandardE8dsV4 => serializer.serialize_unit_variant("AzureVmSize", 202u32, "Standard_E8ds_v4"),
            Self::StandardE16dsV4 => serializer.serialize_unit_variant("AzureVmSize", 203u32, "Standard_E16ds_v4"),
            Self::StandardE20dsV4 => serializer.serialize_unit_variant("AzureVmSize", 204u32, "Standard_E20ds_v4"),
            Self::StandardE32dsV4 => serializer.serialize_unit_variant("AzureVmSize", 205u32, "Standard_E32ds_v4"),
            Self::StandardE48dsV4 => serializer.serialize_unit_variant("AzureVmSize", 206u32, "Standard_E48ds_v4"),
            Self::StandardE64dsV4 => serializer.serialize_unit_variant("AzureVmSize", 207u32, "Standard_E64ds_v4"),
            Self::StandardE2asV4 => serializer.serialize_unit_variant("AzureVmSize", 208u32, "Standard_E2as_v4"),
            Self::StandardE4asV4 => serializer.serialize_unit_variant("AzureVmSize", 209u32, "Standard_E4as_v4"),
            Self::StandardE8asV4 => serializer.serialize_unit_variant("AzureVmSize", 210u32, "Standard_E8as_v4"),
            Self::StandardE16asV4 => serializer.serialize_unit_variant("AzureVmSize", 211u32, "Standard_E16as_v4"),
            Self::StandardE20asV4 => serializer.serialize_unit_variant("AzureVmSize", 212u32, "Standard_E20as_v4"),
            Self::StandardE32asV4 => serializer.serialize_unit_variant("AzureVmSize", 213u32, "Standard_E32as_v4"),
            Self::StandardE48asV4 => serializer.serialize_unit_variant("AzureVmSize", 214u32, "Standard_E48as_v4"),
            Self::StandardE64asV4 => serializer.serialize_unit_variant("AzureVmSize", 215u32, "Standard_E64as_v4"),
            Self::StandardE96asV4 => serializer.serialize_unit_variant("AzureVmSize", 216u32, "Standard_E96as_v4"),
            Self::StandardD2asV4 => serializer.serialize_unit_variant("AzureVmSize", 217u32, "Standard_D2as_v4"),
            Self::StandardD4asV4 => serializer.serialize_unit_variant("AzureVmSize", 218u32, "Standard_D4as_v4"),
            Self::StandardD8asV4 => serializer.serialize_unit_variant("AzureVmSize", 219u32, "Standard_D8as_v4"),
            Self::StandardD16asV4 => serializer.serialize_unit_variant("AzureVmSize", 220u32, "Standard_D16as_v4"),
            Self::StandardD32asV4 => serializer.serialize_unit_variant("AzureVmSize", 221u32, "Standard_D32as_v4"),
            Self::StandardD48asV4 => serializer.serialize_unit_variant("AzureVmSize", 222u32, "Standard_D48as_v4"),
            Self::StandardD64asV4 => serializer.serialize_unit_variant("AzureVmSize", 223u32, "Standard_D64as_v4"),
            Self::StandardD96asV4 => serializer.serialize_unit_variant("AzureVmSize", 224u32, "Standard_D96as_v4"),
            Self::StandardM208msV2 => serializer.serialize_unit_variant("AzureVmSize", 225u32, "Standard_M208ms_v2"),
            Self::StandardM208sV2 => serializer.serialize_unit_variant("AzureVmSize", 226u32, "Standard_M208s_v2"),
            Self::StandardM416msV2 => serializer.serialize_unit_variant("AzureVmSize", 227u32, "Standard_M416ms_v2"),
            Self::StandardM416sV2 => serializer.serialize_unit_variant("AzureVmSize", 228u32, "Standard_M416s_v2"),
            Self::StandardF48sV2 => serializer.serialize_unit_variant("AzureVmSize", 229u32, "Standard_F48s_v2"),
            Self::StandardE48V3 => serializer.serialize_unit_variant("AzureVmSize", 230u32, "Standard_E48_v3"),
            Self::StandardE48sV3 => serializer.serialize_unit_variant("AzureVmSize", 231u32, "Standard_E48s_v3"),
            Self::StandardE80isV4 => serializer.serialize_unit_variant("AzureVmSize", 232u32, "Standard_E80is_v4"),
            Self::StandardE80idsV4 => serializer.serialize_unit_variant("AzureVmSize", 233u32, "Standard_E80ids_v4"),
            Self::StandardE2aV4 => serializer.serialize_unit_variant("AzureVmSize", 234u32, "Standard_E2a_v4"),
            Self::StandardE4aV4 => serializer.serialize_unit_variant("AzureVmSize", 235u32, "Standard_E4a_v4"),
            Self::StandardE8aV4 => serializer.serialize_unit_variant("AzureVmSize", 236u32, "Standard_E8a_v4"),
            Self::StandardE16aV4 => serializer.serialize_unit_variant("AzureVmSize", 237u32, "Standard_E16a_v4"),
            Self::StandardE20aV4 => serializer.serialize_unit_variant("AzureVmSize", 238u32, "Standard_E20a_v4"),
            Self::StandardE32aV4 => serializer.serialize_unit_variant("AzureVmSize", 239u32, "Standard_E32a_v4"),
            Self::StandardE48aV4 => serializer.serialize_unit_variant("AzureVmSize", 240u32, "Standard_E48a_v4"),
            Self::StandardE64aV4 => serializer.serialize_unit_variant("AzureVmSize", 241u32, "Standard_E64a_v4"),
            Self::StandardE96aV4 => serializer.serialize_unit_variant("AzureVmSize", 242u32, "Standard_E96a_v4"),
            Self::StandardD2aV4 => serializer.serialize_unit_variant("AzureVmSize", 243u32, "Standard_D2a_v4"),
            Self::StandardD4aV4 => serializer.serialize_unit_variant("AzureVmSize", 244u32, "Standard_D4a_v4"),
            Self::StandardD8aV4 => serializer.serialize_unit_variant("AzureVmSize", 245u32, "Standard_D8a_v4"),
            Self::StandardD16aV4 => serializer.serialize_unit_variant("AzureVmSize", 246u32, "Standard_D16a_v4"),
            Self::StandardD32aV4 => serializer.serialize_unit_variant("AzureVmSize", 247u32, "Standard_D32a_v4"),
            Self::StandardD48aV4 => serializer.serialize_unit_variant("AzureVmSize", 248u32, "Standard_D48a_v4"),
            Self::StandardD64aV4 => serializer.serialize_unit_variant("AzureVmSize", 249u32, "Standard_D64a_v4"),
            Self::StandardD96aV4 => serializer.serialize_unit_variant("AzureVmSize", 250u32, "Standard_D96a_v4"),
            Self::StandardM32msV2 => serializer.serialize_unit_variant("AzureVmSize", 251u32, "Standard_M32ms_v2"),
            Self::StandardM64sV2 => serializer.serialize_unit_variant("AzureVmSize", 252u32, "Standard_M64s_v2"),
            Self::StandardM64msV2 => serializer.serialize_unit_variant("AzureVmSize", 253u32, "Standard_M64ms_v2"),
            Self::StandardM128sV2 => serializer.serialize_unit_variant("AzureVmSize", 254u32, "Standard_M128s_v2"),
            Self::StandardM128msV2 => serializer.serialize_unit_variant("AzureVmSize", 255u32, "Standard_M128ms_v2"),
            Self::StandardM192isV2 => serializer.serialize_unit_variant("AzureVmSize", 256u32, "Standard_M192is_v2"),
            Self::StandardM192imsV2 => serializer.serialize_unit_variant("AzureVmSize", 257u32, "Standard_M192ims_v2"),
            Self::StandardM32dmsV2 => serializer.serialize_unit_variant("AzureVmSize", 258u32, "Standard_M32dms_v2"),
            Self::StandardM64dsV2 => serializer.serialize_unit_variant("AzureVmSize", 259u32, "Standard_M64ds_v2"),
            Self::StandardM64dmsV2 => serializer.serialize_unit_variant("AzureVmSize", 260u32, "Standard_M64dms_v2"),
            Self::StandardM128dsV2 => serializer.serialize_unit_variant("AzureVmSize", 261u32, "Standard_M128ds_v2"),
            Self::StandardM128dmsV2 => serializer.serialize_unit_variant("AzureVmSize", 262u32, "Standard_M128dms_v2"),
            Self::StandardM192idsV2 => serializer.serialize_unit_variant("AzureVmSize", 263u32, "Standard_M192ids_v2"),
            Self::StandardM192idmsV2 => serializer.serialize_unit_variant("AzureVmSize", 264u32, "Standard_M192idms_v2"),
            Self::StandardD2V5 => serializer.serialize_unit_variant("AzureVmSize", 265u32, "Standard_D2_v5"),
            Self::StandardD4V5 => serializer.serialize_unit_variant("AzureVmSize", 266u32, "Standard_D4_v5"),
            Self::StandardD8V5 => serializer.serialize_unit_variant("AzureVmSize", 267u32, "Standard_D8_v5"),
            Self::StandardD16V5 => serializer.serialize_unit_variant("AzureVmSize", 268u32, "Standard_D16_v5"),
            Self::StandardD32V5 => serializer.serialize_unit_variant("AzureVmSize", 269u32, "Standard_D32_v5"),
            Self::StandardD48V5 => serializer.serialize_unit_variant("AzureVmSize", 270u32, "Standard_D48_v5"),
            Self::StandardD64V5 => serializer.serialize_unit_variant("AzureVmSize", 271u32, "Standard_D64_v5"),
            Self::StandardD96V5 => serializer.serialize_unit_variant("AzureVmSize", 272u32, "Standard_D96_v5"),
            Self::StandardD2sV5 => serializer.serialize_unit_variant("AzureVmSize", 273u32, "Standard_D2s_v5"),
            Self::StandardD4sV5 => serializer.serialize_unit_variant("AzureVmSize", 274u32, "Standard_D4s_v5"),
            Self::StandardD8sV5 => serializer.serialize_unit_variant("AzureVmSize", 275u32, "Standard_D8s_v5"),
            Self::StandardD16sV5 => serializer.serialize_unit_variant("AzureVmSize", 276u32, "Standard_D16s_v5"),
            Self::StandardD32sV5 => serializer.serialize_unit_variant("AzureVmSize", 277u32, "Standard_D32s_v5"),
            Self::StandardD48sV5 => serializer.serialize_unit_variant("AzureVmSize", 278u32, "Standard_D48s_v5"),
            Self::StandardD64sV5 => serializer.serialize_unit_variant("AzureVmSize", 279u32, "Standard_D64s_v5"),
            Self::StandardD96sV5 => serializer.serialize_unit_variant("AzureVmSize", 280u32, "Standard_D96s_v5"),
            Self::StandardD2dV5 => serializer.serialize_unit_variant("AzureVmSize", 281u32, "Standard_D2d_v5"),
            Self::StandardD4dV5 => serializer.serialize_unit_variant("AzureVmSize", 282u32, "Standard_D4d_v5"),
            Self::StandardD8dV5 => serializer.serialize_unit_variant("AzureVmSize", 283u32, "Standard_D8d_v5"),
            Self::StandardD16dV5 => serializer.serialize_unit_variant("AzureVmSize", 284u32, "Standard_D16d_v5"),
            Self::StandardD32dV5 => serializer.serialize_unit_variant("AzureVmSize", 285u32, "Standard_D32d_v5"),
            Self::StandardD48dV5 => serializer.serialize_unit_variant("AzureVmSize", 286u32, "Standard_D48d_v5"),
            Self::StandardD64dV5 => serializer.serialize_unit_variant("AzureVmSize", 287u32, "Standard_D64d_v5"),
            Self::StandardD96dV5 => serializer.serialize_unit_variant("AzureVmSize", 288u32, "Standard_D96d_v5"),
            Self::StandardD2dsV5 => serializer.serialize_unit_variant("AzureVmSize", 289u32, "Standard_D2ds_v5"),
            Self::StandardD4dsV5 => serializer.serialize_unit_variant("AzureVmSize", 290u32, "Standard_D4ds_v5"),
            Self::StandardD8dsV5 => serializer.serialize_unit_variant("AzureVmSize", 291u32, "Standard_D8ds_v5"),
            Self::StandardD16dsV5 => serializer.serialize_unit_variant("AzureVmSize", 292u32, "Standard_D16ds_v5"),
            Self::StandardD32dsV5 => serializer.serialize_unit_variant("AzureVmSize", 293u32, "Standard_D32ds_v5"),
            Self::StandardD48dsV5 => serializer.serialize_unit_variant("AzureVmSize", 294u32, "Standard_D48ds_v5"),
            Self::StandardD64dsV5 => serializer.serialize_unit_variant("AzureVmSize", 295u32, "Standard_D64ds_v5"),
            Self::StandardD96dsV5 => serializer.serialize_unit_variant("AzureVmSize", 296u32, "Standard_D96ds_v5"),
            Self::StandardD2asV5 => serializer.serialize_unit_variant("AzureVmSize", 297u32, "Standard_D2as_v5"),
            Self::StandardD4asV5 => serializer.serialize_unit_variant("AzureVmSize", 298u32, "Standard_D4as_v5"),
            Self::StandardD8asV5 => serializer.serialize_unit_variant("AzureVmSize", 299u32, "Standard_D8as_v5"),
            Self::StandardD16asV5 => serializer.serialize_unit_variant("AzureVmSize", 300u32, "Standard_D16as_v5"),
            Self::StandardD32asV5 => serializer.serialize_unit_variant("AzureVmSize", 301u32, "Standard_D32as_v5"),
            Self::StandardD48asV5 => serializer.serialize_unit_variant("AzureVmSize", 302u32, "Standard_D48as_v5"),
            Self::StandardD64asV5 => serializer.serialize_unit_variant("AzureVmSize", 303u32, "Standard_D64as_v5"),
            Self::StandardD96asV5 => serializer.serialize_unit_variant("AzureVmSize", 304u32, "Standard_D96as_v5"),
            Self::StandardD2adsV5 => serializer.serialize_unit_variant("AzureVmSize", 305u32, "Standard_D2ads_v5"),
            Self::StandardD4adsV5 => serializer.serialize_unit_variant("AzureVmSize", 306u32, "Standard_D4ads_v5"),
            Self::StandardD8adsV5 => serializer.serialize_unit_variant("AzureVmSize", 307u32, "Standard_D8ads_v5"),
            Self::StandardD16adsV5 => serializer.serialize_unit_variant("AzureVmSize", 308u32, "Standard_D16ads_v5"),
            Self::StandardD32adsV5 => serializer.serialize_unit_variant("AzureVmSize", 309u32, "Standard_D32ads_v5"),
            Self::StandardD48adsV5 => serializer.serialize_unit_variant("AzureVmSize", 310u32, "Standard_D48ads_v5"),
            Self::StandardD64adsV5 => serializer.serialize_unit_variant("AzureVmSize", 311u32, "Standard_D64ads_v5"),
            Self::StandardD96adsV5 => serializer.serialize_unit_variant("AzureVmSize", 312u32, "Standard_D96ads_v5"),
            Self::StandardE2V5 => serializer.serialize_unit_variant("AzureVmSize", 313u32, "Standard_E2_v5"),
            Self::StandardE4V5 => serializer.serialize_unit_variant("AzureVmSize", 314u32, "Standard_E4_v5"),
            Self::StandardE8V5 => serializer.serialize_unit_variant("AzureVmSize", 315u32, "Standard_E8_v5"),
            Self::StandardE16V5 => serializer.serialize_unit_variant("AzureVmSize", 316u32, "Standard_E16_v5"),
            Self::StandardE20V5 => serializer.serialize_unit_variant("AzureVmSize", 317u32, "Standard_E20_v5"),
            Self::StandardE32V5 => serializer.serialize_unit_variant("AzureVmSize", 318u32, "Standard_E32_v5"),
            Self::StandardE48V5 => serializer.serialize_unit_variant("AzureVmSize", 319u32, "Standard_E48_v5"),
            Self::StandardE64V5 => serializer.serialize_unit_variant("AzureVmSize", 320u32, "Standard_E64_v5"),
            Self::StandardE96V5 => serializer.serialize_unit_variant("AzureVmSize", 321u32, "Standard_E96_v5"),
            Self::StandardE104iV5 => serializer.serialize_unit_variant("AzureVmSize", 322u32, "Standard_E104i_v5"),
            Self::StandardE2sV5 => serializer.serialize_unit_variant("AzureVmSize", 323u32, "Standard_E2s_v5"),
            Self::StandardE4sV5 => serializer.serialize_unit_variant("AzureVmSize", 324u32, "Standard_E4s_v5"),
            Self::StandardE8sV5 => serializer.serialize_unit_variant("AzureVmSize", 325u32, "Standard_E8s_v5"),
            Self::StandardE16sV5 => serializer.serialize_unit_variant("AzureVmSize", 326u32, "Standard_E16s_v5"),
            Self::StandardE20sV5 => serializer.serialize_unit_variant("AzureVmSize", 327u32, "Standard_E20s_v5"),
            Self::StandardE32sV5 => serializer.serialize_unit_variant("AzureVmSize", 328u32, "Standard_E32s_v5"),
            Self::StandardE48sV5 => serializer.serialize_unit_variant("AzureVmSize", 329u32, "Standard_E48s_v5"),
            Self::StandardE64sV5 => serializer.serialize_unit_variant("AzureVmSize", 330u32, "Standard_E64s_v5"),
            Self::StandardE96sV5 => serializer.serialize_unit_variant("AzureVmSize", 331u32, "Standard_E96s_v5"),
            Self::StandardE104isV5 => serializer.serialize_unit_variant("AzureVmSize", 332u32, "Standard_E104is_v5"),
            Self::StandardE2dV5 => serializer.serialize_unit_variant("AzureVmSize", 333u32, "Standard_E2d_v5"),
            Self::StandardE4dV5 => serializer.serialize_unit_variant("AzureVmSize", 334u32, "Standard_E4d_v5"),
            Self::StandardE8dV5 => serializer.serialize_unit_variant("AzureVmSize", 335u32, "Standard_E8d_v5"),
            Self::StandardE16dV5 => serializer.serialize_unit_variant("AzureVmSize", 336u32, "Standard_E16d_v5"),
            Self::StandardE20dV5 => serializer.serialize_unit_variant("AzureVmSize", 337u32, "Standard_E20d_v5"),
            Self::StandardE32dV5 => serializer.serialize_unit_variant("AzureVmSize", 338u32, "Standard_E32d_v5"),
            Self::StandardE48dV5 => serializer.serialize_unit_variant("AzureVmSize", 339u32, "Standard_E48d_v5"),
            Self::StandardE64dV5 => serializer.serialize_unit_variant("AzureVmSize", 340u32, "Standard_E64d_v5"),
            Self::StandardE96dV5 => serializer.serialize_unit_variant("AzureVmSize", 341u32, "Standard_E96d_v5"),
            Self::StandardE104idV5 => serializer.serialize_unit_variant("AzureVmSize", 342u32, "Standard_E104id_v5"),
            Self::StandardE2dsV5 => serializer.serialize_unit_variant("AzureVmSize", 343u32, "Standard_E2ds_v5"),
            Self::StandardE4dsV5 => serializer.serialize_unit_variant("AzureVmSize", 344u32, "Standard_E4ds_v5"),
            Self::StandardE8dsV5 => serializer.serialize_unit_variant("AzureVmSize", 345u32, "Standard_E8ds_v5"),
            Self::StandardE16dsV5 => serializer.serialize_unit_variant("AzureVmSize", 346u32, "Standard_E16ds_v5"),
            Self::StandardE20dsV5 => serializer.serialize_unit_variant("AzureVmSize", 347u32, "Standard_E20ds_v5"),
            Self::StandardE32dsV5 => serializer.serialize_unit_variant("AzureVmSize", 348u32, "Standard_E32ds_v5"),
            Self::StandardE48dsV5 => serializer.serialize_unit_variant("AzureVmSize", 349u32, "Standard_E48ds_v5"),
            Self::StandardE64dsV5 => serializer.serialize_unit_variant("AzureVmSize", 350u32, "Standard_E64ds_v5"),
            Self::StandardE96dsV5 => serializer.serialize_unit_variant("AzureVmSize", 351u32, "Standard_E96ds_v5"),
            Self::StandardE104idsV5 => serializer.serialize_unit_variant("AzureVmSize", 352u32, "Standard_E104ids_v5"),
            Self::StandardE2asV5 => serializer.serialize_unit_variant("AzureVmSize", 353u32, "Standard_E2as_v5"),
            Self::StandardE4asV5 => serializer.serialize_unit_variant("AzureVmSize", 354u32, "Standard_E4as_v5"),
            Self::StandardE8asV5 => serializer.serialize_unit_variant("AzureVmSize", 355u32, "Standard_E8as_v5"),
            Self::StandardE16asV5 => serializer.serialize_unit_variant("AzureVmSize", 356u32, "Standard_E16as_v5"),
            Self::StandardE20asV5 => serializer.serialize_unit_variant("AzureVmSize", 357u32, "Standard_E20as_v5"),
            Self::StandardE32asV5 => serializer.serialize_unit_variant("AzureVmSize", 358u32, "Standard_E32as_v5"),
            Self::StandardE48asV5 => serializer.serialize_unit_variant("AzureVmSize", 359u32, "Standard_E48as_v5"),
            Self::StandardE64asV5 => serializer.serialize_unit_variant("AzureVmSize", 360u32, "Standard_E64as_v5"),
            Self::StandardE96asV5 => serializer.serialize_unit_variant("AzureVmSize", 361u32, "Standard_E96as_v5"),
            Self::StandardE2adsV5 => serializer.serialize_unit_variant("AzureVmSize", 362u32, "Standard_E2ads_v5"),
            Self::StandardE4adsV5 => serializer.serialize_unit_variant("AzureVmSize", 363u32, "Standard_E4ads_v5"),
            Self::StandardE8adsV5 => serializer.serialize_unit_variant("AzureVmSize", 364u32, "Standard_E8ads_v5"),
            Self::StandardE16adsV5 => serializer.serialize_unit_variant("AzureVmSize", 365u32, "Standard_E16ads_v5"),
            Self::StandardE20adsV5 => serializer.serialize_unit_variant("AzureVmSize", 366u32, "Standard_E20ads_v5"),
            Self::StandardE32adsV5 => serializer.serialize_unit_variant("AzureVmSize", 367u32, "Standard_E32ads_v5"),
            Self::StandardE48adsV5 => serializer.serialize_unit_variant("AzureVmSize", 368u32, "Standard_E48ads_v5"),
            Self::StandardE64adsV5 => serializer.serialize_unit_variant("AzureVmSize", 369u32, "Standard_E64ads_v5"),
            Self::StandardE96adsV5 => serializer.serialize_unit_variant("AzureVmSize", 370u32, "Standard_E96ads_v5"),
            Self::StandardM82ms => serializer.serialize_unit_variant("AzureVmSize", 371u32, "Standard_M8_2ms"),
            Self::StandardM84ms => serializer.serialize_unit_variant("AzureVmSize", 372u32, "Standard_M8_4ms"),
            Self::StandardM164ms => serializer.serialize_unit_variant("AzureVmSize", 373u32, "Standard_M16_4ms"),
            Self::StandardM168ms => serializer.serialize_unit_variant("AzureVmSize", 374u32, "Standard_M16_8ms"),
            Self::StandardM328ms => serializer.serialize_unit_variant("AzureVmSize", 375u32, "Standard_M32_8ms"),
            Self::StandardM3216ms => serializer.serialize_unit_variant("AzureVmSize", 376u32, "Standard_M32_16ms"),
            Self::StandardM6432ms => serializer.serialize_unit_variant("AzureVmSize", 377u32, "Standard_M64_32ms"),
            Self::StandardM6416ms => serializer.serialize_unit_variant("AzureVmSize", 378u32, "Standard_M64_16ms"),
            Self::StandardM12864ms => serializer.serialize_unit_variant("AzureVmSize", 379u32, "Standard_M128_64ms"),
            Self::StandardM12832ms => serializer.serialize_unit_variant("AzureVmSize", 380u32, "Standard_M128_32ms"),
            Self::StandardE42sV3 => serializer.serialize_unit_variant("AzureVmSize", 381u32, "Standard_E4_2s_v3"),
            Self::StandardE84sV3 => serializer.serialize_unit_variant("AzureVmSize", 382u32, "Standard_E8_4s_v3"),
            Self::StandardE82sV3 => serializer.serialize_unit_variant("AzureVmSize", 383u32, "Standard_E8_2s_v3"),
            Self::StandardE168sV3 => serializer.serialize_unit_variant("AzureVmSize", 384u32, "Standard_E16_8s_v3"),
            Self::StandardE164sV3 => serializer.serialize_unit_variant("AzureVmSize", 385u32, "Standard_E16_4s_v3"),
            Self::StandardE3216sV3 => serializer.serialize_unit_variant("AzureVmSize", 386u32, "Standard_E32_16s_v3"),
            Self::StandardE328sV3 => serializer.serialize_unit_variant("AzureVmSize", 387u32, "Standard_E32_8s_v3"),
            Self::StandardE6432sV3 => serializer.serialize_unit_variant("AzureVmSize", 388u32, "Standard_E64_32s_v3"),
            Self::StandardE6416sV3 => serializer.serialize_unit_variant("AzureVmSize", 389u32, "Standard_E64_16s_v3"),
            Self::StandardE42sV4 => serializer.serialize_unit_variant("AzureVmSize", 390u32, "Standard_E4_2s_v4"),
            Self::StandardE84sV4 => serializer.serialize_unit_variant("AzureVmSize", 391u32, "Standard_E8_4s_v4"),
            Self::StandardE82sV4 => serializer.serialize_unit_variant("AzureVmSize", 392u32, "Standard_E8_2s_v4"),
            Self::StandardE168sV4 => serializer.serialize_unit_variant("AzureVmSize", 393u32, "Standard_E16_8s_v4"),
            Self::StandardE164sV4 => serializer.serialize_unit_variant("AzureVmSize", 394u32, "Standard_E16_4s_v4"),
            Self::StandardE3216sV4 => serializer.serialize_unit_variant("AzureVmSize", 395u32, "Standard_E32_16s_v4"),
            Self::StandardE328sV4 => serializer.serialize_unit_variant("AzureVmSize", 396u32, "Standard_E32_8s_v4"),
            Self::StandardE6432sV4 => serializer.serialize_unit_variant("AzureVmSize", 397u32, "Standard_E64_32s_v4"),
            Self::StandardE6416sV4 => serializer.serialize_unit_variant("AzureVmSize", 398u32, "Standard_E64_16s_v4"),
            Self::StandardE42dsV4 => serializer.serialize_unit_variant("AzureVmSize", 399u32, "Standard_E4_2ds_v4"),
            Self::StandardE84dsV4 => serializer.serialize_unit_variant("AzureVmSize", 400u32, "Standard_E8_4ds_v4"),
            Self::StandardE82dsV4 => serializer.serialize_unit_variant("AzureVmSize", 401u32, "Standard_E8_2ds_v4"),
            Self::StandardE168dsV4 => serializer.serialize_unit_variant("AzureVmSize", 402u32, "Standard_E16_8ds_v4"),
            Self::StandardE164dsV4 => serializer.serialize_unit_variant("AzureVmSize", 403u32, "Standard_E16_4ds_v4"),
            Self::StandardE3216dsV4 => serializer.serialize_unit_variant("AzureVmSize", 404u32, "Standard_E32_16ds_v4"),
            Self::StandardE328dsV4 => serializer.serialize_unit_variant("AzureVmSize", 405u32, "Standard_E32_8ds_v4"),
            Self::StandardE6432dsV4 => serializer.serialize_unit_variant("AzureVmSize", 406u32, "Standard_E64_32ds_v4"),
            Self::StandardE6416dsV4 => serializer.serialize_unit_variant("AzureVmSize", 407u32, "Standard_E64_16ds_v4"),
            Self::StandardE42asV4 => serializer.serialize_unit_variant("AzureVmSize", 408u32, "Standard_E4_2as_v4"),
            Self::StandardE84asV4 => serializer.serialize_unit_variant("AzureVmSize", 409u32, "Standard_E8_4as_v4"),
            Self::StandardE82asV4 => serializer.serialize_unit_variant("AzureVmSize", 410u32, "Standard_E8_2as_v4"),
            Self::StandardE168asV4 => serializer.serialize_unit_variant("AzureVmSize", 411u32, "Standard_E16_8as_v4"),
            Self::StandardE164asV4 => serializer.serialize_unit_variant("AzureVmSize", 412u32, "Standard_E16_4as_v4"),
            Self::StandardE3216asV4 => serializer.serialize_unit_variant("AzureVmSize", 413u32, "Standard_E32_16as_v4"),
            Self::StandardE328asV4 => serializer.serialize_unit_variant("AzureVmSize", 414u32, "Standard_E32_8as_v4"),
            Self::StandardE6432asV4 => serializer.serialize_unit_variant("AzureVmSize", 415u32, "Standard_E64_32as_v4"),
            Self::StandardE6416asV4 => serializer.serialize_unit_variant("AzureVmSize", 416u32, "Standard_E64_16as_v4"),
            Self::StandardE9648asV4 => serializer.serialize_unit_variant("AzureVmSize", 417u32, "Standard_E96_48as_v4"),
            Self::StandardE9624asV4 => serializer.serialize_unit_variant("AzureVmSize", 418u32, "Standard_E96_24as_v4"),
            Self::StandardE42adsV5 => serializer.serialize_unit_variant("AzureVmSize", 419u32, "Standard_E4_2ads_v5"),
            Self::StandardE84adsV5 => serializer.serialize_unit_variant("AzureVmSize", 420u32, "Standard_E8_4ads_v5"),
            Self::StandardE82adsV5 => serializer.serialize_unit_variant("AzureVmSize", 421u32, "Standard_E8_2ads_v5"),
            Self::StandardE168adsV5 => serializer.serialize_unit_variant("AzureVmSize", 422u32, "Standard_E16_8ads_v5"),
            Self::StandardE164adsV5 => serializer.serialize_unit_variant("AzureVmSize", 423u32, "Standard_E16_4ads_v5"),
            Self::StandardE3216adsV5 => serializer.serialize_unit_variant("AzureVmSize", 424u32, "Standard_E32_16ads_v5"),
            Self::StandardE328adsV5 => serializer.serialize_unit_variant("AzureVmSize", 425u32, "Standard_E32_8ads_v5"),
            Self::StandardE6432adsV5 => serializer.serialize_unit_variant("AzureVmSize", 426u32, "Standard_E64_32ads_v5"),
            Self::StandardE6416adsV5 => serializer.serialize_unit_variant("AzureVmSize", 427u32, "Standard_E64_16ads_v5"),
            Self::StandardE9648adsV5 => serializer.serialize_unit_variant("AzureVmSize", 428u32, "Standard_E96_48ads_v5"),
            Self::StandardE9624adsV5 => serializer.serialize_unit_variant("AzureVmSize", 429u32, "Standard_E96_24ads_v5"),
            Self::StandardE42sV5 => serializer.serialize_unit_variant("AzureVmSize", 430u32, "Standard_E4_2s_v5"),
            Self::StandardE84sV5 => serializer.serialize_unit_variant("AzureVmSize", 431u32, "Standard_E8_4s_v5"),
            Self::StandardE82sV5 => serializer.serialize_unit_variant("AzureVmSize", 432u32, "Standard_E8_2s_v5"),
            Self::StandardE168sV5 => serializer.serialize_unit_variant("AzureVmSize", 433u32, "Standard_E16_8s_v5"),
            Self::StandardE164sV5 => serializer.serialize_unit_variant("AzureVmSize", 434u32, "Standard_E16_4s_v5"),
            Self::StandardE3216sV5 => serializer.serialize_unit_variant("AzureVmSize", 435u32, "Standard_E32_16s_v5"),
            Self::StandardE328sV5 => serializer.serialize_unit_variant("AzureVmSize", 436u32, "Standard_E32_8s_v5"),
            Self::StandardE6432sV5 => serializer.serialize_unit_variant("AzureVmSize", 437u32, "Standard_E64_32s_v5"),
            Self::StandardE6416sV5 => serializer.serialize_unit_variant("AzureVmSize", 438u32, "Standard_E64_16s_v5"),
            Self::StandardE9648sV5 => serializer.serialize_unit_variant("AzureVmSize", 439u32, "Standard_E96_48s_v5"),
            Self::StandardE9624sV5 => serializer.serialize_unit_variant("AzureVmSize", 440u32, "Standard_E96_24s_v5"),
            Self::StandardE42dsV5 => serializer.serialize_unit_variant("AzureVmSize", 441u32, "Standard_E4_2ds_v5"),
            Self::StandardE84dsV5 => serializer.serialize_unit_variant("AzureVmSize", 442u32, "Standard_E8_4ds_v5"),
            Self::StandardE82dsV5 => serializer.serialize_unit_variant("AzureVmSize", 443u32, "Standard_E8_2ds_v5"),
            Self::StandardE168dsV5 => serializer.serialize_unit_variant("AzureVmSize", 444u32, "Standard_E16_8ds_v5"),
            Self::StandardE164dsV5 => serializer.serialize_unit_variant("AzureVmSize", 445u32, "Standard_E16_4ds_v5"),
            Self::StandardE3216dsV5 => serializer.serialize_unit_variant("AzureVmSize", 446u32, "Standard_E32_16ds_v5"),
            Self::StandardE328dsV5 => serializer.serialize_unit_variant("AzureVmSize", 447u32, "Standard_E32_8ds_v5"),
            Self::StandardE6432dsV5 => serializer.serialize_unit_variant("AzureVmSize", 448u32, "Standard_E64_32ds_v5"),
            Self::StandardE6416dsV5 => serializer.serialize_unit_variant("AzureVmSize", 449u32, "Standard_E64_16ds_v5"),
            Self::StandardE9648dsV5 => serializer.serialize_unit_variant("AzureVmSize", 450u32, "Standard_E96_48ds_v5"),
            Self::StandardE9624dsV5 => serializer.serialize_unit_variant("AzureVmSize", 451u32, "Standard_E96_24ds_v5"),
            Self::StandardE42asV5 => serializer.serialize_unit_variant("AzureVmSize", 452u32, "Standard_E4_2as_v5"),
            Self::StandardE84asV5 => serializer.serialize_unit_variant("AzureVmSize", 453u32, "Standard_E8_4as_v5"),
            Self::StandardE82asV5 => serializer.serialize_unit_variant("AzureVmSize", 454u32, "Standard_E8_2as_v5"),
            Self::StandardE168asV5 => serializer.serialize_unit_variant("AzureVmSize", 455u32, "Standard_E16_8as_v5"),
            Self::StandardE164asV5 => serializer.serialize_unit_variant("AzureVmSize", 456u32, "Standard_E16_4as_v5"),
            Self::StandardE3216asV5 => serializer.serialize_unit_variant("AzureVmSize", 457u32, "Standard_E32_16as_v5"),
            Self::StandardE328asV5 => serializer.serialize_unit_variant("AzureVmSize", 458u32, "Standard_E32_8as_v5"),
            Self::StandardE6432asV5 => serializer.serialize_unit_variant("AzureVmSize", 459u32, "Standard_E64_32as_v5"),
            Self::StandardE6416asV5 => serializer.serialize_unit_variant("AzureVmSize", 460u32, "Standard_E64_16as_v5"),
            Self::StandardE9648asV5 => serializer.serialize_unit_variant("AzureVmSize", 461u32, "Standard_E96_48as_v5"),
            Self::StandardE9624asV5 => serializer.serialize_unit_variant("AzureVmSize", 462u32, "Standard_E96_24as_v5"),
            Self::StandardGs48 => serializer.serialize_unit_variant("AzureVmSize", 463u32, "Standard_GS4_8"),
            Self::StandardGs44 => serializer.serialize_unit_variant("AzureVmSize", 464u32, "Standard_GS4_4"),
            Self::StandardGs516 => serializer.serialize_unit_variant("AzureVmSize", 465u32, "Standard_GS5_16"),
            Self::StandardGs58 => serializer.serialize_unit_variant("AzureVmSize", 466u32, "Standard_GS5_8"),
            Self::StandardDs111V2 => serializer.serialize_unit_variant("AzureVmSize", 467u32, "Standard_DS11_1_v2"),
            Self::StandardDs122V2 => serializer.serialize_unit_variant("AzureVmSize", 468u32, "Standard_DS12_2_v2"),
            Self::StandardDs121V2 => serializer.serialize_unit_variant("AzureVmSize", 469u32, "Standard_DS12_1_v2"),
            Self::StandardDs134V2 => serializer.serialize_unit_variant("AzureVmSize", 470u32, "Standard_DS13_4_v2"),
            Self::StandardDs132V2 => serializer.serialize_unit_variant("AzureVmSize", 471u32, "Standard_DS13_2_v2"),
            Self::StandardDs148V2 => serializer.serialize_unit_variant("AzureVmSize", 472u32, "Standard_DS14_8_v2"),
            Self::StandardDs144V2 => serializer.serialize_unit_variant("AzureVmSize", 473u32, "Standard_DS14_4_v2"),
            Self::StandardM416208sV2 => serializer.serialize_unit_variant("AzureVmSize", 474u32, "Standard_M416_208s_v2"),
            Self::StandardM416208msV2 => serializer.serialize_unit_variant("AzureVmSize", 475u32, "Standard_M416_208ms_v2"),
            Self::StandardE2bsV5 => serializer.serialize_unit_variant("AzureVmSize", 476u32, "Standard_E2bs_v5"),
            Self::StandardE4bsV5 => serializer.serialize_unit_variant("AzureVmSize", 477u32, "Standard_E4bs_v5"),
            Self::StandardE8bsV5 => serializer.serialize_unit_variant("AzureVmSize", 478u32, "Standard_E8bs_v5"),
            Self::StandardE16bsV5 => serializer.serialize_unit_variant("AzureVmSize", 479u32, "Standard_E16bs_v5"),
            Self::StandardE32bsV5 => serializer.serialize_unit_variant("AzureVmSize", 480u32, "Standard_E32bs_v5"),
            Self::StandardE48bsV5 => serializer.serialize_unit_variant("AzureVmSize", 481u32, "Standard_E48bs_v5"),
            Self::StandardE64bsV5 => serializer.serialize_unit_variant("AzureVmSize", 482u32, "Standard_E64bs_v5"),
            Self::StandardE2bdsV5 => serializer.serialize_unit_variant("AzureVmSize", 483u32, "Standard_E2bds_v5"),
            Self::StandardE4bdsV5 => serializer.serialize_unit_variant("AzureVmSize", 484u32, "Standard_E4bds_v5"),
            Self::StandardE8bdsV5 => serializer.serialize_unit_variant("AzureVmSize", 485u32, "Standard_E8bds_v5"),
            Self::StandardE16bdsV5 => serializer.serialize_unit_variant("AzureVmSize", 486u32, "Standard_E16bds_v5"),
            Self::StandardE32bdsV5 => serializer.serialize_unit_variant("AzureVmSize", 487u32, "Standard_E32bds_v5"),
            Self::StandardE48bdsV5 => serializer.serialize_unit_variant("AzureVmSize", 488u32, "Standard_E48bds_v5"),
            Self::StandardE64bdsV5 => serializer.serialize_unit_variant("AzureVmSize", 489u32, "Standard_E64bds_v5"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureVmSuitabilityDetail")]
pub enum AzureVmSuitabilityDetail {
    None,
    RecommendedSizeHasLessNetworkAdapters,
    CannotReportComputeCost,
    CannotReportStorageCost,
    CannotReportBandwidthCosts,
    PercentageOfCoresUtilizedMissing,
    PercentageOfMemoryUtilizedMissing,
    PercentageOfCoresUtilizedOutOfRange,
    PercentageOfMemoryUtilizedOutOfRange,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureVmSuitabilityDetail {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureVmSuitabilityDetail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureVmSuitabilityDetail {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("AzureVmSuitabilityDetail", 0u32, "None"),
            Self::RecommendedSizeHasLessNetworkAdapters => {
                serializer.serialize_unit_variant("AzureVmSuitabilityDetail", 1u32, "RecommendedSizeHasLessNetworkAdapters")
            }
            Self::CannotReportComputeCost => serializer.serialize_unit_variant("AzureVmSuitabilityDetail", 2u32, "CannotReportComputeCost"),
            Self::CannotReportStorageCost => serializer.serialize_unit_variant("AzureVmSuitabilityDetail", 3u32, "CannotReportStorageCost"),
            Self::CannotReportBandwidthCosts => {
                serializer.serialize_unit_variant("AzureVmSuitabilityDetail", 4u32, "CannotReportBandwidthCosts")
            }
            Self::PercentageOfCoresUtilizedMissing => {
                serializer.serialize_unit_variant("AzureVmSuitabilityDetail", 5u32, "PercentageOfCoresUtilizedMissing")
            }
            Self::PercentageOfMemoryUtilizedMissing => {
                serializer.serialize_unit_variant("AzureVmSuitabilityDetail", 6u32, "PercentageOfMemoryUtilizedMissing")
            }
            Self::PercentageOfCoresUtilizedOutOfRange => {
                serializer.serialize_unit_variant("AzureVmSuitabilityDetail", 7u32, "PercentageOfCoresUtilizedOutOfRange")
            }
            Self::PercentageOfMemoryUtilizedOutOfRange => {
                serializer.serialize_unit_variant("AzureVmSuitabilityDetail", 8u32, "PercentageOfMemoryUtilizedOutOfRange")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureVmSuitabilityExplanation")]
pub enum AzureVmSuitabilityExplanation {
    Unknown,
    NotApplicable,
    GuestOperatingSystemArchitectureNotSupported,
    GuestOperatingSystemNotSupported,
    BootTypeNotSupported,
    MoreDisksThanSupported,
    NoSuitableVmSizeFound,
    OneOrMoreDisksNotSuitable,
    OneOrMoreAdaptersNotSuitable,
    InternalErrorOccurredDuringComputeEvaluation,
    InternalErrorOccurredDuringStorageEvaluation,
    InternalErrorOccurredDuringNetworkEvaluation,
    NoVmSizeSupportsStoragePerformance,
    NoVmSizeSupportsNetworkPerformance,
    NoVmSizeForSelectedPricingTier,
    NoVmSizeForSelectedAzureLocation,
    CheckRedHatLinuxVersion,
    CheckOpenSuseLinuxVersion,
    CheckWindowsServer2008R2Version,
    CheckCentOsVersion,
    CheckDebianLinuxVersion,
    CheckSuseLinuxVersion,
    CheckOracleLinuxVersion,
    CheckUbuntuLinuxVersion,
    CheckCoreOsLinuxVersion,
    WindowsServerVersionConditionallySupported,
    NoGuestOperatingSystemConditionallySupported,
    WindowsClientVersionsConditionallySupported,
    BootTypeUnknown,
    GuestOperatingSystemUnknown,
    WindowsServerVersionsSupportedWithCaveat,
    #[serde(rename = "WindowsOSNoLongerUnderMSSupport")]
    WindowsOsNoLongerUnderMsSupport,
    EndorsedWithConditionsLinuxDistributions,
    UnendorsedLinuxDistributions,
    NoVmSizeForStandardPricingTier,
    NoVmSizeForBasicPricingTier,
    NoVmSizeInSelectedFamilyFound,
    NoEaPriceFoundForVmSize,
    NoVmSizeFoundForOfferCurrencyReservedInstance,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureVmSuitabilityExplanation {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureVmSuitabilityExplanation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureVmSuitabilityExplanation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 0u32, "Unknown"),
            Self::NotApplicable => serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 1u32, "NotApplicable"),
            Self::GuestOperatingSystemArchitectureNotSupported => serializer.serialize_unit_variant(
                "AzureVmSuitabilityExplanation",
                2u32,
                "GuestOperatingSystemArchitectureNotSupported",
            ),
            Self::GuestOperatingSystemNotSupported => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 3u32, "GuestOperatingSystemNotSupported")
            }
            Self::BootTypeNotSupported => serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 4u32, "BootTypeNotSupported"),
            Self::MoreDisksThanSupported => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 5u32, "MoreDisksThanSupported")
            }
            Self::NoSuitableVmSizeFound => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 6u32, "NoSuitableVmSizeFound")
            }
            Self::OneOrMoreDisksNotSuitable => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 7u32, "OneOrMoreDisksNotSuitable")
            }
            Self::OneOrMoreAdaptersNotSuitable => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 8u32, "OneOrMoreAdaptersNotSuitable")
            }
            Self::InternalErrorOccurredDuringComputeEvaluation => serializer.serialize_unit_variant(
                "AzureVmSuitabilityExplanation",
                9u32,
                "InternalErrorOccurredDuringComputeEvaluation",
            ),
            Self::InternalErrorOccurredDuringStorageEvaluation => serializer.serialize_unit_variant(
                "AzureVmSuitabilityExplanation",
                10u32,
                "InternalErrorOccurredDuringStorageEvaluation",
            ),
            Self::InternalErrorOccurredDuringNetworkEvaluation => serializer.serialize_unit_variant(
                "AzureVmSuitabilityExplanation",
                11u32,
                "InternalErrorOccurredDuringNetworkEvaluation",
            ),
            Self::NoVmSizeSupportsStoragePerformance => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 12u32, "NoVmSizeSupportsStoragePerformance")
            }
            Self::NoVmSizeSupportsNetworkPerformance => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 13u32, "NoVmSizeSupportsNetworkPerformance")
            }
            Self::NoVmSizeForSelectedPricingTier => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 14u32, "NoVmSizeForSelectedPricingTier")
            }
            Self::NoVmSizeForSelectedAzureLocation => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 15u32, "NoVmSizeForSelectedAzureLocation")
            }
            Self::CheckRedHatLinuxVersion => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 16u32, "CheckRedHatLinuxVersion")
            }
            Self::CheckOpenSuseLinuxVersion => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 17u32, "CheckOpenSuseLinuxVersion")
            }
            Self::CheckWindowsServer2008R2Version => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 18u32, "CheckWindowsServer2008R2Version")
            }
            Self::CheckCentOsVersion => serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 19u32, "CheckCentOsVersion"),
            Self::CheckDebianLinuxVersion => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 20u32, "CheckDebianLinuxVersion")
            }
            Self::CheckSuseLinuxVersion => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 21u32, "CheckSuseLinuxVersion")
            }
            Self::CheckOracleLinuxVersion => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 22u32, "CheckOracleLinuxVersion")
            }
            Self::CheckUbuntuLinuxVersion => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 23u32, "CheckUbuntuLinuxVersion")
            }
            Self::CheckCoreOsLinuxVersion => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 24u32, "CheckCoreOsLinuxVersion")
            }
            Self::WindowsServerVersionConditionallySupported => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 25u32, "WindowsServerVersionConditionallySupported")
            }
            Self::NoGuestOperatingSystemConditionallySupported => serializer.serialize_unit_variant(
                "AzureVmSuitabilityExplanation",
                26u32,
                "NoGuestOperatingSystemConditionallySupported",
            ),
            Self::WindowsClientVersionsConditionallySupported => serializer.serialize_unit_variant(
                "AzureVmSuitabilityExplanation",
                27u32,
                "WindowsClientVersionsConditionallySupported",
            ),
            Self::BootTypeUnknown => serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 28u32, "BootTypeUnknown"),
            Self::GuestOperatingSystemUnknown => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 29u32, "GuestOperatingSystemUnknown")
            }
            Self::WindowsServerVersionsSupportedWithCaveat => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 30u32, "WindowsServerVersionsSupportedWithCaveat")
            }
            Self::WindowsOsNoLongerUnderMsSupport => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 31u32, "WindowsOSNoLongerUnderMSSupport")
            }
            Self::EndorsedWithConditionsLinuxDistributions => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 32u32, "EndorsedWithConditionsLinuxDistributions")
            }
            Self::UnendorsedLinuxDistributions => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 33u32, "UnendorsedLinuxDistributions")
            }
            Self::NoVmSizeForStandardPricingTier => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 34u32, "NoVmSizeForStandardPricingTier")
            }
            Self::NoVmSizeForBasicPricingTier => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 35u32, "NoVmSizeForBasicPricingTier")
            }
            Self::NoVmSizeInSelectedFamilyFound => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 36u32, "NoVmSizeInSelectedFamilyFound")
            }
            Self::NoEaPriceFoundForVmSize => {
                serializer.serialize_unit_variant("AzureVmSuitabilityExplanation", 37u32, "NoEaPriceFoundForVmSize")
            }
            Self::NoVmSizeFoundForOfferCurrencyReservedInstance => serializer.serialize_unit_variant(
                "AzureVmSuitabilityExplanation",
                38u32,
                "NoVmSizeFoundForOfferCurrencyReservedInstance",
            ),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Cloud Suitability for Azure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CloudSuitability")]
pub enum CloudSuitability {
    Unknown,
    NotSuitable,
    Suitable,
    ConditionallySuitable,
    ReadinessUnknown,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CloudSuitability {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CloudSuitability {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CloudSuitability {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("CloudSuitability", 0u32, "Unknown"),
            Self::NotSuitable => serializer.serialize_unit_variant("CloudSuitability", 1u32, "NotSuitable"),
            Self::Suitable => serializer.serialize_unit_variant("CloudSuitability", 2u32, "Suitable"),
            Self::ConditionallySuitable => serializer.serialize_unit_variant("CloudSuitability", 3u32, "ConditionallySuitable"),
            Self::ReadinessUnknown => serializer.serialize_unit_variant("CloudSuitability", 4u32, "ReadinessUnknown"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Collector agent property class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CollectorAgentPropertiesBase {
    #[doc = "Gets the collector agent id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the collector agent version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets the collector last heartbeat time."]
    #[serde(rename = "lastHeartbeatUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat_utc: Option<time::OffsetDateTime>,
    #[doc = "Collector agent SPN details class."]
    #[serde(rename = "spnDetails", default, skip_serializing_if = "Option::is_none")]
    pub spn_details: Option<CollectorAgentSpnPropertiesBase>,
}
impl CollectorAgentPropertiesBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collector agent SPN details class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CollectorAgentSpnPropertiesBase {
    #[doc = "Gets the AAD authority endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[doc = "Gets the AAD application id."]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[doc = "Gets the AAD audience url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[doc = "Gets the object id of the AAD application."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Gets the tenant id of the AAD application."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl CollectorAgentSpnPropertiesBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collector properties class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CollectorPropertiesBase {
    #[serde(flatten)]
    pub azure_resource_properties: AzureResourceProperties,
    #[doc = "Gets the discovery site id."]
    #[serde(rename = "discoverySiteId", default, skip_serializing_if = "Option::is_none")]
    pub discovery_site_id: Option<String>,
    #[doc = "Gets the Timestamp when collector was created."]
    #[serde(rename = "createdTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Timestamp when collector was last updated."]
    #[serde(rename = "updatedTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
}
impl CollectorPropertiesBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collector properties class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CollectorPropertiesBaseWithAgent {
    #[serde(flatten)]
    pub azure_resource_properties: AzureResourceProperties,
    #[doc = "Collector agent property class."]
    #[serde(rename = "agentProperties", default, skip_serializing_if = "Option::is_none")]
    pub agent_properties: Option<CollectorAgentPropertiesBase>,
    #[doc = "Gets the discovery site id."]
    #[serde(rename = "discoverySiteId", default, skip_serializing_if = "Option::is_none")]
    pub discovery_site_id: Option<String>,
    #[doc = "Gets the Timestamp when collector was created."]
    #[serde(rename = "createdTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Timestamp when collector was last updated."]
    #[serde(rename = "updatedTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
}
impl CollectorPropertiesBaseWithAgent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CompatibilityLevel")]
pub enum CompatibilityLevel {
    Unknown,
    CompatLevel80,
    CompatLevel90,
    CompatLevel100,
    CompatLevel110,
    CompatLevel120,
    CompatLevel130,
    CompatLevel140,
    CompatLevel150,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CompatibilityLevel {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CompatibilityLevel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CompatibilityLevel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("CompatibilityLevel", 0u32, "Unknown"),
            Self::CompatLevel80 => serializer.serialize_unit_variant("CompatibilityLevel", 1u32, "CompatLevel80"),
            Self::CompatLevel90 => serializer.serialize_unit_variant("CompatibilityLevel", 2u32, "CompatLevel90"),
            Self::CompatLevel100 => serializer.serialize_unit_variant("CompatibilityLevel", 3u32, "CompatLevel100"),
            Self::CompatLevel110 => serializer.serialize_unit_variant("CompatibilityLevel", 4u32, "CompatLevel110"),
            Self::CompatLevel120 => serializer.serialize_unit_variant("CompatibilityLevel", 5u32, "CompatLevel120"),
            Self::CompatLevel130 => serializer.serialize_unit_variant("CompatibilityLevel", 6u32, "CompatLevel130"),
            Self::CompatLevel140 => serializer.serialize_unit_variant("CompatibilityLevel", 7u32, "CompatLevel140"),
            Self::CompatLevel150 => serializer.serialize_unit_variant("CompatibilityLevel", 8u32, "CompatLevel150"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ComputeTier")]
pub enum ComputeTier {
    Unknown,
    Automatic,
    Provisioned,
    Serverless,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ComputeTier {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ComputeTier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ComputeTier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ComputeTier", 0u32, "Unknown"),
            Self::Automatic => serializer.serialize_unit_variant("ComputeTier", 1u32, "Automatic"),
            Self::Provisioned => serializer.serialize_unit_variant("ComputeTier", 2u32, "Provisioned"),
            Self::Serverless => serializer.serialize_unit_variant("ComputeTier", 3u32, "Serverless"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Class to represent the component of the cost."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CostComponent {
    #[doc = "Enum to represent component name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CostComponentName>,
    #[doc = "The value of the component."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f32>,
    #[doc = "The textual description of the component."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl CostComponent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enum to represent component name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CostComponentName")]
pub enum CostComponentName {
    Unknown,
    MonthlyAzureHybridCostSavings,
    MonthlySecurityCost,
    MonthlyPremiumV2StorageCost,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CostComponentName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CostComponentName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CostComponentName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("CostComponentName", 0u32, "Unknown"),
            Self::MonthlyAzureHybridCostSavings => {
                serializer.serialize_unit_variant("CostComponentName", 1u32, "MonthlyAzureHybridCostSavings")
            }
            Self::MonthlySecurityCost => serializer.serialize_unit_variant("CostComponentName", 2u32, "MonthlySecurityCost"),
            Self::MonthlyPremiumV2StorageCost => {
                serializer.serialize_unit_variant("CostComponentName", 3u32, "MonthlyPremiumV2StorageCost")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A disk discovered on a machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Disk {
    #[doc = "Gigabytes of storage provisioned for this disk."]
    #[serde(rename = "gigabytesAllocated", default, skip_serializing_if = "Option::is_none")]
    pub gigabytes_allocated: Option<f32>,
    #[doc = "User friendly name of the disk."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl Disk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data model of Download URL for assessment report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DownloadUrl {
    #[doc = "Hyperlink to download report."]
    #[serde(rename = "assessmentReportUrl")]
    pub assessment_report_url: String,
    #[doc = "Expiry date of download url."]
    #[serde(rename = "expirationTime", with = "azure_core::date::rfc3339")]
    pub expiration_time: time::OffsetDateTime,
}
impl DownloadUrl {
    pub fn new(assessment_report_url: String, expiration_time: time::OffsetDateTime) -> Self {
        Self {
            assessment_report_url,
            expiration_time,
        }
    }
}
#[doc = "Entity Uptime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityUptime {
    #[doc = "Gets the days per month."]
    #[serde(rename = "daysPerMonth", default, skip_serializing_if = "Option::is_none")]
    pub days_per_month: Option<i32>,
    #[doc = "Gets the hours per day."]
    #[serde(rename = "hoursPerDay", default, skip_serializing_if = "Option::is_none")]
    pub hours_per_day: Option<i32>,
}
impl EntityUptime {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnvironmentType")]
pub enum EnvironmentType {
    Production,
    Test,
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
            Self::Production => serializer.serialize_unit_variant("EnvironmentType", 0u32, "Production"),
            Self::Test => serializer.serialize_unit_variant("EnvironmentType", 1u32, "Test"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Error web model class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "Gets the error ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Gets the error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Gets the Run as account ID."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "Gets the Appliance name."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
    #[doc = "Gets the error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Gets the error summary message."]
    #[serde(rename = "summaryMessage", default, skip_serializing_if = "Option::is_none")]
    pub summary_message: Option<String>,
    #[doc = "Gets the agent scenario where this error occurred."]
    #[serde(rename = "agentScenario", default, skip_serializing_if = "Option::is_none")]
    pub agent_scenario: Option<String>,
    #[doc = "Gets the error possible causes."]
    #[serde(rename = "possibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub possible_causes: Option<String>,
    #[doc = "Gets the recommended action for the error."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[doc = "Gets the error severity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[doc = "Gets the error message parameters."]
    #[serde(rename = "messageParameters", default, skip_serializing_if = "Option::is_none")]
    pub message_parameters: Option<serde_json::Value>,
    #[doc = "Gets the time stamp when the error was updated."]
    #[serde(rename = "updatedTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub updated_time_stamp: Option<time::OffsetDateTime>,
    #[doc = "Gets the type of assessment impacted by this error."]
    #[serde(rename = "impactedAssessmentType", default, skip_serializing_if = "Option::is_none")]
    pub impacted_assessment_type: Option<String>,
}
impl Error {
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
#[doc = "Error summary containing affected entities for each type of assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorSummary {
    #[serde(rename = "assessmentType", default, skip_serializing_if = "Option::is_none")]
    pub assessment_type: Option<AssessmentType>,
    #[doc = "Gets the affected entity count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
impl ErrorSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FttAndRaidLevel")]
pub enum FttAndRaidLevel {
    Unknown,
    Ftt1Raid1,
    Ftt1Raid5,
    Ftt2Raid1,
    Ftt2Raid6,
    Ftt3Raid1,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FttAndRaidLevel {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FttAndRaidLevel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FttAndRaidLevel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("FttAndRaidLevel", 0u32, "Unknown"),
            Self::Ftt1Raid1 => serializer.serialize_unit_variant("FttAndRaidLevel", 1u32, "Ftt1Raid1"),
            Self::Ftt1Raid5 => serializer.serialize_unit_variant("FttAndRaidLevel", 2u32, "Ftt1Raid5"),
            Self::Ftt2Raid1 => serializer.serialize_unit_variant("FttAndRaidLevel", 3u32, "Ftt2Raid1"),
            Self::Ftt2Raid6 => serializer.serialize_unit_variant("FttAndRaidLevel", 4u32, "Ftt2Raid6"),
            Self::Ftt3Raid1 => serializer.serialize_unit_variant("FttAndRaidLevel", 5u32, "Ftt3Raid1"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Group {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of group resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GroupProperties>,
}
impl Group {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Body properties of group update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupBodyProperties {
    #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<GroupUpdateOperation>,
    #[doc = "List of machine names that are part of this group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machines: Vec<String>,
}
impl GroupBodyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a Group list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupListResult {
    #[doc = "The Group items on this page"]
    pub value: Vec<Group>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GroupListResult {
    pub fn new(value: Vec<Group>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties of group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupProperties {
    #[serde(flatten)]
    pub azure_resource_properties: AzureResourceProperties,
    #[serde(rename = "groupStatus", default, skip_serializing_if = "Option::is_none")]
    pub group_status: Option<GroupStatus>,
    #[doc = "Number of machines part of this group."]
    #[serde(rename = "machineCount", default, skip_serializing_if = "Option::is_none")]
    pub machine_count: Option<i32>,
    #[doc = "List of References to Assessments created on this group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub assessments: Vec<String>,
    #[doc = "List of assessment types supported on this group."]
    #[serde(
        rename = "supportedAssessmentTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_assessment_types: Vec<AssessmentType>,
    #[doc = "If the assessments are in running state."]
    #[serde(rename = "areAssessmentsRunning", default, skip_serializing_if = "Option::is_none")]
    pub are_assessments_running: Option<bool>,
    #[doc = "Time when this group was created. Date-Time represented in ISO-8601 format."]
    #[serde(rename = "createdTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Time when this group was last updated. Date-Time represented in ISO-8601 format."]
    #[serde(rename = "updatedTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
    #[serde(rename = "groupType", default, skip_serializing_if = "Option::is_none")]
    pub group_type: Option<GroupType>,
}
impl GroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "GroupStatus")]
pub enum GroupStatus {
    Created,
    Updated,
    Running,
    Completed,
    Invalid,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for GroupStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for GroupStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for GroupStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Created => serializer.serialize_unit_variant("GroupStatus", 0u32, "Created"),
            Self::Updated => serializer.serialize_unit_variant("GroupStatus", 1u32, "Updated"),
            Self::Running => serializer.serialize_unit_variant("GroupStatus", 2u32, "Running"),
            Self::Completed => serializer.serialize_unit_variant("GroupStatus", 3u32, "Completed"),
            Self::Invalid => serializer.serialize_unit_variant("GroupStatus", 4u32, "Invalid"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "GroupType")]
pub enum GroupType {
    Default,
    Import,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for GroupType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for GroupType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for GroupType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Default => serializer.serialize_unit_variant("GroupType", 0u32, "Default"),
            Self::Import => serializer.serialize_unit_variant("GroupType", 1u32, "Import"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "GroupUpdateOperation")]
pub enum GroupUpdateOperation {
    Add,
    Remove,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for GroupUpdateOperation {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for GroupUpdateOperation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for GroupUpdateOperation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Add => serializer.serialize_unit_variant("GroupUpdateOperation", 0u32, "Add"),
            Self::Remove => serializer.serialize_unit_variant("GroupUpdateOperation", 1u32, "Remove"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "GuestOperatingSystemArchitecture")]
pub enum GuestOperatingSystemArchitecture {
    Unknown,
    X86,
    X64,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for GuestOperatingSystemArchitecture {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for GuestOperatingSystemArchitecture {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for GuestOperatingSystemArchitecture {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("GuestOperatingSystemArchitecture", 0u32, "Unknown"),
            Self::X86 => serializer.serialize_unit_variant("GuestOperatingSystemArchitecture", 1u32, "X86"),
            Self::X64 => serializer.serialize_unit_variant("GuestOperatingSystemArchitecture", 2u32, "X64"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HardwareGeneration")]
pub enum HardwareGeneration {
    Unknown,
    Automatic,
    Gen5,
    #[serde(rename = "Fsv2_series")]
    Fsv2Series,
    #[serde(rename = "M_series")]
    MSeries,
    #[serde(rename = "DC_series")]
    DcSeries,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HardwareGeneration {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HardwareGeneration {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HardwareGeneration {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("HardwareGeneration", 0u32, "Unknown"),
            Self::Automatic => serializer.serialize_unit_variant("HardwareGeneration", 1u32, "Automatic"),
            Self::Gen5 => serializer.serialize_unit_variant("HardwareGeneration", 2u32, "Gen5"),
            Self::Fsv2Series => serializer.serialize_unit_variant("HardwareGeneration", 3u32, "Fsv2_series"),
            Self::MSeries => serializer.serialize_unit_variant("HardwareGeneration", 4u32, "M_series"),
            Self::DcSeries => serializer.serialize_unit_variant("HardwareGeneration", 5u32, "DC_series"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Hyper-V collector resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervCollector {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Collector properties class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CollectorPropertiesBaseWithAgent>,
}
impl HypervCollector {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a HypervCollector list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HypervCollectorListResult {
    #[doc = "The HypervCollector items on this page"]
    pub value: Vec<HypervCollector>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HypervCollectorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl HypervCollectorListResult {
    pub fn new(value: Vec<HypervCollector>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Class representing the impacted objects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImpactedAssessmentObject {
    #[doc = "Gets the object name."]
    #[serde(rename = "objectName", default, skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    #[doc = "Gets the object type."]
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
}
impl ImpactedAssessmentObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Import collector resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportCollector {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Collector properties class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CollectorPropertiesBase>,
}
impl ImportCollector {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a ImportCollector list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportCollectorListResult {
    #[doc = "The ImportCollector items on this page"]
    pub value: Vec<ImportCollector>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ImportCollectorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ImportCollectorListResult {
    pub fn new(value: Vec<ImportCollector>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Machine resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Machine {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineProperties>,
}
impl Machine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineAssessmentProperties {
    #[serde(flatten)]
    pub azure_resource_properties: AzureResourceProperties,
    #[doc = "Gets or sets the assessment error summary.\n            This is the number of\nmachines affected by each type of error in this assessment."]
    #[serde(rename = "assessmentErrorSummary", default, skip_serializing_if = "Option::is_none")]
    pub assessment_error_summary: Option<serde_json::Value>,
    #[doc = "Gets or sets the aggregate ultra storage cost for all machines in the\nassessment."]
    #[serde(rename = "monthlyUltraStorageCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_ultra_storage_cost: Option<f32>,
    #[doc = "Gets the collection of cost components."]
    #[serde(
        rename = "costComponents",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cost_components: Vec<CostComponent>,
    #[doc = "Gets or sets enterprise agreement subscription id."]
    #[serde(rename = "eaSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub ea_subscription_id: Option<String>,
    #[serde(rename = "azurePricingTier", default, skip_serializing_if = "Option::is_none")]
    pub azure_pricing_tier: Option<AzurePricingTier>,
    #[serde(rename = "azureStorageRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub azure_storage_redundancy: Option<AzureStorageRedundancy>,
    #[serde(rename = "reservedInstance", default, skip_serializing_if = "Option::is_none")]
    pub reserved_instance: Option<AzureReservedInstance>,
    #[serde(rename = "azureHybridUseBenefit", default, skip_serializing_if = "Option::is_none")]
    pub azure_hybrid_use_benefit: Option<AzureHybridUseBenefit>,
    #[doc = "Gets or sets the azure storage type. Premium, Standard etc."]
    #[serde(
        rename = "azureDiskTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub azure_disk_types: Vec<AzureDiskType>,
    #[doc = "Gets or sets the Azure VM families."]
    #[serde(
        rename = "azureVmFamilies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub azure_vm_families: Vec<AzureVmFamily>,
    #[doc = "Gets the distribution of sqlInstances by support status."]
    #[serde(rename = "distributionBySupportStatus", default, skip_serializing_if = "Option::is_none")]
    pub distribution_by_support_status: Option<serde_json::Value>,
    #[doc = "Gets the distribution distribution of sqlInstances by service pack insight."]
    #[serde(rename = "distributionByServicePackInsight", default, skip_serializing_if = "Option::is_none")]
    pub distribution_by_service_pack_insight: Option<serde_json::Value>,
    #[doc = "Gets the distribution by os name."]
    #[serde(rename = "distributionByOsName", default, skip_serializing_if = "Option::is_none")]
    pub distribution_by_os_name: Option<serde_json::Value>,
    #[doc = "Gets or sets the aggregate Compute Cost for all machines in the assessment."]
    #[serde(rename = "monthlyComputeCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_compute_cost: Option<f32>,
    #[doc = "Gets or sets the aggregate Bandwidth Cost for all machines in the assessment."]
    #[serde(rename = "monthlyBandwidthCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_bandwidth_cost: Option<f32>,
    #[doc = "Gets or sets the aggregate Storage Cost for all machines in the assessment."]
    #[serde(rename = "monthlyStorageCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_storage_cost: Option<f32>,
    #[doc = "Gets or sets the aggregate premium storage cost for all machines in the\nassessment."]
    #[serde(rename = "monthlyPremiumStorageCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_premium_storage_cost: Option<f32>,
    #[doc = "Gets or sets the aggregate standard SSD storage cost for all the machines in\nthe assessment."]
    #[serde(rename = "monthlyStandardSsdStorageCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_standard_ssd_storage_cost: Option<f32>,
    #[doc = "Gets or sets the Cloud suitability summary for all the machines in the\nassessment."]
    #[serde(rename = "suitabilitySummary", default, skip_serializing_if = "Option::is_none")]
    pub suitability_summary: Option<serde_json::Value>,
    #[doc = "Gets or sets the Number of machines part of the assessment."]
    #[serde(rename = "numberOfMachines", default, skip_serializing_if = "Option::is_none")]
    pub number_of_machines: Option<i32>,
    #[doc = "Details on the total up-time for the VM."]
    #[serde(rename = "vmUptime", default, skip_serializing_if = "Option::is_none")]
    pub vm_uptime: Option<VmUptime>,
    #[serde(rename = "groupType", default, skip_serializing_if = "Option::is_none")]
    pub group_type: Option<GroupType>,
    #[serde(rename = "assessmentType", default, skip_serializing_if = "Option::is_none")]
    pub assessment_type: Option<AssessmentType>,
    #[doc = "Azure Location or Azure region where to which the machines will be migrated."]
    #[serde(rename = "azureLocation", default, skip_serializing_if = "Option::is_none")]
    pub azure_location: Option<String>,
    #[serde(rename = "azureOfferCode", default, skip_serializing_if = "Option::is_none")]
    pub azure_offer_code: Option<AzureOfferCode>,
    #[doc = "Currency for Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<AzureCurrency>,
    #[doc = "Percentage of buffer that user wants on performance metrics when recommending\nAzure sizes."]
    #[serde(rename = "scalingFactor", default, skip_serializing_if = "Option::is_none")]
    pub scaling_factor: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentile: Option<Percentile>,
    #[serde(rename = "timeRange", default, skip_serializing_if = "Option::is_none")]
    pub time_range: Option<TimeRange>,
    #[doc = "Gets or sets the start time to consider performance data for assessment."]
    #[serde(rename = "perfDataStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub perf_data_start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the end time to consider performance data for assessment."]
    #[serde(rename = "perfDataEndTime", default, with = "azure_core::date::rfc3339::option")]
    pub perf_data_end_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<AssessmentStage>,
    #[doc = "Custom discount percentage."]
    #[serde(rename = "discountPercentage", default, skip_serializing_if = "Option::is_none")]
    pub discount_percentage: Option<f32>,
    #[doc = "Assessment Sizing Criteria."]
    #[serde(rename = "sizingCriterion", default, skip_serializing_if = "Option::is_none")]
    pub sizing_criterion: Option<AssessmentSizingCriterion>,
    #[doc = "Confidence Rating in Percentage."]
    #[serde(rename = "confidenceRatingInPercentage", default, skip_serializing_if = "Option::is_none")]
    pub confidence_rating_in_percentage: Option<f32>,
    #[doc = "Last time when rates were queried."]
    #[serde(rename = "pricesTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub prices_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Date and Time when assessment was created."]
    #[serde(rename = "createdTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Date and Time when assessment was last updated."]
    #[serde(rename = "updatedTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Assessment Status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AssessmentStatus>,
    #[doc = "Schema version."]
    #[serde(rename = "schemaVersion", default, skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
}
impl MachineAssessmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MachineBootType")]
pub enum MachineBootType {
    Unknown,
    #[serde(rename = "EFI")]
    Efi,
    #[serde(rename = "BIOS")]
    Bios,
    NotSpecified,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MachineBootType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MachineBootType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MachineBootType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("MachineBootType", 0u32, "Unknown"),
            Self::Efi => serializer.serialize_unit_variant("MachineBootType", 1u32, "EFI"),
            Self::Bios => serializer.serialize_unit_variant("MachineBootType", 2u32, "BIOS"),
            Self::NotSpecified => serializer.serialize_unit_variant("MachineBootType", 3u32, "NotSpecified"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a Machine list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineListResult {
    #[doc = "The Machine items on this page"]
    pub value: Vec<Machine>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MachineListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl MachineListResult {
    pub fn new(value: Vec<Machine>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties of a machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineProperties {
    #[doc = "Workload summary."]
    #[serde(rename = "workloadSummary", default, skip_serializing_if = "Option::is_none")]
    pub workload_summary: Option<WorkloadSummary>,
    #[doc = "List of errors for this machine."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<Error>,
    #[doc = "Represents a information \\ details of a processor."]
    #[serde(rename = "hostProcessor", default, skip_serializing_if = "Option::is_none")]
    pub host_processor: Option<ProcessorInfo>,
    #[doc = "Class to represent the Product Support Status."]
    #[serde(rename = "productSupportStatus", default, skip_serializing_if = "Option::is_none")]
    pub product_support_status: Option<ProductSupportStatus>,
    #[doc = "Site id of machine discovered in private data center."]
    #[serde(rename = "discoveryMachineArmId", default, skip_serializing_if = "Option::is_none")]
    pub discovery_machine_arm_id: Option<String>,
    #[doc = "The data center management server ARM Id for the machine."]
    #[serde(rename = "datacenterManagementServerArmId", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_management_server_arm_id: Option<String>,
    #[doc = "The data center management server name for the machine."]
    #[serde(rename = "datacenterManagementServerName", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_management_server_name: Option<String>,
    #[serde(rename = "bootType", default, skip_serializing_if = "Option::is_none")]
    pub boot_type: Option<MachineBootType>,
    #[doc = "Display Name of the Machine."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Megabytes of memory found allocated for the machine in private data center."]
    #[serde(rename = "megabytesOfMemory", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_of_memory: Option<f32>,
    #[doc = "Number of CPU cores found on the machine."]
    #[serde(rename = "numberOfCores", default, skip_serializing_if = "Option::is_none")]
    pub number_of_cores: Option<i32>,
    #[doc = "Operating system as reported by datacenter management solution."]
    #[serde(rename = "operatingSystemType", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_type: Option<String>,
    #[doc = "Operating system as reported by datacenter management solution."]
    #[serde(rename = "operatingSystemName", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_name: Option<String>,
    #[doc = "Operating system version as reported by datacenter management solution."]
    #[serde(rename = "operatingSystemVersion", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_version: Option<String>,
    #[doc = "Description for the machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "When was machine first created."]
    #[serde(rename = "createdTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Disks attached to the machine discovered in private data center."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disks: Option<serde_json::Value>,
    #[doc = "Gets the References to the groups that this machine is member of."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub groups: Vec<String>,
    #[doc = "Network adapters attached to the machine discovered in private data center."]
    #[serde(rename = "networkAdapters", default, skip_serializing_if = "Option::is_none")]
    pub network_adapters: Option<serde_json::Value>,
    #[doc = "SQL instances discovered on the machine."]
    #[serde(
        rename = "sqlInstances",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sql_instances: Vec<String>,
    #[doc = "Web applications discovered on the machine."]
    #[serde(
        rename = "webApplications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub web_applications: Vec<String>,
    #[doc = "When was machine last updated."]
    #[serde(rename = "updatedTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
}
impl MachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Migration Guideline Context."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationGuidelineContext {
    #[doc = "Gets the reasoning context key."]
    #[serde(rename = "contextKey", default, skip_serializing_if = "Option::is_none")]
    pub context_key: Option<String>,
    #[doc = "Gets the reasoning context value."]
    #[serde(rename = "contextValue", default, skip_serializing_if = "Option::is_none")]
    pub context_value: Option<String>,
}
impl MigrationGuidelineContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MultiSubnetIntent")]
pub enum MultiSubnetIntent {
    None,
    HighAvailability,
    DisasterRecovery,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MultiSubnetIntent {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MultiSubnetIntent {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MultiSubnetIntent {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("MultiSubnetIntent", 0u32, "None"),
            Self::HighAvailability => serializer.serialize_unit_variant("MultiSubnetIntent", 1u32, "HighAvailability"),
            Self::DisasterRecovery => serializer.serialize_unit_variant("MultiSubnetIntent", 2u32, "DisasterRecovery"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A network adapter discovered on a machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkAdapter {
    #[doc = "MAC Address of the network adapter."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "List of IP Addresses on the network adapter."]
    #[serde(
        rename = "ipAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_addresses: Vec<String>,
    #[doc = "User friendly name of the network adapter."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl NetworkAdapter {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OptimizationLogic")]
pub enum OptimizationLogic {
    MinimizeCost,
    ModernizeToPaaS,
    ModernizeToAzureSqlMi,
    ModernizeToAzureSqlDb,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OptimizationLogic {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OptimizationLogic {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OptimizationLogic {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::MinimizeCost => serializer.serialize_unit_variant("OptimizationLogic", 0u32, "MinimizeCost"),
            Self::ModernizeToPaaS => serializer.serialize_unit_variant("OptimizationLogic", 1u32, "ModernizeToPaaS"),
            Self::ModernizeToAzureSqlMi => serializer.serialize_unit_variant("OptimizationLogic", 2u32, "ModernizeToAzureSqlMi"),
            Self::ModernizeToAzureSqlDb => serializer.serialize_unit_variant("OptimizationLogic", 3u32, "ModernizeToAzureSqlDb"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OsLicense")]
pub enum OsLicense {
    Unknown,
    Yes,
    No,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OsLicense {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OsLicense {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OsLicense {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("OsLicense", 0u32, "Unknown"),
            Self::Yes => serializer.serialize_unit_variant("OsLicense", 1u32, "Yes"),
            Self::No => serializer.serialize_unit_variant("OsLicense", 2u32, "No"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Percentile")]
pub enum Percentile {
    Percentile50,
    Percentile90,
    Percentile95,
    Percentile99,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Percentile {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Percentile {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Percentile {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Percentile50 => serializer.serialize_unit_variant("Percentile", 0u32, "Percentile50"),
            Self::Percentile90 => serializer.serialize_unit_variant("Percentile", 1u32, "Percentile90"),
            Self::Percentile95 => serializer.serialize_unit_variant("Percentile", 2u32, "Percentile95"),
            Self::Percentile99 => serializer.serialize_unit_variant("Percentile", 3u32, "Percentile99"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The private endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The ARM identifier for private endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of the private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a PrivateEndpointConnection list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "The PrivateEndpointConnection items on this page"]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PrivateEndpointConnectionListResult {
    pub fn new(value: Vec<PrivateEndpointConnection>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties of the private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The group ids for the private endpoint resource."]
    #[serde(
        rename = "groupIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_ids: Vec<String>,
    #[doc = "The private endpoint resource."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "A collection of information about the state of the connection between service consumer and provider."]
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new(private_link_service_connection_state: PrivateLinkServiceConnectionState) -> Self {
        Self {
            group_ids: Vec::new(),
            private_endpoint: None,
            private_link_service_connection_state,
            provisioning_state: None,
        }
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointConnectionProvisioningState")]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointConnectionProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointConnectionProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointConnectionProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 0u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 1u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 2u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The private endpoint connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointServiceConnectionStatus")]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointServiceConnectionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointServiceConnectionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointServiceConnectionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 0u32, "Pending"),
            Self::Approved => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 1u32, "Approved"),
            Self::Rejected => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 2u32, "Rejected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a PrivateLinkResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceListResult {
    #[doc = "The PrivateLinkResource items on this page"]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateLinkResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PrivateLinkResourceListResult {
    pub fn new(value: Vec<PrivateLinkResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource required member names."]
    #[serde(
        rename = "requiredMembers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_members: Vec<String>,
    #[doc = "The private link resource private link DNS zone name."]
    #[serde(
        rename = "requiredZoneNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of information about the state of the connection between service consumer and provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[doc = "The reason for approval/rejection of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a information \\ details of a processor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessorInfo {
    #[doc = "Gets or sets the name \\ model of a processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the number of sockets."]
    #[serde(rename = "numberOfSockets", default, skip_serializing_if = "Option::is_none")]
    pub number_of_sockets: Option<i32>,
    #[doc = "Gets or sets the number of cores in a socket."]
    #[serde(rename = "numberOfCoresPerSocket", default, skip_serializing_if = "Option::is_none")]
    pub number_of_cores_per_socket: Option<i32>,
}
impl ProcessorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to represent the Product Support Status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductSupportStatus {
    #[doc = "Gets or sets current version of ServicePack."]
    #[serde(rename = "currentVersion", default, skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[doc = "Gets or sets ServicePack of the product."]
    #[serde(rename = "servicePackStatus", default, skip_serializing_if = "Option::is_none")]
    pub service_pack_status: Option<String>,
    #[doc = "Gets or sets the Extended Security Update ESU status."]
    #[serde(rename = "esuStatus", default, skip_serializing_if = "Option::is_none")]
    pub esu_status: Option<String>,
    #[doc = "Gets or sets the support status of the product."]
    #[serde(rename = "supportStatus", default, skip_serializing_if = "Option::is_none")]
    pub support_status: Option<String>,
    #[doc = "Gets or sets the ETA."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eta: Option<i32>,
    #[doc = "Gets or sets the current ESU support year."]
    #[serde(rename = "currentEsuYear", default, skip_serializing_if = "Option::is_none")]
    pub current_esu_year: Option<String>,
    #[doc = "Gets or sets the main stream end date of the product."]
    #[serde(rename = "mainstreamEndDate", default, with = "azure_core::date::rfc3339::option")]
    pub mainstream_end_date: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the extended support end date of the product."]
    #[serde(rename = "extendedSupportEndDate", default, with = "azure_core::date::rfc3339::option")]
    pub extended_support_end_date: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the extended security update year 1 end date of the product."]
    #[serde(rename = "extendedSecurityUpdateYear1EndDate", default, with = "azure_core::date::rfc3339::option")]
    pub extended_security_update_year1_end_date: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the extended security update year 2 end date of the product."]
    #[serde(rename = "extendedSecurityUpdateYear2EndDate", default, with = "azure_core::date::rfc3339::option")]
    pub extended_security_update_year2_end_date: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the extended security update year 3 end date of the product."]
    #[serde(rename = "extendedSecurityUpdateYear3EndDate", default, with = "azure_core::date::rfc3339::option")]
    pub extended_security_update_year3_end_date: Option<time::OffsetDateTime>,
}
impl ProductSupportStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectProperties {
    #[serde(flatten)]
    pub azure_resource_properties: AzureResourceProperties,
    #[doc = "Time when this project was created. Date-Time represented in ISO-8601 format."]
    #[serde(rename = "createdTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Time when this project was last updated. Date-Time represented in ISO-8601\nformat."]
    #[serde(rename = "updatedTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Endpoint at which the collector agent can call agent REST API."]
    #[serde(rename = "serviceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_endpoint: Option<String>,
    #[doc = "Assessment solution ARM id tracked by Microsoft.Migrate/migrateProjects."]
    #[serde(rename = "assessmentSolutionId", default, skip_serializing_if = "Option::is_none")]
    pub assessment_solution_id: Option<String>,
    #[doc = "Project Status."]
    #[serde(rename = "projectStatus", default, skip_serializing_if = "Option::is_none")]
    pub project_status: Option<ProjectStatus>,
    #[doc = "The ARM id of service map workspace created by customer."]
    #[serde(rename = "customerWorkspaceId", default, skip_serializing_if = "Option::is_none")]
    pub customer_workspace_id: Option<String>,
    #[doc = "Location of service map workspace created by customer."]
    #[serde(rename = "customerWorkspaceLocation", default, skip_serializing_if = "Option::is_none")]
    pub customer_workspace_location: Option<String>,
    #[doc = "This value can be set to 'enabled' to avoid breaking changes on existing\ncustomer resources and templates. If set to 'disabled', traffic over public\ninterface is not allowed, and private endpoint connections would be the\nexclusive access method."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<String>,
    #[doc = "The list of private endpoint connections to the project."]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "The ARM id of the storage account used for interactions when public access is\ndisabled."]
    #[serde(rename = "customerStorageAccountArmId", default, skip_serializing_if = "Option::is_none")]
    pub customer_storage_account_arm_id: Option<String>,
}
impl ProjectProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Project Status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProjectStatus")]
pub enum ProjectStatus {
    Active,
    Inactive,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProjectStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProjectStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProjectStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("ProjectStatus", 0u32, "Active"),
            Self::Inactive => serializer.serialize_unit_variant("ProjectStatus", 1u32, "Inactive"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The status of the current operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Provisioning,
    Updating,
    Deleting,
    Accepted,
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
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
            Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Accepted"),
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RecommendedSuitability")]
pub enum RecommendedSuitability {
    Unknown,
    #[serde(rename = "SuitableForSqlDB")]
    SuitableForSqlDb,
    #[serde(rename = "SuitableForSqlMI")]
    SuitableForSqlMi,
    #[serde(rename = "SuitableForVM")]
    SuitableForVm,
    #[serde(rename = "PotentiallySuitableForVM")]
    PotentiallySuitableForVm,
    ReadinessUnknown,
    NotSuitable,
    #[serde(rename = "SuitableForSqlVM")]
    SuitableForSqlVm,
    #[serde(rename = "ConditionallySuitableForSqlDB")]
    ConditionallySuitableForSqlDb,
    #[serde(rename = "ConditionallySuitableForSqlMI")]
    ConditionallySuitableForSqlMi,
    #[serde(rename = "ConditionallySuitableForVM")]
    ConditionallySuitableForVm,
    #[serde(rename = "ConditionallySuitableForSqlVM")]
    ConditionallySuitableForSqlVm,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RecommendedSuitability {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RecommendedSuitability {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RecommendedSuitability {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("RecommendedSuitability", 0u32, "Unknown"),
            Self::SuitableForSqlDb => serializer.serialize_unit_variant("RecommendedSuitability", 1u32, "SuitableForSqlDB"),
            Self::SuitableForSqlMi => serializer.serialize_unit_variant("RecommendedSuitability", 2u32, "SuitableForSqlMI"),
            Self::SuitableForVm => serializer.serialize_unit_variant("RecommendedSuitability", 3u32, "SuitableForVM"),
            Self::PotentiallySuitableForVm => serializer.serialize_unit_variant("RecommendedSuitability", 4u32, "PotentiallySuitableForVM"),
            Self::ReadinessUnknown => serializer.serialize_unit_variant("RecommendedSuitability", 5u32, "ReadinessUnknown"),
            Self::NotSuitable => serializer.serialize_unit_variant("RecommendedSuitability", 6u32, "NotSuitable"),
            Self::SuitableForSqlVm => serializer.serialize_unit_variant("RecommendedSuitability", 7u32, "SuitableForSqlVM"),
            Self::ConditionallySuitableForSqlDb => {
                serializer.serialize_unit_variant("RecommendedSuitability", 8u32, "ConditionallySuitableForSqlDB")
            }
            Self::ConditionallySuitableForSqlMi => {
                serializer.serialize_unit_variant("RecommendedSuitability", 9u32, "ConditionallySuitableForSqlMI")
            }
            Self::ConditionallySuitableForVm => {
                serializer.serialize_unit_variant("RecommendedSuitability", 10u32, "ConditionallySuitableForVM")
            }
            Self::ConditionallySuitableForSqlVm => {
                serializer.serialize_unit_variant("RecommendedSuitability", 11u32, "ConditionallySuitableForSqlVM")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "ARM id for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceId {
    #[doc = "Gets the relative URL to get to this REST resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ResourceId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Physical server collector resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerCollector {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Collector properties class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CollectorPropertiesBaseWithAgent>,
}
impl ServerCollector {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a ServerCollector list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerCollectorListResult {
    #[doc = "The ServerCollector items on this page"]
    pub value: Vec<ServerCollector>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerCollectorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerCollectorListResult {
    pub fn new(value: Vec<ServerCollector>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Shared Resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedResourcesDto {
    #[doc = "Gets the list of shared data disks."]
    #[serde(
        rename = "sharedDataDisks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub shared_data_disks: Vec<AzureManagedDiskSkuDto>,
    #[doc = "Gets the list of shared log disks."]
    #[serde(
        rename = "sharedLogDisks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub shared_log_disks: Vec<AzureManagedDiskSkuDto>,
    #[doc = "Gets the list of shared Temporary database disks."]
    #[serde(
        rename = "sharedTempDbDisks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub shared_temp_db_disks: Vec<AzureManagedDiskSkuDto>,
    #[doc = "Gets number of mounts of shared disks."]
    #[serde(rename = "numberOfMounts", default, skip_serializing_if = "Option::is_none")]
    pub number_of_mounts: Option<i32>,
    #[doc = "Quorum Witness."]
    #[serde(rename = "quorumWitness", default, skip_serializing_if = "Option::is_none")]
    pub quorum_witness: Option<AzureQuorumWitnessDto>,
}
impl SharedResourcesDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SkuReplicationMode")]
pub enum SkuReplicationMode {
    NotApplicable,
    ActiveGeoReplication,
    FailoverGroupInstance,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SkuReplicationMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SkuReplicationMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SkuReplicationMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotApplicable => serializer.serialize_unit_variant("SkuReplicationMode", 0u32, "NotApplicable"),
            Self::ActiveGeoReplication => serializer.serialize_unit_variant("SkuReplicationMode", 1u32, "ActiveGeoReplication"),
            Self::FailoverGroupInstance => serializer.serialize_unit_variant("SkuReplicationMode", 2u32, "FailoverGroupInstance"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SQL Assessed Network Adapter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAssessedNetworkAdapter {
    #[doc = "Cloud Suitability for Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suitability: Option<CloudSuitability>,
    #[serde(rename = "suitabilityDetail", default, skip_serializing_if = "Option::is_none")]
    pub suitability_detail: Option<AzureNetworkAdapterSuitabilityDetail>,
    #[serde(rename = "suitabilityExplanation", default, skip_serializing_if = "Option::is_none")]
    pub suitability_explanation: Option<AzureNetworkAdapterSuitabilityExplanation>,
    #[doc = "Gets the monthly bandwidth costs."]
    #[serde(rename = "monthlyBandwidthCosts", default, skip_serializing_if = "Option::is_none")]
    pub monthly_bandwidth_costs: Option<f32>,
    #[doc = "Gets the net gigabytes transmitted per month."]
    #[serde(rename = "netGigabytesTransmittedPerMonth", default, skip_serializing_if = "Option::is_none")]
    pub net_gigabytes_transmitted_per_month: Option<f32>,
    #[doc = "Gets the name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets the mac address."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "Gets the ip addresses."]
    #[serde(
        rename = "ipAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_addresses: Vec<String>,
    #[doc = "Gets the megabytes per second received."]
    #[serde(rename = "megabytesPerSecondReceived", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_received: Option<f32>,
    #[doc = "Gets the megabytes per second transmitted."]
    #[serde(rename = "megabytesPerSecondTransmitted", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_transmitted: Option<f32>,
}
impl SqlAssessedNetworkAdapter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the SQL migration issues."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAssessmentMigrationIssue {
    #[doc = "Gets the issue id."]
    #[serde(rename = "issueId", default, skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<String>,
    #[serde(rename = "issueCategory", default, skip_serializing_if = "Option::is_none")]
    pub issue_category: Option<SqlAssessmentMigrationIssueCategory>,
    #[doc = "Gets the list of impacted objects."]
    #[serde(
        rename = "impactedObjects",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub impacted_objects: Vec<ImpactedAssessmentObject>,
}
impl SqlAssessmentMigrationIssue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlAssessmentMigrationIssueCategory")]
pub enum SqlAssessmentMigrationIssueCategory {
    Issue,
    Warning,
    Internal,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlAssessmentMigrationIssueCategory {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlAssessmentMigrationIssueCategory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlAssessmentMigrationIssueCategory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Issue => serializer.serialize_unit_variant("SqlAssessmentMigrationIssueCategory", 0u32, "Issue"),
            Self::Warning => serializer.serialize_unit_variant("SqlAssessmentMigrationIssueCategory", 1u32, "Warning"),
            Self::Internal => serializer.serialize_unit_variant("SqlAssessmentMigrationIssueCategory", 2u32, "Internal"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SQL Assessment options web model object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAssessmentOptions {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "SQL Assessment options properties Web model object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlAssessmentOptionsProperties>,
}
impl SqlAssessmentOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a SqlAssessmentOptions list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlAssessmentOptionsListResult {
    #[doc = "The SqlAssessmentOptions items on this page"]
    pub value: Vec<SqlAssessmentOptions>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlAssessmentOptionsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlAssessmentOptionsListResult {
    pub fn new(value: Vec<SqlAssessmentOptions>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "SQL Assessment options properties Web model object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAssessmentOptionsProperties {
    #[doc = "Gets the list of VM families."]
    #[serde(
        rename = "vmFamilies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vm_families: Vec<VmFamilyOptions>,
    #[doc = "Gets the Reserved Instance VM Families list."]
    #[serde(
        rename = "reservedInstanceVmFamilies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reserved_instance_vm_families: Vec<AzureVmFamily>,
    #[doc = "Gets the Premium disk VM Families list."]
    #[serde(
        rename = "premiumDiskVmFamilies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub premium_disk_vm_families: Vec<AzureVmFamily>,
    #[doc = "Gets or sets the Premium disk VM Families list."]
    #[serde(
        rename = "savingsPlanVmFamilies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub savings_plan_vm_families: Vec<AzureVmFamily>,
    #[doc = "Gets or sets the list of Azure locations supporting Saving Plans for IAAS."]
    #[serde(
        rename = "savingsPlanSupportedLocations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub savings_plan_supported_locations: Vec<AzureLocation>,
    #[doc = "Gets or sets the list of Azure locations supporting Saving Plans for PAAS."]
    #[serde(
        rename = "savingsPlanSupportedLocationsForPaas",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub savings_plan_supported_locations_for_paas: Vec<AzureLocation>,
    #[doc = "Gets or sets the list of Azure locations supporting Reserved Instances for IAAS."]
    #[serde(
        rename = "reservedInstanceSupportedLocationsForIaas",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reserved_instance_supported_locations_for_iaas: Vec<AzureLocation>,
    #[doc = "Gets or sets the list of Azure Offers supporting Saving Plans."]
    #[serde(
        rename = "savingsPlanSupportedOffers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub savings_plan_supported_offers: Vec<AzureOfferCode>,
    #[doc = "Gets or sets the list of SQL target SKU properties for dropdowns."]
    #[serde(
        rename = "sqlSkus",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sql_skus: Vec<SqlPaaSTargetOptions>,
    #[doc = "Gets or sets the Reserved Instance SQL target types."]
    #[serde(
        rename = "reservedInstanceSqlTargets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reserved_instance_sql_targets: Vec<TargetType>,
    #[doc = "Gets or sets the list of Azure locations supporting Reserved Instances."]
    #[serde(
        rename = "reservedInstanceSupportedLocations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reserved_instance_supported_locations: Vec<AzureLocation>,
    #[doc = "Gets or sets the list of currencies supported for Reserved Instances."]
    #[serde(
        rename = "reservedInstanceSupportedCurrencies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reserved_instance_supported_currencies: Vec<AzureCurrency>,
    #[doc = "Gets or sets the list of offers supported for Reserved Instances."]
    #[serde(
        rename = "reservedInstanceSupportedOffers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reserved_instance_supported_offers: Vec<AzureOfferCode>,
    #[doc = "Gets or sets the list of offers supported for SQL assessments."]
    #[serde(
        rename = "supportedOffers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_offers: Vec<AzureOfferCode>,
}
impl SqlAssessmentOptionsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SQL Assessment REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAssessmentV2 {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "SQL assessment properties class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlAssessmentV2Properties>,
}
impl SqlAssessmentV2 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing Azure SQL IAAS suitability details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAssessmentV2IaasSuitabilityData {
    #[doc = "Class representing Azure SQL IAAS SKU."]
    #[serde(rename = "azureSqlSku", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_sku: Option<AzureSqlIaasSkuDto>,
    #[doc = "Gets the replica azure SQL IAAS SKU."]
    #[serde(
        rename = "replicaAzureSqlSku",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub replica_azure_sql_sku: Vec<AzureSqlIaasSkuDto>,
    #[doc = "Shared Resources."]
    #[serde(rename = "sharedResources", default, skip_serializing_if = "Option::is_none")]
    pub shared_resources: Option<SharedResourcesDto>,
    #[doc = "Gets the monthly compute cost."]
    #[serde(rename = "monthlyComputeCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_compute_cost: Option<f32>,
    #[doc = "Gets the monthly storage cost."]
    #[serde(rename = "monthlyStorageCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_storage_cost: Option<f32>,
    #[doc = "Gets the collection of cost components."]
    #[serde(
        rename = "costComponents",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cost_components: Vec<CostComponent>,
    #[doc = "Cloud Suitability for Azure."]
    #[serde(rename = "securitySuitability", default, skip_serializing_if = "Option::is_none")]
    pub security_suitability: Option<CloudSuitability>,
    #[doc = "Gets a value indicating whether replicas should be provisioned."]
    #[serde(rename = "shouldProvisionReplicas", default, skip_serializing_if = "Option::is_none")]
    pub should_provision_replicas: Option<bool>,
    #[serde(rename = "skuReplicationMode", default, skip_serializing_if = "Option::is_none")]
    pub sku_replication_mode: Option<SkuReplicationMode>,
    #[doc = "Gets the list of migration guidelines applicable to this target."]
    #[serde(
        rename = "migrationGuidelines",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub migration_guidelines: Vec<SqlMigrationGuideline>,
    #[doc = "Gets the list of SQL recommendation Reasoning."]
    #[serde(
        rename = "recommendationReasonings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub recommendation_reasonings: Vec<SqlRecommendationReasoning>,
    #[serde(rename = "migrationTargetPlatform", default, skip_serializing_if = "Option::is_none")]
    pub migration_target_platform: Option<TargetType>,
    #[doc = "Cloud Suitability for Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suitability: Option<CloudSuitability>,
    #[doc = "Gets the list of migrations issues."]
    #[serde(
        rename = "migrationIssues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub migration_issues: Vec<SqlAssessmentMigrationIssue>,
}
impl SqlAssessmentV2IaasSuitabilityData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a SqlAssessmentV2 list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlAssessmentV2ListResult {
    #[doc = "The SqlAssessmentV2 items on this page"]
    pub value: Vec<SqlAssessmentV2>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlAssessmentV2ListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlAssessmentV2ListResult {
    pub fn new(value: Vec<SqlAssessmentV2>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Class representing Azure SQL PAAS suitability details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAssessmentV2PaasSuitabilityData {
    #[doc = "Class representing Azure SQL PAAS SKU."]
    #[serde(rename = "azureSqlSku", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_sku: Option<AzureSqlPaasSkuDto>,
    #[doc = "Gets the replica azure SQL PAAS SKU."]
    #[serde(
        rename = "replicaAzureSqlSku",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub replica_azure_sql_sku: Vec<AzureSqlPaasSkuDto>,
    #[doc = "Shared Resources."]
    #[serde(rename = "sharedResources", default, skip_serializing_if = "Option::is_none")]
    pub shared_resources: Option<SharedResourcesDto>,
    #[doc = "Gets the monthly compute cost."]
    #[serde(rename = "monthlyComputeCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_compute_cost: Option<f32>,
    #[doc = "Gets the monthly storage cost."]
    #[serde(rename = "monthlyStorageCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_storage_cost: Option<f32>,
    #[doc = "Gets the collection of cost components."]
    #[serde(
        rename = "costComponents",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cost_components: Vec<CostComponent>,
    #[doc = "Cloud Suitability for Azure."]
    #[serde(rename = "securitySuitability", default, skip_serializing_if = "Option::is_none")]
    pub security_suitability: Option<CloudSuitability>,
    #[doc = "Gets a value indicating whether replicas should be provisioned."]
    #[serde(rename = "shouldProvisionReplicas", default, skip_serializing_if = "Option::is_none")]
    pub should_provision_replicas: Option<bool>,
    #[serde(rename = "skuReplicationMode", default, skip_serializing_if = "Option::is_none")]
    pub sku_replication_mode: Option<SkuReplicationMode>,
    #[doc = "Gets the list of migration guidelines applicable to this target."]
    #[serde(
        rename = "migrationGuidelines",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub migration_guidelines: Vec<SqlMigrationGuideline>,
    #[doc = "Gets the list of SQL recommendation Reasoning."]
    #[serde(
        rename = "recommendationReasonings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub recommendation_reasonings: Vec<SqlRecommendationReasoning>,
    #[serde(rename = "migrationTargetPlatform", default, skip_serializing_if = "Option::is_none")]
    pub migration_target_platform: Option<TargetType>,
    #[doc = "Cloud Suitability for Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suitability: Option<CloudSuitability>,
    #[doc = "Gets the list of migrations issues."]
    #[serde(
        rename = "migrationIssues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub migration_issues: Vec<SqlAssessmentMigrationIssue>,
}
impl SqlAssessmentV2PaasSuitabilityData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SQL assessment properties class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAssessmentV2Properties {
    #[serde(flatten)]
    pub azure_resource_properties: AzureResourceProperties,
    #[serde(rename = "osLicense", default, skip_serializing_if = "Option::is_none")]
    pub os_license: Option<OsLicense>,
    #[serde(rename = "environmentType", default, skip_serializing_if = "Option::is_none")]
    pub environment_type: Option<EnvironmentType>,
    #[doc = "Entity Uptime."]
    #[serde(rename = "entityUptime", default, skip_serializing_if = "Option::is_none")]
    pub entity_uptime: Option<EntityUptime>,
    #[serde(rename = "optimizationLogic", default, skip_serializing_if = "Option::is_none")]
    pub optimization_logic: Option<OptimizationLogic>,
    #[serde(rename = "reservedInstanceForVm", default, skip_serializing_if = "Option::is_none")]
    pub reserved_instance_for_vm: Option<AzureReservedInstance>,
    #[serde(rename = "azureOfferCodeForVm", default, skip_serializing_if = "Option::is_none")]
    pub azure_offer_code_for_vm: Option<AzureOfferCode>,
    #[doc = "Gets or sets the Enterprise agreement subscription id."]
    #[serde(rename = "eaSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub ea_subscription_id: Option<String>,
    #[doc = "SQL managed instance assessment settings."]
    #[serde(rename = "azureSqlManagedInstanceSettings", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_managed_instance_settings: Option<SqlMiSettings>,
    #[doc = "SQL database assessment settings."]
    #[serde(rename = "azureSqlDatabaseSettings", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_database_settings: Option<SqlDbSettings>,
    #[doc = "SQL VM assessment settings."]
    #[serde(rename = "azureSqlVmSettings", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_vm_settings: Option<SqlVmSettings>,
    #[serde(rename = "multiSubnetIntent", default, skip_serializing_if = "Option::is_none")]
    pub multi_subnet_intent: Option<MultiSubnetIntent>,
    #[serde(rename = "asyncCommitModeIntent", default, skip_serializing_if = "Option::is_none")]
    pub async_commit_mode_intent: Option<AsyncCommitModeIntent>,
    #[doc = "Gets or sets a value indicating whether internet access is available."]
    #[serde(rename = "isInternetAccessAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_internet_access_available: Option<bool>,
    #[doc = "Location for Azure."]
    #[serde(rename = "disasterRecoveryLocation", default, skip_serializing_if = "Option::is_none")]
    pub disaster_recovery_location: Option<AzureLocation>,
    #[doc = "Gets or sets a value indicating whether HADR assessments needs to be created."]
    #[serde(rename = "enableHadrAssessment", default, skip_serializing_if = "Option::is_none")]
    pub enable_hadr_assessment: Option<bool>,
    #[serde(rename = "azureSecurityOfferingType", default, skip_serializing_if = "Option::is_none")]
    pub azure_security_offering_type: Option<AzureSecurityOfferingType>,
    #[serde(rename = "reservedInstance", default, skip_serializing_if = "Option::is_none")]
    pub reserved_instance: Option<AzureReservedInstance>,
    #[serde(rename = "sqlServerLicense", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_license: Option<SqlServerLicense>,
    #[serde(rename = "groupType", default, skip_serializing_if = "Option::is_none")]
    pub group_type: Option<GroupType>,
    #[serde(rename = "assessmentType", default, skip_serializing_if = "Option::is_none")]
    pub assessment_type: Option<AssessmentType>,
    #[doc = "Azure Location or Azure region where to which the machines will be migrated."]
    #[serde(rename = "azureLocation", default, skip_serializing_if = "Option::is_none")]
    pub azure_location: Option<String>,
    #[serde(rename = "azureOfferCode", default, skip_serializing_if = "Option::is_none")]
    pub azure_offer_code: Option<AzureOfferCode>,
    #[doc = "Currency for Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<AzureCurrency>,
    #[doc = "Percentage of buffer that user wants on performance metrics when recommending\nAzure sizes."]
    #[serde(rename = "scalingFactor", default, skip_serializing_if = "Option::is_none")]
    pub scaling_factor: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentile: Option<Percentile>,
    #[serde(rename = "timeRange", default, skip_serializing_if = "Option::is_none")]
    pub time_range: Option<TimeRange>,
    #[doc = "Gets or sets the start time to consider performance data for assessment."]
    #[serde(rename = "perfDataStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub perf_data_start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the end time to consider performance data for assessment."]
    #[serde(rename = "perfDataEndTime", default, with = "azure_core::date::rfc3339::option")]
    pub perf_data_end_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<AssessmentStage>,
    #[doc = "Custom discount percentage."]
    #[serde(rename = "discountPercentage", default, skip_serializing_if = "Option::is_none")]
    pub discount_percentage: Option<f32>,
    #[doc = "Assessment Sizing Criteria."]
    #[serde(rename = "sizingCriterion", default, skip_serializing_if = "Option::is_none")]
    pub sizing_criterion: Option<AssessmentSizingCriterion>,
    #[doc = "Confidence Rating in Percentage."]
    #[serde(rename = "confidenceRatingInPercentage", default, skip_serializing_if = "Option::is_none")]
    pub confidence_rating_in_percentage: Option<f32>,
    #[doc = "Last time when rates were queried."]
    #[serde(rename = "pricesTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub prices_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Date and Time when assessment was created."]
    #[serde(rename = "createdTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Date and Time when assessment was last updated."]
    #[serde(rename = "updatedTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Assessment Status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AssessmentStatus>,
    #[doc = "Schema version."]
    #[serde(rename = "schemaVersion", default, skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
}
impl SqlAssessmentV2Properties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SQL Assessment REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAssessmentV2Summary {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "SQL Assessment V2 summary properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlAssessmentV2SummaryProperties>,
}
impl SqlAssessmentV2Summary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SQL Assessment V2 summary data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAssessmentV2SummaryData {
    #[doc = "Sql assessment summary data"]
    #[serde(rename = "suitabilitySummary", default, skip_serializing_if = "Option::is_none")]
    pub suitability_summary: Option<serde_json::Value>,
    #[doc = "Monthly compute cost"]
    #[serde(rename = "monthlyComputeCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_compute_cost: Option<f32>,
    #[doc = "Monthly storage cost"]
    #[serde(rename = "monthlyStorageCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_storage_cost: Option<f32>,
    #[doc = "Monthly license cost"]
    #[serde(rename = "monthlyLicenseCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_license_cost: Option<f32>,
    #[doc = "Confidence Rating"]
    #[serde(rename = "confidenceScore", default, skip_serializing_if = "Option::is_none")]
    pub confidence_score: Option<f32>,
    #[doc = "Monthly security cost"]
    #[serde(rename = "monthlySecurityCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_security_cost: Option<f32>,
}
impl SqlAssessmentV2SummaryData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a SqlAssessmentV2Summary list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlAssessmentV2SummaryListResult {
    #[doc = "The SqlAssessmentV2Summary items on this page"]
    pub value: Vec<SqlAssessmentV2Summary>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlAssessmentV2SummaryListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlAssessmentV2SummaryListResult {
    pub fn new(value: Vec<SqlAssessmentV2Summary>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "SQL Assessment V2 summary properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAssessmentV2SummaryProperties {
    #[doc = "Gets or sets the Assessment summary."]
    #[serde(rename = "assessmentSummary", default, skip_serializing_if = "Option::is_none")]
    pub assessment_summary: Option<serde_json::Value>,
    #[doc = "Gets the distribution of sqlInstances by support status."]
    #[serde(rename = "distributionBySupportStatus", default, skip_serializing_if = "Option::is_none")]
    pub distribution_by_support_status: Option<serde_json::Value>,
    #[doc = "Gets the distribution distribution of sqlInstances by service pack insight."]
    #[serde(rename = "distributionByServicePackInsight", default, skip_serializing_if = "Option::is_none")]
    pub distribution_by_service_pack_insight: Option<serde_json::Value>,
    #[doc = "Gets the distribution of sqlInstances by sql version."]
    #[serde(rename = "distributionBySqlVersion", default, skip_serializing_if = "Option::is_none")]
    pub distribution_by_sql_version: Option<serde_json::Value>,
    #[doc = "Gets the distribution of sqlInstances by sql edition."]
    #[serde(rename = "distributionBySqlEdition", default, skip_serializing_if = "Option::is_none")]
    pub distribution_by_sql_edition: Option<serde_json::Value>,
    #[doc = "Gets the instance distribution by sizing criterion."]
    #[serde(rename = "instanceDistributionBySizingCriterion", default, skip_serializing_if = "Option::is_none")]
    pub instance_distribution_by_sizing_criterion: Option<serde_json::Value>,
    #[doc = "Gets the database distribution by sizing criterion."]
    #[serde(rename = "databaseDistributionBySizingCriterion", default, skip_serializing_if = "Option::is_none")]
    pub database_distribution_by_sizing_criterion: Option<serde_json::Value>,
    #[doc = "Number of machines part of the assessment."]
    #[serde(rename = "numberOfMachines", default, skip_serializing_if = "Option::is_none")]
    pub number_of_machines: Option<i32>,
    #[doc = "Number of sql instances part of the assessment."]
    #[serde(rename = "numberOfSqlInstances", default, skip_serializing_if = "Option::is_none")]
    pub number_of_sql_instances: Option<i32>,
    #[doc = "Number of sql databases part of the assessment."]
    #[serde(rename = "numberOfSqlDatabases", default, skip_serializing_if = "Option::is_none")]
    pub number_of_sql_databases: Option<i32>,
    #[doc = "Number of sql failover cluster instances part of the assessment."]
    #[serde(rename = "numberOfFciInstances", default, skip_serializing_if = "Option::is_none")]
    pub number_of_fci_instances: Option<i32>,
    #[doc = "Number of sql availability groups part of the assessment."]
    #[serde(rename = "numberOfSqlAvailabilityGroups", default, skip_serializing_if = "Option::is_none")]
    pub number_of_sql_availability_groups: Option<i32>,
}
impl SqlAssessmentV2SummaryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Assessed Sql Availability Group Data Overview."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAvailabilityGroupDataOverview {
    #[doc = "Gets the availability group id."]
    #[serde(rename = "availabilityGroupId", default, skip_serializing_if = "Option::is_none")]
    pub availability_group_id: Option<String>,
    #[doc = "Gets the availability group name."]
    #[serde(rename = "availabilityGroupName", default, skip_serializing_if = "Option::is_none")]
    pub availability_group_name: Option<String>,
    #[doc = "Gets the availability group arm id."]
    #[serde(rename = "sqlAvailabilityGroupSdsArmId", default, skip_serializing_if = "Option::is_none")]
    pub sql_availability_group_sds_arm_id: Option<String>,
    #[doc = "Gets the availability group entity id."]
    #[serde(rename = "sqlAvailabilityGroupEntityId", default, skip_serializing_if = "Option::is_none")]
    pub sql_availability_group_entity_id: Option<String>,
    #[doc = "Gets the availability replica id."]
    #[serde(rename = "sqlAvailabilityReplicaId", default, skip_serializing_if = "Option::is_none")]
    pub sql_availability_replica_id: Option<String>,
}
impl SqlAvailabilityGroupDataOverview {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Assessed Sql Availability Replica Summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAvailabilityReplicaSummary {
    #[doc = "Gets the number Of synchronous read replicas."]
    #[serde(rename = "numberOfSynchronousReadReplicas", default, skip_serializing_if = "Option::is_none")]
    pub number_of_synchronous_read_replicas: Option<i32>,
    #[doc = "Gets the number Of synchronous non read replicas."]
    #[serde(rename = "numberOfSynchronousNonReadReplicas", default, skip_serializing_if = "Option::is_none")]
    pub number_of_synchronous_non_read_replicas: Option<i32>,
    #[doc = "Gets the number Of asynchronous read replicas."]
    #[serde(rename = "numberOfAsynchronousReadReplicas", default, skip_serializing_if = "Option::is_none")]
    pub number_of_asynchronous_read_replicas: Option<i32>,
    #[doc = "Gets the number Of asynchronous non read replicas."]
    #[serde(rename = "numberOfAsynchronousNonReadReplicas", default, skip_serializing_if = "Option::is_none")]
    pub number_of_asynchronous_non_read_replicas: Option<i32>,
    #[doc = "Gets the number Of primary replicas."]
    #[serde(rename = "numberOfPrimaryReplicas", default, skip_serializing_if = "Option::is_none")]
    pub number_of_primary_replicas: Option<i32>,
}
impl SqlAvailabilityReplicaSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SQL collector REST object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlCollector {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Collector properties class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CollectorPropertiesBaseWithAgent>,
}
impl SqlCollector {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a SqlCollector list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlCollectorListResult {
    #[doc = "The SqlCollector items on this page"]
    pub value: Vec<SqlCollector>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlCollectorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlCollectorListResult {
    pub fn new(value: Vec<SqlCollector>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "SQL database assessment settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlDbSettings {
    #[serde(rename = "azureSqlServiceTier", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_service_tier: Option<AzureSqlServiceTier>,
    #[serde(rename = "azureSqlDataBaseType", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_data_base_type: Option<AzureSqlDataBaseType>,
    #[serde(rename = "azureSqlComputeTier", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_compute_tier: Option<ComputeTier>,
    #[serde(rename = "azureSqlPurchaseModel", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_purchase_model: Option<AzureSqlPurchaseModel>,
}
impl SqlDbSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sql fci meta data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlFciMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<SqlFciMetadataState>,
    #[doc = "Gets whether fci is multi subnet."]
    #[serde(rename = "isMultiSubnet", default, skip_serializing_if = "Option::is_none")]
    pub is_multi_subnet: Option<bool>,
    #[doc = "Gets the fci shared disk count."]
    #[serde(rename = "fciSharedDiskCount", default, skip_serializing_if = "Option::is_none")]
    pub fci_shared_disk_count: Option<i32>,
}
impl SqlFciMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlFciMetadataState")]
pub enum SqlFciMetadataState {
    Unknown,
    Inherited,
    Initializing,
    Online,
    Offline,
    Failed,
    Pending,
    OnlinePending,
    OfflinePending,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlFciMetadataState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlFciMetadataState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlFciMetadataState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("SqlFciMetadataState", 0u32, "Unknown"),
            Self::Inherited => serializer.serialize_unit_variant("SqlFciMetadataState", 1u32, "Inherited"),
            Self::Initializing => serializer.serialize_unit_variant("SqlFciMetadataState", 2u32, "Initializing"),
            Self::Online => serializer.serialize_unit_variant("SqlFciMetadataState", 3u32, "Online"),
            Self::Offline => serializer.serialize_unit_variant("SqlFciMetadataState", 4u32, "Offline"),
            Self::Failed => serializer.serialize_unit_variant("SqlFciMetadataState", 5u32, "Failed"),
            Self::Pending => serializer.serialize_unit_variant("SqlFciMetadataState", 6u32, "Pending"),
            Self::OnlinePending => serializer.serialize_unit_variant("SqlFciMetadataState", 7u32, "OnlinePending"),
            Self::OfflinePending => serializer.serialize_unit_variant("SqlFciMetadataState", 8u32, "OfflinePending"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlFciState")]
pub enum SqlFciState {
    Unknown,
    Active,
    Passive,
    NotApplicable,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlFciState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlFciState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlFciState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("SqlFciState", 0u32, "Unknown"),
            Self::Active => serializer.serialize_unit_variant("SqlFciState", 1u32, "Active"),
            Self::Passive => serializer.serialize_unit_variant("SqlFciState", 2u32, "Passive"),
            Self::NotApplicable => serializer.serialize_unit_variant("SqlFciState", 3u32, "NotApplicable"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SQL managed instance assessment settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlMiSettings {
    #[serde(rename = "azureSqlServiceTier", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_service_tier: Option<AzureSqlServiceTier>,
    #[serde(rename = "azureSqlInstanceType", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_instance_type: Option<AzureSqlInstanceType>,
}
impl SqlMiSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sql Migration Guideline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlMigrationGuideline {
    #[doc = "Gets the guideline id."]
    #[serde(rename = "guidelineId", default, skip_serializing_if = "Option::is_none")]
    pub guideline_id: Option<String>,
    #[serde(rename = "migrationGuidelineCategory", default, skip_serializing_if = "Option::is_none")]
    pub migration_guideline_category: Option<SqlMigrationGuidelineCategory>,
    #[doc = "Gets the migration guideline context."]
    #[serde(
        rename = "migrationGuidelineContext",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub migration_guideline_context: Vec<MigrationGuidelineContext>,
}
impl SqlMigrationGuideline {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlMigrationGuidelineCategory")]
pub enum SqlMigrationGuidelineCategory {
    Unknown,
    General,
    FailoverCluterInstanceGuideLine,
    AvailabilityGroupGuideline,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlMigrationGuidelineCategory {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlMigrationGuidelineCategory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlMigrationGuidelineCategory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("SqlMigrationGuidelineCategory", 0u32, "Unknown"),
            Self::General => serializer.serialize_unit_variant("SqlMigrationGuidelineCategory", 1u32, "General"),
            Self::FailoverCluterInstanceGuideLine => {
                serializer.serialize_unit_variant("SqlMigrationGuidelineCategory", 2u32, "FailoverCluterInstanceGuideLine")
            }
            Self::AvailabilityGroupGuideline => {
                serializer.serialize_unit_variant("SqlMigrationGuidelineCategory", 3u32, "AvailabilityGroupGuideline")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SQL target options."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPaaSTargetOptions {
    #[serde(rename = "computeTier", default, skip_serializing_if = "Option::is_none")]
    pub compute_tier: Option<ComputeTier>,
    #[serde(rename = "hardwareGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hardware_generation: Option<HardwareGeneration>,
    #[serde(rename = "targetType", default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<TargetType>,
    #[serde(rename = "serviceTier", default, skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<AzureSqlServiceTier>,
    #[doc = "Gets or sets the target location."]
    #[serde(
        rename = "targetLocations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub target_locations: Vec<AzureLocation>,
}
impl SqlPaaSTargetOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing Azure SQL Recommendation Reasoning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlRecommendationReasoning {
    #[doc = "Gets the reasoning id."]
    #[serde(rename = "reasoningId", default, skip_serializing_if = "Option::is_none")]
    pub reasoning_id: Option<String>,
    #[doc = "Gets the reasoning status."]
    #[serde(rename = "reasoningString", default, skip_serializing_if = "Option::is_none")]
    pub reasoning_string: Option<String>,
    #[doc = "Gets the reasoning category."]
    #[serde(rename = "reasoningCategory", default, skip_serializing_if = "Option::is_none")]
    pub reasoning_category: Option<String>,
    #[doc = "Gets the Sql recommended reasoning parameters."]
    #[serde(
        rename = "contextParameters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub context_parameters: Vec<SqlRecommendationReasoningContext>,
}
impl SqlRecommendationReasoning {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing Azure SQL Recommendation Reasoning Context."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlRecommendationReasoningContext {
    #[doc = "Gets the reasoning context key."]
    #[serde(rename = "contextKey", default, skip_serializing_if = "Option::is_none")]
    pub context_key: Option<String>,
    #[doc = "Gets the reasoning context value."]
    #[serde(rename = "contextValue", default, skip_serializing_if = "Option::is_none")]
    pub context_value: Option<String>,
}
impl SqlRecommendationReasoningContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlServerLicense")]
pub enum SqlServerLicense {
    Unknown,
    Yes,
    No,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlServerLicense {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlServerLicense {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlServerLicense {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("SqlServerLicense", 0u32, "Unknown"),
            Self::Yes => serializer.serialize_unit_variant("SqlServerLicense", 1u32, "Yes"),
            Self::No => serializer.serialize_unit_variant("SqlServerLicense", 2u32, "No"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SQL VM assessment settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVmSettings {
    #[doc = "Gets or sets the Azure VM families (calling instance series to keep it\nconsistent with other targets)."]
    #[serde(
        rename = "instanceSeries",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub instance_series: Vec<AzureVmFamily>,
}
impl SqlVmSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TargetType")]
pub enum TargetType {
    Unknown,
    Recommended,
    AzureSqlDatabase,
    AzureSqlManagedInstance,
    AzureSqlVirtualMachine,
    AzureVirtualMachine,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TargetType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TargetType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TargetType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("TargetType", 0u32, "Unknown"),
            Self::Recommended => serializer.serialize_unit_variant("TargetType", 1u32, "Recommended"),
            Self::AzureSqlDatabase => serializer.serialize_unit_variant("TargetType", 2u32, "AzureSqlDatabase"),
            Self::AzureSqlManagedInstance => serializer.serialize_unit_variant("TargetType", 3u32, "AzureSqlManagedInstance"),
            Self::AzureSqlVirtualMachine => serializer.serialize_unit_variant("TargetType", 4u32, "AzureSqlVirtualMachine"),
            Self::AzureVirtualMachine => serializer.serialize_unit_variant("TargetType", 5u32, "AzureVirtualMachine"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TimeRange")]
pub enum TimeRange {
    Day,
    Week,
    Month,
    Custom,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TimeRange {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TimeRange {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TimeRange {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Day => serializer.serialize_unit_variant("TimeRange", 0u32, "Day"),
            Self::Week => serializer.serialize_unit_variant("TimeRange", 1u32, "Week"),
            Self::Month => serializer.serialize_unit_variant("TimeRange", 2u32, "Month"),
            Self::Custom => serializer.serialize_unit_variant("TimeRange", 3u32, "Custom"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
#[doc = "Assessment options for Ultra disk type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UltraDiskAssessmentOptions {
    #[doc = "Family name."]
    #[serde(rename = "familyName", default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[doc = "List of locations where ultra disk is supported for this VMfamily."]
    #[serde(
        rename = "targetLocations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub target_locations: Vec<String>,
}
impl UltraDiskAssessmentOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of group update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateGroupBody {
    #[doc = "For optimistic concurrency control."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Body properties of group update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GroupBodyProperties>,
}
impl UpdateGroupBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VM family name, the list of targeted azure locations and the category of the\nfamily."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmFamilyOptions {
    #[doc = "Name of the VM family."]
    #[serde(rename = "familyName", default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[doc = "List of Azure regions."]
    #[serde(
        rename = "targetLocations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub target_locations: Vec<String>,
    #[doc = "Category of the VM family."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub category: Vec<String>,
}
impl VmFamilyOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details on the total up-time for the VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmUptime {
    #[doc = "Number of days in a month for VM uptime."]
    #[serde(rename = "daysPerMonth", default, skip_serializing_if = "Option::is_none")]
    pub days_per_month: Option<i32>,
    #[doc = "Number of hours per day for VM uptime."]
    #[serde(rename = "hoursPerDay", default, skip_serializing_if = "Option::is_none")]
    pub hours_per_day: Option<i32>,
}
impl VmUptime {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VMware collector resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmwareCollector {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Collector properties class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CollectorPropertiesBaseWithAgent>,
}
impl VmwareCollector {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a VmwareCollector list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmwareCollectorListResult {
    #[doc = "The VmwareCollector items on this page"]
    pub value: Vec<VmwareCollector>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VmwareCollectorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VmwareCollectorListResult {
    pub fn new(value: Vec<VmwareCollector>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Workload summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadSummary {
    #[doc = "Gets or sets oracle databases."]
    #[serde(rename = "oracleInstances", default, skip_serializing_if = "Option::is_none")]
    pub oracle_instances: Option<i32>,
    #[doc = "Gets or sets oracle databases."]
    #[serde(rename = "springApps", default, skip_serializing_if = "Option::is_none")]
    pub spring_apps: Option<i32>,
}
impl WorkloadSummary {
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
