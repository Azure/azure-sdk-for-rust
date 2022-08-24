#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Represents the base class for all object models."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmBaseModel {
    #[doc = "The path ID that uniquely identifies the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The object name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The hierarchical type of the object."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ArmBaseModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role Addon"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Addon {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "Addon type."]
    pub kind: addon::Kind,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Addon {
    pub fn new(kind: addon::Kind) -> Self {
        Self {
            arm_base_model: ArmBaseModel::default(),
            kind,
            system_data: None,
        }
    }
}
pub mod addon {
    use super::*;
    #[doc = "Addon type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        IotEdge,
        ArcForKubernetes,
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
                Self::IotEdge => serializer.serialize_unit_variant("Kind", 0u32, "IotEdge"),
                Self::ArcForKubernetes => serializer.serialize_unit_variant("Kind", 1u32, "ArcForKubernetes"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Collection of all the Role addon on the Azure Stack Edge device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddonList {
    #[doc = "The Value."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Addon>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AddonList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AddonList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The shipping address of the customer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Address {
    #[doc = "The address line1."]
    #[serde(rename = "addressLine1", default, skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    #[doc = "The address line2."]
    #[serde(rename = "addressLine2", default, skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[doc = "The address line3."]
    #[serde(rename = "addressLine3", default, skip_serializing_if = "Option::is_none")]
    pub address_line3: Option<String>,
    #[doc = "The postal code."]
    #[serde(rename = "postalCode", default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "The city name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "The state name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The country name."]
    pub country: String,
}
impl Address {
    pub fn new(country: String) -> Self {
        Self {
            address_line1: None,
            address_line2: None,
            address_line3: None,
            postal_code: None,
            city: None,
            state: None,
            country,
        }
    }
}
#[doc = "Alert on the data box edge/gateway device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Alert {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of alert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProperties>,
}
impl Alert {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error details for the alert."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertErrorDetails {
    #[doc = "Error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Error Message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Number of occurrences."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occurrences: Option<i32>,
}
impl AlertErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of alerts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertList {
    #[doc = "The value."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Alert>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AlertList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AlertList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of alert."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertProperties {
    #[doc = "Alert title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Alert type."]
    #[serde(rename = "alertType", default, skip_serializing_if = "Option::is_none")]
    pub alert_type: Option<String>,
    #[doc = "UTC time when the alert appeared."]
    #[serde(rename = "appearedAtDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub appeared_at_date_time: Option<time::OffsetDateTime>,
    #[doc = "Alert recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<String>,
    #[doc = "Severity of the alert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<alert_properties::Severity>,
    #[doc = "Error details for the alert."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<AlertErrorDetails>,
    #[doc = "Alert details."]
    #[serde(rename = "detailedInformation", default, skip_serializing_if = "Option::is_none")]
    pub detailed_information: Option<serde_json::Value>,
}
impl AlertProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod alert_properties {
    use super::*;
    #[doc = "Severity of the alert."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Severity")]
    pub enum Severity {
        Informational,
        Warning,
        Critical,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Severity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Severity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Severity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Informational => serializer.serialize_unit_variant("Severity", 0u32, "Informational"),
                Self::Warning => serializer.serialize_unit_variant("Severity", 1u32, "Warning"),
                Self::Critical => serializer.serialize_unit_variant("Severity", 2u32, "Critical"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Arc Addon."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArcAddon {
    #[serde(flatten)]
    pub addon: Addon,
    #[doc = "Arc addon properties."]
    pub properties: ArcAddonProperties,
    #[doc = "The path ID that uniquely identifies the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The object name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The hierarchical type of the object."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ArcAddon {
    pub fn new(addon: Addon, properties: ArcAddonProperties) -> Self {
        Self {
            addon,
            properties,
            id: None,
            name: None,
            type_: None,
        }
    }
}
#[doc = "Arc addon properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArcAddonProperties {
    #[doc = "Arc resource subscription Id"]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
    #[doc = "Arc resource group name"]
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
    #[doc = "Arc resource Name"]
    #[serde(rename = "resourceName")]
    pub resource_name: String,
    #[doc = "Arc resource location"]
    #[serde(rename = "resourceLocation")]
    pub resource_location: String,
    #[doc = "Arc resource version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Host OS supported by the Arc addon."]
    #[serde(rename = "hostPlatform", default, skip_serializing_if = "Option::is_none")]
    pub host_platform: Option<arc_addon_properties::HostPlatform>,
    #[doc = "Platform where the runtime is hosted."]
    #[serde(rename = "hostPlatformType", default, skip_serializing_if = "Option::is_none")]
    pub host_platform_type: Option<arc_addon_properties::HostPlatformType>,
    #[doc = "Addon Provisioning State"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<arc_addon_properties::ProvisioningState>,
}
impl ArcAddonProperties {
    pub fn new(subscription_id: String, resource_group_name: String, resource_name: String, resource_location: String) -> Self {
        Self {
            subscription_id,
            resource_group_name,
            resource_name,
            resource_location,
            version: None,
            host_platform: None,
            host_platform_type: None,
            provisioning_state: None,
        }
    }
}
pub mod arc_addon_properties {
    use super::*;
    #[doc = "Host OS supported by the Arc addon."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HostPlatform")]
    pub enum HostPlatform {
        Windows,
        Linux,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HostPlatform {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HostPlatform {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HostPlatform {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Windows => serializer.serialize_unit_variant("HostPlatform", 0u32, "Windows"),
                Self::Linux => serializer.serialize_unit_variant("HostPlatform", 1u32, "Linux"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Platform where the runtime is hosted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HostPlatformType")]
    pub enum HostPlatformType {
        KubernetesCluster,
        #[serde(rename = "LinuxVM")]
        LinuxVm,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HostPlatformType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HostPlatformType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HostPlatformType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::KubernetesCluster => serializer.serialize_unit_variant("HostPlatformType", 0u32, "KubernetesCluster"),
                Self::LinuxVm => serializer.serialize_unit_variant("HostPlatformType", 1u32, "LinuxVM"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Addon Provisioning State"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Invalid,
        Creating,
        Created,
        Updating,
        Reconfiguring,
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
                Self::Invalid => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Invalid"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Created => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Created"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Updating"),
                Self::Reconfiguring => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Reconfiguring"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represent the secrets intended for encryption with asymmetric key pair."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AsymmetricEncryptedSecret {
    #[doc = "The value of the secret."]
    pub value: String,
    #[doc = "Thumbprint certificate used to encrypt \\\"Value\\\". If the value is unencrypted, it will be null."]
    #[serde(rename = "encryptionCertThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub encryption_cert_thumbprint: Option<String>,
    #[doc = "The algorithm used to encrypt \"Value\"."]
    #[serde(rename = "encryptionAlgorithm")]
    pub encryption_algorithm: asymmetric_encrypted_secret::EncryptionAlgorithm,
}
impl AsymmetricEncryptedSecret {
    pub fn new(value: String, encryption_algorithm: asymmetric_encrypted_secret::EncryptionAlgorithm) -> Self {
        Self {
            value,
            encryption_cert_thumbprint: None,
            encryption_algorithm,
        }
    }
}
pub mod asymmetric_encrypted_secret {
    use super::*;
    #[doc = "The algorithm used to encrypt \"Value\"."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EncryptionAlgorithm")]
    pub enum EncryptionAlgorithm {
        None,
        #[serde(rename = "AES256")]
        Aes256,
        #[serde(rename = "RSAES_PKCS1_v_1_5")]
        RsaesPkcs1V15,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EncryptionAlgorithm {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EncryptionAlgorithm {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EncryptionAlgorithm {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("EncryptionAlgorithm", 0u32, "None"),
                Self::Aes256 => serializer.serialize_unit_variant("EncryptionAlgorithm", 1u32, "AES256"),
                Self::RsaesPkcs1V15 => serializer.serialize_unit_variant("EncryptionAlgorithm", 2u32, "RSAES_PKCS1_v_1_5"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Authentication mechanism for IoT devices."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Authentication {
    #[doc = "Symmetric key for authentication."]
    #[serde(rename = "symmetricKey", default, skip_serializing_if = "Option::is_none")]
    pub symmetric_key: Option<SymmetricKey>,
}
impl Authentication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure container mapping of the endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureContainerInfo {
    #[doc = "ID of the storage account credential used to access storage."]
    #[serde(rename = "storageAccountCredentialId")]
    pub storage_account_credential_id: String,
    #[doc = "Container name (Based on the data format specified, this represents the name of Azure Files/Page blob/Block blob)."]
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "Storage format used for the file represented by the share."]
    #[serde(rename = "dataFormat")]
    pub data_format: azure_container_info::DataFormat,
}
impl AzureContainerInfo {
    pub fn new(storage_account_credential_id: String, container_name: String, data_format: azure_container_info::DataFormat) -> Self {
        Self {
            storage_account_credential_id,
            container_name,
            data_format,
        }
    }
}
pub mod azure_container_info {
    use super::*;
    #[doc = "Storage format used for the file represented by the share."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataFormat")]
    pub enum DataFormat {
        BlockBlob,
        PageBlob,
        AzureFile,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataFormat {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataFormat {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataFormat {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BlockBlob => serializer.serialize_unit_variant("DataFormat", 0u32, "BlockBlob"),
                Self::PageBlob => serializer.serialize_unit_variant("DataFormat", 1u32, "PageBlob"),
                Self::AzureFile => serializer.serialize_unit_variant("DataFormat", 2u32, "AzureFile"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The bandwidth schedule details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BandwidthSchedule {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties of the bandwidth schedule."]
    pub properties: BandwidthScheduleProperties,
}
impl BandwidthSchedule {
    pub fn new(properties: BandwidthScheduleProperties) -> Self {
        Self {
            arm_base_model: ArmBaseModel::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "The properties of the bandwidth schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BandwidthScheduleProperties {
    #[doc = "The start time of the schedule in UTC."]
    pub start: String,
    #[doc = "The stop time of the schedule in UTC."]
    pub stop: String,
    #[doc = "The bandwidth rate in Mbps."]
    #[serde(rename = "rateInMbps")]
    pub rate_in_mbps: i32,
    #[doc = "The days of the week when this schedule is applicable."]
    pub days: Vec<String>,
}
impl BandwidthScheduleProperties {
    pub fn new(start: String, stop: String, rate_in_mbps: i32, days: Vec<String>) -> Self {
        Self {
            start,
            stop,
            rate_in_mbps,
            days,
        }
    }
}
#[doc = "The collection of bandwidth schedules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BandwidthSchedulesList {
    #[doc = "The list of bandwidth schedules."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BandwidthSchedule>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BandwidthSchedulesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BandwidthSchedulesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The mapping between a particular client IP and the type of access client has on the NFS share."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientAccessRight {
    #[doc = "IP of the client."]
    pub client: String,
    #[doc = "Type of access to be allowed for the client."]
    #[serde(rename = "accessPermission")]
    pub access_permission: client_access_right::AccessPermission,
}
impl ClientAccessRight {
    pub fn new(client: String, access_permission: client_access_right::AccessPermission) -> Self {
        Self { client, access_permission }
    }
}
pub mod client_access_right {
    use super::*;
    #[doc = "Type of access to be allowed for the client."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccessPermission")]
    pub enum AccessPermission {
        NoAccess,
        ReadOnly,
        ReadWrite,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AccessPermission {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AccessPermission {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AccessPermission {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NoAccess => serializer.serialize_unit_variant("AccessPermission", 0u32, "NoAccess"),
                Self::ReadOnly => serializer.serialize_unit_variant("AccessPermission", 1u32, "ReadOnly"),
                Self::ReadWrite => serializer.serialize_unit_variant("AccessPermission", 2u32, "ReadWrite"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The preview of Virtual Machine Cloud Management from the Azure supports deploying and managing VMs on your Azure Stack Edge device from Azure Portal. \r\nFor more information, refer to: https://docs.microsoft.com/en-us/azure/databox-online/azure-stack-edge-gpu-virtual-machine-overview\r\nBy using this feature, you agree to the preview legal terms. See the https://azure.microsoft.com/en-us/support/legal/preview-supplemental-terms/ for additional details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudEdgeManagementRole {
    #[serde(flatten)]
    pub role: Role,
    #[doc = "CloudEdgeManagement Role properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudEdgeManagementRoleProperties>,
    #[doc = "The path ID that uniquely identifies the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The object name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The hierarchical type of the object."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CloudEdgeManagementRole {
    pub fn new(role: Role) -> Self {
        Self {
            role,
            properties: None,
            id: None,
            name: None,
            type_: None,
        }
    }
}
#[doc = "CloudEdgeManagement Role properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudEdgeManagementRoleProperties {
    #[doc = "Local Edge Management Status"]
    #[serde(rename = "localManagementStatus", default, skip_serializing_if = "Option::is_none")]
    pub local_management_status: Option<cloud_edge_management_role_properties::LocalManagementStatus>,
    #[doc = "Details about Edge Profile for the resource"]
    #[serde(rename = "edgeProfile", default, skip_serializing_if = "Option::is_none")]
    pub edge_profile: Option<EdgeProfile>,
    #[doc = "Role status."]
    #[serde(rename = "roleStatus")]
    pub role_status: cloud_edge_management_role_properties::RoleStatus,
}
impl CloudEdgeManagementRoleProperties {
    pub fn new(role_status: cloud_edge_management_role_properties::RoleStatus) -> Self {
        Self {
            local_management_status: None,
            edge_profile: None,
            role_status,
        }
    }
}
pub mod cloud_edge_management_role_properties {
    use super::*;
    #[doc = "Local Edge Management Status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LocalManagementStatus")]
    pub enum LocalManagementStatus {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LocalManagementStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LocalManagementStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LocalManagementStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("LocalManagementStatus", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("LocalManagementStatus", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Role status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RoleStatus")]
    pub enum RoleStatus {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RoleStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RoleStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RoleStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("RoleStatus", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("RoleStatus", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the service."]
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
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cni configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CniConfig {
    #[doc = "Cni type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Cni version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Pod Subnet"]
    #[serde(rename = "podSubnet", default, skip_serializing_if = "Option::is_none")]
    pub pod_subnet: Option<String>,
    #[doc = "Service subnet"]
    #[serde(rename = "serviceSubnet", default, skip_serializing_if = "Option::is_none")]
    pub service_subnet: Option<String>,
}
impl CniConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Compute infrastructure Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeResource {
    #[doc = "Processor count"]
    #[serde(rename = "processorCount")]
    pub processor_count: i32,
    #[doc = "Memory in GB"]
    #[serde(rename = "memoryInGB")]
    pub memory_in_gb: i64,
}
impl ComputeResource {
    pub fn new(processor_count: i32, memory_in_gb: i64) -> Self {
        Self {
            processor_count,
            memory_in_gb,
        }
    }
}
#[doc = "Contains all the contact details of the customer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactDetails {
    #[doc = "The contact person name."]
    #[serde(rename = "contactPerson")]
    pub contact_person: String,
    #[doc = "The name of the company."]
    #[serde(rename = "companyName")]
    pub company_name: String,
    #[doc = "The phone number."]
    pub phone: String,
    #[doc = "The email list."]
    #[serde(rename = "emailList")]
    pub email_list: Vec<String>,
}
impl ContactDetails {
    pub fn new(contact_person: String, company_name: String, phone: String, email_list: Vec<String>) -> Self {
        Self {
            contact_person,
            company_name,
            phone,
            email_list,
        }
    }
}
#[doc = "Represents a container on the  Data Box Edge/Gateway device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Container {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The container properties."]
    pub properties: ContainerProperties,
}
impl Container {
    pub fn new(properties: ContainerProperties) -> Self {
        Self {
            arm_base_model: ArmBaseModel::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "Collection of all the containers on the Data Box Edge/Gateway device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerList {
    #[doc = "The list of containers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Container>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ContainerList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ContainerList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The container properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerProperties {
    #[doc = "Current status of the container."]
    #[serde(rename = "containerStatus", default, skip_serializing_if = "Option::is_none")]
    pub container_status: Option<container_properties::ContainerStatus>,
    #[doc = "DataFormat for Container"]
    #[serde(rename = "dataFormat")]
    pub data_format: container_properties::DataFormat,
    #[doc = "Fields for tracking refresh job on the share or container."]
    #[serde(rename = "refreshDetails", default, skip_serializing_if = "Option::is_none")]
    pub refresh_details: Option<RefreshDetails>,
    #[doc = "The UTC time when container got created."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
}
impl ContainerProperties {
    pub fn new(data_format: container_properties::DataFormat) -> Self {
        Self {
            container_status: None,
            data_format,
            refresh_details: None,
            created_date_time: None,
        }
    }
}
pub mod container_properties {
    use super::*;
    #[doc = "Current status of the container."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ContainerStatus")]
    pub enum ContainerStatus {
        #[serde(rename = "OK")]
        Ok,
        Offline,
        Unknown,
        Updating,
        NeedsAttention,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ContainerStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ContainerStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ContainerStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ok => serializer.serialize_unit_variant("ContainerStatus", 0u32, "OK"),
                Self::Offline => serializer.serialize_unit_variant("ContainerStatus", 1u32, "Offline"),
                Self::Unknown => serializer.serialize_unit_variant("ContainerStatus", 2u32, "Unknown"),
                Self::Updating => serializer.serialize_unit_variant("ContainerStatus", 3u32, "Updating"),
                Self::NeedsAttention => serializer.serialize_unit_variant("ContainerStatus", 4u32, "NeedsAttention"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "DataFormat for Container"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataFormat")]
    pub enum DataFormat {
        BlockBlob,
        PageBlob,
        AzureFile,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataFormat {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataFormat {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataFormat {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BlockBlob => serializer.serialize_unit_variant("DataFormat", 0u32, "BlockBlob"),
                Self::PageBlob => serializer.serialize_unit_variant("DataFormat", 1u32, "PageBlob"),
                Self::AzureFile => serializer.serialize_unit_variant("DataFormat", 2u32, "AzureFile"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "DC Access code in the case of Self Managed Shipping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DcAccessCode {
    #[doc = "DCAccessCode Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DcAccessCodeProperties>,
}
impl DcAccessCode {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DCAccessCode Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DcAccessCodeProperties {
    #[doc = "DCAccess Code for the Self Managed shipment."]
    #[serde(rename = "authCode", default, skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
}
impl DcAccessCodeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Data Box Edge/Gateway device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxEdgeDevice {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "The location of the device. This is a supported and registered Azure geographical region (for example, West US, East US, or Southeast Asia). The geographical region of a device cannot be changed once it is created, but if an identical geographical region is specified on update, the request will succeed."]
    pub location: String,
    #[doc = "The list of tags that describe the device. These tags can be used to view and group this device (across resource groups)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The SKU type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The etag for the devices."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Msi identity details of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
    #[doc = "The kind of the device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<data_box_edge_device::Kind>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties of the Data Box Edge/Gateway device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataBoxEdgeDeviceProperties>,
}
impl DataBoxEdgeDevice {
    pub fn new(location: String) -> Self {
        Self {
            arm_base_model: ArmBaseModel::default(),
            location,
            tags: None,
            sku: None,
            etag: None,
            identity: None,
            kind: None,
            system_data: None,
            properties: None,
        }
    }
}
pub mod data_box_edge_device {
    use super::*;
    #[doc = "The kind of the device."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        AzureDataBoxGateway,
        AzureStackEdge,
        AzureStackHub,
        AzureModularDataCentre,
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
                Self::AzureDataBoxGateway => serializer.serialize_unit_variant("Kind", 0u32, "AzureDataBoxGateway"),
                Self::AzureStackEdge => serializer.serialize_unit_variant("Kind", 1u32, "AzureStackEdge"),
                Self::AzureStackHub => serializer.serialize_unit_variant("Kind", 2u32, "AzureStackHub"),
                Self::AzureModularDataCentre => serializer.serialize_unit_variant("Kind", 3u32, "AzureModularDataCentre"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The extended Info of the Data Box Edge/Gateway device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBoxEdgeDeviceExtendedInfo {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "The properties of the Data Box Edge/Gateway device extended info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataBoxEdgeDeviceExtendedInfoProperties>,
}
impl DataBoxEdgeDeviceExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Data Box Edge/Gateway device extended info patch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBoxEdgeDeviceExtendedInfoPatch {
    #[doc = "The Key Vault ARM Id for client secrets"]
    #[serde(rename = "clientSecretStoreId", default, skip_serializing_if = "Option::is_none")]
    pub client_secret_store_id: Option<String>,
    #[doc = "The url to access the Client Key Vault"]
    #[serde(rename = "clientSecretStoreUrl", default, skip_serializing_if = "Option::is_none")]
    pub client_secret_store_url: Option<String>,
    #[doc = "The name for Channel Integrity Key stored in the Client Key Vault"]
    #[serde(rename = "channelIntegrityKeyName", default, skip_serializing_if = "Option::is_none")]
    pub channel_integrity_key_name: Option<String>,
    #[doc = "The version of Channel Integrity Key stored in the Client Key Vault"]
    #[serde(rename = "channelIntegrityKeyVersion", default, skip_serializing_if = "Option::is_none")]
    pub channel_integrity_key_version: Option<String>,
    #[doc = "For changing or to initiate the resync to key-vault set the status to KeyVaultSyncPending, rest of the status will not be applicable."]
    #[serde(rename = "syncStatus", default, skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<data_box_edge_device_extended_info_patch::SyncStatus>,
}
impl DataBoxEdgeDeviceExtendedInfoPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_box_edge_device_extended_info_patch {
    use super::*;
    #[doc = "For changing or to initiate the resync to key-vault set the status to KeyVaultSyncPending, rest of the status will not be applicable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SyncStatus")]
    pub enum SyncStatus {
        KeyVaultSynced,
        KeyVaultSyncFailed,
        KeyVaultNotConfigured,
        KeyVaultSyncPending,
        KeyVaultSyncing,
        KeyVaultNotSynced,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SyncStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SyncStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SyncStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::KeyVaultSynced => serializer.serialize_unit_variant("SyncStatus", 0u32, "KeyVaultSynced"),
                Self::KeyVaultSyncFailed => serializer.serialize_unit_variant("SyncStatus", 1u32, "KeyVaultSyncFailed"),
                Self::KeyVaultNotConfigured => serializer.serialize_unit_variant("SyncStatus", 2u32, "KeyVaultNotConfigured"),
                Self::KeyVaultSyncPending => serializer.serialize_unit_variant("SyncStatus", 3u32, "KeyVaultSyncPending"),
                Self::KeyVaultSyncing => serializer.serialize_unit_variant("SyncStatus", 4u32, "KeyVaultSyncing"),
                Self::KeyVaultNotSynced => serializer.serialize_unit_variant("SyncStatus", 5u32, "KeyVaultNotSynced"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of the Data Box Edge/Gateway device extended info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBoxEdgeDeviceExtendedInfoProperties {
    #[doc = "The digital signature of encrypted certificate."]
    #[serde(rename = "encryptionKeyThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key_thumbprint: Option<String>,
    #[doc = "The public part of the encryption certificate. Client uses this to encrypt any secret."]
    #[serde(rename = "encryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[doc = "The Resource ID of the Resource."]
    #[serde(rename = "resourceKey", default, skip_serializing_if = "Option::is_none")]
    pub resource_key: Option<String>,
    #[doc = "The Key Vault ARM Id for client secrets"]
    #[serde(rename = "clientSecretStoreId", default, skip_serializing_if = "Option::is_none")]
    pub client_secret_store_id: Option<String>,
    #[doc = "The url to access the Client Key Vault"]
    #[serde(rename = "clientSecretStoreUrl", default, skip_serializing_if = "Option::is_none")]
    pub client_secret_store_url: Option<String>,
    #[doc = "The name of Channel Integrity Key stored in the Client Key Vault"]
    #[serde(rename = "channelIntegrityKeyName", default, skip_serializing_if = "Option::is_none")]
    pub channel_integrity_key_name: Option<String>,
    #[doc = "The version of Channel Integrity Key stored in the Client Key Vault"]
    #[serde(rename = "channelIntegrityKeyVersion", default, skip_serializing_if = "Option::is_none")]
    pub channel_integrity_key_version: Option<String>,
    #[doc = "Key vault sync status"]
    #[serde(rename = "keyVaultSyncStatus", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_sync_status: Option<data_box_edge_device_extended_info_properties::KeyVaultSyncStatus>,
    #[doc = "Device secrets, will be returned only with ODataFilter $expand=deviceSecrets"]
    #[serde(rename = "deviceSecrets", default, skip_serializing_if = "Option::is_none")]
    pub device_secrets: Option<serde_json::Value>,
}
impl DataBoxEdgeDeviceExtendedInfoProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_box_edge_device_extended_info_properties {
    use super::*;
    #[doc = "Key vault sync status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KeyVaultSyncStatus")]
    pub enum KeyVaultSyncStatus {
        KeyVaultSynced,
        KeyVaultSyncFailed,
        KeyVaultNotConfigured,
        KeyVaultSyncPending,
        KeyVaultSyncing,
        KeyVaultNotSynced,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for KeyVaultSyncStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for KeyVaultSyncStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for KeyVaultSyncStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::KeyVaultSynced => serializer.serialize_unit_variant("KeyVaultSyncStatus", 0u32, "KeyVaultSynced"),
                Self::KeyVaultSyncFailed => serializer.serialize_unit_variant("KeyVaultSyncStatus", 1u32, "KeyVaultSyncFailed"),
                Self::KeyVaultNotConfigured => serializer.serialize_unit_variant("KeyVaultSyncStatus", 2u32, "KeyVaultNotConfigured"),
                Self::KeyVaultSyncPending => serializer.serialize_unit_variant("KeyVaultSyncStatus", 3u32, "KeyVaultSyncPending"),
                Self::KeyVaultSyncing => serializer.serialize_unit_variant("KeyVaultSyncStatus", 4u32, "KeyVaultSyncing"),
                Self::KeyVaultNotSynced => serializer.serialize_unit_variant("KeyVaultSyncStatus", 5u32, "KeyVaultNotSynced"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The collection of Data Box Edge/Gateway devices."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBoxEdgeDeviceList {
    #[doc = "The list of Data Box Edge/Gateway devices."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataBoxEdgeDevice>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataBoxEdgeDeviceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataBoxEdgeDeviceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Data Box Edge/Gateway device patch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBoxEdgeDevicePatch {
    #[doc = "The tags attached to the Data Box Edge/Gateway resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Msi identity details of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
    #[doc = "The Data Box Edge/Gateway device properties patch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataBoxEdgeDevicePropertiesPatch>,
}
impl DataBoxEdgeDevicePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the Data Box Edge/Gateway device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBoxEdgeDeviceProperties {
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The status of the Data Box Edge/Gateway device."]
    #[serde(rename = "dataBoxEdgeDeviceStatus", default, skip_serializing_if = "Option::is_none")]
    pub data_box_edge_device_status: Option<data_box_edge_device_properties::DataBoxEdgeDeviceStatus>,
    #[doc = "The Serial Number of Data Box Edge/Gateway device."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "The Description of the Data Box Edge/Gateway device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The description of the Data Box Edge/Gateway device model."]
    #[serde(rename = "modelDescription", default, skip_serializing_if = "Option::is_none")]
    pub model_description: Option<String>,
    #[doc = "The type of the Data Box Edge/Gateway device."]
    #[serde(rename = "deviceType", default, skip_serializing_if = "Option::is_none")]
    pub device_type: Option<data_box_edge_device_properties::DeviceType>,
    #[doc = "The Data Box Edge/Gateway device name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The Data Box Edge/Gateway device culture."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub culture: Option<String>,
    #[doc = "The Data Box Edge/Gateway device model."]
    #[serde(rename = "deviceModel", default, skip_serializing_if = "Option::is_none")]
    pub device_model: Option<String>,
    #[doc = "The Data Box Edge/Gateway device software version."]
    #[serde(rename = "deviceSoftwareVersion", default, skip_serializing_if = "Option::is_none")]
    pub device_software_version: Option<String>,
    #[doc = "The Data Box Edge/Gateway device local capacity in MB."]
    #[serde(rename = "deviceLocalCapacity", default, skip_serializing_if = "Option::is_none")]
    pub device_local_capacity: Option<i64>,
    #[doc = "The Data Box Edge/Gateway device timezone."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "The device software version number of the device (eg: 1.2.18105.6)."]
    #[serde(rename = "deviceHcsVersion", default, skip_serializing_if = "Option::is_none")]
    pub device_hcs_version: Option<String>,
    #[doc = "Type of compute roles configured."]
    #[serde(rename = "configuredRoleTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub configured_role_types: Vec<String>,
    #[doc = "The number of nodes in the cluster."]
    #[serde(rename = "nodeCount", default, skip_serializing_if = "Option::is_none")]
    pub node_count: Option<i32>,
    #[doc = "Fields for tracking resource move"]
    #[serde(rename = "resourceMoveDetails", default, skip_serializing_if = "Option::is_none")]
    pub resource_move_details: Option<ResourceMoveDetails>,
    #[doc = "Details about Edge Profile for the resource"]
    #[serde(rename = "edgeProfile", default, skip_serializing_if = "Option::is_none")]
    pub edge_profile: Option<EdgeProfile>,
    #[doc = "Wraps data-residency related information for edge-resource and this should be used with ARM layer."]
    #[serde(rename = "dataResidency", default, skip_serializing_if = "Option::is_none")]
    pub data_residency: Option<DataResidency>,
}
impl DataBoxEdgeDeviceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_box_edge_device_properties {
    use super::*;
    #[doc = "The status of the Data Box Edge/Gateway device."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataBoxEdgeDeviceStatus")]
    pub enum DataBoxEdgeDeviceStatus {
        ReadyToSetup,
        Online,
        Offline,
        NeedsAttention,
        Disconnected,
        PartiallyDisconnected,
        Maintenance,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataBoxEdgeDeviceStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataBoxEdgeDeviceStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataBoxEdgeDeviceStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ReadyToSetup => serializer.serialize_unit_variant("DataBoxEdgeDeviceStatus", 0u32, "ReadyToSetup"),
                Self::Online => serializer.serialize_unit_variant("DataBoxEdgeDeviceStatus", 1u32, "Online"),
                Self::Offline => serializer.serialize_unit_variant("DataBoxEdgeDeviceStatus", 2u32, "Offline"),
                Self::NeedsAttention => serializer.serialize_unit_variant("DataBoxEdgeDeviceStatus", 3u32, "NeedsAttention"),
                Self::Disconnected => serializer.serialize_unit_variant("DataBoxEdgeDeviceStatus", 4u32, "Disconnected"),
                Self::PartiallyDisconnected => serializer.serialize_unit_variant("DataBoxEdgeDeviceStatus", 5u32, "PartiallyDisconnected"),
                Self::Maintenance => serializer.serialize_unit_variant("DataBoxEdgeDeviceStatus", 6u32, "Maintenance"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of the Data Box Edge/Gateway device."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeviceType")]
    pub enum DeviceType {
        DataBoxEdgeDevice,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeviceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeviceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeviceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::DataBoxEdgeDevice => serializer.serialize_unit_variant("DeviceType", 0u32, "DataBoxEdgeDevice"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Data Box Edge/Gateway device properties patch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBoxEdgeDevicePropertiesPatch {
    #[doc = "The Data Box Edge/Gateway Edge Profile patch."]
    #[serde(rename = "edgeProfile", default, skip_serializing_if = "Option::is_none")]
    pub edge_profile: Option<EdgeProfilePatch>,
}
impl DataBoxEdgeDevicePropertiesPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource Move details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxEdgeMoveRequest {
    #[doc = "Target resource group ARMId"]
    #[serde(rename = "targetResourceGroup")]
    pub target_resource_group: String,
    #[doc = "List of resources to be moved"]
    pub resources: Vec<String>,
}
impl DataBoxEdgeMoveRequest {
    pub fn new(target_resource_group: String, resources: Vec<String>) -> Self {
        Self {
            target_resource_group,
            resources,
        }
    }
}
#[doc = "The Sku information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBoxEdgeSku {
    #[doc = "The type of the resource."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The Sku name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<data_box_edge_sku::Name>,
    #[doc = "The Sku kind."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "The Sku tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<data_box_edge_sku::Tier>,
    #[doc = "The Sku kind."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "The Sku family."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "Availability of the Sku for the region."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "The API versions in which Sku is available."]
    #[serde(rename = "apiVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,
    #[doc = "Availability of the Sku for the location/zone/site."]
    #[serde(rename = "locationInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub location_info: Vec<SkuLocationInfo>,
    #[doc = "The pricing info of the Sku."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub costs: Vec<SkuCost>,
    #[doc = "Sku can be signed up by customer or not."]
    #[serde(rename = "signupOption", default, skip_serializing_if = "Option::is_none")]
    pub signup_option: Option<data_box_edge_sku::SignupOption>,
    #[doc = "Availability of the Sku as preview/stable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<data_box_edge_sku::Version>,
    #[doc = "Links to the next set of results"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability: Option<data_box_edge_sku::Availability>,
    #[doc = "List of Shipment Types supported by this SKU"]
    #[serde(rename = "shipmentTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub shipment_types: Vec<String>,
    #[doc = "The capability info of the SKU."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<SkuCapability>,
}
impl DataBoxEdgeSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_box_edge_sku {
    use super::*;
    #[doc = "The Sku name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Gateway,
        Edge,
        #[serde(rename = "TEA_1Node")]
        Tea1node,
        #[serde(rename = "TEA_1Node_UPS")]
        Tea1nodeUps,
        #[serde(rename = "TEA_1Node_Heater")]
        Tea1nodeHeater,
        #[serde(rename = "TEA_1Node_UPS_Heater")]
        Tea1nodeUpsHeater,
        #[serde(rename = "TEA_4Node_Heater")]
        Tea4nodeHeater,
        #[serde(rename = "TEA_4Node_UPS_Heater")]
        Tea4nodeUpsHeater,
        #[serde(rename = "TMA")]
        Tma,
        #[serde(rename = "TDC")]
        Tdc,
        #[serde(rename = "TCA_Small")]
        TcaSmall,
        #[serde(rename = "GPU")]
        Gpu,
        #[serde(rename = "TCA_Large")]
        TcaLarge,
        #[serde(rename = "EdgeP_Base")]
        EdgePBase,
        #[serde(rename = "EdgeP_High")]
        EdgePHigh,
        #[serde(rename = "EdgePR_Base")]
        EdgePrBase,
        #[serde(rename = "EdgePR_Base_UPS")]
        EdgePrBaseUps,
        #[serde(rename = "EP2_64_1VPU_W")]
        Ep2641vpuW,
        #[serde(rename = "EP2_128_1T4_Mx1_W")]
        Ep21281t4Mx1W,
        #[serde(rename = "EP2_256_2T4_W")]
        Ep22562t4W,
        #[serde(rename = "EdgeMR_Mini")]
        EdgeMrMini,
        #[serde(rename = "RCA_Small")]
        RcaSmall,
        #[serde(rename = "RCA_Large")]
        RcaLarge,
        #[serde(rename = "RDC")]
        Rdc,
        Management,
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
                Self::Gateway => serializer.serialize_unit_variant("Name", 0u32, "Gateway"),
                Self::Edge => serializer.serialize_unit_variant("Name", 1u32, "Edge"),
                Self::Tea1node => serializer.serialize_unit_variant("Name", 2u32, "TEA_1Node"),
                Self::Tea1nodeUps => serializer.serialize_unit_variant("Name", 3u32, "TEA_1Node_UPS"),
                Self::Tea1nodeHeater => serializer.serialize_unit_variant("Name", 4u32, "TEA_1Node_Heater"),
                Self::Tea1nodeUpsHeater => serializer.serialize_unit_variant("Name", 5u32, "TEA_1Node_UPS_Heater"),
                Self::Tea4nodeHeater => serializer.serialize_unit_variant("Name", 6u32, "TEA_4Node_Heater"),
                Self::Tea4nodeUpsHeater => serializer.serialize_unit_variant("Name", 7u32, "TEA_4Node_UPS_Heater"),
                Self::Tma => serializer.serialize_unit_variant("Name", 8u32, "TMA"),
                Self::Tdc => serializer.serialize_unit_variant("Name", 9u32, "TDC"),
                Self::TcaSmall => serializer.serialize_unit_variant("Name", 10u32, "TCA_Small"),
                Self::Gpu => serializer.serialize_unit_variant("Name", 11u32, "GPU"),
                Self::TcaLarge => serializer.serialize_unit_variant("Name", 12u32, "TCA_Large"),
                Self::EdgePBase => serializer.serialize_unit_variant("Name", 13u32, "EdgeP_Base"),
                Self::EdgePHigh => serializer.serialize_unit_variant("Name", 14u32, "EdgeP_High"),
                Self::EdgePrBase => serializer.serialize_unit_variant("Name", 15u32, "EdgePR_Base"),
                Self::EdgePrBaseUps => serializer.serialize_unit_variant("Name", 16u32, "EdgePR_Base_UPS"),
                Self::Ep2641vpuW => serializer.serialize_unit_variant("Name", 17u32, "EP2_64_1VPU_W"),
                Self::Ep21281t4Mx1W => serializer.serialize_unit_variant("Name", 18u32, "EP2_128_1T4_Mx1_W"),
                Self::Ep22562t4W => serializer.serialize_unit_variant("Name", 19u32, "EP2_256_2T4_W"),
                Self::EdgeMrMini => serializer.serialize_unit_variant("Name", 20u32, "EdgeMR_Mini"),
                Self::RcaSmall => serializer.serialize_unit_variant("Name", 21u32, "RCA_Small"),
                Self::RcaLarge => serializer.serialize_unit_variant("Name", 22u32, "RCA_Large"),
                Self::Rdc => serializer.serialize_unit_variant("Name", 23u32, "RDC"),
                Self::Management => serializer.serialize_unit_variant("Name", 24u32, "Management"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The Sku tier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Standard,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("Tier", 0u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Sku can be signed up by customer or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SignupOption")]
    pub enum SignupOption {
        None,
        Available,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SignupOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SignupOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SignupOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("SignupOption", 0u32, "None"),
                Self::Available => serializer.serialize_unit_variant("SignupOption", 1u32, "Available"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Availability of the Sku as preview/stable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Version")]
    pub enum Version {
        Stable,
        Preview,
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
                Self::Stable => serializer.serialize_unit_variant("Version", 0u32, "Stable"),
                Self::Preview => serializer.serialize_unit_variant("Version", 1u32, "Preview"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Links to the next set of results"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Availability")]
    pub enum Availability {
        Available,
        Unavailable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Availability {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Availability {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Availability {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Available => serializer.serialize_unit_variant("Availability", 0u32, "Available"),
                Self::Unavailable => serializer.serialize_unit_variant("Availability", 1u32, "Unavailable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of SKU Information objects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBoxEdgeSkuList {
    #[doc = "List of ResourceType Sku"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataBoxEdgeSku>,
    #[doc = "Links to the next set of results"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataBoxEdgeSkuList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataBoxEdgeSkuList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Wraps data-residency related information for edge-resource and this should be used with ARM layer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataResidency {
    #[doc = "DataResidencyType enum"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<data_residency::Type>,
}
impl DataResidency {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_residency {
    use super::*;
    #[doc = "DataResidencyType enum"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        GeoZoneReplication,
        ZoneReplication,
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
                Self::GeoZoneReplication => serializer.serialize_unit_variant("Type", 0u32, "GeoZoneReplication"),
                Self::ZoneReplication => serializer.serialize_unit_variant("Type", 1u32, "ZoneReplication"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The diagnostic proactive log collection settings of a device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticProactiveLogCollectionSettings {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties of proactive log collection settings."]
    pub properties: ProactiveLogCollectionSettingsProperties,
}
impl DiagnosticProactiveLogCollectionSettings {
    pub fn new(properties: ProactiveLogCollectionSettingsProperties) -> Self {
        Self {
            arm_base_model: ArmBaseModel::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "The remote support settings of a device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticRemoteSupportSettings {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties of remote support settings."]
    pub properties: DiagnosticRemoteSupportSettingsProperties,
}
impl DiagnosticRemoteSupportSettings {
    pub fn new(properties: DiagnosticRemoteSupportSettingsProperties) -> Self {
        Self {
            arm_base_model: ArmBaseModel::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "The properties of remote support settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticRemoteSupportSettingsProperties {
    #[doc = "Remote support settings list according to the RemoteApplicationType"]
    #[serde(rename = "remoteSupportSettingsList", default, skip_serializing_if = "Vec::is_empty")]
    pub remote_support_settings_list: Vec<RemoteSupportSettings>,
}
impl DiagnosticRemoteSupportSettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about Edge Profile for the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EdgeProfile {
    #[doc = "Subscription details for the Edge Profile"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<EdgeProfileSubscription>,
}
impl EdgeProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Data Box Edge/Gateway Edge Profile patch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EdgeProfilePatch {
    #[doc = "The Data Box Edge/Gateway Edge Profile Subscription patch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<EdgeProfileSubscriptionPatch>,
}
impl EdgeProfilePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subscription details for the Edge Profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EdgeProfileSubscription {
    #[doc = "Edge Subscription Registration ID"]
    #[serde(rename = "registrationId", default, skip_serializing_if = "Option::is_none")]
    pub registration_id: Option<String>,
    #[doc = "ARM ID of the subscription"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<edge_profile_subscription::State>,
    #[serde(rename = "registrationDate", default, skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<String>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionProperties>,
}
impl EdgeProfileSubscription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod edge_profile_subscription {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Registered,
        Warned,
        Suspended,
        Deleted,
        Unregistered,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Registered => serializer.serialize_unit_variant("State", 0u32, "Registered"),
                Self::Warned => serializer.serialize_unit_variant("State", 1u32, "Warned"),
                Self::Suspended => serializer.serialize_unit_variant("State", 2u32, "Suspended"),
                Self::Deleted => serializer.serialize_unit_variant("State", 3u32, "Deleted"),
                Self::Unregistered => serializer.serialize_unit_variant("State", 4u32, "Unregistered"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Data Box Edge/Gateway Edge Profile Subscription patch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EdgeProfileSubscriptionPatch {
    #[doc = "The path ID that uniquely identifies the subscription of the edge profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl EdgeProfileSubscriptionPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Etcd configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EtcdInfo {
    #[doc = "Etcd type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Etcd version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl EtcdInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Trigger details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileEventTrigger {
    #[serde(flatten)]
    pub trigger: Trigger,
    #[doc = "File trigger properties."]
    pub properties: FileTriggerProperties,
    #[doc = "The path ID that uniquely identifies the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The object name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The hierarchical type of the object."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl FileEventTrigger {
    pub fn new(trigger: Trigger, properties: FileTriggerProperties) -> Self {
        Self {
            trigger,
            properties,
            id: None,
            name: None,
            type_: None,
        }
    }
}
#[doc = "File source details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileSourceInfo {
    #[doc = "File share ID."]
    #[serde(rename = "shareId")]
    pub share_id: String,
}
impl FileSourceInfo {
    pub fn new(share_id: String) -> Self {
        Self { share_id }
    }
}
#[doc = "File trigger properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileTriggerProperties {
    #[doc = "File source details."]
    #[serde(rename = "sourceInfo")]
    pub source_info: FileSourceInfo,
    #[doc = "Compute role against which events will be raised."]
    #[serde(rename = "sinkInfo")]
    pub sink_info: RoleSinkInfo,
    #[doc = "A custom context tag typically used to correlate the trigger against its usage. For example, if a periodic timer trigger is intended for certain specific IoT modules in the device, the tag can be the name or the image URL of the module."]
    #[serde(rename = "customContextTag", default, skip_serializing_if = "Option::is_none")]
    pub custom_context_tag: Option<String>,
}
impl FileTriggerProperties {
    pub fn new(source_info: FileSourceInfo, sink_info: RoleSinkInfo) -> Self {
        Self {
            source_info,
            sink_info,
            custom_context_tag: None,
        }
    }
}
#[doc = "Used in activation key generation flow."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateCertResponse {
    #[doc = "Gets or sets base64 encoded certificate raw data,\r\nthis is the public part needed to be uploaded to cert vault"]
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[doc = "Gets or sets base64 encoded private part of the certificate,\r\nneeded to form the activation key"]
    #[serde(rename = "privateKey", default, skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[doc = "Gets or sets expiry time in UTC"]
    #[serde(rename = "expiryTimeInUTC", default, skip_serializing_if = "Option::is_none")]
    pub expiry_time_in_utc: Option<String>,
}
impl GenerateCertResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image repository credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageRepositoryCredential {
    #[doc = "Image repository url (e.g.: mcr.microsoft.com)."]
    #[serde(rename = "imageRepositoryUrl")]
    pub image_repository_url: String,
    #[doc = "Repository user name."]
    #[serde(rename = "userName")]
    pub user_name: String,
    #[doc = "Represent the secrets intended for encryption with asymmetric key pair."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<AsymmetricEncryptedSecret>,
}
impl ImageRepositoryCredential {
    pub fn new(image_repository_url: String, user_name: String) -> Self {
        Self {
            image_repository_url,
            user_name,
            password: None,
        }
    }
}
#[doc = "IoT Addon."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTAddon {
    #[serde(flatten)]
    pub addon: Addon,
    #[doc = "IoT addon properties."]
    pub properties: IoTAddonProperties,
    #[doc = "The path ID that uniquely identifies the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The object name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The hierarchical type of the object."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl IoTAddon {
    pub fn new(addon: Addon, properties: IoTAddonProperties) -> Self {
        Self {
            addon,
            properties,
            id: None,
            name: None,
            type_: None,
        }
    }
}
#[doc = "IoT addon properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTAddonProperties {
    #[doc = "Metadata of IoT device/IoT Edge device to be configured."]
    #[serde(rename = "ioTDeviceDetails")]
    pub io_t_device_details: IoTDeviceInfo,
    #[doc = "Metadata of IoT device/IoT Edge device to be configured."]
    #[serde(rename = "ioTEdgeDeviceDetails")]
    pub io_t_edge_device_details: IoTDeviceInfo,
    #[doc = "Version of IoT running on the appliance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Host OS supported by the IoT addon."]
    #[serde(rename = "hostPlatform", default, skip_serializing_if = "Option::is_none")]
    pub host_platform: Option<io_t_addon_properties::HostPlatform>,
    #[doc = "Platform where the runtime is hosted."]
    #[serde(rename = "hostPlatformType", default, skip_serializing_if = "Option::is_none")]
    pub host_platform_type: Option<io_t_addon_properties::HostPlatformType>,
    #[doc = "Addon Provisioning State"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<io_t_addon_properties::ProvisioningState>,
}
impl IoTAddonProperties {
    pub fn new(io_t_device_details: IoTDeviceInfo, io_t_edge_device_details: IoTDeviceInfo) -> Self {
        Self {
            io_t_device_details,
            io_t_edge_device_details,
            version: None,
            host_platform: None,
            host_platform_type: None,
            provisioning_state: None,
        }
    }
}
pub mod io_t_addon_properties {
    use super::*;
    #[doc = "Host OS supported by the IoT addon."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HostPlatform")]
    pub enum HostPlatform {
        Windows,
        Linux,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HostPlatform {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HostPlatform {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HostPlatform {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Windows => serializer.serialize_unit_variant("HostPlatform", 0u32, "Windows"),
                Self::Linux => serializer.serialize_unit_variant("HostPlatform", 1u32, "Linux"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Platform where the runtime is hosted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HostPlatformType")]
    pub enum HostPlatformType {
        KubernetesCluster,
        #[serde(rename = "LinuxVM")]
        LinuxVm,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HostPlatformType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HostPlatformType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HostPlatformType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::KubernetesCluster => serializer.serialize_unit_variant("HostPlatformType", 0u32, "KubernetesCluster"),
                Self::LinuxVm => serializer.serialize_unit_variant("HostPlatformType", 1u32, "LinuxVM"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Addon Provisioning State"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Invalid,
        Creating,
        Created,
        Updating,
        Reconfiguring,
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
                Self::Invalid => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Invalid"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Created => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Created"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Updating"),
                Self::Reconfiguring => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Reconfiguring"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Metadata of IoT device/IoT Edge device to be configured."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTDeviceInfo {
    #[doc = "ID of the IoT device/edge device."]
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[doc = "Host name for the IoT hub associated to the device."]
    #[serde(rename = "ioTHostHub")]
    pub io_t_host_hub: String,
    #[doc = "Id for the IoT hub associated to the device."]
    #[serde(rename = "ioTHostHubId", default, skip_serializing_if = "Option::is_none")]
    pub io_t_host_hub_id: Option<String>,
    #[doc = "Authentication mechanism for IoT devices."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<Authentication>,
}
impl IoTDeviceInfo {
    pub fn new(device_id: String, io_t_host_hub: String) -> Self {
        Self {
            device_id,
            io_t_host_hub,
            io_t_host_hub_id: None,
            authentication: None,
        }
    }
}
#[doc = "IoT edge agent details is optional, this will be used for download system Agent module while bootstrapping IoT Role if specified."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTEdgeAgentInfo {
    #[doc = "Name of the IoT edge agent image."]
    #[serde(rename = "imageName")]
    pub image_name: String,
    #[doc = "Image Tag."]
    pub tag: String,
    #[doc = "Image repository credential."]
    #[serde(rename = "imageRepository", default, skip_serializing_if = "Option::is_none")]
    pub image_repository: Option<ImageRepositoryCredential>,
}
impl IoTEdgeAgentInfo {
    pub fn new(image_name: String, tag: String) -> Self {
        Self {
            image_name,
            tag,
            image_repository: None,
        }
    }
}
#[doc = "Compute role."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTRole {
    #[serde(flatten)]
    pub role: Role,
    #[doc = "IoT role properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTRoleProperties>,
    #[doc = "The path ID that uniquely identifies the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The object name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The hierarchical type of the object."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl IoTRole {
    pub fn new(role: Role) -> Self {
        Self {
            role,
            properties: None,
            id: None,
            name: None,
            type_: None,
        }
    }
}
#[doc = "IoT role properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTRoleProperties {
    #[doc = "Host OS supported by the IoT role."]
    #[serde(rename = "hostPlatform")]
    pub host_platform: io_t_role_properties::HostPlatform,
    #[doc = "Metadata of IoT device/IoT Edge device to be configured."]
    #[serde(rename = "ioTDeviceDetails")]
    pub io_t_device_details: IoTDeviceInfo,
    #[doc = "Metadata of IoT device/IoT Edge device to be configured."]
    #[serde(rename = "ioTEdgeDeviceDetails")]
    pub io_t_edge_device_details: IoTDeviceInfo,
    #[doc = "Mount points of shares in role(s)."]
    #[serde(rename = "shareMappings", default, skip_serializing_if = "Vec::is_empty")]
    pub share_mappings: Vec<MountPointMap>,
    #[doc = "IoT edge agent details is optional, this will be used for download system Agent module while bootstrapping IoT Role if specified."]
    #[serde(rename = "ioTEdgeAgentInfo", default, skip_serializing_if = "Option::is_none")]
    pub io_t_edge_agent_info: Option<IoTEdgeAgentInfo>,
    #[doc = "Platform where the Iot runtime is hosted."]
    #[serde(rename = "hostPlatformType", default, skip_serializing_if = "Option::is_none")]
    pub host_platform_type: Option<io_t_role_properties::HostPlatformType>,
    #[doc = "Compute infrastructure Resource"]
    #[serde(rename = "computeResource", default, skip_serializing_if = "Option::is_none")]
    pub compute_resource: Option<ComputeResource>,
    #[doc = "Role status."]
    #[serde(rename = "roleStatus")]
    pub role_status: io_t_role_properties::RoleStatus,
}
impl IoTRoleProperties {
    pub fn new(
        host_platform: io_t_role_properties::HostPlatform,
        io_t_device_details: IoTDeviceInfo,
        io_t_edge_device_details: IoTDeviceInfo,
        role_status: io_t_role_properties::RoleStatus,
    ) -> Self {
        Self {
            host_platform,
            io_t_device_details,
            io_t_edge_device_details,
            share_mappings: Vec::new(),
            io_t_edge_agent_info: None,
            host_platform_type: None,
            compute_resource: None,
            role_status,
        }
    }
}
pub mod io_t_role_properties {
    use super::*;
    #[doc = "Host OS supported by the IoT role."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HostPlatform")]
    pub enum HostPlatform {
        Windows,
        Linux,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HostPlatform {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HostPlatform {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HostPlatform {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Windows => serializer.serialize_unit_variant("HostPlatform", 0u32, "Windows"),
                Self::Linux => serializer.serialize_unit_variant("HostPlatform", 1u32, "Linux"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Platform where the Iot runtime is hosted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HostPlatformType")]
    pub enum HostPlatformType {
        KubernetesCluster,
        #[serde(rename = "LinuxVM")]
        LinuxVm,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HostPlatformType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HostPlatformType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HostPlatformType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::KubernetesCluster => serializer.serialize_unit_variant("HostPlatformType", 0u32, "KubernetesCluster"),
                Self::LinuxVm => serializer.serialize_unit_variant("HostPlatformType", 1u32, "LinuxVM"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Role status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RoleStatus")]
    pub enum RoleStatus {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RoleStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RoleStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RoleStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("RoleStatus", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("RoleStatus", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Details related to the IPv4 address configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Ipv4Config {
    #[doc = "The IPv4 address of the network adapter."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The IPv4 subnet of the network adapter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    #[doc = "The IPv4 gateway of the network adapter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
}
impl Ipv4Config {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details related to the IPv6 address configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Ipv6Config {
    #[doc = "The IPv6 address of the network adapter."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The IPv6 prefix of the network adapter."]
    #[serde(rename = "prefixLength", default, skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<i32>,
    #[doc = "The IPv6 gateway of the network adapter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
}
impl Ipv6Config {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A device job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Job {
    #[doc = "The path ID that uniquely identifies the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The hierarchical type of the object."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The current status of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<job::Status>,
    #[doc = "The UTC date and time at which the job started."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The UTC date and time at which the job completed."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The percentage of the job that is complete."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[doc = "The job error information containing the list of job errors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<JobErrorDetails>,
    #[doc = "The properties for the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobProperties>,
}
impl Job {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job {
    use super::*;
    #[doc = "The current status of the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Invalid,
        Running,
        Succeeded,
        Failed,
        Canceled,
        Paused,
        Scheduled,
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
                Self::Invalid => serializer.serialize_unit_variant("Status", 0u32, "Invalid"),
                Self::Running => serializer.serialize_unit_variant("Status", 1u32, "Running"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 3u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 4u32, "Canceled"),
                Self::Paused => serializer.serialize_unit_variant("Status", 5u32, "Paused"),
                Self::Scheduled => serializer.serialize_unit_variant("Status", 6u32, "Scheduled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The job error information containing the list of job errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobErrorDetails {
    #[doc = "The error details."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<JobErrorItem>,
    #[doc = "The code intended for programmatic access."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The message that describes the error in detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl JobErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The job error items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobErrorItem {
    #[doc = "The recommended actions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommendations: Vec<String>,
    #[doc = "The code intended for programmatic access."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The message that describes the error in detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl JobErrorItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties for the job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobProperties {
    #[doc = "The type of the job."]
    #[serde(rename = "jobType", default, skip_serializing_if = "Option::is_none")]
    pub job_type: Option<job_properties::JobType>,
    #[doc = "Current stage of the update operation."]
    #[serde(rename = "currentStage", default, skip_serializing_if = "Option::is_none")]
    pub current_stage: Option<job_properties::CurrentStage>,
    #[doc = "Details about the download progress of update."]
    #[serde(rename = "downloadProgress", default, skip_serializing_if = "Option::is_none")]
    pub download_progress: Option<UpdateDownloadProgress>,
    #[doc = "Progress details during installation of updates."]
    #[serde(rename = "installProgress", default, skip_serializing_if = "Option::is_none")]
    pub install_progress: Option<UpdateInstallProgress>,
    #[doc = "Total number of errors encountered during the refresh process."]
    #[serde(rename = "totalRefreshErrors", default, skip_serializing_if = "Option::is_none")]
    pub total_refresh_errors: Option<i32>,
    #[doc = "Local share/remote container relative path to the error manifest file of the refresh."]
    #[serde(rename = "errorManifestFile", default, skip_serializing_if = "Option::is_none")]
    pub error_manifest_file: Option<String>,
    #[doc = "ARM ID of the entity that was refreshed."]
    #[serde(rename = "refreshedEntityId", default, skip_serializing_if = "Option::is_none")]
    pub refreshed_entity_id: Option<String>,
    #[doc = "If only subfolders need to be refreshed, then the subfolder path inside the share or container. (The path is empty if there are no subfolders.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
}
impl JobProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_properties {
    use super::*;
    #[doc = "The type of the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "JobType")]
    pub enum JobType {
        Invalid,
        ScanForUpdates,
        DownloadUpdates,
        InstallUpdates,
        RefreshShare,
        RefreshContainer,
        Backup,
        Restore,
        TriggerSupportPackage,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for JobType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for JobType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for JobType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("JobType", 0u32, "Invalid"),
                Self::ScanForUpdates => serializer.serialize_unit_variant("JobType", 1u32, "ScanForUpdates"),
                Self::DownloadUpdates => serializer.serialize_unit_variant("JobType", 2u32, "DownloadUpdates"),
                Self::InstallUpdates => serializer.serialize_unit_variant("JobType", 3u32, "InstallUpdates"),
                Self::RefreshShare => serializer.serialize_unit_variant("JobType", 4u32, "RefreshShare"),
                Self::RefreshContainer => serializer.serialize_unit_variant("JobType", 5u32, "RefreshContainer"),
                Self::Backup => serializer.serialize_unit_variant("JobType", 6u32, "Backup"),
                Self::Restore => serializer.serialize_unit_variant("JobType", 7u32, "Restore"),
                Self::TriggerSupportPackage => serializer.serialize_unit_variant("JobType", 8u32, "TriggerSupportPackage"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Current stage of the update operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CurrentStage")]
    pub enum CurrentStage {
        Unknown,
        Initial,
        ScanStarted,
        ScanComplete,
        ScanFailed,
        DownloadStarted,
        DownloadComplete,
        DownloadFailed,
        InstallStarted,
        InstallComplete,
        InstallFailed,
        RebootInitiated,
        Success,
        Failure,
        RescanStarted,
        RescanComplete,
        RescanFailed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CurrentStage {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CurrentStage {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CurrentStage {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("CurrentStage", 0u32, "Unknown"),
                Self::Initial => serializer.serialize_unit_variant("CurrentStage", 1u32, "Initial"),
                Self::ScanStarted => serializer.serialize_unit_variant("CurrentStage", 2u32, "ScanStarted"),
                Self::ScanComplete => serializer.serialize_unit_variant("CurrentStage", 3u32, "ScanComplete"),
                Self::ScanFailed => serializer.serialize_unit_variant("CurrentStage", 4u32, "ScanFailed"),
                Self::DownloadStarted => serializer.serialize_unit_variant("CurrentStage", 5u32, "DownloadStarted"),
                Self::DownloadComplete => serializer.serialize_unit_variant("CurrentStage", 6u32, "DownloadComplete"),
                Self::DownloadFailed => serializer.serialize_unit_variant("CurrentStage", 7u32, "DownloadFailed"),
                Self::InstallStarted => serializer.serialize_unit_variant("CurrentStage", 8u32, "InstallStarted"),
                Self::InstallComplete => serializer.serialize_unit_variant("CurrentStage", 9u32, "InstallComplete"),
                Self::InstallFailed => serializer.serialize_unit_variant("CurrentStage", 10u32, "InstallFailed"),
                Self::RebootInitiated => serializer.serialize_unit_variant("CurrentStage", 11u32, "RebootInitiated"),
                Self::Success => serializer.serialize_unit_variant("CurrentStage", 12u32, "Success"),
                Self::Failure => serializer.serialize_unit_variant("CurrentStage", 13u32, "Failure"),
                Self::RescanStarted => serializer.serialize_unit_variant("CurrentStage", 14u32, "RescanStarted"),
                Self::RescanComplete => serializer.serialize_unit_variant("CurrentStage", 15u32, "RescanComplete"),
                Self::RescanFailed => serializer.serialize_unit_variant("CurrentStage", 16u32, "RescanFailed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Kubernetes cluster configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubernetesClusterInfo {
    #[doc = "Etcd configuration"]
    #[serde(rename = "etcdInfo", default, skip_serializing_if = "Option::is_none")]
    pub etcd_info: Option<EtcdInfo>,
    #[doc = "Kubernetes cluster nodes"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<NodeInfo>,
    #[doc = "Kubernetes cluster version"]
    pub version: String,
}
impl KubernetesClusterInfo {
    pub fn new(version: String) -> Self {
        Self {
            etcd_info: None,
            nodes: Vec::new(),
            version,
        }
    }
}
#[doc = "Kubernetes node IP configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesIpConfiguration {
    #[doc = "Port of the Kubernetes node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[doc = "IP address of the Kubernetes node."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}
impl KubernetesIpConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The limited preview of Kubernetes Cluster Management from the Azure supports:\r\n1. Using a simple turn-key option in Azure Portal, deploy a Kubernetes cluster on your Azure Stack Edge device. \r\n2. Configure Kubernetes cluster running on your device with Arc enabled Kubernetes with a click of a button in the Azure Portal. \r\n Azure Arc enables organizations to view, manage, and govern their on-premises Kubernetes clusters using the Azure Portal, command line tools, and APIs.\r\n3. Easily configure Persistent Volumes using SMB and NFS shares for storing container data. \r\n For more information, refer to the document here: https://databoxupdatepackages.blob.core.windows.net/documentation/Microsoft-Azure-Stack-Edge-K8-Cloud-Management-20210323.pdf \r\n Or Demo: https://databoxupdatepackages.blob.core.windows.net/documentation/Microsoft-Azure-Stack-Edge-K8S-Cloud-Management-20210323.mp4\r\n By using this feature, you agree to the preview legal terms. See the https://azure.microsoft.com/en-us/support/legal/preview-supplemental-terms/"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubernetesRole {
    #[serde(flatten)]
    pub role: Role,
    #[doc = "Kubernetes role properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<KubernetesRoleProperties>,
    #[doc = "The path ID that uniquely identifies the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The object name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The hierarchical type of the object."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl KubernetesRole {
    pub fn new(role: Role) -> Self {
        Self {
            role,
            properties: None,
            id: None,
            name: None,
            type_: None,
        }
    }
}
#[doc = "Kubernetes role compute resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubernetesRoleCompute {
    #[doc = "VM profile"]
    #[serde(rename = "vmProfile")]
    pub vm_profile: String,
    #[doc = "Memory in bytes"]
    #[serde(rename = "memoryInBytes", default, skip_serializing_if = "Option::is_none")]
    pub memory_in_bytes: Option<i64>,
    #[doc = "Processor count"]
    #[serde(rename = "processorCount", default, skip_serializing_if = "Option::is_none")]
    pub processor_count: Option<i32>,
}
impl KubernetesRoleCompute {
    pub fn new(vm_profile: String) -> Self {
        Self {
            vm_profile,
            memory_in_bytes: None,
            processor_count: None,
        }
    }
}
#[doc = "Kubernetes role network resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesRoleNetwork {
    #[doc = "Cni configuration"]
    #[serde(rename = "cniConfig", default, skip_serializing_if = "Option::is_none")]
    pub cni_config: Option<CniConfig>,
    #[doc = "Load balancer configuration"]
    #[serde(rename = "loadBalancerConfig", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_config: Option<LoadBalancerConfig>,
}
impl KubernetesRoleNetwork {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kubernetes role properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubernetesRoleProperties {
    #[doc = "Host OS supported by the Kubernetes role."]
    #[serde(rename = "hostPlatform")]
    pub host_platform: kubernetes_role_properties::HostPlatform,
    #[doc = "State of Kubernetes deployment"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<kubernetes_role_properties::ProvisioningState>,
    #[doc = "Platform where the runtime is hosted."]
    #[serde(rename = "hostPlatformType", default, skip_serializing_if = "Option::is_none")]
    pub host_platform_type: Option<kubernetes_role_properties::HostPlatformType>,
    #[doc = "Kubernetes cluster configuration"]
    #[serde(rename = "kubernetesClusterInfo")]
    pub kubernetes_cluster_info: KubernetesClusterInfo,
    #[doc = "Kubernetes role resources"]
    #[serde(rename = "kubernetesRoleResources")]
    pub kubernetes_role_resources: KubernetesRoleResources,
    #[doc = "Role status."]
    #[serde(rename = "roleStatus")]
    pub role_status: kubernetes_role_properties::RoleStatus,
}
impl KubernetesRoleProperties {
    pub fn new(
        host_platform: kubernetes_role_properties::HostPlatform,
        kubernetes_cluster_info: KubernetesClusterInfo,
        kubernetes_role_resources: KubernetesRoleResources,
        role_status: kubernetes_role_properties::RoleStatus,
    ) -> Self {
        Self {
            host_platform,
            provisioning_state: None,
            host_platform_type: None,
            kubernetes_cluster_info,
            kubernetes_role_resources,
            role_status,
        }
    }
}
pub mod kubernetes_role_properties {
    use super::*;
    #[doc = "Host OS supported by the Kubernetes role."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HostPlatform")]
    pub enum HostPlatform {
        Windows,
        Linux,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HostPlatform {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HostPlatform {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HostPlatform {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Windows => serializer.serialize_unit_variant("HostPlatform", 0u32, "Windows"),
                Self::Linux => serializer.serialize_unit_variant("HostPlatform", 1u32, "Linux"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "State of Kubernetes deployment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Invalid,
        Creating,
        Created,
        Updating,
        Reconfiguring,
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
                Self::Invalid => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Invalid"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Created => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Created"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Updating"),
                Self::Reconfiguring => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Reconfiguring"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Platform where the runtime is hosted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HostPlatformType")]
    pub enum HostPlatformType {
        KubernetesCluster,
        #[serde(rename = "LinuxVM")]
        LinuxVm,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HostPlatformType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HostPlatformType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HostPlatformType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::KubernetesCluster => serializer.serialize_unit_variant("HostPlatformType", 0u32, "KubernetesCluster"),
                Self::LinuxVm => serializer.serialize_unit_variant("HostPlatformType", 1u32, "LinuxVM"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Role status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RoleStatus")]
    pub enum RoleStatus {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RoleStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RoleStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RoleStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("RoleStatus", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("RoleStatus", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Kubernetes role resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubernetesRoleResources {
    #[doc = "Kubernetes role storage resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<KubernetesRoleStorage>,
    #[doc = "Kubernetes role compute resource"]
    pub compute: KubernetesRoleCompute,
    #[doc = "Kubernetes role network resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<KubernetesRoleNetwork>,
}
impl KubernetesRoleResources {
    pub fn new(compute: KubernetesRoleCompute) -> Self {
        Self {
            storage: None,
            compute,
            network: None,
        }
    }
}
#[doc = "Kubernetes role storage resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesRoleStorage {
    #[doc = "Kubernetes storage class info."]
    #[serde(rename = "storageClasses", default, skip_serializing_if = "Vec::is_empty")]
    pub storage_classes: Vec<KubernetesRoleStorageClassInfo>,
    #[doc = "Mount points of shares in role(s)."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<MountPointMap>,
}
impl KubernetesRoleStorage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kubernetes storage class info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesRoleStorageClassInfo {
    #[doc = "Storage class name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Storage class type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "If provisioned storage is posix compliant."]
    #[serde(rename = "posixCompliant", default, skip_serializing_if = "Option::is_none")]
    pub posix_compliant: Option<kubernetes_role_storage_class_info::PosixCompliant>,
}
impl KubernetesRoleStorageClassInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod kubernetes_role_storage_class_info {
    use super::*;
    #[doc = "If provisioned storage is posix compliant."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PosixCompliant")]
    pub enum PosixCompliant {
        Invalid,
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PosixCompliant {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PosixCompliant {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PosixCompliant {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("PosixCompliant", 0u32, "Invalid"),
                Self::Enabled => serializer.serialize_unit_variant("PosixCompliant", 1u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PosixCompliant", 2u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Load balancer configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancerConfig {
    #[doc = "Load balancer type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Load balancer version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl LoadBalancerConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MEC role."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MecRole {
    #[serde(flatten)]
    pub role: Role,
    #[doc = "MEC role properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MecRoleProperties>,
    #[doc = "The path ID that uniquely identifies the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The object name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The hierarchical type of the object."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl MecRole {
    pub fn new(role: Role) -> Self {
        Self {
            role,
            properties: None,
            id: None,
            name: None,
            type_: None,
        }
    }
}
#[doc = "MEC role properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MecRoleProperties {
    #[doc = "Represent the secrets intended for encryption with asymmetric key pair."]
    #[serde(rename = "connectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<AsymmetricEncryptedSecret>,
    #[doc = "Controller Endpoint."]
    #[serde(rename = "controllerEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub controller_endpoint: Option<String>,
    #[doc = "Unique Id of the Resource."]
    #[serde(rename = "resourceUniqueId", default, skip_serializing_if = "Option::is_none")]
    pub resource_unique_id: Option<String>,
    #[doc = "Role status."]
    #[serde(rename = "roleStatus")]
    pub role_status: mec_role_properties::RoleStatus,
}
impl MecRoleProperties {
    pub fn new(role_status: mec_role_properties::RoleStatus) -> Self {
        Self {
            connection_string: None,
            controller_endpoint: None,
            resource_unique_id: None,
            role_status,
        }
    }
}
pub mod mec_role_properties {
    use super::*;
    #[doc = "Role status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RoleStatus")]
    pub enum RoleStatus {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RoleStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RoleStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RoleStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("RoleStatus", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("RoleStatus", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Metric configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricConfiguration {
    #[doc = "The Resource ID on which the metrics should be pushed."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "The MDM account to which the counters should be pushed."]
    #[serde(rename = "mdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub mdm_account: Option<String>,
    #[doc = "The MDM namespace to which the counters should be pushed. This is required if MDMAccount is specified"]
    #[serde(rename = "metricNameSpace", default, skip_serializing_if = "Option::is_none")]
    pub metric_name_space: Option<String>,
    #[doc = "Host name for the IoT hub associated to the device."]
    #[serde(rename = "counterSets")]
    pub counter_sets: Vec<MetricCounterSet>,
}
impl MetricConfiguration {
    pub fn new(resource_id: String, counter_sets: Vec<MetricCounterSet>) -> Self {
        Self {
            resource_id,
            mdm_account: None,
            metric_name_space: None,
            counter_sets,
        }
    }
}
#[doc = "The metric counter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricCounter {
    #[doc = "The counter name."]
    pub name: String,
    #[doc = "The instance from which counter should be collected."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[doc = "The dimension filter."]
    #[serde(rename = "dimensionFilter", default, skip_serializing_if = "Vec::is_empty")]
    pub dimension_filter: Vec<MetricDimension>,
    #[doc = "The additional dimensions to be added to metric."]
    #[serde(rename = "additionalDimensions", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_dimensions: Vec<MetricDimension>,
}
impl MetricCounter {
    pub fn new(name: String) -> Self {
        Self {
            name,
            instance: None,
            dimension_filter: Vec::new(),
            additional_dimensions: Vec::new(),
        }
    }
}
#[doc = "The metric counter set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricCounterSet {
    #[doc = "The counters that should be collected in this set."]
    pub counters: Vec<MetricCounter>,
}
impl MetricCounterSet {
    pub fn new(counters: Vec<MetricCounter>) -> Self {
        Self { counters }
    }
}
#[doc = "The metric dimension"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDimension {
    #[doc = "The dimension type."]
    #[serde(rename = "sourceType")]
    pub source_type: String,
    #[doc = "The dimension value."]
    #[serde(rename = "sourceName")]
    pub source_name: String,
}
impl MetricDimension {
    pub fn new(source_type: String, source_name: String) -> Self {
        Self { source_type, source_name }
    }
}
#[doc = "Metric Dimension v1."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricDimensionV1 {
    #[doc = "Name of the metrics dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of the metrics dimension."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "To be exported to shoe box."]
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
impl MetricDimensionV1 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metric specification version 1."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecificationV1 {
    #[doc = "Name of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of the metric."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Description of the metric to be displayed."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "Metric units."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<metric_specification_v1::Unit>,
    #[doc = "Metric aggregation type."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<metric_specification_v1::AggregationType>,
    #[doc = "Metric dimensions, other than default dimension which is resource."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<MetricDimensionV1>,
    #[doc = "Set true to fill the gaps with zero."]
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[doc = "Metric category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<metric_specification_v1::Category>,
    #[doc = "Resource name override."]
    #[serde(rename = "resourceIdDimensionNameOverride", default, skip_serializing_if = "Option::is_none")]
    pub resource_id_dimension_name_override: Option<String>,
    #[doc = "Support granularity of metrics."]
    #[serde(rename = "supportedTimeGrainTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_time_grain_types: Vec<String>,
    #[doc = "Support metric aggregation type."]
    #[serde(rename = "supportedAggregationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<String>,
}
impl MetricSpecificationV1 {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod metric_specification_v1 {
    use super::*;
    #[doc = "Metric units."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        NotSpecified,
        Percent,
        Count,
        Seconds,
        Milliseconds,
        Bytes,
        BytesPerSecond,
        CountPerSecond,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("Unit", 0u32, "NotSpecified"),
                Self::Percent => serializer.serialize_unit_variant("Unit", 1u32, "Percent"),
                Self::Count => serializer.serialize_unit_variant("Unit", 2u32, "Count"),
                Self::Seconds => serializer.serialize_unit_variant("Unit", 3u32, "Seconds"),
                Self::Milliseconds => serializer.serialize_unit_variant("Unit", 4u32, "Milliseconds"),
                Self::Bytes => serializer.serialize_unit_variant("Unit", 5u32, "Bytes"),
                Self::BytesPerSecond => serializer.serialize_unit_variant("Unit", 6u32, "BytesPerSecond"),
                Self::CountPerSecond => serializer.serialize_unit_variant("Unit", 7u32, "CountPerSecond"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Metric aggregation type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AggregationType")]
    pub enum AggregationType {
        NotSpecified,
        None,
        Average,
        Minimum,
        Maximum,
        Total,
        Count,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AggregationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AggregationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AggregationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("AggregationType", 0u32, "NotSpecified"),
                Self::None => serializer.serialize_unit_variant("AggregationType", 1u32, "None"),
                Self::Average => serializer.serialize_unit_variant("AggregationType", 2u32, "Average"),
                Self::Minimum => serializer.serialize_unit_variant("AggregationType", 3u32, "Minimum"),
                Self::Maximum => serializer.serialize_unit_variant("AggregationType", 4u32, "Maximum"),
                Self::Total => serializer.serialize_unit_variant("AggregationType", 5u32, "Total"),
                Self::Count => serializer.serialize_unit_variant("AggregationType", 6u32, "Count"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Metric category."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Category")]
    pub enum Category {
        Capacity,
        Transaction,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Category {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Category {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Category {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Capacity => serializer.serialize_unit_variant("Category", 0u32, "Capacity"),
                Self::Transaction => serializer.serialize_unit_variant("Category", 1u32, "Transaction"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The metric setting details for the role"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitoringMetricConfiguration {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Metrics properties"]
    pub properties: MonitoringMetricConfigurationProperties,
}
impl MonitoringMetricConfiguration {
    pub fn new(properties: MonitoringMetricConfigurationProperties) -> Self {
        Self {
            arm_base_model: ArmBaseModel::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "Collection of metric configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringMetricConfigurationList {
    #[doc = "The list of metric configurations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MonitoringMetricConfiguration>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MonitoringMetricConfigurationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MonitoringMetricConfigurationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metrics properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitoringMetricConfigurationProperties {
    #[doc = "The metrics configuration details"]
    #[serde(rename = "metricConfigurations")]
    pub metric_configurations: Vec<MetricConfiguration>,
}
impl MonitoringMetricConfigurationProperties {
    pub fn new(metric_configurations: Vec<MetricConfiguration>) -> Self {
        Self { metric_configurations }
    }
}
#[doc = "The share mount point."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MountPointMap {
    #[doc = "ID of the share mounted to the role VM."]
    #[serde(rename = "shareId")]
    pub share_id: String,
    #[doc = "ID of the role to which share is mounted."]
    #[serde(rename = "roleId", default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    #[doc = "Mount point for the share."]
    #[serde(rename = "mountPoint", default, skip_serializing_if = "Option::is_none")]
    pub mount_point: Option<String>,
    #[doc = "Mounting type."]
    #[serde(rename = "mountType", default, skip_serializing_if = "Option::is_none")]
    pub mount_type: Option<mount_point_map::MountType>,
    #[doc = "Role type."]
    #[serde(rename = "roleType", default, skip_serializing_if = "Option::is_none")]
    pub role_type: Option<mount_point_map::RoleType>,
}
impl MountPointMap {
    pub fn new(share_id: String) -> Self {
        Self {
            share_id,
            role_id: None,
            mount_point: None,
            mount_type: None,
            role_type: None,
        }
    }
}
pub mod mount_point_map {
    use super::*;
    #[doc = "Mounting type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MountType")]
    pub enum MountType {
        Volume,
        HostPath,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MountType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MountType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MountType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Volume => serializer.serialize_unit_variant("MountType", 0u32, "Volume"),
                Self::HostPath => serializer.serialize_unit_variant("MountType", 1u32, "HostPath"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Role type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RoleType")]
    pub enum RoleType {
        #[serde(rename = "IOT")]
        Iot,
        #[serde(rename = "ASA")]
        Asa,
        Functions,
        Cognitive,
        #[serde(rename = "MEC")]
        Mec,
        CloudEdgeManagement,
        Kubernetes,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RoleType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RoleType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RoleType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Iot => serializer.serialize_unit_variant("RoleType", 0u32, "IOT"),
                Self::Asa => serializer.serialize_unit_variant("RoleType", 1u32, "ASA"),
                Self::Functions => serializer.serialize_unit_variant("RoleType", 2u32, "Functions"),
                Self::Cognitive => serializer.serialize_unit_variant("RoleType", 3u32, "Cognitive"),
                Self::Mec => serializer.serialize_unit_variant("RoleType", 4u32, "MEC"),
                Self::CloudEdgeManagement => serializer.serialize_unit_variant("RoleType", 5u32, "CloudEdgeManagement"),
                Self::Kubernetes => serializer.serialize_unit_variant("RoleType", 6u32, "Kubernetes"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents the networkAdapter on a device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkAdapter {
    #[doc = "Instance ID of network adapter."]
    #[serde(rename = "adapterId", default, skip_serializing_if = "Option::is_none")]
    pub adapter_id: Option<String>,
    #[doc = "The network adapter position."]
    #[serde(rename = "adapterPosition", default, skip_serializing_if = "Option::is_none")]
    pub adapter_position: Option<NetworkAdapterPosition>,
    #[doc = "Logical index of the adapter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[doc = "Node ID of the network adapter."]
    #[serde(rename = "nodeId", default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[doc = "Network adapter name."]
    #[serde(rename = "networkAdapterName", default, skip_serializing_if = "Option::is_none")]
    pub network_adapter_name: Option<String>,
    #[doc = "Hardware label for the adapter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "MAC address."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "Link speed."]
    #[serde(rename = "linkSpeed", default, skip_serializing_if = "Option::is_none")]
    pub link_speed: Option<i64>,
    #[doc = "Value indicating whether this adapter is valid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<network_adapter::Status>,
    #[doc = "Value indicating whether this adapter is RDMA capable."]
    #[serde(rename = "rdmaStatus", default, skip_serializing_if = "Option::is_none")]
    pub rdma_status: Option<network_adapter::RdmaStatus>,
    #[doc = "Value indicating whether this adapter has DHCP enabled."]
    #[serde(rename = "dhcpStatus", default, skip_serializing_if = "Option::is_none")]
    pub dhcp_status: Option<network_adapter::DhcpStatus>,
    #[doc = "Details related to the IPv4 address configuration."]
    #[serde(rename = "ipv4Configuration", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_configuration: Option<Ipv4Config>,
    #[doc = "Details related to the IPv6 address configuration."]
    #[serde(rename = "ipv6Configuration", default, skip_serializing_if = "Option::is_none")]
    pub ipv6_configuration: Option<Ipv6Config>,
    #[doc = "The IPv6 local address."]
    #[serde(rename = "ipv6LinkLocalAddress", default, skip_serializing_if = "Option::is_none")]
    pub ipv6_link_local_address: Option<String>,
    #[doc = "The list of DNS Servers of the device."]
    #[serde(rename = "dnsServers", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_servers: Vec<String>,
}
impl NetworkAdapter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_adapter {
    use super::*;
    #[doc = "Value indicating whether this adapter is valid."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Inactive,
        Active,
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
                Self::Inactive => serializer.serialize_unit_variant("Status", 0u32, "Inactive"),
                Self::Active => serializer.serialize_unit_variant("Status", 1u32, "Active"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Value indicating whether this adapter is RDMA capable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RdmaStatus")]
    pub enum RdmaStatus {
        Incapable,
        Capable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RdmaStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RdmaStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RdmaStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Incapable => serializer.serialize_unit_variant("RdmaStatus", 0u32, "Incapable"),
                Self::Capable => serializer.serialize_unit_variant("RdmaStatus", 1u32, "Capable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Value indicating whether this adapter has DHCP enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DhcpStatus")]
    pub enum DhcpStatus {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DhcpStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DhcpStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DhcpStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("DhcpStatus", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("DhcpStatus", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The network adapter position."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkAdapterPosition {
    #[doc = "The network group."]
    #[serde(rename = "networkGroup", default, skip_serializing_if = "Option::is_none")]
    pub network_group: Option<network_adapter_position::NetworkGroup>,
    #[doc = "The port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
impl NetworkAdapterPosition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_adapter_position {
    use super::*;
    #[doc = "The network group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NetworkGroup")]
    pub enum NetworkGroup {
        None,
        #[serde(rename = "NonRDMA")]
        NonRdma,
        #[serde(rename = "RDMA")]
        Rdma,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NetworkGroup {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NetworkGroup {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NetworkGroup {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("NetworkGroup", 0u32, "None"),
                Self::NonRdma => serializer.serialize_unit_variant("NetworkGroup", 1u32, "NonRDMA"),
                Self::Rdma => serializer.serialize_unit_variant("NetworkGroup", 2u32, "RDMA"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The network settings of a device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkSettings {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties of network settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkSettingsProperties>,
}
impl NetworkSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of network settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkSettingsProperties {
    #[doc = "The network adapter list on the device."]
    #[serde(rename = "networkAdapters", default, skip_serializing_if = "Vec::is_empty")]
    pub network_adapters: Vec<NetworkAdapter>,
}
impl NetworkSettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a single node in a Data box Edge/Gateway device\r\nGateway devices, standalone Edge devices and a single node cluster Edge device will all have 1 node\r\nMulti-node Edge devices will have more than 1 nodes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Node {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "This class represents the nodes in a highly available cluster"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NodeProperties>,
}
impl Node {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kubernetes node info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeInfo {
    #[doc = "Node name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Node type - Master/Worker"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<node_info::Type>,
    #[doc = "IP Configuration of the Kubernetes node."]
    #[serde(rename = "ipConfiguration", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_configuration: Vec<KubernetesIpConfiguration>,
}
impl NodeInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod node_info {
    use super::*;
    #[doc = "Node type - Master/Worker"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Invalid,
        Master,
        Worker,
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
                Self::Invalid => serializer.serialize_unit_variant("Type", 0u32, "Invalid"),
                Self::Master => serializer.serialize_unit_variant("Type", 1u32, "Master"),
                Self::Worker => serializer.serialize_unit_variant("Type", 2u32, "Worker"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Collection of Nodes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeList {
    #[doc = "The list of Nodes."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Node>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NodeList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NodeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This class represents the nodes in a highly available cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeProperties {
    #[doc = "The current status of the individual node"]
    #[serde(rename = "nodeStatus", default, skip_serializing_if = "Option::is_none")]
    pub node_status: Option<node_properties::NodeStatus>,
    #[doc = "Serial number of the Chassis"]
    #[serde(rename = "nodeChassisSerialNumber", default, skip_serializing_if = "Option::is_none")]
    pub node_chassis_serial_number: Option<String>,
    #[doc = "Serial number of the individual node"]
    #[serde(rename = "nodeSerialNumber", default, skip_serializing_if = "Option::is_none")]
    pub node_serial_number: Option<String>,
    #[doc = "Display Name of the individual node"]
    #[serde(rename = "nodeDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub node_display_name: Option<String>,
    #[doc = "Friendly software version name that is currently installed on the node"]
    #[serde(rename = "nodeFriendlySoftwareVersion", default, skip_serializing_if = "Option::is_none")]
    pub node_friendly_software_version: Option<String>,
    #[doc = "HCS version that is currently installed on the node"]
    #[serde(rename = "nodeHcsVersion", default, skip_serializing_if = "Option::is_none")]
    pub node_hcs_version: Option<String>,
    #[doc = "Guid instance id of the node"]
    #[serde(rename = "nodeInstanceId", default, skip_serializing_if = "Option::is_none")]
    pub node_instance_id: Option<String>,
}
impl NodeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod node_properties {
    use super::*;
    #[doc = "The current status of the individual node"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NodeStatus")]
    pub enum NodeStatus {
        Unknown,
        Up,
        Down,
        Rebooting,
        ShuttingDown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NodeStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NodeStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NodeStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("NodeStatus", 0u32, "Unknown"),
                Self::Up => serializer.serialize_unit_variant("NodeStatus", 1u32, "Up"),
                Self::Down => serializer.serialize_unit_variant("NodeStatus", 2u32, "Down"),
                Self::Rebooting => serializer.serialize_unit_variant("NodeStatus", 3u32, "Rebooting"),
                Self::ShuttingDown => serializer.serialize_unit_variant("NodeStatus", 4u32, "ShuttingDown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Is data action."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Operation display properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Origin of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Operation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation display properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Provider name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The type of resource in which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operation to be performed on the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation to be performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "Service specification."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of operations used for the discovery of available provider operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsList {
    #[doc = "The value."]
    pub value: Vec<Operation>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationsList {
    pub fn new(value: Vec<Operation>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The order details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Order {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Order properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OrderProperties>,
}
impl Order {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of order entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrderList {
    #[doc = "The list of orders."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Order>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OrderList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OrderList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Order properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderProperties {
    #[doc = "Contains all the contact details of the customer."]
    #[serde(rename = "contactInformation")]
    pub contact_information: ContactDetails,
    #[doc = "The shipping address of the customer."]
    #[serde(rename = "shippingAddress", default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,
    #[doc = "Represents a single status change."]
    #[serde(rename = "currentStatus", default, skip_serializing_if = "Option::is_none")]
    pub current_status: Option<OrderStatus>,
    #[doc = "List of status changes in the order."]
    #[serde(rename = "orderHistory", default, skip_serializing_if = "Vec::is_empty")]
    pub order_history: Vec<OrderStatus>,
    #[doc = "Serial number of the device."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "Tracking information for the package delivered to the customer whether it has an original or a replacement device."]
    #[serde(rename = "deliveryTrackingInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub delivery_tracking_info: Vec<TrackingInfo>,
    #[doc = "Tracking information for the package returned from the customer whether it has an original or a replacement device."]
    #[serde(rename = "returnTrackingInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub return_tracking_info: Vec<TrackingInfo>,
    #[doc = "ShipmentType of the order"]
    #[serde(rename = "shipmentType", default, skip_serializing_if = "Option::is_none")]
    pub shipment_type: Option<order_properties::ShipmentType>,
}
impl OrderProperties {
    pub fn new(contact_information: ContactDetails) -> Self {
        Self {
            contact_information,
            shipping_address: None,
            current_status: None,
            order_history: Vec::new(),
            serial_number: None,
            delivery_tracking_info: Vec::new(),
            return_tracking_info: Vec::new(),
            shipment_type: None,
        }
    }
}
pub mod order_properties {
    use super::*;
    #[doc = "ShipmentType of the order"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ShipmentType")]
    pub enum ShipmentType {
        NotApplicable,
        ShippedToCustomer,
        SelfPickup,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ShipmentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ShipmentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ShipmentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotApplicable => serializer.serialize_unit_variant("ShipmentType", 0u32, "NotApplicable"),
                Self::ShippedToCustomer => serializer.serialize_unit_variant("ShipmentType", 1u32, "ShippedToCustomer"),
                Self::SelfPickup => serializer.serialize_unit_variant("ShipmentType", 2u32, "SelfPickup"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents a single status change."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderStatus {
    #[doc = "Status of the order as per the allowed status types."]
    pub status: order_status::Status,
    #[doc = "Time of status update."]
    #[serde(rename = "updateDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub update_date_time: Option<time::OffsetDateTime>,
    #[doc = "Comments related to this status change."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[doc = "Tracking courier information."]
    #[serde(rename = "trackingInformation", default, skip_serializing_if = "Option::is_none")]
    pub tracking_information: Option<TrackingInfo>,
    #[doc = "Dictionary to hold generic information which is not stored\r\nby the already existing properties"]
    #[serde(rename = "additionalOrderDetails", default, skip_serializing_if = "Option::is_none")]
    pub additional_order_details: Option<serde_json::Value>,
}
impl OrderStatus {
    pub fn new(status: order_status::Status) -> Self {
        Self {
            status,
            update_date_time: None,
            comments: None,
            tracking_information: None,
            additional_order_details: None,
        }
    }
}
pub mod order_status {
    use super::*;
    #[doc = "Status of the order as per the allowed status types."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Untracked,
        AwaitingFulfillment,
        AwaitingPreparation,
        AwaitingShipment,
        Shipped,
        Arriving,
        Delivered,
        ReplacementRequested,
        LostDevice,
        Declined,
        ReturnInitiated,
        AwaitingReturnShipment,
        ShippedBack,
        CollectedAtMicrosoft,
        AwaitingPickup,
        PickupCompleted,
        AwaitingDrop,
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
                Self::Untracked => serializer.serialize_unit_variant("Status", 0u32, "Untracked"),
                Self::AwaitingFulfillment => serializer.serialize_unit_variant("Status", 1u32, "AwaitingFulfillment"),
                Self::AwaitingPreparation => serializer.serialize_unit_variant("Status", 2u32, "AwaitingPreparation"),
                Self::AwaitingShipment => serializer.serialize_unit_variant("Status", 3u32, "AwaitingShipment"),
                Self::Shipped => serializer.serialize_unit_variant("Status", 4u32, "Shipped"),
                Self::Arriving => serializer.serialize_unit_variant("Status", 5u32, "Arriving"),
                Self::Delivered => serializer.serialize_unit_variant("Status", 6u32, "Delivered"),
                Self::ReplacementRequested => serializer.serialize_unit_variant("Status", 7u32, "ReplacementRequested"),
                Self::LostDevice => serializer.serialize_unit_variant("Status", 8u32, "LostDevice"),
                Self::Declined => serializer.serialize_unit_variant("Status", 9u32, "Declined"),
                Self::ReturnInitiated => serializer.serialize_unit_variant("Status", 10u32, "ReturnInitiated"),
                Self::AwaitingReturnShipment => serializer.serialize_unit_variant("Status", 11u32, "AwaitingReturnShipment"),
                Self::ShippedBack => serializer.serialize_unit_variant("Status", 12u32, "ShippedBack"),
                Self::CollectedAtMicrosoft => serializer.serialize_unit_variant("Status", 13u32, "CollectedAtMicrosoft"),
                Self::AwaitingPickup => serializer.serialize_unit_variant("Status", 14u32, "AwaitingPickup"),
                Self::PickupCompleted => serializer.serialize_unit_variant("Status", 15u32, "PickupCompleted"),
                Self::AwaitingDrop => serializer.serialize_unit_variant("Status", 16u32, "AwaitingDrop"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Trigger details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeriodicTimerEventTrigger {
    #[serde(flatten)]
    pub trigger: Trigger,
    #[doc = "Periodic timer trigger properties."]
    pub properties: PeriodicTimerProperties,
    #[doc = "The path ID that uniquely identifies the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The object name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The hierarchical type of the object."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl PeriodicTimerEventTrigger {
    pub fn new(trigger: Trigger, properties: PeriodicTimerProperties) -> Self {
        Self {
            trigger,
            properties,
            id: None,
            name: None,
            type_: None,
        }
    }
}
#[doc = "Periodic timer trigger properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeriodicTimerProperties {
    #[doc = "Periodic timer event source."]
    #[serde(rename = "sourceInfo")]
    pub source_info: PeriodicTimerSourceInfo,
    #[doc = "Compute role against which events will be raised."]
    #[serde(rename = "sinkInfo")]
    pub sink_info: RoleSinkInfo,
    #[doc = "A custom context tag typically used to correlate the trigger against its usage. For example, if a periodic timer trigger is intended for certain specific IoT modules in the device, the tag can be the name or the image URL of the module."]
    #[serde(rename = "customContextTag", default, skip_serializing_if = "Option::is_none")]
    pub custom_context_tag: Option<String>,
}
impl PeriodicTimerProperties {
    pub fn new(source_info: PeriodicTimerSourceInfo, sink_info: RoleSinkInfo) -> Self {
        Self {
            source_info,
            sink_info,
            custom_context_tag: None,
        }
    }
}
#[doc = "Periodic timer event source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeriodicTimerSourceInfo {
    #[doc = "The time of the day that results in a valid trigger. Schedule is computed with reference to the time specified upto seconds. If timezone is not specified the time will considered to be in device timezone. The value will always be returned as UTC time."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "Periodic frequency at which timer event needs to be raised. Supports daily, hourly, minutes, and seconds."]
    pub schedule: String,
    #[doc = "Topic where periodic events are published to IoT device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}
impl PeriodicTimerSourceInfo {
    pub fn new(start_time: time::OffsetDateTime, schedule: String) -> Self {
        Self {
            start_time,
            schedule,
            topic: None,
        }
    }
}
#[doc = "The properties of proactive log collection settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProactiveLogCollectionSettingsProperties {
    #[doc = "Proactive diagnostic collection consent flag"]
    #[serde(rename = "userConsent")]
    pub user_consent: proactive_log_collection_settings_properties::UserConsent,
}
impl ProactiveLogCollectionSettingsProperties {
    pub fn new(user_consent: proactive_log_collection_settings_properties::UserConsent) -> Self {
        Self { user_consent }
    }
}
pub mod proactive_log_collection_settings_properties {
    use super::*;
    #[doc = "Proactive diagnostic collection consent flag"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UserConsent")]
    pub enum UserConsent {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UserConsent {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UserConsent {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UserConsent {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("UserConsent", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("UserConsent", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Raw Certificate Data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawCertificateData {
    #[doc = "The authentication type."]
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<raw_certificate_data::AuthenticationType>,
    #[doc = "The base64 encoded certificate raw data."]
    pub certificate: String,
}
impl RawCertificateData {
    pub fn new(certificate: String) -> Self {
        Self {
            authentication_type: None,
            certificate,
        }
    }
}
pub mod raw_certificate_data {
    use super::*;
    #[doc = "The authentication type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthenticationType")]
    pub enum AuthenticationType {
        Invalid,
        AzureActiveDirectory,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuthenticationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuthenticationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuthenticationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("AuthenticationType", 0u32, "Invalid"),
                Self::AzureActiveDirectory => serializer.serialize_unit_variant("AuthenticationType", 1u32, "AzureActiveDirectory"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Fields for tracking refresh job on the share or container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefreshDetails {
    #[doc = "If a refresh job is currently in progress on this share or container, this field indicates the ARM resource ID of that job. The field is empty if no job is in progress."]
    #[serde(rename = "inProgressRefreshJobId", default, skip_serializing_if = "Option::is_none")]
    pub in_progress_refresh_job_id: Option<String>,
    #[doc = "Indicates the completed time for the last refresh job on this particular share or container, if any.This could be a failed job or a successful job."]
    #[serde(rename = "lastCompletedRefreshJobTimeInUTC", default, with = "azure_core::date::rfc3339::option")]
    pub last_completed_refresh_job_time_in_utc: Option<time::OffsetDateTime>,
    #[doc = "Indicates the relative path of the error xml for the last refresh job on this particular share or container, if any. This could be a failed job or a successful job."]
    #[serde(rename = "errorManifestFile", default, skip_serializing_if = "Option::is_none")]
    pub error_manifest_file: Option<String>,
    #[doc = "Indicates the id of the last refresh job on this particular share or container,if any. This could be a failed job or a successful job."]
    #[serde(rename = "lastJob", default, skip_serializing_if = "Option::is_none")]
    pub last_job: Option<String>,
}
impl RefreshDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "RemoteApplicationType for which remote support settings is being modified"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemoteSupportSettings {
    #[doc = "Remote application type"]
    #[serde(rename = "remoteApplicationType", default, skip_serializing_if = "Option::is_none")]
    pub remote_application_type: Option<remote_support_settings::RemoteApplicationType>,
    #[doc = "Access level allowed for this remote application type"]
    #[serde(rename = "accessLevel", default, skip_serializing_if = "Option::is_none")]
    pub access_level: Option<remote_support_settings::AccessLevel>,
    #[doc = "Expiration time stamp"]
    #[serde(rename = "expirationTimeStampInUTC", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_time_stamp_in_utc: Option<time::OffsetDateTime>,
}
impl RemoteSupportSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod remote_support_settings {
    use super::*;
    #[doc = "Remote application type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RemoteApplicationType")]
    pub enum RemoteApplicationType {
        Powershell,
        #[serde(rename = "WAC")]
        Wac,
        #[serde(rename = "LocalUI")]
        LocalUi,
        AllApplications,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RemoteApplicationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RemoteApplicationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RemoteApplicationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Powershell => serializer.serialize_unit_variant("RemoteApplicationType", 0u32, "Powershell"),
                Self::Wac => serializer.serialize_unit_variant("RemoteApplicationType", 1u32, "WAC"),
                Self::LocalUi => serializer.serialize_unit_variant("RemoteApplicationType", 2u32, "LocalUI"),
                Self::AllApplications => serializer.serialize_unit_variant("RemoteApplicationType", 3u32, "AllApplications"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Access level allowed for this remote application type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccessLevel")]
    pub enum AccessLevel {
        None,
        ReadOnly,
        ReadWrite,
        FullAccess,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AccessLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AccessLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AccessLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("AccessLevel", 0u32, "None"),
                Self::ReadOnly => serializer.serialize_unit_variant("AccessLevel", 1u32, "ReadOnly"),
                Self::ReadWrite => serializer.serialize_unit_variant("AccessLevel", 2u32, "ReadWrite"),
                Self::FullAccess => serializer.serialize_unit_variant("AccessLevel", 3u32, "FullAccess"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Msi identity details of the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceIdentity {
    #[doc = "Identity type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<resource_identity::Type>,
    #[doc = "Service Principal Id backing the Msi"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Home Tenant Id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ResourceIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_identity {
    use super::*;
    #[doc = "Identity type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
        SystemAssigned,
        UserAssigned,
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
                Self::None => serializer.serialize_unit_variant("Type", 0u32, "None"),
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 1u32, "SystemAssigned"),
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 2u32, "UserAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Fields for tracking resource move"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceMoveDetails {
    #[doc = "Denotes whether move operation is in progress"]
    #[serde(rename = "operationInProgress", default, skip_serializing_if = "Option::is_none")]
    pub operation_in_progress: Option<resource_move_details::OperationInProgress>,
    #[doc = "Denotes the timeout of the operation to finish"]
    #[serde(
        rename = "operationInProgressLockTimeoutInUTC",
        default,
        with = "azure_core::date::rfc3339::option"
    )]
    pub operation_in_progress_lock_timeout_in_utc: Option<time::OffsetDateTime>,
}
impl ResourceMoveDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_move_details {
    use super::*;
    #[doc = "Denotes whether move operation is in progress"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OperationInProgress")]
    pub enum OperationInProgress {
        None,
        ResourceMoveInProgress,
        ResourceMoveFailed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OperationInProgress {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OperationInProgress {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OperationInProgress {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("OperationInProgress", 0u32, "None"),
                Self::ResourceMoveInProgress => serializer.serialize_unit_variant("OperationInProgress", 1u32, "ResourceMoveInProgress"),
                Self::ResourceMoveFailed => serializer.serialize_unit_variant("OperationInProgress", 2u32, "ResourceMoveFailed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Resource type Sku object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTypeSku {
    #[doc = "The resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The skus."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skus: Vec<SkuInformation>,
}
impl ResourceTypeSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Compute role."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Role {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "Role type."]
    pub kind: role::Kind,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Role {
    pub fn new(kind: role::Kind) -> Self {
        Self {
            arm_base_model: ArmBaseModel::default(),
            kind,
            system_data: None,
        }
    }
}
pub mod role {
    use super::*;
    #[doc = "Role type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "IOT")]
        Iot,
        #[serde(rename = "ASA")]
        Asa,
        Functions,
        Cognitive,
        #[serde(rename = "MEC")]
        Mec,
        CloudEdgeManagement,
        Kubernetes,
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
                Self::Iot => serializer.serialize_unit_variant("Kind", 0u32, "IOT"),
                Self::Asa => serializer.serialize_unit_variant("Kind", 1u32, "ASA"),
                Self::Functions => serializer.serialize_unit_variant("Kind", 2u32, "Functions"),
                Self::Cognitive => serializer.serialize_unit_variant("Kind", 3u32, "Cognitive"),
                Self::Mec => serializer.serialize_unit_variant("Kind", 4u32, "MEC"),
                Self::CloudEdgeManagement => serializer.serialize_unit_variant("Kind", 5u32, "CloudEdgeManagement"),
                Self::Kubernetes => serializer.serialize_unit_variant("Kind", 6u32, "Kubernetes"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Collection of all the roles on the Data Box Edge device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleList {
    #[doc = "The Value."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Role>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RoleList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Compute role against which events will be raised."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleSinkInfo {
    #[doc = "Compute role ID."]
    #[serde(rename = "roleId")]
    pub role_id: String,
}
impl RoleSinkInfo {
    pub fn new(role_id: String) -> Self {
        Self { role_id }
    }
}
#[doc = "Holds device secret either as a KeyVault reference or as an encrypted value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Secret {
    #[doc = "Represent the secrets intended for encryption with asymmetric key pair."]
    #[serde(rename = "encryptedSecret", default, skip_serializing_if = "Option::is_none")]
    pub encrypted_secret: Option<AsymmetricEncryptedSecret>,
    #[doc = "Id of the Key-Vault where secret is stored (ex: secrets/AuthClientSecret/82ef4346187a4033a10d629cde07d740)."]
    #[serde(rename = "keyVaultId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_id: Option<String>,
}
impl Secret {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The security settings of a device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecuritySettings {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "The properties of security settings."]
    pub properties: SecuritySettingsProperties,
}
impl SecuritySettings {
    pub fn new(properties: SecuritySettingsProperties) -> Self {
        Self {
            arm_base_model: ArmBaseModel::default(),
            properties,
        }
    }
}
#[doc = "The properties of security settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecuritySettingsProperties {
    #[doc = "Represent the secrets intended for encryption with asymmetric key pair."]
    #[serde(rename = "deviceAdminPassword")]
    pub device_admin_password: AsymmetricEncryptedSecret,
}
impl SecuritySettingsProperties {
    pub fn new(device_admin_password: AsymmetricEncryptedSecret) -> Self {
        Self { device_admin_password }
    }
}
#[doc = "Service specification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "Metric specification as defined by shoebox."]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecificationV1>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a share on the  Data Box Edge/Gateway device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Share {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The share properties."]
    pub properties: ShareProperties,
}
impl Share {
    pub fn new(properties: ShareProperties) -> Self {
        Self {
            arm_base_model: ArmBaseModel::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "Specifies the mapping between this particular user and the type of access he has on shares on this device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShareAccessRight {
    #[doc = "The share ID."]
    #[serde(rename = "shareId")]
    pub share_id: String,
    #[doc = "Type of access to be allowed on the share for this user."]
    #[serde(rename = "accessType")]
    pub access_type: share_access_right::AccessType,
}
impl ShareAccessRight {
    pub fn new(share_id: String, access_type: share_access_right::AccessType) -> Self {
        Self { share_id, access_type }
    }
}
pub mod share_access_right {
    use super::*;
    #[doc = "Type of access to be allowed on the share for this user."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccessType")]
    pub enum AccessType {
        Change,
        Read,
        Custom,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AccessType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AccessType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AccessType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Change => serializer.serialize_unit_variant("AccessType", 0u32, "Change"),
                Self::Read => serializer.serialize_unit_variant("AccessType", 1u32, "Read"),
                Self::Custom => serializer.serialize_unit_variant("AccessType", 2u32, "Custom"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Collection of all the shares on the Data Box Edge/Gateway device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShareList {
    #[doc = "The list of shares."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Share>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ShareList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ShareList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The share properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShareProperties {
    #[doc = "Description for the share."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Current status of the share."]
    #[serde(rename = "shareStatus")]
    pub share_status: share_properties::ShareStatus,
    #[doc = "Current monitoring status of the share."]
    #[serde(rename = "monitoringStatus")]
    pub monitoring_status: share_properties::MonitoringStatus,
    #[doc = "Azure container mapping of the endpoint."]
    #[serde(rename = "azureContainerInfo", default, skip_serializing_if = "Option::is_none")]
    pub azure_container_info: Option<AzureContainerInfo>,
    #[doc = "Access protocol to be used by the share."]
    #[serde(rename = "accessProtocol")]
    pub access_protocol: share_properties::AccessProtocol,
    #[doc = "Mapping of users and corresponding access rights on the share (required for SMB protocol)."]
    #[serde(rename = "userAccessRights", default, skip_serializing_if = "Vec::is_empty")]
    pub user_access_rights: Vec<UserAccessRight>,
    #[doc = "List of IP addresses and corresponding access rights on the share(required for NFS protocol)."]
    #[serde(rename = "clientAccessRights", default, skip_serializing_if = "Vec::is_empty")]
    pub client_access_rights: Vec<ClientAccessRight>,
    #[doc = "Fields for tracking refresh job on the share or container."]
    #[serde(rename = "refreshDetails", default, skip_serializing_if = "Option::is_none")]
    pub refresh_details: Option<RefreshDetails>,
    #[doc = "Share mount point to the role."]
    #[serde(rename = "shareMappings", default, skip_serializing_if = "Vec::is_empty")]
    pub share_mappings: Vec<MountPointMap>,
    #[doc = "Data policy of the share."]
    #[serde(rename = "dataPolicy", default, skip_serializing_if = "Option::is_none")]
    pub data_policy: Option<share_properties::DataPolicy>,
}
impl ShareProperties {
    pub fn new(
        share_status: share_properties::ShareStatus,
        monitoring_status: share_properties::MonitoringStatus,
        access_protocol: share_properties::AccessProtocol,
    ) -> Self {
        Self {
            description: None,
            share_status,
            monitoring_status,
            azure_container_info: None,
            access_protocol,
            user_access_rights: Vec::new(),
            client_access_rights: Vec::new(),
            refresh_details: None,
            share_mappings: Vec::new(),
            data_policy: None,
        }
    }
}
pub mod share_properties {
    use super::*;
    #[doc = "Current status of the share."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ShareStatus")]
    pub enum ShareStatus {
        Offline,
        Unknown,
        #[serde(rename = "OK")]
        Ok,
        Updating,
        NeedsAttention,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ShareStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ShareStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ShareStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Offline => serializer.serialize_unit_variant("ShareStatus", 0u32, "Offline"),
                Self::Unknown => serializer.serialize_unit_variant("ShareStatus", 1u32, "Unknown"),
                Self::Ok => serializer.serialize_unit_variant("ShareStatus", 2u32, "OK"),
                Self::Updating => serializer.serialize_unit_variant("ShareStatus", 3u32, "Updating"),
                Self::NeedsAttention => serializer.serialize_unit_variant("ShareStatus", 4u32, "NeedsAttention"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Current monitoring status of the share."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MonitoringStatus")]
    pub enum MonitoringStatus {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MonitoringStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MonitoringStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MonitoringStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("MonitoringStatus", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("MonitoringStatus", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Access protocol to be used by the share."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccessProtocol")]
    pub enum AccessProtocol {
        #[serde(rename = "SMB")]
        Smb,
        #[serde(rename = "NFS")]
        Nfs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AccessProtocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AccessProtocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AccessProtocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Smb => serializer.serialize_unit_variant("AccessProtocol", 0u32, "SMB"),
                Self::Nfs => serializer.serialize_unit_variant("AccessProtocol", 1u32, "NFS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Data policy of the share."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataPolicy")]
    pub enum DataPolicy {
        Cloud,
        Local,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataPolicy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataPolicy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataPolicy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Cloud => serializer.serialize_unit_variant("DataPolicy", 0u32, "Cloud"),
                Self::Local => serializer.serialize_unit_variant("DataPolicy", 1u32, "Local"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The SKU type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[doc = "SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<sku::Name>,
    #[doc = "The SKU tier. This is based on the SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sku {
    use super::*;
    #[doc = "SKU name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Gateway,
        Edge,
        #[serde(rename = "TEA_1Node")]
        Tea1node,
        #[serde(rename = "TEA_1Node_UPS")]
        Tea1nodeUps,
        #[serde(rename = "TEA_1Node_Heater")]
        Tea1nodeHeater,
        #[serde(rename = "TEA_1Node_UPS_Heater")]
        Tea1nodeUpsHeater,
        #[serde(rename = "TEA_4Node_Heater")]
        Tea4nodeHeater,
        #[serde(rename = "TEA_4Node_UPS_Heater")]
        Tea4nodeUpsHeater,
        #[serde(rename = "TMA")]
        Tma,
        #[serde(rename = "TDC")]
        Tdc,
        #[serde(rename = "TCA_Small")]
        TcaSmall,
        #[serde(rename = "GPU")]
        Gpu,
        #[serde(rename = "TCA_Large")]
        TcaLarge,
        #[serde(rename = "EdgeP_Base")]
        EdgePBase,
        #[serde(rename = "EdgeP_High")]
        EdgePHigh,
        #[serde(rename = "EdgePR_Base")]
        EdgePrBase,
        #[serde(rename = "EdgePR_Base_UPS")]
        EdgePrBaseUps,
        #[serde(rename = "EP2_64_1VPU_W")]
        Ep2641vpuW,
        #[serde(rename = "EP2_128_1T4_Mx1_W")]
        Ep21281t4Mx1W,
        #[serde(rename = "EP2_256_2T4_W")]
        Ep22562t4W,
        #[serde(rename = "EdgeMR_Mini")]
        EdgeMrMini,
        #[serde(rename = "RCA_Small")]
        RcaSmall,
        #[serde(rename = "RCA_Large")]
        RcaLarge,
        #[serde(rename = "RDC")]
        Rdc,
        Management,
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
                Self::Gateway => serializer.serialize_unit_variant("Name", 0u32, "Gateway"),
                Self::Edge => serializer.serialize_unit_variant("Name", 1u32, "Edge"),
                Self::Tea1node => serializer.serialize_unit_variant("Name", 2u32, "TEA_1Node"),
                Self::Tea1nodeUps => serializer.serialize_unit_variant("Name", 3u32, "TEA_1Node_UPS"),
                Self::Tea1nodeHeater => serializer.serialize_unit_variant("Name", 4u32, "TEA_1Node_Heater"),
                Self::Tea1nodeUpsHeater => serializer.serialize_unit_variant("Name", 5u32, "TEA_1Node_UPS_Heater"),
                Self::Tea4nodeHeater => serializer.serialize_unit_variant("Name", 6u32, "TEA_4Node_Heater"),
                Self::Tea4nodeUpsHeater => serializer.serialize_unit_variant("Name", 7u32, "TEA_4Node_UPS_Heater"),
                Self::Tma => serializer.serialize_unit_variant("Name", 8u32, "TMA"),
                Self::Tdc => serializer.serialize_unit_variant("Name", 9u32, "TDC"),
                Self::TcaSmall => serializer.serialize_unit_variant("Name", 10u32, "TCA_Small"),
                Self::Gpu => serializer.serialize_unit_variant("Name", 11u32, "GPU"),
                Self::TcaLarge => serializer.serialize_unit_variant("Name", 12u32, "TCA_Large"),
                Self::EdgePBase => serializer.serialize_unit_variant("Name", 13u32, "EdgeP_Base"),
                Self::EdgePHigh => serializer.serialize_unit_variant("Name", 14u32, "EdgeP_High"),
                Self::EdgePrBase => serializer.serialize_unit_variant("Name", 15u32, "EdgePR_Base"),
                Self::EdgePrBaseUps => serializer.serialize_unit_variant("Name", 16u32, "EdgePR_Base_UPS"),
                Self::Ep2641vpuW => serializer.serialize_unit_variant("Name", 17u32, "EP2_64_1VPU_W"),
                Self::Ep21281t4Mx1W => serializer.serialize_unit_variant("Name", 18u32, "EP2_128_1T4_Mx1_W"),
                Self::Ep22562t4W => serializer.serialize_unit_variant("Name", 19u32, "EP2_256_2T4_W"),
                Self::EdgeMrMini => serializer.serialize_unit_variant("Name", 20u32, "EdgeMR_Mini"),
                Self::RcaSmall => serializer.serialize_unit_variant("Name", 21u32, "RCA_Small"),
                Self::RcaLarge => serializer.serialize_unit_variant("Name", 22u32, "RCA_Large"),
                Self::Rdc => serializer.serialize_unit_variant("Name", 23u32, "RDC"),
                Self::Management => serializer.serialize_unit_variant("Name", 24u32, "Management"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The SKU tier. This is based on the SKU name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Standard,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("Tier", 0u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The metadata to describe the capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCapability {
    #[doc = "An invariant to describe the feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "An invariant if the feature is measured by quantity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SkuCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The metadata for retrieving price info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCost {
    #[doc = "Used for querying price from commerce."]
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "The cost quantity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[doc = "The extended unit."]
    #[serde(rename = "extendedUnit", default, skip_serializing_if = "Option::is_none")]
    pub extended_unit: Option<String>,
}
impl SkuCost {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sku information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuInformation {
    #[doc = "The sku name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The sku tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The sku kind."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "The Sku family."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The pricing info of the Sku."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub costs: Vec<SkuCost>,
    #[doc = "The locations where Sku is available."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "The locations where Sku is available with zones and sites info"]
    #[serde(rename = "locationInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub location_info: Vec<SkuLocationInfo>,
    #[doc = "The required quotaIds for the sku to be available."]
    #[serde(rename = "requiredQuotaIds", default, skip_serializing_if = "Vec::is_empty")]
    pub required_quota_ids: Vec<String>,
    #[doc = "The required features for the sku to be available."]
    #[serde(rename = "requiredFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub required_features: Vec<String>,
}
impl SkuInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of SKU Information objects"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuInformationList {
    #[doc = "List of ResourceTypeSku objects"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceTypeSku>,
    #[doc = "Links to the next set of results"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SkuInformationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The location info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuLocationInfo {
    #[doc = "The location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The zones."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "The sites."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sites: Vec<String>,
}
impl SkuLocationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Storage Account on the  Data Box Edge/Gateway device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccount {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The storage account properties."]
    pub properties: StorageAccountProperties,
}
impl StorageAccount {
    pub fn new(properties: StorageAccountProperties) -> Self {
        Self {
            arm_base_model: ArmBaseModel::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "The storage account credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCredential {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The storage account credential properties."]
    pub properties: StorageAccountCredentialProperties,
}
impl StorageAccountCredential {
    pub fn new(properties: StorageAccountCredentialProperties) -> Self {
        Self {
            arm_base_model: ArmBaseModel::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "The collection of storage account credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountCredentialList {
    #[doc = "The value."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageAccountCredential>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StorageAccountCredentialList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StorageAccountCredentialList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The storage account credential properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCredentialProperties {
    #[doc = "Alias for the storage account."]
    pub alias: String,
    #[doc = "Username for the storage account."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "Represent the secrets intended for encryption with asymmetric key pair."]
    #[serde(rename = "accountKey", default, skip_serializing_if = "Option::is_none")]
    pub account_key: Option<AsymmetricEncryptedSecret>,
    #[doc = "Connection string for the storage account. Use this string if username and account key are not specified."]
    #[serde(rename = "connectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    #[doc = "Signifies whether SSL needs to be enabled or not."]
    #[serde(rename = "sslStatus")]
    pub ssl_status: storage_account_credential_properties::SslStatus,
    #[doc = "Blob end point for private clouds."]
    #[serde(rename = "blobDomainName", default, skip_serializing_if = "Option::is_none")]
    pub blob_domain_name: Option<String>,
    #[doc = "Type of storage accessed on the storage account."]
    #[serde(rename = "accountType")]
    pub account_type: storage_account_credential_properties::AccountType,
    #[doc = "Id of the storage account."]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
}
impl StorageAccountCredentialProperties {
    pub fn new(
        alias: String,
        ssl_status: storage_account_credential_properties::SslStatus,
        account_type: storage_account_credential_properties::AccountType,
    ) -> Self {
        Self {
            alias,
            user_name: None,
            account_key: None,
            connection_string: None,
            ssl_status,
            blob_domain_name: None,
            account_type,
            storage_account_id: None,
        }
    }
}
pub mod storage_account_credential_properties {
    use super::*;
    #[doc = "Signifies whether SSL needs to be enabled or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SslStatus")]
    pub enum SslStatus {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SslStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SslStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SslStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("SslStatus", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("SslStatus", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of storage accessed on the storage account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccountType")]
    pub enum AccountType {
        GeneralPurposeStorage,
        BlobStorage,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AccountType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AccountType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AccountType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::GeneralPurposeStorage => serializer.serialize_unit_variant("AccountType", 0u32, "GeneralPurposeStorage"),
                Self::BlobStorage => serializer.serialize_unit_variant("AccountType", 1u32, "BlobStorage"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Collection of all the Storage Accounts on the Data Box Edge/Gateway device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountList {
    #[doc = "The list of storageAccounts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageAccount>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StorageAccountList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StorageAccountList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The storage account properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountProperties {
    #[doc = "Description for the storage Account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Current status of the storage account"]
    #[serde(rename = "storageAccountStatus", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_status: Option<storage_account_properties::StorageAccountStatus>,
    #[doc = "Data policy of the storage Account."]
    #[serde(rename = "dataPolicy")]
    pub data_policy: storage_account_properties::DataPolicy,
    #[doc = "Storage Account Credential Id"]
    #[serde(rename = "storageAccountCredentialId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_credential_id: Option<String>,
    #[doc = "BlobEndpoint of Storage Account"]
    #[serde(rename = "blobEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub blob_endpoint: Option<String>,
    #[doc = "The Container Count. Present only for Storage Accounts with DataPolicy set to Cloud."]
    #[serde(rename = "containerCount", default, skip_serializing_if = "Option::is_none")]
    pub container_count: Option<i32>,
}
impl StorageAccountProperties {
    pub fn new(data_policy: storage_account_properties::DataPolicy) -> Self {
        Self {
            description: None,
            storage_account_status: None,
            data_policy,
            storage_account_credential_id: None,
            blob_endpoint: None,
            container_count: None,
        }
    }
}
pub mod storage_account_properties {
    use super::*;
    #[doc = "Current status of the storage account"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageAccountStatus")]
    pub enum StorageAccountStatus {
        #[serde(rename = "OK")]
        Ok,
        Offline,
        Unknown,
        Updating,
        NeedsAttention,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageAccountStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageAccountStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageAccountStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ok => serializer.serialize_unit_variant("StorageAccountStatus", 0u32, "OK"),
                Self::Offline => serializer.serialize_unit_variant("StorageAccountStatus", 1u32, "Offline"),
                Self::Unknown => serializer.serialize_unit_variant("StorageAccountStatus", 2u32, "Unknown"),
                Self::Updating => serializer.serialize_unit_variant("StorageAccountStatus", 3u32, "Updating"),
                Self::NeedsAttention => serializer.serialize_unit_variant("StorageAccountStatus", 4u32, "NeedsAttention"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Data policy of the storage Account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataPolicy")]
    pub enum DataPolicy {
        Cloud,
        Local,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataPolicy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataPolicy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataPolicy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Cloud => serializer.serialize_unit_variant("DataPolicy", 0u32, "Cloud"),
                Self::Local => serializer.serialize_unit_variant("DataPolicy", 1u32, "Local"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionProperties {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "locationPlacementId", default, skip_serializing_if = "Option::is_none")]
    pub location_placement_id: Option<String>,
    #[serde(rename = "quotaId", default, skip_serializing_if = "Option::is_none")]
    pub quota_id: Option<String>,
    #[serde(rename = "serializedDetails", default, skip_serializing_if = "Option::is_none")]
    pub serialized_details: Option<String>,
    #[serde(rename = "registeredFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub registered_features: Vec<SubscriptionRegisteredFeatures>,
}
impl SubscriptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionRegisteredFeatures {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl SubscriptionRegisteredFeatures {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The share properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportPackageRequestProperties {
    #[doc = "MinimumTimeStamp from where logs need to be collected"]
    #[serde(rename = "minimumTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub minimum_time_stamp: Option<time::OffsetDateTime>,
    #[doc = "MaximumTimeStamp until where logs need to be collected"]
    #[serde(rename = "maximumTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub maximum_time_stamp: Option<time::OffsetDateTime>,
    #[doc = "Type of files, which need to be included in the logs\r\nThis will contain the type of logs (Default/DefaultWithDumps/None/All/DefaultWithArchived)\r\nor a comma separated list of log types that are required"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
}
impl SupportPackageRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Symmetric key for authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SymmetricKey {
    #[doc = "Represent the secrets intended for encryption with asymmetric key pair."]
    #[serde(rename = "connectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<AsymmetricEncryptedSecret>,
}
impl SymmetricKey {
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
    #[doc = "The type of identity that last modified the resource."]
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
#[doc = "Tracking courier information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackingInfo {
    #[doc = "Serial number of the device being tracked."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "Name of the carrier used in the delivery."]
    #[serde(rename = "carrierName", default, skip_serializing_if = "Option::is_none")]
    pub carrier_name: Option<String>,
    #[doc = "Tracking ID of the shipment."]
    #[serde(rename = "trackingId", default, skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[doc = "Tracking URL of the shipment."]
    #[serde(rename = "trackingUrl", default, skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
}
impl TrackingInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Trigger details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trigger {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Trigger Kind."]
    pub kind: trigger::Kind,
}
impl Trigger {
    pub fn new(kind: trigger::Kind) -> Self {
        Self {
            arm_base_model: ArmBaseModel::default(),
            system_data: None,
            kind,
        }
    }
}
pub mod trigger {
    use super::*;
    #[doc = "Trigger Kind."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        FileEvent,
        PeriodicTimerEvent,
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
                Self::FileEvent => serializer.serialize_unit_variant("Kind", 0u32, "FileEvent"),
                Self::PeriodicTimerEvent => serializer.serialize_unit_variant("Kind", 1u32, "PeriodicTimerEvent"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Collection of all trigger on the data box edge device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerList {
    #[doc = "The list of triggers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Trigger>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TriggerList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TriggerList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The request object for trigger support package."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerSupportPackageRequest {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "The share properties."]
    pub properties: SupportPackageRequestProperties,
}
impl TriggerSupportPackageRequest {
    pub fn new(properties: SupportPackageRequestProperties) -> Self {
        Self {
            arm_base_model: ArmBaseModel::default(),
            properties,
        }
    }
}
#[doc = "Update Specific attributes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateDetails {
    #[doc = "Title of the Update"]
    #[serde(rename = "updateTitle", default, skip_serializing_if = "Option::is_none")]
    pub update_title: Option<String>,
    #[doc = "Size of the update(In Bytes)"]
    #[serde(rename = "updateSize", default, skip_serializing_if = "Option::is_none")]
    pub update_size: Option<f64>,
    #[doc = "Type of the Update"]
    #[serde(rename = "updateType", default, skip_serializing_if = "Option::is_none")]
    pub update_type: Option<update_details::UpdateType>,
    #[doc = "Target Version number"]
    #[serde(rename = "targetVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_version: Option<String>,
    #[doc = "Friendly Version Number"]
    #[serde(rename = "friendlyVersionNumber", default, skip_serializing_if = "Option::is_none")]
    pub friendly_version_number: Option<String>,
    #[doc = "Estimated Install Time for the update"]
    #[serde(rename = "estimatedInstallTimeInMins", default, skip_serializing_if = "Option::is_none")]
    pub estimated_install_time_in_mins: Option<i32>,
    #[doc = "Indicates if updates are available and at least one of the updates needs a reboot."]
    #[serde(rename = "rebootBehavior", default, skip_serializing_if = "Option::is_none")]
    pub reboot_behavior: Option<update_details::RebootBehavior>,
    #[doc = "Impact of Installing an updateType"]
    #[serde(rename = "installationImpact", default, skip_serializing_if = "Option::is_none")]
    pub installation_impact: Option<update_details::InstallationImpact>,
    #[doc = "Status of the update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<update_details::Status>,
}
impl UpdateDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_details {
    use super::*;
    #[doc = "Type of the Update"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UpdateType")]
    pub enum UpdateType {
        Software,
        Kubernetes,
        Firmware,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UpdateType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UpdateType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UpdateType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Software => serializer.serialize_unit_variant("UpdateType", 0u32, "Software"),
                Self::Kubernetes => serializer.serialize_unit_variant("UpdateType", 1u32, "Kubernetes"),
                Self::Firmware => serializer.serialize_unit_variant("UpdateType", 2u32, "Firmware"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates if updates are available and at least one of the updates needs a reboot."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RebootBehavior")]
    pub enum RebootBehavior {
        NeverReboots,
        RequiresReboot,
        RequestReboot,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RebootBehavior {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RebootBehavior {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RebootBehavior {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NeverReboots => serializer.serialize_unit_variant("RebootBehavior", 0u32, "NeverReboots"),
                Self::RequiresReboot => serializer.serialize_unit_variant("RebootBehavior", 1u32, "RequiresReboot"),
                Self::RequestReboot => serializer.serialize_unit_variant("RebootBehavior", 2u32, "RequestReboot"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Impact of Installing an updateType"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InstallationImpact")]
    pub enum InstallationImpact {
        None,
        DeviceRebooted,
        KubernetesWorkloadsDown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InstallationImpact {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InstallationImpact {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InstallationImpact {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("InstallationImpact", 0u32, "None"),
                Self::DeviceRebooted => serializer.serialize_unit_variant("InstallationImpact", 1u32, "DeviceRebooted"),
                Self::KubernetesWorkloadsDown => serializer.serialize_unit_variant("InstallationImpact", 2u32, "KubernetesWorkloadsDown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Status of the update."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        DownloadPending,
        DownloadStarted,
        DownloadCompleted,
        InstallStarted,
        InstallCompleted,
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
                Self::DownloadPending => serializer.serialize_unit_variant("Status", 0u32, "DownloadPending"),
                Self::DownloadStarted => serializer.serialize_unit_variant("Status", 1u32, "DownloadStarted"),
                Self::DownloadCompleted => serializer.serialize_unit_variant("Status", 2u32, "DownloadCompleted"),
                Self::InstallStarted => serializer.serialize_unit_variant("Status", 3u32, "InstallStarted"),
                Self::InstallCompleted => serializer.serialize_unit_variant("Status", 4u32, "InstallCompleted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Details about the download progress of update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateDownloadProgress {
    #[doc = "The download phase."]
    #[serde(rename = "downloadPhase", default, skip_serializing_if = "Option::is_none")]
    pub download_phase: Option<update_download_progress::DownloadPhase>,
    #[doc = "Percentage of completion."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[doc = "Total bytes to download."]
    #[serde(rename = "totalBytesToDownload", default, skip_serializing_if = "Option::is_none")]
    pub total_bytes_to_download: Option<f64>,
    #[doc = "Total bytes downloaded."]
    #[serde(rename = "totalBytesDownloaded", default, skip_serializing_if = "Option::is_none")]
    pub total_bytes_downloaded: Option<f64>,
    #[doc = "Number of updates to download."]
    #[serde(rename = "numberOfUpdatesToDownload", default, skip_serializing_if = "Option::is_none")]
    pub number_of_updates_to_download: Option<i32>,
    #[doc = "Number of updates downloaded."]
    #[serde(rename = "numberOfUpdatesDownloaded", default, skip_serializing_if = "Option::is_none")]
    pub number_of_updates_downloaded: Option<i32>,
}
impl UpdateDownloadProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_download_progress {
    use super::*;
    #[doc = "The download phase."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DownloadPhase")]
    pub enum DownloadPhase {
        Unknown,
        Initializing,
        Downloading,
        Verifying,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DownloadPhase {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DownloadPhase {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DownloadPhase {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("DownloadPhase", 0u32, "Unknown"),
                Self::Initializing => serializer.serialize_unit_variant("DownloadPhase", 1u32, "Initializing"),
                Self::Downloading => serializer.serialize_unit_variant("DownloadPhase", 2u32, "Downloading"),
                Self::Verifying => serializer.serialize_unit_variant("DownloadPhase", 3u32, "Verifying"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Progress details during installation of updates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateInstallProgress {
    #[doc = "Percentage completed."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[doc = "Number of updates to install."]
    #[serde(rename = "numberOfUpdatesToInstall", default, skip_serializing_if = "Option::is_none")]
    pub number_of_updates_to_install: Option<i32>,
    #[doc = "Number of updates installed."]
    #[serde(rename = "numberOfUpdatesInstalled", default, skip_serializing_if = "Option::is_none")]
    pub number_of_updates_installed: Option<i32>,
}
impl UpdateInstallProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about ongoing updates and availability of updates on the device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateSummary {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The device update information summary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateSummaryProperties>,
}
impl UpdateSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The device update information summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateSummaryProperties {
    #[doc = "The current version of the device in format: 1.2.17312.13.\","]
    #[serde(rename = "deviceVersionNumber", default, skip_serializing_if = "Option::is_none")]
    pub device_version_number: Option<String>,
    #[doc = "The current version of the device in text format."]
    #[serde(rename = "friendlyDeviceVersionName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_device_version_name: Option<String>,
    #[doc = "The last time when a scan was done on the device."]
    #[serde(rename = "deviceLastScannedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub device_last_scanned_date_time: Option<time::OffsetDateTime>,
    #[doc = "The time when the last scan job was completed (success/cancelled/failed) on the appliance."]
    #[serde(rename = "lastCompletedScanJobDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_completed_scan_job_date_time: Option<time::OffsetDateTime>,
    #[doc = "Time when the last scan job is successfully completed."]
    #[serde(rename = "lastSuccessfulScanJobTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_successful_scan_job_time: Option<time::OffsetDateTime>,
    #[doc = "The time when the last Download job was completed (success/cancelled/failed) on the appliance."]
    #[serde(rename = "lastCompletedDownloadJobDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_completed_download_job_date_time: Option<time::OffsetDateTime>,
    #[doc = "JobId of the last ran download job.(Can be success/cancelled/failed)"]
    #[serde(rename = "lastCompletedDownloadJobId", default, skip_serializing_if = "Option::is_none")]
    pub last_completed_download_job_id: Option<String>,
    #[doc = "JobStatus of the last ran download job."]
    #[serde(rename = "lastDownloadJobStatus", default, skip_serializing_if = "Option::is_none")]
    pub last_download_job_status: Option<update_summary_properties::LastDownloadJobStatus>,
    #[doc = "The time when the Last Install job was completed successfully on the appliance"]
    #[serde(rename = "lastSuccessfulInstallJobDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_successful_install_job_date_time: Option<time::OffsetDateTime>,
    #[doc = "The time when the last Install job was completed (success/cancelled/failed) on the appliance."]
    #[serde(rename = "lastCompletedInstallJobDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_completed_install_job_date_time: Option<time::OffsetDateTime>,
    #[doc = "JobId of the last ran install job.(Can be success/cancelled/failed)"]
    #[serde(rename = "lastCompletedInstallJobId", default, skip_serializing_if = "Option::is_none")]
    pub last_completed_install_job_id: Option<String>,
    #[doc = "JobStatus of the last ran install job."]
    #[serde(rename = "lastInstallJobStatus", default, skip_serializing_if = "Option::is_none")]
    pub last_install_job_status: Option<update_summary_properties::LastInstallJobStatus>,
    #[doc = "The number of updates available for the current device version as per the last device scan."]
    #[serde(rename = "totalNumberOfUpdatesAvailable", default, skip_serializing_if = "Option::is_none")]
    pub total_number_of_updates_available: Option<i32>,
    #[doc = "The total number of items pending download."]
    #[serde(rename = "totalNumberOfUpdatesPendingDownload", default, skip_serializing_if = "Option::is_none")]
    pub total_number_of_updates_pending_download: Option<i32>,
    #[doc = "The total number of items pending install."]
    #[serde(rename = "totalNumberOfUpdatesPendingInstall", default, skip_serializing_if = "Option::is_none")]
    pub total_number_of_updates_pending_install: Option<i32>,
    #[doc = "Indicates if updates are available and at least one of the updates needs a reboot."]
    #[serde(rename = "rebootBehavior", default, skip_serializing_if = "Option::is_none")]
    pub reboot_behavior: Option<update_summary_properties::RebootBehavior>,
    #[doc = "The current update operation."]
    #[serde(rename = "ongoingUpdateOperation", default, skip_serializing_if = "Option::is_none")]
    pub ongoing_update_operation: Option<update_summary_properties::OngoingUpdateOperation>,
    #[doc = "The job ID of the download job in progress."]
    #[serde(rename = "inProgressDownloadJobId", default, skip_serializing_if = "Option::is_none")]
    pub in_progress_download_job_id: Option<String>,
    #[doc = "The job ID of the install job in progress."]
    #[serde(rename = "inProgressInstallJobId", default, skip_serializing_if = "Option::is_none")]
    pub in_progress_install_job_id: Option<String>,
    #[doc = "The time when the currently running download (if any) started."]
    #[serde(
        rename = "inProgressDownloadJobStartedDateTime",
        default,
        with = "azure_core::date::rfc3339::option"
    )]
    pub in_progress_download_job_started_date_time: Option<time::OffsetDateTime>,
    #[doc = "The time when the currently running install (if any) started."]
    #[serde(
        rename = "inProgressInstallJobStartedDateTime",
        default,
        with = "azure_core::date::rfc3339::option"
    )]
    pub in_progress_install_job_started_date_time: Option<time::OffsetDateTime>,
    #[doc = "The list of updates available for install."]
    #[serde(rename = "updateTitles", default, skip_serializing_if = "Vec::is_empty")]
    pub update_titles: Vec<String>,
    #[doc = "The list of updates available for install."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub updates: Vec<UpdateDetails>,
    #[doc = "The total size of updates available for download in bytes."]
    #[serde(rename = "totalUpdateSizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_update_size_in_bytes: Option<f64>,
    #[doc = "The total time in Minutes"]
    #[serde(rename = "totalTimeInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub total_time_in_minutes: Option<i32>,
}
impl UpdateSummaryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_summary_properties {
    use super::*;
    #[doc = "JobStatus of the last ran download job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastDownloadJobStatus")]
    pub enum LastDownloadJobStatus {
        Invalid,
        Running,
        Succeeded,
        Failed,
        Canceled,
        Paused,
        Scheduled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastDownloadJobStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastDownloadJobStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastDownloadJobStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("LastDownloadJobStatus", 0u32, "Invalid"),
                Self::Running => serializer.serialize_unit_variant("LastDownloadJobStatus", 1u32, "Running"),
                Self::Succeeded => serializer.serialize_unit_variant("LastDownloadJobStatus", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("LastDownloadJobStatus", 3u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("LastDownloadJobStatus", 4u32, "Canceled"),
                Self::Paused => serializer.serialize_unit_variant("LastDownloadJobStatus", 5u32, "Paused"),
                Self::Scheduled => serializer.serialize_unit_variant("LastDownloadJobStatus", 6u32, "Scheduled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "JobStatus of the last ran install job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastInstallJobStatus")]
    pub enum LastInstallJobStatus {
        Invalid,
        Running,
        Succeeded,
        Failed,
        Canceled,
        Paused,
        Scheduled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastInstallJobStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastInstallJobStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastInstallJobStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("LastInstallJobStatus", 0u32, "Invalid"),
                Self::Running => serializer.serialize_unit_variant("LastInstallJobStatus", 1u32, "Running"),
                Self::Succeeded => serializer.serialize_unit_variant("LastInstallJobStatus", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("LastInstallJobStatus", 3u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("LastInstallJobStatus", 4u32, "Canceled"),
                Self::Paused => serializer.serialize_unit_variant("LastInstallJobStatus", 5u32, "Paused"),
                Self::Scheduled => serializer.serialize_unit_variant("LastInstallJobStatus", 6u32, "Scheduled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates if updates are available and at least one of the updates needs a reboot."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RebootBehavior")]
    pub enum RebootBehavior {
        NeverReboots,
        RequiresReboot,
        RequestReboot,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RebootBehavior {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RebootBehavior {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RebootBehavior {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NeverReboots => serializer.serialize_unit_variant("RebootBehavior", 0u32, "NeverReboots"),
                Self::RequiresReboot => serializer.serialize_unit_variant("RebootBehavior", 1u32, "RequiresReboot"),
                Self::RequestReboot => serializer.serialize_unit_variant("RebootBehavior", 2u32, "RequestReboot"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The current update operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OngoingUpdateOperation")]
    pub enum OngoingUpdateOperation {
        None,
        Scan,
        Download,
        Install,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OngoingUpdateOperation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OngoingUpdateOperation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OngoingUpdateOperation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("OngoingUpdateOperation", 0u32, "None"),
                Self::Scan => serializer.serialize_unit_variant("OngoingUpdateOperation", 1u32, "Scan"),
                Self::Download => serializer.serialize_unit_variant("OngoingUpdateOperation", 2u32, "Download"),
                Self::Install => serializer.serialize_unit_variant("OngoingUpdateOperation", 3u32, "Install"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The upload certificate request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UploadCertificateRequest {
    #[doc = "Raw Certificate Data."]
    pub properties: RawCertificateData,
}
impl UploadCertificateRequest {
    pub fn new(properties: RawCertificateData) -> Self {
        Self { properties }
    }
}
#[doc = "The upload registration certificate response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UploadCertificateResponse {
    #[doc = "Specifies authentication type."]
    #[serde(rename = "authType", default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<upload_certificate_response::AuthType>,
    #[doc = "The resource ID of the Data Box Edge/Gateway device."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Azure Active Directory tenant authority."]
    #[serde(rename = "aadAuthority", default, skip_serializing_if = "Option::is_none")]
    pub aad_authority: Option<String>,
    #[doc = "Azure Active Directory tenant ID."]
    #[serde(rename = "aadTenantId", default, skip_serializing_if = "Option::is_none")]
    pub aad_tenant_id: Option<String>,
    #[doc = "Azure Active Directory service principal client ID."]
    #[serde(rename = "servicePrincipalClientId", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_client_id: Option<String>,
    #[doc = "Azure Active Directory service principal object ID."]
    #[serde(rename = "servicePrincipalObjectId", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_object_id: Option<String>,
    #[doc = "The azure management endpoint audience."]
    #[serde(rename = "azureManagementEndpointAudience", default, skip_serializing_if = "Option::is_none")]
    pub azure_management_endpoint_audience: Option<String>,
    #[doc = "Identifier of the target resource that is the recipient of the requested token."]
    #[serde(rename = "aadAudience", default, skip_serializing_if = "Option::is_none")]
    pub aad_audience: Option<String>,
}
impl UploadCertificateResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod upload_certificate_response {
    use super::*;
    #[doc = "Specifies authentication type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthType")]
    pub enum AuthType {
        Invalid,
        AzureActiveDirectory,
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
                Self::Invalid => serializer.serialize_unit_variant("AuthType", 0u32, "Invalid"),
                Self::AzureActiveDirectory => serializer.serialize_unit_variant("AuthType", 1u32, "AzureActiveDirectory"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents a user who has access to one or more shares on the Data Box Edge/Gateway device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(flatten)]
    pub arm_base_model: ArmBaseModel,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The user properties."]
    pub properties: UserProperties,
}
impl User {
    pub fn new(properties: UserProperties) -> Self {
        Self {
            arm_base_model: ArmBaseModel::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "The mapping between a particular user and the access type on the SMB share."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAccessRight {
    #[doc = "User ID (already existing in the device)."]
    #[serde(rename = "userId")]
    pub user_id: String,
    #[doc = "Type of access to be allowed for the user."]
    #[serde(rename = "accessType")]
    pub access_type: user_access_right::AccessType,
}
impl UserAccessRight {
    pub fn new(user_id: String, access_type: user_access_right::AccessType) -> Self {
        Self { user_id, access_type }
    }
}
pub mod user_access_right {
    use super::*;
    #[doc = "Type of access to be allowed for the user."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccessType")]
    pub enum AccessType {
        Change,
        Read,
        Custom,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AccessType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AccessType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AccessType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Change => serializer.serialize_unit_variant("AccessType", 0u32, "Change"),
                Self::Read => serializer.serialize_unit_variant("AccessType", 1u32, "Read"),
                Self::Custom => serializer.serialize_unit_variant("AccessType", 2u32, "Custom"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Collection of users."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserList {
    #[doc = "The list of users."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<User>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UserList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl UserList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The user properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserProperties {
    #[doc = "Represent the secrets intended for encryption with asymmetric key pair."]
    #[serde(rename = "encryptedPassword", default, skip_serializing_if = "Option::is_none")]
    pub encrypted_password: Option<AsymmetricEncryptedSecret>,
    #[doc = "List of shares that the user has rights on. This field should not be specified during user creation."]
    #[serde(rename = "shareAccessRights", default, skip_serializing_if = "Vec::is_empty")]
    pub share_access_rights: Vec<ShareAccessRight>,
    #[doc = "Type of the user."]
    #[serde(rename = "userType")]
    pub user_type: user_properties::UserType,
}
impl UserProperties {
    pub fn new(user_type: user_properties::UserType) -> Self {
        Self {
            encrypted_password: None,
            share_access_rights: Vec::new(),
            user_type,
        }
    }
}
pub mod user_properties {
    use super::*;
    #[doc = "Type of the user."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UserType")]
    pub enum UserType {
        Share,
        LocalManagement,
        #[serde(rename = "ARM")]
        Arm,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UserType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UserType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UserType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Share => serializer.serialize_unit_variant("UserType", 0u32, "Share"),
                Self::LocalManagement => serializer.serialize_unit_variant("UserType", 1u32, "LocalManagement"),
                Self::Arm => serializer.serialize_unit_variant("UserType", 2u32, "ARM"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
