#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An operation that is available in this resource provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableRpOperation {
    #[doc = "Description of an available operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<AvailableRpOperationDisplayInfo>,
    #[doc = "Whether this operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<String>,
    #[doc = "Operation name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "What is this?"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationMetaPropertyInfo>,
    #[doc = "Operation origin"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl AvailableRpOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of an available operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableRpOperationDisplayInfo {
    #[doc = "Operation description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Resource type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Resource provider name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Operation name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
}
impl AvailableRpOperationDisplayInfo {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "What is this?"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaLogSpecification {
    #[doc = "Log display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Time range the log covers"]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
    #[doc = "Log unique name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl OperationMetaLogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "What is this?"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaMetricDimensionSpecification {
    #[doc = "Dimension display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Dimension unique name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether this metric should be exported for Shoebox"]
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
impl OperationMetaMetricDimensionSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "What is this?"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaMetricSpecification {
    #[doc = "The source MDM namespace"]
    #[serde(rename = "sourceMdmNamespace", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_namespace: Option<String>,
    #[doc = "Metric display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Metric unique name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Metric aggregation type"]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "Metric description"]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "The source MDM account"]
    #[serde(rename = "sourceMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_account: Option<String>,
    #[doc = "Whether the regional MDM account is enabled"]
    #[serde(rename = "enableRegionalMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub enable_regional_mdm_account: Option<bool>,
    #[doc = "Metric units"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Metric dimensions"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<OperationMetaMetricDimensionSpecification>,
    #[doc = "Whether the metric supports instance-level aggregation"]
    #[serde(rename = "supportsInstanceLevelAggregation", default, skip_serializing_if = "Option::is_none")]
    pub supports_instance_level_aggregation: Option<bool>,
    #[doc = "Metric filter"]
    #[serde(rename = "metricFilterPattern", default, skip_serializing_if = "Option::is_none")]
    pub metric_filter_pattern: Option<String>,
}
impl OperationMetaMetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "What is this?"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaPropertyInfo {
    #[doc = "What is this?"]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<OperationMetaServiceSpecification>,
}
impl OperationMetaPropertyInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "What is this?"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaServiceSpecification {
    #[doc = "Service metric specifications"]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<OperationMetaMetricSpecification>,
    #[doc = "Service log specifications"]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<OperationMetaLogSpecification>,
}
impl OperationMetaServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResource {
    #[doc = "Operation ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Operation name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Operation status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_resource::Status>,
    #[doc = "Operation properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
    #[doc = "Operation start time"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Operation start time"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Completion percentage of the operation"]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
}
impl OperationResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_resource {
    use super::*;
    #[doc = "Operation status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        InProgress,
        Succeeded,
        Failed,
        Canceled,
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
                Self::InProgress => serializer.serialize_unit_variant("Status", 0u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 3u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An ARM Resource SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuV3 {
    #[doc = "The name of the SKU, typically, a letter + Number code, e.g. P3."]
    pub name: String,
    #[doc = "The tier or edition of the particular SKU, e.g. Basic, Premium."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl SkuV3 {
    pub fn new(name: String) -> Self {
        Self { name, tier: None }
    }
}
#[doc = "A sql database resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDatabase {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "ARM System Data."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The sql database's properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlDatabaseProperties>,
}
impl SqlDatabase {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            system_data: None,
            properties: None,
        }
    }
}
#[doc = "Sql database data retention."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlDatabaseDataRetention {
    #[doc = "Specifies the data retention period (ISO8601 format)."]
    #[serde(rename = "retentionPeriod", default, skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<String>,
    #[doc = "Specifies the dropped database retention period (ISO8601 format)."]
    #[serde(rename = "dropRetentionPeriod", default, skip_serializing_if = "Option::is_none")]
    pub drop_retention_period: Option<String>,
}
impl SqlDatabaseDataRetention {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of databases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlDatabaseListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlDatabase>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlDatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SqlDatabaseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The sql database's properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlDatabaseProperties {
    #[doc = "The status of the database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<sql_database_properties::Status>,
    #[doc = "The collation of the database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
    #[doc = "The Guid of the database."]
    #[serde(rename = "databaseGuid", default, skip_serializing_if = "Option::is_none")]
    pub database_guid: Option<String>,
    #[doc = "The storage redundancy of the database."]
    #[serde(rename = "storageRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub storage_redundancy: Option<sql_database_properties::StorageRedundancy>,
    #[doc = "Sql database data retention."]
    #[serde(rename = "dataRetention", default, skip_serializing_if = "Option::is_none")]
    pub data_retention: Option<SqlDatabaseDataRetention>,
}
impl SqlDatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_database_properties {
    use super::*;
    #[doc = "The status of the database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Online,
        Restoring,
        RecoveryPending,
        Recovering,
        Suspect,
        Offline,
        Standby,
        Shutdown,
        EmergencyMode,
        AutoClosed,
        Copying,
        Creating,
        Inaccessible,
        OfflineSecondary,
        Pausing,
        Paused,
        Resuming,
        Scaling,
        OfflineChangingDwPerformanceTiers,
        OnlineChangingDwPerformanceTiers,
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
                Self::Online => serializer.serialize_unit_variant("Status", 0u32, "Online"),
                Self::Restoring => serializer.serialize_unit_variant("Status", 1u32, "Restoring"),
                Self::RecoveryPending => serializer.serialize_unit_variant("Status", 2u32, "RecoveryPending"),
                Self::Recovering => serializer.serialize_unit_variant("Status", 3u32, "Recovering"),
                Self::Suspect => serializer.serialize_unit_variant("Status", 4u32, "Suspect"),
                Self::Offline => serializer.serialize_unit_variant("Status", 5u32, "Offline"),
                Self::Standby => serializer.serialize_unit_variant("Status", 6u32, "Standby"),
                Self::Shutdown => serializer.serialize_unit_variant("Status", 7u32, "Shutdown"),
                Self::EmergencyMode => serializer.serialize_unit_variant("Status", 8u32, "EmergencyMode"),
                Self::AutoClosed => serializer.serialize_unit_variant("Status", 9u32, "AutoClosed"),
                Self::Copying => serializer.serialize_unit_variant("Status", 10u32, "Copying"),
                Self::Creating => serializer.serialize_unit_variant("Status", 11u32, "Creating"),
                Self::Inaccessible => serializer.serialize_unit_variant("Status", 12u32, "Inaccessible"),
                Self::OfflineSecondary => serializer.serialize_unit_variant("Status", 13u32, "OfflineSecondary"),
                Self::Pausing => serializer.serialize_unit_variant("Status", 14u32, "Pausing"),
                Self::Paused => serializer.serialize_unit_variant("Status", 15u32, "Paused"),
                Self::Resuming => serializer.serialize_unit_variant("Status", 16u32, "Resuming"),
                Self::Scaling => serializer.serialize_unit_variant("Status", 17u32, "Scaling"),
                Self::OfflineChangingDwPerformanceTiers => {
                    serializer.serialize_unit_variant("Status", 18u32, "OfflineChangingDwPerformanceTiers")
                }
                Self::OnlineChangingDwPerformanceTiers => {
                    serializer.serialize_unit_variant("Status", 19u32, "OnlineChangingDwPerformanceTiers")
                }
                Self::Disabled => serializer.serialize_unit_variant("Status", 20u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The storage redundancy of the database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageRedundancy")]
    pub enum StorageRedundancy {
        Local,
        Geo,
        Zone,
        GeoZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageRedundancy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageRedundancy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageRedundancy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Local => serializer.serialize_unit_variant("StorageRedundancy", 0u32, "Local"),
                Self::Geo => serializer.serialize_unit_variant("StorageRedundancy", 1u32, "Geo"),
                Self::Zone => serializer.serialize_unit_variant("StorageRedundancy", 2u32, "Zone"),
                Self::GeoZone => serializer.serialize_unit_variant("StorageRedundancy", 3u32, "GeoZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A sql database resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlDatabaseUpdate {
    #[doc = "The sql database's properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlDatabaseProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SqlDatabaseUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of sql pools."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlPoolV3>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlPoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SqlPoolListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The sql pool's properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolProperties {
    #[doc = "The status of the sql pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<sql_pool_properties::Status>,
    #[doc = "The Guid of the sql pool."]
    #[serde(rename = "sqlPoolGuid", default, skip_serializing_if = "Option::is_none")]
    pub sql_pool_guid: Option<String>,
    #[doc = "The current service level objective name of the sql pool."]
    #[serde(rename = "currentServiceObjectiveName", default, skip_serializing_if = "Option::is_none")]
    pub current_service_objective_name: Option<String>,
    #[doc = "The requested service level objective name of the sql pool."]
    #[serde(rename = "requestedServiceObjectiveName", default, skip_serializing_if = "Option::is_none")]
    pub requested_service_objective_name: Option<String>,
    #[doc = "The max service level objective name of the sql pool."]
    #[serde(rename = "maxServiceObjectiveName", default, skip_serializing_if = "Option::is_none")]
    pub max_service_objective_name: Option<String>,
    #[doc = "The period of inactivity in minutes before automatically pausing the sql pool."]
    #[serde(rename = "autoPauseTimer", default, skip_serializing_if = "Option::is_none")]
    pub auto_pause_timer: Option<i32>,
    #[doc = "Indicates whether the sql pool can automatically resume when connection attempts are made."]
    #[serde(rename = "autoResume", default, skip_serializing_if = "Option::is_none")]
    pub auto_resume: Option<bool>,
}
impl SqlPoolProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_pool_properties {
    use super::*;
    #[doc = "The status of the sql pool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Invisible,
        Online,
        Offline,
        Creating,
        Inaccessible,
        Pausing,
        Paused,
        Resuming,
        Scaling,
        Dropping,
        Error,
        Unknown,
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
                Self::Invisible => serializer.serialize_unit_variant("Status", 0u32, "Invisible"),
                Self::Online => serializer.serialize_unit_variant("Status", 1u32, "Online"),
                Self::Offline => serializer.serialize_unit_variant("Status", 2u32, "Offline"),
                Self::Creating => serializer.serialize_unit_variant("Status", 3u32, "Creating"),
                Self::Inaccessible => serializer.serialize_unit_variant("Status", 4u32, "Inaccessible"),
                Self::Pausing => serializer.serialize_unit_variant("Status", 5u32, "Pausing"),
                Self::Paused => serializer.serialize_unit_variant("Status", 6u32, "Paused"),
                Self::Resuming => serializer.serialize_unit_variant("Status", 7u32, "Resuming"),
                Self::Scaling => serializer.serialize_unit_variant("Status", 8u32, "Scaling"),
                Self::Dropping => serializer.serialize_unit_variant("Status", 9u32, "Dropping"),
                Self::Error => serializer.serialize_unit_variant("Status", 10u32, "Error"),
                Self::Unknown => serializer.serialize_unit_variant("Status", 11u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A sql pool resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolUpdate {
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuV3>,
    #[doc = "The sql pool's properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlPoolProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SqlPoolUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A sql pool resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlPoolV3 {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuV3>,
    #[doc = "Kind of SqlPool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "ARM System Data."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The sql pool's properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlPoolProperties>,
}
impl SqlPoolV3 {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            sku: None,
            kind: None,
            system_data: None,
            properties: None,
        }
    }
}
#[doc = "ARM System Data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "A string identifier for the identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource: <User|Application|ManagedIdentity|Key>"]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "A string identifier for the identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource: <User|Application|ManagedIdentity|Key>"]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of last modification (UTC)."]
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
    #[doc = "The type of identity that created the resource: <User|Application|ManagedIdentity|Key>"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[doc = "The type of identity that last modified the resource: <User|Application|ManagedIdentity|Key>"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
#[doc = "The resource model definition for an Azure Resource Manager tracked top level resource which has 'tags' and a 'location'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
