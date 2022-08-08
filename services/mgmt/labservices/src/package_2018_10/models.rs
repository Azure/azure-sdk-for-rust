#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Payload for Add Users operation on a Lab."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddUsersPayload {
    #[doc = "List of user emails addresses to add to the lab."]
    #[serde(rename = "emailAddresses")]
    pub email_addresses: Vec<String>,
}
impl AddUsersPayload {
    pub fn new(email_addresses: Vec<String>) -> Self {
        Self { email_addresses }
    }
}
#[doc = "Error from a REST request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "Body of an error from a REST request."]
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
#[doc = "Body of an error from a REST request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Inner errors."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for creating a managed lab and a default environment setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateLabProperties {
    #[doc = "Settings related to creating an environment setting"]
    #[serde(rename = "environmentSettingCreationParameters", default, skip_serializing_if = "Option::is_none")]
    pub environment_setting_creation_parameters: Option<EnvironmentSettingCreationParameters>,
    #[doc = "Settings related to creating a lab"]
    #[serde(rename = "labCreationParameters")]
    pub lab_creation_parameters: LabCreationParameters,
    #[doc = "The name of the resource"]
    pub name: String,
    #[doc = "The location of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The tags of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CreateLabProperties {
    pub fn new(lab_creation_parameters: LabCreationParameters, name: String) -> Self {
        Self {
            environment_setting_creation_parameters: None,
            lab_creation_parameters,
            name,
            location: None,
            tags: None,
        }
    }
}
#[doc = "Represents an environment instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Environment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of an environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnvironmentProperties>,
}
impl Environment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This represents the details about a User's environment and its state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentDetails {
    #[doc = "Name of the Environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Description of the Environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Resource Id of the environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The provisioning state of the environment. This also includes LabIsFull and NotYetProvisioned status."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Details of the backing virtual machine."]
    #[serde(rename = "virtualMachineDetails", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_details: Option<VirtualMachineDetails>,
    #[doc = "Details of the status of an operation."]
    #[serde(rename = "latestOperationResult", default, skip_serializing_if = "Option::is_none")]
    pub latest_operation_result: Option<LatestOperationResult>,
    #[doc = "Publishing state of the environment setting Possible values are Creating, Created, Failed"]
    #[serde(rename = "environmentState", default, skip_serializing_if = "Option::is_none")]
    pub environment_state: Option<String>,
    #[doc = "How long the environment has been used by a lab user"]
    #[serde(rename = "totalUsage", default, skip_serializing_if = "Option::is_none")]
    pub total_usage: Option<String>,
    #[doc = "When the password was last reset on the environment."]
    #[serde(rename = "passwordLastReset", with = "azure_core::date::rfc3339::option")]
    pub password_last_reset: Option<time::OffsetDateTime>,
}
impl EnvironmentDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an environment instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentFragment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of an environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnvironmentPropertiesFragment>,
}
impl EnvironmentFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents payload for any Environment operations like get, start, stop, connect"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentOperationsPayload {
    #[doc = "The resourceId of the environment"]
    #[serde(rename = "environmentId")]
    pub environment_id: String,
}
impl EnvironmentOperationsPayload {
    pub fn new(environment_id: String) -> Self {
        Self { environment_id }
    }
}
#[doc = "Properties of an environment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentProperties {
    #[doc = "Represents a VM and the setting Id it was created for."]
    #[serde(rename = "resourceSets", default, skip_serializing_if = "Option::is_none")]
    pub resource_sets: Option<ResourceSet>,
    #[doc = "The AAD object Id of the user who has claimed the environment"]
    #[serde(rename = "claimedByUserObjectId", default, skip_serializing_if = "Option::is_none")]
    pub claimed_by_user_object_id: Option<String>,
    #[doc = "The user principal Id of the user who has claimed the environment"]
    #[serde(rename = "claimedByUserPrincipalId", default, skip_serializing_if = "Option::is_none")]
    pub claimed_by_user_principal_id: Option<String>,
    #[doc = "The name or email address of the user who has claimed the environment"]
    #[serde(rename = "claimedByUserName", default, skip_serializing_if = "Option::is_none")]
    pub claimed_by_user_name: Option<String>,
    #[doc = "Is the environment claimed or not"]
    #[serde(rename = "isClaimed", default, skip_serializing_if = "Option::is_none")]
    pub is_claimed: Option<bool>,
    #[doc = "Last known power state of the environment"]
    #[serde(rename = "lastKnownPowerState", default, skip_serializing_if = "Option::is_none")]
    pub last_known_power_state: Option<String>,
    #[doc = "Network details of the environment"]
    #[serde(rename = "networkInterface", default, skip_serializing_if = "Option::is_none")]
    pub network_interface: Option<NetworkInterface>,
    #[doc = "How long the environment has been used by a lab user"]
    #[serde(rename = "totalUsage", default, skip_serializing_if = "Option::is_none")]
    pub total_usage: Option<String>,
    #[doc = "When the password was last reset on the environment."]
    #[serde(rename = "passwordLastReset", with = "azure_core::date::rfc3339::option")]
    pub password_last_reset: Option<time::OffsetDateTime>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
    #[doc = "Details of the status of an operation."]
    #[serde(rename = "latestOperationResult", default, skip_serializing_if = "Option::is_none")]
    pub latest_operation_result: Option<LatestOperationResult>,
}
impl EnvironmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an environment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentPropertiesFragment {
    #[doc = "Represents a VM and the setting Id it was created for."]
    #[serde(rename = "resourceSets", default, skip_serializing_if = "Option::is_none")]
    pub resource_sets: Option<ResourceSetFragment>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl EnvironmentPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents settings of an environment, from which environment instances would be created"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentSetting {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of an environment setting"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnvironmentSettingProperties>,
}
impl EnvironmentSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Settings related to creating an environment setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentSettingCreationParameters {
    #[doc = "Represents resource specific settings"]
    #[serde(rename = "resourceSettingCreationParameters")]
    pub resource_setting_creation_parameters: ResourceSettingCreationParameters,
}
impl EnvironmentSettingCreationParameters {
    pub fn new(resource_setting_creation_parameters: ResourceSettingCreationParameters) -> Self {
        Self {
            resource_setting_creation_parameters,
        }
    }
}
#[doc = "Represents settings of an environment, from which environment instances would be created"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentSettingFragment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of an environment setting"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnvironmentSettingPropertiesFragment>,
}
impl EnvironmentSettingFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an environment setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentSettingProperties {
    #[doc = "Describes the readiness of this environment setting"]
    #[serde(rename = "publishingState", default, skip_serializing_if = "Option::is_none")]
    pub publishing_state: Option<environment_setting_properties::PublishingState>,
    #[doc = "Describes the user's progress in configuring their environment setting"]
    #[serde(rename = "configurationState", default, skip_serializing_if = "Option::is_none")]
    pub configuration_state: Option<environment_setting_properties::ConfigurationState>,
    #[doc = "Describes the environment and its resource settings"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Brief title describing the environment and its resource settings"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Represents resource specific settings"]
    #[serde(rename = "resourceSettings")]
    pub resource_settings: ResourceSettings,
    #[doc = "Time when the template VM was last changed."]
    #[serde(rename = "lastChanged", with = "azure_core::date::rfc3339::option")]
    pub last_changed: Option<time::OffsetDateTime>,
    #[doc = "Time when the template VM was last sent for publishing."]
    #[serde(rename = "lastPublished", with = "azure_core::date::rfc3339::option")]
    pub last_published: Option<time::OffsetDateTime>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
    #[doc = "Details of the status of an operation."]
    #[serde(rename = "latestOperationResult", default, skip_serializing_if = "Option::is_none")]
    pub latest_operation_result: Option<LatestOperationResult>,
}
impl EnvironmentSettingProperties {
    pub fn new(resource_settings: ResourceSettings) -> Self {
        Self {
            publishing_state: None,
            configuration_state: None,
            description: None,
            title: None,
            resource_settings,
            last_changed: None,
            last_published: None,
            provisioning_state: None,
            unique_identifier: None,
            latest_operation_result: None,
        }
    }
}
pub mod environment_setting_properties {
    use super::*;
    #[doc = "Describes the readiness of this environment setting"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublishingState")]
    pub enum PublishingState {
        Draft,
        Publishing,
        Published,
        PublishFailed,
        Scaling,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublishingState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublishingState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublishingState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Draft => serializer.serialize_unit_variant("PublishingState", 0u32, "Draft"),
                Self::Publishing => serializer.serialize_unit_variant("PublishingState", 1u32, "Publishing"),
                Self::Published => serializer.serialize_unit_variant("PublishingState", 2u32, "Published"),
                Self::PublishFailed => serializer.serialize_unit_variant("PublishingState", 3u32, "PublishFailed"),
                Self::Scaling => serializer.serialize_unit_variant("PublishingState", 4u32, "Scaling"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Describes the user's progress in configuring their environment setting"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConfigurationState")]
    pub enum ConfigurationState {
        NotApplicable,
        Completed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConfigurationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConfigurationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConfigurationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotApplicable => serializer.serialize_unit_variant("ConfigurationState", 0u32, "NotApplicable"),
                Self::Completed => serializer.serialize_unit_variant("ConfigurationState", 1u32, "Completed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of an environment setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentSettingPropertiesFragment {
    #[doc = "Describes the user's progress in configuring their environment setting"]
    #[serde(rename = "configurationState", default, skip_serializing_if = "Option::is_none")]
    pub configuration_state: Option<environment_setting_properties_fragment::ConfigurationState>,
    #[doc = "Describes the environment and its resource settings"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Brief title describing the environment and its resource settings"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Represents resource specific settings"]
    #[serde(rename = "resourceSettings", default, skip_serializing_if = "Option::is_none")]
    pub resource_settings: Option<ResourceSettingsFragment>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl EnvironmentSettingPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod environment_setting_properties_fragment {
    use super::*;
    #[doc = "Describes the user's progress in configuring their environment setting"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConfigurationState")]
    pub enum ConfigurationState {
        NotApplicable,
        Completed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConfigurationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConfigurationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConfigurationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotApplicable => serializer.serialize_unit_variant("ConfigurationState", 0u32, "NotApplicable"),
                Self::Completed => serializer.serialize_unit_variant("ConfigurationState", 1u32, "Completed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents a size category supported by this Lab Account (small, medium or large)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentSize {
    #[doc = "The size category"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<environment_size::Name>,
    #[doc = "Represents a set of compute sizes that can serve this given size type"]
    #[serde(rename = "vmSizes", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_sizes: Vec<SizeInfo>,
    #[doc = "The pay-as-you-go dollar price per hour this size will cost. It does not include discounts and may not reflect the actual price the size will cost. This is the maximum price of all prices within this tier."]
    #[serde(rename = "maxPrice", default, skip_serializing_if = "Option::is_none")]
    pub max_price: Option<f64>,
    #[doc = "The number of cores a VM of this size has. This is the minimum number of cores within this tier."]
    #[serde(rename = "minNumberOfCores", default, skip_serializing_if = "Option::is_none")]
    pub min_number_of_cores: Option<i32>,
    #[doc = "The amount of memory available (in GB). This is the minimum amount of memory within this tier."]
    #[serde(rename = "minMemory", default, skip_serializing_if = "Option::is_none")]
    pub min_memory: Option<f64>,
}
impl EnvironmentSize {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod environment_size {
    use super::*;
    #[doc = "The size category"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Basic,
        Standard,
        Performance,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Basic => serializer.serialize_unit_variant("Name", 0u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("Name", 1u32, "Standard"),
                Self::Performance => serializer.serialize_unit_variant("Name", 2u32, "Performance"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents a size category supported by this Lab Account (small, medium or large)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentSizeFragment {
    #[doc = "The size category"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<environment_size_fragment::Name>,
    #[doc = "Represents a set of compute sizes that can serve this given size type"]
    #[serde(rename = "vmSizes", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_sizes: Vec<SizeInfoFragment>,
}
impl EnvironmentSizeFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod environment_size_fragment {
    use super::*;
    #[doc = "The size category"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Basic,
        Standard,
        Performance,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Basic => serializer.serialize_unit_variant("Name", 0u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("Name", 1u32, "Standard"),
                Self::Performance => serializer.serialize_unit_variant("Name", 2u32, "Performance"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents an image from the Azure Marketplace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryImage {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The gallery image properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryImageProperties>,
}
impl GalleryImage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an image from the Azure Marketplace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryImageFragment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The gallery image properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryImagePropertiesFragment>,
}
impl GalleryImageFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The gallery image properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryImageProperties {
    #[doc = "The author of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[doc = "The creation date of the gallery image."]
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The description of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The reference information for an Azure Marketplace image."]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<GalleryImageReference>,
    #[doc = "The icon of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[doc = "Indicates whether this gallery image is enabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Indicates whether this gallery has been overridden for this lab account"]
    #[serde(rename = "isOverride", default, skip_serializing_if = "Option::is_none")]
    pub is_override: Option<bool>,
    #[doc = "The third party plan that applies to this image"]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Indicates if the plan has been authorized for programmatic deployment."]
    #[serde(rename = "isPlanAuthorized", default, skip_serializing_if = "Option::is_none")]
    pub is_plan_authorized: Option<bool>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
    #[doc = "Details of the status of an operation."]
    #[serde(rename = "latestOperationResult", default, skip_serializing_if = "Option::is_none")]
    pub latest_operation_result: Option<LatestOperationResult>,
}
impl GalleryImageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The gallery image properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryImagePropertiesFragment {
    #[doc = "Indicates whether this gallery image is enabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Indicates whether this gallery has been overridden for this lab account"]
    #[serde(rename = "isOverride", default, skip_serializing_if = "Option::is_none")]
    pub is_override: Option<bool>,
    #[doc = "Indicates if the plan has been authorized for programmatic deployment."]
    #[serde(rename = "isPlanAuthorized", default, skip_serializing_if = "Option::is_none")]
    pub is_plan_authorized: Option<bool>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl GalleryImagePropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The reference information for an Azure Marketplace image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryImageReference {
    #[doc = "The offer of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "The publisher of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The SKU of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "The OS type of the gallery image."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The version of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl GalleryImageReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The reference information for an Azure Marketplace image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryImageReferenceFragment {
    #[doc = "The offer of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "The publisher of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The SKU of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "The OS type of the gallery image."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The version of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl GalleryImageReferenceFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the environments details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetEnvironmentResponse {
    #[doc = "This represents the details about a User's environment and its state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<EnvironmentDetails>,
}
impl GetEnvironmentResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the PersonalPreferences for the user"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetPersonalPreferencesResponse {
    #[doc = "Id to be used by the cache orchestrator"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Array of favorite lab resource ids"]
    #[serde(rename = "favoriteLabResourceIds", default, skip_serializing_if = "Vec::is_empty")]
    pub favorite_lab_resource_ids: Vec<String>,
}
impl GetPersonalPreferencesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model from the GetRegionalAvailability action"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetRegionalAvailabilityResponse {
    #[doc = "Availability information for different size categories per region"]
    #[serde(rename = "regionalAvailability", default, skip_serializing_if = "Vec::is_empty")]
    pub regional_availability: Vec<RegionalAvailability>,
}
impl GetRegionalAvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a lab."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Lab {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a Lab."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabProperties>,
}
impl Lab {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a lab account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a Lab Account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabAccountProperties>,
}
impl LabAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a lab account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabAccountFragment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a Lab Account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabAccountPropertiesFragment>,
}
impl LabAccountFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Lab Account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabAccountProperties {
    #[doc = "Represents the size configuration under the lab account"]
    #[serde(rename = "sizeConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub size_configuration: Option<SizeConfigurationProperties>,
    #[doc = "Represents if region selection is enabled"]
    #[serde(rename = "enabledRegionSelection", default, skip_serializing_if = "Option::is_none")]
    pub enabled_region_selection: Option<bool>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
    #[doc = "Details of the status of an operation."]
    #[serde(rename = "latestOperationResult", default, skip_serializing_if = "Option::is_none")]
    pub latest_operation_result: Option<LatestOperationResult>,
}
impl LabAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Lab Account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabAccountPropertiesFragment {
    #[doc = "Represents if region selection is enabled"]
    #[serde(rename = "enabledRegionSelection", default, skip_serializing_if = "Option::is_none")]
    pub enabled_region_selection: Option<bool>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl LabAccountPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Settings related to creating a lab"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabCreationParameters {
    #[doc = "Maximum number of users allowed in the lab."]
    #[serde(rename = "maxUsersInLab", default, skip_serializing_if = "Option::is_none")]
    pub max_users_in_lab: Option<i32>,
}
impl LabCreationParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This represents the details about a lab that the User is in, and its state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabDetails {
    #[doc = "Name of the lab"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The provisioning state of the lab."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The Id of the lab."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The maximum duration a user can use a VM in this lab."]
    #[serde(rename = "usageQuota", default, skip_serializing_if = "Option::is_none")]
    pub usage_quota: Option<String>,
}
impl LabDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a lab."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabFragment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a Lab."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabPropertiesFragment>,
}
impl LabFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Lab."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabProperties {
    #[doc = "Maximum number of users allowed in the lab."]
    #[serde(rename = "maxUsersInLab", default, skip_serializing_if = "Option::is_none")]
    pub max_users_in_lab: Option<i32>,
    #[doc = "Maximum value MaxUsersInLab can be set to, as specified by the service"]
    #[serde(rename = "userQuota", default, skip_serializing_if = "Option::is_none")]
    pub user_quota: Option<i32>,
    #[doc = "Invitation code that users can use to join a lab."]
    #[serde(rename = "invitationCode", default, skip_serializing_if = "Option::is_none")]
    pub invitation_code: Option<String>,
    #[doc = "Object id of the user that created the lab."]
    #[serde(rename = "createdByObjectId", default, skip_serializing_if = "Option::is_none")]
    pub created_by_object_id: Option<String>,
    #[doc = "Maximum duration a user can use an environment for in the lab."]
    #[serde(rename = "usageQuota", default, skip_serializing_if = "Option::is_none")]
    pub usage_quota: Option<String>,
    #[doc = "Lab user access mode (open to all vs. restricted to those listed on the lab)."]
    #[serde(rename = "userAccessMode", default, skip_serializing_if = "Option::is_none")]
    pub user_access_mode: Option<lab_properties::UserAccessMode>,
    #[doc = "Lab creator name"]
    #[serde(rename = "createdByUserPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub created_by_user_principal_name: Option<String>,
    #[doc = "Creation date for the lab"]
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
    #[doc = "Details of the status of an operation."]
    #[serde(rename = "latestOperationResult", default, skip_serializing_if = "Option::is_none")]
    pub latest_operation_result: Option<LatestOperationResult>,
}
impl LabProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod lab_properties {
    use super::*;
    #[doc = "Lab user access mode (open to all vs. restricted to those listed on the lab)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UserAccessMode")]
    pub enum UserAccessMode {
        Restricted,
        Open,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UserAccessMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UserAccessMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UserAccessMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Restricted => serializer.serialize_unit_variant("UserAccessMode", 0u32, "Restricted"),
                Self::Open => serializer.serialize_unit_variant("UserAccessMode", 1u32, "Open"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of a Lab."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabPropertiesFragment {
    #[doc = "Maximum number of users allowed in the lab."]
    #[serde(rename = "maxUsersInLab", default, skip_serializing_if = "Option::is_none")]
    pub max_users_in_lab: Option<i32>,
    #[doc = "Maximum duration a user can use an environment for in the lab."]
    #[serde(rename = "usageQuota", default, skip_serializing_if = "Option::is_none")]
    pub usage_quota: Option<String>,
    #[doc = "Lab user access mode (open to all vs. restricted to those listed on the lab)."]
    #[serde(rename = "userAccessMode", default, skip_serializing_if = "Option::is_none")]
    pub user_access_mode: Option<lab_properties_fragment::UserAccessMode>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl LabPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod lab_properties_fragment {
    use super::*;
    #[doc = "Lab user access mode (open to all vs. restricted to those listed on the lab)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UserAccessMode")]
    pub enum UserAccessMode {
        Restricted,
        Open,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UserAccessMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UserAccessMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UserAccessMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Restricted => serializer.serialize_unit_variant("UserAccessMode", 0u32, "Restricted"),
                Self::Open => serializer.serialize_unit_variant("UserAccessMode", 1u32, "Open"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Details of the status of an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LatestOperationResult {
    #[doc = "The current status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Error code on failure."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "The error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Request URI of the operation."]
    #[serde(rename = "requestUri", default, skip_serializing_if = "Option::is_none")]
    pub request_uri: Option<String>,
    #[doc = "The HttpMethod - PUT/POST/DELETE for the operation."]
    #[serde(rename = "httpMethod", default, skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    #[doc = "The URL to use to check long-running operation status"]
    #[serde(rename = "operationUrl", default, skip_serializing_if = "Option::is_none")]
    pub operation_url: Option<String>,
}
impl LatestOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the status of an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LatestOperationResultFragment {}
impl LatestOperationResultFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the payload to list environments owned by a user"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListEnvironmentsPayload {
    #[doc = "The resource Id of the lab"]
    #[serde(rename = "labId", default, skip_serializing_if = "Option::is_none")]
    pub lab_id: Option<String>,
}
impl ListEnvironmentsPayload {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the list of environments owned by a user"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListEnvironmentsResponse {
    #[doc = "List of all the environments"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub environments: Vec<EnvironmentDetails>,
}
impl ListEnvironmentsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lists the labs owned by a user"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListLabsResponse {
    #[doc = "List of all the labs"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labs: Vec<LabDetails>,
}
impl ListLabsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network details of the environment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterface {
    #[doc = "PrivateIp address of the Compute VM"]
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "Connection information for Linux"]
    #[serde(rename = "sshAuthority", default, skip_serializing_if = "Option::is_none")]
    pub ssh_authority: Option<String>,
    #[doc = "Connection information for Windows"]
    #[serde(rename = "rdpAuthority", default, skip_serializing_if = "Option::is_none")]
    pub rdp_authority: Option<String>,
    #[doc = "Username of the VM"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
impl NetworkInterface {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network details of the environment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceFragment {}
impl NetworkInterfaceFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Payload to get the status of an operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationBatchStatusPayload {
    #[doc = "The operation url of long running operation"]
    pub urls: Vec<String>,
}
impl OperationBatchStatusPayload {
    pub fn new(urls: Vec<String>) -> Self {
        Self { urls }
    }
}
#[doc = "Status Details of the long running operation for an environment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationBatchStatusResponse {
    #[doc = "Gets a collection of items that contain the operation url and status."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<OperationBatchStatusResponseItem>,
}
impl OperationBatchStatusResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the status of an operation that used the batch API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationBatchStatusResponseItem {
    #[doc = "status of the long running operation for an environment"]
    #[serde(rename = "operationUrl", default, skip_serializing_if = "Option::is_none")]
    pub operation_url: Option<String>,
    #[doc = "status of the long running operation for an environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl OperationBatchStatusResponseItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error details for the operation in case of a failure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationError {
    #[doc = "The error code of the operation error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message of the operation error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl OperationError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The REST API operation supported by ManagedLab ResourceProvider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetadata {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that describes the operations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationMetadataDisplay>,
}
impl OperationMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object that describes the operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetadataDisplay {
    #[doc = "Friendly name of the resource provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource type on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operation type: read, write, delete, listKeys/action, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Friendly name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationMetadataDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Operation Result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[doc = "The operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Error details for the operation in case of a failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<OperationError>,
}
impl OperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Payload to get the status of an operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatusPayload {
    #[doc = "The operation url of long running operation"]
    #[serde(rename = "operationUrl")]
    pub operation_url: String,
}
impl OperationStatusPayload {
    pub fn new(operation_url: String) -> Self {
        Self { operation_url }
    }
}
#[doc = "Status Details of the long running operation for an environment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatusResponse {
    #[doc = "status of the long running operation for an environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl OperationStatusResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents payload for any Environment operations like get, start, stop, connect"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PersonalPreferencesOperationsPayload {
    #[doc = "Resource Id of the lab account"]
    #[serde(rename = "labAccountResourceId", default, skip_serializing_if = "Option::is_none")]
    pub lab_account_resource_id: Option<String>,
    #[doc = "Enum indicating if user is adding or removing a favorite lab"]
    #[serde(rename = "addRemove", default, skip_serializing_if = "Option::is_none")]
    pub add_remove: Option<personal_preferences_operations_payload::AddRemove>,
    #[doc = "Resource Id of the lab to add/remove from the favorites list"]
    #[serde(rename = "labResourceId", default, skip_serializing_if = "Option::is_none")]
    pub lab_resource_id: Option<String>,
}
impl PersonalPreferencesOperationsPayload {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod personal_preferences_operations_payload {
    use super::*;
    #[doc = "Enum indicating if user is adding or removing a favorite lab"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AddRemove")]
    pub enum AddRemove {
        Add,
        Remove,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AddRemove {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AddRemove {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AddRemove {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Add => serializer.serialize_unit_variant("AddRemove", 0u32, "Add"),
                Self::Remove => serializer.serialize_unit_variant("AddRemove", 1u32, "Remove"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result of the request to list REST API operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderOperationResult {
    #[doc = "List of operations supported by the resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationMetadata>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProviderOperationResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProviderOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Payload for Publish operation on EnvironmentSetting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublishPayload {
    #[doc = "Whether to use existing VM custom image when publishing."]
    #[serde(rename = "useExistingImage", default, skip_serializing_if = "Option::is_none")]
    pub use_existing_image: Option<bool>,
}
impl PublishPayload {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of a Reference Vm"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceVm {
    #[doc = "The username of the virtual machine"]
    #[serde(rename = "userName")]
    pub user_name: String,
    #[doc = "The password of the virtual machine. This will be set to null in GET resource API"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Details about the state of the reference virtual machine."]
    #[serde(rename = "vmStateDetails", default, skip_serializing_if = "Option::is_none")]
    pub vm_state_details: Option<VmStateDetails>,
    #[doc = "VM resource Id for the environment"]
    #[serde(rename = "vmResourceId", default, skip_serializing_if = "Option::is_none")]
    pub vm_resource_id: Option<String>,
}
impl ReferenceVm {
    pub fn new(user_name: String) -> Self {
        Self {
            user_name,
            password: None,
            vm_state_details: None,
            vm_resource_id: None,
        }
    }
}
#[doc = "Creation parameters for Reference Vm"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceVmCreationParameters {
    #[doc = "The username of the virtual machine"]
    #[serde(rename = "userName")]
    pub user_name: String,
    #[doc = "The password of the virtual machine."]
    pub password: String,
}
impl ReferenceVmCreationParameters {
    pub fn new(user_name: String, password: String) -> Self {
        Self { user_name, password }
    }
}
#[doc = "Details of a Reference Vm"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReferenceVmFragment {
    #[doc = "The username of the virtual machine"]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "The password of the virtual machine. This will be set to null in GET resource API"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl ReferenceVmFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The availability information of sizes across regions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegionalAvailability {
    #[doc = "Corresponding region"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "List of all the size information for the region"]
    #[serde(rename = "sizeAvailabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub size_availabilities: Vec<SizeAvailability>,
}
impl RegionalAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents payload for Register action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegisterPayload {
    #[doc = "The registration code of the lab."]
    #[serde(rename = "registrationCode", default, skip_serializing_if = "Option::is_none")]
    pub registration_code: Option<String>,
}
impl RegisterPayload {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the payload for resetting passwords."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResetPasswordPayload {
    #[doc = "The resourceId of the environment"]
    #[serde(rename = "environmentId")]
    pub environment_id: String,
    #[doc = "The username for which the password will be reset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The password to assign to the user specified in"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl ResetPasswordPayload {
    pub fn new(environment_id: String) -> Self {
        Self {
            environment_id,
            username: None,
            password: None,
        }
    }
}
#[doc = "An Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "The identifier of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The tags of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a VM and the setting Id it was created for."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSet {
    #[doc = "VM resource Id for the environment"]
    #[serde(rename = "vmResourceId", default, skip_serializing_if = "Option::is_none")]
    pub vm_resource_id: Option<String>,
    #[doc = "resourceSettingId for the environment"]
    #[serde(rename = "resourceSettingId", default, skip_serializing_if = "Option::is_none")]
    pub resource_setting_id: Option<String>,
}
impl ResourceSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a VM and the setting Id it was created for."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSetFragment {
    #[doc = "VM resource Id for the environment"]
    #[serde(rename = "vmResourceId", default, skip_serializing_if = "Option::is_none")]
    pub vm_resource_id: Option<String>,
    #[doc = "resourceSettingId for the environment"]
    #[serde(rename = "resourceSettingId", default, skip_serializing_if = "Option::is_none")]
    pub resource_setting_id: Option<String>,
}
impl ResourceSetFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents resource specific settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSettingCreationParameters {
    #[doc = "The location where the virtual machine will live"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The name of the resource setting"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource id of the gallery image used for creating the virtual machine"]
    #[serde(rename = "galleryImageResourceId")]
    pub gallery_image_resource_id: String,
    #[doc = "The size of the virtual machine"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<resource_setting_creation_parameters::Size>,
    #[doc = "Creation parameters for Reference Vm"]
    #[serde(rename = "referenceVmCreationParameters")]
    pub reference_vm_creation_parameters: ReferenceVmCreationParameters,
}
impl ResourceSettingCreationParameters {
    pub fn new(gallery_image_resource_id: String, reference_vm_creation_parameters: ReferenceVmCreationParameters) -> Self {
        Self {
            location: None,
            name: None,
            gallery_image_resource_id,
            size: None,
            reference_vm_creation_parameters,
        }
    }
}
pub mod resource_setting_creation_parameters {
    use super::*;
    #[doc = "The size of the virtual machine"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Size")]
    pub enum Size {
        Basic,
        Standard,
        Performance,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Size {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Size {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Size {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Basic => serializer.serialize_unit_variant("Size", 0u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("Size", 1u32, "Standard"),
                Self::Performance => serializer.serialize_unit_variant("Size", 2u32, "Performance"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents resource specific settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSettings {
    #[doc = "The unique id of the resource setting"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource id of the gallery image used for creating the virtual machine"]
    #[serde(rename = "galleryImageResourceId", default, skip_serializing_if = "Option::is_none")]
    pub gallery_image_resource_id: Option<String>,
    #[doc = "The name of the image used to created the environment setting"]
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[doc = "The size of the virtual machine"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<resource_settings::Size>,
    #[doc = "The translated compute cores of the virtual machine"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cores: Option<i32>,
    #[doc = "Details of a Reference Vm"]
    #[serde(rename = "referenceVm")]
    pub reference_vm: ReferenceVm,
}
impl ResourceSettings {
    pub fn new(reference_vm: ReferenceVm) -> Self {
        Self {
            id: None,
            gallery_image_resource_id: None,
            image_name: None,
            size: None,
            cores: None,
            reference_vm,
        }
    }
}
pub mod resource_settings {
    use super::*;
    #[doc = "The size of the virtual machine"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Size")]
    pub enum Size {
        Basic,
        Standard,
        Performance,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Size {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Size {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Size {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Basic => serializer.serialize_unit_variant("Size", 0u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("Size", 1u32, "Standard"),
                Self::Performance => serializer.serialize_unit_variant("Size", 2u32, "Performance"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents resource specific settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSettingsFragment {
    #[doc = "The resource id of the gallery image used for creating the virtual machine"]
    #[serde(rename = "galleryImageResourceId", default, skip_serializing_if = "Option::is_none")]
    pub gallery_image_resource_id: Option<String>,
    #[doc = "The size of the virtual machine"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<resource_settings_fragment::Size>,
    #[doc = "Details of a Reference Vm"]
    #[serde(rename = "referenceVm", default, skip_serializing_if = "Option::is_none")]
    pub reference_vm: Option<ReferenceVmFragment>,
}
impl ResourceSettingsFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_settings_fragment {
    use super::*;
    #[doc = "The size of the virtual machine"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Size")]
    pub enum Size {
        Basic,
        Standard,
        Performance,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Size {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Size {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Size {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Basic => serializer.serialize_unit_variant("Size", 0u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("Size", 1u32, "Standard"),
                Self::Performance => serializer.serialize_unit_variant("Size", 2u32, "Performance"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseWithContinuationEnvironmentSetting {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EnvironmentSetting>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResponseWithContinuationEnvironmentSetting {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResponseWithContinuationEnvironmentSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseWithContinuationEnvironment {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Environment>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResponseWithContinuationEnvironment {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResponseWithContinuationEnvironment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseWithContinuationGalleryImage {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GalleryImage>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResponseWithContinuationGalleryImage {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResponseWithContinuationGalleryImage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseWithContinuationLabAccount {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LabAccount>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResponseWithContinuationLabAccount {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResponseWithContinuationLabAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseWithContinuationLab {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Lab>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResponseWithContinuationLab {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResponseWithContinuationLab {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseWithContinuationUser {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<User>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResponseWithContinuationUser {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResponseWithContinuationUser {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the size information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SizeAvailability {
    #[doc = "The category of the size (Basic, Standard, Performance)."]
    #[serde(rename = "sizeCategory", default, skip_serializing_if = "Option::is_none")]
    pub size_category: Option<size_availability::SizeCategory>,
    #[doc = "Whether or not this size category is available"]
    #[serde(rename = "isAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_available: Option<bool>,
}
impl SizeAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod size_availability {
    use super::*;
    #[doc = "The category of the size (Basic, Standard, Performance)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SizeCategory")]
    pub enum SizeCategory {
        Basic,
        Standard,
        Performance,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SizeCategory {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SizeCategory {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SizeCategory {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Basic => serializer.serialize_unit_variant("SizeCategory", 0u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("SizeCategory", 1u32, "Standard"),
                Self::Performance => serializer.serialize_unit_variant("SizeCategory", 2u32, "Performance"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents the size configuration under the lab account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SizeConfigurationProperties {
    #[doc = "Represents a list of size categories supported by this Lab Account (Small, Medium, Large)"]
    #[serde(rename = "environmentSizes", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_sizes: Vec<EnvironmentSize>,
}
impl SizeConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the size configuration under the lab account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SizeConfigurationPropertiesFragment {
    #[doc = "Represents a list of size categories supported by this Lab Account (Small, Medium, Large)"]
    #[serde(rename = "environmentSizes", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_sizes: Vec<EnvironmentSizeFragment>,
}
impl SizeConfigurationPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains detailed information about a size"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SizeInfo {
    #[doc = "Represents the actual compute size, e.g. Standard_A2_v2."]
    #[serde(rename = "computeSize", default, skip_serializing_if = "Option::is_none")]
    pub compute_size: Option<String>,
    #[doc = "The pay-as-you-go price per hour this size will cost. It does not include discounts and may not reflect the actual price the size will cost."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[doc = "The number of cores a VM of this size has."]
    #[serde(rename = "numberOfCores", default, skip_serializing_if = "Option::is_none")]
    pub number_of_cores: Option<i32>,
    #[doc = "The amount of memory available (in GB)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<f64>,
}
impl SizeInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains detailed information about a size"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SizeInfoFragment {
    #[doc = "Represents the actual compute size, e.g. Standard_A2_v2."]
    #[serde(rename = "computeSize", default, skip_serializing_if = "Option::is_none")]
    pub compute_size: Option<String>,
    #[doc = "The pay-as-you-go price per hour this size will cost. It does not include discounts and may not reflect the actual price the size will cost."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[doc = "The number of cores a VM of this size has."]
    #[serde(rename = "numberOfCores", default, skip_serializing_if = "Option::is_none")]
    pub number_of_cores: Option<i32>,
    #[doc = "The amount of memory available (in GB)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<f64>,
}
impl SizeInfoFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The User registered to a lab"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct User {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Lab User properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserProperties>,
}
impl User {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The User registered to a lab"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserFragment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Lab User properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserPropertiesFragment>,
}
impl UserFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lab User properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserProperties {
    #[doc = "The user email address, as it was specified during registration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The user family name, as it was specified during registration."]
    #[serde(rename = "familyName", default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[doc = "The user given name, as it was specified during registration."]
    #[serde(rename = "givenName", default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The user tenant ID, as it was specified during registration."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "How long the user has used his VMs in this lab"]
    #[serde(rename = "totalUsage", default, skip_serializing_if = "Option::is_none")]
    pub total_usage: Option<String>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
    #[doc = "Details of the status of an operation."]
    #[serde(rename = "latestOperationResult", default, skip_serializing_if = "Option::is_none")]
    pub latest_operation_result: Option<LatestOperationResult>,
}
impl UserProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lab User properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserPropertiesFragment {
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl UserPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the backing virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineDetails {
    #[doc = "Provisioning state of the Dtl VM"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Connection information for Windows"]
    #[serde(rename = "rdpAuthority", default, skip_serializing_if = "Option::is_none")]
    pub rdp_authority: Option<String>,
    #[doc = "Connection information for Linux"]
    #[serde(rename = "sshAuthority", default, skip_serializing_if = "Option::is_none")]
    pub ssh_authority: Option<String>,
    #[doc = "PrivateIp address of the compute VM"]
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "Compute VM login user name"]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "Last known compute power state captured in DTL"]
    #[serde(rename = "lastKnownPowerState", default, skip_serializing_if = "Option::is_none")]
    pub last_known_power_state: Option<String>,
}
impl VirtualMachineDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about the state of the reference virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmStateDetails {
    #[doc = "The RdpAuthority property is a server DNS host name or IP address followed by the service port number for RDP (Remote Desktop Protocol)."]
    #[serde(rename = "rdpAuthority", default, skip_serializing_if = "Option::is_none")]
    pub rdp_authority: Option<String>,
    #[doc = "The SshAuthority property is a server DNS host name or IP address followed by the service port number for SSH."]
    #[serde(rename = "sshAuthority", default, skip_serializing_if = "Option::is_none")]
    pub ssh_authority: Option<String>,
    #[doc = "The power state of the reference virtual machine."]
    #[serde(rename = "powerState", default, skip_serializing_if = "Option::is_none")]
    pub power_state: Option<String>,
    #[doc = "Last known compute power state captured in DTL"]
    #[serde(rename = "lastKnownPowerState", default, skip_serializing_if = "Option::is_none")]
    pub last_known_power_state: Option<String>,
}
impl VmStateDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about the state of the reference virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmStateDetailsFragment {}
impl VmStateDetailsFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
