#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An single request in a batch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchQueryRequest {
    #[doc = "The error details."]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    #[doc = "The Analytics query. Learn more about the [Analytics query syntax](https://azure.microsoft.com/documentation/articles/app-insights-analytics-reference/)"]
    pub body: QueryBody,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<batch_query_request::Path>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<batch_query_request::Method>,
    #[doc = "Workspace Id to be included in the query"]
    pub workspace: String,
}
impl BatchQueryRequest {
    pub fn new(id: String, body: QueryBody, workspace: String) -> Self {
        Self {
            id,
            headers: None,
            body,
            path: None,
            method: None,
            workspace,
        }
    }
}
pub mod batch_query_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Path {
        #[serde(rename = "/query")]
        Query,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Method {
        #[serde(rename = "POST")]
        Post,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchQueryResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    #[doc = "Contains the tables, columns & rows resulting from a query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<BatchQueryResults>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
}
impl BatchQueryResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the tables, columns & rows resulting from a query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchQueryResults {
    #[doc = "The list of tables, columns and rows."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<Table>,
    #[doc = "Statistics represented in JSON format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<serde_json::Value>,
    #[doc = "Visualization data in JSON format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub render: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorInfo>,
}
impl BatchQueryResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An array of requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchRequest {
    #[doc = "An single request in a batch."]
    pub requests: Vec<BatchQueryRequest>,
}
impl BatchRequest {
    pub fn new(requests: Vec<BatchQueryRequest>) -> Self {
        Self { requests }
    }
}
#[doc = "Response to a batch query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchResponse {
    #[doc = "An array of responses corresponding to each individual request in a batch."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub responses: Vec<BatchQueryResponse>,
}
impl BatchResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A column in a table."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Column {
    #[doc = "The name of this column."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The data type of this column."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<LogsColumnType>,
}
impl Column {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[doc = "The error's code."]
    pub code: String,
    #[doc = "A human readable error message."]
    pub message: String,
    #[doc = "Indicates which property in the request is responsible for the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Indicates which value in 'target' is responsible for the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Indicates resources which were responsible for the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    #[doc = "Additional properties that can be provided on the error details object"]
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
}
impl ErrorDetail {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            value: None,
            resources: Vec::new(),
            additional_properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorInfo {
    #[doc = "A machine readable error code."]
    pub code: String,
    #[doc = "A human readable error message."]
    pub message: String,
    #[doc = "error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Box<Option<ErrorInfo>>,
    #[doc = "Additional properties that can be provided on the error info object"]
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
}
impl ErrorInfo {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            details: Vec::new(),
            innererror: Box::new(None),
            additional_properties: None,
        }
    }
}
#[doc = "Contains details when the response code indicates an error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorInfo,
}
impl ErrorResponse {
    pub fn new(error: ErrorInfo) -> Self {
        Self { error }
    }
}
#[doc = "The data type of this column."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LogsColumnType")]
pub enum LogsColumnType {
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "datetime")]
    Datetime,
    #[serde(rename = "dynamic")]
    Dynamic,
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "real")]
    Real,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "guid")]
    Guid,
    #[serde(rename = "decimal")]
    Decimal,
    #[serde(rename = "timespan")]
    Timespan,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LogsColumnType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LogsColumnType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LogsColumnType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Bool => serializer.serialize_unit_variant("LogsColumnType", 0u32, "bool"),
            Self::Datetime => serializer.serialize_unit_variant("LogsColumnType", 1u32, "datetime"),
            Self::Dynamic => serializer.serialize_unit_variant("LogsColumnType", 2u32, "dynamic"),
            Self::Int => serializer.serialize_unit_variant("LogsColumnType", 3u32, "int"),
            Self::Long => serializer.serialize_unit_variant("LogsColumnType", 4u32, "long"),
            Self::Real => serializer.serialize_unit_variant("LogsColumnType", 5u32, "real"),
            Self::String => serializer.serialize_unit_variant("LogsColumnType", 6u32, "string"),
            Self::Guid => serializer.serialize_unit_variant("LogsColumnType", 7u32, "guid"),
            Self::Decimal => serializer.serialize_unit_variant("LogsColumnType", 8u32, "decimal"),
            Self::Timespan => serializer.serialize_unit_variant("LogsColumnType", 9u32, "timespan"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Application Insights apps that were part of the metadata request and that the user has access to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataApplication {
    #[doc = "The ID of the Application Insights app."]
    pub id: String,
    #[doc = "The ARM resource ID of the Application Insights app."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "The name of the Application Insights app."]
    pub name: String,
    #[doc = "The Azure region of the Application Insights app."]
    pub region: String,
    #[doc = "The related metadata items for the Application Insights app."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<metadata_application::Related>,
}
impl MetadataApplication {
    pub fn new(id: String, resource_id: String, name: String, region: String) -> Self {
        Self {
            id,
            resource_id,
            name,
            region,
            related: None,
        }
    }
}
pub mod metadata_application {
    use super::*;
    #[doc = "The related metadata items for the Application Insights app."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Related {
        #[doc = "The related tables for the Application Insights app."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tables: Vec<String>,
        #[doc = "The related functions for the Application Insights app."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub functions: Vec<String>,
    }
    impl Related {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Categories are used to group other metadata entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataCategory {
    #[doc = "The ID of the category"]
    pub id: String,
    #[doc = "The display name of the category"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The description of the category"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The related metadata items for the category"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<metadata_category::Related>,
}
impl MetadataCategory {
    pub fn new(id: String, display_name: String) -> Self {
        Self {
            id,
            display_name,
            description: None,
            related: None,
        }
    }
}
pub mod metadata_category {
    use super::*;
    #[doc = "The related metadata items for the category"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Related {
        #[doc = "The tables related to the category"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tables: Vec<String>,
        #[doc = "The functions related to the category"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub functions: Vec<String>,
        #[doc = "The resource types related to the category"]
        #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
        pub resource_types: Vec<String>,
        #[doc = "The saved queries related to the category"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub queries: Vec<String>,
        #[doc = "The Log Analytics solutions related to the category"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub solutions: Vec<String>,
    }
    impl Related {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Functions are stored Kusto queries that can be specified as part of queries by using their name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataFunction {
    #[doc = "The ID of the function."]
    pub id: String,
    #[doc = "The name of the function, to be used in queries."]
    pub name: String,
    #[doc = "The parameters/arguments of the function, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    #[doc = "The display name of the function."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description of the function."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The KQL body of the function."]
    pub body: String,
    #[doc = "String-based key-value tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "The properties of the function."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The related metadata items for the function."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<metadata_function::Related>,
}
impl MetadataFunction {
    pub fn new(id: String, name: String, body: String) -> Self {
        Self {
            id,
            name,
            parameters: None,
            display_name: None,
            description: None,
            body,
            tags: None,
            properties: None,
            related: None,
        }
    }
}
pub mod metadata_function {
    use super::*;
    #[doc = "The related metadata items for the function."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Related {
        #[doc = "The related tables for the function."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tables: Vec<String>,
        #[doc = "The related Log Analytics solutions for the function."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub solutions: Vec<String>,
        #[doc = "The related resource types for the function."]
        #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
        pub resource_types: Vec<String>,
        #[doc = "The related categories for the function."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub categories: Vec<String>,
        #[doc = "The related workspaces for the function."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub workspaces: Vec<String>,
    }
    impl Related {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Permission information for the metadata call, includes apps/workspaces/resource the user didn't have access to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataPermissions {
    #[doc = "The permission indication for the workspaces on the metadata request."]
    pub workspaces: Vec<serde_json::Value>,
    #[doc = "The permission indication for the Azure resources on the metadata request."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<serde_json::Value>,
    #[doc = "The permission indication for the Application Insights apps on the metadata request."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applications: Vec<serde_json::Value>,
}
impl MetadataPermissions {
    pub fn new(workspaces: Vec<serde_json::Value>) -> Self {
        Self {
            workspaces,
            resources: Vec::new(),
            applications: Vec::new(),
        }
    }
}
#[doc = "Queries are stored pieces of KQL, along with a list of relevant metadata items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataQuery {
    #[doc = "The ID of the query."]
    pub id: String,
    #[doc = "The display name of the query."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description of the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The KQL body of the query."]
    pub body: String,
    #[doc = "The user defined labels associated with the query."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[doc = "String-based key-value tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "The properties of the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The related metadata items for the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<metadata_query::Related>,
}
impl MetadataQuery {
    pub fn new(id: String, body: String) -> Self {
        Self {
            id,
            display_name: None,
            description: None,
            body,
            labels: Vec::new(),
            tags: None,
            properties: None,
            related: None,
        }
    }
}
pub mod metadata_query {
    use super::*;
    #[doc = "The related metadata items for the query."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Related {
        #[doc = "The related categories for the query."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub categories: Vec<String>,
        #[doc = "The related Log Analytics solutions for the query."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub solutions: Vec<String>,
        #[doc = "The related resource types for the query."]
        #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
        pub resource_types: Vec<String>,
        #[doc = "The related tables for the query."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tables: Vec<String>,
    }
    impl Related {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Azure resources that were part of the metadata request and that the user has access to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataResource {}
impl MetadataResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata about types of Azure resources, containing relevant tables, functions, etc."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataResourceType {
    #[doc = "The ID of the resource-type"]
    pub id: String,
    #[doc = "The type of the resource-type"]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The display name of the resource-type"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description of the resource-type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The user-defined labels of the resource-type"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[doc = "String-based key-value tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "The properties of the resource-type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The related metadata items for the resource-type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<metadata_resource_type::Related>,
}
impl MetadataResourceType {
    pub fn new(id: String, type_: String) -> Self {
        Self {
            id,
            type_,
            display_name: None,
            description: None,
            labels: Vec::new(),
            tags: None,
            properties: None,
            related: None,
        }
    }
}
pub mod metadata_resource_type {
    use super::*;
    #[doc = "The related metadata items for the resource-type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Related {
        #[doc = "The tables related to the resource-type"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tables: Vec<String>,
        #[doc = "The functions related to the resource-type"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub functions: Vec<String>,
        #[doc = "The categories related to the resource-type"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub categories: Vec<String>,
        #[doc = "The queries related to the resource-type"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub queries: Vec<String>,
        #[doc = "The Log Analytics workspaces related to the resource-type"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub workspaces: Vec<String>,
        #[doc = "The Azure resources related to the resource-type"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub resources: Vec<String>,
    }
    impl Related {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The metadata response for the app, including available tables, etc."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataResults {
    #[doc = "The list of categories that are referenced in this metadata response."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<MetadataCategory>,
    #[doc = "The list of resource types that are referenced in this metadata response."]
    #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_types: Vec<MetadataResourceType>,
    #[doc = "The list of Log Analytics solutions installed on the workspace."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub solutions: Vec<MetadataSolution>,
    #[doc = "The list of tables and columns that comprise the schema of the workspace."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<MetadataTable>,
    #[doc = "The list of functions stored on the workspace, or introduced by solutions etc."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<MetadataFunction>,
    #[doc = "The list of saved queries stored on the workspace, or introduced by solutions, resource types, etc."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub queries: Vec<MetadataQuery>,
    #[doc = "The list of Application Insights apps that were referenced in the metadata request."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applications: Vec<MetadataApplication>,
    #[doc = "The list of Log Analytics workspaces that were referenced in the metadata request."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub workspaces: Vec<MetadataWorkspace>,
    #[doc = "The list of Azure resources that were referenced in the metadata request."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<MetadataResource>,
    #[doc = "The list of permission rules that affected the metadata request."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<MetadataPermissions>,
}
impl MetadataResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Solutions can group tables and functions that are associated with a certain Azure Log Analytics offering."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataSolution {
    #[doc = "The ID of the Log Analytics solution"]
    pub id: String,
    #[doc = "The name of the Log Analytics solution"]
    pub name: String,
    #[doc = "The display name of the Log Analytics solution"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description of the Log Analytics solution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "String-based key-value tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "The properties of the Log Analytics solution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The related metadata items for the Log Analytics solution"]
    pub related: metadata_solution::Related,
}
impl MetadataSolution {
    pub fn new(id: String, name: String, related: metadata_solution::Related) -> Self {
        Self {
            id,
            name,
            display_name: None,
            description: None,
            tags: None,
            properties: None,
            related,
        }
    }
}
pub mod metadata_solution {
    use super::*;
    #[doc = "The related metadata items for the Log Analytics solution"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Related {
        #[doc = "The tables related to the Log Analytics solution"]
        pub tables: Vec<String>,
        #[doc = "The functions related to the Log Analytics solution"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub functions: Vec<String>,
        #[doc = "The categories related to the Log Analytics solution"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub categories: Vec<String>,
        #[doc = "The saved queries related to the Log Analytics solution"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub queries: Vec<String>,
        #[doc = "The Workspaces referenced in the metadata request that are related to the Log Analytics solution"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub workspaces: Vec<String>,
    }
    impl Related {
        pub fn new(tables: Vec<String>) -> Self {
            Self {
                tables,
                functions: Vec::new(),
                categories: Vec::new(),
                queries: Vec::new(),
                workspaces: Vec::new(),
            }
        }
    }
}
#[doc = "Tables are part of the workspace schema, and contain a list of columns and a reference to other relevant metadata items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataTable {
    #[doc = "The ID of the table"]
    pub id: String,
    #[doc = "The name of the table"]
    pub name: String,
    #[doc = "The description of the table"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The column associated with the timespan query parameter for the table"]
    #[serde(rename = "timespanColumn", default, skip_serializing_if = "Option::is_none")]
    pub timespan_column: Option<String>,
    #[doc = "The user defined labels of the table"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[doc = "String-based key-value tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "The properties of the table"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The list of columns defined on the table"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<serde_json::Value>,
    #[doc = "The related metadata items for the table"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<metadata_table::Related>,
}
impl MetadataTable {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            description: None,
            timespan_column: None,
            labels: Vec::new(),
            tags: None,
            properties: None,
            columns: Vec::new(),
            related: None,
        }
    }
}
pub mod metadata_table {
    use super::*;
    #[doc = "The related metadata items for the table"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Related {
        #[doc = "The related categories for the table"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub categories: Vec<String>,
        #[doc = "The related Log Analytics solutions for the table"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub solutions: Vec<String>,
        #[doc = "The related resource types for the table"]
        #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
        pub resource_types: Vec<String>,
        #[doc = "The related Log Analytics workspaces for the table"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub workspaces: Vec<String>,
        #[doc = "The related functions for the table"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub functions: Vec<String>,
        #[doc = "The related saved queries for the table"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub queries: Vec<String>,
    }
    impl Related {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Log Analytics workspaces that were part of the metadata request and that the user has access to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataWorkspace {
    #[doc = "The ID of the Log Analytics workspace."]
    pub id: String,
    #[doc = "The ARM resource ID of the Log Analytics workspace."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "The name of the Log Analytics workspace."]
    pub name: String,
    #[doc = "The Azure region of the Log Analytics workspace."]
    pub region: String,
    #[doc = "The related metadata items for the Log Analytics workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<metadata_workspace::Related>,
}
impl MetadataWorkspace {
    pub fn new(id: String, resource_id: String, name: String, region: String) -> Self {
        Self {
            id,
            resource_id,
            name,
            region,
            related: None,
        }
    }
}
pub mod metadata_workspace {
    use super::*;
    #[doc = "The related metadata items for the Log Analytics workspace."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Related {
        #[doc = "The related tables for the Log Analytics workspace."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tables: Vec<String>,
        #[doc = "The related Log Analytics solutions for the Log Analytics workspace."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub solutions: Vec<String>,
        #[doc = "The related resource types for the Log Analytics workspace."]
        #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
        pub resource_types: Vec<String>,
        #[doc = "The related functions for the Log Analytics workspace."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub functions: Vec<String>,
        #[doc = "The related Azure resources for the Log Analytics workspace."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub resources: Vec<String>,
    }
    impl Related {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The Analytics query. Learn more about the [Analytics query syntax](https://azure.microsoft.com/documentation/articles/app-insights-analytics-reference/)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryBody {
    #[doc = "The Analytics query. Learn more about the [Analytics query syntax](https://azure.microsoft.com/documentation/articles/app-insights-analytics-reference/)"]
    pub query: QueryParam,
    #[doc = "Optional. The timespan over which to query data. This is an ISO8601 time period value.  This timespan is applied in addition to any that are specified in the query expression."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timespan: Option<TimespanParam>,
    #[doc = "Workspace names to include in cross-workspace queries."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<WorkspacesParam>,
}
impl QueryBody {
    pub fn new(query: QueryParam) -> Self {
        Self {
            query,
            timespan: None,
            workspaces: None,
        }
    }
}
pub type QueryParam = String;
#[doc = "Contains the tables, columns & rows resulting from a query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResults {
    #[doc = "The list of tables, columns and rows."]
    pub tables: Vec<Table>,
    #[doc = "Statistics represented in JSON format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<serde_json::Value>,
    #[doc = "Visualization data in JSON format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub render: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorInfo>,
}
impl QueryResults {
    pub fn new(tables: Vec<Table>) -> Self {
        Self {
            tables,
            statistics: None,
            render: None,
            error: None,
        }
    }
}
#[doc = "Contains the columns and rows for one table in a query response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Table {
    #[doc = "The name of the table."]
    pub name: String,
    #[doc = "The list of columns in this table."]
    pub columns: Vec<Column>,
    #[doc = "The resulting rows from this query."]
    pub rows: Vec<Vec<serde_json::Value>>,
}
impl Table {
    pub fn new(name: String, columns: Vec<Column>, rows: Vec<Vec<serde_json::Value>>) -> Self {
        Self { name, columns, rows }
    }
}
#[doc = "String-based key-value tags"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {}
impl Tags {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type TimespanParam = String;
pub type WorkspacesParam = Vec<String>;
