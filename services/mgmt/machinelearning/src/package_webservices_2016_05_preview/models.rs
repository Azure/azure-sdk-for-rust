#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Information about an asset associated with the web service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetItem {
    #[doc = "Asset's friendly name."]
    pub name: String,
    #[doc = "Asset's Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Asset's type."]
    #[serde(rename = "type")]
    pub type_: asset_item::Type,
    #[doc = "Describes the access location for a web service asset."]
    #[serde(rename = "locationInfo")]
    pub location_info: AssetLocation,
    #[doc = "Information about the asset's input ports."]
    #[serde(rename = "inputPorts", default, skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<serde_json::Value>,
    #[doc = "Information about the asset's output ports."]
    #[serde(rename = "outputPorts", default, skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<serde_json::Value>,
    #[doc = "If the asset is a custom module, this holds the module's metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "If the asset is a custom module, this holds the module's parameters."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ModuleAssetParameter>,
}
impl AssetItem {
    pub fn new(name: String, type_: asset_item::Type, location_info: AssetLocation) -> Self {
        Self {
            name,
            id: None,
            type_,
            location_info,
            input_ports: None,
            output_ports: None,
            metadata: None,
            parameters: Vec::new(),
        }
    }
}
pub mod asset_item {
    use super::*;
    #[doc = "Asset's type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Module,
        Resource,
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
                Self::Module => serializer.serialize_unit_variant("Type", 0u32, "Module"),
                Self::Resource => serializer.serialize_unit_variant("Type", 1u32, "Resource"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes the access location for a web service asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetLocation {
    #[doc = "The URI where the asset is accessible from, (e.g. aml://abc for system assets or https://xyz for user assets"]
    pub uri: String,
    #[doc = "Access credentials for the asset, if applicable (e.g. asset specified by storage account connection string + blob URI)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
}
impl AssetLocation {
    pub fn new(uri: String) -> Self {
        Self { uri, credentials: None }
    }
}
#[doc = "Swagger 2.0 schema for a column within the data table representing a web service input or output. See Swagger specification: http://swagger.io/specification/"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColumnSpecification {
    #[doc = "Data type of the column."]
    #[serde(rename = "type")]
    pub type_: column_specification::Type,
    #[doc = "Additional format information for the data type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<column_specification::Format>,
    #[doc = "If the data type is categorical, this provides the list of accepted categories."]
    #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
    pub enum_: Vec<serde_json::Value>,
    #[doc = "Flag indicating if the type supports null values or not."]
    #[serde(rename = "x-ms-isnullable", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_isnullable: Option<bool>,
    #[doc = "Flag indicating whether the categories are treated as an ordered set or not, if this is a categorical column."]
    #[serde(rename = "x-ms-isordered", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_isordered: Option<bool>,
}
impl ColumnSpecification {
    pub fn new(type_: column_specification::Type) -> Self {
        Self {
            type_,
            format: None,
            enum_: Vec::new(),
            x_ms_isnullable: None,
            x_ms_isordered: None,
        }
    }
}
pub mod column_specification {
    use super::*;
    #[doc = "Data type of the column."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Boolean,
        Integer,
        Number,
        String,
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
                Self::Boolean => serializer.serialize_unit_variant("Type", 0u32, "Boolean"),
                Self::Integer => serializer.serialize_unit_variant("Type", 1u32, "Integer"),
                Self::Number => serializer.serialize_unit_variant("Type", 2u32, "Number"),
                Self::String => serializer.serialize_unit_variant("Type", 3u32, "String"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Additional format information for the data type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Format")]
    pub enum Format {
        Byte,
        Char,
        Complex64,
        Complex128,
        #[serde(rename = "Date-time")]
        DateTime,
        #[serde(rename = "Date-timeOffset")]
        DateTimeOffset,
        Double,
        Duration,
        Float,
        Int8,
        Int16,
        Int32,
        Int64,
        Uint8,
        Uint16,
        Uint32,
        Uint64,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Format {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Format {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Format {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Byte => serializer.serialize_unit_variant("Format", 0u32, "Byte"),
                Self::Char => serializer.serialize_unit_variant("Format", 1u32, "Char"),
                Self::Complex64 => serializer.serialize_unit_variant("Format", 2u32, "Complex64"),
                Self::Complex128 => serializer.serialize_unit_variant("Format", 3u32, "Complex128"),
                Self::DateTime => serializer.serialize_unit_variant("Format", 4u32, "Date-time"),
                Self::DateTimeOffset => serializer.serialize_unit_variant("Format", 5u32, "Date-timeOffset"),
                Self::Double => serializer.serialize_unit_variant("Format", 6u32, "Double"),
                Self::Duration => serializer.serialize_unit_variant("Format", 7u32, "Duration"),
                Self::Float => serializer.serialize_unit_variant("Format", 8u32, "Float"),
                Self::Int8 => serializer.serialize_unit_variant("Format", 9u32, "Int8"),
                Self::Int16 => serializer.serialize_unit_variant("Format", 10u32, "Int16"),
                Self::Int32 => serializer.serialize_unit_variant("Format", 11u32, "Int32"),
                Self::Int64 => serializer.serialize_unit_variant("Format", 12u32, "Int64"),
                Self::Uint8 => serializer.serialize_unit_variant("Format", 13u32, "Uint8"),
                Self::Uint16 => serializer.serialize_unit_variant("Format", 14u32, "Uint16"),
                Self::Uint32 => serializer.serialize_unit_variant("Format", 15u32, "Uint32"),
                Self::Uint64 => serializer.serialize_unit_variant("Format", 16u32, "Uint64"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Information about the machine learning commitment plan associated with the web service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentPlan {
    #[doc = "Specifies the Azure Resource Manager ID of the commitment plan associated with the web service."]
    pub id: String,
}
impl CommitmentPlan {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Diagnostics settings for an Azure ML web service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticsConfiguration {
    #[doc = "Specifies the verbosity of the diagnostic output. Valid values are: None - disables tracing; Error - collects only error (stderr) traces; All - collects all traces (stdout and stderr)."]
    pub level: diagnostics_configuration::Level,
    #[doc = "Specifies the date and time when the logging will cease. If null, diagnostic collection is not time limited."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub expiry: Option<time::OffsetDateTime>,
}
impl DiagnosticsConfiguration {
    pub fn new(level: diagnostics_configuration::Level) -> Self {
        Self { level, expiry: None }
    }
}
pub mod diagnostics_configuration {
    use super::*;
    #[doc = "Specifies the verbosity of the diagnostic output. Valid values are: None - disables tracing; Error - collects only error (stderr) traces; All - collects all traces (stdout and stderr)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Level")]
    pub enum Level {
        None,
        Error,
        All,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Level {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Level {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Level {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Level", 0u32, "None"),
                Self::Error => serializer.serialize_unit_variant("Level", 1u32, "Error"),
                Self::All => serializer.serialize_unit_variant("Level", 2u32, "All"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Sample input data for the service's input(s)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExampleRequest {
    #[doc = "Sample input data for the web service's input(s) given as an input name to sample input values matrix map."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    #[doc = "Sample input data for the web service's global parameters"]
    #[serde(rename = "globalParameters", default, skip_serializing_if = "Option::is_none")]
    pub global_parameters: Option<serde_json::Value>,
}
impl ExampleRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines an edge within the web service's graph."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphEdge {
    #[doc = "The source graph node's identifier."]
    #[serde(rename = "sourceNodeId", default, skip_serializing_if = "Option::is_none")]
    pub source_node_id: Option<String>,
    #[doc = "The identifier of the source node's port that the edge connects from."]
    #[serde(rename = "sourcePortId", default, skip_serializing_if = "Option::is_none")]
    pub source_port_id: Option<String>,
    #[doc = "The destination graph node's identifier."]
    #[serde(rename = "targetNodeId", default, skip_serializing_if = "Option::is_none")]
    pub target_node_id: Option<String>,
    #[doc = "The identifier of the destination node's port that the edge connects into."]
    #[serde(rename = "targetPortId", default, skip_serializing_if = "Option::is_none")]
    pub target_port_id: Option<String>,
}
impl GraphEdge {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies a node in the web service graph. The node can either be an input, output or asset node, so only one of the corresponding id properties is populated at any given time."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphNode {
    #[doc = "The id of the asset represented by this node."]
    #[serde(rename = "assetId", default, skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[doc = "The id of the input element represented by this node."]
    #[serde(rename = "inputId", default, skip_serializing_if = "Option::is_none")]
    pub input_id: Option<String>,
    #[doc = "The id of the output element represented by this node."]
    #[serde(rename = "outputId", default, skip_serializing_if = "Option::is_none")]
    pub output_id: Option<String>,
    #[doc = "If applicable, parameters of the node. Global graph parameters map into these, with values set at runtime."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl GraphNode {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the graph of modules making up the machine learning solution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphPackage {
    #[doc = "The set of nodes making up the graph, provided as a nodeId to GraphNode map"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<serde_json::Value>,
    #[doc = "The list of edges making up the graph."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edges: Vec<GraphEdge>,
    #[doc = "The collection of global parameters for the graph, given as a global parameter name to GraphParameter map. Each parameter here has a 1:1 match with the global parameters values map declared at the WebServiceProperties level."]
    #[serde(rename = "graphParameters", default, skip_serializing_if = "Option::is_none")]
    pub graph_parameters: Option<serde_json::Value>,
}
impl GraphPackage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a global parameter in the graph."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphParameter {
    #[doc = "Description of this graph parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Graph parameter's type."]
    #[serde(rename = "type")]
    pub type_: graph_parameter::Type,
    #[doc = "Association links for this parameter to nodes in the graph."]
    pub links: Vec<GraphParameterLink>,
}
impl GraphParameter {
    pub fn new(type_: graph_parameter::Type, links: Vec<GraphParameterLink>) -> Self {
        Self {
            description: None,
            type_,
            links,
        }
    }
}
pub mod graph_parameter {
    use super::*;
    #[doc = "Graph parameter's type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        String,
        Int,
        Float,
        Enumerated,
        Script,
        Mode,
        Credential,
        Boolean,
        Double,
        ColumnPicker,
        ParameterRange,
        DataGatewayName,
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
                Self::String => serializer.serialize_unit_variant("Type", 0u32, "String"),
                Self::Int => serializer.serialize_unit_variant("Type", 1u32, "Int"),
                Self::Float => serializer.serialize_unit_variant("Type", 2u32, "Float"),
                Self::Enumerated => serializer.serialize_unit_variant("Type", 3u32, "Enumerated"),
                Self::Script => serializer.serialize_unit_variant("Type", 4u32, "Script"),
                Self::Mode => serializer.serialize_unit_variant("Type", 5u32, "Mode"),
                Self::Credential => serializer.serialize_unit_variant("Type", 6u32, "Credential"),
                Self::Boolean => serializer.serialize_unit_variant("Type", 7u32, "Boolean"),
                Self::Double => serializer.serialize_unit_variant("Type", 8u32, "Double"),
                Self::ColumnPicker => serializer.serialize_unit_variant("Type", 9u32, "ColumnPicker"),
                Self::ParameterRange => serializer.serialize_unit_variant("Type", 10u32, "ParameterRange"),
                Self::DataGatewayName => serializer.serialize_unit_variant("Type", 11u32, "DataGatewayName"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Association link for a graph global parameter to a node in the graph."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphParameterLink {
    #[doc = "The graph node's identifier"]
    #[serde(rename = "nodeId")]
    pub node_id: String,
    #[doc = "The identifier of the node parameter that the global parameter maps to."]
    #[serde(rename = "parameterKey")]
    pub parameter_key: String,
}
impl GraphParameterLink {
    pub fn new(node_id: String, parameter_key: String) -> Self {
        Self { node_id, parameter_key }
    }
}
#[doc = "Asset input port"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputPort {
    #[doc = "Port data type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<input_port::Type>,
}
impl InputPort {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod input_port {
    use super::*;
    #[doc = "Port data type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Dataset,
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
                Self::Dataset => serializer.serialize_unit_variant("Type", 0u32, "Dataset"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Type {
        fn default() -> Self {
            Self::Dataset
        }
    }
}
#[doc = "Information about the machine learning workspace containing the experiment that is source for the web service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineLearningWorkspace {
    #[doc = "Specifies the workspace ID of the machine learning workspace associated with the web service"]
    pub id: String,
}
impl MachineLearningWorkspace {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Nested parameter definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModeValueInfo {
    #[doc = "The interface string name for the nested parameter."]
    #[serde(rename = "interfaceString", default, skip_serializing_if = "Option::is_none")]
    pub interface_string: Option<String>,
    #[doc = "The definition of the parameter."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ModuleAssetParameter>,
}
impl ModeValueInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameter definition for a module asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModuleAssetParameter {
    #[doc = "Parameter name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Parameter type."]
    #[serde(rename = "parameterType", default, skip_serializing_if = "Option::is_none")]
    pub parameter_type: Option<String>,
    #[doc = "Definitions for nested interface parameters if this is a complex module parameter."]
    #[serde(rename = "modeValuesInfo", default, skip_serializing_if = "Option::is_none")]
    pub mode_values_info: Option<serde_json::Value>,
}
impl ModuleAssetParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Asset output port"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutputPort {
    #[doc = "Port data type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<output_port::Type>,
}
impl OutputPort {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod output_port {
    use super::*;
    #[doc = "Port data type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Dataset,
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
                Self::Dataset => serializer.serialize_unit_variant("Type", 0u32, "Dataset"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Type {
        fn default() -> Self {
            Self::Dataset
        }
    }
}
#[doc = "Paginated list of web services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginatedWebServicesList {
    #[doc = "An array of web service objects."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WebService>,
    #[doc = "A continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PaginatedWebServicesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PaginatedWebServicesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Holds the available configuration options for an Azure ML web service endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeConfiguration {
    #[doc = "Specifies the maximum concurrent calls that can be made to the web service. Minimum value: 4, Maximum value: 200."]
    #[serde(rename = "maxConcurrentCalls", default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_calls: Option<i64>,
}
impl RealtimeConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "Specifies the resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Specifies the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the location of the resource."]
    pub location: String,
    #[doc = "Specifies the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Contains resource tags defined as key/value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            location,
            type_: None,
            tags: None,
        }
    }
}
#[doc = "The swagger 2.0 schema describing the service's inputs or outputs. See Swagger specification: http://swagger.io/specification/"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceInputOutputSpecification {
    #[doc = "The title of your Swagger schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The description of the Swagger schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The type of the entity described in swagger. Always 'object'."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Specifies a collection that contains the column schema for each input or output of the web service. For more information, see the Swagger specification."]
    pub properties: serde_json::Value,
}
impl ServiceInputOutputSpecification {
    pub fn new(type_: String, properties: serde_json::Value) -> Self {
        Self {
            title: None,
            description: None,
            type_,
            properties,
        }
    }
}
#[doc = "Access information for a storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccount {
    #[doc = "Specifies the name of the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the key used to access the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
impl StorageAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The swagger 2.0 schema describing a single service input or output. See Swagger specification: http://swagger.io/specification/"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableSpecification {
    #[doc = "Swagger schema title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Swagger schema description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The type of the entity described in swagger."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The format, if 'type' is not 'object'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[doc = "The set of columns within the data table."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl TableSpecification {
    pub fn new(type_: String) -> Self {
        Self {
            title: None,
            description: None,
            type_,
            format: None,
            properties: None,
        }
    }
}
#[doc = "Instance of an Azure ML web service resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebService {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The set of properties specific to the Azure ML web service resource."]
    pub properties: WebServiceProperties,
}
impl WebService {
    pub fn new(resource: Resource, properties: WebServiceProperties) -> Self {
        Self { resource, properties }
    }
}
#[doc = "Access keys for the web service calls."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebServiceKeys {
    #[doc = "The primary access key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,
    #[doc = "The secondary access key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary: Option<String>,
}
impl WebServiceKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The set of properties specific to the Azure ML web service resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebServiceProperties {
    #[doc = "The title of the web service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The description of the web service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Read Only: The date and time when the web service was created."]
    #[serde(rename = "createdOn", with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Read Only: The date and time when the web service was last modified."]
    #[serde(rename = "modifiedOn", with = "azure_core::date::rfc3339::option")]
    pub modified_on: Option<time::OffsetDateTime>,
    #[doc = "Read Only: The provision state of the web service. Valid values are Unknown, Provisioning, Succeeded, and Failed."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<web_service_properties::ProvisioningState>,
    #[doc = "Access keys for the web service calls."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<WebServiceKeys>,
    #[doc = "When set to true, indicates that the web service is read-only and can no longer be updated or patched, only removed. Default, is false. Note: Once set to true, you cannot change its value."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "Read Only: Contains the URI of the swagger spec associated with this web service."]
    #[serde(rename = "swaggerLocation", default, skip_serializing_if = "Option::is_none")]
    pub swagger_location: Option<String>,
    #[doc = "When set to true, sample data is included in the web service's swagger definition. The default value is true."]
    #[serde(rename = "exposeSampleData", default, skip_serializing_if = "Option::is_none")]
    pub expose_sample_data: Option<bool>,
    #[doc = "Holds the available configuration options for an Azure ML web service endpoint."]
    #[serde(rename = "realtimeConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub realtime_configuration: Option<RealtimeConfiguration>,
    #[doc = "Diagnostics settings for an Azure ML web service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticsConfiguration>,
    #[doc = "Access information for a storage account."]
    #[serde(rename = "storageAccount", default, skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<StorageAccount>,
    #[doc = "Information about the machine learning workspace containing the experiment that is source for the web service."]
    #[serde(rename = "machineLearningWorkspace", default, skip_serializing_if = "Option::is_none")]
    pub machine_learning_workspace: Option<MachineLearningWorkspace>,
    #[doc = "Information about the machine learning commitment plan associated with the web service."]
    #[serde(rename = "commitmentPlan", default, skip_serializing_if = "Option::is_none")]
    pub commitment_plan: Option<CommitmentPlan>,
    #[doc = "The swagger 2.0 schema describing the service's inputs or outputs. See Swagger specification: http://swagger.io/specification/"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<ServiceInputOutputSpecification>,
    #[doc = "The swagger 2.0 schema describing the service's inputs or outputs. See Swagger specification: http://swagger.io/specification/"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<ServiceInputOutputSpecification>,
    #[doc = "Sample input data for the service's input(s)."]
    #[serde(rename = "exampleRequest", default, skip_serializing_if = "Option::is_none")]
    pub example_request: Option<ExampleRequest>,
    #[doc = "Contains user defined properties describing web service assets. Properties are expressed as Key/Value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<serde_json::Value>,
    #[doc = "The set of global parameters values defined for the web service, given as a global parameter name to default value map. If no default value is specified, the parameter is considered to be required."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Specifies the package type. Valid values are Graph (Specifies a web service published through the Machine Learning Studio) and Code (Specifies a web service published using code such as Python). Note: Code is not supported at this time."]
    #[serde(rename = "packageType")]
    pub package_type: web_service_properties::PackageType,
}
impl WebServiceProperties {
    pub fn new(package_type: web_service_properties::PackageType) -> Self {
        Self {
            title: None,
            description: None,
            created_on: None,
            modified_on: None,
            provisioning_state: None,
            keys: None,
            read_only: None,
            swagger_location: None,
            expose_sample_data: None,
            realtime_configuration: None,
            diagnostics: None,
            storage_account: None,
            machine_learning_workspace: None,
            commitment_plan: None,
            input: None,
            output: None,
            example_request: None,
            assets: None,
            parameters: None,
            package_type,
        }
    }
}
pub mod web_service_properties {
    use super::*;
    #[doc = "Read Only: The provision state of the web service. Valid values are Unknown, Provisioning, Succeeded, and Failed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Unknown,
        Provisioning,
        Succeeded,
        Failed,
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
                Self::Unknown => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Unknown"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Provisioning"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the package type. Valid values are Graph (Specifies a web service published through the Machine Learning Studio) and Code (Specifies a web service published using code such as Python). Note: Code is not supported at this time."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PackageType {
        Graph,
    }
}
#[doc = "Properties specific to a Graph based web service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebServicePropertiesForGraph {
    #[serde(flatten)]
    pub web_service_properties: WebServiceProperties,
    #[doc = "Defines the graph of modules making up the machine learning solution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<GraphPackage>,
}
impl WebServicePropertiesForGraph {
    pub fn new(web_service_properties: WebServiceProperties) -> Self {
        Self {
            web_service_properties,
            package: None,
        }
    }
}
