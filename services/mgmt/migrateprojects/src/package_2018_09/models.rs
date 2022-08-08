#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplyClause {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transformations: Vec<TransformationNode>,
}
impl ApplyClause {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Assessment properties that can be shared by various publishers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessmentDetails {
    #[doc = "Gets or sets the id of the assessment done on the machine."]
    #[serde(rename = "assessmentId", default, skip_serializing_if = "Option::is_none")]
    pub assessment_id: Option<String>,
    #[doc = "Gets or sets the target VM size."]
    #[serde(rename = "targetVMSize", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_size: Option<String>,
    #[doc = "Gets or sets the target VM location."]
    #[serde(rename = "targetVMLocation", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_location: Option<String>,
    #[doc = "Gets or sets the target storage type."]
    #[serde(rename = "targetStorageType", default, skip_serializing_if = "Option::is_none")]
    pub target_storage_type: Option<serde_json::Value>,
    #[doc = "Gets or sets the time the message was enqueued."]
    #[serde(rename = "enqueueTime", default, skip_serializing_if = "Option::is_none")]
    pub enqueue_time: Option<String>,
    #[doc = "Gets or sets the name of the solution that sent the data."]
    #[serde(rename = "solutionName", default, skip_serializing_if = "Option::is_none")]
    pub solution_name: Option<String>,
    #[doc = "Gets or sets the unique identifier of the machine."]
    #[serde(rename = "machineId", default, skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,
    #[doc = "Gets or sets the unique identifier of the virtual machine manager(vCenter/VMM)."]
    #[serde(rename = "machineManagerId", default, skip_serializing_if = "Option::is_none")]
    pub machine_manager_id: Option<String>,
    #[doc = "Gets or sets the fabric type."]
    #[serde(rename = "fabricType", default, skip_serializing_if = "Option::is_none")]
    pub fabric_type: Option<String>,
    #[doc = "Gets or sets the time of the last modification of the machine details."]
    #[serde(rename = "lastUpdatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the name of the machine."]
    #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[doc = "Gets or sets the list of IP addresses of the machine. IP addresses could be IP V4 or IP V6."]
    #[serde(rename = "ipAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_addresses: Vec<String>,
    #[doc = "Gets or sets the FQDN of the machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Gets or sets the BIOS ID of the machine."]
    #[serde(rename = "biosId", default, skip_serializing_if = "Option::is_none")]
    pub bios_id: Option<String>,
    #[doc = "Gets or sets the list of MAC addresses of the machine."]
    #[serde(rename = "macAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub mac_addresses: Vec<String>,
    #[doc = "Gets or sets the ISV specific extended information."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<serde_json::Value>,
}
impl AssessmentDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Database REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Database {
    #[doc = "Gets or sets the relative URL to get to this REST resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name of this REST resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the type of this REST resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of the database resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseProperties>,
}
impl Database {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Assessment properties that can be shared by various publishers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseAssessmentDetails {
    #[doc = "Gets or sets the database assessment scope/Id."]
    #[serde(rename = "assessmentId", default, skip_serializing_if = "Option::is_none")]
    pub assessment_id: Option<String>,
    #[doc = "Gets or sets the number of blocking changes found."]
    #[serde(rename = "migrationBlockersCount", default, skip_serializing_if = "Option::is_none")]
    pub migration_blockers_count: Option<i32>,
    #[doc = "Gets or sets the number of breaking changes found."]
    #[serde(rename = "breakingChangesCount", default, skip_serializing_if = "Option::is_none")]
    pub breaking_changes_count: Option<i32>,
    #[doc = "Gets or sets a value indicating whether the database is ready for migration."]
    #[serde(rename = "isReadyForMigration", default, skip_serializing_if = "Option::is_none")]
    pub is_ready_for_migration: Option<bool>,
    #[doc = "Gets or sets the assessed target database type."]
    #[serde(rename = "assessmentTargetType", default, skip_serializing_if = "Option::is_none")]
    pub assessment_target_type: Option<String>,
    #[doc = "Gets or sets the time when the database was last assessed."]
    #[serde(rename = "lastAssessedTime", with = "azure_core::date::rfc3339::option")]
    pub last_assessed_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the compatibility level of the database."]
    #[serde(rename = "compatibilityLevel", default, skip_serializing_if = "Option::is_none")]
    pub compatibility_level: Option<String>,
    #[doc = "Gets or sets the database size."]
    #[serde(rename = "databaseSizeInMB", default, skip_serializing_if = "Option::is_none")]
    pub database_size_in_mb: Option<String>,
    #[doc = "Gets or sets the time of the last modification of the database details."]
    #[serde(rename = "lastUpdatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the time the message was enqueued."]
    #[serde(rename = "enqueueTime", default, skip_serializing_if = "Option::is_none")]
    pub enqueue_time: Option<String>,
    #[doc = "Gets or sets the name of the solution that sent the data."]
    #[serde(rename = "solutionName", default, skip_serializing_if = "Option::is_none")]
    pub solution_name: Option<String>,
    #[doc = "Gets or sets the database server instance Id."]
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[doc = "Gets or sets the database name."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Gets or sets the extended properties of the database."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<serde_json::Value>,
}
impl DatabaseAssessmentDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of databases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseCollection {
    #[doc = "Gets or sets the databases."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Database>,
    #[doc = "Gets or sets the value of nextLink."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DatabaseCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DatabaseInstance REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseInstance {
    #[doc = "Gets or sets the relative URL to get to this REST resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name of this REST resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the type of this REST resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of the database instance resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseInstanceProperties>,
}
impl DatabaseInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of database instances."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseInstanceCollection {
    #[doc = "Gets or sets the database instances."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatabaseInstance>,
    #[doc = "Gets or sets the value of nextLink."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DatabaseInstanceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Discovery properties that can be shared by various publishers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseInstanceDiscoveryDetails {
    #[doc = "Gets or sets the time of the last modification of the database instance details."]
    #[serde(rename = "lastUpdatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the database instance Id."]
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[doc = "Gets or sets the time the message was enqueued."]
    #[serde(rename = "enqueueTime", default, skip_serializing_if = "Option::is_none")]
    pub enqueue_time: Option<String>,
    #[doc = "Gets or sets the name of the solution that sent the data."]
    #[serde(rename = "solutionName", default, skip_serializing_if = "Option::is_none")]
    pub solution_name: Option<String>,
    #[doc = "Gets or sets the database instance name."]
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[doc = "Gets or sets the database instance version."]
    #[serde(rename = "instanceVersion", default, skip_serializing_if = "Option::is_none")]
    pub instance_version: Option<String>,
    #[doc = "Gets or sets the database instance type."]
    #[serde(rename = "instanceType", default, skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[doc = "Gets or sets the host name of the database server."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "Gets or sets the IP addresses of the database server. IP addresses could be IP V4 or IP V6."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "Gets or sets the port number of the database server."]
    #[serde(rename = "portNumber", default, skip_serializing_if = "Option::is_none")]
    pub port_number: Option<i32>,
    #[doc = "Gets or sets the extended properties of the database server."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<serde_json::Value>,
}
impl DatabaseInstanceDiscoveryDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the database instance resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseInstanceProperties {
    #[doc = "Gets or sets the assessment details of the database instance published by various sources."]
    #[serde(rename = "discoveryData", default, skip_serializing_if = "Vec::is_empty")]
    pub discovery_data: Vec<DatabaseInstanceDiscoveryDetails>,
    #[doc = "Gets or sets the database instances summary per solution. The key of dictionary is the solution name and value is the corresponding database instance summary object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<serde_json::Value>,
    #[doc = "Gets or sets the time of the last modification of the database."]
    #[serde(rename = "lastUpdatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
}
impl DatabaseInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the database instance summary object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseInstanceSummary {
    #[doc = "Gets or sets the count of databases assessed."]
    #[serde(rename = "databasesAssessedCount", default, skip_serializing_if = "Option::is_none")]
    pub databases_assessed_count: Option<i32>,
    #[doc = "Gets or sets the count of databases ready for migration."]
    #[serde(rename = "migrationReadyCount", default, skip_serializing_if = "Option::is_none")]
    pub migration_ready_count: Option<i32>,
}
impl DatabaseInstanceSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the database error resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseMigrateEventProperties {
    #[serde(flatten)]
    pub migrate_event_properties: MigrateEventProperties,
    #[doc = "Gets or sets the database for which the error is being reported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[doc = "Gets or sets the database instance for which the error is being reported."]
    #[serde(rename = "databaseInstanceId", default, skip_serializing_if = "Option::is_none")]
    pub database_instance_id: Option<String>,
}
impl DatabaseMigrateEventProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The database project summary class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseProjectSummary {
    #[serde(flatten)]
    pub project_summary: ProjectSummary,
}
impl DatabaseProjectSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the database resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseProperties {
    #[doc = "Gets or sets the assessment details of the database published by various sources."]
    #[serde(rename = "assessmentData", default, skip_serializing_if = "Vec::is_empty")]
    pub assessment_data: Vec<DatabaseAssessmentDetails>,
    #[doc = "Gets or sets the time of the last modification of the database."]
    #[serde(rename = "lastUpdatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
}
impl DatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the databases solution summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabasesSolutionSummary {
    #[serde(flatten)]
    pub solution_summary: SolutionSummary,
    #[doc = "Gets or sets the count of databases assessed."]
    #[serde(rename = "databasesAssessedCount", default, skip_serializing_if = "Option::is_none")]
    pub databases_assessed_count: Option<i32>,
    #[doc = "Gets or sets the count of database instances assessed."]
    #[serde(rename = "databaseInstancesAssessedCount", default, skip_serializing_if = "Option::is_none")]
    pub database_instances_assessed_count: Option<i32>,
    #[doc = "Gets or sets the count of databases ready for migration."]
    #[serde(rename = "migrationReadyCount", default, skip_serializing_if = "Option::is_none")]
    pub migration_ready_count: Option<i32>,
}
impl DatabasesSolutionSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefaultQuerySettings {
    #[serde(rename = "enableExpand", default, skip_serializing_if = "Option::is_none")]
    pub enable_expand: Option<bool>,
    #[serde(rename = "enableSelect", default, skip_serializing_if = "Option::is_none")]
    pub enable_select: Option<bool>,
    #[serde(rename = "enableCount", default, skip_serializing_if = "Option::is_none")]
    pub enable_count: Option<bool>,
    #[serde(rename = "enableOrderBy", default, skip_serializing_if = "Option::is_none")]
    pub enable_order_by: Option<bool>,
    #[serde(rename = "enableFilter", default, skip_serializing_if = "Option::is_none")]
    pub enable_filter: Option<bool>,
    #[serde(rename = "maxTop", default, skip_serializing_if = "Option::is_none")]
    pub max_top: Option<i32>,
}
impl DefaultQuerySettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Discovery properties that can be published by various ISVs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscoveryDetails {
    #[doc = "Gets or sets the OS type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "Gets or sets the OS name."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "Gets or sets the OS version."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[doc = "Gets or sets the time the message was enqueued."]
    #[serde(rename = "enqueueTime", default, skip_serializing_if = "Option::is_none")]
    pub enqueue_time: Option<String>,
    #[doc = "Gets or sets the name of the solution that sent the data."]
    #[serde(rename = "solutionName", default, skip_serializing_if = "Option::is_none")]
    pub solution_name: Option<String>,
    #[doc = "Gets or sets the unique identifier of the machine."]
    #[serde(rename = "machineId", default, skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,
    #[doc = "Gets or sets the unique identifier of the virtual machine manager(vCenter/VMM)."]
    #[serde(rename = "machineManagerId", default, skip_serializing_if = "Option::is_none")]
    pub machine_manager_id: Option<String>,
    #[doc = "Gets or sets the fabric type."]
    #[serde(rename = "fabricType", default, skip_serializing_if = "Option::is_none")]
    pub fabric_type: Option<String>,
    #[doc = "Gets or sets the time of the last modification of the machine details."]
    #[serde(rename = "lastUpdatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the name of the machine."]
    #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[doc = "Gets or sets the list of IP addresses of the machine. IP addresses could be IP V4 or IP V6."]
    #[serde(rename = "ipAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_addresses: Vec<String>,
    #[doc = "Gets or sets the FQDN of the machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Gets or sets the BIOS ID of the machine."]
    #[serde(rename = "biosId", default, skip_serializing_if = "Option::is_none")]
    pub bios_id: Option<String>,
    #[doc = "Gets or sets the list of MAC addresses of the machine."]
    #[serde(rename = "macAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub mac_addresses: Vec<String>,
    #[doc = "Gets or sets the ISV specific extended information."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<serde_json::Value>,
}
impl DiscoveryDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EdmReferentialConstraintPropertyPair {
    #[serde(rename = "dependentProperty", default, skip_serializing_if = "Option::is_none")]
    pub dependent_property: Option<IEdmStructuralProperty>,
    #[serde(rename = "principalProperty", default, skip_serializing_if = "Option::is_none")]
    pub principal_property: Option<IEdmStructuralProperty>,
}
impl EdmReferentialConstraintPropertyPair {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventCollection {
    #[doc = "Gets or sets the machines."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MigrateEvent>,
    #[doc = "Gets or sets the value of nextLink."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl EventCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FilterClause {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<SingleValueNode>,
    #[serde(rename = "rangeVariable", default, skip_serializing_if = "Option::is_none")]
    pub range_variable: Option<RangeVariable>,
    #[serde(rename = "itemType", default, skip_serializing_if = "Option::is_none")]
    pub item_type: Option<IEdmTypeReference>,
}
impl FilterClause {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FilterQueryOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<ODataQueryContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validator: Option<FilterQueryValidator>,
    #[serde(rename = "filterClause", default, skip_serializing_if = "Option::is_none")]
    pub filter_clause: Option<FilterClause>,
    #[serde(rename = "rawValue", default, skip_serializing_if = "Option::is_none")]
    pub raw_value: Option<String>,
}
impl FilterQueryOption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FilterQueryValidator {}
impl FilterQueryValidator {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmDirectValueAnnotationsManager {}
impl IEdmDirectValueAnnotationsManager {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmEntityContainer {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub elements: Vec<IEdmEntityContainerElement>,
    #[serde(rename = "schemaElementKind", default, skip_serializing_if = "Option::is_none")]
    pub schema_element_kind: Option<i_edm_entity_container::SchemaElementKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl IEdmEntityContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod i_edm_entity_container {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SchemaElementKind {
        None,
        TypeDefinition,
        Term,
        Action,
        EntityContainer,
        Function,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmEntityContainerElement {
    #[serde(rename = "containerElementKind", default, skip_serializing_if = "Option::is_none")]
    pub container_element_kind: Option<i_edm_entity_container_element::ContainerElementKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<IEdmEntityContainer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl IEdmEntityContainerElement {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod i_edm_entity_container_element {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ContainerElementKind {
        None,
        EntitySet,
        ActionImport,
        FunctionImport,
        Singleton,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmExpression {
    #[serde(rename = "expressionKind", default, skip_serializing_if = "Option::is_none")]
    pub expression_kind: Option<i_edm_expression::ExpressionKind>,
}
impl IEdmExpression {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod i_edm_expression {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ExpressionKind {
        None,
        BinaryConstant,
        BooleanConstant,
        DateTimeOffsetConstant,
        DecimalConstant,
        FloatingConstant,
        GuidConstant,
        IntegerConstant,
        StringConstant,
        DurationConstant,
        Null,
        Record,
        Collection,
        Path,
        If,
        Cast,
        IsType,
        FunctionApplication,
        LabeledExpressionReference,
        Labeled,
        PropertyPath,
        NavigationPropertyPath,
        DateConstant,
        TimeOfDayConstant,
        EnumMember,
        AnnotationPath,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmModel {
    #[serde(rename = "schemaElements", default, skip_serializing_if = "Vec::is_empty")]
    pub schema_elements: Vec<IEdmSchemaElement>,
    #[serde(rename = "vocabularyAnnotations", default, skip_serializing_if = "Vec::is_empty")]
    pub vocabulary_annotations: Vec<IEdmVocabularyAnnotation>,
    #[serde(rename = "referencedModels", default, skip_serializing_if = "Vec::is_empty")]
    pub referenced_models: Vec<IEdmModel>,
    #[serde(rename = "declaredNamespaces", default, skip_serializing_if = "Vec::is_empty")]
    pub declared_namespaces: Vec<String>,
    #[serde(rename = "directValueAnnotationsManager", default, skip_serializing_if = "Option::is_none")]
    pub direct_value_annotations_manager: Option<IEdmDirectValueAnnotationsManager>,
    #[serde(rename = "entityContainer", default, skip_serializing_if = "Option::is_none")]
    pub entity_container: Option<IEdmEntityContainer>,
}
impl IEdmModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmNavigationProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partner: Box<Option<IEdmNavigationProperty>>,
    #[serde(rename = "onDelete", default, skip_serializing_if = "Option::is_none")]
    pub on_delete: Option<i_edm_navigation_property::OnDelete>,
    #[serde(rename = "containsTarget", default, skip_serializing_if = "Option::is_none")]
    pub contains_target: Option<bool>,
    #[serde(rename = "referentialConstraint", default, skip_serializing_if = "Option::is_none")]
    pub referential_constraint: Option<IEdmReferentialConstraint>,
    #[serde(rename = "propertyKind", default, skip_serializing_if = "Option::is_none")]
    pub property_kind: Option<i_edm_navigation_property::PropertyKind>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<IEdmTypeReference>,
    #[serde(rename = "declaringType", default, skip_serializing_if = "Option::is_none")]
    pub declaring_type: Option<IEdmStructuredType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl IEdmNavigationProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod i_edm_navigation_property {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OnDelete {
        None,
        Cascade,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PropertyKind {
        None,
        Structural,
        Navigation,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmNavigationPropertyBinding {
    #[serde(rename = "navigationProperty", default, skip_serializing_if = "Option::is_none")]
    pub navigation_property: Option<IEdmNavigationProperty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<IEdmNavigationSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<IEdmPathExpression>,
}
impl IEdmNavigationPropertyBinding {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmNavigationSource {
    #[serde(rename = "navigationPropertyBindings", default, skip_serializing_if = "Vec::is_empty")]
    pub navigation_property_bindings: Vec<IEdmNavigationPropertyBinding>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<IEdmPathExpression>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<IEdmType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl IEdmNavigationSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmPathExpression {
    #[serde(rename = "pathSegments", default, skip_serializing_if = "Vec::is_empty")]
    pub path_segments: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "expressionKind", default, skip_serializing_if = "Option::is_none")]
    pub expression_kind: Option<i_edm_path_expression::ExpressionKind>,
}
impl IEdmPathExpression {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod i_edm_path_expression {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ExpressionKind {
        None,
        BinaryConstant,
        BooleanConstant,
        DateTimeOffsetConstant,
        DecimalConstant,
        FloatingConstant,
        GuidConstant,
        IntegerConstant,
        StringConstant,
        DurationConstant,
        Null,
        Record,
        Collection,
        Path,
        If,
        Cast,
        IsType,
        FunctionApplication,
        LabeledExpressionReference,
        Labeled,
        PropertyPath,
        NavigationPropertyPath,
        DateConstant,
        TimeOfDayConstant,
        EnumMember,
        AnnotationPath,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmProperty {
    #[serde(rename = "propertyKind", default, skip_serializing_if = "Option::is_none")]
    pub property_kind: Option<i_edm_property::PropertyKind>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<IEdmTypeReference>,
    #[serde(rename = "declaringType", default, skip_serializing_if = "Option::is_none")]
    pub declaring_type: Option<IEdmStructuredType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl IEdmProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod i_edm_property {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PropertyKind {
        None,
        Structural,
        Navigation,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmReferentialConstraint {
    #[serde(rename = "propertyPairs", default, skip_serializing_if = "Vec::is_empty")]
    pub property_pairs: Vec<EdmReferentialConstraintPropertyPair>,
}
impl IEdmReferentialConstraint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmSchemaElement {
    #[serde(rename = "schemaElementKind", default, skip_serializing_if = "Option::is_none")]
    pub schema_element_kind: Option<i_edm_schema_element::SchemaElementKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl IEdmSchemaElement {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod i_edm_schema_element {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SchemaElementKind {
        None,
        TypeDefinition,
        Term,
        Action,
        EntityContainer,
        Function,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmStructuralProperty {
    #[serde(rename = "defaultValueString", default, skip_serializing_if = "Option::is_none")]
    pub default_value_string: Option<String>,
    #[serde(rename = "propertyKind", default, skip_serializing_if = "Option::is_none")]
    pub property_kind: Option<i_edm_structural_property::PropertyKind>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<IEdmTypeReference>,
    #[serde(rename = "declaringType", default, skip_serializing_if = "Option::is_none")]
    pub declaring_type: Option<IEdmStructuredType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl IEdmStructuralProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod i_edm_structural_property {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PropertyKind {
        None,
        Structural,
        Navigation,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmStructuredType {
    #[serde(rename = "isAbstract", default, skip_serializing_if = "Option::is_none")]
    pub is_abstract: Option<bool>,
    #[serde(rename = "isOpen", default, skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
    #[serde(rename = "baseType", default, skip_serializing_if = "Option::is_none")]
    pub base_type: Box<Option<IEdmStructuredType>>,
    #[serde(rename = "declaredProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub declared_properties: Vec<IEdmProperty>,
    #[serde(rename = "typeKind", default, skip_serializing_if = "Option::is_none")]
    pub type_kind: Option<i_edm_structured_type::TypeKind>,
}
impl IEdmStructuredType {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod i_edm_structured_type {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeKind {
        None,
        Primitive,
        Entity,
        Complex,
        Collection,
        EntityReference,
        Enum,
        TypeDefinition,
        Untyped,
        Path,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmTerm {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<IEdmTypeReference>,
    #[serde(rename = "appliesTo", default, skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<String>,
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "schemaElementKind", default, skip_serializing_if = "Option::is_none")]
    pub schema_element_kind: Option<i_edm_term::SchemaElementKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl IEdmTerm {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod i_edm_term {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SchemaElementKind {
        None,
        TypeDefinition,
        Term,
        Action,
        EntityContainer,
        Function,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmType {
    #[serde(rename = "typeKind", default, skip_serializing_if = "Option::is_none")]
    pub type_kind: Option<i_edm_type::TypeKind>,
}
impl IEdmType {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod i_edm_type {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeKind {
        None,
        Primitive,
        Entity,
        Complex,
        Collection,
        EntityReference,
        Enum,
        TypeDefinition,
        Untyped,
        Path,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmTypeReference {
    #[serde(rename = "isNullable", default, skip_serializing_if = "Option::is_none")]
    pub is_nullable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<IEdmType>,
}
impl IEdmTypeReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmVocabularyAnnotatable {}
impl IEdmVocabularyAnnotatable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IEdmVocabularyAnnotation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<IEdmTerm>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<IEdmVocabularyAnnotatable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<IEdmExpression>,
}
impl IEdmVocabularyAnnotation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IServiceProvider {}
impl IServiceProvider {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Machine REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Machine {
    #[doc = "Gets or sets the relative URL to get to this REST resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name of this REST resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the type of this REST resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of the machine resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineProperties>,
}
impl Machine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineCollection {
    #[doc = "Gets or sets the machines."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Machine>,
    #[doc = "Gets or sets the value of nextLink."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl MachineCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the machine error resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineMigrateEventProperties {
    #[serde(flatten)]
    pub migrate_event_properties: MigrateEventProperties,
    #[doc = "Gets or sets the machine for which the error is being reported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub machine: Option<String>,
}
impl MachineMigrateEventProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the machine resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineProperties {
    #[doc = "Gets or sets the discovery details of the machine published by various sources."]
    #[serde(rename = "discoveryData", default, skip_serializing_if = "Vec::is_empty")]
    pub discovery_data: Vec<DiscoveryDetails>,
    #[doc = "Gets or sets the assessment details of the machine published by various sources."]
    #[serde(rename = "assessmentData", default, skip_serializing_if = "Vec::is_empty")]
    pub assessment_data: Vec<AssessmentDetails>,
    #[doc = "Gets or sets the migration details of the machine published by various sources."]
    #[serde(rename = "migrationData", default, skip_serializing_if = "Vec::is_empty")]
    pub migration_data: Vec<MigrationDetails>,
    #[doc = "Gets or sets the time of the last modification of the machine."]
    #[serde(rename = "lastUpdatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
}
impl MachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MigrateEvent REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateEvent {
    #[doc = "Gets or sets the relative URL to get to this REST resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name of this REST resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the type of this REST resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of the error resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MigrateEventProperties>,
}
impl MigrateEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the error resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateEventProperties {
    #[doc = "Gets the Instance type."]
    #[serde(rename = "instanceType", default, skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[doc = "Gets or sets the error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Gets or sets the error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Gets or sets the recommendation for the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<String>,
    #[doc = "Gets or sets the possible causes for the error."]
    #[serde(rename = "possibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub possible_causes: Option<String>,
    #[doc = "Gets or sets the solution for which the error is being reported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solution: Option<String>,
    #[doc = "Gets or sets the client request Id of the payload for which the event is being reported."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
}
impl MigrateEventProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Migrate Project REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateProject {
    #[doc = "Gets or sets the eTag for concurrency control."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Gets or sets the Azure location in which migrate project is created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Class for migrate project properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MigrateProjectProperties>,
    #[doc = "Gets the relative URL to get this migrate project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the name of the migrate project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Handled by resource provider. Type = Microsoft.Migrate/MigrateProject."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets the tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<migrate_project::Tags>,
}
impl MigrateProject {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod migrate_project {
    use super::*;
    #[doc = "Gets or sets the tags."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Tags {
        #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
        pub additional_properties: Option<String>,
    }
    impl Tags {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Class for migrate project properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateProjectProperties {
    #[doc = "Gets or sets the list of tools registered with the migrate project."]
    #[serde(rename = "registeredTools", default, skip_serializing_if = "Vec::is_empty")]
    pub registered_tools: Vec<String>,
    #[doc = "Gets the summary of the migrate project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<serde_json::Value>,
    #[doc = "Gets the last time the project summary was refreshed."]
    #[serde(rename = "lastSummaryRefreshedTime", with = "azure_core::date::rfc3339::option")]
    pub last_summary_refreshed_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the refresh summary state."]
    #[serde(rename = "refreshSummaryState", default, skip_serializing_if = "Option::is_none")]
    pub refresh_summary_state: Option<migrate_project_properties::RefreshSummaryState>,
    #[doc = "Provisioning state of the migrate project."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<migrate_project_properties::ProvisioningState>,
}
impl MigrateProjectProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod migrate_project_properties {
    use super::*;
    #[doc = "Gets the refresh summary state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RefreshSummaryState {
        Started,
        InProgress,
        Completed,
        Failed,
    }
    #[doc = "Provisioning state of the migrate project."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Accepted,
        Creating,
        Deleting,
        Failed,
        Moving,
        Succeeded,
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
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Accepted"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Moving"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Succeeded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Migration properties that can be shared by various publishers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationDetails {
    #[doc = "Gets or sets the phase of migration of the machine."]
    #[serde(rename = "migrationPhase", default, skip_serializing_if = "Option::is_none")]
    pub migration_phase: Option<String>,
    #[doc = "Gets or sets a value indicating whether migration was tested on the machine."]
    #[serde(rename = "migrationTested", default, skip_serializing_if = "Option::is_none")]
    pub migration_tested: Option<bool>,
    #[doc = "Gets or sets the progress percentage of migration on the machine."]
    #[serde(rename = "replicationProgressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub replication_progress_percentage: Option<i32>,
    #[doc = "Gets or sets the ARM id the migrated VM."]
    #[serde(rename = "targetVMArmId", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_arm_id: Option<String>,
    #[doc = "Gets or sets the time the message was enqueued."]
    #[serde(rename = "enqueueTime", default, skip_serializing_if = "Option::is_none")]
    pub enqueue_time: Option<String>,
    #[doc = "Gets or sets the name of the solution that sent the data."]
    #[serde(rename = "solutionName", default, skip_serializing_if = "Option::is_none")]
    pub solution_name: Option<String>,
    #[doc = "Gets or sets the unique identifier of the machine."]
    #[serde(rename = "machineId", default, skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,
    #[doc = "Gets or sets the unique identifier of the virtual machine manager(vCenter/VMM)."]
    #[serde(rename = "machineManagerId", default, skip_serializing_if = "Option::is_none")]
    pub machine_manager_id: Option<String>,
    #[doc = "Gets or sets the fabric type."]
    #[serde(rename = "fabricType", default, skip_serializing_if = "Option::is_none")]
    pub fabric_type: Option<String>,
    #[doc = "Gets or sets the time of the last modification of the machine details."]
    #[serde(rename = "lastUpdatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the name of the machine."]
    #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[doc = "Gets or sets the list of IP addresses of the machine. IP addresses could be IP V4 or IP V6."]
    #[serde(rename = "ipAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_addresses: Vec<String>,
    #[doc = "Gets or sets the FQDN of the machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Gets or sets the BIOS ID of the machine."]
    #[serde(rename = "biosId", default, skip_serializing_if = "Option::is_none")]
    pub bios_id: Option<String>,
    #[doc = "Gets or sets the list of MAC addresses of the machine."]
    #[serde(rename = "macAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub mac_addresses: Vec<String>,
    #[doc = "Gets or sets the ISV specific extended information."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<serde_json::Value>,
}
impl MigrationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ODataPath {
    #[serde(rename = "edmType", default, skip_serializing_if = "Option::is_none")]
    pub edm_type: Option<IEdmType>,
    #[serde(rename = "navigationSource", default, skip_serializing_if = "Option::is_none")]
    pub navigation_source: Option<IEdmNavigationSource>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub segments: Vec<ODataPathSegment>,
    #[serde(rename = "pathTemplate", default, skip_serializing_if = "Option::is_none")]
    pub path_template: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub path: Vec<ODataPathSegment>,
}
impl ODataPath {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ODataPathSegment {
    #[serde(rename = "edmType", default, skip_serializing_if = "Option::is_none")]
    pub edm_type: Option<IEdmType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}
impl ODataPathSegment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ODataQueryContext {
    #[serde(rename = "defaultQuerySettings", default, skip_serializing_if = "Option::is_none")]
    pub default_query_settings: Option<DefaultQuerySettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<IEdmModel>,
    #[serde(rename = "elementType", default, skip_serializing_if = "Option::is_none")]
    pub element_type: Option<IEdmType>,
    #[serde(rename = "navigationSource", default, skip_serializing_if = "Option::is_none")]
    pub navigation_source: Option<IEdmNavigationSource>,
    #[serde(rename = "elementClrType", default, skip_serializing_if = "Option::is_none")]
    pub element_clr_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<ODataPath>,
    #[serde(rename = "requestContainer", default, skip_serializing_if = "Option::is_none")]
    pub request_container: Option<IServiceProvider>,
}
impl ODataQueryContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ODataQueryOptions1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<FilterQueryOption>,
}
impl ODataQueryOptions1 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ODataQueryValidator {}
impl ODataQueryValidator {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ODataRawQueryOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}
impl ODataRawQueryOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Object {}
impl Object {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A REST API operation supported by the provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Displayable properties of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Origin of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Displayable properties of the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Provider of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource operated on by the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operation Type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of API operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResultList {
    #[doc = "List of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationResultList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationResultList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The project summary class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectSummary {
    #[doc = "Gets the Instance type."]
    #[serde(rename = "instanceType", default, skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[doc = "Gets or sets the state of refresh summary."]
    #[serde(rename = "refreshSummaryState", default, skip_serializing_if = "Option::is_none")]
    pub refresh_summary_state: Option<project_summary::RefreshSummaryState>,
    #[doc = "Gets or sets the time when summary was last refreshed."]
    #[serde(rename = "lastSummaryRefreshedTime", with = "azure_core::date::rfc3339::option")]
    pub last_summary_refreshed_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the extended summary."]
    #[serde(rename = "extendedSummary", default, skip_serializing_if = "Option::is_none")]
    pub extended_summary: Option<serde_json::Value>,
}
impl ProjectSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod project_summary {
    use super::*;
    #[doc = "Gets or sets the state of refresh summary."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RefreshSummaryState {
        Started,
        InProgress,
        Completed,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RangeVariable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "typeReference", default, skip_serializing_if = "Option::is_none")]
    pub type_reference: Option<IEdmTypeReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<i32>,
}
impl RangeVariable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the refresh summary input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefreshSummaryInput {
    #[doc = "Gets or sets the goal for which summary needs to be refreshed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goal: Option<refresh_summary_input::Goal>,
}
impl RefreshSummaryInput {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod refresh_summary_input {
    use super::*;
    #[doc = "Gets or sets the goal for which summary needs to be refreshed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Goal {
        Servers,
        Databases,
    }
}
#[doc = "Class representing the refresh summary status of the migrate project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefreshSummaryResult {
    #[doc = "Gets or sets a value indicating whether the migrate project summary is refreshed."]
    #[serde(rename = "isRefreshed", default, skip_serializing_if = "Option::is_none")]
    pub is_refreshed: Option<bool>,
}
impl RefreshSummaryResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the register tool input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegisterToolInput {
    #[doc = "Gets or sets the tool to be registered."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<register_tool_input::Tool>,
}
impl RegisterToolInput {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod register_tool_input {
    use super::*;
    #[doc = "Gets or sets the tool to be registered."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tool {
        ServerDiscovery,
        ServerAssessment,
        ServerMigration,
        Cloudamize,
        Turbonomic,
        Zerto,
        CorentTech,
        ServerAssessmentV1,
        #[serde(rename = "ServerMigration_Replication")]
        ServerMigrationReplication,
        Carbonite,
        DataMigrationAssistant,
        DatabaseMigrationService,
    }
}
#[doc = "Class representing the registration status of a tool with the migrate project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationResult {
    #[doc = "Gets or sets a value indicating whether the tool is registered or not."]
    #[serde(rename = "isRegistered", default, skip_serializing_if = "Option::is_none")]
    pub is_registered: Option<bool>,
}
impl RegistrationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SelectExpandClause {
    #[serde(rename = "selectedItems", default, skip_serializing_if = "Vec::is_empty")]
    pub selected_items: Vec<SelectItem>,
    #[serde(rename = "allSelected", default, skip_serializing_if = "Option::is_none")]
    pub all_selected: Option<bool>,
}
impl SelectExpandClause {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SelectExpandQueryValidator {}
impl SelectExpandQueryValidator {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SelectItem {}
impl SelectItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the servers project summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServersProjectSummary {
    #[serde(flatten)]
    pub project_summary: ProjectSummary,
    #[doc = "Gets or sets the count of entities discovered."]
    #[serde(rename = "discoveredCount", default, skip_serializing_if = "Option::is_none")]
    pub discovered_count: Option<i32>,
    #[doc = "Gets or sets the count of entities assessed."]
    #[serde(rename = "assessedCount", default, skip_serializing_if = "Option::is_none")]
    pub assessed_count: Option<i32>,
    #[doc = "Gets or sets the count of entities being replicated."]
    #[serde(rename = "replicatingCount", default, skip_serializing_if = "Option::is_none")]
    pub replicating_count: Option<i32>,
    #[doc = "Gets or sets the count of entities test migrated."]
    #[serde(rename = "testMigratedCount", default, skip_serializing_if = "Option::is_none")]
    pub test_migrated_count: Option<i32>,
    #[doc = "Gets or sets the count of entities migrated."]
    #[serde(rename = "migratedCount", default, skip_serializing_if = "Option::is_none")]
    pub migrated_count: Option<i32>,
}
impl ServersProjectSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the servers solution summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServersSolutionSummary {
    #[serde(flatten)]
    pub solution_summary: SolutionSummary,
    #[doc = "Gets or sets the count of servers discovered."]
    #[serde(rename = "discoveredCount", default, skip_serializing_if = "Option::is_none")]
    pub discovered_count: Option<i32>,
    #[doc = "Gets or sets the count of servers assessed."]
    #[serde(rename = "assessedCount", default, skip_serializing_if = "Option::is_none")]
    pub assessed_count: Option<i32>,
    #[doc = "Gets or sets the count of servers being replicated."]
    #[serde(rename = "replicatingCount", default, skip_serializing_if = "Option::is_none")]
    pub replicating_count: Option<i32>,
    #[doc = "Gets or sets the count of servers test migrated."]
    #[serde(rename = "testMigratedCount", default, skip_serializing_if = "Option::is_none")]
    pub test_migrated_count: Option<i32>,
    #[doc = "Gets or sets the count of servers migrated."]
    #[serde(rename = "migratedCount", default, skip_serializing_if = "Option::is_none")]
    pub migrated_count: Option<i32>,
}
impl ServersSolutionSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SingleValueNode {
    #[serde(rename = "typeReference", default, skip_serializing_if = "Option::is_none")]
    pub type_reference: Option<IEdmTypeReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<single_value_node::Kind>,
}
impl SingleValueNode {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod single_value_node {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        None,
        Constant,
        Convert,
        NonResourceRangeVariableReference,
        BinaryOperator,
        UnaryOperator,
        SingleValuePropertyAccess,
        CollectionPropertyAccess,
        SingleValueFunctionCall,
        Any,
        CollectionNavigationNode,
        SingleNavigationNode,
        SingleValueOpenPropertyAccess,
        SingleResourceCast,
        All,
        CollectionResourceCast,
        ResourceRangeVariableReference,
        SingleResourceFunctionCall,
        CollectionFunctionCall,
        CollectionResourceFunctionCall,
        NamedFunctionParameter,
        ParameterAlias,
        EntitySet,
        KeyLookup,
        SearchTerm,
        CollectionOpenPropertyAccess,
        CollectionComplexNode,
        SingleComplexNode,
        Count,
        SingleValueCast,
        CollectionPropertyNode,
        AggregatedCollectionPropertyNode,
        In,
        CollectionConstant,
    }
}
#[doc = "Solution REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Solution {
    #[doc = "Gets the relative URL to get to this REST resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the name of this REST resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the type of this REST resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets the ETAG for optimistic concurrency control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Class for solution properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SolutionProperties>,
}
impl Solution {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the config for the solution in the migrate project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionConfig {
    #[doc = "Gets or sets the publisher sas uri for the solution."]
    #[serde(rename = "publisherSasUri", default, skip_serializing_if = "Option::is_none")]
    pub publisher_sas_uri: Option<String>,
}
impl SolutionConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the details of the solution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionDetails {
    #[doc = "Gets or sets the count of groups reported by the solution."]
    #[serde(rename = "groupCount", default, skip_serializing_if = "Option::is_none")]
    pub group_count: Option<i32>,
    #[doc = "Gets or sets the count of assessments reported by the solution."]
    #[serde(rename = "assessmentCount", default, skip_serializing_if = "Option::is_none")]
    pub assessment_count: Option<i32>,
    #[doc = "Gets or sets the extended details reported by the solution."]
    #[serde(rename = "extendedDetails", default, skip_serializing_if = "Option::is_none")]
    pub extended_details: Option<serde_json::Value>,
}
impl SolutionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for solution properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionProperties {
    #[doc = "Gets or sets the tool being used in the solution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<solution_properties::Tool>,
    #[doc = "Gets or sets the purpose of the solution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<solution_properties::Purpose>,
    #[doc = "Gets or sets the goal of the solution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goal: Option<solution_properties::Goal>,
    #[doc = "Gets or sets the current status of the solution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<solution_properties::Status>,
    #[doc = "Gets or sets the cleanup state of the solution."]
    #[serde(rename = "cleanupState", default, skip_serializing_if = "Option::is_none")]
    pub cleanup_state: Option<solution_properties::CleanupState>,
    #[doc = "The solution summary class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<SolutionSummary>,
    #[doc = "Class representing the details of the solution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<SolutionDetails>,
}
impl SolutionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod solution_properties {
    use super::*;
    #[doc = "Gets or sets the tool being used in the solution."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tool {
        ServerDiscovery,
        ServerAssessment,
        ServerMigration,
        Cloudamize,
        Turbonomic,
        Zerto,
        CorentTech,
        ServerAssessmentV1,
        #[serde(rename = "ServerMigration_Replication")]
        ServerMigrationReplication,
        Carbonite,
        DataMigrationAssistant,
        DatabaseMigrationService,
    }
    #[doc = "Gets or sets the purpose of the solution."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Purpose {
        Discovery,
        Assessment,
        Migration,
    }
    #[doc = "Gets or sets the goal of the solution."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Goal {
        Servers,
        Databases,
    }
    #[doc = "Gets or sets the current status of the solution."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Inactive,
        Active,
    }
    #[doc = "Gets or sets the cleanup state of the solution."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CleanupState {
        None,
        Started,
        InProgress,
        Completed,
        Failed,
    }
}
#[doc = "The solution summary class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionSummary {
    #[doc = "Gets the Instance type."]
    #[serde(rename = "instanceType", default, skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
}
impl SolutionSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of solutions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionsCollection {
    #[doc = "Gets or sets the list of solutions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Solution>,
    #[doc = "Gets or sets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SolutionsCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransformationNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<transformation_node::Kind>,
}
impl TransformationNode {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod transformation_node {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Aggregate,
        GroupBy,
        Filter,
        Compute,
    }
}
