#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Model for component."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Component {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "For optimistic concurrency control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Model for properties of a component."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ComponentProperties>,
}
impl Component {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model for properties of a component."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentProperties {
    #[doc = "ID of the workspace."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "ID of the OMS solution this component belong to."]
    #[serde(rename = "solutionId", default, skip_serializing_if = "Option::is_none")]
    pub solution_id: Option<String>,
    #[doc = "Type of the workload."]
    #[serde(rename = "workloadType", default, skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<component_properties::WorkloadType>,
    #[doc = "Name of the component."]
    #[serde(rename = "componentName", default, skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    #[doc = "ID of the component type."]
    #[serde(rename = "componentTypeId", default, skip_serializing_if = "Option::is_none")]
    pub component_type_id: Option<String>,
    #[doc = "Name of the component type. Qualifies the type of component such as whether it is a SQL database, logical disk, website, etc."]
    #[serde(rename = "componentTypeName", default, skip_serializing_if = "Option::is_none")]
    pub component_type_name: Option<String>,
    #[doc = "Component type group category. Classification of component type groups into a logical category. e.g. Network, Disk, Memory, CPU."]
    #[serde(rename = "componentTypeGroupCategory", default, skip_serializing_if = "Option::is_none")]
    pub component_type_group_category: Option<String>,
    #[doc = "Health state of the component."]
    #[serde(rename = "healthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<component_properties::HealthState>,
    #[doc = "Category of component's health state."]
    #[serde(rename = "healthStateCategory", default, skip_serializing_if = "Option::is_none")]
    pub health_state_category: Option<component_properties::HealthStateCategory>,
    #[doc = "Start time for health state changes."]
    #[serde(rename = "healthStateChangesStartTime", with = "azure_core::date::rfc3339::option")]
    pub health_state_changes_start_time: Option<time::OffsetDateTime>,
    #[doc = "End time for health state changes."]
    #[serde(rename = "healthStateChangesEndTime", with = "azure_core::date::rfc3339::option")]
    pub health_state_changes_end_time: Option<time::OffsetDateTime>,
    #[doc = "Time of last health state change."]
    #[serde(rename = "lastHealthStateChangeTime", with = "azure_core::date::rfc3339::option")]
    pub last_health_state_change_time: Option<time::OffsetDateTime>,
    #[doc = "ID of the VM this component belongs to."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "Name of the VM this component belongs to."]
    #[serde(rename = "vmName", default, skip_serializing_if = "Option::is_none")]
    pub vm_name: Option<String>,
    #[doc = "Tags on the VM this component belongs to."]
    #[serde(rename = "vmTags", default, skip_serializing_if = "Option::is_none")]
    pub vm_tags: Option<serde_json::Value>,
    #[doc = "Properties requested in aggregation queries."]
    #[serde(rename = "aggregateProperties", default, skip_serializing_if = "Option::is_none")]
    pub aggregate_properties: Option<serde_json::Value>,
    #[doc = "component children."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Component>,
}
impl ComponentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod component_properties {
    use super::*;
    #[doc = "Type of the workload."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "WorkloadType")]
    pub enum WorkloadType {
        #[serde(rename = "BaseOS")]
        BaseOs,
        #[serde(rename = "SQL")]
        Sql,
        #[serde(rename = "IIS")]
        Iis,
        Apache,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for WorkloadType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for WorkloadType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for WorkloadType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BaseOs => serializer.serialize_unit_variant("WorkloadType", 0u32, "BaseOS"),
                Self::Sql => serializer.serialize_unit_variant("WorkloadType", 1u32, "SQL"),
                Self::Iis => serializer.serialize_unit_variant("WorkloadType", 2u32, "IIS"),
                Self::Apache => serializer.serialize_unit_variant("WorkloadType", 3u32, "Apache"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Health state of the component."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HealthState {
        Error,
        Warning,
        Success,
        Unknown,
        Uninitialized,
    }
    #[doc = "Category of component's health state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HealthStateCategory")]
    pub enum HealthStateCategory {
        Identity,
        CustomGroup,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HealthStateCategory {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HealthStateCategory {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HealthStateCategory {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Identity => serializer.serialize_unit_variant("HealthStateCategory", 0u32, "Identity"),
                Self::CustomGroup => serializer.serialize_unit_variant("HealthStateCategory", 1u32, "CustomGroup"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Model for collection of components."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentsCollection {
    #[doc = "URL to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of components."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Component>,
}
impl azure_core::Continuable for ComponentsCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ComponentsCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error field contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorFieldContract {
    #[doc = "Property level error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Human-readable representation of property-level error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Property name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl ErrorFieldContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error body contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Service-defined error code. This code serves as a sub-status for the HTTP error code specified in the response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Human-readable representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The list of invalid fields send in request, in case of validation error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorFieldContract>,
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
#[doc = "Model for health state change"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthStateChange {
    #[doc = "Health state of monitor instance."]
    #[serde(rename = "healthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<health_state_change::HealthState>,
    #[doc = "Time at which this Health state was reached."]
    #[serde(rename = "healthStateChangeTime", with = "azure_core::date::rfc3339::option")]
    pub health_state_change_time: Option<time::OffsetDateTime>,
}
impl HealthStateChange {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod health_state_change {
    use super::*;
    #[doc = "Health state of monitor instance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HealthState {
        Error,
        Warning,
        Success,
        Unknown,
        Uninitialized,
    }
}
#[doc = "Model for Monitor"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Monitor {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "For optimistic concurrency control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Model for properties of a Monitor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MonitorProperties>,
}
impl Monitor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Criteria for monitor configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorCriteria {
    #[doc = "Target health state of the criteria"]
    #[serde(rename = "healthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<monitor_criteria::HealthState>,
    #[doc = "Threshold value for this criteria"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[doc = "Comparison enum on threshold of this criteria"]
    #[serde(rename = "comparisonOperator", default, skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<monitor_criteria::ComparisonOperator>,
}
impl MonitorCriteria {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod monitor_criteria {
    use super::*;
    #[doc = "Target health state of the criteria"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HealthState {
        Error,
        Warning,
        Success,
        Unknown,
        Uninitialized,
    }
    #[doc = "Comparison enum on threshold of this criteria"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ComparisonOperator {
        Equals,
        GreaterThan,
        GreaterThanOrEqual,
        LessThan,
        LessThanOrEqual,
        NotEquals,
    }
}
#[doc = "Model for monitor instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorInstance {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "For optimistic concurrency control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Model for properties of a monitor instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MonitorInstanceProperties>,
}
impl MonitorInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model for properties of a monitor instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorInstanceProperties {
    #[doc = "ID of the workspace."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "ID of the OMS solution this health instance belong to."]
    #[serde(rename = "solutionId", default, skip_serializing_if = "Option::is_none")]
    pub solution_id: Option<String>,
    #[doc = "Type of the workload."]
    #[serde(rename = "workloadType", default, skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<monitor_instance_properties::WorkloadType>,
    #[doc = "ID of the component."]
    #[serde(rename = "componentId", default, skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    #[doc = "Name of the component."]
    #[serde(rename = "componentName", default, skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    #[doc = "ID of the component type."]
    #[serde(rename = "componentTypeId", default, skip_serializing_if = "Option::is_none")]
    pub component_type_id: Option<String>,
    #[doc = "Name of the component type. Qualifies the type of component such as whether it is a SQL database, logical disk, website, etc."]
    #[serde(rename = "componentTypeName", default, skip_serializing_if = "Option::is_none")]
    pub component_type_name: Option<String>,
    #[doc = "ID of the monitor instance."]
    #[serde(rename = "monitorId", default, skip_serializing_if = "Option::is_none")]
    pub monitor_id: Option<String>,
    #[doc = "Name of the monitor."]
    #[serde(rename = "monitorName", default, skip_serializing_if = "Option::is_none")]
    pub monitor_name: Option<String>,
    #[doc = "Type of the monitor. The qualifier for the health criteria depending on the functionality it performs such as Unit, Aggregate, Dependency."]
    #[serde(rename = "monitorType", default, skip_serializing_if = "Option::is_none")]
    pub monitor_type: Option<monitor_instance_properties::MonitorType>,
    #[doc = "Monitor type category. Indicates the attribute of the component that the health criteria monitors such as Performance, Availability, etc."]
    #[serde(rename = "monitorCategory", default, skip_serializing_if = "Option::is_none")]
    pub monitor_category: Option<monitor_instance_properties::MonitorCategory>,
    #[doc = "Health state of monitor instance."]
    #[serde(rename = "healthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<monitor_instance_properties::HealthState>,
    #[doc = "Category of monitor instance's health state."]
    #[serde(rename = "healthStateCategory", default, skip_serializing_if = "Option::is_none")]
    pub health_state_category: Option<monitor_instance_properties::HealthStateCategory>,
    #[doc = "Health state changes."]
    #[serde(rename = "healthStateChanges", default, skip_serializing_if = "Vec::is_empty")]
    pub health_state_changes: Vec<HealthStateChange>,
    #[doc = "Start time for health state changes."]
    #[serde(rename = "healthStateChangesStartTime", with = "azure_core::date::rfc3339::option")]
    pub health_state_changes_start_time: Option<time::OffsetDateTime>,
    #[doc = "End time for health state changes."]
    #[serde(rename = "healthStateChangesEndTime", with = "azure_core::date::rfc3339::option")]
    pub health_state_changes_end_time: Option<time::OffsetDateTime>,
    #[doc = "Time of last health state change."]
    #[serde(rename = "lastHealthStateChangeTime", with = "azure_core::date::rfc3339::option")]
    pub last_health_state_change_time: Option<time::OffsetDateTime>,
    #[doc = "Generates alert or not."]
    #[serde(rename = "alertGeneration", default, skip_serializing_if = "Option::is_none")]
    pub alert_generation: Option<monitor_instance_properties::AlertGeneration>,
    #[doc = "Properties requested in aggregation queries."]
    #[serde(rename = "aggregateProperties", default, skip_serializing_if = "Option::is_none")]
    pub aggregate_properties: Option<serde_json::Value>,
    #[doc = "Health instance children."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<MonitorInstance>,
}
impl MonitorInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod monitor_instance_properties {
    use super::*;
    #[doc = "Type of the workload."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "WorkloadType")]
    pub enum WorkloadType {
        #[serde(rename = "BaseOS")]
        BaseOs,
        #[serde(rename = "SQL")]
        Sql,
        #[serde(rename = "IIS")]
        Iis,
        Apache,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for WorkloadType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for WorkloadType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for WorkloadType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BaseOs => serializer.serialize_unit_variant("WorkloadType", 0u32, "BaseOS"),
                Self::Sql => serializer.serialize_unit_variant("WorkloadType", 1u32, "SQL"),
                Self::Iis => serializer.serialize_unit_variant("WorkloadType", 2u32, "IIS"),
                Self::Apache => serializer.serialize_unit_variant("WorkloadType", 3u32, "Apache"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of the monitor. The qualifier for the health criteria depending on the functionality it performs such as Unit, Aggregate, Dependency."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MonitorType {
        Aggregate,
        Dependency,
        Unit,
    }
    #[doc = "Monitor type category. Indicates the attribute of the component that the health criteria monitors such as Performance, Availability, etc."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MonitorCategory {
        AvailabilityHealth,
        Configuration,
        EntityHealth,
        PerformanceHealth,
        Security,
    }
    #[doc = "Health state of monitor instance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HealthState {
        Error,
        Warning,
        Success,
        Unknown,
        Uninitialized,
    }
    #[doc = "Category of monitor instance's health state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HealthStateCategory")]
    pub enum HealthStateCategory {
        Identity,
        CustomGroup,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HealthStateCategory {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HealthStateCategory {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HealthStateCategory {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Identity => serializer.serialize_unit_variant("HealthStateCategory", 0u32, "Identity"),
                Self::CustomGroup => serializer.serialize_unit_variant("HealthStateCategory", 1u32, "CustomGroup"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Generates alert or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AlertGeneration")]
    pub enum AlertGeneration {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AlertGeneration {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AlertGeneration {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AlertGeneration {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("AlertGeneration", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("AlertGeneration", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Model for collection of health instances."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorInstancesCollection {
    #[doc = "URL to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of health instances."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MonitorInstance>,
}
impl azure_core::Continuable for MonitorInstancesCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MonitorInstancesCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model for properties of a Monitor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorProperties {
    #[doc = "Description of the monitor"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "ID of the monitor"]
    #[serde(rename = "monitorId", default, skip_serializing_if = "Option::is_none")]
    pub monitor_id: Option<String>,
    #[doc = "Name of the monitor"]
    #[serde(rename = "monitorName", default, skip_serializing_if = "Option::is_none")]
    pub monitor_name: Option<String>,
    #[doc = "User friendly display name of the monitor"]
    #[serde(rename = "monitorDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub monitor_display_name: Option<String>,
    #[doc = "Name of the parent monitor"]
    #[serde(rename = "parentMonitorName", default, skip_serializing_if = "Option::is_none")]
    pub parent_monitor_name: Option<String>,
    #[doc = "User friendly display name of the parent monitor"]
    #[serde(rename = "parentMonitorDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub parent_monitor_display_name: Option<String>,
    #[doc = "Type of the monitor"]
    #[serde(rename = "monitorType", default, skip_serializing_if = "Option::is_none")]
    pub monitor_type: Option<monitor_properties::MonitorType>,
    #[doc = "Category of the monitor"]
    #[serde(rename = "monitorCategory", default, skip_serializing_if = "Option::is_none")]
    pub monitor_category: Option<monitor_properties::MonitorCategory>,
    #[doc = "Component Type Id of monitor"]
    #[serde(rename = "componentTypeId", default, skip_serializing_if = "Option::is_none")]
    pub component_type_id: Option<String>,
    #[doc = "Component Type Name of monitor"]
    #[serde(rename = "componentTypeName", default, skip_serializing_if = "Option::is_none")]
    pub component_type_name: Option<String>,
    #[doc = "Component Type Display Name of the monitor"]
    #[serde(rename = "componentTypeDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub component_type_display_name: Option<String>,
    #[doc = "Is the monitor state enabled or disabled"]
    #[serde(rename = "monitorState", default, skip_serializing_if = "Option::is_none")]
    pub monitor_state: Option<monitor_properties::MonitorState>,
    #[doc = "Collection of MonitorCriteria. For PATCH calls, instead of partial list, complete list of expected criteria should be passed for proper updating."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub criteria: Vec<MonitorCriteria>,
    #[doc = "Generates alerts or not"]
    #[serde(rename = "alertGeneration", default, skip_serializing_if = "Option::is_none")]
    pub alert_generation: Option<monitor_properties::AlertGeneration>,
    #[doc = "Frequency at which monitor condition is evaluated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i32>,
    #[doc = "The duration in minutes in the past during which the monitor is evaluated"]
    #[serde(rename = "lookbackDuration", default, skip_serializing_if = "Option::is_none")]
    pub lookback_duration: Option<i32>,
    #[doc = "URL pointing to the documentation of the monitor"]
    #[serde(rename = "documentationURL", default, skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    #[doc = "Name of the signal on which this monitor is configured."]
    #[serde(rename = "signalName", default, skip_serializing_if = "Option::is_none")]
    pub signal_name: Option<String>,
    #[doc = "Type of the signal on which this monitor is configured."]
    #[serde(rename = "signalType", default, skip_serializing_if = "Option::is_none")]
    pub signal_type: Option<String>,
}
impl MonitorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod monitor_properties {
    use super::*;
    #[doc = "Type of the monitor"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MonitorType {
        Aggregate,
        Dependency,
        Unit,
    }
    #[doc = "Category of the monitor"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MonitorCategory {
        AvailabilityHealth,
        Configuration,
        EntityHealth,
        PerformanceHealth,
        Security,
    }
    #[doc = "Is the monitor state enabled or disabled"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MonitorState {
        Enabled,
        Disabled,
    }
    #[doc = "Generates alerts or not"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AlertGeneration")]
    pub enum AlertGeneration {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AlertGeneration {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AlertGeneration {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AlertGeneration {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("AlertGeneration", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("AlertGeneration", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Model for collection of Monitor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorsCollection {
    #[doc = "URL for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of Monitor."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Monitor>,
}
impl azure_core::Continuable for MonitorsCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MonitorsCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model for NotificationSetting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationSetting {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "For optimistic concurrency control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Model for properties of a NotificationSetting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NotificationSettingProperties>,
}
impl NotificationSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model for properties of a NotificationSetting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationSettingProperties {
    #[doc = "List of action group resource ids to be notified"]
    #[serde(rename = "actionGroupResourceIds", default, skip_serializing_if = "Vec::is_empty")]
    pub action_group_resource_ids: Vec<String>,
}
impl NotificationSettingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model for collection of notificationSettings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationSettingsCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NotificationSetting>,
    #[doc = "URL to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NotificationSettingsCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NotificationSettingsCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation supported by the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Properties of an operation supported by the resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationProperties>,
    #[doc = "The name of the resource operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The origin of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container for a list of operations supported by the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "URL to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an operation supported by the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "The description of the resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "This operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The provider name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
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
