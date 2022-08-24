#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Request body for applying artifacts to a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplyArtifactsRequest {
    #[doc = "The list of artifacts to apply."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<ArtifactInstallProperties>,
}
impl ApplyArtifactsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a generated ARM template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmTemplateInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<Object>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Object>,
}
impl ArmTemplateInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Artifact {
    #[doc = "Properties of an artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ArtifactProperties>,
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
impl Artifact {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an artifact deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactDeploymentStatusProperties {
    #[doc = "The deployment status of the artifact."]
    #[serde(rename = "deploymentStatus", default, skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    #[doc = "The total count of the artifacts that were successfully applied."]
    #[serde(rename = "artifactsApplied", default, skip_serializing_if = "Option::is_none")]
    pub artifacts_applied: Option<i32>,
    #[doc = "The total count of the artifacts that were tentatively applied."]
    #[serde(rename = "totalArtifacts", default, skip_serializing_if = "Option::is_none")]
    pub total_artifacts: Option<i32>,
}
impl ArtifactDeploymentStatusProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactInstallProperties {
    #[doc = "The artifact's identifier."]
    #[serde(rename = "artifactId", default, skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    #[doc = "The parameters of the artifact."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ArtifactParameterProperties>,
}
impl ArtifactInstallProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an artifact parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactParameterProperties {
    #[doc = "The name of the artifact parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the artifact parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ArtifactParameterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactProperties {
    #[doc = "The title of the artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The description of the artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The file path of the artifact."]
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[doc = "The icon of the artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[doc = "Gets or sets the type of the target os."]
    #[serde(rename = "targetOsType", default, skip_serializing_if = "Option::is_none")]
    pub target_os_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Object>,
}
impl ArtifactProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an artifact source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactSource {
    #[doc = "Properties of an artifact source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ArtifactSourceProperties>,
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
impl ArtifactSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an artifact source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactSourceProperties {
    #[doc = "The display name of the artifact source."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The URI of the artifact source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "The type of the artifact source."]
    #[serde(rename = "sourceType", default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<artifact_source_properties::SourceType>,
    #[doc = "The folder path of the artifact source."]
    #[serde(rename = "folderPath", default, skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<String>,
    #[doc = "The branch reference of the artifact source."]
    #[serde(rename = "branchRef", default, skip_serializing_if = "Option::is_none")]
    pub branch_ref: Option<String>,
    #[doc = "The security token of the artifact source."]
    #[serde(rename = "securityToken", default, skip_serializing_if = "Option::is_none")]
    pub security_token: Option<String>,
    #[doc = "The status of the artifact source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<artifact_source_properties::Status>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ArtifactSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod artifact_source_properties {
    use super::*;
    #[doc = "The type of the artifact source."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SourceType")]
    pub enum SourceType {
        VsoGit,
        GitHub,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SourceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SourceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SourceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::VsoGit => serializer.serialize_unit_variant("SourceType", 0u32, "VsoGit"),
                Self::GitHub => serializer.serialize_unit_variant("SourceType", 1u32, "GitHub"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the artifact source."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A cost item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Cost {
    #[doc = "Properties of a cost item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CostProperties>,
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
impl Cost {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CostInsight {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CostInsightProperties>,
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
impl CostInsight {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CostInsightProperties {
    #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "vmCosts", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_costs: Vec<VmCostProperties>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl CostInsightProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The per-day properties of a cost item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CostPerDayProperties {
    #[doc = "The date of the cost item."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub date: Option<time::OffsetDateTime>,
    #[doc = "The cost of the cost item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    #[doc = "The type of the cost."]
    #[serde(rename = "costType", default, skip_serializing_if = "Option::is_none")]
    pub cost_type: Option<cost_per_day_properties::CostType>,
}
impl CostPerDayProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cost_per_day_properties {
    use super::*;
    #[doc = "The type of the cost."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CostType")]
    pub enum CostType {
        Unavailable,
        Reported,
        Projected,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CostType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CostType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CostType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unavailable => serializer.serialize_unit_variant("CostType", 0u32, "Unavailable"),
                Self::Reported => serializer.serialize_unit_variant("CostType", 1u32, "Reported"),
                Self::Projected => serializer.serialize_unit_variant("CostType", 2u32, "Projected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of a cost item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CostProperties {
    #[doc = "The currency code of the cost."]
    #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[doc = "The per-day costs items of the cost."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub costs: Vec<CostPerDayProperties>,
}
impl CostProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A custom image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomImage {
    #[doc = "Properties of a custom image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomImageProperties>,
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
impl CustomImage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a custom image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomImageProperties {
    #[doc = "Properties for creating a custom image from a virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm: Option<CustomImagePropertiesFromVm>,
    #[doc = "Properties for creating a custom image from a VHD."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vhd: Option<CustomImagePropertiesCustom>,
    #[doc = "The description of the custom image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The OS type of the custom image."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<custom_image_properties::OsType>,
    #[doc = "The author of the custom image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[doc = "The creation date of the custom image."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl CustomImageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod custom_image_properties {
    use super::*;
    #[doc = "The OS type of the custom image."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OsType")]
    pub enum OsType {
        Windows,
        Linux,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OsType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OsType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OsType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Windows => serializer.serialize_unit_variant("OsType", 0u32, "Windows"),
                Self::Linux => serializer.serialize_unit_variant("OsType", 1u32, "Linux"),
                Self::None => serializer.serialize_unit_variant("OsType", 2u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties for creating a custom image from a VHD."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomImagePropertiesCustom {
    #[doc = "The image name."]
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[doc = "Indicates whether sysprep has been run on the VHD."]
    #[serde(rename = "sysPrep", default, skip_serializing_if = "Option::is_none")]
    pub sys_prep: Option<bool>,
}
impl CustomImagePropertiesCustom {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for creating a custom image from a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomImagePropertiesFromVm {
    #[doc = "The source vm identifier."]
    #[serde(rename = "sourceVmId", default, skip_serializing_if = "Option::is_none")]
    pub source_vm_id: Option<String>,
    #[doc = "Indicates whether sysprep has been run on the VHD."]
    #[serde(rename = "sysPrep", default, skip_serializing_if = "Option::is_none")]
    pub sys_prep: Option<bool>,
    #[doc = "Information about a Windows OS."]
    #[serde(rename = "windowsOsInfo", default, skip_serializing_if = "Option::is_none")]
    pub windows_os_info: Option<WindowsOsInfo>,
    #[doc = "Information about a Linux OS."]
    #[serde(rename = "linuxOsInfo", default, skip_serializing_if = "Option::is_none")]
    pub linux_os_info: Option<LinuxOsInfo>,
}
impl CustomImagePropertiesFromVm {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a daily schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DayDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}
impl DayDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for evaluating a policy set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EvaluatePoliciesProperties {
    #[doc = "The fact name."]
    #[serde(rename = "factName", default, skip_serializing_if = "Option::is_none")]
    pub fact_name: Option<String>,
    #[doc = "The fact data."]
    #[serde(rename = "factData", default, skip_serializing_if = "Option::is_none")]
    pub fact_data: Option<String>,
    #[doc = "The value offset."]
    #[serde(rename = "valueOffset", default, skip_serializing_if = "Option::is_none")]
    pub value_offset: Option<String>,
}
impl EvaluatePoliciesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request body for evaluating a policy set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EvaluatePoliciesRequest {
    #[doc = "Policies to evaluate."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policies: Vec<EvaluatePoliciesProperties>,
}
impl EvaluatePoliciesRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response body for evaluating a policy set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EvaluatePoliciesResponse {
    #[doc = "Results of evaluating a policy set."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<PolicySetResult>,
}
impl EvaluatePoliciesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A formula."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Formula {
    #[doc = "Properties of a formula."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FormulaProperties>,
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
impl Formula {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a formula."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FormulaProperties {
    #[doc = "The description of the formula."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The author of the formula."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[doc = "The OS type of the formula."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The creation date of the formula."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "A virtual machine."]
    #[serde(rename = "formulaContent", default, skip_serializing_if = "Option::is_none")]
    pub formula_content: Option<LabVirtualMachine>,
    #[doc = "Information about a VM from which a formula is to be created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm: Option<FormulaPropertiesFromVm>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl FormulaProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a VM from which a formula is to be created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FormulaPropertiesFromVm {
    #[doc = "The identifier of the VM from which a formula is to be created."]
    #[serde(rename = "labVmId", default, skip_serializing_if = "Option::is_none")]
    pub lab_vm_id: Option<String>,
}
impl FormulaPropertiesFromVm {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A gallery image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryImage {
    #[doc = "Properties of a gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryImageProperties>,
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
impl GalleryImage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a gallery image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryImageProperties {
    #[doc = "The author of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[doc = "The creation date of the gallery image."]
    #[serde(rename = "createdDate", default, with = "azure_core::date::rfc3339::option")]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl GalleryImageProperties {
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
#[doc = "Parameters for generating an ARM template for deploying artifacts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateArmTemplateRequest {
    #[doc = "The resource name of the virtual machine."]
    #[serde(rename = "virtualMachineName", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_name: Option<String>,
    #[doc = "The parameters of the ARM template."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterInfo>,
    #[doc = "The location of the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl GenerateArmTemplateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for generating an upload URI."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateUploadUriParameter {
    #[doc = "The blob name of the upload URI."]
    #[serde(rename = "blobName", default, skip_serializing_if = "Option::is_none")]
    pub blob_name: Option<String>,
}
impl GenerateUploadUriParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response body for generating an upload URI."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateUploadUriResponse {
    #[doc = "The upload URI for the VHD."]
    #[serde(rename = "uploadUri", default, skip_serializing_if = "Option::is_none")]
    pub upload_uri: Option<String>,
}
impl GenerateUploadUriResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an hourly schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HourDetails {
    #[doc = "Minutes of the hour the schedule will run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minute: Option<i32>,
}
impl HourDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A lab."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Lab {
    #[doc = "Properties of a lab."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabProperties>,
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
impl Lab {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a lab."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabProperties {
    #[doc = "The lab's default storage account."]
    #[serde(rename = "defaultStorageAccount", default, skip_serializing_if = "Option::is_none")]
    pub default_storage_account: Option<String>,
    #[doc = "The artifact storage account of the lab."]
    #[serde(rename = "artifactsStorageAccount", default, skip_serializing_if = "Option::is_none")]
    pub artifacts_storage_account: Option<String>,
    #[doc = "The storage accounts of the lab."]
    #[serde(rename = "storageAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub storage_accounts: Vec<String>,
    #[doc = "The name of the key vault of the lab."]
    #[serde(rename = "vaultName", default, skip_serializing_if = "Option::is_none")]
    pub vault_name: Option<String>,
    #[doc = "The type of the lab storage."]
    #[serde(rename = "labStorageType", default, skip_serializing_if = "Option::is_none")]
    pub lab_storage_type: Option<lab_properties::LabStorageType>,
    #[doc = "The default virtual network identifier of the lab."]
    #[serde(rename = "defaultVirtualNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub default_virtual_network_id: Option<String>,
    #[doc = "The creation date of the lab."]
    #[serde(rename = "createdDate", default, with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl LabProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod lab_properties {
    use super::*;
    #[doc = "The type of the lab storage."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LabStorageType")]
    pub enum LabStorageType {
        Standard,
        Premium,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LabStorageType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LabStorageType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LabStorageType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("LabStorageType", 0u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("LabStorageType", 1u32, "Premium"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of a VHD in the lab."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabVhd {
    #[doc = "The absolute URI of the VHD."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl LabVhd {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabVirtualMachine {
    #[doc = "Properties of a virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabVirtualMachineProperties>,
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
impl LabVirtualMachine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabVirtualMachineProperties {
    #[doc = "The notes of the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[doc = "The object identifier of the owner of the virtual machine."]
    #[serde(rename = "ownerObjectId", default, skip_serializing_if = "Option::is_none")]
    pub owner_object_id: Option<String>,
    #[doc = "The object identifier of the creator of the virtual machine."]
    #[serde(rename = "createdByUserId", default, skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<String>,
    #[doc = "The email address of creator of the virtual machine."]
    #[serde(rename = "createdByUser", default, skip_serializing_if = "Option::is_none")]
    pub created_by_user: Option<String>,
    #[doc = "The resource identifier (Microsoft.Compute) of the virtual machine."]
    #[serde(rename = "computeId", default, skip_serializing_if = "Option::is_none")]
    pub compute_id: Option<String>,
    #[doc = "The custom image identifier of the virtual machine."]
    #[serde(rename = "customImageId", default, skip_serializing_if = "Option::is_none")]
    pub custom_image_id: Option<String>,
    #[doc = "The OS type of the virtual machine."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The size of the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "The user name of the virtual machine."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "The password of the virtual machine administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "The SSH key of the virtual machine administrator."]
    #[serde(rename = "sshKey", default, skip_serializing_if = "Option::is_none")]
    pub ssh_key: Option<String>,
    #[doc = "A value indicating whether this virtual machine uses an SSH key for authentication."]
    #[serde(rename = "isAuthenticationWithSshKey", default, skip_serializing_if = "Option::is_none")]
    pub is_authentication_with_ssh_key: Option<bool>,
    #[doc = "The fully-qualified domain name of the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "The lab subnet name of the virtual machine."]
    #[serde(rename = "labSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub lab_subnet_name: Option<String>,
    #[doc = "The lab virtual network identifier of the virtual machine."]
    #[serde(rename = "labVirtualNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub lab_virtual_network_id: Option<String>,
    #[doc = "Indicates whether the virtual machine is to be created without a public IP address."]
    #[serde(rename = "disallowPublicIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub disallow_public_ip_address: Option<bool>,
    #[doc = "The artifacts to be installed on the virtual machine."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<ArtifactInstallProperties>,
    #[doc = "Properties of an artifact deployment."]
    #[serde(rename = "artifactDeploymentStatus", default, skip_serializing_if = "Option::is_none")]
    pub artifact_deployment_status: Option<ArtifactDeploymentStatusProperties>,
    #[doc = "The reference information for an Azure Marketplace image."]
    #[serde(rename = "galleryImageReference", default, skip_serializing_if = "Option::is_none")]
    pub gallery_image_reference: Option<GalleryImageReference>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl LabVirtualMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a Linux OS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxOsInfo {
    #[doc = "The state of the Linux OS."]
    #[serde(rename = "linuxOsState", default, skip_serializing_if = "Option::is_none")]
    pub linux_os_state: Option<linux_os_info::LinuxOsState>,
}
impl LinuxOsInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod linux_os_info {
    use super::*;
    #[doc = "The state of the Linux OS."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LinuxOsState")]
    pub enum LinuxOsState {
        NonDeprovisioned,
        DeprovisionRequested,
        DeprovisionApplied,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LinuxOsState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LinuxOsState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LinuxOsState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NonDeprovisioned => serializer.serialize_unit_variant("LinuxOsState", 0u32, "NonDeprovisioned"),
                Self::DeprovisionRequested => serializer.serialize_unit_variant("LinuxOsState", 1u32, "DeprovisionRequested"),
                Self::DeprovisionApplied => serializer.serialize_unit_variant("LinuxOsState", 2u32, "DeprovisionApplied"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Object {}
impl Object {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ParameterInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Policy {
    #[doc = "Properties of a Policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyProperties>,
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
impl Policy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyProperties {
    #[doc = "The description of the policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The status of the policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<policy_properties::Status>,
    #[doc = "The fact name of the policy."]
    #[serde(rename = "factName", default, skip_serializing_if = "Option::is_none")]
    pub fact_name: Option<policy_properties::FactName>,
    #[doc = "The fact data of the policy."]
    #[serde(rename = "factData", default, skip_serializing_if = "Option::is_none")]
    pub fact_data: Option<String>,
    #[doc = "The threshold of the policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<String>,
    #[doc = "The evaluator type of the policy."]
    #[serde(rename = "evaluatorType", default, skip_serializing_if = "Option::is_none")]
    pub evaluator_type: Option<policy_properties::EvaluatorType>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl PolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod policy_properties {
    use super::*;
    #[doc = "The status of the policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The fact name of the policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FactName")]
    pub enum FactName {
        UserOwnedLabVmCount,
        LabVmCount,
        LabVmSize,
        GalleryImage,
        UserOwnedLabVmCountInSubnet,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FactName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FactName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FactName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::UserOwnedLabVmCount => serializer.serialize_unit_variant("FactName", 0u32, "UserOwnedLabVmCount"),
                Self::LabVmCount => serializer.serialize_unit_variant("FactName", 1u32, "LabVmCount"),
                Self::LabVmSize => serializer.serialize_unit_variant("FactName", 2u32, "LabVmSize"),
                Self::GalleryImage => serializer.serialize_unit_variant("FactName", 3u32, "GalleryImage"),
                Self::UserOwnedLabVmCountInSubnet => serializer.serialize_unit_variant("FactName", 4u32, "UserOwnedLabVmCountInSubnet"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The evaluator type of the policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EvaluatorType")]
    pub enum EvaluatorType {
        AllowedValuesPolicy,
        MaxValuePolicy,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EvaluatorType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EvaluatorType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EvaluatorType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AllowedValuesPolicy => serializer.serialize_unit_variant("EvaluatorType", 0u32, "AllowedValuesPolicy"),
                Self::MaxValuePolicy => serializer.serialize_unit_variant("EvaluatorType", 1u32, "MaxValuePolicy"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result of a policy set evaluation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicySetResult {
    #[doc = "A value indicating whether this policy set evaluation has discovered violations."]
    #[serde(rename = "hasError", default, skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,
    #[doc = "The list of policy violations."]
    #[serde(rename = "policyViolations", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_violations: Vec<PolicyViolation>,
}
impl PolicySetResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy violation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyViolation {
    #[doc = "The code of the policy violation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The message of the policy violation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl PolicyViolation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseWithContinuationArtifactSource {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ArtifactSource>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResponseWithContinuationArtifactSource {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResponseWithContinuationArtifactSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseWithContinuationArtifact {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Artifact>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResponseWithContinuationArtifact {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResponseWithContinuationArtifact {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseWithContinuationCostInsight {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CostInsight>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResponseWithContinuationCostInsight {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResponseWithContinuationCostInsight {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseWithContinuationCost {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cost>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResponseWithContinuationCost {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResponseWithContinuationCost {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseWithContinuationCustomImage {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CustomImage>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResponseWithContinuationCustomImage {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResponseWithContinuationCustomImage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseWithContinuationFormula {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Formula>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResponseWithContinuationFormula {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResponseWithContinuationFormula {
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
pub struct ResponseWithContinuationLabVhd {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LabVhd>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResponseWithContinuationLabVhd {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResponseWithContinuationLabVhd {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseWithContinuationLabVirtualMachine {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LabVirtualMachine>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResponseWithContinuationLabVirtualMachine {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResponseWithContinuationLabVirtualMachine {
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
pub struct ResponseWithContinuationPolicy {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Policy>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResponseWithContinuationPolicy {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResponseWithContinuationPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseWithContinuationSchedule {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Schedule>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResponseWithContinuationSchedule {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResponseWithContinuationSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseWithContinuationVirtualNetwork {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetwork>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResponseWithContinuationVirtualNetwork {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResponseWithContinuationVirtualNetwork {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Schedule {
    #[doc = "Properties of a schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduleProperties>,
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
impl Schedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleProperties {
    #[doc = "The status of the schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<schedule_properties::Status>,
    #[doc = "The task type of the schedule."]
    #[serde(rename = "taskType", default, skip_serializing_if = "Option::is_none")]
    pub task_type: Option<schedule_properties::TaskType>,
    #[doc = "Properties of a weekly schedule."]
    #[serde(rename = "weeklyRecurrence", default, skip_serializing_if = "Option::is_none")]
    pub weekly_recurrence: Option<WeekDetails>,
    #[doc = "Properties of a daily schedule."]
    #[serde(rename = "dailyRecurrence", default, skip_serializing_if = "Option::is_none")]
    pub daily_recurrence: Option<DayDetails>,
    #[doc = "Properties of an hourly schedule."]
    #[serde(rename = "hourlyRecurrence", default, skip_serializing_if = "Option::is_none")]
    pub hourly_recurrence: Option<HourDetails>,
    #[doc = "The time zone id."]
    #[serde(rename = "timeZoneId", default, skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ScheduleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod schedule_properties {
    use super::*;
    #[doc = "The status of the schedule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The task type of the schedule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TaskType")]
    pub enum TaskType {
        LabVmsShutdownTask,
        LabVmsStartupTask,
        LabBillingTask,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TaskType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TaskType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TaskType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::LabVmsShutdownTask => serializer.serialize_unit_variant("TaskType", 0u32, "LabVmsShutdownTask"),
                Self::LabVmsStartupTask => serializer.serialize_unit_variant("TaskType", 1u32, "LabVmsStartupTask"),
                Self::LabBillingTask => serializer.serialize_unit_variant("TaskType", 2u32, "LabBillingTask"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Subnet {
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "labSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub lab_subnet_name: Option<String>,
    #[serde(rename = "allowPublicIp", default, skip_serializing_if = "Option::is_none")]
    pub allow_public_ip: Option<subnet::AllowPublicIp>,
}
impl Subnet {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subnet {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AllowPublicIp")]
    pub enum AllowPublicIp {
        Default,
        Deny,
        Allow,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AllowPublicIp {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AllowPublicIp {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AllowPublicIp {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("AllowPublicIp", 0u32, "Default"),
                Self::Deny => serializer.serialize_unit_variant("AllowPublicIp", 1u32, "Deny"),
                Self::Allow => serializer.serialize_unit_variant("AllowPublicIp", 2u32, "Allow"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Property overrides on a subnet of a virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubnetOverride {
    #[doc = "The resource identifier of the subnet."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The name given to the subnet within the lab."]
    #[serde(rename = "labSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub lab_subnet_name: Option<String>,
    #[doc = "Indicates whether this subnet can be used during virtual machine creation."]
    #[serde(rename = "useInVmCreationPermission", default, skip_serializing_if = "Option::is_none")]
    pub use_in_vm_creation_permission: Option<subnet_override::UseInVmCreationPermission>,
    #[doc = "Indicates whether public IP addresses can be assigned to virtual machines on this subnet."]
    #[serde(rename = "usePublicIpAddressPermission", default, skip_serializing_if = "Option::is_none")]
    pub use_public_ip_address_permission: Option<subnet_override::UsePublicIpAddressPermission>,
}
impl SubnetOverride {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subnet_override {
    use super::*;
    #[doc = "Indicates whether this subnet can be used during virtual machine creation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UseInVmCreationPermission")]
    pub enum UseInVmCreationPermission {
        Default,
        Deny,
        Allow,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UseInVmCreationPermission {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UseInVmCreationPermission {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UseInVmCreationPermission {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("UseInVmCreationPermission", 0u32, "Default"),
                Self::Deny => serializer.serialize_unit_variant("UseInVmCreationPermission", 1u32, "Deny"),
                Self::Allow => serializer.serialize_unit_variant("UseInVmCreationPermission", 2u32, "Allow"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates whether public IP addresses can be assigned to virtual machines on this subnet."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UsePublicIpAddressPermission")]
    pub enum UsePublicIpAddressPermission {
        Default,
        Deny,
        Allow,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UsePublicIpAddressPermission {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UsePublicIpAddressPermission {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UsePublicIpAddressPermission {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("UsePublicIpAddressPermission", 0u32, "Default"),
                Self::Deny => serializer.serialize_unit_variant("UsePublicIpAddressPermission", 1u32, "Deny"),
                Self::Allow => serializer.serialize_unit_variant("UsePublicIpAddressPermission", 2u32, "Allow"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionNotification {
    #[serde(rename = "registrationDate", default, skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<subscription_notification::State>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionNotificationProperties>,
}
impl SubscriptionNotification {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription_notification {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        NotDefined,
        Registered,
        Unregistered,
        Warned,
        Suspended,
        Deleted,
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
                Self::NotDefined => serializer.serialize_unit_variant("State", 0u32, "NotDefined"),
                Self::Registered => serializer.serialize_unit_variant("State", 1u32, "Registered"),
                Self::Unregistered => serializer.serialize_unit_variant("State", 2u32, "Unregistered"),
                Self::Warned => serializer.serialize_unit_variant("State", 3u32, "Warned"),
                Self::Suspended => serializer.serialize_unit_variant("State", 4u32, "Suspended"),
                Self::Deleted => serializer.serialize_unit_variant("State", 5u32, "Deleted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionNotificationProperties {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl SubscriptionNotificationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmCostProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
}
impl VmCostProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetwork {
    #[doc = "Properties of a virtual network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkProperties>,
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
impl VirtualNetwork {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkProperties {
    #[doc = "The allowed subnets of the virtual network."]
    #[serde(rename = "allowedSubnets", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_subnets: Vec<Subnet>,
    #[doc = "The description of the virtual network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The Microsoft.Network resource identifier of the virtual network."]
    #[serde(rename = "externalProviderResourceId", default, skip_serializing_if = "Option::is_none")]
    pub external_provider_resource_id: Option<String>,
    #[doc = "The subnet overrides of the virtual network."]
    #[serde(rename = "subnetOverrides", default, skip_serializing_if = "Vec::is_empty")]
    pub subnet_overrides: Vec<SubnetOverride>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl VirtualNetworkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a weekly schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WeekDetails {
    #[doc = "The days of the week."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weekdays: Vec<String>,
    #[doc = "The time of the day."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}
impl WeekDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a Windows OS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsOsInfo {
    #[doc = "The state of the Windows OS."]
    #[serde(rename = "windowsOsState", default, skip_serializing_if = "Option::is_none")]
    pub windows_os_state: Option<windows_os_info::WindowsOsState>,
}
impl WindowsOsInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod windows_os_info {
    use super::*;
    #[doc = "The state of the Windows OS."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "WindowsOsState")]
    pub enum WindowsOsState {
        NonSysprepped,
        SysprepRequested,
        SysprepApplied,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for WindowsOsState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for WindowsOsState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for WindowsOsState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NonSysprepped => serializer.serialize_unit_variant("WindowsOsState", 0u32, "NonSysprepped"),
                Self::SysprepRequested => serializer.serialize_unit_variant("WindowsOsState", 1u32, "SysprepRequested"),
                Self::SysprepApplied => serializer.serialize_unit_variant("WindowsOsState", 2u32, "SysprepApplied"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
