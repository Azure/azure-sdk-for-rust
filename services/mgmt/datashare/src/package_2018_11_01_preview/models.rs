#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An ADLS Gen 1 file data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen1FileDataSet {
    #[serde(flatten)]
    pub data_set: DataSet,
    #[doc = "Properties of the ADLS Gen1 file data set."]
    pub properties: AdlsGen1FileProperties,
}
impl AdlsGen1FileDataSet {
    pub fn new(data_set: DataSet, properties: AdlsGen1FileProperties) -> Self {
        Self { data_set, properties }
    }
}
#[doc = "Properties of the ADLS Gen1 file data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen1FileProperties {
    #[doc = "The ADLS account name."]
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[doc = "Unique id for identifying a data set resource"]
    #[serde(rename = "dataSetId", default, skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[doc = "The file name in the ADLS account."]
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[doc = "The folder path within the ADLS account."]
    #[serde(rename = "folderPath")]
    pub folder_path: String,
    #[doc = "Resource group of ADLS account."]
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[doc = "Subscription id of ADLS account."]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}
impl AdlsGen1FileProperties {
    pub fn new(account_name: String, file_name: String, folder_path: String, resource_group: String, subscription_id: String) -> Self {
        Self {
            account_name,
            data_set_id: None,
            file_name,
            folder_path,
            resource_group,
            subscription_id,
        }
    }
}
#[doc = "An ADLS Gen 1 folder data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen1FolderDataSet {
    #[serde(flatten)]
    pub data_set: DataSet,
    #[doc = "Properties of the ADLS Gen1 folder data set."]
    pub properties: AdlsGen1FolderProperties,
}
impl AdlsGen1FolderDataSet {
    pub fn new(data_set: DataSet, properties: AdlsGen1FolderProperties) -> Self {
        Self { data_set, properties }
    }
}
#[doc = "Properties of the ADLS Gen1 folder data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen1FolderProperties {
    #[doc = "The ADLS account name."]
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[doc = "Unique id for identifying a data set resource"]
    #[serde(rename = "dataSetId", default, skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[doc = "The folder path within the ADLS account."]
    #[serde(rename = "folderPath")]
    pub folder_path: String,
    #[doc = "Resource group of ADLS account."]
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[doc = "Subscription id of ADLS account."]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}
impl AdlsGen1FolderProperties {
    pub fn new(account_name: String, folder_path: String, resource_group: String, subscription_id: String) -> Self {
        Self {
            account_name,
            data_set_id: None,
            folder_path,
            resource_group,
            subscription_id,
        }
    }
}
#[doc = "An ADLS Gen 2 file data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2FileDataSet {
    #[serde(flatten)]
    pub data_set: DataSet,
    #[doc = "Properties of the ADLS Gen2 file data set."]
    pub properties: AdlsGen2FileProperties,
}
impl AdlsGen2FileDataSet {
    pub fn new(data_set: DataSet, properties: AdlsGen2FileProperties) -> Self {
        Self { data_set, properties }
    }
}
#[doc = "An ADLS Gen2 file data set mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2FileDataSetMapping {
    #[serde(flatten)]
    pub data_set_mapping: DataSetMapping,
    #[doc = "ADLS Gen 2 file data set mapping property bag."]
    pub properties: AdlsGen2FileDataSetMappingProperties,
}
impl AdlsGen2FileDataSetMapping {
    pub fn new(data_set_mapping: DataSetMapping, properties: AdlsGen2FileDataSetMappingProperties) -> Self {
        Self {
            data_set_mapping,
            properties,
        }
    }
}
#[doc = "ADLS Gen 2 file data set mapping property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2FileDataSetMappingProperties {
    #[doc = "The id of the source data set."]
    #[serde(rename = "dataSetId")]
    pub data_set_id: String,
    #[doc = "Gets the status of the data set mapping."]
    #[serde(rename = "dataSetMappingStatus", default, skip_serializing_if = "Option::is_none")]
    pub data_set_mapping_status: Option<adls_gen2_file_data_set_mapping_properties::DataSetMappingStatus>,
    #[doc = "File path within the file system."]
    #[serde(rename = "filePath")]
    pub file_path: String,
    #[doc = "File system to which the file belongs."]
    #[serde(rename = "fileSystem")]
    pub file_system: String,
    #[doc = "Type of output file"]
    #[serde(rename = "outputType", default, skip_serializing_if = "Option::is_none")]
    pub output_type: Option<adls_gen2_file_data_set_mapping_properties::OutputType>,
    #[doc = "Provisioning state of the data set mapping."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<adls_gen2_file_data_set_mapping_properties::ProvisioningState>,
    #[doc = "Resource group of storage account."]
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[doc = "Storage account name of the source data set."]
    #[serde(rename = "storageAccountName")]
    pub storage_account_name: String,
    #[doc = "Subscription id of storage account."]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}
impl AdlsGen2FileDataSetMappingProperties {
    pub fn new(
        data_set_id: String,
        file_path: String,
        file_system: String,
        resource_group: String,
        storage_account_name: String,
        subscription_id: String,
    ) -> Self {
        Self {
            data_set_id,
            data_set_mapping_status: None,
            file_path,
            file_system,
            output_type: None,
            provisioning_state: None,
            resource_group,
            storage_account_name,
            subscription_id,
        }
    }
}
pub mod adls_gen2_file_data_set_mapping_properties {
    use super::*;
    #[doc = "Gets the status of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataSetMappingStatus")]
    pub enum DataSetMappingStatus {
        Ok,
        Broken,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataSetMappingStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataSetMappingStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataSetMappingStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ok => serializer.serialize_unit_variant("DataSetMappingStatus", 0u32, "Ok"),
                Self::Broken => serializer.serialize_unit_variant("DataSetMappingStatus", 1u32, "Broken"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of output file"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OutputType")]
    pub enum OutputType {
        Csv,
        Parquet,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OutputType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OutputType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OutputType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Csv => serializer.serialize_unit_variant("OutputType", 0u32, "Csv"),
                Self::Parquet => serializer.serialize_unit_variant("OutputType", 1u32, "Parquet"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning state of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Deleting,
        Moving,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the ADLS Gen2 file data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2FileProperties {
    #[doc = "Unique id for identifying a data set resource"]
    #[serde(rename = "dataSetId", default, skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[doc = "File path within the file system."]
    #[serde(rename = "filePath")]
    pub file_path: String,
    #[doc = "File system to which the file belongs."]
    #[serde(rename = "fileSystem")]
    pub file_system: String,
    #[doc = "Resource group of storage account"]
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[doc = "Storage account name of the source data set"]
    #[serde(rename = "storageAccountName")]
    pub storage_account_name: String,
    #[doc = "Subscription id of storage account"]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}
impl AdlsGen2FileProperties {
    pub fn new(
        file_path: String,
        file_system: String,
        resource_group: String,
        storage_account_name: String,
        subscription_id: String,
    ) -> Self {
        Self {
            data_set_id: None,
            file_path,
            file_system,
            resource_group,
            storage_account_name,
            subscription_id,
        }
    }
}
#[doc = "An ADLS Gen 2 file system data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2FileSystemDataSet {
    #[serde(flatten)]
    pub data_set: DataSet,
    #[doc = "Properties of the ADLS Gen2 file system data set."]
    pub properties: AdlsGen2FileSystemProperties,
}
impl AdlsGen2FileSystemDataSet {
    pub fn new(data_set: DataSet, properties: AdlsGen2FileSystemProperties) -> Self {
        Self { data_set, properties }
    }
}
#[doc = "An ADLS Gen2 file system data set mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2FileSystemDataSetMapping {
    #[serde(flatten)]
    pub data_set_mapping: DataSetMapping,
    #[doc = "ADLS Gen 2 file system data set mapping property bag."]
    pub properties: AdlsGen2FileSystemDataSetMappingProperties,
}
impl AdlsGen2FileSystemDataSetMapping {
    pub fn new(data_set_mapping: DataSetMapping, properties: AdlsGen2FileSystemDataSetMappingProperties) -> Self {
        Self {
            data_set_mapping,
            properties,
        }
    }
}
#[doc = "ADLS Gen 2 file system data set mapping property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2FileSystemDataSetMappingProperties {
    #[doc = "The id of the source data set."]
    #[serde(rename = "dataSetId")]
    pub data_set_id: String,
    #[doc = "Gets the status of the data set mapping."]
    #[serde(rename = "dataSetMappingStatus", default, skip_serializing_if = "Option::is_none")]
    pub data_set_mapping_status: Option<adls_gen2_file_system_data_set_mapping_properties::DataSetMappingStatus>,
    #[doc = "The file system name."]
    #[serde(rename = "fileSystem")]
    pub file_system: String,
    #[doc = "Provisioning state of the data set mapping."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<adls_gen2_file_system_data_set_mapping_properties::ProvisioningState>,
    #[doc = "Resource group of storage account."]
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[doc = "Storage account name of the source data set."]
    #[serde(rename = "storageAccountName")]
    pub storage_account_name: String,
    #[doc = "Subscription id of storage account."]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}
impl AdlsGen2FileSystemDataSetMappingProperties {
    pub fn new(
        data_set_id: String,
        file_system: String,
        resource_group: String,
        storage_account_name: String,
        subscription_id: String,
    ) -> Self {
        Self {
            data_set_id,
            data_set_mapping_status: None,
            file_system,
            provisioning_state: None,
            resource_group,
            storage_account_name,
            subscription_id,
        }
    }
}
pub mod adls_gen2_file_system_data_set_mapping_properties {
    use super::*;
    #[doc = "Gets the status of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataSetMappingStatus")]
    pub enum DataSetMappingStatus {
        Ok,
        Broken,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataSetMappingStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataSetMappingStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataSetMappingStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ok => serializer.serialize_unit_variant("DataSetMappingStatus", 0u32, "Ok"),
                Self::Broken => serializer.serialize_unit_variant("DataSetMappingStatus", 1u32, "Broken"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning state of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Deleting,
        Moving,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the ADLS Gen2 file system data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2FileSystemProperties {
    #[doc = "Unique id for identifying a data set resource"]
    #[serde(rename = "dataSetId", default, skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[doc = "The file system name."]
    #[serde(rename = "fileSystem")]
    pub file_system: String,
    #[doc = "Resource group of storage account"]
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[doc = "Storage account name of the source data set"]
    #[serde(rename = "storageAccountName")]
    pub storage_account_name: String,
    #[doc = "Subscription id of storage account"]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}
impl AdlsGen2FileSystemProperties {
    pub fn new(file_system: String, resource_group: String, storage_account_name: String, subscription_id: String) -> Self {
        Self {
            data_set_id: None,
            file_system,
            resource_group,
            storage_account_name,
            subscription_id,
        }
    }
}
#[doc = "An ADLS Gen 2 folder data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2FolderDataSet {
    #[serde(flatten)]
    pub data_set: DataSet,
    #[doc = "Properties of the ADLS Gen2 folder data set."]
    pub properties: AdlsGen2FolderProperties,
}
impl AdlsGen2FolderDataSet {
    pub fn new(data_set: DataSet, properties: AdlsGen2FolderProperties) -> Self {
        Self { data_set, properties }
    }
}
#[doc = "An ADLS Gen2 folder data set mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2FolderDataSetMapping {
    #[serde(flatten)]
    pub data_set_mapping: DataSetMapping,
    #[doc = "ADLS Gen 2 folder data set mapping property bag."]
    pub properties: AdlsGen2FolderDataSetMappingProperties,
}
impl AdlsGen2FolderDataSetMapping {
    pub fn new(data_set_mapping: DataSetMapping, properties: AdlsGen2FolderDataSetMappingProperties) -> Self {
        Self {
            data_set_mapping,
            properties,
        }
    }
}
#[doc = "ADLS Gen 2 folder data set mapping property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2FolderDataSetMappingProperties {
    #[doc = "The id of the source data set."]
    #[serde(rename = "dataSetId")]
    pub data_set_id: String,
    #[doc = "Gets the status of the data set mapping."]
    #[serde(rename = "dataSetMappingStatus", default, skip_serializing_if = "Option::is_none")]
    pub data_set_mapping_status: Option<adls_gen2_folder_data_set_mapping_properties::DataSetMappingStatus>,
    #[doc = "File system to which the folder belongs."]
    #[serde(rename = "fileSystem")]
    pub file_system: String,
    #[doc = "Folder path within the file system."]
    #[serde(rename = "folderPath")]
    pub folder_path: String,
    #[doc = "Provisioning state of the data set mapping."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<adls_gen2_folder_data_set_mapping_properties::ProvisioningState>,
    #[doc = "Resource group of storage account."]
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[doc = "Storage account name of the source data set."]
    #[serde(rename = "storageAccountName")]
    pub storage_account_name: String,
    #[doc = "Subscription id of storage account."]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}
impl AdlsGen2FolderDataSetMappingProperties {
    pub fn new(
        data_set_id: String,
        file_system: String,
        folder_path: String,
        resource_group: String,
        storage_account_name: String,
        subscription_id: String,
    ) -> Self {
        Self {
            data_set_id,
            data_set_mapping_status: None,
            file_system,
            folder_path,
            provisioning_state: None,
            resource_group,
            storage_account_name,
            subscription_id,
        }
    }
}
pub mod adls_gen2_folder_data_set_mapping_properties {
    use super::*;
    #[doc = "Gets the status of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataSetMappingStatus")]
    pub enum DataSetMappingStatus {
        Ok,
        Broken,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataSetMappingStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataSetMappingStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataSetMappingStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ok => serializer.serialize_unit_variant("DataSetMappingStatus", 0u32, "Ok"),
                Self::Broken => serializer.serialize_unit_variant("DataSetMappingStatus", 1u32, "Broken"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning state of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Deleting,
        Moving,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the ADLS Gen2 folder data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2FolderProperties {
    #[doc = "Unique id for identifying a data set resource"]
    #[serde(rename = "dataSetId", default, skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[doc = "File system to which the folder belongs."]
    #[serde(rename = "fileSystem")]
    pub file_system: String,
    #[doc = "Folder path within the file system."]
    #[serde(rename = "folderPath")]
    pub folder_path: String,
    #[doc = "Resource group of storage account"]
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[doc = "Storage account name of the source data set"]
    #[serde(rename = "storageAccountName")]
    pub storage_account_name: String,
    #[doc = "Subscription id of storage account"]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}
impl AdlsGen2FolderProperties {
    pub fn new(
        file_system: String,
        folder_path: String,
        resource_group: String,
        storage_account_name: String,
        subscription_id: String,
    ) -> Self {
        Self {
            data_set_id: None,
            file_system,
            folder_path,
            resource_group,
            storage_account_name,
            subscription_id,
        }
    }
}
#[doc = "An account data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    #[serde(flatten)]
    pub default_dto: DefaultDto,
    #[doc = "Identity of resource"]
    pub identity: Identity,
    #[doc = "Account property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountProperties>,
}
impl Account {
    pub fn new(identity: Identity) -> Self {
        Self {
            default_dto: DefaultDto::default(),
            identity,
            properties: None,
        }
    }
}
#[doc = "List response for get Accounts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<Account>,
}
impl azure_core::Continuable for AccountList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AccountList {
    pub fn new(value: Vec<Account>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Account property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountProperties {
    #[doc = "Time at which the account was created."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "Provisioning state of the Account"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<account_properties::ProvisioningState>,
    #[doc = "Email of the user who created the resource"]
    #[serde(rename = "userEmail", default, skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    #[doc = "Name of the user who created the resource"]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}
impl AccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod account_properties {
    use super::*;
    #[doc = "Provisioning state of the Account"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Deleting,
        Moving,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Update parameters for accounts"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountUpdateParameters {
    #[doc = "Tags on the azure resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AccountUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure storage blob container data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobContainerDataSet {
    #[serde(flatten)]
    pub data_set: DataSet,
    #[doc = "Properties of the BLOB container data set."]
    pub properties: BlobContainerProperties,
}
impl BlobContainerDataSet {
    pub fn new(data_set: DataSet, properties: BlobContainerProperties) -> Self {
        Self { data_set, properties }
    }
}
#[doc = "A Blob container data set mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobContainerDataSetMapping {
    #[serde(flatten)]
    pub data_set_mapping: DataSetMapping,
    #[doc = "Azure storage Blob container data set mapping property bag."]
    pub properties: BlobContainerMappingProperties,
}
impl BlobContainerDataSetMapping {
    pub fn new(data_set_mapping: DataSetMapping, properties: BlobContainerMappingProperties) -> Self {
        Self {
            data_set_mapping,
            properties,
        }
    }
}
#[doc = "Azure storage Blob container data set mapping property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobContainerMappingProperties {
    #[doc = "BLOB Container name."]
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "The id of the source data set."]
    #[serde(rename = "dataSetId")]
    pub data_set_id: String,
    #[doc = "Gets the status of the data set mapping."]
    #[serde(rename = "dataSetMappingStatus", default, skip_serializing_if = "Option::is_none")]
    pub data_set_mapping_status: Option<blob_container_mapping_properties::DataSetMappingStatus>,
    #[doc = "Provisioning state of the data set mapping."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<blob_container_mapping_properties::ProvisioningState>,
    #[doc = "Resource group of storage account."]
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[doc = "Storage account name of the source data set."]
    #[serde(rename = "storageAccountName")]
    pub storage_account_name: String,
    #[doc = "Subscription id of storage account."]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}
impl BlobContainerMappingProperties {
    pub fn new(
        container_name: String,
        data_set_id: String,
        resource_group: String,
        storage_account_name: String,
        subscription_id: String,
    ) -> Self {
        Self {
            container_name,
            data_set_id,
            data_set_mapping_status: None,
            provisioning_state: None,
            resource_group,
            storage_account_name,
            subscription_id,
        }
    }
}
pub mod blob_container_mapping_properties {
    use super::*;
    #[doc = "Gets the status of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataSetMappingStatus")]
    pub enum DataSetMappingStatus {
        Ok,
        Broken,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataSetMappingStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataSetMappingStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataSetMappingStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ok => serializer.serialize_unit_variant("DataSetMappingStatus", 0u32, "Ok"),
                Self::Broken => serializer.serialize_unit_variant("DataSetMappingStatus", 1u32, "Broken"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning state of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Deleting,
        Moving,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the BLOB container data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobContainerProperties {
    #[doc = "BLOB Container name."]
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "Unique id for identifying a data set resource"]
    #[serde(rename = "dataSetId", default, skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[doc = "Resource group of storage account"]
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[doc = "Storage account name of the source data set"]
    #[serde(rename = "storageAccountName")]
    pub storage_account_name: String,
    #[doc = "Subscription id of storage account"]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}
impl BlobContainerProperties {
    pub fn new(container_name: String, resource_group: String, storage_account_name: String, subscription_id: String) -> Self {
        Self {
            container_name,
            data_set_id: None,
            resource_group,
            storage_account_name,
            subscription_id,
        }
    }
}
#[doc = "An Azure storage blob data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobDataSet {
    #[serde(flatten)]
    pub data_set: DataSet,
    #[doc = "Properties of the blob data set."]
    pub properties: BlobProperties,
}
impl BlobDataSet {
    pub fn new(data_set: DataSet, properties: BlobProperties) -> Self {
        Self { data_set, properties }
    }
}
#[doc = "A Blob data set mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobDataSetMapping {
    #[serde(flatten)]
    pub data_set_mapping: DataSetMapping,
    #[doc = "Azure storage Blob data set mapping property bag."]
    pub properties: BlobMappingProperties,
}
impl BlobDataSetMapping {
    pub fn new(data_set_mapping: DataSetMapping, properties: BlobMappingProperties) -> Self {
        Self {
            data_set_mapping,
            properties,
        }
    }
}
#[doc = "An Azure storage blob folder data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobFolderDataSet {
    #[serde(flatten)]
    pub data_set: DataSet,
    #[doc = "Properties of the blob folder data set."]
    pub properties: BlobFolderProperties,
}
impl BlobFolderDataSet {
    pub fn new(data_set: DataSet, properties: BlobFolderProperties) -> Self {
        Self { data_set, properties }
    }
}
#[doc = "A Blob folder data set mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobFolderDataSetMapping {
    #[serde(flatten)]
    pub data_set_mapping: DataSetMapping,
    #[doc = "Azure storage Blob folder data set mapping property bag."]
    pub properties: BlobFolderMappingProperties,
}
impl BlobFolderDataSetMapping {
    pub fn new(data_set_mapping: DataSetMapping, properties: BlobFolderMappingProperties) -> Self {
        Self {
            data_set_mapping,
            properties,
        }
    }
}
#[doc = "Azure storage Blob folder data set mapping property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobFolderMappingProperties {
    #[doc = "Container that has the file path."]
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "The id of the source data set."]
    #[serde(rename = "dataSetId")]
    pub data_set_id: String,
    #[doc = "Gets the status of the data set mapping."]
    #[serde(rename = "dataSetMappingStatus", default, skip_serializing_if = "Option::is_none")]
    pub data_set_mapping_status: Option<blob_folder_mapping_properties::DataSetMappingStatus>,
    #[doc = "Prefix for blob folder"]
    pub prefix: String,
    #[doc = "Provisioning state of the data set mapping."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<blob_folder_mapping_properties::ProvisioningState>,
    #[doc = "Resource group of storage account."]
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[doc = "Storage account name of the source data set."]
    #[serde(rename = "storageAccountName")]
    pub storage_account_name: String,
    #[doc = "Subscription id of storage account."]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}
impl BlobFolderMappingProperties {
    pub fn new(
        container_name: String,
        data_set_id: String,
        prefix: String,
        resource_group: String,
        storage_account_name: String,
        subscription_id: String,
    ) -> Self {
        Self {
            container_name,
            data_set_id,
            data_set_mapping_status: None,
            prefix,
            provisioning_state: None,
            resource_group,
            storage_account_name,
            subscription_id,
        }
    }
}
pub mod blob_folder_mapping_properties {
    use super::*;
    #[doc = "Gets the status of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataSetMappingStatus")]
    pub enum DataSetMappingStatus {
        Ok,
        Broken,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataSetMappingStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataSetMappingStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataSetMappingStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ok => serializer.serialize_unit_variant("DataSetMappingStatus", 0u32, "Ok"),
                Self::Broken => serializer.serialize_unit_variant("DataSetMappingStatus", 1u32, "Broken"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning state of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Deleting,
        Moving,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the blob folder data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobFolderProperties {
    #[doc = "Container that has the file path."]
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "Unique id for identifying a data set resource"]
    #[serde(rename = "dataSetId", default, skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[doc = "Prefix for blob folder"]
    pub prefix: String,
    #[doc = "Resource group of storage account"]
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[doc = "Storage account name of the source data set"]
    #[serde(rename = "storageAccountName")]
    pub storage_account_name: String,
    #[doc = "Subscription id of storage account"]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}
impl BlobFolderProperties {
    pub fn new(
        container_name: String,
        prefix: String,
        resource_group: String,
        storage_account_name: String,
        subscription_id: String,
    ) -> Self {
        Self {
            container_name,
            data_set_id: None,
            prefix,
            resource_group,
            storage_account_name,
            subscription_id,
        }
    }
}
#[doc = "Azure storage Blob data set mapping property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobMappingProperties {
    #[doc = "Container that has the file path."]
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "The id of the source data set."]
    #[serde(rename = "dataSetId")]
    pub data_set_id: String,
    #[doc = "Gets the status of the data set mapping."]
    #[serde(rename = "dataSetMappingStatus", default, skip_serializing_if = "Option::is_none")]
    pub data_set_mapping_status: Option<blob_mapping_properties::DataSetMappingStatus>,
    #[doc = "File path within the source data set"]
    #[serde(rename = "filePath")]
    pub file_path: String,
    #[doc = "File output type"]
    #[serde(rename = "outputType", default, skip_serializing_if = "Option::is_none")]
    pub output_type: Option<blob_mapping_properties::OutputType>,
    #[doc = "Provisioning state of the data set mapping."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<blob_mapping_properties::ProvisioningState>,
    #[doc = "Resource group of storage account."]
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[doc = "Storage account name of the source data set."]
    #[serde(rename = "storageAccountName")]
    pub storage_account_name: String,
    #[doc = "Subscription id of storage account."]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}
impl BlobMappingProperties {
    pub fn new(
        container_name: String,
        data_set_id: String,
        file_path: String,
        resource_group: String,
        storage_account_name: String,
        subscription_id: String,
    ) -> Self {
        Self {
            container_name,
            data_set_id,
            data_set_mapping_status: None,
            file_path,
            output_type: None,
            provisioning_state: None,
            resource_group,
            storage_account_name,
            subscription_id,
        }
    }
}
pub mod blob_mapping_properties {
    use super::*;
    #[doc = "Gets the status of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataSetMappingStatus")]
    pub enum DataSetMappingStatus {
        Ok,
        Broken,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataSetMappingStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataSetMappingStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataSetMappingStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ok => serializer.serialize_unit_variant("DataSetMappingStatus", 0u32, "Ok"),
                Self::Broken => serializer.serialize_unit_variant("DataSetMappingStatus", 1u32, "Broken"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "File output type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OutputType")]
    pub enum OutputType {
        Csv,
        Parquet,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OutputType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OutputType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OutputType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Csv => serializer.serialize_unit_variant("OutputType", 0u32, "Csv"),
                Self::Parquet => serializer.serialize_unit_variant("OutputType", 1u32, "Parquet"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning state of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Deleting,
        Moving,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the blob data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobProperties {
    #[doc = "Container that has the file path."]
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "Unique id for identifying a data set resource"]
    #[serde(rename = "dataSetId", default, skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[doc = "File path within the source data set"]
    #[serde(rename = "filePath")]
    pub file_path: String,
    #[doc = "Resource group of storage account"]
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[doc = "Storage account name of the source data set"]
    #[serde(rename = "storageAccountName")]
    pub storage_account_name: String,
    #[doc = "Subscription id of storage account"]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}
impl BlobProperties {
    pub fn new(
        container_name: String,
        file_path: String,
        resource_group: String,
        storage_account_name: String,
        subscription_id: String,
    ) -> Self {
        Self {
            container_name,
            data_set_id: None,
            file_path,
            resource_group,
            storage_account_name,
            subscription_id,
        }
    }
}
#[doc = "A consumer Invitation data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsumerInvitation {
    #[serde(flatten)]
    pub proxy_dto: ProxyDto,
    #[doc = "Properties of consumer invitation"]
    pub properties: ConsumerInvitationProperties,
}
impl ConsumerInvitation {
    pub fn new(properties: ConsumerInvitationProperties) -> Self {
        Self {
            proxy_dto: ProxyDto::default(),
            properties,
        }
    }
}
#[doc = "List response for get InvitationList"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsumerInvitationList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<ConsumerInvitation>,
}
impl azure_core::Continuable for ConsumerInvitationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ConsumerInvitationList {
    pub fn new(value: Vec<ConsumerInvitation>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Properties of consumer invitation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsumerInvitationProperties {
    #[doc = "Number of data sets in a share"]
    #[serde(rename = "dataSetCount", default, skip_serializing_if = "Option::is_none")]
    pub data_set_count: Option<i32>,
    #[doc = "Description shared when the invitation was created"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Unique id of the invitation."]
    #[serde(rename = "invitationId")]
    pub invitation_id: String,
    #[doc = "The status of the invitation."]
    #[serde(rename = "invitationStatus", default, skip_serializing_if = "Option::is_none")]
    pub invitation_status: Option<consumer_invitation_properties::InvitationStatus>,
    #[doc = "invitation location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Email of the provider who created the resource"]
    #[serde(rename = "providerEmail", default, skip_serializing_if = "Option::is_none")]
    pub provider_email: Option<String>,
    #[doc = "Name of the provider who created the resource"]
    #[serde(rename = "providerName", default, skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[doc = "Tenant name of the provider who created the resource"]
    #[serde(rename = "providerTenantName", default, skip_serializing_if = "Option::is_none")]
    pub provider_tenant_name: Option<String>,
    #[doc = "The time the recipient responded to the invitation."]
    #[serde(rename = "respondedAt", default, with = "azure_core::date::rfc3339::option")]
    pub responded_at: Option<time::OffsetDateTime>,
    #[doc = "Gets the time at which the invitation was sent."]
    #[serde(rename = "sentAt", default, with = "azure_core::date::rfc3339::option")]
    pub sent_at: Option<time::OffsetDateTime>,
    #[doc = "Gets the source share Name."]
    #[serde(rename = "shareName", default, skip_serializing_if = "Option::is_none")]
    pub share_name: Option<String>,
    #[doc = "Terms of use shared when the invitation was created"]
    #[serde(rename = "termsOfUse", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_use: Option<String>,
    #[doc = "Email of the user who created the resource"]
    #[serde(rename = "userEmail", default, skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    #[doc = "Name of the user who created the resource"]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}
impl ConsumerInvitationProperties {
    pub fn new(invitation_id: String) -> Self {
        Self {
            data_set_count: None,
            description: None,
            invitation_id,
            invitation_status: None,
            location: None,
            provider_email: None,
            provider_name: None,
            provider_tenant_name: None,
            responded_at: None,
            sent_at: None,
            share_name: None,
            terms_of_use: None,
            user_email: None,
            user_name: None,
        }
    }
}
pub mod consumer_invitation_properties {
    use super::*;
    #[doc = "The status of the invitation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InvitationStatus")]
    pub enum InvitationStatus {
        Pending,
        Accepted,
        Rejected,
        Withdrawn,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InvitationStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InvitationStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InvitationStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pending => serializer.serialize_unit_variant("InvitationStatus", 0u32, "Pending"),
                Self::Accepted => serializer.serialize_unit_variant("InvitationStatus", 1u32, "Accepted"),
                Self::Rejected => serializer.serialize_unit_variant("InvitationStatus", 2u32, "Rejected"),
                Self::Withdrawn => serializer.serialize_unit_variant("InvitationStatus", 3u32, "Withdrawn"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A consumer side dataSet data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsumerSourceDataSet {
    #[serde(flatten)]
    pub proxy_dto: ProxyDto,
    #[doc = "Properties of consumer source dataSet"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConsumerSourceDataSetProperties>,
}
impl ConsumerSourceDataSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A consumer side list of source dataSets"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsumerSourceDataSetList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<ConsumerSourceDataSet>,
}
impl azure_core::Continuable for ConsumerSourceDataSetList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ConsumerSourceDataSetList {
    pub fn new(value: Vec<ConsumerSourceDataSet>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Properties of consumer source dataSet"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsumerSourceDataSetProperties {
    #[doc = "DataSet Id"]
    #[serde(rename = "dataSetId", default, skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[doc = "Location of the data set."]
    #[serde(rename = "dataSetLocation", default, skip_serializing_if = "Option::is_none")]
    pub data_set_location: Option<String>,
    #[doc = "DataSet name"]
    #[serde(rename = "dataSetName", default, skip_serializing_if = "Option::is_none")]
    pub data_set_name: Option<String>,
    #[doc = "DataSet path"]
    #[serde(rename = "dataSetPath", default, skip_serializing_if = "Option::is_none")]
    pub data_set_path: Option<String>,
    #[doc = "Type of data set"]
    #[serde(rename = "dataSetType", default, skip_serializing_if = "Option::is_none")]
    pub data_set_type: Option<consumer_source_data_set_properties::DataSetType>,
}
impl ConsumerSourceDataSetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod consumer_source_data_set_properties {
    use super::*;
    #[doc = "Type of data set"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataSetType")]
    pub enum DataSetType {
        Blob,
        Container,
        BlobFolder,
        AdlsGen2FileSystem,
        AdlsGen2Folder,
        AdlsGen2File,
        AdlsGen1Folder,
        AdlsGen1File,
        KustoCluster,
        KustoDatabase,
        #[serde(rename = "SqlDBTable")]
        SqlDbTable,
        #[serde(rename = "SqlDWTable")]
        SqlDwTable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataSetType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataSetType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataSetType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Blob => serializer.serialize_unit_variant("DataSetType", 0u32, "Blob"),
                Self::Container => serializer.serialize_unit_variant("DataSetType", 1u32, "Container"),
                Self::BlobFolder => serializer.serialize_unit_variant("DataSetType", 2u32, "BlobFolder"),
                Self::AdlsGen2FileSystem => serializer.serialize_unit_variant("DataSetType", 3u32, "AdlsGen2FileSystem"),
                Self::AdlsGen2Folder => serializer.serialize_unit_variant("DataSetType", 4u32, "AdlsGen2Folder"),
                Self::AdlsGen2File => serializer.serialize_unit_variant("DataSetType", 5u32, "AdlsGen2File"),
                Self::AdlsGen1Folder => serializer.serialize_unit_variant("DataSetType", 6u32, "AdlsGen1Folder"),
                Self::AdlsGen1File => serializer.serialize_unit_variant("DataSetType", 7u32, "AdlsGen1File"),
                Self::KustoCluster => serializer.serialize_unit_variant("DataSetType", 8u32, "KustoCluster"),
                Self::KustoDatabase => serializer.serialize_unit_variant("DataSetType", 9u32, "KustoDatabase"),
                Self::SqlDbTable => serializer.serialize_unit_variant("DataSetType", 10u32, "SqlDBTable"),
                Self::SqlDwTable => serializer.serialize_unit_variant("DataSetType", 11u32, "SqlDWTable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A DataSet data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSet {
    #[serde(flatten)]
    pub proxy_dto: ProxyDto,
    #[doc = "Kind of data set."]
    pub kind: data_set::Kind,
}
impl DataSet {
    pub fn new(kind: data_set::Kind) -> Self {
        Self {
            proxy_dto: ProxyDto::default(),
            kind,
        }
    }
}
pub mod data_set {
    use super::*;
    #[doc = "Kind of data set."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Blob,
        Container,
        BlobFolder,
        AdlsGen2FileSystem,
        AdlsGen2Folder,
        AdlsGen2File,
        AdlsGen1Folder,
        AdlsGen1File,
        KustoCluster,
        KustoDatabase,
        #[serde(rename = "SqlDBTable")]
        SqlDbTable,
        #[serde(rename = "SqlDWTable")]
        SqlDwTable,
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
                Self::Blob => serializer.serialize_unit_variant("Kind", 0u32, "Blob"),
                Self::Container => serializer.serialize_unit_variant("Kind", 1u32, "Container"),
                Self::BlobFolder => serializer.serialize_unit_variant("Kind", 2u32, "BlobFolder"),
                Self::AdlsGen2FileSystem => serializer.serialize_unit_variant("Kind", 3u32, "AdlsGen2FileSystem"),
                Self::AdlsGen2Folder => serializer.serialize_unit_variant("Kind", 4u32, "AdlsGen2Folder"),
                Self::AdlsGen2File => serializer.serialize_unit_variant("Kind", 5u32, "AdlsGen2File"),
                Self::AdlsGen1Folder => serializer.serialize_unit_variant("Kind", 6u32, "AdlsGen1Folder"),
                Self::AdlsGen1File => serializer.serialize_unit_variant("Kind", 7u32, "AdlsGen1File"),
                Self::KustoCluster => serializer.serialize_unit_variant("Kind", 8u32, "KustoCluster"),
                Self::KustoDatabase => serializer.serialize_unit_variant("Kind", 9u32, "KustoDatabase"),
                Self::SqlDbTable => serializer.serialize_unit_variant("Kind", 10u32, "SqlDBTable"),
                Self::SqlDwTable => serializer.serialize_unit_variant("Kind", 11u32, "SqlDWTable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List response for get DataSets"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSetList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<DataSet>,
}
impl azure_core::Continuable for DataSetList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataSetList {
    pub fn new(value: Vec<DataSet>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "A data set mapping data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSetMapping {
    #[serde(flatten)]
    pub proxy_dto: ProxyDto,
    #[doc = "Kind of data set mapping."]
    pub kind: data_set_mapping::Kind,
}
impl DataSetMapping {
    pub fn new(kind: data_set_mapping::Kind) -> Self {
        Self {
            proxy_dto: ProxyDto::default(),
            kind,
        }
    }
}
pub mod data_set_mapping {
    use super::*;
    #[doc = "Kind of data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Blob,
        Container,
        BlobFolder,
        AdlsGen2FileSystem,
        AdlsGen2Folder,
        AdlsGen2File,
        KustoCluster,
        KustoDatabase,
        #[serde(rename = "SqlDBTable")]
        SqlDbTable,
        #[serde(rename = "SqlDWTable")]
        SqlDwTable,
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
                Self::Blob => serializer.serialize_unit_variant("Kind", 0u32, "Blob"),
                Self::Container => serializer.serialize_unit_variant("Kind", 1u32, "Container"),
                Self::BlobFolder => serializer.serialize_unit_variant("Kind", 2u32, "BlobFolder"),
                Self::AdlsGen2FileSystem => serializer.serialize_unit_variant("Kind", 3u32, "AdlsGen2FileSystem"),
                Self::AdlsGen2Folder => serializer.serialize_unit_variant("Kind", 4u32, "AdlsGen2Folder"),
                Self::AdlsGen2File => serializer.serialize_unit_variant("Kind", 5u32, "AdlsGen2File"),
                Self::KustoCluster => serializer.serialize_unit_variant("Kind", 6u32, "KustoCluster"),
                Self::KustoDatabase => serializer.serialize_unit_variant("Kind", 7u32, "KustoDatabase"),
                Self::SqlDbTable => serializer.serialize_unit_variant("Kind", 8u32, "SqlDBTable"),
                Self::SqlDwTable => serializer.serialize_unit_variant("Kind", 9u32, "SqlDWTable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List response for get DataSetMappings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSetMappingList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<DataSetMapping>,
}
impl azure_core::Continuable for DataSetMappingList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataSetMappingList {
    pub fn new(value: Vec<DataSetMapping>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "The data share error model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataShareError {
    #[doc = "The data share error body model."]
    pub error: DataShareErrorInfo,
}
impl azure_core::Continuable for DataShareError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DataShareError {
    pub fn new(error: DataShareErrorInfo) -> Self {
        Self { error }
    }
}
#[doc = "The data share error body model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataShareErrorInfo {
    #[doc = "Code of the error"]
    pub code: String,
    #[doc = "Nested details of the error model"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<DataShareErrorInfo>,
    #[doc = "Message of the error"]
    pub message: String,
    #[doc = "Target of the error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl DataShareErrorInfo {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            details: Vec::new(),
            message,
            target: None,
        }
    }
}
#[doc = "Base data transfer object implementation for default resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefaultDto {
    #[serde(flatten)]
    pub proxy_dto: ProxyDto,
    #[doc = "Location of the azure resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Tags on the azure resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DefaultDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "properties for dimension"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DimensionProperties {
    #[doc = "localized display name of the dimension to customer"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "dimension name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl DimensionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity of resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[doc = "service principal Id"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Tenant Id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Identity Type"]
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
    #[doc = "Identity Type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        SystemAssigned,
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
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 0u32, "SystemAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A Invitation data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Invitation {
    #[serde(flatten)]
    pub proxy_dto: ProxyDto,
    #[doc = "Invitation property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InvitationProperties>,
}
impl Invitation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List response for get InvitationList"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvitationList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<Invitation>,
}
impl azure_core::Continuable for InvitationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl InvitationList {
    pub fn new(value: Vec<Invitation>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Invitation property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvitationProperties {
    #[doc = "unique invitation id"]
    #[serde(rename = "invitationId", default, skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    #[doc = "The status of the invitation."]
    #[serde(rename = "invitationStatus", default, skip_serializing_if = "Option::is_none")]
    pub invitation_status: Option<invitation_properties::InvitationStatus>,
    #[doc = "The time the recipient responded to the invitation."]
    #[serde(rename = "respondedAt", default, with = "azure_core::date::rfc3339::option")]
    pub responded_at: Option<time::OffsetDateTime>,
    #[doc = "Gets the time at which the invitation was sent."]
    #[serde(rename = "sentAt", default, with = "azure_core::date::rfc3339::option")]
    pub sent_at: Option<time::OffsetDateTime>,
    #[doc = "The target Azure AD Id. Can't be combined with email."]
    #[serde(rename = "targetActiveDirectoryId", default, skip_serializing_if = "Option::is_none")]
    pub target_active_directory_id: Option<String>,
    #[doc = "The email the invitation is directed to."]
    #[serde(rename = "targetEmail", default, skip_serializing_if = "Option::is_none")]
    pub target_email: Option<String>,
    #[doc = "The target user or application Id that invitation is being sent to.\r\nMust be specified along TargetActiveDirectoryId. This enables sending\r\ninvitations to specific users or applications in an AD tenant."]
    #[serde(rename = "targetObjectId", default, skip_serializing_if = "Option::is_none")]
    pub target_object_id: Option<String>,
    #[doc = "Email of the user who created the resource"]
    #[serde(rename = "userEmail", default, skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    #[doc = "Name of the user who created the resource"]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}
impl InvitationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod invitation_properties {
    use super::*;
    #[doc = "The status of the invitation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InvitationStatus")]
    pub enum InvitationStatus {
        Pending,
        Accepted,
        Rejected,
        Withdrawn,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InvitationStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InvitationStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InvitationStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pending => serializer.serialize_unit_variant("InvitationStatus", 0u32, "Pending"),
                Self::Accepted => serializer.serialize_unit_variant("InvitationStatus", 1u32, "Accepted"),
                Self::Rejected => serializer.serialize_unit_variant("InvitationStatus", 2u32, "Rejected"),
                Self::Withdrawn => serializer.serialize_unit_variant("InvitationStatus", 3u32, "Withdrawn"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A kusto cluster data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KustoClusterDataSet {
    #[serde(flatten)]
    pub data_set: DataSet,
    #[doc = "Properties of the kusto cluster data set."]
    pub properties: KustoClusterDataSetProperties,
}
impl KustoClusterDataSet {
    pub fn new(data_set: DataSet, properties: KustoClusterDataSetProperties) -> Self {
        Self { data_set, properties }
    }
}
#[doc = "A Kusto cluster data set mapping"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KustoClusterDataSetMapping {
    #[serde(flatten)]
    pub data_set_mapping: DataSetMapping,
    #[doc = "Properties of the Kusto cluster data set mapping"]
    pub properties: KustoClusterDataSetMappingProperties,
}
impl KustoClusterDataSetMapping {
    pub fn new(data_set_mapping: DataSetMapping, properties: KustoClusterDataSetMappingProperties) -> Self {
        Self {
            data_set_mapping,
            properties,
        }
    }
}
#[doc = "Properties of the Kusto cluster data set mapping"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KustoClusterDataSetMappingProperties {
    #[doc = "The id of the source data set."]
    #[serde(rename = "dataSetId")]
    pub data_set_id: String,
    #[doc = "Gets the status of the data set mapping."]
    #[serde(rename = "dataSetMappingStatus", default, skip_serializing_if = "Option::is_none")]
    pub data_set_mapping_status: Option<kusto_cluster_data_set_mapping_properties::DataSetMappingStatus>,
    #[doc = "Resource id of the sink kusto cluster."]
    #[serde(rename = "kustoClusterResourceId")]
    pub kusto_cluster_resource_id: String,
    #[doc = "Location of the sink kusto cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Provisioning state of the data set mapping."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<kusto_cluster_data_set_mapping_properties::ProvisioningState>,
}
impl KustoClusterDataSetMappingProperties {
    pub fn new(data_set_id: String, kusto_cluster_resource_id: String) -> Self {
        Self {
            data_set_id,
            data_set_mapping_status: None,
            kusto_cluster_resource_id,
            location: None,
            provisioning_state: None,
        }
    }
}
pub mod kusto_cluster_data_set_mapping_properties {
    use super::*;
    #[doc = "Gets the status of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataSetMappingStatus")]
    pub enum DataSetMappingStatus {
        Ok,
        Broken,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataSetMappingStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataSetMappingStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataSetMappingStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ok => serializer.serialize_unit_variant("DataSetMappingStatus", 0u32, "Ok"),
                Self::Broken => serializer.serialize_unit_variant("DataSetMappingStatus", 1u32, "Broken"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning state of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Deleting,
        Moving,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the kusto cluster data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KustoClusterDataSetProperties {
    #[doc = "Unique id for identifying a data set resource"]
    #[serde(rename = "dataSetId", default, skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[doc = "Resource id of the kusto cluster."]
    #[serde(rename = "kustoClusterResourceId")]
    pub kusto_cluster_resource_id: String,
    #[doc = "Location of the kusto cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Provisioning state of the kusto cluster data set."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<kusto_cluster_data_set_properties::ProvisioningState>,
}
impl KustoClusterDataSetProperties {
    pub fn new(kusto_cluster_resource_id: String) -> Self {
        Self {
            data_set_id: None,
            kusto_cluster_resource_id,
            location: None,
            provisioning_state: None,
        }
    }
}
pub mod kusto_cluster_data_set_properties {
    use super::*;
    #[doc = "Provisioning state of the kusto cluster data set."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Deleting,
        Moving,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A kusto database data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KustoDatabaseDataSet {
    #[serde(flatten)]
    pub data_set: DataSet,
    #[doc = "Properties of the kusto database data set."]
    pub properties: KustoDatabaseDataSetProperties,
}
impl KustoDatabaseDataSet {
    pub fn new(data_set: DataSet, properties: KustoDatabaseDataSetProperties) -> Self {
        Self { data_set, properties }
    }
}
#[doc = "A Kusto database data set mapping"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KustoDatabaseDataSetMapping {
    #[serde(flatten)]
    pub data_set_mapping: DataSetMapping,
    #[doc = "Properties of the Kusto database data set mapping"]
    pub properties: KustoDatabaseDataSetMappingProperties,
}
impl KustoDatabaseDataSetMapping {
    pub fn new(data_set_mapping: DataSetMapping, properties: KustoDatabaseDataSetMappingProperties) -> Self {
        Self {
            data_set_mapping,
            properties,
        }
    }
}
#[doc = "Properties of the Kusto database data set mapping"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KustoDatabaseDataSetMappingProperties {
    #[doc = "The id of the source data set."]
    #[serde(rename = "dataSetId")]
    pub data_set_id: String,
    #[doc = "Gets the status of the data set mapping."]
    #[serde(rename = "dataSetMappingStatus", default, skip_serializing_if = "Option::is_none")]
    pub data_set_mapping_status: Option<kusto_database_data_set_mapping_properties::DataSetMappingStatus>,
    #[doc = "Resource id of the sink kusto cluster."]
    #[serde(rename = "kustoClusterResourceId")]
    pub kusto_cluster_resource_id: String,
    #[doc = "Location of the sink kusto cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Provisioning state of the data set mapping."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<kusto_database_data_set_mapping_properties::ProvisioningState>,
}
impl KustoDatabaseDataSetMappingProperties {
    pub fn new(data_set_id: String, kusto_cluster_resource_id: String) -> Self {
        Self {
            data_set_id,
            data_set_mapping_status: None,
            kusto_cluster_resource_id,
            location: None,
            provisioning_state: None,
        }
    }
}
pub mod kusto_database_data_set_mapping_properties {
    use super::*;
    #[doc = "Gets the status of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataSetMappingStatus")]
    pub enum DataSetMappingStatus {
        Ok,
        Broken,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataSetMappingStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataSetMappingStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataSetMappingStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ok => serializer.serialize_unit_variant("DataSetMappingStatus", 0u32, "Ok"),
                Self::Broken => serializer.serialize_unit_variant("DataSetMappingStatus", 1u32, "Broken"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning state of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Deleting,
        Moving,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the kusto database data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KustoDatabaseDataSetProperties {
    #[doc = "Unique id for identifying a data set resource"]
    #[serde(rename = "dataSetId", default, skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[doc = "Resource id of the kusto database."]
    #[serde(rename = "kustoDatabaseResourceId")]
    pub kusto_database_resource_id: String,
    #[doc = "Location of the kusto cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Provisioning state of the kusto database data set."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<kusto_database_data_set_properties::ProvisioningState>,
}
impl KustoDatabaseDataSetProperties {
    pub fn new(kusto_database_resource_id: String) -> Self {
        Self {
            data_set_id: None,
            kusto_database_resource_id,
            location: None,
            provisioning_state: None,
        }
    }
}
pub mod kusto_database_data_set_properties {
    use super::*;
    #[doc = "Provisioning state of the kusto database data set."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Deleting,
        Moving,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List response for get operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<OperationModel>,
}
impl azure_core::Continuable for OperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationList {
    pub fn new(value: Vec<OperationModel>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "log specifications for operation api"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaLogSpecification {
    #[doc = "blob duration of the log"]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
    #[doc = "localized name of the log category"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "name of the log category"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl OperationMetaLogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "metric specifications for the operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaMetricSpecification {
    #[doc = "aggregation type of metric"]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "properties for dimension"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<DimensionProperties>,
    #[doc = "description of the metric"]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "localized name of the metric"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "enable regional mdm account"]
    #[serde(rename = "enableRegionalMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub enable_regional_mdm_account: Option<String>,
    #[doc = "fill gap with zero"]
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[doc = "internal metric name"]
    #[serde(rename = "internalMetricName", default, skip_serializing_if = "Option::is_none")]
    pub internal_metric_name: Option<String>,
    #[doc = "name of the metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "dimension name use to replace resource id if specified"]
    #[serde(rename = "resourceIdDimensionNameOverride", default, skip_serializing_if = "Option::is_none")]
    pub resource_id_dimension_name_override: Option<String>,
    #[doc = "supported aggregation types"]
    #[serde(rename = "supportedAggregationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<String>,
    #[doc = "supported time grain types"]
    #[serde(rename = "supportedTimeGrainTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_time_grain_types: Vec<String>,
    #[doc = "units for the metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl OperationMetaMetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "properties on meta info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaPropertyInfo {
    #[doc = "The operation meta service specification"]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<OperationMetaServiceSpecification>,
}
impl OperationMetaPropertyInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The operation meta service specification"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaServiceSpecification {
    #[doc = "log specifications for the operation"]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<OperationMetaLogSpecification>,
    #[doc = "metric specifications for the operation"]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<OperationMetaMetricSpecification>,
}
impl OperationMetaServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model for get operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationModel {
    #[doc = "Properties on operations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationModelProperties>,
    #[doc = "Operation name for display purposes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "properties on meta info"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationMetaPropertyInfo>,
}
impl OperationModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties on operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationModelProperties {
    #[doc = "Description of the operation for display purposes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Name of the operation for display purposes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Name of the provider for display purposes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Name of the resource type for display purposes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl OperationModelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for long running operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResponse {
    #[doc = "start time"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The data share error body model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<DataShareErrorInfo>,
    #[doc = "start time"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Operation state of the long running operation."]
    pub status: operation_response::Status,
}
impl OperationResponse {
    pub fn new(status: operation_response::Status) -> Self {
        Self {
            end_time: None,
            error: None,
            start_time: None,
            status,
        }
    }
}
pub mod operation_response {
    use super::*;
    #[doc = "Operation state of the long running operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Accepted,
        InProgress,
        TransientFailure,
        Succeeded,
        Failed,
        Canceled,
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
                Self::Accepted => serializer.serialize_unit_variant("Status", 0u32, "Accepted"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::TransientFailure => serializer.serialize_unit_variant("Status", 2u32, "TransientFailure"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A provider side share subscription data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderShareSubscription {
    #[serde(flatten)]
    pub proxy_dto: ProxyDto,
    #[doc = "Provider share subscription properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProviderShareSubscriptionProperties>,
}
impl ProviderShareSubscription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List response for get ShareSubscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderShareSubscriptionList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<ProviderShareSubscription>,
}
impl azure_core::Continuable for ProviderShareSubscriptionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProviderShareSubscriptionList {
    pub fn new(value: Vec<ProviderShareSubscription>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Provider share subscription properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderShareSubscriptionProperties {
    #[doc = "Email of the consumer who created the share subscription"]
    #[serde(rename = "consumerEmail", default, skip_serializing_if = "Option::is_none")]
    pub consumer_email: Option<String>,
    #[doc = "Name of the consumer who created the share subscription"]
    #[serde(rename = "consumerName", default, skip_serializing_if = "Option::is_none")]
    pub consumer_name: Option<String>,
    #[doc = "Tenant name of the consumer who created the share subscription"]
    #[serde(rename = "consumerTenantName", default, skip_serializing_if = "Option::is_none")]
    pub consumer_tenant_name: Option<String>,
    #[doc = "created at"]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "Email of the provider who created the share"]
    #[serde(rename = "providerEmail", default, skip_serializing_if = "Option::is_none")]
    pub provider_email: Option<String>,
    #[doc = "Name of the provider who created the share"]
    #[serde(rename = "providerName", default, skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[doc = "Shared at"]
    #[serde(rename = "sharedAt", default, with = "azure_core::date::rfc3339::option")]
    pub shared_at: Option<time::OffsetDateTime>,
    #[doc = "share Subscription Object Id"]
    #[serde(rename = "shareSubscriptionObjectId", default, skip_serializing_if = "Option::is_none")]
    pub share_subscription_object_id: Option<String>,
    #[doc = "Gets the status of share subscription"]
    #[serde(rename = "shareSubscriptionStatus", default, skip_serializing_if = "Option::is_none")]
    pub share_subscription_status: Option<provider_share_subscription_properties::ShareSubscriptionStatus>,
}
impl ProviderShareSubscriptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod provider_share_subscription_properties {
    use super::*;
    #[doc = "Gets the status of share subscription"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ShareSubscriptionStatus")]
    pub enum ShareSubscriptionStatus {
        Active,
        Revoked,
        SourceDeleted,
        Revoking,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ShareSubscriptionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ShareSubscriptionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ShareSubscriptionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Active => serializer.serialize_unit_variant("ShareSubscriptionStatus", 0u32, "Active"),
                Self::Revoked => serializer.serialize_unit_variant("ShareSubscriptionStatus", 1u32, "Revoked"),
                Self::SourceDeleted => serializer.serialize_unit_variant("ShareSubscriptionStatus", 2u32, "SourceDeleted"),
                Self::Revoking => serializer.serialize_unit_variant("ShareSubscriptionStatus", 3u32, "Revoking"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Base data transfer object implementation for proxy resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyDto {
    #[doc = "The resource id of the azure resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the azure resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the azure resource"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ProxyDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Scheduled source synchronization setting data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduledSourceShareSynchronizationSettingProperties {
    #[doc = "Recurrence Interval"]
    #[serde(rename = "recurrenceInterval", default, skip_serializing_if = "Option::is_none")]
    pub recurrence_interval: Option<scheduled_source_share_synchronization_setting_properties::RecurrenceInterval>,
    #[doc = "Synchronization time"]
    #[serde(rename = "synchronizationTime", default, with = "azure_core::date::rfc3339::option")]
    pub synchronization_time: Option<time::OffsetDateTime>,
}
impl ScheduledSourceShareSynchronizationSettingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod scheduled_source_share_synchronization_setting_properties {
    use super::*;
    #[doc = "Recurrence Interval"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecurrenceInterval")]
    pub enum RecurrenceInterval {
        Hour,
        Day,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecurrenceInterval {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecurrenceInterval {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecurrenceInterval {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Hour => serializer.serialize_unit_variant("RecurrenceInterval", 0u32, "Hour"),
                Self::Day => serializer.serialize_unit_variant("RecurrenceInterval", 1u32, "Day"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A type of synchronization setting based on schedule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledSourceSynchronizationSetting {
    #[serde(flatten)]
    pub source_share_synchronization_setting: SourceShareSynchronizationSetting,
    #[doc = "A Scheduled source synchronization setting data transfer object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduledSourceShareSynchronizationSettingProperties>,
}
impl ScheduledSourceSynchronizationSetting {
    pub fn new(source_share_synchronization_setting: SourceShareSynchronizationSetting) -> Self {
        Self {
            source_share_synchronization_setting,
            properties: None,
        }
    }
}
#[doc = "A type of synchronization setting based on schedule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledSynchronizationSetting {
    #[serde(flatten)]
    pub synchronization_setting: SynchronizationSetting,
    #[doc = "A Scheduled synchronization setting data transfer object."]
    pub properties: ScheduledSynchronizationSettingProperties,
}
impl ScheduledSynchronizationSetting {
    pub fn new(synchronization_setting: SynchronizationSetting, properties: ScheduledSynchronizationSettingProperties) -> Self {
        Self {
            synchronization_setting,
            properties,
        }
    }
}
#[doc = "A Scheduled synchronization setting data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledSynchronizationSettingProperties {
    #[doc = "Time at which the synchronization setting was created."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<scheduled_synchronization_setting_properties::ProvisioningState>,
    #[doc = "Recurrence Interval"]
    #[serde(rename = "recurrenceInterval")]
    pub recurrence_interval: scheduled_synchronization_setting_properties::RecurrenceInterval,
    #[doc = "Synchronization time"]
    #[serde(rename = "synchronizationTime", with = "azure_core::date::rfc3339")]
    pub synchronization_time: time::OffsetDateTime,
    #[doc = "Name of the user who created the synchronization setting."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}
impl ScheduledSynchronizationSettingProperties {
    pub fn new(
        recurrence_interval: scheduled_synchronization_setting_properties::RecurrenceInterval,
        synchronization_time: time::OffsetDateTime,
    ) -> Self {
        Self {
            created_at: None,
            provisioning_state: None,
            recurrence_interval,
            synchronization_time,
            user_name: None,
        }
    }
}
pub mod scheduled_synchronization_setting_properties {
    use super::*;
    #[doc = "Gets or sets the provisioning state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Deleting,
        Moving,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Recurrence Interval"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecurrenceInterval")]
    pub enum RecurrenceInterval {
        Hour,
        Day,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecurrenceInterval {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecurrenceInterval {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecurrenceInterval {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Hour => serializer.serialize_unit_variant("RecurrenceInterval", 0u32, "Hour"),
                Self::Day => serializer.serialize_unit_variant("RecurrenceInterval", 1u32, "Day"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A type of trigger based on schedule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledTrigger {
    #[serde(flatten)]
    pub trigger: Trigger,
    #[doc = "A Scheduled trigger data transfer object."]
    pub properties: ScheduledTriggerProperties,
}
impl ScheduledTrigger {
    pub fn new(trigger: Trigger, properties: ScheduledTriggerProperties) -> Self {
        Self { trigger, properties }
    }
}
#[doc = "A Scheduled trigger data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledTriggerProperties {
    #[doc = "Time at which the trigger was created."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "Gets the provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<scheduled_trigger_properties::ProvisioningState>,
    #[doc = "Recurrence Interval"]
    #[serde(rename = "recurrenceInterval")]
    pub recurrence_interval: scheduled_trigger_properties::RecurrenceInterval,
    #[doc = "Synchronization mode"]
    #[serde(rename = "synchronizationMode", default, skip_serializing_if = "Option::is_none")]
    pub synchronization_mode: Option<scheduled_trigger_properties::SynchronizationMode>,
    #[doc = "Synchronization time"]
    #[serde(rename = "synchronizationTime", with = "azure_core::date::rfc3339")]
    pub synchronization_time: time::OffsetDateTime,
    #[doc = "Gets the trigger state"]
    #[serde(rename = "triggerStatus", default, skip_serializing_if = "Option::is_none")]
    pub trigger_status: Option<scheduled_trigger_properties::TriggerStatus>,
    #[doc = "Name of the user who created the trigger."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}
impl ScheduledTriggerProperties {
    pub fn new(recurrence_interval: scheduled_trigger_properties::RecurrenceInterval, synchronization_time: time::OffsetDateTime) -> Self {
        Self {
            created_at: None,
            provisioning_state: None,
            recurrence_interval,
            synchronization_mode: None,
            synchronization_time,
            trigger_status: None,
            user_name: None,
        }
    }
}
pub mod scheduled_trigger_properties {
    use super::*;
    #[doc = "Gets the provisioning state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Deleting,
        Moving,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Recurrence Interval"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecurrenceInterval")]
    pub enum RecurrenceInterval {
        Hour,
        Day,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecurrenceInterval {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecurrenceInterval {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecurrenceInterval {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Hour => serializer.serialize_unit_variant("RecurrenceInterval", 0u32, "Hour"),
                Self::Day => serializer.serialize_unit_variant("RecurrenceInterval", 1u32, "Day"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Synchronization mode"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SynchronizationMode")]
    pub enum SynchronizationMode {
        Incremental,
        FullSync,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SynchronizationMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SynchronizationMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SynchronizationMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Incremental => serializer.serialize_unit_variant("SynchronizationMode", 0u32, "Incremental"),
                Self::FullSync => serializer.serialize_unit_variant("SynchronizationMode", 1u32, "FullSync"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets the trigger state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TriggerStatus")]
    pub enum TriggerStatus {
        Active,
        Inactive,
        SourceSynchronizationSettingDeleted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TriggerStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TriggerStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TriggerStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Active => serializer.serialize_unit_variant("TriggerStatus", 0u32, "Active"),
                Self::Inactive => serializer.serialize_unit_variant("TriggerStatus", 1u32, "Inactive"),
                Self::SourceSynchronizationSettingDeleted => {
                    serializer.serialize_unit_variant("TriggerStatus", 2u32, "SourceSynchronizationSettingDeleted")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A share data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Share {
    #[serde(flatten)]
    pub proxy_dto: ProxyDto,
    #[doc = "Share property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ShareProperties>,
}
impl Share {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List response for get Shares."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShareList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<Share>,
}
impl azure_core::Continuable for ShareList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ShareList {
    pub fn new(value: Vec<Share>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Share property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShareProperties {
    #[doc = "Time at which the share was created."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "Share description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets the provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<share_properties::ProvisioningState>,
    #[doc = "Share kind."]
    #[serde(rename = "shareKind", default, skip_serializing_if = "Option::is_none")]
    pub share_kind: Option<share_properties::ShareKind>,
    #[doc = "Share terms."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terms: Option<String>,
    #[doc = "Email of the user who created the resource"]
    #[serde(rename = "userEmail", default, skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    #[doc = "Name of the user who created the resource"]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}
impl ShareProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod share_properties {
    use super::*;
    #[doc = "Gets or sets the provisioning state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Deleting,
        Moving,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Share kind."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ShareKind")]
    pub enum ShareKind {
        CopyBased,
        InPlace,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ShareKind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ShareKind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ShareKind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::CopyBased => serializer.serialize_unit_variant("ShareKind", 0u32, "CopyBased"),
                Self::InPlace => serializer.serialize_unit_variant("ShareKind", 1u32, "InPlace"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A share subscription data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShareSubscription {
    #[serde(flatten)]
    pub proxy_dto: ProxyDto,
    #[doc = "Share subscription property bag."]
    pub properties: ShareSubscriptionProperties,
}
impl ShareSubscription {
    pub fn new(properties: ShareSubscriptionProperties) -> Self {
        Self {
            proxy_dto: ProxyDto::default(),
            properties,
        }
    }
}
#[doc = "List response for get ShareSubscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShareSubscriptionList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<ShareSubscription>,
}
impl azure_core::Continuable for ShareSubscriptionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ShareSubscriptionList {
    pub fn new(value: Vec<ShareSubscription>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Share subscription property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShareSubscriptionProperties {
    #[doc = "Time at which the share subscription was created."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The invitation id."]
    #[serde(rename = "invitationId")]
    pub invitation_id: String,
    #[doc = "Email of the provider who created the resource"]
    #[serde(rename = "providerEmail", default, skip_serializing_if = "Option::is_none")]
    pub provider_email: Option<String>,
    #[doc = "Name of the provider who created the resource"]
    #[serde(rename = "providerName", default, skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[doc = "Tenant name of the provider who created the resource"]
    #[serde(rename = "providerTenantName", default, skip_serializing_if = "Option::is_none")]
    pub provider_tenant_name: Option<String>,
    #[doc = "Provisioning state of the share subscription"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<share_subscription_properties::ProvisioningState>,
    #[doc = "Description of share"]
    #[serde(rename = "shareDescription", default, skip_serializing_if = "Option::is_none")]
    pub share_description: Option<String>,
    #[doc = "Kind of share"]
    #[serde(rename = "shareKind", default, skip_serializing_if = "Option::is_none")]
    pub share_kind: Option<share_subscription_properties::ShareKind>,
    #[doc = "Name of the share"]
    #[serde(rename = "shareName", default, skip_serializing_if = "Option::is_none")]
    pub share_name: Option<String>,
    #[doc = "Gets the current status of share subscription."]
    #[serde(rename = "shareSubscriptionStatus", default, skip_serializing_if = "Option::is_none")]
    pub share_subscription_status: Option<share_subscription_properties::ShareSubscriptionStatus>,
    #[doc = "Terms of a share"]
    #[serde(rename = "shareTerms", default, skip_serializing_if = "Option::is_none")]
    pub share_terms: Option<String>,
    #[doc = "Email of the user who created the resource"]
    #[serde(rename = "userEmail", default, skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    #[doc = "Name of the user who created the resource"]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}
impl ShareSubscriptionProperties {
    pub fn new(invitation_id: String) -> Self {
        Self {
            created_at: None,
            invitation_id,
            provider_email: None,
            provider_name: None,
            provider_tenant_name: None,
            provisioning_state: None,
            share_description: None,
            share_kind: None,
            share_name: None,
            share_subscription_status: None,
            share_terms: None,
            user_email: None,
            user_name: None,
        }
    }
}
pub mod share_subscription_properties {
    use super::*;
    #[doc = "Provisioning state of the share subscription"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Deleting,
        Moving,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Kind of share"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ShareKind")]
    pub enum ShareKind {
        CopyBased,
        InPlace,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ShareKind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ShareKind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ShareKind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::CopyBased => serializer.serialize_unit_variant("ShareKind", 0u32, "CopyBased"),
                Self::InPlace => serializer.serialize_unit_variant("ShareKind", 1u32, "InPlace"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets the current status of share subscription."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ShareSubscriptionStatus")]
    pub enum ShareSubscriptionStatus {
        Active,
        Revoked,
        SourceDeleted,
        Revoking,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ShareSubscriptionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ShareSubscriptionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ShareSubscriptionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Active => serializer.serialize_unit_variant("ShareSubscriptionStatus", 0u32, "Active"),
                Self::Revoked => serializer.serialize_unit_variant("ShareSubscriptionStatus", 1u32, "Revoked"),
                Self::SourceDeleted => serializer.serialize_unit_variant("ShareSubscriptionStatus", 2u32, "SourceDeleted"),
                Self::Revoking => serializer.serialize_unit_variant("ShareSubscriptionStatus", 3u32, "Revoking"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A ShareSubscriptionSynchronization data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShareSubscriptionSynchronization {
    #[doc = "Synchronization duration"]
    #[serde(rename = "durationMs", default, skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<i32>,
    #[doc = "End time of synchronization"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "message of Synchronization"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "start time of synchronization"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Raw Status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Synchronization id"]
    #[serde(rename = "synchronizationId")]
    pub synchronization_id: String,
    #[doc = "Synchronization Mode"]
    #[serde(rename = "synchronizationMode", default, skip_serializing_if = "Option::is_none")]
    pub synchronization_mode: Option<share_subscription_synchronization::SynchronizationMode>,
}
impl ShareSubscriptionSynchronization {
    pub fn new(synchronization_id: String) -> Self {
        Self {
            duration_ms: None,
            end_time: None,
            message: None,
            start_time: None,
            status: None,
            synchronization_id,
            synchronization_mode: None,
        }
    }
}
pub mod share_subscription_synchronization {
    use super::*;
    #[doc = "Synchronization Mode"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SynchronizationMode")]
    pub enum SynchronizationMode {
        Incremental,
        FullSync,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SynchronizationMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SynchronizationMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SynchronizationMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Incremental => serializer.serialize_unit_variant("SynchronizationMode", 0u32, "Incremental"),
                Self::FullSync => serializer.serialize_unit_variant("SynchronizationMode", 1u32, "FullSync"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A consumer side list of share subscription synchronizations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShareSubscriptionSynchronizationList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<ShareSubscriptionSynchronization>,
}
impl azure_core::Continuable for ShareSubscriptionSynchronizationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ShareSubscriptionSynchronizationList {
    pub fn new(value: Vec<ShareSubscriptionSynchronization>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "A ShareSynchronization data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShareSynchronization {
    #[doc = "Email of the user who created the synchronization"]
    #[serde(rename = "consumerEmail", default, skip_serializing_if = "Option::is_none")]
    pub consumer_email: Option<String>,
    #[doc = "Name of the user who created the synchronization"]
    #[serde(rename = "consumerName", default, skip_serializing_if = "Option::is_none")]
    pub consumer_name: Option<String>,
    #[doc = "Tenant name of the consumer who created the synchronization"]
    #[serde(rename = "consumerTenantName", default, skip_serializing_if = "Option::is_none")]
    pub consumer_tenant_name: Option<String>,
    #[doc = "synchronization duration"]
    #[serde(rename = "durationMs", default, skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<i32>,
    #[doc = "End time of synchronization"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "message of synchronization"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "start time of synchronization"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Raw Status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Synchronization id"]
    #[serde(rename = "synchronizationId", default, skip_serializing_if = "Option::is_none")]
    pub synchronization_id: Option<String>,
    #[doc = "Synchronization mode"]
    #[serde(rename = "synchronizationMode", default, skip_serializing_if = "Option::is_none")]
    pub synchronization_mode: Option<share_synchronization::SynchronizationMode>,
}
impl ShareSynchronization {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod share_synchronization {
    use super::*;
    #[doc = "Synchronization mode"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SynchronizationMode")]
    pub enum SynchronizationMode {
        Incremental,
        FullSync,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SynchronizationMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SynchronizationMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SynchronizationMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Incremental => serializer.serialize_unit_variant("SynchronizationMode", 0u32, "Incremental"),
                Self::FullSync => serializer.serialize_unit_variant("SynchronizationMode", 1u32, "FullSync"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List response for get ShareSynchronization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShareSynchronizationList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<ShareSynchronization>,
}
impl azure_core::Continuable for ShareSynchronizationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ShareSynchronizationList {
    pub fn new(value: Vec<ShareSynchronization>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "A view of synchronization setting added by the provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceShareSynchronizationSetting {
    #[doc = "Kind of synchronization setting on share."]
    pub kind: source_share_synchronization_setting::Kind,
}
impl SourceShareSynchronizationSetting {
    pub fn new(kind: source_share_synchronization_setting::Kind) -> Self {
        Self { kind }
    }
}
pub mod source_share_synchronization_setting {
    use super::*;
    #[doc = "Kind of synchronization setting on share."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        ScheduleBased,
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
                Self::ScheduleBased => serializer.serialize_unit_variant("Kind", 0u32, "ScheduleBased"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List response for get source share Synchronization settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceShareSynchronizationSettingList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<SourceShareSynchronizationSetting>,
}
impl azure_core::Continuable for SourceShareSynchronizationSettingList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SourceShareSynchronizationSettingList {
    pub fn new(value: Vec<SourceShareSynchronizationSetting>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "A SQL DB table data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDbTableDataSet {
    #[serde(flatten)]
    pub data_set: DataSet,
    #[doc = "Properties of the SQL DB table data set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlDbTableProperties>,
}
impl SqlDbTableDataSet {
    pub fn new(data_set: DataSet) -> Self {
        Self {
            data_set,
            properties: None,
        }
    }
}
#[doc = "A SQL DB Table data set mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDbTableDataSetMapping {
    #[serde(flatten)]
    pub data_set_mapping: DataSetMapping,
    #[doc = "Properties of the SQL DB table data set mapping."]
    pub properties: SqlDbTableDataSetMappingProperties,
}
impl SqlDbTableDataSetMapping {
    pub fn new(data_set_mapping: DataSetMapping, properties: SqlDbTableDataSetMappingProperties) -> Self {
        Self {
            data_set_mapping,
            properties,
        }
    }
}
#[doc = "Properties of the SQL DB table data set mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDbTableDataSetMappingProperties {
    #[doc = "DatabaseName name of the sink data set"]
    #[serde(rename = "databaseName")]
    pub database_name: String,
    #[doc = "The id of the source data set."]
    #[serde(rename = "dataSetId")]
    pub data_set_id: String,
    #[doc = "Gets the status of the data set mapping."]
    #[serde(rename = "dataSetMappingStatus", default, skip_serializing_if = "Option::is_none")]
    pub data_set_mapping_status: Option<sql_db_table_data_set_mapping_properties::DataSetMappingStatus>,
    #[doc = "Provisioning state of the data set mapping."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<sql_db_table_data_set_mapping_properties::ProvisioningState>,
    #[doc = "Schema of the table. Default value is dbo."]
    #[serde(rename = "schemaName")]
    pub schema_name: String,
    #[doc = "Resource id of SQL server"]
    #[serde(rename = "sqlServerResourceId")]
    pub sql_server_resource_id: String,
    #[doc = "SQL DB table name."]
    #[serde(rename = "tableName")]
    pub table_name: String,
}
impl SqlDbTableDataSetMappingProperties {
    pub fn new(
        database_name: String,
        data_set_id: String,
        schema_name: String,
        sql_server_resource_id: String,
        table_name: String,
    ) -> Self {
        Self {
            database_name,
            data_set_id,
            data_set_mapping_status: None,
            provisioning_state: None,
            schema_name,
            sql_server_resource_id,
            table_name,
        }
    }
}
pub mod sql_db_table_data_set_mapping_properties {
    use super::*;
    #[doc = "Gets the status of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataSetMappingStatus")]
    pub enum DataSetMappingStatus {
        Ok,
        Broken,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataSetMappingStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataSetMappingStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataSetMappingStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ok => serializer.serialize_unit_variant("DataSetMappingStatus", 0u32, "Ok"),
                Self::Broken => serializer.serialize_unit_variant("DataSetMappingStatus", 1u32, "Broken"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning state of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Deleting,
        Moving,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the SQL DB table data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDbTableProperties {
    #[doc = "Database name of the source data set"]
    #[serde(rename = "databaseName")]
    pub database_name: String,
    #[doc = "Unique id for identifying a data set resource"]
    #[serde(rename = "dataSetId", default, skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[doc = "Schema of the table. Default value is dbo."]
    #[serde(rename = "schemaName")]
    pub schema_name: String,
    #[doc = "Resource id of SQL server"]
    #[serde(rename = "sqlServerResourceId")]
    pub sql_server_resource_id: String,
    #[doc = "SQL DB table name."]
    #[serde(rename = "tableName")]
    pub table_name: String,
}
impl SqlDbTableProperties {
    pub fn new(database_name: String, schema_name: String, sql_server_resource_id: String, table_name: String) -> Self {
        Self {
            database_name,
            data_set_id: None,
            schema_name,
            sql_server_resource_id,
            table_name,
        }
    }
}
#[doc = "A SQL DW table data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDwTableDataSet {
    #[serde(flatten)]
    pub data_set: DataSet,
    #[doc = "Properties of the SQL DW table data set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlDwTableProperties>,
}
impl SqlDwTableDataSet {
    pub fn new(data_set: DataSet) -> Self {
        Self {
            data_set,
            properties: None,
        }
    }
}
#[doc = "A SQL DW Table data set mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDwTableDataSetMapping {
    #[serde(flatten)]
    pub data_set_mapping: DataSetMapping,
    #[doc = "Properties of the SQL DW table data set mapping."]
    pub properties: SqlDwTableDataSetMappingProperties,
}
impl SqlDwTableDataSetMapping {
    pub fn new(data_set_mapping: DataSetMapping, properties: SqlDwTableDataSetMappingProperties) -> Self {
        Self {
            data_set_mapping,
            properties,
        }
    }
}
#[doc = "Properties of the SQL DW table data set mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDwTableDataSetMappingProperties {
    #[doc = "The id of the source data set."]
    #[serde(rename = "dataSetId")]
    pub data_set_id: String,
    #[doc = "Gets the status of the data set mapping."]
    #[serde(rename = "dataSetMappingStatus", default, skip_serializing_if = "Option::is_none")]
    pub data_set_mapping_status: Option<sql_dw_table_data_set_mapping_properties::DataSetMappingStatus>,
    #[doc = "DataWarehouse name of the source data set"]
    #[serde(rename = "dataWarehouseName")]
    pub data_warehouse_name: String,
    #[doc = "Provisioning state of the data set mapping."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<sql_dw_table_data_set_mapping_properties::ProvisioningState>,
    #[doc = "Schema of the table. Default value is dbo."]
    #[serde(rename = "schemaName")]
    pub schema_name: String,
    #[doc = "Resource id of SQL server"]
    #[serde(rename = "sqlServerResourceId")]
    pub sql_server_resource_id: String,
    #[doc = "SQL DW table name."]
    #[serde(rename = "tableName")]
    pub table_name: String,
}
impl SqlDwTableDataSetMappingProperties {
    pub fn new(
        data_set_id: String,
        data_warehouse_name: String,
        schema_name: String,
        sql_server_resource_id: String,
        table_name: String,
    ) -> Self {
        Self {
            data_set_id,
            data_set_mapping_status: None,
            data_warehouse_name,
            provisioning_state: None,
            schema_name,
            sql_server_resource_id,
            table_name,
        }
    }
}
pub mod sql_dw_table_data_set_mapping_properties {
    use super::*;
    #[doc = "Gets the status of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataSetMappingStatus")]
    pub enum DataSetMappingStatus {
        Ok,
        Broken,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataSetMappingStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataSetMappingStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataSetMappingStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ok => serializer.serialize_unit_variant("DataSetMappingStatus", 0u32, "Ok"),
                Self::Broken => serializer.serialize_unit_variant("DataSetMappingStatus", 1u32, "Broken"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning state of the data set mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Deleting,
        Moving,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the SQL DW table data set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDwTableProperties {
    #[doc = "Unique id for identifying a data set resource"]
    #[serde(rename = "dataSetId", default, skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[doc = "DataWarehouse name of the source data set"]
    #[serde(rename = "dataWarehouseName")]
    pub data_warehouse_name: String,
    #[doc = "Schema of the table. Default value is dbo."]
    #[serde(rename = "schemaName")]
    pub schema_name: String,
    #[doc = "Resource id of SQL server"]
    #[serde(rename = "sqlServerResourceId")]
    pub sql_server_resource_id: String,
    #[doc = "SQL DW table name."]
    #[serde(rename = "tableName")]
    pub table_name: String,
}
impl SqlDwTableProperties {
    pub fn new(data_warehouse_name: String, schema_name: String, sql_server_resource_id: String, table_name: String) -> Self {
        Self {
            data_set_id: None,
            data_warehouse_name,
            schema_name,
            sql_server_resource_id,
            table_name,
        }
    }
}
#[doc = "Synchronization details at data set level"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SynchronizationDetails {
    #[doc = "Id of data set"]
    #[serde(rename = "dataSetId", default, skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[doc = "Type of the data set"]
    #[serde(rename = "dataSetType", default, skip_serializing_if = "Option::is_none")]
    pub data_set_type: Option<synchronization_details::DataSetType>,
    #[doc = "Duration of data set level copy"]
    #[serde(rename = "durationMs", default, skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<i32>,
    #[doc = "End time of data set level copy"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The number of files read from the source data set"]
    #[serde(rename = "filesRead", default, skip_serializing_if = "Option::is_none")]
    pub files_read: Option<i64>,
    #[doc = "The number of files written into the sink data set"]
    #[serde(rename = "filesWritten", default, skip_serializing_if = "Option::is_none")]
    pub files_written: Option<i64>,
    #[doc = "Error message if any"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Name of the data set"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The number of files copied into the sink data set"]
    #[serde(rename = "rowsCopied", default, skip_serializing_if = "Option::is_none")]
    pub rows_copied: Option<i64>,
    #[doc = "The number of rows read from the source data set."]
    #[serde(rename = "rowsRead", default, skip_serializing_if = "Option::is_none")]
    pub rows_read: Option<i64>,
    #[doc = "The size of the data read from the source data set in bytes"]
    #[serde(rename = "sizeRead", default, skip_serializing_if = "Option::is_none")]
    pub size_read: Option<i64>,
    #[doc = "The size of the data written into the sink data set in bytes"]
    #[serde(rename = "sizeWritten", default, skip_serializing_if = "Option::is_none")]
    pub size_written: Option<i64>,
    #[doc = "Start time of data set level copy"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Raw Status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The vCore units consumed for the data set synchronization"]
    #[serde(rename = "vCore", default, skip_serializing_if = "Option::is_none")]
    pub v_core: Option<i64>,
}
impl SynchronizationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod synchronization_details {
    use super::*;
    #[doc = "Type of the data set"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataSetType")]
    pub enum DataSetType {
        Blob,
        Container,
        BlobFolder,
        AdlsGen2FileSystem,
        AdlsGen2Folder,
        AdlsGen2File,
        AdlsGen1Folder,
        AdlsGen1File,
        KustoCluster,
        KustoDatabase,
        #[serde(rename = "SqlDBTable")]
        SqlDbTable,
        #[serde(rename = "SqlDWTable")]
        SqlDwTable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataSetType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataSetType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataSetType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Blob => serializer.serialize_unit_variant("DataSetType", 0u32, "Blob"),
                Self::Container => serializer.serialize_unit_variant("DataSetType", 1u32, "Container"),
                Self::BlobFolder => serializer.serialize_unit_variant("DataSetType", 2u32, "BlobFolder"),
                Self::AdlsGen2FileSystem => serializer.serialize_unit_variant("DataSetType", 3u32, "AdlsGen2FileSystem"),
                Self::AdlsGen2Folder => serializer.serialize_unit_variant("DataSetType", 4u32, "AdlsGen2Folder"),
                Self::AdlsGen2File => serializer.serialize_unit_variant("DataSetType", 5u32, "AdlsGen2File"),
                Self::AdlsGen1Folder => serializer.serialize_unit_variant("DataSetType", 6u32, "AdlsGen1Folder"),
                Self::AdlsGen1File => serializer.serialize_unit_variant("DataSetType", 7u32, "AdlsGen1File"),
                Self::KustoCluster => serializer.serialize_unit_variant("DataSetType", 8u32, "KustoCluster"),
                Self::KustoDatabase => serializer.serialize_unit_variant("DataSetType", 9u32, "KustoDatabase"),
                Self::SqlDbTable => serializer.serialize_unit_variant("DataSetType", 10u32, "SqlDBTable"),
                Self::SqlDwTable => serializer.serialize_unit_variant("DataSetType", 11u32, "SqlDWTable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "details of synchronization"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SynchronizationDetailsList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<SynchronizationDetails>,
}
impl azure_core::Continuable for SynchronizationDetailsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SynchronizationDetailsList {
    pub fn new(value: Vec<SynchronizationDetails>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "A Synchronization Setting data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SynchronizationSetting {
    #[serde(flatten)]
    pub proxy_dto: ProxyDto,
    #[doc = "Kind of synchronization setting."]
    pub kind: synchronization_setting::Kind,
}
impl SynchronizationSetting {
    pub fn new(kind: synchronization_setting::Kind) -> Self {
        Self {
            proxy_dto: ProxyDto::default(),
            kind,
        }
    }
}
pub mod synchronization_setting {
    use super::*;
    #[doc = "Kind of synchronization setting."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        ScheduleBased,
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
                Self::ScheduleBased => serializer.serialize_unit_variant("Kind", 0u32, "ScheduleBased"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List response for get Synchronization settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SynchronizationSettingList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<SynchronizationSetting>,
}
impl azure_core::Continuable for SynchronizationSettingList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SynchronizationSettingList {
    pub fn new(value: Vec<SynchronizationSetting>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Payload for the synchronizing the data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Synchronize {
    #[doc = "Mode of synchronization used in triggers and snapshot sync. Incremental by default"]
    #[serde(rename = "synchronizationMode", default, skip_serializing_if = "Option::is_none")]
    pub synchronization_mode: Option<synchronize::SynchronizationMode>,
}
impl Synchronize {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod synchronize {
    use super::*;
    #[doc = "Mode of synchronization used in triggers and snapshot sync. Incremental by default"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SynchronizationMode")]
    pub enum SynchronizationMode {
        Incremental,
        FullSync,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SynchronizationMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SynchronizationMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SynchronizationMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Incremental => serializer.serialize_unit_variant("SynchronizationMode", 0u32, "Incremental"),
                Self::FullSync => serializer.serialize_unit_variant("SynchronizationMode", 1u32, "FullSync"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A Trigger data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trigger {
    #[serde(flatten)]
    pub proxy_dto: ProxyDto,
    #[doc = "Kind of synchronization on trigger."]
    pub kind: trigger::Kind,
}
impl Trigger {
    pub fn new(kind: trigger::Kind) -> Self {
        Self {
            proxy_dto: ProxyDto::default(),
            kind,
        }
    }
}
pub mod trigger {
    use super::*;
    #[doc = "Kind of synchronization on trigger."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        ScheduleBased,
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
                Self::ScheduleBased => serializer.serialize_unit_variant("Kind", 0u32, "ScheduleBased"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List response for get triggers"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<Trigger>,
}
impl azure_core::Continuable for TriggerList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TriggerList {
    pub fn new(value: Vec<Trigger>) -> Self {
        Self { next_link: None, value }
    }
}
