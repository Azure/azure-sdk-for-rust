#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "EnterpriseKnowledgeGraph resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnterpriseKnowledgeGraph {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The parameters to provide for the EnterpriseKnowledgeGraph."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnterpriseKnowledgeGraphProperties>,
}
impl EnterpriseKnowledgeGraph {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters to provide for the EnterpriseKnowledgeGraph."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnterpriseKnowledgeGraphProperties {
    #[doc = "The description of the EnterpriseKnowledgeGraph"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Specifies the metadata  of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The state of EnterpriseKnowledgeGraph provisioning"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<enterprise_knowledge_graph_properties::ProvisioningState>,
}
impl EnterpriseKnowledgeGraphProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod enterprise_knowledge_graph_properties {
    use super::*;
    #[doc = "The state of EnterpriseKnowledgeGraph provisioning"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Deleting,
        Failed,
        Succeeded,
    }
}
#[doc = "The list of  EnterpriseKnowledgeGraph service operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnterpriseKnowledgeGraphResponseList {
    #[doc = "The link used to get the next page of EnterpriseKnowledgeGraph service resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Gets the list of EnterpriseKnowledgeGraph service results and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EnterpriseKnowledgeGraph>,
}
impl azure_core::Continuable for EnterpriseKnowledgeGraphResponseList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EnterpriseKnowledgeGraphResponseList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "EnterpriseKnowledgeGraph Service error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "EnterpriseKnowledgeGraph Service error body."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorBody>,
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
#[doc = "EnterpriseKnowledgeGraph Service error body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorBody {
    #[doc = "error code"]
    pub code: String,
    #[doc = "error message"]
    pub message: String,
}
impl ErrorBody {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}
#[doc = "The operation supported by EnterpriseKnowledgeGraph Service Management."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayInfo {
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The action that users can perform, based on their permission level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Service provider: Microsoft EnterpriseKnowledgeGraph Service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl OperationDisplayInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The operations supported by EnterpriseKnowledgeGraph Service Management."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntity {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The operation supported by EnterpriseKnowledgeGraph Service Management."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
    #[doc = "The origin of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Additional properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OperationEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of EnterpriseKnowledgeGraph service operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntityListResult {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationEntity>,
}
impl azure_core::Continuable for OperationEntityListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationEntityListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Specifies the resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Specifies the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Contains resource tags defined as key/value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The SKU of the EnterpriseKnowledgeGraph service account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SKU of the EnterpriseKnowledgeGraph service account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of SKU."]
    pub name: SkuName,
}
impl Sku {
    pub fn new(name: SkuName) -> Self {
        Self { name }
    }
}
#[doc = "The name of SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SkuName")]
pub enum SkuName {
    F0,
    S1,
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
            Self::F0 => serializer.serialize_unit_variant("SkuName", 0u32, "F0"),
            Self::S1 => serializer.serialize_unit_variant("SkuName", 1u32, "S1"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
