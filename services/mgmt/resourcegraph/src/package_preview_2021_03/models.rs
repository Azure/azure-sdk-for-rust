#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Query result column descriptor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Column {
    #[doc = "Column name."]
    pub name: String,
    #[doc = "Data type of a column in a table."]
    #[serde(rename = "type")]
    pub type_: ColumnDataType,
}
impl Column {
    pub fn new(name: String, type_: ColumnDataType) -> Self {
        Self { name, type_ }
    }
}
#[doc = "Data type of a column in a table."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ColumnDataType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "object")]
    Object,
}
#[doc = "An interval in time specifying the date and time for the inclusive start and exclusive end, i.e. `[start, end)`."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateTimeInterval {
    #[doc = "A datetime indicating the inclusive/closed start of the time interval, i.e. `[`**`start`**`, end)`. Specifying a `start` that occurs chronologically after `end` will result in an error."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub start: time::OffsetDateTime,
    #[doc = "A datetime indicating the exclusive/open end of the time interval, i.e. `[start, `**`end`**`)`. Specifying an `end` that occurs chronologically before `start` will result in an error."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub end: time::OffsetDateTime,
}
impl DateTimeInterval {
    pub fn new(start: time::OffsetDateTime, end: time::OffsetDateTime) -> Self {
        Self { start, end }
    }
}
#[doc = "Error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[doc = "Error code identifying the specific error."]
    pub code: String,
    #[doc = "A human readable error message."]
    pub message: String,
    #[doc = "Error details"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetails>,
}
impl Error {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            details: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[doc = "Error code identifying the specific error."]
    pub code: String,
    #[doc = "A human readable error message."]
    pub message: String,
}
impl ErrorDetails {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}
#[doc = "An error response from the API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[doc = "Error details."]
    pub error: Error,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new(error: Error) -> Self {
        Self { error }
    }
}
#[doc = "A facet containing additional statistics on the response of a query. Can be either FacetResult or FacetError."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Facet {
    #[doc = "Facet expression, same as in the corresponding facet request."]
    pub expression: String,
    #[doc = "Result type"]
    #[serde(rename = "resultType")]
    pub result_type: String,
}
impl Facet {
    pub fn new(expression: String, result_type: String) -> Self {
        Self { expression, result_type }
    }
}
#[doc = "A facet whose execution resulted in an error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetError {
    #[serde(flatten)]
    pub facet: Facet,
    #[doc = "An array containing detected facet errors with details."]
    pub errors: Vec<ErrorDetails>,
}
impl FacetError {
    pub fn new(facet: Facet, errors: Vec<ErrorDetails>) -> Self {
        Self { facet, errors }
    }
}
#[doc = "A request to compute additional statistics (facets) over the query results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetRequest {
    #[doc = "The column or list of columns to summarize by"]
    pub expression: String,
    #[doc = "The options for facet evaluation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<FacetRequestOptions>,
}
impl FacetRequest {
    pub fn new(expression: String) -> Self {
        Self { expression, options: None }
    }
}
#[doc = "The options for facet evaluation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FacetRequestOptions {
    #[doc = "The column name or query expression to sort on. Defaults to count if not present."]
    #[serde(rename = "sortBy", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[doc = "The sorting order by the selected column (count by default)."]
    #[serde(rename = "sortOrder", default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<facet_request_options::SortOrder>,
    #[doc = "Specifies the filter condition for the 'where' clause which will be run on main query's result, just before the actual faceting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[doc = "The maximum number of facet rows that should be returned."]
    #[serde(rename = "$top", default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
}
impl FacetRequestOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod facet_request_options {
    use super::*;
    #[doc = "The sorting order by the selected column (count by default)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SortOrder {
        #[serde(rename = "asc")]
        Asc,
        #[serde(rename = "desc")]
        Desc,
    }
    impl Default for SortOrder {
        fn default() -> Self {
            Self::Desc
        }
    }
}
#[doc = "Successfully executed facet containing additional statistics on the response of a query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetResult {
    #[serde(flatten)]
    pub facet: Facet,
    #[doc = "Number of total records in the facet results."]
    #[serde(rename = "totalRecords")]
    pub total_records: i64,
    #[doc = "Number of records returned in the facet response."]
    pub count: i32,
    #[doc = "A JObject array or Table containing the desired facets. Only present if the facet is valid."]
    pub data: serde_json::Value,
}
impl FacetResult {
    pub fn new(facet: Facet, total_records: i64, count: i32, data: serde_json::Value) -> Self {
        Self {
            facet,
            total_records,
            count,
            data,
        }
    }
}
#[doc = "Resource Graph REST API operation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The origin of operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft Resource Graph."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of operation: get, read, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description for the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Resource Graph operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Resource Graph operations supported by the Resource Graph resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a query to be executed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryRequest {
    #[doc = "Azure subscriptions against which to execute the query."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subscriptions: Vec<String>,
    #[doc = "Azure management groups against which to execute the query. Example: [ 'mg1', 'mg2' ]"]
    #[serde(rename = "managementGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub management_groups: Vec<String>,
    #[doc = "The resources query."]
    pub query: String,
    #[doc = "The options for query evaluation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<QueryRequestOptions>,
    #[doc = "An array of facet requests to be computed against the query result."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub facets: Vec<FacetRequest>,
}
impl QueryRequest {
    pub fn new(query: String) -> Self {
        Self {
            subscriptions: Vec::new(),
            management_groups: Vec::new(),
            query,
            options: None,
            facets: Vec::new(),
        }
    }
}
#[doc = "The options for query evaluation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryRequestOptions {
    #[doc = "Continuation token for pagination, capturing the next page size and offset, as well as the context of the query."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "The maximum number of rows that the query should return. Overrides the page size when ```$skipToken``` property is present."]
    #[serde(rename = "$top", default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    #[doc = "The number of rows to skip from the beginning of the results. Overrides the next page offset when ```$skipToken``` property is present."]
    #[serde(rename = "$skip", default, skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,
    #[doc = "Defines in which format query result returned."]
    #[serde(rename = "resultFormat", default, skip_serializing_if = "Option::is_none")]
    pub result_format: Option<query_request_options::ResultFormat>,
    #[doc = "Only applicable for tenant and management group level queries to decide whether to allow partial scopes for result in case the number of subscriptions exceed allowed limits."]
    #[serde(rename = "allowPartialScopes", default, skip_serializing_if = "Option::is_none")]
    pub allow_partial_scopes: Option<bool>,
}
impl QueryRequestOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod query_request_options {
    use super::*;
    #[doc = "Defines in which format query result returned."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResultFormat {
        #[serde(rename = "table")]
        Table,
        #[serde(rename = "objectArray")]
        ObjectArray,
    }
    impl Default for ResultFormat {
        fn default() -> Self {
            Self::ObjectArray
        }
    }
}
#[doc = "Query result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResponse {
    #[doc = "Number of total records matching the query."]
    #[serde(rename = "totalRecords")]
    pub total_records: i64,
    #[doc = "Number of records returned in the current response. In the case of paging, this is the number of records in the current page."]
    pub count: i64,
    #[doc = "Indicates whether the query results are truncated."]
    #[serde(rename = "resultTruncated")]
    pub result_truncated: query_response::ResultTruncated,
    #[doc = "When present, the value can be passed to a subsequent query call (together with the same query and scopes used in the current request) to retrieve the next page of data."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Query output in JObject array or Table format."]
    pub data: serde_json::Value,
    #[doc = "Query facets."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub facets: Vec<Facet>,
}
impl QueryResponse {
    pub fn new(total_records: i64, count: i64, result_truncated: query_response::ResultTruncated, data: serde_json::Value) -> Self {
        Self {
            total_records,
            count,
            result_truncated,
            skip_token: None,
            data,
            facets: Vec::new(),
        }
    }
}
pub mod query_response {
    use super::*;
    #[doc = "Indicates whether the query results are truncated."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResultTruncated {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
    }
}
#[doc = "Data on a specific change, represented by a pair of before and after resource snapshots."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceChangeData {
    #[doc = "The resource for a change."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The change ID. Valid and unique within the specified resource only."]
    #[serde(rename = "changeId")]
    pub change_id: String,
    #[doc = "The snapshot before the change."]
    #[serde(rename = "beforeSnapshot")]
    pub before_snapshot: serde_json::Value,
    #[doc = "The snapshot after the change."]
    #[serde(rename = "afterSnapshot")]
    pub after_snapshot: serde_json::Value,
    #[doc = "The change type for snapshot. PropertyChanges will be provided in case of Update change type"]
    #[serde(rename = "changeType", default, skip_serializing_if = "Option::is_none")]
    pub change_type: Option<resource_change_data::ChangeType>,
    #[doc = "An array of resource property change"]
    #[serde(rename = "propertyChanges", default, skip_serializing_if = "Vec::is_empty")]
    pub property_changes: Vec<ResourcePropertyChange>,
}
impl ResourceChangeData {
    pub fn new(change_id: String, before_snapshot: serde_json::Value, after_snapshot: serde_json::Value) -> Self {
        Self {
            resource_id: None,
            change_id,
            before_snapshot,
            after_snapshot,
            change_type: None,
            property_changes: Vec::new(),
        }
    }
}
pub mod resource_change_data {
    use super::*;
    #[doc = "The change type for snapshot. PropertyChanges will be provided in case of Update change type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeType {
        Create,
        Update,
        Delete,
    }
}
pub type ResourceChangeDataList = Vec<ResourceChangeData>;
#[doc = "The parameters for a specific change details request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceChangeDetailsRequestParameters {
    #[doc = "Specifies the list of resources for a change details request."]
    #[serde(rename = "resourceIds")]
    pub resource_ids: Vec<String>,
    #[doc = "Specifies the list of change IDs for a change details request."]
    #[serde(rename = "changeIds")]
    pub change_ids: Vec<String>,
}
impl ResourceChangeDetailsRequestParameters {
    pub fn new(resource_ids: Vec<String>, change_ids: Vec<String>) -> Self {
        Self { resource_ids, change_ids }
    }
}
#[doc = "A list of changes associated with a resource over a specific time interval."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceChangeList {
    #[doc = "The pageable value returned by the operation, i.e. a list of changes to the resource.\n\n- The list is ordered from the most recent changes to the least recent changes.\n- This list will be empty if there were no changes during the requested interval.\n- The `Before` snapshot timestamp value of the oldest change can be outside of the specified time interval."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub changes: Vec<ResourceChangeData>,
    #[doc = "Skip token that encodes the skip information while executing the current request"]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<serde_json::Value>,
}
impl ResourceChangeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters for a specific changes request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceChangesRequestParameters {
    #[doc = "Specifies the list of resources for a changes request."]
    #[serde(rename = "resourceIds", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_ids: Vec<String>,
    #[doc = "The subscription id of resources to query the changes from. "]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Specifies the date and time interval for a changes request."]
    pub interval: serde_json::Value,
    #[doc = "Acts as the continuation token for paged responses."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "The maximum number of changes the client can accept in a paged response."]
    #[serde(rename = "$top", default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    #[doc = "The table name to query resources from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[doc = "The flag if set to true will fetch property changes"]
    #[serde(rename = "fetchPropertyChanges", default, skip_serializing_if = "Option::is_none")]
    pub fetch_property_changes: Option<bool>,
    #[doc = "The flag if set to true will fetch change snapshots"]
    #[serde(rename = "fetchSnapshots", default, skip_serializing_if = "Option::is_none")]
    pub fetch_snapshots: Option<bool>,
}
impl ResourceChangesRequestParameters {
    pub fn new(interval: serde_json::Value) -> Self {
        Self {
            resource_ids: Vec::new(),
            subscription_id: None,
            interval,
            skip_token: None,
            top: None,
            table: None,
            fetch_property_changes: None,
            fetch_snapshots: None,
        }
    }
}
#[doc = "The resource property change"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcePropertyChange {
    #[doc = "The property name"]
    #[serde(rename = "propertyName")]
    pub property_name: String,
    #[doc = "The property value in before snapshot"]
    #[serde(rename = "beforeValue", default, skip_serializing_if = "Option::is_none")]
    pub before_value: Option<String>,
    #[doc = "The property value in after snapshot"]
    #[serde(rename = "afterValue", default, skip_serializing_if = "Option::is_none")]
    pub after_value: Option<String>,
    #[doc = "The change category."]
    #[serde(rename = "changeCategory")]
    pub change_category: resource_property_change::ChangeCategory,
    #[doc = "The property change Type"]
    #[serde(rename = "propertyChangeType")]
    pub property_change_type: resource_property_change::PropertyChangeType,
}
impl ResourcePropertyChange {
    pub fn new(
        property_name: String,
        change_category: resource_property_change::ChangeCategory,
        property_change_type: resource_property_change::PropertyChangeType,
    ) -> Self {
        Self {
            property_name,
            before_value: None,
            after_value: None,
            change_category,
            property_change_type,
        }
    }
}
pub mod resource_property_change {
    use super::*;
    #[doc = "The change category."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeCategory {
        User,
        System,
    }
    #[doc = "The property change Type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PropertyChangeType {
        Insert,
        Update,
        Remove,
    }
}
#[doc = "Data on a specific resource snapshot."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSnapshotData {
    #[doc = "The ID of the snapshot."]
    #[serde(rename = "snapshotId", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[doc = "The time when the snapshot was created.\nThe snapshot timestamp provides an approximation as to when a modification to a resource was detected.  There can be a difference between the actual modification time and the detection time.  This is due to differences in how operations that modify a resource are processed, versus how operation that record resource snapshots are processed."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub timestamp: time::OffsetDateTime,
    #[doc = "The resource snapshot content (in resourceChangeDetails response only)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
}
impl ResourceSnapshotData {
    pub fn new(timestamp: time::OffsetDateTime) -> Self {
        Self {
            snapshot_id: None,
            timestamp,
            content: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcesHistoryRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subscriptions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<ResourcesHistoryRequestOptions>,
    #[serde(rename = "managementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub management_group_id: Option<String>,
}
impl ResourcesHistoryRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcesHistoryRequestOptions {
    #[doc = "An interval in time specifying the date and time for the inclusive start and exclusive end, i.e. `[start, end)`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<DateTimeInterval>,
    #[serde(rename = "$top", default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    #[serde(rename = "$skip", default, skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[serde(rename = "resultFormat", default, skip_serializing_if = "Option::is_none")]
    pub result_format: Option<resources_history_request_options::ResultFormat>,
}
impl ResourcesHistoryRequestOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resources_history_request_options {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResultFormat {
        #[serde(rename = "table")]
        Table,
        #[serde(rename = "objectArray")]
        ObjectArray,
    }
}
pub type Row = Vec<serde_json::Value>;
#[doc = "Query output in tabular format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Table {
    #[doc = "Query result column descriptors."]
    pub columns: Vec<Column>,
    #[doc = "Query result rows."]
    pub rows: Vec<Row>,
}
impl Table {
    pub fn new(columns: Vec<Column>, rows: Vec<Row>) -> Self {
        Self { columns, rows }
    }
}
