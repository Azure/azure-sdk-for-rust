#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A class represent the assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Assessment {
    #[doc = "The name of the assessment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates the assessment severity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<AssessmentSeverity>,
    #[doc = "The description of the assessment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The remediation of the assessment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remediation: Option<String>,
    #[doc = "Indicates whether all the resource(s) are compliant."]
    #[serde(rename = "isPass", default, skip_serializing_if = "Option::is_none")]
    pub is_pass: Option<assessment::IsPass>,
    #[doc = "The policy id mapping to this assessment."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "List of resource assessments."]
    #[serde(
        rename = "resourceList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resource_list: Vec<AssessmentResource>,
}
impl Assessment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod assessment {
    use super::*;
    #[doc = "Indicates whether all the resource(s) are compliant."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IsPass")]
    pub enum IsPass {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IsPass {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IsPass {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IsPass {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("IsPass", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("IsPass", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A class represent the assessment resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessmentResource {
    #[doc = "The Id of the resource."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Indicates the resource status."]
    #[serde(rename = "resourceStatus", default, skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<ResourceStatus>,
    #[doc = "The reason for the N/A resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The status change date for the resource. For unavailable date, set it as N/A."]
    #[serde(rename = "statusChangeDate", default, skip_serializing_if = "Option::is_none")]
    pub status_change_date: Option<String>,
}
impl AssessmentResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates the assessment severity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AssessmentSeverity")]
pub enum AssessmentSeverity {
    High,
    Medium,
    Low,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AssessmentSeverity {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AssessmentSeverity {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AssessmentSeverity {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::High => serializer.serialize_unit_variant("AssessmentSeverity", 0u32, "High"),
            Self::Medium => serializer.serialize_unit_variant("AssessmentSeverity", 1u32, "Medium"),
            Self::Low => serializer.serialize_unit_variant("AssessmentSeverity", 2u32, "Low"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A class represent the compliance category."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Category {
    #[doc = "The name of the compliance category. e.g. \"Operational Security\""]
    #[serde(rename = "categoryName", default, skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    #[doc = "Indicates the compliance category type."]
    #[serde(rename = "categoryType", default, skip_serializing_if = "Option::is_none")]
    pub category_type: Option<CategoryType>,
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
    Healthy,
    Unhealthy,
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
            Self::Healthy => serializer.serialize_unit_variant("CategoryStatus", 0u32, "Healthy"),
            Self::Unhealthy => serializer.serialize_unit_variant("CategoryStatus", 1u32, "Unhealthy"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates the compliance category type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CategoryType")]
pub enum CategoryType {
    FullyAutomated,
    PartiallyAutomated,
    Manual,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CategoryType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CategoryType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CategoryType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::FullyAutomated => serializer.serialize_unit_variant("CategoryType", 0u32, "FullyAutomated"),
            Self::PartiallyAutomated => serializer.serialize_unit_variant("CategoryType", 1u32, "PartiallyAutomated"),
            Self::Manual => serializer.serialize_unit_variant("CategoryType", 2u32, "Manual"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Object that includes all the content for single compliance result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComplianceReportItem {
    #[doc = "The category name."]
    #[serde(rename = "categoryName", default, skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    #[doc = "The control Id - e.g. \"1\"."]
    #[serde(rename = "controlId", default, skip_serializing_if = "Option::is_none")]
    pub control_id: Option<String>,
    #[doc = "The control name."]
    #[serde(rename = "controlName", default, skip_serializing_if = "Option::is_none")]
    pub control_name: Option<String>,
    #[doc = "Indicates the control type."]
    #[serde(rename = "controlType", default, skip_serializing_if = "Option::is_none")]
    pub control_type: Option<ControlType>,
    #[doc = "The compliance result's status."]
    #[serde(rename = "complianceState", default, skip_serializing_if = "Option::is_none")]
    pub compliance_state: Option<compliance_report_item::ComplianceState>,
    #[doc = "The compliance result mapped policy Id."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "The policy's display name."]
    #[serde(rename = "policyDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub policy_display_name: Option<String>,
    #[doc = "The policy's detail description."]
    #[serde(rename = "policyDescription", default, skip_serializing_if = "Option::is_none")]
    pub policy_description: Option<String>,
    #[doc = "The compliance result mapped subscription Id."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The compliance result mapped resource group."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "The compliance result mapped resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The compliance result mapped resource Id - e.g. \"/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.Compute/virtualMachines/vm1\"."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The compliance result last changed date - e.g. \"2022-10-24T02:55:16.3274379Z\". For unavailable date, set it as \"N/A\"."]
    #[serde(rename = "statusChangeDate", default, skip_serializing_if = "Option::is_none")]
    pub status_change_date: Option<String>,
}
impl ComplianceReportItem {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod compliance_report_item {
    use super::*;
    #[doc = "The compliance result's status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ComplianceState")]
    pub enum ComplianceState {
        Healthy,
        Unhealthy,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ComplianceState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ComplianceState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ComplianceState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Healthy => serializer.serialize_unit_variant("ComplianceState", 0u32, "Healthy"),
                Self::Unhealthy => serializer.serialize_unit_variant("ComplianceState", 1u32, "Unhealthy"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "A class represent the control."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Control {
    #[doc = "The Id of the control. e.g. \"Operational Security#10\""]
    #[serde(rename = "controlId", default, skip_serializing_if = "Option::is_none")]
    pub control_id: Option<String>,
    #[doc = "The short name of the control. e.g. \"Unsupported OS and Software.\""]
    #[serde(rename = "controlShortName", default, skip_serializing_if = "Option::is_none")]
    pub control_short_name: Option<String>,
    #[doc = "The full name of the control. e.g. \"Validate that unsupported operating systems and software components are not in use.\""]
    #[serde(rename = "controlFullName", default, skip_serializing_if = "Option::is_none")]
    pub control_full_name: Option<String>,
    #[doc = "Indicates the control type."]
    #[serde(rename = "controlType", default, skip_serializing_if = "Option::is_none")]
    pub control_type: Option<ControlType>,
    #[doc = "The control's description"]
    #[serde(rename = "controlDescription", default, skip_serializing_if = "Option::is_none")]
    pub control_description: Option<String>,
    #[doc = "The hyper link to the control's description'."]
    #[serde(rename = "controlDescriptionHyperLink", default, skip_serializing_if = "Option::is_none")]
    pub control_description_hyper_link: Option<String>,
    #[doc = "Indicates the control status."]
    #[serde(rename = "controlStatus", default, skip_serializing_if = "Option::is_none")]
    pub control_status: Option<ControlStatus>,
    #[doc = "List of assessments."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub assessments: Vec<Assessment>,
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
    #[serde(rename = "familyName", default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[doc = "Indicates the control family type."]
    #[serde(rename = "familyType", default, skip_serializing_if = "Option::is_none")]
    pub family_type: Option<ControlFamilyType>,
    #[doc = "Indicates the control family status."]
    #[serde(rename = "familyStatus", default, skip_serializing_if = "Option::is_none")]
    pub family_status: Option<ControlFamilyStatus>,
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
    Healthy,
    Unhealthy,
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
            Self::Healthy => serializer.serialize_unit_variant("ControlFamilyStatus", 0u32, "Healthy"),
            Self::Unhealthy => serializer.serialize_unit_variant("ControlFamilyStatus", 1u32, "Unhealthy"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates the control family type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ControlFamilyType")]
pub enum ControlFamilyType {
    FullyAutomated,
    PartiallyAutomated,
    Manual,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ControlFamilyType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ControlFamilyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ControlFamilyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::FullyAutomated => serializer.serialize_unit_variant("ControlFamilyType", 0u32, "FullyAutomated"),
            Self::PartiallyAutomated => serializer.serialize_unit_variant("ControlFamilyType", 1u32, "PartiallyAutomated"),
            Self::Manual => serializer.serialize_unit_variant("ControlFamilyType", 2u32, "Manual"),
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
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates the control type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ControlType")]
pub enum ControlType {
    FullyAutomated,
    PartiallyAutomated,
    Manual,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ControlType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ControlType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ControlType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::FullyAutomated => serializer.serialize_unit_variant("ControlType", 0u32, "FullyAutomated"),
            Self::PartiallyAutomated => serializer.serialize_unit_variant("ControlType", 1u32, "PartiallyAutomated"),
            Self::Manual => serializer.serialize_unit_variant("ControlType", 2u32, "Manual"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Object that includes all the possible response for the download operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DownloadResponse {
    #[doc = "List of the reports"]
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
    #[doc = "compliance pdf report"]
    #[serde(rename = "compliancePdfReport", default, skip_serializing_if = "Option::is_none")]
    pub compliance_pdf_report: Option<download_response::CompliancePdfReport>,
    #[doc = "compliance detailed pdf report"]
    #[serde(rename = "complianceDetailedPdfReport", default, skip_serializing_if = "Option::is_none")]
    pub compliance_detailed_pdf_report: Option<download_response::ComplianceDetailedPdfReport>,
}
impl DownloadResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod download_response {
    use super::*;
    #[doc = "compliance pdf report"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct CompliancePdfReport {
        #[doc = "uri of compliance pdf report"]
        #[serde(rename = "sasUri", default, skip_serializing_if = "Option::is_none")]
        pub sas_uri: Option<String>,
    }
    impl CompliancePdfReport {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "compliance detailed pdf report"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ComplianceDetailedPdfReport {
        #[doc = "uri of compliance detailed pdf report"]
        #[serde(rename = "sasUri", default, skip_serializing_if = "Option::is_none")]
        pub sas_uri: Option<String>,
    }
    impl ComplianceDetailedPdfReport {
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
    #[doc = "The count of all passed full automation control."]
    #[serde(rename = "passedCount", default, skip_serializing_if = "Option::is_none")]
    pub passed_count: Option<i32>,
    #[doc = "The count of all failed full automation control."]
    #[serde(rename = "failedCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i32>,
    #[doc = "The count of all manual control."]
    #[serde(rename = "manualCount", default, skip_serializing_if = "Option::is_none")]
    pub manual_count: Option<i32>,
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
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Updating"),
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
#[doc = "Report's properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportProperties {
    #[doc = "Report id in database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Report status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<report_properties::Status>,
    #[doc = "Report's tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Report name."]
    #[serde(rename = "reportName", default, skip_serializing_if = "Option::is_none")]
    pub report_name: Option<String>,
    #[doc = "Report offer Guid."]
    #[serde(rename = "offerGuid", default, skip_serializing_if = "Option::is_none")]
    pub offer_guid: Option<String>,
    #[doc = "Report collection trigger time's time zone, the available list can be obtained by executing \"Get-TimeZone -ListAvailable\" in PowerShell.\r\nAn example of valid timezone id is \"Pacific Standard Time\"."]
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    #[doc = "Report collection trigger time."]
    #[serde(rename = "triggerTime", with = "azure_core::date::rfc3339")]
    pub trigger_time: time::OffsetDateTime,
    #[doc = "Report next collection trigger time."]
    #[serde(rename = "nextTriggerTime", default, with = "azure_core::date::rfc3339::option")]
    pub next_trigger_time: Option<time::OffsetDateTime>,
    #[doc = "Report last collection trigger time."]
    #[serde(rename = "lastTriggerTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_trigger_time: Option<time::OffsetDateTime>,
    #[doc = "List of subscription Ids."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subscriptions: Vec<String>,
    #[doc = "List of resource data."]
    pub resources: Vec<ResourceMetadata>,
    #[doc = "A list which includes all the compliance result for one report."]
    #[serde(rename = "complianceStatus", default, skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<ReportComplianceStatus>,
    #[doc = "Resource provisioning states."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ReportProperties {
    pub fn new(time_zone: String, trigger_time: time::OffsetDateTime, resources: Vec<ResourceMetadata>) -> Self {
        Self {
            id: None,
            status: None,
            tenant_id: None,
            report_name: None,
            offer_guid: None,
            time_zone,
            trigger_time,
            next_trigger_time: None,
            last_trigger_time: None,
            subscriptions: Vec::new(),
            resources,
            compliance_status: None,
            provisioning_state: None,
        }
    }
}
pub mod report_properties {
    use super::*;
    #[doc = "Report status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Active,
        Failed,
        Disabled,
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
                Self::Active => serializer.serialize_unit_variant("Status", 0u32, "Active"),
                Self::Failed => serializer.serialize_unit_variant("Status", 1u32, "Failed"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 2u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A class represent an AppComplianceAutomation report resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Report's properties."]
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
#[doc = "Object that includes an array of resources and a possible link for next set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportResourceList {
    #[doc = "List of the reports"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ReportResource>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReportResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ReportResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class represent a AppComplianceAutomation report resource update properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportResourcePatch {
    #[doc = "Report's properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReportProperties>,
}
impl ReportResourcePatch {
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
    #[doc = "The resource type of this resource."]
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
    #[doc = "Resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Resource kind."]
    #[serde(rename = "resourceKind", default, skip_serializing_if = "Option::is_none")]
    pub resource_kind: Option<String>,
    #[doc = "Resource name."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "Resource's tag type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceMetadata {
    pub fn new(resource_id: String) -> Self {
        Self {
            resource_id,
            resource_type: None,
            resource_kind: None,
            resource_name: None,
            tags: None,
        }
    }
}
#[doc = "Indicates the resource status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceStatus")]
pub enum ResourceStatus {
    Healthy,
    Unhealthy,
    NotApplicable,
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
            Self::NotApplicable => serializer.serialize_unit_variant("ResourceStatus", 2u32, "NotApplicable"),
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
    pub download_type: snapshot_download_request::DownloadType,
    #[doc = "The offerGuid which mapping to the reports."]
    #[serde(rename = "offerGuid", default, skip_serializing_if = "Option::is_none")]
    pub offer_guid: Option<String>,
}
impl SnapshotDownloadRequest {
    pub fn new(download_type: snapshot_download_request::DownloadType) -> Self {
        Self {
            report_creator_tenant_id: None,
            download_type,
            offer_guid: None,
        }
    }
}
pub mod snapshot_download_request {
    use super::*;
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
}
#[doc = "Snapshot's properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotProperties {
    #[doc = "Snapshot id in the database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Snapshot name."]
    #[serde(rename = "snapshotName", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "Resource provisioning states."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Report's properties."]
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
#[doc = "Object that includes an array of resources and a possible link for next set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotResourceList {
    #[doc = "List of the snapshots"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SnapshotResource>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SnapshotResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SnapshotResourceList {
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
