#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Contains metadata of a diagnostic type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeDiagnosticBase {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Contains additional properties of a diagnostic"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiagnosticProperties>,
}
impl ComputeDiagnosticBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lists all available Compute diagnostics for a subscription in a location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeDiagnosticsList {
    #[doc = "The collection of available Compute diagnostics returned by the listing operation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ComputeDiagnosticBase>,
    #[doc = "The continuation token."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ComputeDiagnosticsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ComputeDiagnosticsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Api output result when Compute Diagnostic operation is completed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeDiagnosticsOperationResult {
    #[doc = "The result of the disk inspection operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The response fields of the disk inspection operation."]
    #[serde(rename = "responseFields", default, skip_serializing_if = "Option::is_none")]
    pub response_fields: Option<String>,
    #[doc = "Result status of the async operation."]
    #[serde(rename = "resultStatus", default, skip_serializing_if = "Option::is_none")]
    pub result_status: Option<compute_diagnostics_operation_result::ResultStatus>,
    #[doc = "The error detail."]
    #[serde(rename = "errorDetail", default, skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    #[doc = "The time when the disk inspection was completed."]
    #[serde(rename = "createdUTC", default, with = "azure_core::date::rfc3339::option")]
    pub created_utc: Option<::time::OffsetDateTime>,
}
impl ComputeDiagnosticsOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod compute_diagnostics_operation_result {
    use super::*;
    #[doc = "Result status of the async operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResultStatus")]
    pub enum ResultStatus {
        Success,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResultStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResultStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResultStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Success => serializer.serialize_unit_variant("ResultStatus", 0u32, "Success"),
                Self::Failed => serializer.serialize_unit_variant("ResultStatus", 1u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Contains additional properties of a diagnostic"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticProperties {
    #[doc = "Describes what are the supported resource types for a diagnostic."]
    #[serde(
        rename = "supportedResourceTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_resource_types: Vec<String>,
}
impl DiagnosticProperties {
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
#[doc = "An error response from the Compute Diagnostic Resource Provider service."]
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
#[doc = "Inner error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerError {
    #[doc = "The exception type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exceptiontype: Option<String>,
    #[doc = "The internal error message or exception dump."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errordetail: Option<String>,
}
impl InnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The spot placement score for sku/region/zone combination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlacementScore {
    #[doc = "The resource's CRP virtual machine SKU size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "The region."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "The availability region."]
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc = "The placement score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    #[doc = "Whether the desired quota is available."]
    #[serde(rename = "isQuotaAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_quota_available: Option<bool>,
}
impl PlacementScore {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "Fully qualified resource ID for the resource. E.g. \"/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}\""]
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
#[doc = "SpotPlacementRecommender API response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSize {
    #[doc = "The resource's CRP virtual machine SKU size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
}
impl ResourceSize {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data used for requesting a Disk Inspection execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunDiskInspectionInput {
    #[doc = "Qualified ID of the resource."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "Name of manifest in order to trigger Disk Inspection."]
    pub manifest: String,
    #[doc = "SAS uri to the blob where results will be uploaded."]
    #[serde(rename = "uploadSasUri")]
    pub upload_sas_uri: String,
}
impl RunDiskInspectionInput {
    pub fn new(resource_id: String, manifest: String, upload_sas_uri: String) -> Self {
        Self {
            resource_id,
            manifest,
            upload_sas_uri,
        }
    }
}
#[doc = "SpotPlacementRecommender API Input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpotPlacementRecommenderInput {
    #[doc = "The desired regions"]
    #[serde(
        rename = "desiredLocations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub desired_locations: Vec<String>,
    #[doc = "The desired resource SKUs."]
    #[serde(
        rename = "desiredSizes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub desired_sizes: Vec<ResourceSize>,
    #[doc = "Desired instance count per region/zone based on the scope."]
    #[serde(rename = "desiredCount", default, skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i32>,
    #[doc = "Defines if the scope is zonal or regional."]
    #[serde(rename = "availabilityZones", default, skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<bool>,
}
impl SpotPlacementRecommenderInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SpotPlacementRecommender API response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpotPlacementRecommenderResponse {
    #[doc = "The desired regions"]
    #[serde(
        rename = "desiredLocations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub desired_locations: Vec<String>,
    #[doc = "The desired resource SKUs."]
    #[serde(
        rename = "desiredSizes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub desired_sizes: Vec<ResourceSize>,
    #[doc = "Desired instance count per region/zone based on the scope."]
    #[serde(rename = "desiredCount", default, skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i32>,
    #[doc = "Defines if the scope is zonal or regional."]
    #[serde(rename = "availabilityZones", default, skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<bool>,
    #[doc = "The spot placement scores."]
    #[serde(
        rename = "placementScores",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub placement_scores: Vec<PlacementScore>,
}
impl SpotPlacementRecommenderResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data used for registering a Storage Account for a Subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageConfigurationInput {
    #[doc = "Fully qualified storage account Id. Example: \"/subscriptions/{subId}/resourceGroups/{rgName}/providers/Microsoft.Storage/storageAccounts/{storageAccountName}\""]
    #[serde(rename = "storageAccountId")]
    pub storage_account_id: String,
}
impl StorageConfigurationInput {
    pub fn new(storage_account_id: String) -> Self {
        Self { storage_account_id }
    }
}
#[doc = "Api output result when there is an existing storage configuration entry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageConfigurationResponse {
    #[doc = "Fully qualified storage account Id. Example: \"/subscriptions/{subId}/resourceGroups/{rgName}/providers/Microsoft.Storage/storageAccounts/{storageAccountName}\""]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
}
impl StorageConfigurationResponse {
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
    pub created_at: Option<::time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<::time::OffsetDateTime>,
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
