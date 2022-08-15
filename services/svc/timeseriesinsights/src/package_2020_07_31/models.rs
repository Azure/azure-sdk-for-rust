#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Aggregate Series query. Allows to calculate an aggregated time series from events for a given Time Series ID and search span."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregateSeries {
    #[doc = "A single Time Series ID value that is an array of primitive values that uniquely identifies a time series instance (e.g. a single device). Note that a single Time Series ID can be composite if multiple properties are specified as Time Series ID at environment creation time. The position and type of values must match Time Series ID properties specified on the environment and returned by Get Model Setting API. Cannot be empty."]
    #[serde(rename = "timeSeriesId")]
    pub time_series_id: TimeSeriesId,
    #[doc = "The range of time. Cannot be null or negative."]
    #[serde(rename = "searchSpan")]
    pub search_span: DateTimeRange,
    #[doc = "Time series expression (TSX) written as a single string. Examples: \"$event.Status.String='Good'\", \"avg($event.Temperature)\". Refer to the documentation on how to write time series expressions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Tsx>,
    #[doc = "Interval size is specified in ISO-8601 duration format. All intervals are the same size. One month is always converted to 30 days, and one year is always 365 days. Examples: 1 minute is \"PT1M\", 1 millisecond is \"PT0.001S\". For more information, see https://www.w3.org/TR/xmlschema-2/#duration"]
    pub interval: String,
    #[doc = "This allows the user to optionally select the variables that needs to be projected. When it is null or not set, all the variables from inlineVariables and model are returned. Can be null."]
    #[serde(rename = "projectedVariables", default, skip_serializing_if = "Vec::is_empty")]
    pub projected_variables: Vec<String>,
    #[doc = "This allows the user the optionally define inline-variables apart from the ones already defined in the model. When the inline variable names have the same name as the model, the inline variable definition takes precedence. Can be null."]
    #[serde(rename = "inlineVariables", default, skip_serializing_if = "Option::is_none")]
    pub inline_variables: Option<serde_json::Value>,
}
impl AggregateSeries {
    pub fn new(time_series_id: TimeSeriesId, search_span: DateTimeRange, interval: String) -> Self {
        Self {
            time_series_id,
            search_span,
            filter: None,
            interval,
            projected_variables: Vec::new(),
            inline_variables: None,
        }
    }
}
#[doc = "Aggregate variable represents any aggregation calculation. Aggregate Variables does not support interpolation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregateVariable {
    #[serde(flatten)]
    pub variable: Variable,
    #[doc = "Time series expression (TSX) written as a single string. Examples: \"$event.Status.String='Good'\", \"avg($event.Temperature)\". Refer to the documentation on how to write time series expressions."]
    pub aggregation: Tsx,
}
impl AggregateVariable {
    pub fn new(variable: Variable, aggregation: Tsx) -> Self {
        Self { variable, aggregation }
    }
}
#[doc = "Event availability information when environment contains events. Contains time range of events and approximate distribution of events over time."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Availability {
    #[doc = "The range of time. Cannot be null or negative."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<DateTimeRange>,
    #[doc = "Interval size for the returned distribution of the events. Returned interval is selected to return a reasonable number of points. All intervals are the same size. On the wire interval is specified in ISO-8601 duration format. One month is always converted to 30 days, and one year is always 365 days. Examples: 1 minute is \"PT1M\", 1 millisecond is \"PT0.001S\". For more information, see https://www.w3.org/TR/xmlschema-2/#duration"]
    #[serde(rename = "intervalSize", default, skip_serializing_if = "Option::is_none")]
    pub interval_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distribution: Option<serde_json::Value>,
}
impl Availability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of Get Availability operation. When environment has no data, availability property is null."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityResponse {
    #[doc = "Event availability information when environment contains events. Contains time range of events and approximate distribution of events over time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,
}
impl AvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Categorical variable represents signal that needs to be analyzed based on the count or duration of occurrence of limited set of defined values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CategoricalVariable {
    #[serde(flatten)]
    pub variable: Variable,
    #[doc = "Time series expression (TSX) written as a single string. Examples: \"$event.Status.String='Good'\", \"avg($event.Temperature)\". Refer to the documentation on how to write time series expressions."]
    pub value: Tsx,
    #[doc = "The interpolation operation to be performed on the raw data points. Currently, only sampling of interpolated time series is allowed. Allowed aggregate function - eg: left($value). Can be null if no interpolation needs to be applied."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation: Option<Interpolation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<TimeSeriesAggregateCategory>,
    #[doc = "Represents the default category."]
    #[serde(rename = "defaultCategory")]
    pub default_category: TimeSeriesDefaultCategory,
}
impl CategoricalVariable {
    pub fn new(variable: Variable, value: Tsx, default_category: TimeSeriesDefaultCategory) -> Self {
        Self {
            variable,
            value,
            interpolation: None,
            categories: Vec::new(),
            default_category,
        }
    }
}
#[doc = "The range of time. Cannot be null or negative."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateTimeRange {
    #[doc = "Start timestamp of the time range. Start timestamp is inclusive when used in time series query requests. Events that have this timestamp are included."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub from: time::OffsetDateTime,
    #[doc = "End timestamp of the time range. End timestamp is exclusive when used in time series query requests. Events that match this timestamp are excluded. Note that end timestamp is inclusive when returned by Get Availability (meaning that there is an event with this exact \"to\" timestamp)."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub to: time::OffsetDateTime,
}
impl DateTimeRange {
    pub fn new(from: time::OffsetDateTime, to: time::OffsetDateTime) -> Self {
        Self { from, to }
    }
}
#[doc = "Property of an event that is either stored or computed. Properties are identified by both name and type. Different events can have properties with same name, but different type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventProperty {
    #[doc = "The name of the property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the property that is either stored in events or computed by a calculation."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<PropertyType>,
}
impl EventProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Event schema of all events within a given search span. Event schema is a set of property definitions. Properties are identified by both name and type. Different events can have properties with same name, but different type. Event schema may not be contain all persisted properties when there are too many properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventSchema {
    #[doc = "A set of property definitions. When environment has no data, the returned array is empty."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<EventProperty>,
}
impl EventSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request to get the event schema of all events within a given search span."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetEventSchemaRequest {
    #[doc = "The range of time. Cannot be null or negative."]
    #[serde(rename = "searchSpan")]
    pub search_span: DateTimeRange,
}
impl GetEventSchemaRequest {
    pub fn new(search_span: DateTimeRange) -> Self {
        Self { search_span }
    }
}
#[doc = "Get Events query. Allows to retrieve raw events for a given Time Series ID and search span."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetEvents {
    #[doc = "A single Time Series ID value that is an array of primitive values that uniquely identifies a time series instance (e.g. a single device). Note that a single Time Series ID can be composite if multiple properties are specified as Time Series ID at environment creation time. The position and type of values must match Time Series ID properties specified on the environment and returned by Get Model Setting API. Cannot be empty."]
    #[serde(rename = "timeSeriesId")]
    pub time_series_id: TimeSeriesId,
    #[doc = "The range of time. Cannot be null or negative."]
    #[serde(rename = "searchSpan")]
    pub search_span: DateTimeRange,
    #[doc = "Time series expression (TSX) written as a single string. Examples: \"$event.Status.String='Good'\", \"avg($event.Temperature)\". Refer to the documentation on how to write time series expressions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Tsx>,
    #[doc = "Projected properties is an array of properties which you want to project. These properties must appear in the events; otherwise, they are not returned."]
    #[serde(rename = "projectedProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub projected_properties: Vec<EventProperty>,
    #[doc = "Maximum number of property values in the whole response set, not the maximum number of property values per page. Defaults to 10,000 when not set. Maximum value of take can be 250,000."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub take: Option<i32>,
}
impl GetEvents {
    pub fn new(time_series_id: TimeSeriesId, search_span: DateTimeRange) -> Self {
        Self {
            time_series_id,
            search_span,
            filter: None,
            projected_properties: Vec::new(),
            take: None,
        }
    }
}
#[doc = "Partial list of time series hierarchies returned in a single request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetHierarchiesPage {
    #[serde(flatten)]
    pub paged_response: PagedResponse,
    #[doc = "Partial list of time series hierarchies returned in a single request. Can be empty if server was unable to fill the page in this request, or there is no more objects when continuation token is null."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hierarchies: Vec<TimeSeriesHierarchy>,
}
impl GetHierarchiesPage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Partial list of time series instances returned in a single request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetInstancesPage {
    #[serde(flatten)]
    pub paged_response: PagedResponse,
    #[doc = "Partial list of time series instances returned in a single request. Can be empty if server was unable to fill the page in this request, or there is no more objects when continuation token is null."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instances: Vec<TimeSeriesInstance>,
}
impl GetInstancesPage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Get Series query. Allows to retrieve time series of calculated variable values from events for a given Time Series ID and search span."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSeries {
    #[doc = "A single Time Series ID value that is an array of primitive values that uniquely identifies a time series instance (e.g. a single device). Note that a single Time Series ID can be composite if multiple properties are specified as Time Series ID at environment creation time. The position and type of values must match Time Series ID properties specified on the environment and returned by Get Model Setting API. Cannot be empty."]
    #[serde(rename = "timeSeriesId")]
    pub time_series_id: TimeSeriesId,
    #[doc = "The range of time. Cannot be null or negative."]
    #[serde(rename = "searchSpan")]
    pub search_span: DateTimeRange,
    #[doc = "Time series expression (TSX) written as a single string. Examples: \"$event.Status.String='Good'\", \"avg($event.Temperature)\". Refer to the documentation on how to write time series expressions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Tsx>,
    #[doc = "Selected variables that needs to be projected in the query result. When it is null or not set, all the variables from inlineVariables and time series type in the model are returned. Can be null."]
    #[serde(rename = "projectedVariables", default, skip_serializing_if = "Vec::is_empty")]
    pub projected_variables: Vec<String>,
    #[doc = "Optional inline variables apart from the ones already defined in the time series type in the model. When the inline variable name is the same name as in the model, the inline variable definition takes precedence. Can be null."]
    #[serde(rename = "inlineVariables", default, skip_serializing_if = "Option::is_none")]
    pub inline_variables: Option<serde_json::Value>,
    #[doc = "Maximum number of property values in the whole response set, not the maximum number of property values per page. Defaults to 10,000 when not set. Maximum value of take can be 250,000."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub take: Option<i32>,
}
impl GetSeries {
    pub fn new(time_series_id: TimeSeriesId, search_span: DateTimeRange) -> Self {
        Self {
            time_series_id,
            search_span,
            filter: None,
            projected_variables: Vec::new(),
            inline_variables: None,
            take: None,
        }
    }
}
#[doc = "Partial list of time series types returned in a single request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetTypesPage {
    #[serde(flatten)]
    pub paged_response: PagedResponse,
    #[doc = "Partial list of time series types returned in a single request. Can be empty if server was unable to fill the page with more types in this request, or there is no more types when continuation token is null."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<TimeSeriesType>,
}
impl GetTypesPage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request to perform a single operation on a batch of hierarchies. Exactly one of \"get\", \"put\" or \"delete\" must be set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HierarchiesBatchRequest {
    #[doc = "Request to get or delete multiple time series hierarchies. Exactly one of \"hierarchyIds\" or \"names\" must be set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get: Option<HierarchiesRequestBatchGetDelete>,
    #[doc = "\"put\" should be set while creating or updating hierarchies."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub put: Vec<TimeSeriesHierarchy>,
    #[doc = "Request to get or delete multiple time series hierarchies. Exactly one of \"hierarchyIds\" or \"names\" must be set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<HierarchiesRequestBatchGetDelete>,
}
impl HierarchiesBatchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a single operation on a batch of time series hierarchies. Only one of \"get\", \"put\" or \"delete\" will be set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HierarchiesBatchResponse {
    #[doc = "List of hierarchy or error objects corresponding by position to the \"get\" array in the request. Hierarchy object is set when operation is successful and error object is set when operation is unsuccessful."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub get: Vec<TimeSeriesHierarchyOrError>,
    #[doc = "List of hierarchy or error object corresponding by position to the \"put\" array in the request. Hierarchy object is set when operation is successful and error object is set when operation is unsuccessful."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub put: Vec<TimeSeriesHierarchyOrError>,
    #[doc = "List of error objects corresponding by position to the \"delete\" array in the request - null when the operation is successful."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delete: Vec<TsiErrorBody>,
}
impl HierarchiesBatchResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of whether to expand hierarchy nodes in the same search instances call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HierarchiesExpandParameter {
    #[doc = "Kind of the expansion of hierarchy nodes. When it is set to 'UntilChildren', the hierarchy nodes are expanded recursively until there is more than one child. When it is set to 'OneLevel', the hierarchies are expanded only at the single level matching path search instances parameter. Optional, default is 'UntilChildren'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<hierarchies_expand_parameter::Kind>,
}
impl HierarchiesExpandParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hierarchies_expand_parameter {
    use super::*;
    #[doc = "Kind of the expansion of hierarchy nodes. When it is set to 'UntilChildren', the hierarchy nodes are expanded recursively until there is more than one child. When it is set to 'OneLevel', the hierarchies are expanded only at the single level matching path search instances parameter. Optional, default is 'UntilChildren'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        UntilChildren,
        OneLevel,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::UntilChildren => serializer.serialize_unit_variant("Kind", 0u32, "UntilChildren"),
                Self::OneLevel => serializer.serialize_unit_variant("Kind", 1u32, "OneLevel"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Request to get or delete multiple time series hierarchies. Exactly one of \"hierarchyIds\" or \"names\" must be set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HierarchiesRequestBatchGetDelete {
    #[doc = "List of hierarchy IDs."]
    #[serde(rename = "hierarchyIds", default, skip_serializing_if = "Vec::is_empty")]
    pub hierarchy_ids: Vec<String>,
    #[doc = "List of hierarchy names."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
}
impl HierarchiesRequestBatchGetDelete {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of sorting of hierarchy nodes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HierarchiesSortParameter {
    #[doc = "Value to use for hierarchy node sorting. When it is set to 'CumulativeInstanceCount', the returned hierarchies are sorted based on the total instances belonging to the hierarchy node and its child hierarchy nodes. When it is set to 'Name', the returned hierarchies are sorted based on the hierarchy name. Optional, default is 'CumulativeInstanceCount'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub by: Option<hierarchies_sort_parameter::By>,
}
impl HierarchiesSortParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hierarchies_sort_parameter {
    use super::*;
    #[doc = "Value to use for hierarchy node sorting. When it is set to 'CumulativeInstanceCount', the returned hierarchies are sorted based on the total instances belonging to the hierarchy node and its child hierarchy nodes. When it is set to 'Name', the returned hierarchies are sorted based on the hierarchy name. Optional, default is 'CumulativeInstanceCount'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "By")]
    pub enum By {
        CumulativeInstanceCount,
        Name,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for By {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for By {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for By {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::CumulativeInstanceCount => serializer.serialize_unit_variant("By", 0u32, "CumulativeInstanceCount"),
                Self::Name => serializer.serialize_unit_variant("By", 1u32, "Name"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The hierarchy node which contains the instances matching the query based on the input. May be empty or null."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HierarchyHit {
    #[doc = "Name of the hierarchy node. May be empty, cannot be null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Total number of instances that belong to this node and it's subtrees matching the query."]
    #[serde(rename = "cumulativeInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_instance_count: Option<i32>,
    #[doc = "The hierarchy nodes which contains the instances matching the query based on the input. May be empty or null."]
    #[serde(rename = "hierarchyNodes", default, skip_serializing_if = "Option::is_none")]
    pub hierarchy_nodes: Option<SearchHierarchyNodesResponse>,
}
impl HierarchyHit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Time series instance that is returned by instances search call. Returned instance matched the search request and contains highlighted text to be displayed to the user if it is set to 'true'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstanceHit {
    #[doc = "A single Time Series ID value that is an array of primitive values that uniquely identifies a time series instance (e.g. a single device). Note that a single Time Series ID can be composite if multiple properties are specified as Time Series ID at environment creation time. The position and type of values must match Time Series ID properties specified on the environment and returned by Get Model Setting API. Cannot be empty."]
    #[serde(rename = "timeSeriesId", default, skip_serializing_if = "Option::is_none")]
    pub time_series_id: Option<TimeSeriesId>,
    #[doc = "Name of the time series instance that matched the search request. May be null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Represents the type that time series instance which matched the search request belongs to. Never null."]
    #[serde(rename = "typeId", default, skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
    #[doc = "List of time series hierarchy IDs that time series instance which matched the search request belongs to. Cannot be used to lookup hierarchies. May be null."]
    #[serde(rename = "hierarchyIds", default, skip_serializing_if = "Vec::is_empty")]
    pub hierarchy_ids: Vec<String>,
    #[doc = "Highlighted text of time series instance to be displayed to the user. Highlighting inserts <hit> and </hit> tags in the portions of text that matched the search string. Do not use any of the highlighted properties to do further API calls."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlights: Option<instance_hit::Highlights>,
}
impl InstanceHit {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod instance_hit {
    use super::*;
    #[doc = "Highlighted text of time series instance to be displayed to the user. Highlighting inserts <hit> and </hit> tags in the portions of text that matched the search string. Do not use any of the highlighted properties to do further API calls."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Highlights {
        #[doc = "List of highlighted string values of Time Series ID for displaying. Cannot be used to lookup instance."]
        #[serde(rename = "timeSeriesId", default, skip_serializing_if = "Vec::is_empty")]
        pub time_series_id: Vec<String>,
        #[doc = "Highlighted time series type name that this instance belongs to."]
        #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
        pub type_name: Option<String>,
        #[doc = "Highlighted name of time series instance. May be null."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[doc = "Highlighted description of time series instance. May be null."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "List of highlighted time series hierarchy IDs that time series instance belongs to. Cannot be used to lookup hierarchies. May be null."]
        #[serde(rename = "hierarchyIds", default, skip_serializing_if = "Vec::is_empty")]
        pub hierarchy_ids: Vec<String>,
        #[doc = "List of highlighted time series hierarchy names that time series instance belongs to. Cannot be used to lookup hierarchies. May be null."]
        #[serde(rename = "hierarchyNames", default, skip_serializing_if = "Vec::is_empty")]
        pub hierarchy_names: Vec<String>,
        #[doc = "List of highlighted time series instance field names. May be null."]
        #[serde(rename = "instanceFieldNames", default, skip_serializing_if = "Vec::is_empty")]
        pub instance_field_names: Vec<String>,
        #[doc = "List of highlighted time series instance field values. May be null."]
        #[serde(rename = "instanceFieldValues", default, skip_serializing_if = "Vec::is_empty")]
        pub instance_field_values: Vec<String>,
    }
    impl Highlights {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of a batch operation on a particular time series instance. Instance object is set when operation is successful (except put operation) and error object is set when operation is unsuccessful."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstanceOrError {
    #[doc = "Time series instances are the time series themselves. In most cases, the deviceId or assetId is the unique identifier of the asset in the environment. Instances have descriptive information associated with them called instance fields. At a minimum, instance fields include hierarchy information. They can also include useful, descriptive data like the manufacturer, operator, or the last service date."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<TimeSeriesInstance>,
    #[doc = "A particular API error with an error code and a message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<TsiErrorBody>,
}
impl InstanceOrError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request to perform a single operation on a batch of instances. Exactly one of \"get\", \"put\", \"update\" or \"delete\" must be set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstancesBatchRequest {
    #[doc = "Request to get or delete instances by time series IDs or time series names. Exactly one of \"timeSeriesIds\" or \"names\" must be set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get: Option<InstancesRequestBatchGetOrDelete>,
    #[doc = "Time series instances to be created or updated."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub put: Vec<TimeSeriesInstance>,
    #[doc = "Time series instance to be updated. If instance does not exist, an error is returned."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub update: Vec<TimeSeriesInstance>,
    #[doc = "Request to get or delete instances by time series IDs or time series names. Exactly one of \"timeSeriesIds\" or \"names\" must be set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<InstancesRequestBatchGetOrDelete>,
}
impl InstancesBatchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a single operation on a batch of instances. Only one of \"get\", \"put\", \"update\" or \"delete\" will be set based on the request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstancesBatchResponse {
    #[doc = "List of instance or error objects corresponding by position to the \"get\" array in the request. Instance object is set when operation is successful and error object is set when operation is unsuccessful."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub get: Vec<InstanceOrError>,
    #[doc = "List of error objects corresponding by position to the \"put\" array in the request. Error object is set when operation is unsuccessful."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub put: Vec<InstanceOrError>,
    #[doc = "List of error objects corresponding by position to the \"update\" array in the request. Instance object is set when operation is successful and error object is set when operation is unsuccessful."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub update: Vec<InstanceOrError>,
    #[doc = "List of error objects corresponding by position to the \"delete\" array in the request. Null means the instance has been deleted, or did not exist. Error object is set when operation is unsuccessful (e.g. when there are events associated with this time series instance)."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delete: Vec<TsiErrorBody>,
}
impl InstancesBatchResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request to get or delete instances by time series IDs or time series names. Exactly one of \"timeSeriesIds\" or \"names\" must be set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstancesRequestBatchGetOrDelete {
    #[doc = "List of time series IDs of the time series instances to return or delete."]
    #[serde(rename = "timeSeriesIds", default, skip_serializing_if = "Vec::is_empty")]
    pub time_series_ids: Vec<TimeSeriesId>,
    #[doc = "List of names of the time series instances to return or delete."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
}
impl InstancesRequestBatchGetOrDelete {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Suggested search string to be used for further search for time series instances."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstancesSearchStringSuggestion {
    #[doc = "Suggested search string. Can be used for further search for time series instances."]
    #[serde(rename = "searchString", default, skip_serializing_if = "Option::is_none")]
    pub search_string: Option<String>,
    #[doc = "Highlighted suggested search string to be displayed to the user. Highlighting inserts <hit> and </hit> tags in the portions of text that matched the search string. Do not use highlighted search string to do further search calls."]
    #[serde(rename = "highlightedSearchString", default, skip_serializing_if = "Option::is_none")]
    pub highlighted_search_string: Option<String>,
}
impl InstancesSearchStringSuggestion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of how time series instances are sorted before being returned by search instances call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstancesSortParameter {
    #[doc = "Value to use for sorting of the time series instances before being returned by search instances call. When it is set to 'Rank', the returned instances are sorted based on the relevance. When it is set to 'DisplayName', the returned results are sorted based on the display name. Display name is the name of the instance if it exists, otherwise, display name is the time series ID. Default is 'Rank'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub by: Option<instances_sort_parameter::By>,
}
impl InstancesSortParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod instances_sort_parameter {
    use super::*;
    #[doc = "Value to use for sorting of the time series instances before being returned by search instances call. When it is set to 'Rank', the returned instances are sorted based on the relevance. When it is set to 'DisplayName', the returned results are sorted based on the display name. Display name is the name of the instance if it exists, otherwise, display name is the time series ID. Default is 'Rank'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "By")]
    pub enum By {
        Rank,
        DisplayName,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for By {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for By {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for By {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Rank => serializer.serialize_unit_variant("By", 0u32, "Rank"),
                Self::DisplayName => serializer.serialize_unit_variant("By", 1u32, "DisplayName"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Request to get search string suggestions for time series instances search based on prefix text."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstancesSuggestRequest {
    #[doc = "Search string for which suggestions are required. Empty is allowed, but not null."]
    #[serde(rename = "searchString")]
    pub search_string: String,
    #[doc = "Maximum number of suggestions expected in the result. Defaults to 10 when not set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub take: Option<i32>,
}
impl InstancesSuggestRequest {
    pub fn new(search_string: String) -> Self {
        Self { search_string, take: None }
    }
}
#[doc = "Response of getting suggestions for search for time series instances."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstancesSuggestResponse {
    #[doc = "List of instance suggestions for searching time series models."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub suggestions: Vec<InstancesSearchStringSuggestion>,
}
impl InstancesSuggestResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The interpolation operation to be performed on the raw data points. Currently, only sampling of interpolated time series is allowed. Allowed aggregate function - eg: left($value). Can be null if no interpolation needs to be applied."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Interpolation {
    #[doc = "The type of interpolation technique : \"Linear\" or \"Step\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<interpolation::Kind>,
    #[doc = "The time range to the left and right of the search span to be used for Interpolation. This is helpful in scenarios where the data points are missing close to the start or end of the input search span. Can be null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boundary: Option<interpolation::Boundary>,
}
impl Interpolation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod interpolation {
    use super::*;
    #[doc = "The type of interpolation technique : \"Linear\" or \"Step\"."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Linear,
        Step,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Linear => serializer.serialize_unit_variant("Kind", 0u32, "Linear"),
                Self::Step => serializer.serialize_unit_variant("Kind", 1u32, "Step"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The time range to the left and right of the search span to be used for Interpolation. This is helpful in scenarios where the data points are missing close to the start or end of the input search span. Can be null."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Boundary {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub span: Option<String>,
    }
    impl Boundary {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Response containing full time series model settings which include model name, Time Series ID properties and default type ID."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelSettingsResponse {
    #[doc = "Time series model settings including model name, Time Series ID properties and default type ID."]
    #[serde(rename = "modelSettings", default, skip_serializing_if = "Option::is_none")]
    pub model_settings: Option<TimeSeriesModelSettings>,
}
impl ModelSettingsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Numeric variable represents a single continuous numeric signal that can be reconstructed using interpolation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumericVariable {
    #[serde(flatten)]
    pub variable: Variable,
    #[doc = "Time series expression (TSX) written as a single string. Examples: \"$event.Status.String='Good'\", \"avg($event.Temperature)\". Refer to the documentation on how to write time series expressions."]
    pub value: Tsx,
    #[doc = "The interpolation operation to be performed on the raw data points. Currently, only sampling of interpolated time series is allowed. Allowed aggregate function - eg: left($value). Can be null if no interpolation needs to be applied."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation: Option<Interpolation>,
    #[doc = "Time series expression (TSX) written as a single string. Examples: \"$event.Status.String='Good'\", \"avg($event.Temperature)\". Refer to the documentation on how to write time series expressions."]
    pub aggregation: Tsx,
}
impl NumericVariable {
    pub fn new(variable: Variable, value: Tsx, aggregation: Tsx) -> Self {
        Self {
            variable,
            value,
            interpolation: None,
            aggregation,
        }
    }
}
#[doc = "Partial result that has continuation token to fetch the next partial result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedResponse {
    #[doc = "If returned, this means that current results represent a partial result. Continuation token allows to get the next page of results. To get the next page of query results, send the same request with continuation token parameter in \"x-ms-continuation\" HTTP header."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl PagedResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of the property that is either stored in events or computed by a calculation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PropertyType")]
pub enum PropertyType {
    Bool,
    DateTime,
    Double,
    String,
    TimeSpan,
    Long,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PropertyType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Bool => serializer.serialize_unit_variant("PropertyType", 0u32, "Bool"),
            Self::DateTime => serializer.serialize_unit_variant("PropertyType", 1u32, "DateTime"),
            Self::Double => serializer.serialize_unit_variant("PropertyType", 2u32, "Double"),
            Self::String => serializer.serialize_unit_variant("PropertyType", 3u32, "String"),
            Self::TimeSpan => serializer.serialize_unit_variant("PropertyType", 4u32, "TimeSpan"),
            Self::Long => serializer.serialize_unit_variant("PropertyType", 5u32, "Long"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Values of a single property corresponding to the timestamps. May contain nulls. Type of values matches the type of property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PropertyValues {
    #[serde(flatten)]
    pub event_property: EventProperty,
    #[doc = "Values of a single property corresponding to the timestamps. May contain nulls. Type of values matches the type of property."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<serde_json::Value>,
}
impl PropertyValues {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request to execute a time series query over events. Exactly one of \"getEvents\", \"getSeries\" or \"aggregateSeries\" must be set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryRequest {
    #[doc = "Get Events query. Allows to retrieve raw events for a given Time Series ID and search span."]
    #[serde(rename = "getEvents", default, skip_serializing_if = "Option::is_none")]
    pub get_events: Option<GetEvents>,
    #[doc = "Get Series query. Allows to retrieve time series of calculated variable values from events for a given Time Series ID and search span."]
    #[serde(rename = "getSeries", default, skip_serializing_if = "Option::is_none")]
    pub get_series: Option<GetSeries>,
    #[doc = "Aggregate Series query. Allows to calculate an aggregated time series from events for a given Time Series ID and search span."]
    #[serde(rename = "aggregateSeries", default, skip_serializing_if = "Option::is_none")]
    pub aggregate_series: Option<AggregateSeries>,
}
impl QueryRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A single page of query results. If query has not finished yet, a page will have continuation token set. In this case, to get the next page of results, send the same request again with continuation token parameter. If query has completed, the continuation token is null. It is also possible to get an empty page with only continuation token set when no query results have been computed yet. If paging has completed (continuation token is null), then timestamps and properties may be empty if there is no data to return."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryResultPage {
    #[serde(flatten)]
    pub paged_response: PagedResponse,
    #[doc = "The timestamps of the values of the time series. If an aggregation over intervals is used, timestamps represent the start of corresponding intervals. If events are retrieved, timestamps are values of timestamp $ts property of events. Can be null if server was unable to fill the page in this request, or can be empty if there are no more objects when continuation token is null."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timestamps: Vec<time::OffsetDateTime>,
    #[doc = "Collection of time series properties and values for each of the timestamps.  Can be null if server was unable to fill the page in this request, or can be empty if there are no more objects when continuation token is null."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<PropertyValues>,
    #[doc = "Approximate progress of the query in percentage. It can be between 0 and 100. When the continuation token in the response is null, the progress is expected to be 100."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
}
impl QueryResultPage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The hierarchy nodes which contains the instances matching the query based on the input. May be empty or null."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchHierarchyNodesResponse {
    #[doc = "The list of hierarchy nodes which contains the instances matching the query based on the input. May be empty."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hits: Vec<HierarchyHit>,
    #[doc = "Total number of hierarchy nodes which contains the instances matching the query based on the input."]
    #[serde(rename = "hitCount", default, skip_serializing_if = "Option::is_none")]
    pub hit_count: Option<i32>,
    #[doc = "If returned, this means that current results represent a partial result. Continuation token allows to get the next page of results. To get the next page of query results, send the same request with continuation token parameter in \"x-ms-continuation\" HTTP header."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl SearchHierarchyNodesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameter of how to return time series instance hierarchies by search instances call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchInstancesHierarchiesParameters {
    #[doc = "Definition of whether to expand hierarchy nodes in the same search instances call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<HierarchiesExpandParameter>,
    #[doc = "Definition of sorting of hierarchy nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<HierarchiesSortParameter>,
    #[doc = "Maximum number of hierarchies in the same level to return in the same page. Optional, default is 10 when not set. Range is from 1 to 100. If there are results beyond the page size, the continuation token can be used to fetch the next page."]
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}
impl SearchInstancesHierarchiesParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters of how to return time series instances by search instances call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchInstancesParameters {
    #[doc = "Definition of which instances are returned. When recursive is set to 'true', all instances that have the path that starts with path the path parameter are returned. When recursive is set to 'false', only instances that have the path that exactly matches the path parameter are returned. Using recursive search allows to implement search user experience, while using non-recursive search allows to implement navigation experience. Optional, default is 'true'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    #[doc = "Definition of how time series instances are sorted before being returned by search instances call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<InstancesSortParameter>,
    #[doc = "Definition of highlighted search results or not. When it is set to 'true', the highlighted search results are returned. When it is set to 'false', the highlighted search results are not returned. Default is 'true'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlights: Option<bool>,
    #[doc = "Maximum number of instances expected in each page of the result. Defaults to 10 when not set. Ranges from 1 to 100. If there are results beyond the page size, the user can use the continuation token to fetch the next page."]
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}
impl SearchInstancesParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request to execute a search query against time series instances and return matching time series instances."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchInstancesRequest {
    #[doc = "Query search string that will be matched to the attributes of time series instances. Example: \"floor 100\". Case-insensitive, must be present, but can be empty string."]
    #[serde(rename = "searchString")]
    pub search_string: String,
    #[doc = "Filter on hierarchy path of time series instances. Path is represented as array of string path segments. First element should be hierarchy name. Example: [\"Location\", \"California\"]. Optional, case sensitive, never empty and can be null."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub path: Vec<String>,
    #[doc = "Parameters of how to return time series instances by search instances call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances: Option<SearchInstancesParameters>,
    #[doc = "Parameter of how to return time series instance hierarchies by search instances call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hierarchies: Option<SearchInstancesHierarchiesParameters>,
}
impl SearchInstancesRequest {
    pub fn new(search_string: String) -> Self {
        Self {
            search_string,
            path: Vec::new(),
            instances: None,
            hierarchies: None,
        }
    }
}
#[doc = "The instances matching the query based on the input. May be empty or null."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchInstancesResponse {
    #[doc = "The list of instances matching the query based on the input. May be empty."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hits: Vec<InstanceHit>,
    #[doc = "Total number of instances matching the query based on the input."]
    #[serde(rename = "hitCount", default, skip_serializing_if = "Option::is_none")]
    pub hit_count: Option<i32>,
    #[doc = "If returned, this means that current results represent a partial result. Continuation token allows to get the next page of results. To get the next page of query results, send the same request with continuation token parameter in \"x-ms-continuation\" HTTP header."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl SearchInstancesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Single page of the search results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchInstancesResponsePage {
    #[doc = "The instances matching the query based on the input. May be empty or null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances: Option<SearchInstancesResponse>,
    #[doc = "The hierarchy nodes which contains the instances matching the query based on the input. May be empty or null."]
    #[serde(rename = "hierarchyNodes", default, skip_serializing_if = "Option::is_none")]
    pub hierarchy_nodes: Option<SearchHierarchyNodesResponse>,
}
impl SearchInstancesResponsePage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Category used in categorical variables. A category is defined by 'label' and the 'values' that are assigned this label."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesAggregateCategory {
    #[doc = "The name of the category which will be used in constructing the output variable names."]
    pub label: String,
    #[doc = "The list of values that a category maps to. Can be either a unique list of string or list of long."]
    pub values: Vec<serde_json::Value>,
}
impl TimeSeriesAggregateCategory {
    pub fn new(label: String, values: Vec<serde_json::Value>) -> Self {
        Self { label, values }
    }
}
#[doc = "Represents the default category."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesDefaultCategory {
    #[doc = "The name of the default category which will be assigned to the values that does not match any of those that are defined in the 'categories'."]
    pub label: String,
}
impl TimeSeriesDefaultCategory {
    pub fn new(label: String) -> Self {
        Self { label }
    }
}
#[doc = "Time series hierarchy organizes time series instances into a tree."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesHierarchy {
    #[doc = "Case-sensitive unique hierarchy identifier. Can be null while creating hierarchy objects and then server generates the id, not null on get and delete operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "User-given unique name for the type. It is mutable and not null."]
    pub name: String,
    #[doc = "Definition of how time series hierarchy tree levels are created."]
    pub source: time_series_hierarchy::Source,
}
impl TimeSeriesHierarchy {
    pub fn new(name: String, source: time_series_hierarchy::Source) -> Self {
        Self { id: None, name, source }
    }
}
pub mod time_series_hierarchy {
    use super::*;
    #[doc = "Definition of how time series hierarchy tree levels are created."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Source {
        #[doc = "List of instance field names that must be set in all time series instances that belong to this hierarchy. The order of the instance fields defines the levels in the hierarchy."]
        #[serde(rename = "instanceFieldNames", default, skip_serializing_if = "Vec::is_empty")]
        pub instance_field_names: Vec<String>,
    }
    impl Source {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of a batch operation on a particular time series hierarchy. Hierarchy is set when operation is successful and error object is set when operation is unsuccessful."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeSeriesHierarchyOrError {
    #[doc = "Time series hierarchy organizes time series instances into a tree."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hierarchy: Option<TimeSeriesHierarchy>,
    #[doc = "A particular API error with an error code and a message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<TsiErrorBody>,
}
impl TimeSeriesHierarchyOrError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type TimeSeriesId = Vec<serde_json::Value>;
pub type TimeSeriesIdProperties = Vec<TimeSeriesIdProperty>;
#[doc = "A definition of a single property that can be used in time series ID properties defined during environment creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeSeriesIdProperty {
    #[doc = "The name of the property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the property. Currently, only \"String\" is supported."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<time_series_id_property::Type>,
}
impl TimeSeriesIdProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod time_series_id_property {
    use super::*;
    #[doc = "The type of the property. Currently, only \"String\" is supported."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
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
                Self::String => serializer.serialize_unit_variant("Type", 0u32, "String"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Time series instances are the time series themselves. In most cases, the deviceId or assetId is the unique identifier of the asset in the environment. Instances have descriptive information associated with them called instance fields. At a minimum, instance fields include hierarchy information. They can also include useful, descriptive data like the manufacturer, operator, or the last service date."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesInstance {
    #[doc = "A single Time Series ID value that is an array of primitive values that uniquely identifies a time series instance (e.g. a single device). Note that a single Time Series ID can be composite if multiple properties are specified as Time Series ID at environment creation time. The position and type of values must match Time Series ID properties specified on the environment and returned by Get Model Setting API. Cannot be empty."]
    #[serde(rename = "timeSeriesId")]
    pub time_series_id: TimeSeriesId,
    #[doc = "This represents the type that this instance belongs to. Never null."]
    #[serde(rename = "typeId")]
    pub type_id: String,
    #[doc = "Optional name of the instance which is unique in an environment. Names acts as a mutable alias or display name of the time series instance. Mutable, may be null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "This optional field contains description about the instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Set of time series hierarchy IDs that the instance belong to. May be null."]
    #[serde(rename = "hierarchyIds", default, skip_serializing_if = "Vec::is_empty")]
    pub hierarchy_ids: Vec<String>,
    #[doc = "Set of key-value pairs that contain user-defined instance properties. It may be null. Supported property value types are: bool, string, long, double and it cannot be nested or null."]
    #[serde(rename = "instanceFields", default, skip_serializing_if = "Option::is_none")]
    pub instance_fields: Option<serde_json::Value>,
}
impl TimeSeriesInstance {
    pub fn new(time_series_id: TimeSeriesId, type_id: String) -> Self {
        Self {
            time_series_id,
            type_id,
            name: None,
            description: None,
            hierarchy_ids: Vec::new(),
            instance_fields: None,
        }
    }
}
#[doc = "Time series model settings including model name, Time Series ID properties and default type ID."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeSeriesModelSettings {
    #[doc = "Time series model display name which is shown in the UX. Examples: \"Temperature Sensors\", \"MyDevices\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Time series ID properties defined during environment creation."]
    #[serde(rename = "timeSeriesIdProperties", default, skip_serializing_if = "Option::is_none")]
    pub time_series_id_properties: Option<TimeSeriesIdProperties>,
    #[doc = "Default type ID of the model that new time series instances will automatically belong to."]
    #[serde(rename = "defaultTypeId", default, skip_serializing_if = "Option::is_none")]
    pub default_type_id: Option<String>,
}
impl TimeSeriesModelSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Time series type defines variables or formulas for doing computation on events associated with time series instances. Each time series instance is associated with exactly one type. A type can have one or more variables. For example, a time series instance might be of type Temperature Sensor, which consists of the variables avg temperature, min temperature, and max temperature."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesType {
    #[doc = "Case-sensitive unique type identifier that is immutable. Can be null on create or update requests, and then server generates the ID. Not null on get and delete operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "User-given unique name for the type. Mutable, not null."]
    pub name: String,
    #[doc = "Description of the type. May be null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Different variables associated with the type. Not empty, not null."]
    pub variables: serde_json::Value,
}
impl TimeSeriesType {
    pub fn new(name: String, variables: serde_json::Value) -> Self {
        Self {
            id: None,
            name,
            description: None,
            variables,
        }
    }
}
#[doc = "Result of a batch operation on a particular time series type. Type object is set when operation is successful and error object is set when operation is unsuccessful."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeSeriesTypeOrError {
    #[doc = "Time series type defines variables or formulas for doing computation on events associated with time series instances. Each time series instance is associated with exactly one type. A type can have one or more variables. For example, a time series instance might be of type Temperature Sensor, which consists of the variables avg temperature, min temperature, and max temperature."]
    #[serde(rename = "timeSeriesType", default, skip_serializing_if = "Option::is_none")]
    pub time_series_type: Option<TimeSeriesType>,
    #[doc = "A particular API error with an error code and a message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<TsiErrorBody>,
}
impl TimeSeriesTypeOrError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about an API error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TsiError {
    #[doc = "A particular API error with an error code and a message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<TsiErrorBody>,
}
impl TsiError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A particular API error with an error code and a message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TsiErrorBody {
    #[doc = "Language-independent, human-readable string that defines a service-specific error code. This code serves as a more specific indicator for the HTTP error code specified in the response. Can be used to programmatically handle specific error cases."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Human-readable, language-independent representation of the error. It is intended as an aid to developers and is not suitable for exposure to end users."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Target of the particular error (for example, the name of the property in error). May be null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A particular API error with an error code and a message."]
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Box<Option<TsiErrorBody>>,
    #[doc = "Contains additional error information. May be null."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<TsiErrorDetails>,
}
impl TsiErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional error information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TsiErrorDetails {
    #[doc = "Language-independent, human-readable string that defines a service-specific error code. This code serves as a more specific indicator for the HTTP error code specified in the response. Can be used to programmatically handle specific error cases."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Human-readable, language-independent representation of the error. It is intended as an aid to developers and is not suitable for exposure to end users."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl TsiErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Time series expression (TSX) written as a single string. Examples: \"$event.Status.String='Good'\", \"avg($event.Temperature)\". Refer to the documentation on how to write time series expressions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tsx {
    #[doc = "Time series expression (TSX) written as a single string. Examples: \"$event.Status.String='Good'\", \"avg($event.Temperature)\". Refer to the documentation on how to write time series expressions."]
    pub tsx: String,
}
impl Tsx {
    pub fn new(tsx: String) -> Self {
        Self { tsx }
    }
}
#[doc = "Request to perform a single operation on a batch of time series types. Exactly one of \"get\", \"put\" or \"delete\" must be set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TypesBatchRequest {
    #[doc = "Request to get or delete time series types by IDs or type names. Exactly one of \"typeIds\" or \"names\" must be set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get: Option<TypesRequestBatchGetOrDelete>,
    #[doc = "Definition of what time series types to update or create."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub put: Vec<TimeSeriesType>,
    #[doc = "Request to get or delete time series types by IDs or type names. Exactly one of \"typeIds\" or \"names\" must be set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<TypesRequestBatchGetOrDelete>,
}
impl TypesBatchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a single operation on a batch of time series types. Exactly one of \"get\", \"put\" or \"delete\" will be set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TypesBatchResponse {
    #[doc = "List of types or error objects corresponding by position to the \"get\" array in the request. Type object is set when operation is successful and error object is set when operation is unsuccessful."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub get: Vec<TimeSeriesTypeOrError>,
    #[doc = "List of types or error objects corresponding by position to the \"put\" array in the request. Type object is set when operation is successful and error object is set when operation is unsuccessful."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub put: Vec<TimeSeriesTypeOrError>,
    #[doc = "List of error objects corresponding by position to the \"delete\" array in the request - null when the operation is successful."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delete: Vec<TsiErrorBody>,
}
impl TypesBatchResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request to get or delete time series types by IDs or type names. Exactly one of \"typeIds\" or \"names\" must be set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TypesRequestBatchGetOrDelete {
    #[doc = "List of IDs of time series types to return or delete."]
    #[serde(rename = "typeIds", default, skip_serializing_if = "Vec::is_empty")]
    pub type_ids: Vec<String>,
    #[doc = "List of names of time series types to return or delete."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
}
impl TypesRequestBatchGetOrDelete {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request to update model settings. One of \"name\" or \"defaultTypeId\" must be set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateModelSettingsRequest {
    #[doc = "Model display name which is shown in the UX and mutable by the user. Initial value is \"DefaultModel\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Default type id of the model that new instances will automatically belong to."]
    #[serde(rename = "defaultTypeId", default, skip_serializing_if = "Option::is_none")]
    pub default_type_id: Option<String>,
}
impl UpdateModelSettingsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Variables are named calculations over values from the events. Time Series Insights variable definitions contain formula and computation rules. Variables are stored in the type definition in Time Series Model and can be provided inline via Query APIs to override the stored definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    #[doc = "Allowed \"kind\" values are - \"numeric\" or \"aggregate\". While \"numeric\" allows you to specify value of the reconstructed signal and the expression to aggregate them, the \"aggregate\" kind lets you directly aggregate on the event properties without specifying value."]
    pub kind: String,
    #[doc = "Time series expression (TSX) written as a single string. Examples: \"$event.Status.String='Good'\", \"avg($event.Temperature)\". Refer to the documentation on how to write time series expressions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Tsx>,
}
impl Variable {
    pub fn new(kind: String) -> Self {
        Self { kind, filter: None }
    }
}
