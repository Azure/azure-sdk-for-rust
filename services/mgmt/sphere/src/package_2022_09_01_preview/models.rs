#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Allow crash dumps values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AllowCrashDumpCollection")]
pub enum AllowCrashDumpCollection {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AllowCrashDumpCollection {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AllowCrashDumpCollection {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AllowCrashDumpCollection {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("AllowCrashDumpCollection", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("AllowCrashDumpCollection", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Capability image type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CapabilityType")]
pub enum CapabilityType {
    ApplicationDevelopment,
    FieldServicing,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CapabilityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CapabilityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CapabilityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ApplicationDevelopment => serializer.serialize_unit_variant("CapabilityType", 0u32, "ApplicationDevelopment"),
            Self::FieldServicing => serializer.serialize_unit_variant("CapabilityType", 1u32, "FieldServicing"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An Azure Sphere catalog"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Catalog {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Catalog properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CatalogProperties>,
}
impl Catalog {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a Catalog list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CatalogListResult {
    #[doc = "The Catalog items on this page"]
    pub value: Vec<Catalog>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CatalogListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CatalogListResult {
    pub fn new(value: Vec<Catalog>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Catalog properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl CatalogProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type used for update operations of the Catalog."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CatalogUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An certificate resource belonging to a catalog resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Certificate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of certificate"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CertificateProperties>,
}
impl Certificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The certificate chain response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateChainResponse {
    #[doc = "The certificate chain."]
    #[serde(rename = "certificateChain", default, skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
}
impl CertificateChainResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a Certificate list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateListResult {
    #[doc = "The Certificate items on this page"]
    pub value: Vec<Certificate>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CertificateListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CertificateListResult {
    pub fn new(value: Vec<Certificate>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of certificate"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateProperties {
    #[doc = "The certificate as a UTF-8 encoded base 64 string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[doc = "Certificate status values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CertificateStatus>,
    #[doc = "The certificate subject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "The certificate thumbprint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "The certificate expiry date."]
    #[serde(rename = "expiryUtc", default, with = "azure_core::date::rfc3339::option")]
    pub expiry_utc: Option<time::OffsetDateTime>,
    #[doc = "The certificate not before date."]
    #[serde(rename = "notBeforeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub not_before_utc: Option<time::OffsetDateTime>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl CertificateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Certificate status values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CertificateStatus")]
pub enum CertificateStatus {
    Active,
    Inactive,
    Expired,
    Revoked,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CertificateStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CertificateStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CertificateStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("CertificateStatus", 0u32, "Active"),
            Self::Inactive => serializer.serialize_unit_variant("CertificateStatus", 1u32, "Inactive"),
            Self::Expired => serializer.serialize_unit_variant("CertificateStatus", 2u32, "Expired"),
            Self::Revoked => serializer.serialize_unit_variant("CertificateStatus", 3u32, "Revoked"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Request to the action call to bulk claim devices."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClaimDevicesRequest {
    #[doc = "Device identifiers of the devices to be claimed."]
    #[serde(rename = "deviceIdentifiers")]
    pub device_identifiers: Vec<String>,
}
impl ClaimDevicesRequest {
    pub fn new(device_identifiers: Vec<String>) -> Self {
        Self { device_identifiers }
    }
}
#[doc = "Response to the action call for count devices in a catalog."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CountDeviceResponse {
    #[serde(flatten)]
    pub count_elements_response: CountElementsResponse,
}
impl CountDeviceResponse {
    pub fn new(count_elements_response: CountElementsResponse) -> Self {
        Self { count_elements_response }
    }
}
#[doc = "Response of the count for elements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CountElementsResponse {
    #[doc = "Number of children resources in parent resource."]
    pub value: i32,
}
impl CountElementsResponse {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}
#[doc = "Provides the custom '$filter' query parameter for list operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomFilterQueryParameter {}
impl CustomFilterQueryParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provides the custom '$maxpagesize' query parameter for list operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomMaxPageSizeQueryParameter {}
impl CustomMaxPageSizeQueryParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provides the custom '$skip' query parameter for list operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomSkipQueryParameter {}
impl CustomSkipQueryParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provides the custom '$top' query parameter for list operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomTopQueryParameter {}
impl CustomTopQueryParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An deployment resource belonging to a device group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Deployment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of deployment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeploymentProperties>,
}
impl Deployment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a Deployment list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentListResult {
    #[doc = "The Deployment items on this page"]
    pub value: Vec<Deployment>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeploymentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeploymentListResult {
    pub fn new(value: Vec<Deployment>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of deployment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentProperties {
    #[doc = "Deployment ID"]
    #[serde(rename = "deploymentId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[doc = "Images deployed"]
    #[serde(
        rename = "deployedImages",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub deployed_images: Vec<Image>,
    #[doc = "Deployment date UTC"]
    #[serde(rename = "deploymentDateUtc", default, with = "azure_core::date::rfc3339::option")]
    pub deployment_date_utc: Option<time::OffsetDateTime>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl DeploymentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An device resource belonging to a device group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Device {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of device"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeviceProperties>,
}
impl Device {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An device group resource belonging to a product resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of deviceGroup"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeviceGroupProperties>,
}
impl DeviceGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a DeviceGroup list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceGroupListResult {
    #[doc = "The DeviceGroup items on this page"]
    pub value: Vec<DeviceGroup>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeviceGroupListResult {
    pub fn new(value: Vec<DeviceGroup>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of deviceGroup"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceGroupProperties {
    #[doc = "Description of the device group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "OS feed type values."]
    #[serde(rename = "osFeedType", default, skip_serializing_if = "Option::is_none")]
    pub os_feed_type: Option<OsFeedType>,
    #[doc = "Update policy values."]
    #[serde(rename = "updatePolicy", default, skip_serializing_if = "Option::is_none")]
    pub update_policy: Option<UpdatePolicy>,
    #[doc = "Allow crash dumps values."]
    #[serde(rename = "allowCrashDumpsCollection", default, skip_serializing_if = "Option::is_none")]
    pub allow_crash_dumps_collection: Option<AllowCrashDumpCollection>,
    #[doc = "Regional data boundary values."]
    #[serde(rename = "regionalDataBoundary", default, skip_serializing_if = "Option::is_none")]
    pub regional_data_boundary: Option<RegionalDataBoundary>,
    #[doc = "Deployment status for the device group."]
    #[serde(rename = "hasDeployment", default, skip_serializing_if = "Option::is_none")]
    pub has_deployment: Option<bool>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl DeviceGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type used for update operations of the DeviceGroup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceGroupUpdate {
    #[doc = "The updatable properties of the DeviceGroup."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeviceGroupUpdateProperties>,
}
impl DeviceGroupUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the DeviceGroup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceGroupUpdateProperties {
    #[doc = "Description of the device group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "OS feed type values."]
    #[serde(rename = "osFeedType", default, skip_serializing_if = "Option::is_none")]
    pub os_feed_type: Option<OsFeedType>,
    #[doc = "Update policy values."]
    #[serde(rename = "updatePolicy", default, skip_serializing_if = "Option::is_none")]
    pub update_policy: Option<UpdatePolicy>,
    #[doc = "Allow crash dumps values."]
    #[serde(rename = "allowCrashDumpsCollection", default, skip_serializing_if = "Option::is_none")]
    pub allow_crash_dumps_collection: Option<AllowCrashDumpCollection>,
    #[doc = "Regional data boundary values."]
    #[serde(rename = "regionalDataBoundary", default, skip_serializing_if = "Option::is_none")]
    pub regional_data_boundary: Option<RegionalDataBoundary>,
}
impl DeviceGroupUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Device insight report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceInsight {
    #[doc = "Device ID"]
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[doc = "Event description"]
    pub description: String,
    #[doc = "Event start timestamp"]
    #[serde(rename = "startTimestampUtc", with = "azure_core::date::rfc3339")]
    pub start_timestamp_utc: time::OffsetDateTime,
    #[doc = "Event end timestamp"]
    #[serde(rename = "endTimestampUtc", with = "azure_core::date::rfc3339")]
    pub end_timestamp_utc: time::OffsetDateTime,
    #[doc = "Event category"]
    #[serde(rename = "eventCategory")]
    pub event_category: String,
    #[doc = "Event class"]
    #[serde(rename = "eventClass")]
    pub event_class: String,
    #[doc = "Event type"]
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[doc = "Event count"]
    #[serde(rename = "eventCount")]
    pub event_count: i32,
}
impl DeviceInsight {
    pub fn new(
        device_id: String,
        description: String,
        start_timestamp_utc: time::OffsetDateTime,
        end_timestamp_utc: time::OffsetDateTime,
        event_category: String,
        event_class: String,
        event_type: String,
        event_count: i32,
    ) -> Self {
        Self {
            device_id,
            description,
            start_timestamp_utc,
            end_timestamp_utc,
            event_category,
            event_class,
            event_type,
            event_count,
        }
    }
}
#[doc = "The response of a Device list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceListResult {
    #[doc = "The Device items on this page"]
    pub value: Vec<Device>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeviceListResult {
    pub fn new(value: Vec<Device>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of device patch"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevicePatchProperties {
    #[doc = "Device group id"]
    #[serde(rename = "deviceGroupId")]
    pub device_group_id: String,
}
impl DevicePatchProperties {
    pub fn new(device_group_id: String) -> Self {
        Self { device_group_id }
    }
}
#[doc = "The properties of device"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceProperties {
    #[doc = "Device ID"]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "SKU of the chip"]
    #[serde(rename = "chipSku", default, skip_serializing_if = "Option::is_none")]
    pub chip_sku: Option<String>,
    #[doc = "OS version available for installation when update requested"]
    #[serde(rename = "lastAvailableOsVersion", default, skip_serializing_if = "Option::is_none")]
    pub last_available_os_version: Option<String>,
    #[doc = "OS version running on device when update requested"]
    #[serde(rename = "lastInstalledOsVersion", default, skip_serializing_if = "Option::is_none")]
    pub last_installed_os_version: Option<String>,
    #[doc = "Time when update requested and new OS version available"]
    #[serde(rename = "lastOsUpdateUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_os_update_utc: Option<time::OffsetDateTime>,
    #[doc = "Time when update was last requested"]
    #[serde(rename = "lastUpdateRequestUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_update_request_utc: Option<time::OffsetDateTime>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl DeviceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type used for update operations of the Device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceUpdate {
    #[doc = "The updatable properties of the Device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeviceUpdateProperties>,
}
impl DeviceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the Device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceUpdateProperties {
    #[doc = "Device group id"]
    #[serde(rename = "deviceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub device_group_id: Option<String>,
}
impl DeviceUpdateProperties {
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
#[doc = "Request of the action to create a signed device capability image"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenerateCapabilityImageRequest {
    #[doc = "List of capabilities to create"]
    pub capabilities: Vec<CapabilityType>,
}
impl GenerateCapabilityImageRequest {
    pub fn new(capabilities: Vec<CapabilityType>) -> Self {
        Self { capabilities }
    }
}
#[doc = "An image resource belonging to a catalog resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Image {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of image"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImageProperties>,
}
impl Image {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a Image list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageListResult {
    #[doc = "The Image items on this page"]
    pub value: Vec<Image>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ImageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ImageListResult {
    pub fn new(value: Vec<Image>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of image"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageProperties {
    #[doc = "Image as a UTF-8 encoded base 64 string on image create. This field contains the image URI on image reads."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[doc = "Image ID"]
    #[serde(rename = "imageId", default, skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[doc = "Image name"]
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[doc = "Regional data boundary values."]
    #[serde(rename = "regionalDataBoundary", default, skip_serializing_if = "Option::is_none")]
    pub regional_data_boundary: Option<RegionalDataBoundary>,
    #[doc = "Location the image"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "The image description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The image component id."]
    #[serde(rename = "componentId", default, skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    #[doc = "Image type values."]
    #[serde(rename = "imageType", default, skip_serializing_if = "Option::is_none")]
    pub image_type: Option<ImageType>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ImageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image type values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ImageType")]
pub enum ImageType {
    InvalidImageType,
    OneBl,
    PlutonRuntime,
    WifiFirmware,
    SecurityMonitor,
    NormalWorldLoader,
    NormalWorldDtb,
    NormalWorldKernel,
    RootFs,
    Services,
    Applications,
    FwConfig,
    BootManifest,
    Nwfs,
    TrustedKeystore,
    Policy,
    CustomerBoardConfig,
    UpdateCertStore,
    BaseSystemUpdateManifest,
    FirmwareUpdateManifest,
    CustomerUpdateManifest,
    RecoveryManifest,
    ManifestSet,
    Other,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ImageType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ImageType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ImageType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::InvalidImageType => serializer.serialize_unit_variant("ImageType", 0u32, "InvalidImageType"),
            Self::OneBl => serializer.serialize_unit_variant("ImageType", 1u32, "OneBl"),
            Self::PlutonRuntime => serializer.serialize_unit_variant("ImageType", 2u32, "PlutonRuntime"),
            Self::WifiFirmware => serializer.serialize_unit_variant("ImageType", 3u32, "WifiFirmware"),
            Self::SecurityMonitor => serializer.serialize_unit_variant("ImageType", 4u32, "SecurityMonitor"),
            Self::NormalWorldLoader => serializer.serialize_unit_variant("ImageType", 5u32, "NormalWorldLoader"),
            Self::NormalWorldDtb => serializer.serialize_unit_variant("ImageType", 6u32, "NormalWorldDtb"),
            Self::NormalWorldKernel => serializer.serialize_unit_variant("ImageType", 7u32, "NormalWorldKernel"),
            Self::RootFs => serializer.serialize_unit_variant("ImageType", 8u32, "RootFs"),
            Self::Services => serializer.serialize_unit_variant("ImageType", 9u32, "Services"),
            Self::Applications => serializer.serialize_unit_variant("ImageType", 10u32, "Applications"),
            Self::FwConfig => serializer.serialize_unit_variant("ImageType", 11u32, "FwConfig"),
            Self::BootManifest => serializer.serialize_unit_variant("ImageType", 12u32, "BootManifest"),
            Self::Nwfs => serializer.serialize_unit_variant("ImageType", 13u32, "Nwfs"),
            Self::TrustedKeystore => serializer.serialize_unit_variant("ImageType", 14u32, "TrustedKeystore"),
            Self::Policy => serializer.serialize_unit_variant("ImageType", 15u32, "Policy"),
            Self::CustomerBoardConfig => serializer.serialize_unit_variant("ImageType", 16u32, "CustomerBoardConfig"),
            Self::UpdateCertStore => serializer.serialize_unit_variant("ImageType", 17u32, "UpdateCertStore"),
            Self::BaseSystemUpdateManifest => serializer.serialize_unit_variant("ImageType", 18u32, "BaseSystemUpdateManifest"),
            Self::FirmwareUpdateManifest => serializer.serialize_unit_variant("ImageType", 19u32, "FirmwareUpdateManifest"),
            Self::CustomerUpdateManifest => serializer.serialize_unit_variant("ImageType", 20u32, "CustomerUpdateManifest"),
            Self::RecoveryManifest => serializer.serialize_unit_variant("ImageType", 21u32, "RecoveryManifest"),
            Self::ManifestSet => serializer.serialize_unit_variant("ImageType", 22u32, "ManifestSet"),
            Self::Other => serializer.serialize_unit_variant("ImageType", 23u32, "Other"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Image upload request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageUploadRequestBody {
    #[doc = "."]
    pub images: String,
}
impl ImageUploadRequestBody {
    pub fn new(images: String) -> Self {
        Self { images }
    }
}
#[doc = "Request of the action to list device groups for a catalog."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListDeviceGroupsRequest {
    #[doc = "Device Group name."]
    #[serde(rename = "deviceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub device_group_name: Option<String>,
}
impl ListDeviceGroupsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for paginated APIs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListQueryParameters {}
impl ListQueryParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "OS feed type values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OsFeedType")]
pub enum OsFeedType {
    Retail,
    RetailEval,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OsFeedType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OsFeedType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OsFeedType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Retail => serializer.serialize_unit_variant("OsFeedType", 0u32, "Retail"),
            Self::RetailEval => serializer.serialize_unit_variant("OsFeedType", 1u32, "RetailEval"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged collection of DeviceInsight items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedDeviceInsight {
    #[doc = "The DeviceInsight items on this page"]
    pub value: Vec<DeviceInsight>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedDeviceInsight {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PagedDeviceInsight {
    pub fn new(value: Vec<DeviceInsight>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "An product resource belonging to a catalog resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Product {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of product"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductProperties>,
}
impl Product {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a Product list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductListResult {
    #[doc = "The Product items on this page"]
    pub value: Vec<Product>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProductListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProductListResult {
    pub fn new(value: Vec<Product>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of product"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductProperties {
    #[doc = "Description of the product"]
    pub description: String,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ProductProperties {
    pub fn new(description: String) -> Self {
        Self {
            description,
            provisioning_state: None,
        }
    }
}
#[doc = "The type used for update operations of the Product."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductUpdate {
    #[doc = "The updatable properties of the Product."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductUpdateProperties>,
}
impl ProductUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the Product."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductUpdateProperties {
    #[doc = "Description of the product"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ProductUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request for the proof of possession nonce"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProofOfPossessionNonceRequest {
    #[doc = "The proof of possession nonce"]
    #[serde(rename = "proofOfPossessionNonce")]
    pub proof_of_possession_nonce: String,
}
impl ProofOfPossessionNonceRequest {
    pub fn new(proof_of_possession_nonce: String) -> Self {
        Self { proof_of_possession_nonce }
    }
}
#[doc = "Result of the action to generate a proof of possession nonce"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProofOfPossessionNonceResponse {
    #[serde(flatten)]
    pub certificate_properties: CertificateProperties,
}
impl ProofOfPossessionNonceResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
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
#[doc = "Regional data boundary values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RegionalDataBoundary")]
pub enum RegionalDataBoundary {
    None,
    #[serde(rename = "EU")]
    Eu,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RegionalDataBoundary {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RegionalDataBoundary {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RegionalDataBoundary {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("RegionalDataBoundary", 0u32, "None"),
            Self::Eu => serializer.serialize_unit_variant("RegionalDataBoundary", 1u32, "EU"),
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
#[doc = "Signed device capability image response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignedCapabilityImageResponse {
    #[doc = "The signed device capability image as a UTF-8 encoded base 64 string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}
impl SignedCapabilityImageResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provides the most common query parameters for list operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StandardListQueryParametersWithCorrectNames {}
impl StandardListQueryParametersWithCorrectNames {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Update policy values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UpdatePolicy")]
pub enum UpdatePolicy {
    UpdateAll,
    No3rdPartyAppUpdates,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UpdatePolicy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UpdatePolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UpdatePolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UpdateAll => serializer.serialize_unit_variant("UpdatePolicy", 0u32, "UpdateAll"),
            Self::No3rdPartyAppUpdates => serializer.serialize_unit_variant("UpdatePolicy", 1u32, "No3rdPartyAppUpdates"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
