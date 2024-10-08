#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A class represent the compliance category."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Category {
    #[doc = "The name of the compliance category. e.g. \"Operational Security\""]
    #[serde(rename = "categoryName", default, skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    #[doc = "Indicates the category status."]
    #[serde(rename = "categoryStatus", default, skip_serializing_if = "Option::is_none")]
    pub category_status: Option<CategoryStatus>,
    #[doc = "List of control families."]
    #[serde(
        rename = "controlFamilies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub control_families: Vec<ControlFamily>,
}
impl Category {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates the category status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CategoryStatus")]
pub enum CategoryStatus {
    Passed,
    Failed,
    NotApplicable,
    PendingApproval,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CategoryStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CategoryStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CategoryStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Passed => serializer.serialize_unit_variant("CategoryStatus", 0u32, "Passed"),
            Self::Failed => serializer.serialize_unit_variant("CategoryStatus", 1u32, "Failed"),
            Self::NotApplicable => serializer.serialize_unit_variant("CategoryStatus", 2u32, "NotApplicable"),
            Self::PendingApproval => serializer.serialize_unit_variant("CategoryStatus", 3u32, "PendingApproval"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A class represent the certification record synchronized from app compliance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertSyncRecord {
    #[doc = "The offerGuid which mapping to the reports."]
    #[serde(rename = "offerGuid", default, skip_serializing_if = "Option::is_none")]
    pub offer_guid: Option<String>,
    #[doc = "Indicates the status of certification process."]
    #[serde(rename = "certificationStatus", default, skip_serializing_if = "Option::is_none")]
    pub certification_status: Option<String>,
    #[doc = "Indicates the status of compliance process."]
    #[serde(rename = "ingestionStatus", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_status: Option<String>,
    #[doc = "The control records list to be synchronized."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub controls: Vec<ControlSyncRecord>,
}
impl CertSyncRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The check availability request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityRequest {
    #[doc = "The name of the resource for which availability needs to be checked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CheckNameAvailabilityRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The check availability result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResponse {
    #[doc = "Indicates if the resource name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason why the given name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_response::Reason>,
    #[doc = "Detailed reason why the given name is available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_response {
    use super::*;
    #[doc = "The reason why the given name is not available."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Reason")]
    pub enum Reason {
        Invalid,
        AlreadyExists,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Reason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Reason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Reason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("Reason", 0u32, "Invalid"),
                Self::AlreadyExists => serializer.serialize_unit_variant("Reason", 1u32, "AlreadyExists"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Object that includes all the content for single compliance result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComplianceReportItem {
    #[doc = "The category name."]
    #[serde(rename = "categoryName", default, skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    #[doc = "The control family name."]
    #[serde(rename = "controlFamilyName", default, skip_serializing_if = "Option::is_none")]
    pub control_family_name: Option<String>,
    #[doc = "The control Id - e.g. \"1\"."]
    #[serde(rename = "controlId", default, skip_serializing_if = "Option::is_none")]
    pub control_id: Option<String>,
    #[doc = "The control name."]
    #[serde(rename = "controlName", default, skip_serializing_if = "Option::is_none")]
    pub control_name: Option<String>,
    #[doc = "Indicates the control status."]
    #[serde(rename = "controlStatus", default, skip_serializing_if = "Option::is_none")]
    pub control_status: Option<ControlStatus>,
    #[doc = "The title of the customer responsibility."]
    #[serde(rename = "responsibilityTitle", default, skip_serializing_if = "Option::is_none")]
    pub responsibility_title: Option<String>,
    #[doc = "The description of the customer responsibility."]
    #[serde(rename = "responsibilityDescription", default, skip_serializing_if = "Option::is_none")]
    pub responsibility_description: Option<String>,
    #[doc = "The Id of the resource."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The type of the resource.  e.g. \"Microsoft.SignalRService/SignalR\""]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Resource Origin."]
    #[serde(rename = "resourceOrigin", default, skip_serializing_if = "Option::is_none")]
    pub resource_origin: Option<ResourceOrigin>,
    #[doc = "Indicates the resource status."]
    #[serde(rename = "resourceStatus", default, skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<ResourceStatus>,
    #[doc = "The status change date for the resource."]
    #[serde(rename = "resourceStatusChangeDate", default, with = "azure_core::date::rfc3339::option")]
    pub resource_status_change_date: Option<::time::OffsetDateTime>,
}
impl ComplianceReportItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class represent the compliance result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComplianceResult {
    #[doc = "The name of the compliance. e.g. \"M365\""]
    #[serde(rename = "complianceName", default, skip_serializing_if = "Option::is_none")]
    pub compliance_name: Option<String>,
    #[doc = "List of categories."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub categories: Vec<Category>,
}
impl ComplianceResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "content type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ContentType")]
pub enum ContentType {
    #[serde(rename = "application/json")]
    ApplicationJson,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ContentType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ContentType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ContentType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ApplicationJson => serializer.serialize_unit_variant("ContentType", 0u32, "application/json"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A class represent the control."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Control {
    #[doc = "The Id of the control. e.g. \"Operational_Security_10\""]
    #[serde(rename = "controlId", default, skip_serializing_if = "Option::is_none")]
    pub control_id: Option<String>,
    #[doc = "The name of the control. e.g. \"Unsupported OS and Software.\""]
    #[serde(rename = "controlName", default, skip_serializing_if = "Option::is_none")]
    pub control_name: Option<String>,
    #[doc = "The full name of the control. e.g. \"Validate that unsupported operating systems and software components are not in use.\""]
    #[serde(rename = "controlFullName", default, skip_serializing_if = "Option::is_none")]
    pub control_full_name: Option<String>,
    #[doc = "The control's description"]
    #[serde(rename = "controlDescription", default, skip_serializing_if = "Option::is_none")]
    pub control_description: Option<String>,
    #[doc = "The hyper link to the control's description'."]
    #[serde(rename = "controlDescriptionHyperLink", default, skip_serializing_if = "Option::is_none")]
    pub control_description_hyper_link: Option<String>,
    #[doc = "Indicates the control status."]
    #[serde(rename = "controlStatus", default, skip_serializing_if = "Option::is_none")]
    pub control_status: Option<ControlStatus>,
    #[doc = "List of customer responsibility."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub responsibilities: Vec<Responsibility>,
}
impl Control {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class represent the control family."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ControlFamily {
    #[doc = "The name of the control family. e.g. \"Malware Protection - Anti-Virus\""]
    #[serde(rename = "controlFamilyName", default, skip_serializing_if = "Option::is_none")]
    pub control_family_name: Option<String>,
    #[doc = "Indicates the control family status."]
    #[serde(rename = "controlFamilyStatus", default, skip_serializing_if = "Option::is_none")]
    pub control_family_status: Option<ControlFamilyStatus>,
    #[doc = "List of controls."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub controls: Vec<Control>,
}
impl ControlFamily {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates the control family status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ControlFamilyStatus")]
pub enum ControlFamilyStatus {
    Passed,
    Failed,
    NotApplicable,
    PendingApproval,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ControlFamilyStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ControlFamilyStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ControlFamilyStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Passed => serializer.serialize_unit_variant("ControlFamilyStatus", 0u32, "Passed"),
            Self::Failed => serializer.serialize_unit_variant("ControlFamilyStatus", 1u32, "Failed"),
            Self::NotApplicable => serializer.serialize_unit_variant("ControlFamilyStatus", 2u32, "NotApplicable"),
            Self::PendingApproval => serializer.serialize_unit_variant("ControlFamilyStatus", 3u32, "PendingApproval"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates the control status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ControlStatus")]
pub enum ControlStatus {
    Passed,
    Failed,
    NotApplicable,
    PendingApproval,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ControlStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ControlStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ControlStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Passed => serializer.serialize_unit_variant("ControlStatus", 0u32, "Passed"),
            Self::Failed => serializer.serialize_unit_variant("ControlStatus", 1u32, "Failed"),
            Self::NotApplicable => serializer.serialize_unit_variant("ControlStatus", 2u32, "NotApplicable"),
            Self::PendingApproval => serializer.serialize_unit_variant("ControlStatus", 3u32, "PendingApproval"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A class represent the control record synchronized from app compliance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ControlSyncRecord {
    #[doc = "The Id of the control. e.g. \"Operational_Security_10\""]
    #[serde(rename = "controlId", default, skip_serializing_if = "Option::is_none")]
    pub control_id: Option<String>,
    #[doc = "Control status synchronized from app compliance."]
    #[serde(rename = "controlStatus", default, skip_serializing_if = "Option::is_none")]
    pub control_status: Option<String>,
}
impl ControlSyncRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "webhook deliveryStatus"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeliveryStatus")]
pub enum DeliveryStatus {
    Succeeded,
    Failed,
    NotStarted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeliveryStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeliveryStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeliveryStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("DeliveryStatus", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("DeliveryStatus", 1u32, "Failed"),
            Self::NotStarted => serializer.serialize_unit_variant("DeliveryStatus", 2u32, "NotStarted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Object that includes all the possible response for the download operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DownloadResponse {
    #[doc = "Resource list of the report"]
    #[serde(
        rename = "resourceList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resource_list: Vec<ResourceItem>,
    #[doc = "List of the compliance result"]
    #[serde(
        rename = "complianceReport",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub compliance_report: Vec<ComplianceReportItem>,
    #[doc = "Compliance pdf report"]
    #[serde(rename = "compliancePdfReport", default, skip_serializing_if = "Option::is_none")]
    pub compliance_pdf_report: Option<DownloadResponseCompliancePdfReport>,
    #[doc = "The detailed compliance pdf report"]
    #[serde(rename = "complianceDetailedPdfReport", default, skip_serializing_if = "Option::is_none")]
    pub compliance_detailed_pdf_report: Option<DownloadResponseComplianceDetailedPdfReport>,
}
impl DownloadResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The detailed compliance pdf report"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DownloadResponseComplianceDetailedPdfReport {
    #[doc = "The uri of detailed compliance pdf report"]
    #[serde(rename = "sasUri", default, skip_serializing_if = "Option::is_none")]
    pub sas_uri: Option<String>,
}
impl DownloadResponseComplianceDetailedPdfReport {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Compliance pdf report"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DownloadResponseCompliancePdfReport {
    #[doc = "The uri of compliance pdf report"]
    #[serde(rename = "sasUri", default, skip_serializing_if = "Option::is_none")]
    pub sas_uri: Option<String>,
}
impl DownloadResponseCompliancePdfReport {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates the download type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DownloadType")]
pub enum DownloadType {
    ComplianceReport,
    CompliancePdfReport,
    ComplianceDetailedPdfReport,
    ResourceList,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DownloadType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DownloadType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DownloadType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ComplianceReport => serializer.serialize_unit_variant("DownloadType", 0u32, "ComplianceReport"),
            Self::CompliancePdfReport => serializer.serialize_unit_variant("DownloadType", 1u32, "CompliancePdfReport"),
            Self::ComplianceDetailedPdfReport => serializer.serialize_unit_variant("DownloadType", 2u32, "ComplianceDetailedPdfReport"),
            Self::ResourceList => serializer.serialize_unit_variant("DownloadType", 3u32, "ResourceList"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "whether to enable ssl verification"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnableSslVerification")]
pub enum EnableSslVerification {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnableSslVerification {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnableSslVerification {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnableSslVerification {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::True => serializer.serialize_unit_variant("EnableSslVerification", 0u32, "true"),
            Self::False => serializer.serialize_unit_variant("EnableSslVerification", 1u32, "false"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
#[doc = "Evidence file's download request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EvidenceFileDownloadRequest {
    #[doc = "Tenant id."]
    #[serde(rename = "reportCreatorTenantId", default, skip_serializing_if = "Option::is_none")]
    pub report_creator_tenant_id: Option<String>,
    #[doc = "The offerGuid which mapping to the reports."]
    #[serde(rename = "offerGuid", default, skip_serializing_if = "Option::is_none")]
    pub offer_guid: Option<String>,
}
impl EvidenceFileDownloadRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes all the possible response for the evidence file download operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EvidenceFileDownloadResponse {
    #[doc = "The uri of evidence file"]
    #[serde(rename = "evidenceFile", default, skip_serializing_if = "Option::is_none")]
    pub evidence_file: Option<EvidenceFileDownloadResponseEvidenceFile>,
}
impl EvidenceFileDownloadResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The uri of evidence file"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EvidenceFileDownloadResponseEvidenceFile {
    #[doc = "The url of evidence file"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl EvidenceFileDownloadResponseEvidenceFile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Evidence's properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvidenceProperties {
    #[doc = "Evidence type"]
    #[serde(rename = "evidenceType", default, skip_serializing_if = "Option::is_none")]
    pub evidence_type: Option<EvidenceType>,
    #[doc = "The path of the file in storage."]
    #[serde(rename = "filePath")]
    pub file_path: String,
    #[doc = "Extra data considered as evidence."]
    #[serde(rename = "extraData", default, skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<String>,
    #[doc = "Control id."]
    #[serde(rename = "controlId", default, skip_serializing_if = "Option::is_none")]
    pub control_id: Option<String>,
    #[doc = "Responsibility id."]
    #[serde(rename = "responsibilityId", default, skip_serializing_if = "Option::is_none")]
    pub responsibility_id: Option<String>,
    #[doc = "Resource provisioning states."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl EvidenceProperties {
    pub fn new(file_path: String) -> Self {
        Self {
            evidence_type: None,
            file_path,
            extra_data: None,
            control_id: None,
            responsibility_id: None,
            provisioning_state: None,
        }
    }
}
#[doc = "A class represent an AppComplianceAutomation evidence resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvidenceResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Evidence's properties."]
    pub properties: EvidenceProperties,
}
impl EvidenceResource {
    pub fn new(properties: EvidenceProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "The response of a EvidenceResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvidenceResourceListResult {
    #[doc = "The EvidenceResource items on this page"]
    pub value: Vec<EvidenceResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EvidenceResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EvidenceResourceListResult {
    pub fn new(value: Vec<EvidenceResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Evidence type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EvidenceType")]
pub enum EvidenceType {
    File,
    AutoCollectedEvidence,
    Data,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EvidenceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EvidenceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EvidenceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::File => serializer.serialize_unit_variant("EvidenceType", 0u32, "File"),
            Self::AutoCollectedEvidence => serializer.serialize_unit_variant("EvidenceType", 1u32, "AutoCollectedEvidence"),
            Self::Data => serializer.serialize_unit_variant("EvidenceType", 2u32, "Data"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Get collection count's request object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetCollectionCountRequest {
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl GetCollectionCountRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The get collection count response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetCollectionCountResponse {
    #[doc = "The count of the specified resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
impl GetCollectionCountResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Get overview status request object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetOverviewStatusRequest {
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl GetOverviewStatusRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The get overview status response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetOverviewStatusResponse {
    #[doc = "List of different status items."]
    #[serde(
        rename = "statusList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub status_list: Vec<StatusItem>,
}
impl GetOverviewStatusResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Question input type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InputType")]
pub enum InputType {
    None,
    Text,
    Email,
    MultilineText,
    Url,
    Number,
    Boolean,
    Telephone,
    YesNoNa,
    Date,
    YearPicker,
    SingleSelection,
    SingleSelectDropdown,
    MultiSelectCheckbox,
    MultiSelectDropdown,
    MultiSelectDropdownCustom,
    Group,
    Upload,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InputType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InputType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InputType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("InputType", 0u32, "None"),
            Self::Text => serializer.serialize_unit_variant("InputType", 1u32, "Text"),
            Self::Email => serializer.serialize_unit_variant("InputType", 2u32, "Email"),
            Self::MultilineText => serializer.serialize_unit_variant("InputType", 3u32, "MultilineText"),
            Self::Url => serializer.serialize_unit_variant("InputType", 4u32, "Url"),
            Self::Number => serializer.serialize_unit_variant("InputType", 5u32, "Number"),
            Self::Boolean => serializer.serialize_unit_variant("InputType", 6u32, "Boolean"),
            Self::Telephone => serializer.serialize_unit_variant("InputType", 7u32, "Telephone"),
            Self::YesNoNa => serializer.serialize_unit_variant("InputType", 8u32, "YesNoNa"),
            Self::Date => serializer.serialize_unit_variant("InputType", 9u32, "Date"),
            Self::YearPicker => serializer.serialize_unit_variant("InputType", 10u32, "YearPicker"),
            Self::SingleSelection => serializer.serialize_unit_variant("InputType", 11u32, "SingleSelection"),
            Self::SingleSelectDropdown => serializer.serialize_unit_variant("InputType", 12u32, "SingleSelectDropdown"),
            Self::MultiSelectCheckbox => serializer.serialize_unit_variant("InputType", 13u32, "MultiSelectCheckbox"),
            Self::MultiSelectDropdown => serializer.serialize_unit_variant("InputType", 14u32, "MultiSelectDropdown"),
            Self::MultiSelectDropdownCustom => serializer.serialize_unit_variant("InputType", 15u32, "MultiSelectDropdownCustom"),
            Self::Group => serializer.serialize_unit_variant("InputType", 16u32, "Group"),
            Self::Upload => serializer.serialize_unit_variant("InputType", 17u32, "Upload"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates whether this solution is the recommended."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IsRecommendSolution")]
pub enum IsRecommendSolution {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IsRecommendSolution {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IsRecommendSolution {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IsRecommendSolution {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::True => serializer.serialize_unit_variant("IsRecommendSolution", 0u32, "true"),
            Self::False => serializer.serialize_unit_variant("IsRecommendSolution", 1u32, "false"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Parameters for listing in use storage accounts operation. If subscription list is null, it will check the user's all subscriptions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListInUseStorageAccountsRequest {
    #[doc = "List of subscription ids to be query. If the list is null or empty, the API will query all the subscriptions of the user."]
    #[serde(
        rename = "subscriptionIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subscription_ids: Vec<String>,
}
impl ListInUseStorageAccountsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for listing in use storage accounts operation. If subscription list is null, it will check the user's all subscriptions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListInUseStorageAccountsResponse {
    #[doc = "The storage account list which in use in related reports."]
    #[serde(
        rename = "storageAccountList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub storage_account_list: Vec<StorageInfo>,
}
impl ListInUseStorageAccountsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "notification event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NotificationEvent")]
pub enum NotificationEvent {
    #[serde(rename = "generate_snapshot_success")]
    GenerateSnapshotSuccess,
    #[serde(rename = "generate_snapshot_failed")]
    GenerateSnapshotFailed,
    #[serde(rename = "assessment_failure")]
    AssessmentFailure,
    #[serde(rename = "report_configuration_changes")]
    ReportConfigurationChanges,
    #[serde(rename = "report_deletion")]
    ReportDeletion,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NotificationEvent {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NotificationEvent {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NotificationEvent {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::GenerateSnapshotSuccess => serializer.serialize_unit_variant("NotificationEvent", 0u32, "generate_snapshot_success"),
            Self::GenerateSnapshotFailed => serializer.serialize_unit_variant("NotificationEvent", 1u32, "generate_snapshot_failed"),
            Self::AssessmentFailure => serializer.serialize_unit_variant("NotificationEvent", 2u32, "assessment_failure"),
            Self::ReportConfigurationChanges => {
                serializer.serialize_unit_variant("NotificationEvent", 3u32, "report_configuration_changes")
            }
            Self::ReportDeletion => serializer.serialize_unit_variant("NotificationEvent", 4u32, "report_deletion"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Parameters for onboard operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnboardRequest {
    #[doc = "List of subscription ids to be onboarded"]
    #[serde(rename = "subscriptionIds")]
    pub subscription_ids: Vec<String>,
}
impl OnboardRequest {
    pub fn new(subscription_ids: Vec<String>) -> Self {
        Self { subscription_ids }
    }
}
#[doc = "Success. The response indicates given subscriptions has been onboarded."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OnboardResponse {
    #[doc = "List of subscription ids that are onboarded"]
    #[serde(
        rename = "subscriptionIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subscription_ids: Vec<String>,
}
impl OnboardResponse {
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
#[doc = "The overview of the compliance result for one report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OverviewStatus {
    #[doc = "The count of all passed control."]
    #[serde(rename = "passedCount", default, skip_serializing_if = "Option::is_none")]
    pub passed_count: Option<i32>,
    #[doc = "The count of all failed control."]
    #[serde(rename = "failedCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i32>,
    #[doc = "The count of all manual control."]
    #[serde(rename = "manualCount", default, skip_serializing_if = "Option::is_none")]
    pub manual_count: Option<i32>,
    #[doc = "The count of all not applicable control."]
    #[serde(rename = "notApplicableCount", default, skip_serializing_if = "Option::is_none")]
    pub not_applicable_count: Option<i32>,
    #[doc = "The count of all pending for approval control."]
    #[serde(rename = "pendingCount", default, skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i32>,
}
impl OverviewStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource provisioning states."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Creating,
    Deleting,
    Fixing,
    Verifying,
    Updating,
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
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
            Self::Fixing => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Fixing"),
            Self::Verifying => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Verifying"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Updating"),
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
#[doc = "A class represent the quick assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuickAssessment {
    #[doc = "Resource id."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Responsibility id."]
    #[serde(rename = "responsibilityId", default, skip_serializing_if = "Option::is_none")]
    pub responsibility_id: Option<String>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<::time::OffsetDateTime>,
    #[doc = "Indicates the resource status."]
    #[serde(rename = "resourceStatus", default, skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<ResourceStatus>,
    #[doc = "Quick assessment display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Quick assessment display name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Link to remediation steps for this quick assessment."]
    #[serde(rename = "remediationLink", default, skip_serializing_if = "Option::is_none")]
    pub remediation_link: Option<String>,
}
impl QuickAssessment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class represent the recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Recommendation {
    #[doc = "The Id of the recommendation."]
    #[serde(rename = "recommendationId", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
    #[doc = "The short name of the recommendation. e.g. \"Invalid TLS config\""]
    #[serde(rename = "recommendationShortName", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_short_name: Option<String>,
    #[doc = "List of recommendation solutions."]
    #[serde(
        rename = "recommendationSolutions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub recommendation_solutions: Vec<RecommendationSolution>,
}
impl Recommendation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class represent the recommendation solution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendationSolution {
    #[doc = "The index of the recommendation solution."]
    #[serde(rename = "recommendationSolutionIndex", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_solution_index: Option<String>,
    #[doc = "The detail steps of the recommendation solution."]
    #[serde(rename = "recommendationSolutionContent", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_solution_content: Option<String>,
    #[doc = "Indicates whether this solution is the recommended."]
    #[serde(rename = "isRecommendSolution", default, skip_serializing_if = "Option::is_none")]
    pub is_recommend_solution: Option<IsRecommendSolution>,
}
impl RecommendationSolution {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list which includes all the compliance result for one report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportComplianceStatus {
    #[doc = "The overview of the compliance result for one report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub m365: Option<OverviewStatus>,
}
impl ReportComplianceStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Report fix result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportFixResult {
    #[doc = "Indicates whether the fix action is Succeeded or Failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Result>,
    #[doc = "If the report fix action failed, to indicate the detailed failed reason."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl ReportFixResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Patch Report's properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportPatchProperties {
    #[doc = "Report collection trigger time."]
    #[serde(rename = "triggerTime", default, with = "azure_core::date::rfc3339::option")]
    pub trigger_time: Option<::time::OffsetDateTime>,
    #[doc = "Report collection trigger time's time zone, the available list can be obtained by executing \"Get-TimeZone -ListAvailable\" in PowerShell.\nAn example of valid timezone id is \"Pacific Standard Time\"."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "List of resource data."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resources: Vec<ResourceMetadata>,
    #[doc = "Report status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReportStatus>,
    #[doc = "List of report error codes."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<String>,
    #[doc = "Report's tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "A list of comma-separated offerGuids indicates a series of offerGuids that map to the report. For example, \"00000000-0000-0000-0000-000000000001,00000000-0000-0000-0000-000000000002\" and \"00000000-0000-0000-0000-000000000003\"."]
    #[serde(rename = "offerGuid", default, skip_serializing_if = "Option::is_none")]
    pub offer_guid: Option<String>,
    #[doc = "Report next collection trigger time."]
    #[serde(rename = "nextTriggerTime", default, with = "azure_core::date::rfc3339::option")]
    pub next_trigger_time: Option<::time::OffsetDateTime>,
    #[doc = "Report last collection trigger time."]
    #[serde(rename = "lastTriggerTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_trigger_time: Option<::time::OffsetDateTime>,
    #[doc = "List of subscription Ids."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subscriptions: Vec<String>,
    #[doc = "A list which includes all the compliance result for one report."]
    #[serde(rename = "complianceStatus", default, skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<ReportComplianceStatus>,
    #[doc = "The information of 'bring your own storage' account binding to the report"]
    #[serde(rename = "storageInfo", default, skip_serializing_if = "Option::is_none")]
    pub storage_info: Option<StorageInfo>,
    #[doc = "List of synchronized certification records."]
    #[serde(
        rename = "certRecords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cert_records: Vec<CertSyncRecord>,
    #[doc = "Resource provisioning states."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ReportPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Create Report's properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportProperties {
    #[doc = "Report collection trigger time."]
    #[serde(rename = "triggerTime", with = "azure_core::date::rfc3339")]
    pub trigger_time: ::time::OffsetDateTime,
    #[doc = "Report collection trigger time's time zone, the available list can be obtained by executing \"Get-TimeZone -ListAvailable\" in PowerShell.\nAn example of valid timezone id is \"Pacific Standard Time\"."]
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    #[doc = "List of resource data."]
    pub resources: Vec<ResourceMetadata>,
    #[doc = "Report status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReportStatus>,
    #[doc = "List of report error codes."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<String>,
    #[doc = "Report's tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "A list of comma-separated offerGuids indicates a series of offerGuids that map to the report. For example, \"00000000-0000-0000-0000-000000000001,00000000-0000-0000-0000-000000000002\" and \"00000000-0000-0000-0000-000000000003\"."]
    #[serde(rename = "offerGuid", default, skip_serializing_if = "Option::is_none")]
    pub offer_guid: Option<String>,
    #[doc = "Report next collection trigger time."]
    #[serde(rename = "nextTriggerTime", default, with = "azure_core::date::rfc3339::option")]
    pub next_trigger_time: Option<::time::OffsetDateTime>,
    #[doc = "Report last collection trigger time."]
    #[serde(rename = "lastTriggerTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_trigger_time: Option<::time::OffsetDateTime>,
    #[doc = "List of subscription Ids."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subscriptions: Vec<String>,
    #[doc = "A list which includes all the compliance result for one report."]
    #[serde(rename = "complianceStatus", default, skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<ReportComplianceStatus>,
    #[doc = "The information of 'bring your own storage' account binding to the report"]
    #[serde(rename = "storageInfo", default, skip_serializing_if = "Option::is_none")]
    pub storage_info: Option<StorageInfo>,
    #[doc = "List of synchronized certification records."]
    #[serde(
        rename = "certRecords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cert_records: Vec<CertSyncRecord>,
    #[doc = "Resource provisioning states."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ReportProperties {
    pub fn new(trigger_time: ::time::OffsetDateTime, time_zone: String, resources: Vec<ResourceMetadata>) -> Self {
        Self {
            trigger_time,
            time_zone,
            resources,
            status: None,
            errors: Vec::new(),
            tenant_id: None,
            offer_guid: None,
            next_trigger_time: None,
            last_trigger_time: None,
            subscriptions: Vec::new(),
            compliance_status: None,
            storage_info: None,
            cert_records: Vec::new(),
            provisioning_state: None,
        }
    }
}
#[doc = "A class represent an AppComplianceAutomation report resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Create Report's properties."]
    pub properties: ReportProperties,
}
impl ReportResource {
    pub fn new(properties: ReportProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "The response of a ReportResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportResourceListResult {
    #[doc = "The ReportResource items on this page"]
    pub value: Vec<ReportResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReportResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ReportResourceListResult {
    pub fn new(value: Vec<ReportResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A class represent a AppComplianceAutomation report resource update properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportResourcePatch {
    #[doc = "Patch Report's properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReportPatchProperties>,
}
impl ReportResourcePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Report status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReportStatus")]
pub enum ReportStatus {
    Active,
    Failed,
    Reviewing,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReportStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReportStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReportStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("ReportStatus", 0u32, "Active"),
            Self::Failed => serializer.serialize_unit_variant("ReportStatus", 1u32, "Failed"),
            Self::Reviewing => serializer.serialize_unit_variant("ReportStatus", 2u32, "Reviewing"),
            Self::Disabled => serializer.serialize_unit_variant("ReportStatus", 3u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Report health status verification result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportVerificationResult {
    #[doc = "Indicates whether the fix action is Succeeded or Failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Result>,
    #[doc = "If the report verification action failed, to indicate the detailed failed reason."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl ReportVerificationResult {
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
#[doc = "Resource Id."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceItem {
    #[doc = "The subscription Id of this resource."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The resource group name of this resource."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "The resource type of this resource. e.g. \"Microsoft.SignalRService/SignalR\""]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The resource Id - e.g. \"/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.Compute/virtualMachines/vm1\"."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl ResourceItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Single resource Id's metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceMetadata {
    #[doc = "Resource Id - e.g. \"/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.Compute/virtualMachines/vm1\"."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "Resource type. e.g. \"Microsoft.Compute/virtualMachines\""]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Resource kind."]
    #[serde(rename = "resourceKind", default, skip_serializing_if = "Option::is_none")]
    pub resource_kind: Option<String>,
    #[doc = "Resource Origin."]
    #[serde(rename = "resourceOrigin", default, skip_serializing_if = "Option::is_none")]
    pub resource_origin: Option<ResourceOrigin>,
    #[doc = "Account Id. For example - the AWS account id."]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}
impl ResourceMetadata {
    pub fn new(resource_id: String) -> Self {
        Self {
            resource_id,
            resource_type: None,
            resource_kind: None,
            resource_origin: None,
            account_id: None,
        }
    }
}
#[doc = "Resource Origin."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceOrigin")]
pub enum ResourceOrigin {
    Azure,
    #[serde(rename = "AWS")]
    Aws,
    #[serde(rename = "GCP")]
    Gcp,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceOrigin {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceOrigin {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceOrigin {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Azure => serializer.serialize_unit_variant("ResourceOrigin", 0u32, "Azure"),
            Self::Aws => serializer.serialize_unit_variant("ResourceOrigin", 1u32, "AWS"),
            Self::Gcp => serializer.serialize_unit_variant("ResourceOrigin", 2u32, "GCP"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates the resource status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceStatus")]
pub enum ResourceStatus {
    Healthy,
    Unhealthy,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Healthy => serializer.serialize_unit_variant("ResourceStatus", 0u32, "Healthy"),
            Self::Unhealthy => serializer.serialize_unit_variant("ResourceStatus", 1u32, "Unhealthy"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A class represent the customer responsibility."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Responsibility {
    #[doc = "The id of the customer responsibility."]
    #[serde(rename = "responsibilityId", default, skip_serializing_if = "Option::is_none")]
    pub responsibility_id: Option<String>,
    #[doc = "The title of the customer responsibility."]
    #[serde(rename = "responsibilityTitle", default, skip_serializing_if = "Option::is_none")]
    pub responsibility_title: Option<String>,
    #[doc = "The description of the customer responsibility."]
    #[serde(rename = "responsibilityDescription", default, skip_serializing_if = "Option::is_none")]
    pub responsibility_description: Option<String>,
    #[doc = "Indicates the customer responsibility type."]
    #[serde(rename = "responsibilityType", default, skip_serializing_if = "Option::is_none")]
    pub responsibility_type: Option<ResponsibilityType>,
    #[doc = "Indicates the customer responsibility severity."]
    #[serde(rename = "responsibilitySeverity", default, skip_serializing_if = "Option::is_none")]
    pub responsibility_severity: Option<ResponsibilitySeverity>,
    #[doc = "Indicates the customer responsibility status."]
    #[serde(rename = "responsibilityStatus", default, skip_serializing_if = "Option::is_none")]
    pub responsibility_status: Option<ResponsibilityStatus>,
    #[doc = "Indicates the customer responsibility supported cloud environment."]
    #[serde(rename = "responsibilityEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub responsibility_environment: Option<ResponsibilityEnvironment>,
    #[doc = "The count of all failed resources."]
    #[serde(rename = "failedResourceCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_resource_count: Option<i32>,
    #[doc = "The count of all resources."]
    #[serde(rename = "totalResourceCount", default, skip_serializing_if = "Option::is_none")]
    pub total_resource_count: Option<i32>,
    #[doc = "List of resource."]
    #[serde(
        rename = "resourceList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resource_list: Vec<ResponsibilityResource>,
    #[doc = "List of recommendation."]
    #[serde(
        rename = "recommendationList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub recommendation_list: Vec<Recommendation>,
    #[doc = "The evidence upload guidance description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guidance: Option<String>,
    #[doc = "The justification given by the user to clarify the reason."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    #[doc = "List of evidence file url."]
    #[serde(
        rename = "evidenceFiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub evidence_files: Vec<String>,
}
impl Responsibility {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates the customer responsibility supported cloud environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResponsibilityEnvironment")]
pub enum ResponsibilityEnvironment {
    Azure,
    #[serde(rename = "AWS")]
    Aws,
    #[serde(rename = "GCP")]
    Gcp,
    General,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResponsibilityEnvironment {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResponsibilityEnvironment {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResponsibilityEnvironment {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Azure => serializer.serialize_unit_variant("ResponsibilityEnvironment", 0u32, "Azure"),
            Self::Aws => serializer.serialize_unit_variant("ResponsibilityEnvironment", 1u32, "AWS"),
            Self::Gcp => serializer.serialize_unit_variant("ResponsibilityEnvironment", 2u32, "GCP"),
            Self::General => serializer.serialize_unit_variant("ResponsibilityEnvironment", 3u32, "General"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A class represent the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponsibilityResource {
    #[doc = "The Id of the resource."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Account Id. For example - AWS account Id."]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "The type of the resource. e.g. \"Microsoft.SignalRService/SignalR\""]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Resource Origin."]
    #[serde(rename = "resourceOrigin", default, skip_serializing_if = "Option::is_none")]
    pub resource_origin: Option<ResourceOrigin>,
    #[doc = "Indicates the resource status."]
    #[serde(rename = "resourceStatus", default, skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<ResourceStatus>,
    #[doc = "The status change date for the resource."]
    #[serde(rename = "resourceStatusChangeDate", default, with = "azure_core::date::rfc3339::option")]
    pub resource_status_change_date: Option<::time::OffsetDateTime>,
    #[doc = "List of recommendation id."]
    #[serde(
        rename = "recommendationIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub recommendation_ids: Vec<String>,
}
impl ResponsibilityResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates the customer responsibility severity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResponsibilitySeverity")]
pub enum ResponsibilitySeverity {
    High,
    Medium,
    Low,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResponsibilitySeverity {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResponsibilitySeverity {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResponsibilitySeverity {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::High => serializer.serialize_unit_variant("ResponsibilitySeverity", 0u32, "High"),
            Self::Medium => serializer.serialize_unit_variant("ResponsibilitySeverity", 1u32, "Medium"),
            Self::Low => serializer.serialize_unit_variant("ResponsibilitySeverity", 2u32, "Low"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates the customer responsibility status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResponsibilityStatus")]
pub enum ResponsibilityStatus {
    Passed,
    Failed,
    NotApplicable,
    PendingApproval,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResponsibilityStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResponsibilityStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResponsibilityStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Passed => serializer.serialize_unit_variant("ResponsibilityStatus", 0u32, "Passed"),
            Self::Failed => serializer.serialize_unit_variant("ResponsibilityStatus", 1u32, "Failed"),
            Self::NotApplicable => serializer.serialize_unit_variant("ResponsibilityStatus", 2u32, "NotApplicable"),
            Self::PendingApproval => serializer.serialize_unit_variant("ResponsibilityStatus", 3u32, "PendingApproval"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates the customer responsibility type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResponsibilityType")]
pub enum ResponsibilityType {
    Automated,
    ScopedManual,
    Manual,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResponsibilityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResponsibilityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResponsibilityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Automated => serializer.serialize_unit_variant("ResponsibilityType", 0u32, "Automated"),
            Self::ScopedManual => serializer.serialize_unit_variant("ResponsibilityType", 1u32, "ScopedManual"),
            Self::Manual => serializer.serialize_unit_variant("ResponsibilityType", 2u32, "Manual"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates whether the fix action is Succeeded or Failed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Result")]
pub enum Result {
    Succeeded,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Result {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Result {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Result {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("Result", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("Result", 1u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Scoping question rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Rule")]
pub enum Rule {
    Required,
    CharLength,
    Url,
    Urls,
    Domains,
    #[serde(rename = "USPrivacyShield")]
    UsPrivacyShield,
    #[serde(rename = "PublicSOX")]
    PublicSox,
    #[serde(rename = "CreditCardPCI")]
    CreditCardPci,
    AzureApplication,
    ValidGuid,
    PublisherVerification,
    DynamicDropdown,
    PreventNonEnglishChar,
    ValidEmail,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Rule {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Rule {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Rule {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Required => serializer.serialize_unit_variant("Rule", 0u32, "Required"),
            Self::CharLength => serializer.serialize_unit_variant("Rule", 1u32, "CharLength"),
            Self::Url => serializer.serialize_unit_variant("Rule", 2u32, "Url"),
            Self::Urls => serializer.serialize_unit_variant("Rule", 3u32, "Urls"),
            Self::Domains => serializer.serialize_unit_variant("Rule", 4u32, "Domains"),
            Self::UsPrivacyShield => serializer.serialize_unit_variant("Rule", 5u32, "USPrivacyShield"),
            Self::PublicSox => serializer.serialize_unit_variant("Rule", 6u32, "PublicSOX"),
            Self::CreditCardPci => serializer.serialize_unit_variant("Rule", 7u32, "CreditCardPCI"),
            Self::AzureApplication => serializer.serialize_unit_variant("Rule", 8u32, "AzureApplication"),
            Self::ValidGuid => serializer.serialize_unit_variant("Rule", 9u32, "ValidGuid"),
            Self::PublisherVerification => serializer.serialize_unit_variant("Rule", 10u32, "PublisherVerification"),
            Self::DynamicDropdown => serializer.serialize_unit_variant("Rule", 11u32, "DynamicDropdown"),
            Self::PreventNonEnglishChar => serializer.serialize_unit_variant("Rule", 12u32, "PreventNonEnglishChar"),
            Self::ValidEmail => serializer.serialize_unit_variant("Rule", 13u32, "ValidEmail"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Scoping answer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopingAnswer {
    #[doc = "Question id."]
    #[serde(rename = "questionId")]
    pub question_id: String,
    #[doc = "Question answer value list."]
    pub answers: Vec<String>,
}
impl ScopingAnswer {
    pub fn new(question_id: String, answers: Vec<String>) -> Self {
        Self { question_id, answers }
    }
}
#[doc = "ScopingConfiguration's properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopingConfigurationProperties {
    #[doc = "List of scoping question answers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub answers: Vec<ScopingAnswer>,
    #[doc = "Resource provisioning states."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ScopingConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class represent an AppComplianceAutomation scoping configuration resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopingConfigurationResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "ScopingConfiguration's properties."]
    pub properties: ScopingConfigurationProperties,
}
impl ScopingConfigurationResource {
    pub fn new(properties: ScopingConfigurationProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "The response of a ScopingConfigurationResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopingConfigurationResourceListResult {
    #[doc = "The ScopingConfigurationResource items on this page"]
    pub value: Vec<ScopingConfigurationResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScopingConfigurationResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ScopingConfigurationResourceListResult {
    pub fn new(value: Vec<ScopingConfigurationResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The definition of a scoping question."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopingQuestion {
    #[doc = "Question id."]
    #[serde(rename = "questionId")]
    pub question_id: String,
    #[doc = "Superior question id."]
    #[serde(rename = "superiorQuestionId", default, skip_serializing_if = "Option::is_none")]
    pub superior_question_id: Option<String>,
    #[doc = "Question input type."]
    #[serde(rename = "inputType")]
    pub input_type: InputType,
    #[doc = "Option id list."]
    #[serde(rename = "optionIds")]
    pub option_ids: Vec<String>,
    #[doc = "The rule of the question."]
    pub rules: Vec<Rule>,
    #[doc = "The answer value to show the sub questions."]
    #[serde(rename = "showSubQuestionsValue", default, skip_serializing_if = "Option::is_none")]
    pub show_sub_questions_value: Option<String>,
}
impl ScopingQuestion {
    pub fn new(question_id: String, input_type: InputType, option_ids: Vec<String>, rules: Vec<Rule>) -> Self {
        Self {
            question_id,
            superior_question_id: None,
            input_type,
            option_ids,
            rules,
            show_sub_questions_value: None,
        }
    }
}
#[doc = "Scoping question list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopingQuestions {
    #[doc = "List of scoping questions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub questions: Vec<ScopingQuestion>,
}
impl ScopingQuestions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "whether to send notification under any event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SendAllEvents")]
pub enum SendAllEvents {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SendAllEvents {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SendAllEvents {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SendAllEvents {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::True => serializer.serialize_unit_variant("SendAllEvents", 0u32, "true"),
            Self::False => serializer.serialize_unit_variant("SendAllEvents", 1u32, "false"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Snapshot's download request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotDownloadRequest {
    #[doc = "Tenant id."]
    #[serde(rename = "reportCreatorTenantId", default, skip_serializing_if = "Option::is_none")]
    pub report_creator_tenant_id: Option<String>,
    #[doc = "Indicates the download type."]
    #[serde(rename = "downloadType")]
    pub download_type: DownloadType,
    #[doc = "The offerGuid which mapping to the reports."]
    #[serde(rename = "offerGuid", default, skip_serializing_if = "Option::is_none")]
    pub offer_guid: Option<String>,
}
impl SnapshotDownloadRequest {
    pub fn new(download_type: DownloadType) -> Self {
        Self {
            report_creator_tenant_id: None,
            download_type,
            offer_guid: None,
        }
    }
}
#[doc = "Snapshot's properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotProperties {
    #[doc = "Snapshot name."]
    #[serde(rename = "snapshotName", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<::time::OffsetDateTime>,
    #[doc = "Resource provisioning states."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Create Report's properties."]
    #[serde(rename = "reportProperties", default, skip_serializing_if = "Option::is_none")]
    pub report_properties: Option<ReportProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "reportSystemData", default, skip_serializing_if = "Option::is_none")]
    pub report_system_data: Option<SystemData>,
    #[doc = "List of compliance results."]
    #[serde(
        rename = "complianceResults",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub compliance_results: Vec<ComplianceResult>,
}
impl SnapshotProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class represent a AppComplianceAutomation snapshot resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Snapshot's properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotProperties>,
}
impl SnapshotResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a SnapshotResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotResourceListResult {
    #[doc = "The SnapshotResource items on this page"]
    pub value: Vec<SnapshotResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SnapshotResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SnapshotResourceListResult {
    pub fn new(value: Vec<SnapshotResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Single status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusItem {
    #[doc = "Status name - e.g. \"Active\", \"Failed\"."]
    #[serde(rename = "statusName", default, skip_serializing_if = "Option::is_none")]
    pub status_name: Option<String>,
    #[doc = "Status value. e.g. \"100\", or \"100%\"."]
    #[serde(rename = "statusValue", default, skip_serializing_if = "Option::is_none")]
    pub status_value: Option<String>,
}
impl StatusItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The information of 'bring your own storage' account binding to the report"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageInfo {
    #[doc = "The subscription id which 'bring your own storage' account belongs to"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The resourceGroup which 'bring your own storage' account belongs to"]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "'bring your own storage' account name"]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "The region of 'bring your own storage' account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl StorageInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Synchronize certification record request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncCertRecordRequest {
    #[doc = "A class represent the certification record synchronized from app compliance."]
    #[serde(rename = "certRecord")]
    pub cert_record: CertSyncRecord,
}
impl SyncCertRecordRequest {
    pub fn new(cert_record: CertSyncRecord) -> Self {
        Self { cert_record }
    }
}
#[doc = "Synchronize certification record response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncCertRecordResponse {
    #[doc = "A class represent the certification record synchronized from app compliance."]
    #[serde(rename = "certRecord", default, skip_serializing_if = "Option::is_none")]
    pub cert_record: Option<CertSyncRecord>,
}
impl SyncCertRecordResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Trigger evaluation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerEvaluationProperty {
    #[doc = "The time when the evaluation is triggered."]
    #[serde(rename = "triggerTime", default, with = "azure_core::date::rfc3339::option")]
    pub trigger_time: Option<::time::OffsetDateTime>,
    #[doc = "The time when the evaluation is end."]
    #[serde(rename = "evaluationEndTime", default, with = "azure_core::date::rfc3339::option")]
    pub evaluation_end_time: Option<::time::OffsetDateTime>,
    #[doc = "List of resource ids to be evaluated"]
    #[serde(
        rename = "resourceIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resource_ids: Vec<String>,
    #[doc = "List of quick assessments"]
    #[serde(
        rename = "quickAssessments",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub quick_assessments: Vec<QuickAssessment>,
}
impl TriggerEvaluationProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Trigger evaluation request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerEvaluationRequest {
    #[doc = "List of resource ids to be evaluated"]
    #[serde(rename = "resourceIds")]
    pub resource_ids: Vec<String>,
}
impl TriggerEvaluationRequest {
    pub fn new(resource_ids: Vec<String>) -> Self {
        Self { resource_ids }
    }
}
#[doc = "Trigger evaluation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerEvaluationResponse {
    #[doc = "Trigger evaluation response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TriggerEvaluationProperty>,
}
impl TriggerEvaluationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "whether to update webhookKey."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UpdateWebhookKey")]
pub enum UpdateWebhookKey {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UpdateWebhookKey {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UpdateWebhookKey {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UpdateWebhookKey {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::True => serializer.serialize_unit_variant("UpdateWebhookKey", 0u32, "true"),
            Self::False => serializer.serialize_unit_variant("UpdateWebhookKey", 1u32, "false"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "whether webhookKey is enabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WebhookKeyEnabled")]
pub enum WebhookKeyEnabled {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WebhookKeyEnabled {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WebhookKeyEnabled {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WebhookKeyEnabled {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::True => serializer.serialize_unit_variant("WebhookKeyEnabled", 0u32, "true"),
            Self::False => serializer.serialize_unit_variant("WebhookKeyEnabled", 1u32, "false"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Webhook properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebhookProperties {
    #[doc = "Webhook id in database."]
    #[serde(rename = "webhookId", default, skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
    #[doc = "Webhook status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WebhookStatus>,
    #[doc = "Tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "whether to send notification under any event."]
    #[serde(rename = "sendAllEvents", default, skip_serializing_if = "Option::is_none")]
    pub send_all_events: Option<SendAllEvents>,
    #[doc = "under which event notification should be sent."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub events: Vec<NotificationEvent>,
    #[doc = "webhook payload url"]
    #[serde(rename = "payloadUrl", default, skip_serializing_if = "Option::is_none")]
    pub payload_url: Option<String>,
    #[doc = "content type"]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ContentType>,
    #[doc = "webhook secret token. If not set, this field value is null; otherwise, please set a string value."]
    #[serde(rename = "webhookKey", default, skip_serializing_if = "Option::is_none")]
    pub webhook_key: Option<String>,
    #[doc = "whether to update webhookKey."]
    #[serde(rename = "updateWebhookKey", default, skip_serializing_if = "Option::is_none")]
    pub update_webhook_key: Option<UpdateWebhookKey>,
    #[doc = "whether webhookKey is enabled."]
    #[serde(rename = "webhookKeyEnabled", default, skip_serializing_if = "Option::is_none")]
    pub webhook_key_enabled: Option<WebhookKeyEnabled>,
    #[doc = "whether to enable ssl verification"]
    #[serde(rename = "enableSslVerification", default, skip_serializing_if = "Option::is_none")]
    pub enable_ssl_verification: Option<EnableSslVerification>,
    #[doc = "webhook deliveryStatus"]
    #[serde(rename = "deliveryStatus", default, skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<DeliveryStatus>,
    #[doc = "Resource provisioning states."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl WebhookProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class represent an AppComplianceAutomation webhook resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Webhook properties."]
    pub properties: WebhookProperties,
}
impl WebhookResource {
    pub fn new(properties: WebhookProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "The response of a WebhookResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookResourceListResult {
    #[doc = "The WebhookResource items on this page"]
    pub value: Vec<WebhookResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WebhookResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WebhookResourceListResult {
    pub fn new(value: Vec<WebhookResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A class represent a AppComplianceAutomation webhook resource update properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebhookResourcePatch {
    #[doc = "Webhook properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebhookProperties>,
}
impl WebhookResourcePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Webhook status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WebhookStatus")]
pub enum WebhookStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WebhookStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WebhookStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WebhookStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("WebhookStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("WebhookStatus", 1u32, "Disabled"),
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
