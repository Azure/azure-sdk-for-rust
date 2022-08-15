#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl CloudErrorBody {
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
#[doc = "The resource of the configuration or data needed to onboard the machine to MDE"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MdeOnboardingData {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the MDE configuration or data parameter needed to onboard the machine to MDE"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MdeOnboardingDataProperties>,
}
impl MdeOnboardingData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of all MDE onboarding data resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MdeOnboardingDataList {
    #[doc = "List of the resources of the configuration or data needed to onboard the machine to MDE"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MdeOnboardingData>,
}
impl MdeOnboardingDataList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the MDE configuration or data parameter needed to onboard the machine to MDE"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MdeOnboardingDataProperties {
    #[doc = "The onboarding package used to onboard Windows machines to MDE, coded in base64. This can also be used for onboarding using the dedicated VM Extension"]
    #[serde(rename = "onboardingPackageWindows", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_package_windows: Option<String>,
    #[doc = "The onboarding package used to onboard Linux machines to MDE, coded in base64. This can also be used for onboarding using the dedicated VM Extension"]
    #[serde(rename = "onboardingPackageLinux", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_package_linux: Option<String>,
}
impl MdeOnboardingDataProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
