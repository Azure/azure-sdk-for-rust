#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
pub type AvailabilityZone = String;
#[doc = "Response for ElasticSan request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticSan {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Elastic San response properties."]
    pub properties: ElasticSanProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ElasticSan {
    pub fn new(properties: ElasticSanProperties) -> Self {
        Self {
            tracked_resource: TrackedResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "List of Elastic Sans"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticSanList {
    #[doc = "An array of Elastic San objects."]
    pub value: Vec<ElasticSan>,
    #[doc = "URI to fetch the next section of the paginated response."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ElasticSanList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ElasticSanList {
    pub fn new(value: Vec<ElasticSan>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Metadata about an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticSanOperationDisplay {
    #[doc = "Localized friendly form of the resource provider name."]
    pub provider: String,
    #[doc = "Localized friendly form of the resource type related to this action/operation."]
    pub resource: String,
    #[doc = "Localized friendly name for the operation, as it should be shown to the user."]
    pub operation: String,
    #[doc = "Localized friendly description for the operation, as it should be shown to the user."]
    pub description: String,
}
impl ElasticSanOperationDisplay {
    pub fn new(provider: String, resource: String, operation: String, description: String) -> Self {
        Self {
            provider,
            resource,
            operation,
            description,
        }
    }
}
#[doc = "List of operations supported by the RP."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticSanOperationListResult {
    #[doc = "An array of operations supported by the ElasticSan RP."]
    pub value: Vec<ElasticSanRpOperation>,
    #[doc = "URI to fetch the next section of the paginated response."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ElasticSanOperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ElasticSanOperationListResult {
    pub fn new(value: Vec<ElasticSanRpOperation>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Elastic San response properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticSanProperties {
    #[doc = "The SKU name. Required for account creation; optional for update."]
    pub sku: Sku,
    #[doc = "Logical zone for Elastic San resource; example: [\"1\"]."]
    #[serde(
        rename = "availabilityZones",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub availability_zones: Vec<AvailabilityZone>,
    #[doc = "Provisioning state of the iSCSI Target."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Base size of the Elastic San appliance in TiB."]
    #[serde(rename = "baseSizeTiB")]
    pub base_size_ti_b: i64,
    #[doc = "Extended size of the Elastic San appliance in TiB."]
    #[serde(rename = "extendedCapacitySizeTiB")]
    pub extended_capacity_size_ti_b: i64,
    #[doc = "Total size of the provisioned Volumes in GiB."]
    #[serde(rename = "totalVolumeSizeGiB", default, skip_serializing_if = "Option::is_none")]
    pub total_volume_size_gi_b: Option<i64>,
    #[doc = "Total number of volume groups in this Elastic San appliance."]
    #[serde(rename = "volumeGroupCount", default, skip_serializing_if = "Option::is_none")]
    pub volume_group_count: Option<i64>,
    #[doc = "Total Provisioned IOPS of the Elastic San appliance."]
    #[serde(rename = "totalIops", default, skip_serializing_if = "Option::is_none")]
    pub total_iops: Option<i64>,
    #[doc = "Total Provisioned MBps Elastic San appliance."]
    #[serde(rename = "totalMBps", default, skip_serializing_if = "Option::is_none")]
    pub total_m_bps: Option<i64>,
    #[doc = "Total size of the Elastic San appliance in TB."]
    #[serde(rename = "totalSizeTiB", default, skip_serializing_if = "Option::is_none")]
    pub total_size_ti_b: Option<i64>,
}
impl ElasticSanProperties {
    pub fn new(sku: Sku, base_size_ti_b: i64, extended_capacity_size_ti_b: i64) -> Self {
        Self {
            sku,
            availability_zones: Vec::new(),
            provisioning_state: None,
            base_size_ti_b,
            extended_capacity_size_ti_b,
            total_volume_size_gi_b: None,
            volume_group_count: None,
            total_iops: None,
            total_m_bps: None,
            total_size_ti_b: None,
        }
    }
}
#[doc = "Description of a ElasticSan RP Operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticSanRpOperation {
    #[doc = "The name of the operation being performed on this particular object"]
    pub name: String,
    #[doc = "Indicates whether the operation applies to data-plane."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Metadata about an operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<ElasticSanOperationDisplay>,
}
impl ElasticSanRpOperation {
    pub fn new(name: String) -> Self {
        Self {
            name,
            is_data_action: None,
            display: None,
        }
    }
}
#[doc = "Response for ElasticSan update request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticSanUpdate {
    #[doc = "Elastic San update properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ElasticSanUpdateProperties>,
    #[doc = "Update tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ElasticSanUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Elastic San update properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticSanUpdateProperties {
    #[doc = "Base size of the Elastic San appliance in TiB."]
    #[serde(rename = "baseSizeTiB", default, skip_serializing_if = "Option::is_none")]
    pub base_size_ti_b: Option<i64>,
    #[doc = "Extended size of the Elastic San appliance in TiB."]
    #[serde(rename = "extendedCapacitySizeTiB", default, skip_serializing_if = "Option::is_none")]
    pub extended_capacity_size_ti_b: Option<i64>,
}
impl ElasticSanUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of key used to encrypt the data of the disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EncryptionType")]
pub enum EncryptionType {
    EncryptionAtRestWithPlatformKey,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EncryptionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EncryptionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EncryptionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::EncryptionAtRestWithPlatformKey => {
                serializer.serialize_unit_variant("EncryptionType", 0u32, "EncryptionAtRestWithPlatformKey")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource management error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "The resource management error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl azure_core::Continuable for Error {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
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
#[doc = "The resource management error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
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
    pub details: Vec<ErrorResponse>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Iscsi target information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IscsiTargetInfo {
    #[doc = "iSCSI Target IQN (iSCSI Qualified Name); example: \"iqn.2005-03.org.iscsi:server\"."]
    #[serde(rename = "targetIqn", default, skip_serializing_if = "Option::is_none")]
    pub target_iqn: Option<String>,
    #[doc = "iSCSI Target Portal Host Name"]
    #[serde(rename = "targetPortalHostname", default, skip_serializing_if = "Option::is_none")]
    pub target_portal_hostname: Option<String>,
    #[doc = "iSCSI Target Portal Port"]
    #[serde(rename = "targetPortalPort", default, skip_serializing_if = "Option::is_none")]
    pub target_portal_port: Option<i32>,
    #[doc = "Provisioning state of the iSCSI Target."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Operational status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OperationalStatus>,
}
impl IscsiTargetInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A set of rules governing the network accessibility."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRuleSet {
    #[doc = "The list of virtual network rules."]
    #[serde(
        rename = "virtualNetworkRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
}
impl NetworkRuleSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operational status of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperationalStatus")]
pub enum OperationalStatus {
    Invalid,
    Unknown,
    Healthy,
    Unhealthy,
    Updating,
    Running,
    Stopped,
    #[serde(rename = "Stopped (deallocated)")]
    StoppedDeallocated,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperationalStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperationalStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperationalStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("OperationalStatus", 0u32, "Invalid"),
            Self::Unknown => serializer.serialize_unit_variant("OperationalStatus", 1u32, "Unknown"),
            Self::Healthy => serializer.serialize_unit_variant("OperationalStatus", 2u32, "Healthy"),
            Self::Unhealthy => serializer.serialize_unit_variant("OperationalStatus", 3u32, "Unhealthy"),
            Self::Updating => serializer.serialize_unit_variant("OperationalStatus", 4u32, "Updating"),
            Self::Running => serializer.serialize_unit_variant("OperationalStatus", 5u32, "Running"),
            Self::Stopped => serializer.serialize_unit_variant("OperationalStatus", 6u32, "Stopped"),
            Self::StoppedDeallocated => serializer.serialize_unit_variant("OperationalStatus", 7u32, "Stopped (deallocated)"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Provisioning state of the iSCSI Target."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Invalid,
    Succeeded,
    Failed,
    Canceled,
    Pending,
    Creating,
    Updating,
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
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
            Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Pending"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleting"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Azure resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Azure resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The capability information in the specified SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCapability {
    #[doc = "The name of capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A string value to indicate states of given capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SkuCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SKU name. Required for account creation; optional for update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The sku name."]
    pub name: SkuName,
    #[doc = "The sku tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<SkuTier>,
}
impl Sku {
    pub fn new(name: SkuName) -> Self {
        Self { name, tier: None }
    }
}
#[doc = "ElasticSAN SKU and its properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuInformation {
    #[doc = "The sku name."]
    pub name: SkuName,
    #[doc = "The sku tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<SkuTier>,
    #[doc = "The type of the resource."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The set of locations that the SKU is available. This will be supported and registered Azure Geo Regions (e.g. West US, East US, Southeast Asia, etc.)."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub locations: Vec<String>,
    #[doc = "Availability of the SKU for the location/zone"]
    #[serde(
        rename = "locationInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub location_info: Vec<SkuLocationInfo>,
    #[doc = "The capability information in the specified SKU."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub capabilities: Vec<SkuCapability>,
}
impl SkuInformation {
    pub fn new(name: SkuName) -> Self {
        Self {
            name,
            tier: None,
            resource_type: None,
            locations: Vec::new(),
            location_info: Vec::new(),
            capabilities: Vec::new(),
        }
    }
}
#[doc = "List of SKU Information objects"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuInformationList {
    #[doc = "List of ResourceType Sku"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SkuInformation>,
}
impl azure_core::Continuable for SkuInformationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub zones: Vec<String>,
}
impl SkuLocationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The sku name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SkuName")]
pub enum SkuName {
    #[serde(rename = "Premium_LRS")]
    PremiumLrs,
    #[serde(rename = "Premium_ZRS")]
    PremiumZrs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SkuName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SkuName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SkuName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::PremiumLrs => serializer.serialize_unit_variant("SkuName", 0u32, "Premium_LRS"),
            Self::PremiumZrs => serializer.serialize_unit_variant("SkuName", 1u32, "Premium_ZRS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The sku tier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SkuTier")]
pub enum SkuTier {
    Premium,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SkuTier {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SkuTier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SkuTier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Premium => serializer.serialize_unit_variant("SkuTier", 0u32, "Premium"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Data source used when creating the volume."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceCreationData {
    #[doc = "This enumerates the possible sources of a volume creation."]
    #[serde(rename = "createSource", default, skip_serializing_if = "Option::is_none")]
    pub create_source: Option<source_creation_data::CreateSource>,
    #[doc = "If createOption is Copy, this is the ARM id of the source snapshot or disk. If createOption is Restore, this is the ARM-like id of the source disk restore point."]
    #[serde(rename = "sourceUri", default, skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
}
impl SourceCreationData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod source_creation_data {
    use super::*;
    #[doc = "This enumerates the possible sources of a volume creation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateSource {
        None,
    }
}
#[doc = "Storage Target type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "StorageTargetType")]
pub enum StorageTargetType {
    Iscsi,
    None,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for StorageTargetType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for StorageTargetType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for StorageTargetType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Iscsi => serializer.serialize_unit_variant("StorageTargetType", 0u32, "Iscsi"),
            Self::None => serializer.serialize_unit_variant("StorageTargetType", 1u32, "None"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource model definition for a ARM tracked top level resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The geo-location where the resource lives."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual Network rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    #[doc = "Resource ID of a subnet, for example: /subscriptions/{subscriptionId}/resourceGroups/{groupName}/providers/Microsoft.Network/virtualNetworks/{vnetName}/subnets/{subnetName}."]
    pub id: String,
    #[doc = "The action of virtual network rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<virtual_network_rule::Action>,
    #[doc = "Gets the state of virtual network rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<virtual_network_rule::State>,
}
impl VirtualNetworkRule {
    pub fn new(id: String) -> Self {
        Self {
            id,
            action: None,
            state: None,
        }
    }
}
pub mod virtual_network_rule {
    use super::*;
    #[doc = "The action of virtual network rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Allow,
    }
    impl Default for Action {
        fn default() -> Self {
            Self::Allow
        }
    }
    #[doc = "Gets the state of virtual network rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "provisioning")]
        Provisioning,
        #[serde(rename = "deprovisioning")]
        Deprovisioning,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "networkSourceDeleted")]
        NetworkSourceDeleted,
    }
}
#[doc = "Response for Volume request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Volume {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Volume response properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VolumeProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Volume {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for Volume Group request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "VolumeGroup response properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VolumeGroupProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl VolumeGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Volume Groups"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeGroupList {
    #[doc = "An array of Volume Groups objects."]
    pub value: Vec<VolumeGroup>,
    #[doc = "URI to fetch the next section of the paginated response."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VolumeGroupList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VolumeGroupList {
    pub fn new(value: Vec<VolumeGroup>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "VolumeGroup response properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeGroupProperties {
    #[doc = "Provisioning state of the iSCSI Target."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Storage Target type."]
    #[serde(rename = "protocolType", default, skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<StorageTargetType>,
    #[doc = "The type of key used to encrypt the data of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionType>,
    #[doc = "A set of rules governing the network accessibility."]
    #[serde(rename = "networkAcls", default, skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,
}
impl VolumeGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Volume Group request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeGroupUpdate {
    #[doc = "VolumeGroup response properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VolumeGroupUpdateProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl VolumeGroupUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VolumeGroup response properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeGroupUpdateProperties {
    #[doc = "Storage Target type."]
    #[serde(rename = "protocolType", default, skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<StorageTargetType>,
    #[doc = "The type of key used to encrypt the data of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionType>,
    #[doc = "A set of rules governing the network accessibility."]
    #[serde(rename = "networkAcls", default, skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,
}
impl VolumeGroupUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Volumes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeList {
    #[doc = "An array of Volume objects."]
    pub value: Vec<Volume>,
    #[doc = "URI to fetch the next section of the paginated response."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VolumeList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VolumeList {
    pub fn new(value: Vec<Volume>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Volume response properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeProperties {
    #[doc = "Unique Id of the volume in GUID format"]
    #[serde(rename = "volumeId", default, skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
    #[doc = "Data source used when creating the volume."]
    #[serde(rename = "creationData", default, skip_serializing_if = "Option::is_none")]
    pub creation_data: Option<SourceCreationData>,
    #[doc = "Volume size."]
    #[serde(rename = "sizeGiB", default, skip_serializing_if = "Option::is_none")]
    pub size_gi_b: Option<i64>,
    #[doc = "Iscsi target information"]
    #[serde(rename = "storageTarget", default, skip_serializing_if = "Option::is_none")]
    pub storage_target: Option<IscsiTargetInfo>,
}
impl VolumeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for Volume request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeUpdate {
    #[doc = "Volume response properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VolumeUpdateProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl VolumeUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Volume response properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeUpdateProperties {
    #[doc = "Volume size."]
    #[serde(rename = "sizeGiB", default, skip_serializing_if = "Option::is_none")]
    pub size_gi_b: Option<i64>,
}
impl VolumeUpdateProperties {
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
