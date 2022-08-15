#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A disk assessed for an assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedDisk {
    #[doc = "Name of the assessed disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gigabytes of storage provisioned for this disk."]
    #[serde(rename = "gigabytesProvisioned", default, skip_serializing_if = "Option::is_none")]
    pub gigabytes_provisioned: Option<f64>,
    #[doc = "Gigabytes of storage consumed by this disk."]
    #[serde(rename = "gigabytesConsumed", default, skip_serializing_if = "Option::is_none")]
    pub gigabytes_consumed: Option<f64>,
    #[doc = "Disk throughput in MegaBytes per second."]
    #[serde(rename = "megabytesPerSecondOfRead", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_of_read: Option<f64>,
    #[doc = "Expected data points for MegaBytes per second of read."]
    #[serde(
        rename = "megabytesPerSecondOfReadDataPointsExpected",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub megabytes_per_second_of_read_data_points_expected: Option<i32>,
    #[doc = "Received data points for MegaBytes per second of read."]
    #[serde(
        rename = "megabytesPerSecondOfReadDataPointsReceived",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub megabytes_per_second_of_read_data_points_received: Option<i32>,
    #[doc = "Disk throughput in MegaBytes per second."]
    #[serde(rename = "megabytesPerSecondOfWrite", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_of_write: Option<f64>,
    #[doc = "Expected data points for MegaBytes per second of write."]
    #[serde(
        rename = "megabytesPerSecondOfWriteDataPointsExpected",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub megabytes_per_second_of_write_data_points_expected: Option<i32>,
    #[doc = "Received data points for MegaBytes per second of write."]
    #[serde(
        rename = "megabytesPerSecondOfWriteDataPointsReceived",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub megabytes_per_second_of_write_data_points_received: Option<i32>,
    #[doc = "Number of read operations per second for the disk."]
    #[serde(rename = "numberOfReadOperationsPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub number_of_read_operations_per_second: Option<f64>,
    #[doc = "Expected number of data points for read operations per second."]
    #[serde(
        rename = "numberOfReadOperationsPerSecondDataPointsExpected",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_read_operations_per_second_data_points_expected: Option<i32>,
    #[doc = "Received number of data points for read operations per second."]
    #[serde(
        rename = "numberOfReadOperationsPerSecondDataPointsReceived",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_read_operations_per_second_data_points_received: Option<i32>,
    #[doc = "Number of read and write operations per second for the disk."]
    #[serde(rename = "numberOfWriteOperationsPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub number_of_write_operations_per_second: Option<f64>,
    #[doc = "Expected number of data points for write operations per second."]
    #[serde(
        rename = "numberOfWriteOperationsPerSecondDataPointsExpected",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_write_operations_per_second_data_points_expected: Option<i32>,
    #[doc = "Received number of data points for write operations per second."]
    #[serde(
        rename = "numberOfWriteOperationsPerSecondDataPointsReceived",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_write_operations_per_second_data_points_received: Option<i32>,
    #[doc = "Estimated aggregate storage cost for a 31-day month for this disk."]
    #[serde(rename = "monthlyStorageCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_storage_cost: Option<f64>,
    #[doc = "Storage type selected for this disk."]
    #[serde(rename = "recommendedDiskType", default, skip_serializing_if = "Option::is_none")]
    pub recommended_disk_type: Option<assessed_disk::RecommendedDiskType>,
    #[doc = "Recommended Azure size for the disk, given utilization data and preferences set on Assessment."]
    #[serde(rename = "recommendedDiskSize", default, skip_serializing_if = "Option::is_none")]
    pub recommended_disk_size: Option<assessed_disk::RecommendedDiskSize>,
    #[doc = "Gigabytes of storage provided by the recommended Azure disk size."]
    #[serde(rename = "gigabytesForRecommendedDiskSize", default, skip_serializing_if = "Option::is_none")]
    pub gigabytes_for_recommended_disk_size: Option<i32>,
    #[doc = "Whether this disk is suitable for Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suitability: Option<assessed_disk::Suitability>,
    #[doc = "If disk is suitable, this explains the reasons and mitigation steps."]
    #[serde(rename = "suitabilityExplanation", default, skip_serializing_if = "Option::is_none")]
    pub suitability_explanation: Option<assessed_disk::SuitabilityExplanation>,
}
impl AssessedDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod assessed_disk {
    use super::*;
    #[doc = "Storage type selected for this disk."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecommendedDiskType")]
    pub enum RecommendedDiskType {
        Unknown,
        Standard,
        Premium,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecommendedDiskType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecommendedDiskType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecommendedDiskType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("RecommendedDiskType", 0u32, "Unknown"),
                Self::Standard => serializer.serialize_unit_variant("RecommendedDiskType", 1u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("RecommendedDiskType", 2u32, "Premium"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Recommended Azure size for the disk, given utilization data and preferences set on Assessment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecommendedDiskSize")]
    pub enum RecommendedDiskSize {
        Unknown,
        #[serde(rename = "Standard_S4")]
        StandardS4,
        #[serde(rename = "Standard_S6")]
        StandardS6,
        #[serde(rename = "Standard_S10")]
        StandardS10,
        #[serde(rename = "Standard_S20")]
        StandardS20,
        #[serde(rename = "Standard_S30")]
        StandardS30,
        #[serde(rename = "Standard_S40")]
        StandardS40,
        #[serde(rename = "Standard_S50")]
        StandardS50,
        #[serde(rename = "Premium_P4")]
        PremiumP4,
        #[serde(rename = "Premium_P6")]
        PremiumP6,
        #[serde(rename = "Premium_P10")]
        PremiumP10,
        #[serde(rename = "Premium_P20")]
        PremiumP20,
        #[serde(rename = "Premium_P30")]
        PremiumP30,
        #[serde(rename = "Premium_P40")]
        PremiumP40,
        #[serde(rename = "Premium_P50")]
        PremiumP50,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecommendedDiskSize {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecommendedDiskSize {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecommendedDiskSize {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("RecommendedDiskSize", 0u32, "Unknown"),
                Self::StandardS4 => serializer.serialize_unit_variant("RecommendedDiskSize", 1u32, "Standard_S4"),
                Self::StandardS6 => serializer.serialize_unit_variant("RecommendedDiskSize", 2u32, "Standard_S6"),
                Self::StandardS10 => serializer.serialize_unit_variant("RecommendedDiskSize", 3u32, "Standard_S10"),
                Self::StandardS20 => serializer.serialize_unit_variant("RecommendedDiskSize", 4u32, "Standard_S20"),
                Self::StandardS30 => serializer.serialize_unit_variant("RecommendedDiskSize", 5u32, "Standard_S30"),
                Self::StandardS40 => serializer.serialize_unit_variant("RecommendedDiskSize", 6u32, "Standard_S40"),
                Self::StandardS50 => serializer.serialize_unit_variant("RecommendedDiskSize", 7u32, "Standard_S50"),
                Self::PremiumP4 => serializer.serialize_unit_variant("RecommendedDiskSize", 8u32, "Premium_P4"),
                Self::PremiumP6 => serializer.serialize_unit_variant("RecommendedDiskSize", 9u32, "Premium_P6"),
                Self::PremiumP10 => serializer.serialize_unit_variant("RecommendedDiskSize", 10u32, "Premium_P10"),
                Self::PremiumP20 => serializer.serialize_unit_variant("RecommendedDiskSize", 11u32, "Premium_P20"),
                Self::PremiumP30 => serializer.serialize_unit_variant("RecommendedDiskSize", 12u32, "Premium_P30"),
                Self::PremiumP40 => serializer.serialize_unit_variant("RecommendedDiskSize", 13u32, "Premium_P40"),
                Self::PremiumP50 => serializer.serialize_unit_variant("RecommendedDiskSize", 14u32, "Premium_P50"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether this disk is suitable for Azure."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Suitability")]
    pub enum Suitability {
        Unknown,
        NotSuitable,
        Suitable,
        ConditionallySuitable,
        ReadinessUnknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Suitability {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Suitability {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Suitability {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("Suitability", 0u32, "Unknown"),
                Self::NotSuitable => serializer.serialize_unit_variant("Suitability", 1u32, "NotSuitable"),
                Self::Suitable => serializer.serialize_unit_variant("Suitability", 2u32, "Suitable"),
                Self::ConditionallySuitable => serializer.serialize_unit_variant("Suitability", 3u32, "ConditionallySuitable"),
                Self::ReadinessUnknown => serializer.serialize_unit_variant("Suitability", 4u32, "ReadinessUnknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "If disk is suitable, this explains the reasons and mitigation steps."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SuitabilityExplanation")]
    pub enum SuitabilityExplanation {
        Unknown,
        NotApplicable,
        DiskSizeGreaterThanSupported,
        NoSuitableDiskSizeForIops,
        NoSuitableDiskSizeForThroughput,
        NoDiskSizeFoundInSelectedLocation,
        NoDiskSizeFoundForSelectedRedundancy,
        InternalErrorOccurredForDiskEvaluation,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SuitabilityExplanation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SuitabilityExplanation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SuitabilityExplanation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("SuitabilityExplanation", 0u32, "Unknown"),
                Self::NotApplicable => serializer.serialize_unit_variant("SuitabilityExplanation", 1u32, "NotApplicable"),
                Self::DiskSizeGreaterThanSupported => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 2u32, "DiskSizeGreaterThanSupported")
                }
                Self::NoSuitableDiskSizeForIops => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 3u32, "NoSuitableDiskSizeForIops")
                }
                Self::NoSuitableDiskSizeForThroughput => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 4u32, "NoSuitableDiskSizeForThroughput")
                }
                Self::NoDiskSizeFoundInSelectedLocation => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 5u32, "NoDiskSizeFoundInSelectedLocation")
                }
                Self::NoDiskSizeFoundForSelectedRedundancy => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 6u32, "NoDiskSizeFoundForSelectedRedundancy")
                }
                Self::InternalErrorOccurredForDiskEvaluation => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 7u32, "InternalErrorOccurredForDiskEvaluation")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A machine evaluated as part of an assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedMachine {
    #[doc = "Path reference to this assessed machine. /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Migrate/projects/{projectName}/groups/{groupName}/assessments/{assessmentName}/assessedMachines/{assessedMachineName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "For optimistic concurrency control."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Type of the object = [Microsoft.Migrate/projects/groups/assessments/assessedMachines]."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of an assessed machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssessedMachineProperties>,
}
impl AssessedMachine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an assessed machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedMachineProperties {
    #[doc = "List of references to the groups that the machine is member of."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    #[doc = "Time when this machine was discovered by Azure Migrate agent. Date-Time represented in ISO-8601 format."]
    #[serde(rename = "discoveredTimestamp", with = "azure_core::date::rfc3339::option")]
    pub discovered_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Boot type of the machine."]
    #[serde(rename = "bootType", default, skip_serializing_if = "Option::is_none")]
    pub boot_type: Option<assessed_machine_properties::BootType>,
    #[doc = "Container defined in the management solution that this machine is part of in the datacenter."]
    #[serde(rename = "datacenterContainer", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_container: Option<String>,
    #[doc = "Name of the server hosting the datacenter management solution."]
    #[serde(rename = "datacenterManagementServer", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_management_server: Option<String>,
    #[doc = "ID of the machine as tracked by the datacenter management solution."]
    #[serde(rename = "datacenterMachineId", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_machine_id: Option<String>,
    #[doc = "ID of the server hosting the datacenter management solution."]
    #[serde(rename = "datacenterManagementServerId", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_management_server_id: Option<String>,
    #[doc = "Description of the machine"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "User readable name of the machine as defined by the user in their private datacenter."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Memory in Megabytes."]
    #[serde(rename = "megabytesOfMemory", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_of_memory: Option<f64>,
    #[doc = "Processor count."]
    #[serde(rename = "numberOfCores", default, skip_serializing_if = "Option::is_none")]
    pub number_of_cores: Option<i32>,
    #[doc = "Operating System of the machine."]
    #[serde(rename = "operatingSystem", default, skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    #[doc = "Monthly network cost estimate for the network adapters that are attached to this machine as a group, for a 31-day month."]
    #[serde(rename = "monthlyBandwidthCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_bandwidth_cost: Option<f64>,
    #[doc = "Monthly storage cost estimate for the disks that are attached to this machine as a group, for a 31-day month."]
    #[serde(rename = "monthlyStorageCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_storage_cost: Option<f64>,
    #[doc = "Dictionary of disks attached to the machine. Key is ID of disk. Value is a disk object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disks: Option<serde_json::Value>,
    #[doc = "Dictionary of network adapters attached to the machine. Key is name of the adapter. Value is a network adapter object."]
    #[serde(rename = "networkAdapters", default, skip_serializing_if = "Option::is_none")]
    pub network_adapters: Option<serde_json::Value>,
    #[doc = "Recommended Azure size for this machine."]
    #[serde(rename = "recommendedSize", default, skip_serializing_if = "Option::is_none")]
    pub recommended_size: Option<assessed_machine_properties::RecommendedSize>,
    #[doc = "Number of CPU cores in the Recommended Azure VM Size."]
    #[serde(rename = "numberOfCoresForRecommendedSize", default, skip_serializing_if = "Option::is_none")]
    pub number_of_cores_for_recommended_size: Option<i32>,
    #[doc = "Megabytes of memory in the Recommended Azure VM Size."]
    #[serde(rename = "megabytesOfMemoryForRecommendedSize", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_of_memory_for_recommended_size: Option<f64>,
    #[doc = "Compute Cost for a 31-day month, if the machine is migrated to Azure with the Recommended Size."]
    #[serde(rename = "monthlyComputeCostForRecommendedSize", default, skip_serializing_if = "Option::is_none")]
    pub monthly_compute_cost_for_recommended_size: Option<f64>,
    #[doc = "Utilization percentage of the processor core as observed in the private data center, in the Time Range selected on Assessment, reported as the Percentile value based on the percentile number selected in assessment."]
    #[serde(rename = "percentageCoresUtilization", default, skip_serializing_if = "Option::is_none")]
    pub percentage_cores_utilization: Option<f64>,
    #[doc = "Utilization percentage of the memory as observed in the private data center, in the Time Range selected on Assessment, reported as the Percentile value based on the percentile number selected in assessment."]
    #[serde(rename = "percentageMemoryUtilization", default, skip_serializing_if = "Option::is_none")]
    pub percentage_memory_utilization: Option<f64>,
    #[doc = "Expected data points for percentage of cores utilization."]
    #[serde(
        rename = "percentageCoresUtilizationDataPointsExpected",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub percentage_cores_utilization_data_points_expected: Option<i32>,
    #[doc = "Received data points for percentage of cores utilization."]
    #[serde(
        rename = "percentageCoresUtilizationDataPointsReceived",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub percentage_cores_utilization_data_points_received: Option<i32>,
    #[doc = "Expected data points for percentage of memory utilization."]
    #[serde(
        rename = "percentageMemoryUtilizationDataPointsExpected",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub percentage_memory_utilization_data_points_expected: Option<i32>,
    #[doc = "Received data points for percentage of memory utilization."]
    #[serde(
        rename = "percentageMemoryUtilizationDataPointsReceived",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub percentage_memory_utilization_data_points_received: Option<i32>,
    #[doc = "Whether machine is suitable for migration to Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suitability: Option<assessed_machine_properties::Suitability>,
    #[doc = "If machine is not ready to be migrated, this explains the reasons and mitigation steps."]
    #[serde(rename = "suitabilityExplanation", default, skip_serializing_if = "Option::is_none")]
    pub suitability_explanation: Option<assessed_machine_properties::SuitabilityExplanation>,
    #[doc = "Time when this machine was created. Date-Time represented in ISO-8601 format."]
    #[serde(rename = "createdTimestamp", with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Time when this machine was last updated. Date-Time represented in ISO-8601 format."]
    #[serde(rename = "updatedTimestamp", with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
}
impl AssessedMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod assessed_machine_properties {
    use super::*;
    #[doc = "Boot type of the machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BootType")]
    pub enum BootType {
        Unknown,
        #[serde(rename = "EFI")]
        Efi,
        #[serde(rename = "BIOS")]
        Bios,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BootType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BootType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BootType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("BootType", 0u32, "Unknown"),
                Self::Efi => serializer.serialize_unit_variant("BootType", 1u32, "EFI"),
                Self::Bios => serializer.serialize_unit_variant("BootType", 2u32, "BIOS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Recommended Azure size for this machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecommendedSize")]
    pub enum RecommendedSize {
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
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecommendedSize {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecommendedSize {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecommendedSize {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("RecommendedSize", 0u32, "Unknown"),
                Self::BasicA0 => serializer.serialize_unit_variant("RecommendedSize", 1u32, "Basic_A0"),
                Self::BasicA1 => serializer.serialize_unit_variant("RecommendedSize", 2u32, "Basic_A1"),
                Self::BasicA2 => serializer.serialize_unit_variant("RecommendedSize", 3u32, "Basic_A2"),
                Self::BasicA3 => serializer.serialize_unit_variant("RecommendedSize", 4u32, "Basic_A3"),
                Self::BasicA4 => serializer.serialize_unit_variant("RecommendedSize", 5u32, "Basic_A4"),
                Self::StandardA0 => serializer.serialize_unit_variant("RecommendedSize", 6u32, "Standard_A0"),
                Self::StandardA1 => serializer.serialize_unit_variant("RecommendedSize", 7u32, "Standard_A1"),
                Self::StandardA2 => serializer.serialize_unit_variant("RecommendedSize", 8u32, "Standard_A2"),
                Self::StandardA3 => serializer.serialize_unit_variant("RecommendedSize", 9u32, "Standard_A3"),
                Self::StandardA4 => serializer.serialize_unit_variant("RecommendedSize", 10u32, "Standard_A4"),
                Self::StandardA5 => serializer.serialize_unit_variant("RecommendedSize", 11u32, "Standard_A5"),
                Self::StandardA6 => serializer.serialize_unit_variant("RecommendedSize", 12u32, "Standard_A6"),
                Self::StandardA7 => serializer.serialize_unit_variant("RecommendedSize", 13u32, "Standard_A7"),
                Self::StandardA8 => serializer.serialize_unit_variant("RecommendedSize", 14u32, "Standard_A8"),
                Self::StandardA9 => serializer.serialize_unit_variant("RecommendedSize", 15u32, "Standard_A9"),
                Self::StandardA10 => serializer.serialize_unit_variant("RecommendedSize", 16u32, "Standard_A10"),
                Self::StandardA11 => serializer.serialize_unit_variant("RecommendedSize", 17u32, "Standard_A11"),
                Self::StandardA1V2 => serializer.serialize_unit_variant("RecommendedSize", 18u32, "Standard_A1_v2"),
                Self::StandardA2V2 => serializer.serialize_unit_variant("RecommendedSize", 19u32, "Standard_A2_v2"),
                Self::StandardA4V2 => serializer.serialize_unit_variant("RecommendedSize", 20u32, "Standard_A4_v2"),
                Self::StandardA8V2 => serializer.serialize_unit_variant("RecommendedSize", 21u32, "Standard_A8_v2"),
                Self::StandardA2mV2 => serializer.serialize_unit_variant("RecommendedSize", 22u32, "Standard_A2m_v2"),
                Self::StandardA4mV2 => serializer.serialize_unit_variant("RecommendedSize", 23u32, "Standard_A4m_v2"),
                Self::StandardA8mV2 => serializer.serialize_unit_variant("RecommendedSize", 24u32, "Standard_A8m_v2"),
                Self::StandardD1 => serializer.serialize_unit_variant("RecommendedSize", 25u32, "Standard_D1"),
                Self::StandardD2 => serializer.serialize_unit_variant("RecommendedSize", 26u32, "Standard_D2"),
                Self::StandardD3 => serializer.serialize_unit_variant("RecommendedSize", 27u32, "Standard_D3"),
                Self::StandardD4 => serializer.serialize_unit_variant("RecommendedSize", 28u32, "Standard_D4"),
                Self::StandardD11 => serializer.serialize_unit_variant("RecommendedSize", 29u32, "Standard_D11"),
                Self::StandardD12 => serializer.serialize_unit_variant("RecommendedSize", 30u32, "Standard_D12"),
                Self::StandardD13 => serializer.serialize_unit_variant("RecommendedSize", 31u32, "Standard_D13"),
                Self::StandardD14 => serializer.serialize_unit_variant("RecommendedSize", 32u32, "Standard_D14"),
                Self::StandardD1V2 => serializer.serialize_unit_variant("RecommendedSize", 33u32, "Standard_D1_v2"),
                Self::StandardD2V2 => serializer.serialize_unit_variant("RecommendedSize", 34u32, "Standard_D2_v2"),
                Self::StandardD3V2 => serializer.serialize_unit_variant("RecommendedSize", 35u32, "Standard_D3_v2"),
                Self::StandardD4V2 => serializer.serialize_unit_variant("RecommendedSize", 36u32, "Standard_D4_v2"),
                Self::StandardD5V2 => serializer.serialize_unit_variant("RecommendedSize", 37u32, "Standard_D5_v2"),
                Self::StandardD11V2 => serializer.serialize_unit_variant("RecommendedSize", 38u32, "Standard_D11_v2"),
                Self::StandardD12V2 => serializer.serialize_unit_variant("RecommendedSize", 39u32, "Standard_D12_v2"),
                Self::StandardD13V2 => serializer.serialize_unit_variant("RecommendedSize", 40u32, "Standard_D13_v2"),
                Self::StandardD14V2 => serializer.serialize_unit_variant("RecommendedSize", 41u32, "Standard_D14_v2"),
                Self::StandardD15V2 => serializer.serialize_unit_variant("RecommendedSize", 42u32, "Standard_D15_v2"),
                Self::StandardDs1 => serializer.serialize_unit_variant("RecommendedSize", 43u32, "Standard_DS1"),
                Self::StandardDs2 => serializer.serialize_unit_variant("RecommendedSize", 44u32, "Standard_DS2"),
                Self::StandardDs3 => serializer.serialize_unit_variant("RecommendedSize", 45u32, "Standard_DS3"),
                Self::StandardDs4 => serializer.serialize_unit_variant("RecommendedSize", 46u32, "Standard_DS4"),
                Self::StandardDs11 => serializer.serialize_unit_variant("RecommendedSize", 47u32, "Standard_DS11"),
                Self::StandardDs12 => serializer.serialize_unit_variant("RecommendedSize", 48u32, "Standard_DS12"),
                Self::StandardDs13 => serializer.serialize_unit_variant("RecommendedSize", 49u32, "Standard_DS13"),
                Self::StandardDs14 => serializer.serialize_unit_variant("RecommendedSize", 50u32, "Standard_DS14"),
                Self::StandardDs1V2 => serializer.serialize_unit_variant("RecommendedSize", 51u32, "Standard_DS1_v2"),
                Self::StandardDs2V2 => serializer.serialize_unit_variant("RecommendedSize", 52u32, "Standard_DS2_v2"),
                Self::StandardDs3V2 => serializer.serialize_unit_variant("RecommendedSize", 53u32, "Standard_DS3_v2"),
                Self::StandardDs4V2 => serializer.serialize_unit_variant("RecommendedSize", 54u32, "Standard_DS4_v2"),
                Self::StandardDs5V2 => serializer.serialize_unit_variant("RecommendedSize", 55u32, "Standard_DS5_v2"),
                Self::StandardDs11V2 => serializer.serialize_unit_variant("RecommendedSize", 56u32, "Standard_DS11_v2"),
                Self::StandardDs12V2 => serializer.serialize_unit_variant("RecommendedSize", 57u32, "Standard_DS12_v2"),
                Self::StandardDs13V2 => serializer.serialize_unit_variant("RecommendedSize", 58u32, "Standard_DS13_v2"),
                Self::StandardDs14V2 => serializer.serialize_unit_variant("RecommendedSize", 59u32, "Standard_DS14_v2"),
                Self::StandardDs15V2 => serializer.serialize_unit_variant("RecommendedSize", 60u32, "Standard_DS15_v2"),
                Self::StandardF1 => serializer.serialize_unit_variant("RecommendedSize", 61u32, "Standard_F1"),
                Self::StandardF2 => serializer.serialize_unit_variant("RecommendedSize", 62u32, "Standard_F2"),
                Self::StandardF4 => serializer.serialize_unit_variant("RecommendedSize", 63u32, "Standard_F4"),
                Self::StandardF8 => serializer.serialize_unit_variant("RecommendedSize", 64u32, "Standard_F8"),
                Self::StandardF16 => serializer.serialize_unit_variant("RecommendedSize", 65u32, "Standard_F16"),
                Self::StandardF1s => serializer.serialize_unit_variant("RecommendedSize", 66u32, "Standard_F1s"),
                Self::StandardF2s => serializer.serialize_unit_variant("RecommendedSize", 67u32, "Standard_F2s"),
                Self::StandardF4s => serializer.serialize_unit_variant("RecommendedSize", 68u32, "Standard_F4s"),
                Self::StandardF8s => serializer.serialize_unit_variant("RecommendedSize", 69u32, "Standard_F8s"),
                Self::StandardF16s => serializer.serialize_unit_variant("RecommendedSize", 70u32, "Standard_F16s"),
                Self::StandardG1 => serializer.serialize_unit_variant("RecommendedSize", 71u32, "Standard_G1"),
                Self::StandardG2 => serializer.serialize_unit_variant("RecommendedSize", 72u32, "Standard_G2"),
                Self::StandardG3 => serializer.serialize_unit_variant("RecommendedSize", 73u32, "Standard_G3"),
                Self::StandardG4 => serializer.serialize_unit_variant("RecommendedSize", 74u32, "Standard_G4"),
                Self::StandardG5 => serializer.serialize_unit_variant("RecommendedSize", 75u32, "Standard_G5"),
                Self::StandardGs1 => serializer.serialize_unit_variant("RecommendedSize", 76u32, "Standard_GS1"),
                Self::StandardGs2 => serializer.serialize_unit_variant("RecommendedSize", 77u32, "Standard_GS2"),
                Self::StandardGs3 => serializer.serialize_unit_variant("RecommendedSize", 78u32, "Standard_GS3"),
                Self::StandardGs4 => serializer.serialize_unit_variant("RecommendedSize", 79u32, "Standard_GS4"),
                Self::StandardGs5 => serializer.serialize_unit_variant("RecommendedSize", 80u32, "Standard_GS5"),
                Self::StandardH8 => serializer.serialize_unit_variant("RecommendedSize", 81u32, "Standard_H8"),
                Self::StandardH16 => serializer.serialize_unit_variant("RecommendedSize", 82u32, "Standard_H16"),
                Self::StandardH8m => serializer.serialize_unit_variant("RecommendedSize", 83u32, "Standard_H8m"),
                Self::StandardH16m => serializer.serialize_unit_variant("RecommendedSize", 84u32, "Standard_H16m"),
                Self::StandardH16r => serializer.serialize_unit_variant("RecommendedSize", 85u32, "Standard_H16r"),
                Self::StandardH16mr => serializer.serialize_unit_variant("RecommendedSize", 86u32, "Standard_H16mr"),
                Self::StandardL4s => serializer.serialize_unit_variant("RecommendedSize", 87u32, "Standard_L4s"),
                Self::StandardL8s => serializer.serialize_unit_variant("RecommendedSize", 88u32, "Standard_L8s"),
                Self::StandardL16s => serializer.serialize_unit_variant("RecommendedSize", 89u32, "Standard_L16s"),
                Self::StandardL32s => serializer.serialize_unit_variant("RecommendedSize", 90u32, "Standard_L32s"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether machine is suitable for migration to Azure."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Suitability")]
    pub enum Suitability {
        Unknown,
        NotSuitable,
        Suitable,
        ConditionallySuitable,
        ReadinessUnknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Suitability {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Suitability {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Suitability {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("Suitability", 0u32, "Unknown"),
                Self::NotSuitable => serializer.serialize_unit_variant("Suitability", 1u32, "NotSuitable"),
                Self::Suitable => serializer.serialize_unit_variant("Suitability", 2u32, "Suitable"),
                Self::ConditionallySuitable => serializer.serialize_unit_variant("Suitability", 3u32, "ConditionallySuitable"),
                Self::ReadinessUnknown => serializer.serialize_unit_variant("Suitability", 4u32, "ReadinessUnknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "If machine is not ready to be migrated, this explains the reasons and mitigation steps."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SuitabilityExplanation")]
    pub enum SuitabilityExplanation {
        Unknown,
        NotApplicable,
        GuestOperatingSystemArchitectureNotSupported,
        GuestOperatingSystemNotSupported,
        BootTypeNotSupported,
        MoreDisksThanSupported,
        NoSuitableVmSizeFound,
        OneOrMoreDisksNotSuitable,
        OneOrMoreAdaptersNotSuitable,
        InternalErrorOccuredDuringComputeEvaluation,
        InternalErrorOccuredDuringStorageEvaluation,
        InternalErrorOccuredDuringNetworkEvaluation,
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
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SuitabilityExplanation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SuitabilityExplanation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SuitabilityExplanation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("SuitabilityExplanation", 0u32, "Unknown"),
                Self::NotApplicable => serializer.serialize_unit_variant("SuitabilityExplanation", 1u32, "NotApplicable"),
                Self::GuestOperatingSystemArchitectureNotSupported => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 2u32, "GuestOperatingSystemArchitectureNotSupported")
                }
                Self::GuestOperatingSystemNotSupported => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 3u32, "GuestOperatingSystemNotSupported")
                }
                Self::BootTypeNotSupported => serializer.serialize_unit_variant("SuitabilityExplanation", 4u32, "BootTypeNotSupported"),
                Self::MoreDisksThanSupported => serializer.serialize_unit_variant("SuitabilityExplanation", 5u32, "MoreDisksThanSupported"),
                Self::NoSuitableVmSizeFound => serializer.serialize_unit_variant("SuitabilityExplanation", 6u32, "NoSuitableVmSizeFound"),
                Self::OneOrMoreDisksNotSuitable => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 7u32, "OneOrMoreDisksNotSuitable")
                }
                Self::OneOrMoreAdaptersNotSuitable => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 8u32, "OneOrMoreAdaptersNotSuitable")
                }
                Self::InternalErrorOccuredDuringComputeEvaluation => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 9u32, "InternalErrorOccuredDuringComputeEvaluation")
                }
                Self::InternalErrorOccuredDuringStorageEvaluation => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 10u32, "InternalErrorOccuredDuringStorageEvaluation")
                }
                Self::InternalErrorOccuredDuringNetworkEvaluation => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 11u32, "InternalErrorOccuredDuringNetworkEvaluation")
                }
                Self::NoVmSizeSupportsStoragePerformance => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 12u32, "NoVmSizeSupportsStoragePerformance")
                }
                Self::NoVmSizeSupportsNetworkPerformance => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 13u32, "NoVmSizeSupportsNetworkPerformance")
                }
                Self::NoVmSizeForSelectedPricingTier => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 14u32, "NoVmSizeForSelectedPricingTier")
                }
                Self::NoVmSizeForSelectedAzureLocation => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 15u32, "NoVmSizeForSelectedAzureLocation")
                }
                Self::CheckRedHatLinuxVersion => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 16u32, "CheckRedHatLinuxVersion")
                }
                Self::CheckOpenSuseLinuxVersion => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 17u32, "CheckOpenSuseLinuxVersion")
                }
                Self::CheckWindowsServer2008R2Version => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 18u32, "CheckWindowsServer2008R2Version")
                }
                Self::CheckCentOsVersion => serializer.serialize_unit_variant("SuitabilityExplanation", 19u32, "CheckCentOsVersion"),
                Self::CheckDebianLinuxVersion => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 20u32, "CheckDebianLinuxVersion")
                }
                Self::CheckSuseLinuxVersion => serializer.serialize_unit_variant("SuitabilityExplanation", 21u32, "CheckSuseLinuxVersion"),
                Self::CheckOracleLinuxVersion => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 22u32, "CheckOracleLinuxVersion")
                }
                Self::CheckUbuntuLinuxVersion => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 23u32, "CheckUbuntuLinuxVersion")
                }
                Self::CheckCoreOsLinuxVersion => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 24u32, "CheckCoreOsLinuxVersion")
                }
                Self::WindowsServerVersionConditionallySupported => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 25u32, "WindowsServerVersionConditionallySupported")
                }
                Self::NoGuestOperatingSystemConditionallySupported => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 26u32, "NoGuestOperatingSystemConditionallySupported")
                }
                Self::WindowsClientVersionsConditionallySupported => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 27u32, "WindowsClientVersionsConditionallySupported")
                }
                Self::BootTypeUnknown => serializer.serialize_unit_variant("SuitabilityExplanation", 28u32, "BootTypeUnknown"),
                Self::GuestOperatingSystemUnknown => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 29u32, "GuestOperatingSystemUnknown")
                }
                Self::WindowsServerVersionsSupportedWithCaveat => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 30u32, "WindowsServerVersionsSupportedWithCaveat")
                }
                Self::WindowsOsNoLongerUnderMsSupport => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 31u32, "WindowsOSNoLongerUnderMSSupport")
                }
                Self::EndorsedWithConditionsLinuxDistributions => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 32u32, "EndorsedWithConditionsLinuxDistributions")
                }
                Self::UnendorsedLinuxDistributions => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 33u32, "UnendorsedLinuxDistributions")
                }
                Self::NoVmSizeForStandardPricingTier => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 34u32, "NoVmSizeForStandardPricingTier")
                }
                Self::NoVmSizeForBasicPricingTier => {
                    serializer.serialize_unit_variant("SuitabilityExplanation", 35u32, "NoVmSizeForBasicPricingTier")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of assessed machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedMachineResultList {
    #[doc = "List of assessed machines."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AssessedMachine>,
}
impl azure_core::Continuable for AssessedMachineResultList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AssessedMachineResultList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A network adapter assessed for an assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessedNetworkAdapter {
    #[doc = "MAC Address of the network adapter."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "List of IP Addresses on the network adapter."]
    #[serde(rename = "ipAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_addresses: Vec<String>,
    #[doc = "Monthly cost estimate for network bandwidth used by this network adapter."]
    #[serde(rename = "monthlyBandwidthCosts", default, skip_serializing_if = "Option::is_none")]
    pub monthly_bandwidth_costs: Option<f64>,
    #[doc = "Adapter throughput for incoming traffic in MegaBytes per second."]
    #[serde(rename = "megabytesPerSecondReceived", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_received: Option<f64>,
    #[doc = "Expected data points for incoming traffic in MegaBytes per second."]
    #[serde(
        rename = "megabytesPerSecondReceivedDataPointsExpected",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub megabytes_per_second_received_data_points_expected: Option<i32>,
    #[doc = "Received data points for incoming traffic in MegaBytes per second."]
    #[serde(
        rename = "megabytesPerSecondOfReadDataPointsReceived",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub megabytes_per_second_of_read_data_points_received: Option<i32>,
    #[doc = "Adapter throughput for outgoing traffic in MegaBytes per second."]
    #[serde(rename = "megabytesPerSecondTransmitted", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_transmitted: Option<f64>,
    #[doc = "Expected data points for outgoing traffic in MegaBytes per second."]
    #[serde(
        rename = "megabytesPerSecondTransmittedDataPointsExpected",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub megabytes_per_second_transmitted_data_points_expected: Option<i32>,
    #[doc = "Received data points for outgoing traffic in MegaBytes per second."]
    #[serde(
        rename = "megabytesPerSecondTransmittedDataPointsReceived",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub megabytes_per_second_transmitted_data_points_received: Option<i32>,
    #[doc = "Gigabytes transmitted through this adapter each month."]
    #[serde(rename = "netGigabytesTransmittedPerMonth", default, skip_serializing_if = "Option::is_none")]
    pub net_gigabytes_transmitted_per_month: Option<f64>,
    #[doc = "Whether this adapter is suitable for Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suitability: Option<assessed_network_adapter::Suitability>,
    #[doc = "If network adapter is suitable, this explains the reasons and mitigation steps."]
    #[serde(rename = "suitabilityExplanation", default, skip_serializing_if = "Option::is_none")]
    pub suitability_explanation: Option<assessed_network_adapter::SuitabilityExplanation>,
}
impl AssessedNetworkAdapter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod assessed_network_adapter {
    use super::*;
    #[doc = "Whether this adapter is suitable for Azure."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Suitability")]
    pub enum Suitability {
        Unknown,
        NotSuitable,
        Suitable,
        ConditionallySuitable,
        ReadinessUnknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Suitability {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Suitability {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Suitability {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("Suitability", 0u32, "Unknown"),
                Self::NotSuitable => serializer.serialize_unit_variant("Suitability", 1u32, "NotSuitable"),
                Self::Suitable => serializer.serialize_unit_variant("Suitability", 2u32, "Suitable"),
                Self::ConditionallySuitable => serializer.serialize_unit_variant("Suitability", 3u32, "ConditionallySuitable"),
                Self::ReadinessUnknown => serializer.serialize_unit_variant("Suitability", 4u32, "ReadinessUnknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "If network adapter is suitable, this explains the reasons and mitigation steps."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SuitabilityExplanation")]
    pub enum SuitabilityExplanation {
        Unknown,
        NotApplicable,
        InternalErrorOccured,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SuitabilityExplanation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SuitabilityExplanation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SuitabilityExplanation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("SuitabilityExplanation", 0u32, "Unknown"),
                Self::NotApplicable => serializer.serialize_unit_variant("SuitabilityExplanation", 1u32, "NotApplicable"),
                Self::InternalErrorOccured => serializer.serialize_unit_variant("SuitabilityExplanation", 2u32, "InternalErrorOccured"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An assessment created for a group in the Migration project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Assessment {
    #[doc = "Path reference to this assessment. /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Migrate/projects/{projectName}/groups/{groupName}/assessment/{assessmentName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Unique name of an assessment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "For optimistic concurrency control."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Type of the object = [Microsoft.Migrate/projects/groups/assessments]."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of an assessment."]
    pub properties: AssessmentProperties,
}
impl Assessment {
    pub fn new(properties: AssessmentProperties) -> Self {
        Self {
            id: None,
            name: None,
            e_tag: None,
            type_: None,
            properties,
        }
    }
}
#[doc = "List of assessment options."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessmentOptionsResultList {
    #[doc = "Dictionary of VM families grouped by vm family name describing the targeted azure locations of VM family and the category of the family."]
    #[serde(rename = "vmFamilies", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_families: Vec<VmFamily>,
    #[doc = "List of supported VM Families."]
    #[serde(rename = "reservedInstanceVmFamilies", default, skip_serializing_if = "Vec::is_empty")]
    pub reserved_instance_vm_families: Vec<String>,
}
impl AssessmentOptionsResultList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssessmentProperties {
    #[doc = "Target Azure location for which the machines should be assessed. These enums are the same as used by Compute API."]
    #[serde(rename = "azureLocation")]
    pub azure_location: assessment_properties::AzureLocation,
    #[doc = "Offer code according to which cost estimation is done."]
    #[serde(rename = "azureOfferCode")]
    pub azure_offer_code: assessment_properties::AzureOfferCode,
    #[doc = "Pricing tier for Size evaluation."]
    #[serde(rename = "azurePricingTier")]
    pub azure_pricing_tier: assessment_properties::AzurePricingTier,
    #[doc = "Storage Redundancy type offered by Azure."]
    #[serde(rename = "azureStorageRedundancy")]
    pub azure_storage_redundancy: assessment_properties::AzureStorageRedundancy,
    #[doc = "Scaling factor used over utilization data to add a performance buffer for new machines to be created in Azure. Min Value = 1.0, Max value = 1.9, Default = 1.3."]
    #[serde(rename = "scalingFactor")]
    pub scaling_factor: f64,
    #[doc = "Percentile of performance data used to recommend Azure size."]
    pub percentile: assessment_properties::Percentile,
    #[doc = "Time range of performance data used to recommend a size."]
    #[serde(rename = "timeRange")]
    pub time_range: assessment_properties::TimeRange,
    #[doc = "User configurable setting that describes the status of the assessment."]
    pub stage: assessment_properties::Stage,
    #[doc = "Currency to report prices in."]
    pub currency: assessment_properties::Currency,
    #[doc = "AHUB discount on windows virtual machines."]
    #[serde(rename = "azureHybridUseBenefit")]
    pub azure_hybrid_use_benefit: assessment_properties::AzureHybridUseBenefit,
    #[doc = "Custom discount percentage to be applied on final costs. Can be in the range [0, 100]."]
    #[serde(rename = "discountPercentage")]
    pub discount_percentage: f64,
    #[doc = "Confidence rating percentage for assessment. Can be in the range [0, 100]."]
    #[serde(rename = "confidenceRatingInPercentage", default, skip_serializing_if = "Option::is_none")]
    pub confidence_rating_in_percentage: Option<f64>,
    #[doc = "Assessment sizing criterion."]
    #[serde(rename = "sizingCriterion")]
    pub sizing_criterion: assessment_properties::SizingCriterion,
    #[doc = "Time when the Azure Prices were queried. Date-Time represented in ISO-8601 format."]
    #[serde(rename = "pricesTimestamp", with = "azure_core::date::rfc3339::option")]
    pub prices_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Time when this project was created. Date-Time represented in ISO-8601 format."]
    #[serde(rename = "createdTimestamp", with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Time when this project was last updated. Date-Time represented in ISO-8601 format."]
    #[serde(rename = "updatedTimestamp", with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Monthly compute cost estimate for the machines that are part of this assessment as a group, for a 31-day month."]
    #[serde(rename = "monthlyComputeCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_compute_cost: Option<f64>,
    #[doc = "Monthly network cost estimate for the machines that are part of this assessment as a group, for a 31-day month."]
    #[serde(rename = "monthlyBandwidthCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_bandwidth_cost: Option<f64>,
    #[doc = "Monthly storage cost estimate for the machines that are part of this assessment as a group, for a 31-day month."]
    #[serde(rename = "monthlyStorageCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_storage_cost: Option<f64>,
    #[doc = "Whether the assessment has been created and is valid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<assessment_properties::Status>,
    #[doc = "Number of assessed machines part of this assessment."]
    #[serde(rename = "numberOfMachines", default, skip_serializing_if = "Option::is_none")]
    pub number_of_machines: Option<i32>,
}
impl AssessmentProperties {
    pub fn new(
        azure_location: assessment_properties::AzureLocation,
        azure_offer_code: assessment_properties::AzureOfferCode,
        azure_pricing_tier: assessment_properties::AzurePricingTier,
        azure_storage_redundancy: assessment_properties::AzureStorageRedundancy,
        scaling_factor: f64,
        percentile: assessment_properties::Percentile,
        time_range: assessment_properties::TimeRange,
        stage: assessment_properties::Stage,
        currency: assessment_properties::Currency,
        azure_hybrid_use_benefit: assessment_properties::AzureHybridUseBenefit,
        discount_percentage: f64,
        sizing_criterion: assessment_properties::SizingCriterion,
    ) -> Self {
        Self {
            azure_location,
            azure_offer_code,
            azure_pricing_tier,
            azure_storage_redundancy,
            scaling_factor,
            percentile,
            time_range,
            stage,
            currency,
            azure_hybrid_use_benefit,
            discount_percentage,
            confidence_rating_in_percentage: None,
            sizing_criterion,
            prices_timestamp: None,
            created_timestamp: None,
            updated_timestamp: None,
            monthly_compute_cost: None,
            monthly_bandwidth_cost: None,
            monthly_storage_cost: None,
            status: None,
            number_of_machines: None,
        }
    }
}
pub mod assessment_properties {
    use super::*;
    #[doc = "Target Azure location for which the machines should be assessed. These enums are the same as used by Compute API."]
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Offer code according to which cost estimation is done."]
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Pricing tier for Size evaluation."]
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
    #[doc = "Storage Redundancy type offered by Azure."]
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
    #[doc = "Percentile of performance data used to recommend Azure size."]
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
    #[doc = "Time range of performance data used to recommend a size."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TimeRange")]
    pub enum TimeRange {
        Day,
        Week,
        Month,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "User configurable setting that describes the status of the assessment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Stage")]
    pub enum Stage {
        InProgress,
        UnderReview,
        Approved,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Stage {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Stage {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Stage {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::InProgress => serializer.serialize_unit_variant("Stage", 0u32, "InProgress"),
                Self::UnderReview => serializer.serialize_unit_variant("Stage", 1u32, "UnderReview"),
                Self::Approved => serializer.serialize_unit_variant("Stage", 2u32, "Approved"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Currency to report prices in."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Currency")]
    pub enum Currency {
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
    impl FromStr for Currency {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Currency {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Currency {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("Currency", 0u32, "Unknown"),
                Self::Usd => serializer.serialize_unit_variant("Currency", 1u32, "USD"),
                Self::Dkk => serializer.serialize_unit_variant("Currency", 2u32, "DKK"),
                Self::Cad => serializer.serialize_unit_variant("Currency", 3u32, "CAD"),
                Self::Idr => serializer.serialize_unit_variant("Currency", 4u32, "IDR"),
                Self::Jpy => serializer.serialize_unit_variant("Currency", 5u32, "JPY"),
                Self::Krw => serializer.serialize_unit_variant("Currency", 6u32, "KRW"),
                Self::Nzd => serializer.serialize_unit_variant("Currency", 7u32, "NZD"),
                Self::Nok => serializer.serialize_unit_variant("Currency", 8u32, "NOK"),
                Self::Rub => serializer.serialize_unit_variant("Currency", 9u32, "RUB"),
                Self::Sar => serializer.serialize_unit_variant("Currency", 10u32, "SAR"),
                Self::Zar => serializer.serialize_unit_variant("Currency", 11u32, "ZAR"),
                Self::Sek => serializer.serialize_unit_variant("Currency", 12u32, "SEK"),
                Self::Try => serializer.serialize_unit_variant("Currency", 13u32, "TRY"),
                Self::Gbp => serializer.serialize_unit_variant("Currency", 14u32, "GBP"),
                Self::Mxn => serializer.serialize_unit_variant("Currency", 15u32, "MXN"),
                Self::Myr => serializer.serialize_unit_variant("Currency", 16u32, "MYR"),
                Self::Inr => serializer.serialize_unit_variant("Currency", 17u32, "INR"),
                Self::Hkd => serializer.serialize_unit_variant("Currency", 18u32, "HKD"),
                Self::Brl => serializer.serialize_unit_variant("Currency", 19u32, "BRL"),
                Self::Twd => serializer.serialize_unit_variant("Currency", 20u32, "TWD"),
                Self::Eur => serializer.serialize_unit_variant("Currency", 21u32, "EUR"),
                Self::Chf => serializer.serialize_unit_variant("Currency", 22u32, "CHF"),
                Self::Ars => serializer.serialize_unit_variant("Currency", 23u32, "ARS"),
                Self::Aud => serializer.serialize_unit_variant("Currency", 24u32, "AUD"),
                Self::Cny => serializer.serialize_unit_variant("Currency", 25u32, "CNY"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "AHUB discount on windows virtual machines."]
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
    #[doc = "Assessment sizing criterion."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SizingCriterion")]
    pub enum SizingCriterion {
        PerformanceBased,
        AsOnPremises,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SizingCriterion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SizingCriterion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SizingCriterion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PerformanceBased => serializer.serialize_unit_variant("SizingCriterion", 0u32, "PerformanceBased"),
                Self::AsOnPremises => serializer.serialize_unit_variant("SizingCriterion", 1u32, "AsOnPremises"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether the assessment has been created and is valid."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Created,
        Updated,
        Running,
        Completed,
        Invalid,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Status {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Status {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Status {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Created => serializer.serialize_unit_variant("Status", 0u32, "Created"),
                Self::Updated => serializer.serialize_unit_variant("Status", 1u32, "Updated"),
                Self::Running => serializer.serialize_unit_variant("Status", 2u32, "Running"),
                Self::Completed => serializer.serialize_unit_variant("Status", 3u32, "Completed"),
                Self::Invalid => serializer.serialize_unit_variant("Status", 4u32, "Invalid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of assessments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessmentResultList {
    #[doc = "List of assessments."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Assessment>,
}
impl azure_core::Continuable for AssessmentResultList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AssessmentResultList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for a check name availability request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameters {
    #[doc = "The name to check for availability"]
    pub name: String,
    #[doc = "The resource type. Must be set to Microsoft.Migrate/projects"]
    #[serde(rename = "type")]
    pub type_: check_name_availability_parameters::Type,
}
impl CheckNameAvailabilityParameters {
    pub fn new(name: String, type_: check_name_availability_parameters::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod check_name_availability_parameters {
    use super::*;
    #[doc = "The resource type. Must be set to Microsoft.Migrate/projects"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Migrate/projects")]
        MicrosoftMigrateProjects,
    }
}
#[doc = "The CheckNameAvailability operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "Gets a boolean value that indicates whether the name is available for you to use. If true, the name is available. If false, the name has already been taken or invalid and cannot be used."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Gets the reason that a project name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_result::Reason>,
    #[doc = "Gets an error message explaining the Reason value in more detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_result {
    use super::*;
    #[doc = "Gets the reason that a project name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Available,
        Invalid,
        AlreadyExists,
    }
}
#[doc = "An error response from the Azure Migrate service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the Azure Migrate service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
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
#[doc = "An error response from the Azure Migrate service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error. For example, the name of the property in error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A disk discovered on a machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Disk {
    #[doc = "Gigabytes of storage provisioned for this disk."]
    #[serde(rename = "gigabytesAllocated", default, skip_serializing_if = "Option::is_none")]
    pub gigabytes_allocated: Option<f64>,
    #[doc = "Gigabytes of storage consumed by this disk."]
    #[serde(rename = "gigabytesConsumed", default, skip_serializing_if = "Option::is_none")]
    pub gigabytes_consumed: Option<f64>,
}
impl Disk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Download URL for assessment report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DownloadUrl {
    #[doc = "Hyperlink to download report."]
    #[serde(rename = "assessmentReportUrl", default, skip_serializing_if = "Option::is_none")]
    pub assessment_report_url: Option<String>,
    #[doc = "Expiry date of download url."]
    #[serde(rename = "expirationTime", with = "azure_core::date::rfc3339::option")]
    pub expiration_time: Option<time::OffsetDateTime>,
}
impl DownloadUrl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A group created in a Migration project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Group {
    #[doc = "Path reference to this group. /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Migrate/projects/{projectName}/groups/{groupName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "For optimistic concurrency control."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Type of the object = [Microsoft.Migrate/projects/groups]."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of group resource."]
    pub properties: GroupProperties,
}
impl Group {
    pub fn new(properties: GroupProperties) -> Self {
        Self {
            id: None,
            name: None,
            e_tag: None,
            type_: None,
            properties,
        }
    }
}
#[doc = "Properties of group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupProperties {
    #[doc = "List of machine names that are part of this group."]
    pub machines: Vec<String>,
    #[doc = "List of References to Assessments created on this group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assessments: Vec<String>,
    #[doc = "Time when this project was created. Date-Time represented in ISO-8601 format."]
    #[serde(rename = "createdTimestamp", with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Time when this project was last updated. Date-Time represented in ISO-8601 format."]
    #[serde(rename = "updatedTimestamp", with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
}
impl GroupProperties {
    pub fn new(machines: Vec<String>) -> Self {
        Self {
            machines,
            assessments: Vec::new(),
            created_timestamp: None,
            updated_timestamp: None,
        }
    }
}
#[doc = "List of groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupResultList {
    #[doc = "List of groups."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Group>,
}
impl azure_core::Continuable for GroupResultList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl GroupResultList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A machine in a migration project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Machine {
    #[doc = "Path reference to this machine. /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Migrate/projects/{projectName}/machines/{machineName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the machine. It is a GUID which is unique identifier of machine in private data center. For user-readable name, we have a displayName property on this machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "For optimistic concurrency control."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Type of the object = [Microsoft.Migrate/projects/machines]."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of a machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineProperties>,
}
impl Machine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineProperties {
    #[doc = "Boot type of the machine."]
    #[serde(rename = "bootType", default, skip_serializing_if = "Option::is_none")]
    pub boot_type: Option<machine_properties::BootType>,
    #[doc = "Container defined in the management solution that this machine is part of in the datacenter."]
    #[serde(rename = "datacenterContainer", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_container: Option<String>,
    #[doc = "Name of the server hosting the datacenter management solution."]
    #[serde(rename = "datacenterManagementServer", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_management_server: Option<String>,
    #[doc = "ID of the machine as tracked by the datacenter management solution."]
    #[serde(rename = "datacenterMachineId", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_machine_id: Option<String>,
    #[doc = "ID of the server hosting the datacenter management solution."]
    #[serde(rename = "datacenterManagementServerId", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_management_server_id: Option<String>,
    #[doc = "Description of the machine"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "User readable name of the machine as defined by the user in their private datacenter."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Memory in Megabytes."]
    #[serde(rename = "megabytesOfMemory", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_of_memory: Option<f64>,
    #[doc = "Processor count."]
    #[serde(rename = "numberOfCores", default, skip_serializing_if = "Option::is_none")]
    pub number_of_cores: Option<i64>,
    #[doc = "Operating System of the machine."]
    #[serde(rename = "operatingSystem", default, skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    #[doc = "List of references to the groups that the machine is member of."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    #[doc = "Time when this machine was created. Date-Time represented in ISO-8601 format."]
    #[serde(rename = "createdTimestamp", with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Time when this machine was last updated. Date-Time represented in ISO-8601 format."]
    #[serde(rename = "updatedTimestamp", with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Time when this machine was discovered by Azure Migrate agent. Date-Time represented in ISO-8601 format."]
    #[serde(rename = "discoveredTimestamp", with = "azure_core::date::rfc3339::option")]
    pub discovered_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Dictionary of disks attached to the machine. Key is ID of disk. Value is a disk object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disks: Option<serde_json::Value>,
    #[doc = "Dictionary of network adapters attached to the machine. Key is ID of network adapter. Value is a network adapter object"]
    #[serde(rename = "networkAdapters", default, skip_serializing_if = "Option::is_none")]
    pub network_adapters: Option<serde_json::Value>,
}
impl MachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod machine_properties {
    use super::*;
    #[doc = "Boot type of the machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BootType")]
    pub enum BootType {
        Unknown,
        #[serde(rename = "EFI")]
        Efi,
        #[serde(rename = "BIOS")]
        Bios,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BootType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BootType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BootType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("BootType", 0u32, "Unknown"),
                Self::Efi => serializer.serialize_unit_variant("BootType", 1u32, "EFI"),
                Self::Bios => serializer.serialize_unit_variant("BootType", 2u32, "BIOS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineResultList {
    #[doc = "List of machines."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Machine>,
}
impl azure_core::Continuable for MachineResultList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MachineResultList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A network adapter discovered on a machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkAdapter {
    #[doc = "MAC Address of the network adapter."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "List of IP Addresses on the network adapter."]
    #[serde(rename = "ipAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_addresses: Vec<String>,
}
impl NetworkAdapter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A REST API operation supported by the provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Displayable properties of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Origin of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Displayable properties of the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Provider of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource operated on by the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operation Type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of API operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResultList {
    #[doc = "List of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationResultList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationResultList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Migrate Project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Project {
    #[doc = "Path reference to this project /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Migrate/projects/{projectName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the object = [Microsoft.Migrate/projects]."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "For optimistic concurrency control."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Azure location in which project is created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Tags provided by Azure Tagging service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of a project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProjectProperties>,
}
impl Project {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ID and Key for Migration Project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectKey {
    #[doc = "ID of Migration Project."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "Key of Migration Project."]
    #[serde(rename = "workspaceKey", default, skip_serializing_if = "Option::is_none")]
    pub workspace_key: Option<String>,
}
impl ProjectKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectProperties {
    #[doc = "Time when this project was created. Date-Time represented in ISO-8601 format."]
    #[serde(rename = "createdTimestamp", with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Time when this project was last updated. Date-Time represented in ISO-8601 format."]
    #[serde(rename = "updatedTimestamp", with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Reports whether project is under discovery."]
    #[serde(rename = "discoveryStatus", default, skip_serializing_if = "Option::is_none")]
    pub discovery_status: Option<project_properties::DiscoveryStatus>,
    #[doc = "ARM ID of the Service Map workspace created by user."]
    #[serde(rename = "customerWorkspaceId", default, skip_serializing_if = "Option::is_none")]
    pub customer_workspace_id: Option<String>,
    #[doc = "Location of the Service Map workspace created by user."]
    #[serde(rename = "customerWorkspaceLocation", default, skip_serializing_if = "Option::is_none")]
    pub customer_workspace_location: Option<String>,
    #[doc = "Time when this project was created. Date-Time represented in ISO-8601 format. This value will be null until discovery is complete."]
    #[serde(rename = "lastDiscoveryTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_discovery_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Session id of the last discovery."]
    #[serde(rename = "lastDiscoverySessionId", default, skip_serializing_if = "Option::is_none")]
    pub last_discovery_session_id: Option<String>,
    #[doc = "Number of groups created in the project."]
    #[serde(rename = "numberOfGroups", default, skip_serializing_if = "Option::is_none")]
    pub number_of_groups: Option<i32>,
    #[doc = "Number of machines in the project."]
    #[serde(rename = "numberOfMachines", default, skip_serializing_if = "Option::is_none")]
    pub number_of_machines: Option<i32>,
    #[doc = "Number of assessments created in the project."]
    #[serde(rename = "numberOfAssessments", default, skip_serializing_if = "Option::is_none")]
    pub number_of_assessments: Option<i32>,
    #[doc = "Time when last assessment was created. Date-Time represented in ISO-8601 format. This value will be null until assessment is created."]
    #[serde(rename = "lastAssessmentTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_assessment_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Provisioning state of the project."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<project_properties::ProvisioningState>,
}
impl ProjectProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod project_properties {
    use super::*;
    #[doc = "Reports whether project is under discovery."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DiscoveryStatus")]
    pub enum DiscoveryStatus {
        Unknown,
        NotStarted,
        InProgress,
        Completed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DiscoveryStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DiscoveryStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DiscoveryStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("DiscoveryStatus", 0u32, "Unknown"),
                Self::NotStarted => serializer.serialize_unit_variant("DiscoveryStatus", 1u32, "NotStarted"),
                Self::InProgress => serializer.serialize_unit_variant("DiscoveryStatus", 2u32, "InProgress"),
                Self::Completed => serializer.serialize_unit_variant("DiscoveryStatus", 3u32, "Completed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning state of the project."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Accepted,
        Creating,
        Deleting,
        Failed,
        Moving,
        Succeeded,
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
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Moving"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Succeeded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of projects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectResultList {
    #[doc = "List of projects."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Project>,
}
impl azure_core::Continuable for ProjectResultList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ProjectResultList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VM family name, the list of targeted azure locations and the category of the family."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmFamily {
    #[doc = "Name of the VM family."]
    #[serde(rename = "familyName", default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[doc = "List of Azure regions."]
    #[serde(rename = "targetLocations", default, skip_serializing_if = "Vec::is_empty")]
    pub target_locations: Vec<String>,
    #[doc = "Category of the VM family."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<String>,
}
impl VmFamily {
    pub fn new() -> Self {
        Self::default()
    }
}
