#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Represents AADIP (Azure Active Directory Identity Protection) requirements check request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AadCheckRequirements {
    #[doc = "AADIP (Azure Active Directory Identity Protection) requirements check properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AadCheckRequirementsProperties>,
}
impl AadCheckRequirements {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "AADIP (Azure Active Directory Identity Protection) requirements check properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AadCheckRequirementsProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
}
impl AadCheckRequirementsProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self { data_connector_tenant_id }
    }
}
#[doc = "Represents AADIP (Azure Active Directory Identity Protection) data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AadDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "AADIP (Azure Active Directory Identity Protection) data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AadDataConnectorProperties>,
}
impl AadDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "AADIP (Azure Active Directory Identity Protection) data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AadDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[serde(flatten)]
    pub data_connector_with_alerts_properties: DataConnectorWithAlertsProperties,
}
impl AadDataConnectorProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self {
            data_connector_tenant_id,
            data_connector_with_alerts_properties: DataConnectorWithAlertsProperties::default(),
        }
    }
}
#[doc = "Represents AATP (Azure Advanced Threat Protection) requirements check request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AatpCheckRequirements {
    #[doc = "AATP (Azure Advanced Threat Protection) requirements check properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AatpCheckRequirementsProperties>,
}
impl AatpCheckRequirements {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "AATP (Azure Advanced Threat Protection) requirements check properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AatpCheckRequirementsProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
}
impl AatpCheckRequirementsProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self { data_connector_tenant_id }
    }
}
#[doc = "Represents AATP (Azure Advanced Threat Protection) data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AatpDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "AATP (Azure Advanced Threat Protection) data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AatpDataConnectorProperties>,
}
impl AatpDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "AATP (Azure Advanced Threat Protection) data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AatpDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[serde(flatten)]
    pub data_connector_with_alerts_properties: DataConnectorWithAlertsProperties,
}
impl AatpDataConnectorProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self {
            data_connector_tenant_id,
            data_connector_with_alerts_properties: DataConnectorWithAlertsProperties::default(),
        }
    }
}
#[doc = "Represents ASC (Azure Security Center) requirements check request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AscCheckRequirements {
    #[doc = "ASC (Azure Security Center) requirements check properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AscCheckRequirementsProperties>,
}
impl AscCheckRequirements {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "ASC (Azure Security Center) requirements check properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AscCheckRequirementsProperties {
    #[doc = "The subscription id to connect to, and get the data from."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}
impl AscCheckRequirementsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents ASC (Azure Security Center) data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AscDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "ASC (Azure Security Center) data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AscDataConnectorProperties>,
}
impl AscDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "ASC (Azure Security Center) data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AscDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_with_alerts_properties: DataConnectorWithAlertsProperties,
    #[doc = "The subscription id to connect to, and get the data from."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}
impl AscDataConnectorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Action property bag base."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionPropertiesBase {
    #[doc = "Logic App Resource Id, /subscriptions/{my-subscription}/resourceGroups/{my-resource-group}/providers/Microsoft.Logic/workflows/{my-workflow-id}."]
    #[serde(rename = "logicAppResourceId")]
    pub logic_app_resource_id: String,
}
impl ActionPropertiesBase {
    pub fn new(logic_app_resource_id: String) -> Self {
        Self { logic_app_resource_id }
    }
}
#[doc = "Action for alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionRequest {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "Action property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionRequestProperties>,
}
impl ActionRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Action property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionRequestProperties {
    #[serde(flatten)]
    pub action_properties_base: ActionPropertiesBase,
    #[doc = "Logic App Callback URL for this specific workflow."]
    #[serde(rename = "triggerUri")]
    pub trigger_uri: String,
}
impl ActionRequestProperties {
    pub fn new(action_properties_base: ActionPropertiesBase, trigger_uri: String) -> Self {
        Self {
            action_properties_base,
            trigger_uri,
        }
    }
}
#[doc = "Action for alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionResponse {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "Action property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionResponseProperties>,
}
impl ActionResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Action property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionResponseProperties {
    #[serde(flatten)]
    pub action_properties_base: ActionPropertiesBase,
    #[doc = "The name of the logic app's workflow."]
    #[serde(rename = "workflowId", default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}
impl ActionResponseProperties {
    pub fn new(action_properties_base: ActionPropertiesBase) -> Self {
        Self {
            action_properties_base,
            workflow_id: None,
        }
    }
}
#[doc = "Represents the state the recommendation action is currently in."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ActionState")]
pub enum ActionState {
    Active,
    InProgress,
    Done,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ActionState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ActionState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ActionState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("ActionState", 0u32, "Active"),
            Self::InProgress => serializer.serialize_unit_variant("ActionState", 1u32, "InProgress"),
            Self::Done => serializer.serialize_unit_variant("ActionState", 2u32, "Done"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type of the automation rule action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ActionType")]
pub enum ActionType {
    ModifyProperties,
    RunPlaybook,
    AddIncidentTask,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ActionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ActionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ActionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ModifyProperties => serializer.serialize_unit_variant("ActionType", 0u32, "ModifyProperties"),
            Self::RunPlaybook => serializer.serialize_unit_variant("ActionType", 1u32, "RunPlaybook"),
            Self::AddIncidentTask => serializer.serialize_unit_variant("ActionType", 2u32, "AddIncidentTask"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "List all the actions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionsList {
    #[doc = "URL to fetch the next set of actions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of actions."]
    pub value: Vec<ActionResponse>,
}
impl azure_core::Continuable for ActionsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ActionsList {
    pub fn new(value: Vec<ActionResponse>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Represents Activity entity query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityCustomEntityQuery {
    #[serde(flatten)]
    pub custom_entity_query: CustomEntityQuery,
    #[doc = "Describes activity entity query properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActivityEntityQueriesProperties>,
}
impl ActivityCustomEntityQuery {
    pub fn new(custom_entity_query: CustomEntityQuery) -> Self {
        Self {
            custom_entity_query,
            properties: None,
        }
    }
}
#[doc = "Describes activity entity query properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityEntityQueriesProperties {
    #[doc = "The entity query title"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The entity query content to display in timeline"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "The entity query description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The Activity query definitions"]
    #[serde(rename = "queryDefinitions", default, skip_serializing_if = "Option::is_none")]
    pub query_definitions: Option<activity_entity_queries_properties::QueryDefinitions>,
    #[doc = "The type of the entity"]
    #[serde(rename = "inputEntityType", default, skip_serializing_if = "Option::is_none")]
    pub input_entity_type: Option<EntityInnerType>,
    #[doc = "List of the fields of the source entity that are required to run the query"]
    #[serde(
        rename = "requiredInputFieldsSets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_input_fields_sets: Vec<Vec<String>>,
    #[doc = "The query applied only to entities matching to all filters"]
    #[serde(rename = "entitiesFilter", default, skip_serializing_if = "Option::is_none")]
    pub entities_filter: Option<serde_json::Value>,
    #[doc = "The template id this activity was created from"]
    #[serde(rename = "templateName", default, skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[doc = "Determines whether this activity is enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The time the activity was created"]
    #[serde(rename = "createdTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub created_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The last time the activity was updated"]
    #[serde(rename = "lastModifiedTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_time_utc: Option<time::OffsetDateTime>,
}
impl ActivityEntityQueriesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod activity_entity_queries_properties {
    use super::*;
    #[doc = "The Activity query definitions"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct QueryDefinitions {
        #[doc = "The Activity query to run on a given entity"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub query: Option<String>,
    }
    impl QueryDefinitions {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Represents Activity entity query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityEntityQuery {
    #[serde(flatten)]
    pub entity_query: EntityQuery,
    #[doc = "Describes activity entity query properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActivityEntityQueriesProperties>,
}
impl ActivityEntityQuery {
    pub fn new(entity_query: EntityQuery) -> Self {
        Self {
            entity_query,
            properties: None,
        }
    }
}
#[doc = "Represents Activity entity query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityEntityQueryTemplate {
    #[serde(flatten)]
    pub entity_query_template: EntityQueryTemplate,
    #[doc = "Describes activity entity query properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActivityEntityQueryTemplateProperties>,
}
impl ActivityEntityQueryTemplate {
    pub fn new(entity_query_template: EntityQueryTemplate) -> Self {
        Self {
            entity_query_template,
            properties: None,
        }
    }
}
#[doc = "Describes activity entity query properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityEntityQueryTemplateProperties {
    #[doc = "The entity query title"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The entity query content to display in timeline"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "The entity query description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The Activity query definitions"]
    #[serde(rename = "queryDefinitions", default, skip_serializing_if = "Option::is_none")]
    pub query_definitions: Option<activity_entity_query_template_properties::QueryDefinitions>,
    #[doc = "List of required data types for the given entity query template"]
    #[serde(
        rename = "dataTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_types: Vec<DataTypeDefinitions>,
    #[doc = "The type of the entity"]
    #[serde(rename = "inputEntityType", default, skip_serializing_if = "Option::is_none")]
    pub input_entity_type: Option<EntityInnerType>,
    #[doc = "List of the fields of the source entity that are required to run the query"]
    #[serde(
        rename = "requiredInputFieldsSets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_input_fields_sets: Vec<Vec<String>>,
    #[doc = "The query applied only to entities matching to all filters"]
    #[serde(rename = "entitiesFilter", default, skip_serializing_if = "Option::is_none")]
    pub entities_filter: Option<serde_json::Value>,
}
impl ActivityEntityQueryTemplateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod activity_entity_query_template_properties {
    use super::*;
    #[doc = "The Activity query definitions"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct QueryDefinitions {
        #[doc = "The Activity query to run on a given entity"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub query: Option<String>,
        #[doc = "The dimensions we want to summarize the timeline results on, this is comma separated list"]
        #[serde(rename = "summarizeBy", default, skip_serializing_if = "Option::is_none")]
        pub summarize_by: Option<String>,
    }
    impl QueryDefinitions {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Represents Activity timeline item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityTimelineItem {
    #[doc = "The activity query id."]
    #[serde(rename = "queryId")]
    pub query_id: String,
    #[doc = "The grouping bucket start time."]
    #[serde(rename = "bucketStartTimeUTC", with = "azure_core::date::rfc3339")]
    pub bucket_start_time_utc: time::OffsetDateTime,
    #[doc = "The grouping bucket end time."]
    #[serde(rename = "bucketEndTimeUTC", with = "azure_core::date::rfc3339")]
    pub bucket_end_time_utc: time::OffsetDateTime,
    #[doc = "The time of the first activity in the grouping bucket."]
    #[serde(rename = "firstActivityTimeUTC", with = "azure_core::date::rfc3339")]
    pub first_activity_time_utc: time::OffsetDateTime,
    #[doc = "The time of the last activity in the grouping bucket."]
    #[serde(rename = "lastActivityTimeUTC", with = "azure_core::date::rfc3339")]
    pub last_activity_time_utc: time::OffsetDateTime,
    #[doc = "The activity timeline content."]
    pub content: String,
    #[doc = "The activity timeline title."]
    pub title: String,
}
impl ActivityTimelineItem {
    pub fn new(
        query_id: String,
        bucket_start_time_utc: time::OffsetDateTime,
        bucket_end_time_utc: time::OffsetDateTime,
        first_activity_time_utc: time::OffsetDateTime,
        last_activity_time_utc: time::OffsetDateTime,
        content: String,
        title: String,
    ) -> Self {
        Self {
            query_id,
            bucket_start_time_utc,
            bucket_end_time_utc,
            first_activity_time_utc,
            last_activity_time_utc,
            content,
            title,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddIncidentTaskActionProperties {
    #[doc = "The title of the task."]
    pub title: String,
    #[doc = "The description of the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AddIncidentTaskActionProperties {
    pub fn new(title: String) -> Self {
        Self { title, description: None }
    }
}
#[doc = "Settings for how to dynamically override alert static details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertDetailsOverride {
    #[doc = "the format containing columns name(s) to override the alert name"]
    #[serde(rename = "alertDisplayNameFormat", default, skip_serializing_if = "Option::is_none")]
    pub alert_display_name_format: Option<String>,
    #[doc = "the format containing columns name(s) to override the alert description"]
    #[serde(rename = "alertDescriptionFormat", default, skip_serializing_if = "Option::is_none")]
    pub alert_description_format: Option<String>,
    #[doc = "the column name to take the alert tactics from"]
    #[serde(rename = "alertTacticsColumnName", default, skip_serializing_if = "Option::is_none")]
    pub alert_tactics_column_name: Option<String>,
    #[doc = "the column name to take the alert severity from"]
    #[serde(rename = "alertSeverityColumnName", default, skip_serializing_if = "Option::is_none")]
    pub alert_severity_column_name: Option<String>,
    #[doc = "List of additional dynamic properties to override"]
    #[serde(
        rename = "alertDynamicProperties",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub alert_dynamic_properties: Vec<AlertPropertyMapping>,
}
impl AlertDetailsOverride {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The V3 alert property"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AlertProperty")]
pub enum AlertProperty {
    AlertLink,
    ConfidenceLevel,
    ConfidenceScore,
    ExtendedLinks,
    ProductName,
    ProviderName,
    ProductComponentName,
    RemediationSteps,
    Techniques,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AlertProperty {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AlertProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AlertProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AlertLink => serializer.serialize_unit_variant("AlertProperty", 0u32, "AlertLink"),
            Self::ConfidenceLevel => serializer.serialize_unit_variant("AlertProperty", 1u32, "ConfidenceLevel"),
            Self::ConfidenceScore => serializer.serialize_unit_variant("AlertProperty", 2u32, "ConfidenceScore"),
            Self::ExtendedLinks => serializer.serialize_unit_variant("AlertProperty", 3u32, "ExtendedLinks"),
            Self::ProductName => serializer.serialize_unit_variant("AlertProperty", 4u32, "ProductName"),
            Self::ProviderName => serializer.serialize_unit_variant("AlertProperty", 5u32, "ProviderName"),
            Self::ProductComponentName => serializer.serialize_unit_variant("AlertProperty", 6u32, "ProductComponentName"),
            Self::RemediationSteps => serializer.serialize_unit_variant("AlertProperty", 7u32, "RemediationSteps"),
            Self::Techniques => serializer.serialize_unit_variant("AlertProperty", 8u32, "Techniques"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A single alert property mapping to override"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertPropertyMapping {
    #[doc = "The V3 alert property"]
    #[serde(rename = "alertProperty", default, skip_serializing_if = "Option::is_none")]
    pub alert_property: Option<AlertProperty>,
    #[doc = "the column name to use to override this property"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl AlertPropertyMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRule {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
}
impl AlertRule {
    pub fn new() -> Self {
        Self {
            resource_with_etag: ResourceWithEtag::default(),
        }
    }
}
#[doc = "The kind of the alert rule"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum AlertRuleUnion {
    Fusion(FusionAlertRule),
    #[serde(rename = "MLBehaviorAnalytics")]
    MlBehaviorAnalytics(MlBehaviorAnalyticsAlertRule),
    MicrosoftSecurityIncidentCreation(MicrosoftSecurityIncidentCreationAlertRule),
    #[serde(rename = "NRT")]
    Nrt(NrtAlertRule),
    Scheduled(ScheduledAlertRule),
    ThreatIntelligence(ThreatIntelligenceAlertRule),
}
#[doc = "The kind of the alert rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AlertRuleKindEnum")]
pub enum AlertRuleKindEnum {
    Scheduled,
    MicrosoftSecurityIncidentCreation,
    Fusion,
    #[serde(rename = "MLBehaviorAnalytics")]
    MlBehaviorAnalytics,
    ThreatIntelligence,
    #[serde(rename = "NRT")]
    Nrt,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AlertRuleKindEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AlertRuleKindEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AlertRuleKindEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Scheduled => serializer.serialize_unit_variant("AlertRuleKindEnum", 0u32, "Scheduled"),
            Self::MicrosoftSecurityIncidentCreation => {
                serializer.serialize_unit_variant("AlertRuleKindEnum", 1u32, "MicrosoftSecurityIncidentCreation")
            }
            Self::Fusion => serializer.serialize_unit_variant("AlertRuleKindEnum", 2u32, "Fusion"),
            Self::MlBehaviorAnalytics => serializer.serialize_unit_variant("AlertRuleKindEnum", 3u32, "MLBehaviorAnalytics"),
            Self::ThreatIntelligence => serializer.serialize_unit_variant("AlertRuleKindEnum", 4u32, "ThreatIntelligence"),
            Self::Nrt => serializer.serialize_unit_variant("AlertRuleKindEnum", 5u32, "NRT"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Alert rule template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleTemplate {
    #[serde(flatten)]
    pub resource: Resource,
}
impl AlertRuleTemplate {
    pub fn new() -> Self {
        Self {
            resource: Resource::default(),
        }
    }
}
#[doc = "The kind of the alert rule"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum AlertRuleTemplateUnion {
    Fusion(FusionAlertRuleTemplate),
    #[serde(rename = "MLBehaviorAnalytics")]
    MlBehaviorAnalytics(MlBehaviorAnalyticsAlertRuleTemplate),
    MicrosoftSecurityIncidentCreation(MicrosoftSecurityIncidentCreationAlertRuleTemplate),
    #[serde(rename = "NRT")]
    Nrt(NrtAlertRuleTemplate),
    Scheduled(ScheduledAlertRuleTemplate),
    ThreatIntelligence(ThreatIntelligenceAlertRuleTemplate),
}
#[doc = "alert rule template data sources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRuleTemplateDataSource {
    #[doc = "The connector id that provides the following data types"]
    #[serde(rename = "connectorId", default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[doc = "The data types used by the alert rule template"]
    #[serde(
        rename = "dataTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_types: Vec<String>,
}
impl AlertRuleTemplateDataSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base alert rule template property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRuleTemplatePropertiesBase {
    #[doc = "the number of alert rules that were created by this template"]
    #[serde(rename = "alertRulesCreatedByTemplateCount", default, skip_serializing_if = "Option::is_none")]
    pub alert_rules_created_by_template_count: Option<i32>,
    #[doc = "The last time that this alert rule template has been updated."]
    #[serde(rename = "lastUpdatedDateUTC", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_date_utc: Option<time::OffsetDateTime>,
    #[doc = "The time that this alert rule template has been added."]
    #[serde(rename = "createdDateUTC", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_utc: Option<time::OffsetDateTime>,
    #[doc = "The description of the alert rule template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display name for alert rule template."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The required data sources for this template"]
    #[serde(
        rename = "requiredDataConnectors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_data_connectors: Vec<AlertRuleTemplateDataSource>,
    #[doc = "The alert rule template status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AlertRuleTemplateStatus>,
}
impl AlertRuleTemplatePropertiesBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The alert rule template status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AlertRuleTemplateStatus")]
pub enum AlertRuleTemplateStatus {
    Installed,
    Available,
    NotAvailable,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AlertRuleTemplateStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AlertRuleTemplateStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AlertRuleTemplateStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Installed => serializer.serialize_unit_variant("AlertRuleTemplateStatus", 0u32, "Installed"),
            Self::Available => serializer.serialize_unit_variant("AlertRuleTemplateStatus", 1u32, "Available"),
            Self::NotAvailable => serializer.serialize_unit_variant("AlertRuleTemplateStatus", 2u32, "NotAvailable"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Alert rule template with MITRE property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRuleTemplateWithMitreProperties {
    #[serde(flatten)]
    pub alert_rule_template_properties_base: AlertRuleTemplatePropertiesBase,
    #[doc = "The tactics of the alert rule"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tactics: Vec<AttackTactic>,
    #[doc = "The techniques of the alert rule"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub techniques: Vec<String>,
}
impl AlertRuleTemplateWithMitreProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List all the alert rule templates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleTemplatesList {
    #[doc = "URL to fetch the next set of alert rule templates."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of alert rule templates."]
    pub value: Vec<AlertRuleTemplateUnion>,
}
impl azure_core::Continuable for AlertRuleTemplatesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AlertRuleTemplatesList {
    pub fn new(value: Vec<AlertRuleTemplateUnion>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "The operation against the threshold that triggers alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AlertRuleTriggerOperator {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
}
#[doc = "List all the alert rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRulesList {
    #[doc = "URL to fetch the next set of alert rules."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of alert rules."]
    pub value: Vec<AlertRuleUnion>,
}
impl azure_core::Continuable for AlertRulesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AlertRulesList {
    pub fn new(value: Vec<AlertRuleUnion>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "The severity of the alert"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AlertSeverityEnum")]
pub enum AlertSeverityEnum {
    High,
    Medium,
    Low,
    Informational,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AlertSeverityEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AlertSeverityEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AlertSeverityEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::High => serializer.serialize_unit_variant("AlertSeverityEnum", 0u32, "High"),
            Self::Medium => serializer.serialize_unit_variant("AlertSeverityEnum", 1u32, "Medium"),
            Self::Low => serializer.serialize_unit_variant("AlertSeverityEnum", 2u32, "Low"),
            Self::Informational => serializer.serialize_unit_variant("AlertSeverityEnum", 3u32, "Informational"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Alerts data type for data connectors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsDataTypeOfDataConnector {
    #[doc = "Common field for data type in data connectors."]
    pub alerts: DataConnectorDataTypeCommon,
}
impl AlertsDataTypeOfDataConnector {
    pub fn new(alerts: DataConnectorDataTypeCommon) -> Self {
        Self { alerts }
    }
}
#[doc = "Analytics Rule Run Trigger request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalyticsRuleRunTrigger {
    #[doc = "The Analytics Rule Run Trigger properties"]
    pub properties: AnalyticsRuleRunTriggerProperties,
}
impl AnalyticsRuleRunTrigger {
    pub fn new(properties: AnalyticsRuleRunTriggerProperties) -> Self {
        Self { properties }
    }
}
#[doc = "The Analytics Rule Run Trigger properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalyticsRuleRunTriggerProperties {
    #[serde(rename = "executionTimeUtc", with = "azure_core::date::rfc3339")]
    pub execution_time_utc: time::OffsetDateTime,
}
impl AnalyticsRuleRunTriggerProperties {
    pub fn new(execution_time_utc: time::OffsetDateTime) -> Self {
        Self { execution_time_utc }
    }
}
#[doc = "Settings with single toggle."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Anomalies {
    #[serde(flatten)]
    pub settings: Settings,
    #[doc = "Anomalies property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AnomaliesSettingsProperties>,
}
impl Anomalies {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            properties: None,
        }
    }
}
#[doc = "Anomalies property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnomaliesSettingsProperties {
    #[doc = "Determines whether the setting is enable or disabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}
impl AnomaliesSettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents Anomaly Security ML Analytics Settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnomalySecurityMlAnalyticsSettings {
    #[serde(flatten)]
    pub security_ml_analytics_setting: SecurityMlAnalyticsSetting,
    #[doc = "AnomalySecurityMLAnalytics settings base property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AnomalySecurityMlAnalyticsSettingsProperties>,
}
impl AnomalySecurityMlAnalyticsSettings {
    pub fn new(security_ml_analytics_setting: SecurityMlAnalyticsSetting) -> Self {
        Self {
            security_ml_analytics_setting,
            properties: None,
        }
    }
}
#[doc = "AnomalySecurityMLAnalytics settings base property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnomalySecurityMlAnalyticsSettingsProperties {
    #[doc = "The description of the SecurityMLAnalyticsSettings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display name for settings created by this SecurityMLAnalyticsSettings."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Determines whether this settings is enabled or disabled."]
    pub enabled: bool,
    #[doc = "The last time that this SecurityMLAnalyticsSettings has been modified."]
    #[serde(rename = "lastModifiedUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_utc: Option<time::OffsetDateTime>,
    #[doc = "The required data sources for this SecurityMLAnalyticsSettings"]
    #[serde(
        rename = "requiredDataConnectors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_data_connectors: Vec<SecurityMlAnalyticsSettingsDataSource>,
    #[doc = "The tactics of the SecurityMLAnalyticsSettings"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tactics: Vec<AttackTactic>,
    #[doc = "The techniques of the SecurityMLAnalyticsSettings"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub techniques: Vec<String>,
    #[doc = "The anomaly version of the AnomalySecurityMLAnalyticsSettings."]
    #[serde(rename = "anomalyVersion")]
    pub anomaly_version: String,
    #[doc = "The customizable observations of the AnomalySecurityMLAnalyticsSettings."]
    #[serde(rename = "customizableObservations", default, skip_serializing_if = "Option::is_none")]
    pub customizable_observations: Option<serde_json::Value>,
    #[doc = "The frequency that this SecurityMLAnalyticsSettings will be run."]
    pub frequency: String,
    #[doc = "The anomaly SecurityMLAnalyticsSettings status"]
    #[serde(rename = "settingsStatus")]
    pub settings_status: AnomalySecurityMlAnalyticsSettingsStatus,
    #[doc = "Determines whether this anomaly security ml analytics settings is a default settings"]
    #[serde(rename = "isDefaultSettings")]
    pub is_default_settings: bool,
    #[doc = "The anomaly settings version of the Anomaly security ml analytics settings that dictates whether job version gets updated or not."]
    #[serde(rename = "anomalySettingsVersion", default, skip_serializing_if = "Option::is_none")]
    pub anomaly_settings_version: Option<i32>,
    #[doc = "The anomaly settings definition Id"]
    #[serde(rename = "settingsDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub settings_definition_id: Option<String>,
}
impl AnomalySecurityMlAnalyticsSettingsProperties {
    pub fn new(
        display_name: String,
        enabled: bool,
        anomaly_version: String,
        frequency: String,
        settings_status: AnomalySecurityMlAnalyticsSettingsStatus,
        is_default_settings: bool,
    ) -> Self {
        Self {
            description: None,
            display_name,
            enabled,
            last_modified_utc: None,
            required_data_connectors: Vec::new(),
            tactics: Vec::new(),
            techniques: Vec::new(),
            anomaly_version,
            customizable_observations: None,
            frequency,
            settings_status,
            is_default_settings,
            anomaly_settings_version: None,
            settings_definition_id: None,
        }
    }
}
#[doc = "The anomaly SecurityMLAnalyticsSettings status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AnomalySecurityMlAnalyticsSettingsStatus")]
pub enum AnomalySecurityMlAnalyticsSettingsStatus {
    Production,
    Flighting,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AnomalySecurityMlAnalyticsSettingsStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AnomalySecurityMlAnalyticsSettingsStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AnomalySecurityMlAnalyticsSettingsStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Production => serializer.serialize_unit_variant("AnomalySecurityMlAnalyticsSettingsStatus", 0u32, "Production"),
            Self::Flighting => serializer.serialize_unit_variant("AnomalySecurityMlAnalyticsSettingsStatus", 1u32, "Flighting"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents anomaly timeline item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnomalyTimelineItem {
    #[doc = "The anomaly azure resource id."]
    #[serde(rename = "azureResourceId")]
    pub azure_resource_id: String,
    #[doc = "The anomaly product name."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "The anomaly description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The anomaly name."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The anomaly end time."]
    #[serde(rename = "endTimeUtc", with = "azure_core::date::rfc3339")]
    pub end_time_utc: time::OffsetDateTime,
    #[doc = "The anomaly start time."]
    #[serde(rename = "startTimeUtc", with = "azure_core::date::rfc3339")]
    pub start_time_utc: time::OffsetDateTime,
    #[doc = "The anomaly generated time."]
    #[serde(rename = "timeGenerated", with = "azure_core::date::rfc3339")]
    pub time_generated: time::OffsetDateTime,
    #[doc = "The name of the anomaly vendor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[doc = "The intent of the anomaly."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,
    #[doc = "The techniques of the anomaly."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub techniques: Vec<String>,
    #[doc = "The reasons that cause the anomaly."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reasons: Vec<String>,
}
impl AnomalyTimelineItem {
    pub fn new(
        azure_resource_id: String,
        display_name: String,
        end_time_utc: time::OffsetDateTime,
        start_time_utc: time::OffsetDateTime,
        time_generated: time::OffsetDateTime,
    ) -> Self {
        Self {
            azure_resource_id,
            product_name: None,
            description: None,
            display_name,
            end_time_utc,
            start_time_utc,
            time_generated,
            vendor: None,
            intent: None,
            techniques: Vec::new(),
            reasons: Vec::new(),
        }
    }
}
#[doc = "Represents Codeless API Polling data connector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiPollingParameters {
    #[doc = "Config to describe the instructions blade"]
    #[serde(rename = "connectorUiConfig", default, skip_serializing_if = "Option::is_none")]
    pub connector_ui_config: Option<CodelessUiConnectorConfigProperties>,
    #[doc = "Config to describe the polling config for API poller connector"]
    #[serde(rename = "pollingConfig", default, skip_serializing_if = "Option::is_none")]
    pub polling_config: Option<CodelessConnectorPollingConfigProperties>,
}
impl ApiPollingParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The severity for alerts created by this alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AttackTactic")]
pub enum AttackTactic {
    Reconnaissance,
    ResourceDevelopment,
    InitialAccess,
    Execution,
    Persistence,
    PrivilegeEscalation,
    DefenseEvasion,
    CredentialAccess,
    Discovery,
    LateralMovement,
    Collection,
    Exfiltration,
    CommandAndControl,
    Impact,
    PreAttack,
    ImpairProcessControl,
    InhibitResponseFunction,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AttackTactic {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AttackTactic {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AttackTactic {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Reconnaissance => serializer.serialize_unit_variant("AttackTactic", 0u32, "Reconnaissance"),
            Self::ResourceDevelopment => serializer.serialize_unit_variant("AttackTactic", 1u32, "ResourceDevelopment"),
            Self::InitialAccess => serializer.serialize_unit_variant("AttackTactic", 2u32, "InitialAccess"),
            Self::Execution => serializer.serialize_unit_variant("AttackTactic", 3u32, "Execution"),
            Self::Persistence => serializer.serialize_unit_variant("AttackTactic", 4u32, "Persistence"),
            Self::PrivilegeEscalation => serializer.serialize_unit_variant("AttackTactic", 5u32, "PrivilegeEscalation"),
            Self::DefenseEvasion => serializer.serialize_unit_variant("AttackTactic", 6u32, "DefenseEvasion"),
            Self::CredentialAccess => serializer.serialize_unit_variant("AttackTactic", 7u32, "CredentialAccess"),
            Self::Discovery => serializer.serialize_unit_variant("AttackTactic", 8u32, "Discovery"),
            Self::LateralMovement => serializer.serialize_unit_variant("AttackTactic", 9u32, "LateralMovement"),
            Self::Collection => serializer.serialize_unit_variant("AttackTactic", 10u32, "Collection"),
            Self::Exfiltration => serializer.serialize_unit_variant("AttackTactic", 11u32, "Exfiltration"),
            Self::CommandAndControl => serializer.serialize_unit_variant("AttackTactic", 12u32, "CommandAndControl"),
            Self::Impact => serializer.serialize_unit_variant("AttackTactic", 13u32, "Impact"),
            Self::PreAttack => serializer.serialize_unit_variant("AttackTactic", 14u32, "PreAttack"),
            Self::ImpairProcessControl => serializer.serialize_unit_variant("AttackTactic", 15u32, "ImpairProcessControl"),
            Self::InhibitResponseFunction => serializer.serialize_unit_variant("AttackTactic", 16u32, "InhibitResponseFunction"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type AttackTechnique = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationRule {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "Automation rule properties"]
    pub properties: AutomationRuleProperties,
}
impl AutomationRule {
    pub fn new(properties: AutomationRuleProperties) -> Self {
        Self {
            resource_with_etag: ResourceWithEtag::default(),
            properties,
        }
    }
}
#[doc = "Describes an automation rule action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationRuleAction {
    pub order: i32,
}
impl AutomationRuleAction {
    pub fn new(order: i32) -> Self {
        Self { order }
    }
}
#[doc = "The type of the automation rule action."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "actionType")]
pub enum AutomationRuleActionUnion {
    AddIncidentTask(AutomationRuleAddIncidentTaskAction),
    ModifyProperties(AutomationRuleModifyPropertiesAction),
    RunPlaybook(AutomationRuleRunPlaybookAction),
}
#[doc = "Describes an automation rule action to add a task to an incident"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationRuleAddIncidentTaskAction {
    #[serde(flatten)]
    pub automation_rule_action: AutomationRuleAction,
    #[serde(rename = "actionConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub action_configuration: Option<AddIncidentTaskActionProperties>,
}
impl AutomationRuleAddIncidentTaskAction {
    pub fn new(automation_rule_action: AutomationRuleAction) -> Self {
        Self {
            automation_rule_action,
            action_configuration: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationRuleBooleanCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<AutomationRuleBooleanConditionSupportedOperator>,
    #[serde(
        rename = "innerConditions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inner_conditions: Vec<AutomationRuleConditionUnion>,
}
impl AutomationRuleBooleanCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutomationRuleBooleanConditionSupportedOperator")]
pub enum AutomationRuleBooleanConditionSupportedOperator {
    And,
    Or,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutomationRuleBooleanConditionSupportedOperator {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutomationRuleBooleanConditionSupportedOperator {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutomationRuleBooleanConditionSupportedOperator {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::And => serializer.serialize_unit_variant("AutomationRuleBooleanConditionSupportedOperator", 0u32, "And"),
            Self::Or => serializer.serialize_unit_variant("AutomationRuleBooleanConditionSupportedOperator", 1u32, "Or"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "conditionType")]
pub enum AutomationRuleConditionUnion {
    Boolean(BooleanConditionProperties),
    PropertyArrayChanged(PropertyArrayChangedConditionProperties),
    PropertyArray(PropertyArrayConditionProperties),
    PropertyChanged(PropertyChangedConditionProperties),
    Property(PropertyConditionProperties),
}
#[doc = "Describes an automation rule action to modify an object's properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationRuleModifyPropertiesAction {
    #[serde(flatten)]
    pub automation_rule_action: AutomationRuleAction,
    #[serde(rename = "actionConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub action_configuration: Option<IncidentPropertiesAction>,
}
impl AutomationRuleModifyPropertiesAction {
    pub fn new(automation_rule_action: AutomationRuleAction) -> Self {
        Self {
            automation_rule_action,
            action_configuration: None,
        }
    }
}
#[doc = "Automation rule properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationRuleProperties {
    #[doc = "The display name of the automation rule."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The order of execution of the automation rule."]
    pub order: i32,
    #[doc = "Describes automation rule triggering logic."]
    #[serde(rename = "triggeringLogic")]
    pub triggering_logic: AutomationRuleTriggeringLogic,
    #[doc = "The actions to execute when the automation rule is triggered."]
    pub actions: Vec<AutomationRuleActionUnion>,
    #[doc = "The last time the automation rule was updated."]
    #[serde(rename = "lastModifiedTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The time the automation rule was created."]
    #[serde(rename = "createdTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub created_time_utc: Option<time::OffsetDateTime>,
    #[doc = "Information on the client (user or application) that made some action"]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<ClientInfo>,
    #[doc = "Information on the client (user or application) that made some action"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<ClientInfo>,
}
impl AutomationRuleProperties {
    pub fn new(
        display_name: String,
        order: i32,
        triggering_logic: AutomationRuleTriggeringLogic,
        actions: Vec<AutomationRuleActionUnion>,
    ) -> Self {
        Self {
            display_name,
            order,
            triggering_logic,
            actions,
            last_modified_time_utc: None,
            created_time_utc: None,
            last_modified_by: None,
            created_by: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutomationRulePropertyArrayChangedConditionSupportedArrayType")]
pub enum AutomationRulePropertyArrayChangedConditionSupportedArrayType {
    Alerts,
    Labels,
    Tactics,
    Comments,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutomationRulePropertyArrayChangedConditionSupportedArrayType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutomationRulePropertyArrayChangedConditionSupportedArrayType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutomationRulePropertyArrayChangedConditionSupportedArrayType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Alerts => {
                serializer.serialize_unit_variant("AutomationRulePropertyArrayChangedConditionSupportedArrayType", 0u32, "Alerts")
            }
            Self::Labels => {
                serializer.serialize_unit_variant("AutomationRulePropertyArrayChangedConditionSupportedArrayType", 1u32, "Labels")
            }
            Self::Tactics => {
                serializer.serialize_unit_variant("AutomationRulePropertyArrayChangedConditionSupportedArrayType", 2u32, "Tactics")
            }
            Self::Comments => {
                serializer.serialize_unit_variant("AutomationRulePropertyArrayChangedConditionSupportedArrayType", 3u32, "Comments")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutomationRulePropertyArrayChangedConditionSupportedChangeType")]
pub enum AutomationRulePropertyArrayChangedConditionSupportedChangeType {
    Added,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutomationRulePropertyArrayChangedConditionSupportedChangeType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutomationRulePropertyArrayChangedConditionSupportedChangeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutomationRulePropertyArrayChangedConditionSupportedChangeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Added => {
                serializer.serialize_unit_variant("AutomationRulePropertyArrayChangedConditionSupportedChangeType", 0u32, "Added")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationRulePropertyArrayChangedValuesCondition {
    #[serde(rename = "arrayType", default, skip_serializing_if = "Option::is_none")]
    pub array_type: Option<AutomationRulePropertyArrayChangedConditionSupportedArrayType>,
    #[serde(rename = "changeType", default, skip_serializing_if = "Option::is_none")]
    pub change_type: Option<AutomationRulePropertyArrayChangedConditionSupportedChangeType>,
}
impl AutomationRulePropertyArrayChangedValuesCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutomationRulePropertyArrayConditionSupportedArrayConditionType")]
pub enum AutomationRulePropertyArrayConditionSupportedArrayConditionType {
    AnyItem,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutomationRulePropertyArrayConditionSupportedArrayConditionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutomationRulePropertyArrayConditionSupportedArrayConditionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutomationRulePropertyArrayConditionSupportedArrayConditionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AnyItem => {
                serializer.serialize_unit_variant("AutomationRulePropertyArrayConditionSupportedArrayConditionType", 0u32, "AnyItem")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutomationRulePropertyArrayConditionSupportedArrayType")]
pub enum AutomationRulePropertyArrayConditionSupportedArrayType {
    CustomDetails,
    CustomDetailValues,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutomationRulePropertyArrayConditionSupportedArrayType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutomationRulePropertyArrayConditionSupportedArrayType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutomationRulePropertyArrayConditionSupportedArrayType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::CustomDetails => {
                serializer.serialize_unit_variant("AutomationRulePropertyArrayConditionSupportedArrayType", 0u32, "CustomDetails")
            }
            Self::CustomDetailValues => {
                serializer.serialize_unit_variant("AutomationRulePropertyArrayConditionSupportedArrayType", 1u32, "CustomDetailValues")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationRulePropertyArrayValuesCondition {
    #[serde(rename = "arrayType", default, skip_serializing_if = "Option::is_none")]
    pub array_type: Option<AutomationRulePropertyArrayConditionSupportedArrayType>,
    #[serde(rename = "arrayConditionType", default, skip_serializing_if = "Option::is_none")]
    pub array_condition_type: Option<AutomationRulePropertyArrayConditionSupportedArrayConditionType>,
    #[serde(
        rename = "itemConditions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub item_conditions: Vec<AutomationRuleConditionUnion>,
}
impl AutomationRulePropertyArrayValuesCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutomationRulePropertyChangedConditionSupportedChangedType")]
pub enum AutomationRulePropertyChangedConditionSupportedChangedType {
    ChangedFrom,
    ChangedTo,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutomationRulePropertyChangedConditionSupportedChangedType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutomationRulePropertyChangedConditionSupportedChangedType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutomationRulePropertyChangedConditionSupportedChangedType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ChangedFrom => {
                serializer.serialize_unit_variant("AutomationRulePropertyChangedConditionSupportedChangedType", 0u32, "ChangedFrom")
            }
            Self::ChangedTo => {
                serializer.serialize_unit_variant("AutomationRulePropertyChangedConditionSupportedChangedType", 1u32, "ChangedTo")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutomationRulePropertyChangedConditionSupportedPropertyType")]
pub enum AutomationRulePropertyChangedConditionSupportedPropertyType {
    IncidentSeverity,
    IncidentStatus,
    IncidentOwner,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutomationRulePropertyChangedConditionSupportedPropertyType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutomationRulePropertyChangedConditionSupportedPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutomationRulePropertyChangedConditionSupportedPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::IncidentSeverity => serializer.serialize_unit_variant(
                "AutomationRulePropertyChangedConditionSupportedPropertyType",
                0u32,
                "IncidentSeverity",
            ),
            Self::IncidentStatus => serializer.serialize_unit_variant(
                "AutomationRulePropertyChangedConditionSupportedPropertyType",
                1u32,
                "IncidentStatus",
            ),
            Self::IncidentOwner => {
                serializer.serialize_unit_variant("AutomationRulePropertyChangedConditionSupportedPropertyType", 2u32, "IncidentOwner")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutomationRulePropertyConditionSupportedOperator")]
pub enum AutomationRulePropertyConditionSupportedOperator {
    Equals,
    NotEquals,
    Contains,
    NotContains,
    StartsWith,
    NotStartsWith,
    EndsWith,
    NotEndsWith,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutomationRulePropertyConditionSupportedOperator {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutomationRulePropertyConditionSupportedOperator {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutomationRulePropertyConditionSupportedOperator {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Equals => serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedOperator", 0u32, "Equals"),
            Self::NotEquals => serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedOperator", 1u32, "NotEquals"),
            Self::Contains => serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedOperator", 2u32, "Contains"),
            Self::NotContains => serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedOperator", 3u32, "NotContains"),
            Self::StartsWith => serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedOperator", 4u32, "StartsWith"),
            Self::NotStartsWith => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedOperator", 5u32, "NotStartsWith")
            }
            Self::EndsWith => serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedOperator", 6u32, "EndsWith"),
            Self::NotEndsWith => serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedOperator", 7u32, "NotEndsWith"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The property to evaluate in an automation rule property condition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutomationRulePropertyConditionSupportedProperty")]
pub enum AutomationRulePropertyConditionSupportedProperty {
    IncidentTitle,
    IncidentDescription,
    IncidentSeverity,
    IncidentStatus,
    IncidentRelatedAnalyticRuleIds,
    IncidentTactics,
    IncidentLabel,
    IncidentProviderName,
    IncidentUpdatedBySource,
    IncidentCustomDetailsKey,
    IncidentCustomDetailsValue,
    AccountAadTenantId,
    AccountAadUserId,
    AccountName,
    #[serde(rename = "AccountNTDomain")]
    AccountNtDomain,
    #[serde(rename = "AccountPUID")]
    AccountPuid,
    AccountSid,
    AccountObjectGuid,
    #[serde(rename = "AccountUPNSuffix")]
    AccountUpnSuffix,
    AlertProductNames,
    AlertAnalyticRuleIds,
    AzureResourceResourceId,
    AzureResourceSubscriptionId,
    CloudApplicationAppId,
    CloudApplicationAppName,
    #[serde(rename = "DNSDomainName")]
    DnsDomainName,
    FileDirectory,
    FileName,
    FileHashValue,
    #[serde(rename = "HostAzureID")]
    HostAzureId,
    HostName,
    HostNetBiosName,
    #[serde(rename = "HostNTDomain")]
    HostNtDomain,
    #[serde(rename = "HostOSVersion")]
    HostOsVersion,
    IoTDeviceId,
    IoTDeviceName,
    IoTDeviceType,
    IoTDeviceVendor,
    IoTDeviceModel,
    IoTDeviceOperatingSystem,
    #[serde(rename = "IPAddress")]
    IpAddress,
    MailboxDisplayName,
    MailboxPrimaryAddress,
    #[serde(rename = "MailboxUPN")]
    MailboxUpn,
    MailMessageDeliveryAction,
    MailMessageDeliveryLocation,
    MailMessageRecipient,
    #[serde(rename = "MailMessageSenderIP")]
    MailMessageSenderIp,
    MailMessageSubject,
    MailMessageP1Sender,
    MailMessageP2Sender,
    MalwareCategory,
    MalwareName,
    ProcessCommandLine,
    ProcessId,
    RegistryKey,
    RegistryValueData,
    Url,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutomationRulePropertyConditionSupportedProperty {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutomationRulePropertyConditionSupportedProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutomationRulePropertyConditionSupportedProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::IncidentTitle => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 0u32, "IncidentTitle")
            }
            Self::IncidentDescription => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 1u32, "IncidentDescription")
            }
            Self::IncidentSeverity => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 2u32, "IncidentSeverity")
            }
            Self::IncidentStatus => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 3u32, "IncidentStatus")
            }
            Self::IncidentRelatedAnalyticRuleIds => serializer.serialize_unit_variant(
                "AutomationRulePropertyConditionSupportedProperty",
                4u32,
                "IncidentRelatedAnalyticRuleIds",
            ),
            Self::IncidentTactics => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 5u32, "IncidentTactics")
            }
            Self::IncidentLabel => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 6u32, "IncidentLabel")
            }
            Self::IncidentProviderName => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 7u32, "IncidentProviderName")
            }
            Self::IncidentUpdatedBySource => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 8u32, "IncidentUpdatedBySource")
            }
            Self::IncidentCustomDetailsKey => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 9u32, "IncidentCustomDetailsKey")
            }
            Self::IncidentCustomDetailsValue => serializer.serialize_unit_variant(
                "AutomationRulePropertyConditionSupportedProperty",
                10u32,
                "IncidentCustomDetailsValue",
            ),
            Self::AccountAadTenantId => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 11u32, "AccountAadTenantId")
            }
            Self::AccountAadUserId => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 12u32, "AccountAadUserId")
            }
            Self::AccountName => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 13u32, "AccountName")
            }
            Self::AccountNtDomain => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 14u32, "AccountNTDomain")
            }
            Self::AccountPuid => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 15u32, "AccountPUID")
            }
            Self::AccountSid => serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 16u32, "AccountSid"),
            Self::AccountObjectGuid => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 17u32, "AccountObjectGuid")
            }
            Self::AccountUpnSuffix => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 18u32, "AccountUPNSuffix")
            }
            Self::AlertProductNames => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 19u32, "AlertProductNames")
            }
            Self::AlertAnalyticRuleIds => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 20u32, "AlertAnalyticRuleIds")
            }
            Self::AzureResourceResourceId => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 21u32, "AzureResourceResourceId")
            }
            Self::AzureResourceSubscriptionId => serializer.serialize_unit_variant(
                "AutomationRulePropertyConditionSupportedProperty",
                22u32,
                "AzureResourceSubscriptionId",
            ),
            Self::CloudApplicationAppId => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 23u32, "CloudApplicationAppId")
            }
            Self::CloudApplicationAppName => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 24u32, "CloudApplicationAppName")
            }
            Self::DnsDomainName => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 25u32, "DNSDomainName")
            }
            Self::FileDirectory => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 26u32, "FileDirectory")
            }
            Self::FileName => serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 27u32, "FileName"),
            Self::FileHashValue => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 28u32, "FileHashValue")
            }
            Self::HostAzureId => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 29u32, "HostAzureID")
            }
            Self::HostName => serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 30u32, "HostName"),
            Self::HostNetBiosName => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 31u32, "HostNetBiosName")
            }
            Self::HostNtDomain => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 32u32, "HostNTDomain")
            }
            Self::HostOsVersion => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 33u32, "HostOSVersion")
            }
            Self::IoTDeviceId => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 34u32, "IoTDeviceId")
            }
            Self::IoTDeviceName => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 35u32, "IoTDeviceName")
            }
            Self::IoTDeviceType => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 36u32, "IoTDeviceType")
            }
            Self::IoTDeviceVendor => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 37u32, "IoTDeviceVendor")
            }
            Self::IoTDeviceModel => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 38u32, "IoTDeviceModel")
            }
            Self::IoTDeviceOperatingSystem => serializer.serialize_unit_variant(
                "AutomationRulePropertyConditionSupportedProperty",
                39u32,
                "IoTDeviceOperatingSystem",
            ),
            Self::IpAddress => serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 40u32, "IPAddress"),
            Self::MailboxDisplayName => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 41u32, "MailboxDisplayName")
            }
            Self::MailboxPrimaryAddress => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 42u32, "MailboxPrimaryAddress")
            }
            Self::MailboxUpn => serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 43u32, "MailboxUPN"),
            Self::MailMessageDeliveryAction => serializer.serialize_unit_variant(
                "AutomationRulePropertyConditionSupportedProperty",
                44u32,
                "MailMessageDeliveryAction",
            ),
            Self::MailMessageDeliveryLocation => serializer.serialize_unit_variant(
                "AutomationRulePropertyConditionSupportedProperty",
                45u32,
                "MailMessageDeliveryLocation",
            ),
            Self::MailMessageRecipient => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 46u32, "MailMessageRecipient")
            }
            Self::MailMessageSenderIp => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 47u32, "MailMessageSenderIP")
            }
            Self::MailMessageSubject => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 48u32, "MailMessageSubject")
            }
            Self::MailMessageP1Sender => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 49u32, "MailMessageP1Sender")
            }
            Self::MailMessageP2Sender => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 50u32, "MailMessageP2Sender")
            }
            Self::MalwareCategory => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 51u32, "MalwareCategory")
            }
            Self::MalwareName => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 52u32, "MalwareName")
            }
            Self::ProcessCommandLine => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 53u32, "ProcessCommandLine")
            }
            Self::ProcessId => serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 54u32, "ProcessId"),
            Self::RegistryKey => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 55u32, "RegistryKey")
            }
            Self::RegistryValueData => {
                serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 56u32, "RegistryValueData")
            }
            Self::Url => serializer.serialize_unit_variant("AutomationRulePropertyConditionSupportedProperty", 57u32, "Url"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationRulePropertyValuesChangedCondition {
    #[serde(rename = "propertyName", default, skip_serializing_if = "Option::is_none")]
    pub property_name: Option<AutomationRulePropertyChangedConditionSupportedPropertyType>,
    #[serde(rename = "changeType", default, skip_serializing_if = "Option::is_none")]
    pub change_type: Option<AutomationRulePropertyChangedConditionSupportedChangedType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<AutomationRulePropertyConditionSupportedOperator>,
    #[serde(
        rename = "propertyValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub property_values: Vec<String>,
}
impl AutomationRulePropertyValuesChangedCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationRulePropertyValuesCondition {
    #[doc = "The property to evaluate in an automation rule property condition."]
    #[serde(rename = "propertyName", default, skip_serializing_if = "Option::is_none")]
    pub property_name: Option<AutomationRulePropertyConditionSupportedProperty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<AutomationRulePropertyConditionSupportedOperator>,
    #[serde(
        rename = "propertyValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub property_values: Vec<String>,
}
impl AutomationRulePropertyValuesCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an automation rule action to run a playbook"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationRuleRunPlaybookAction {
    #[serde(flatten)]
    pub automation_rule_action: AutomationRuleAction,
    #[serde(rename = "actionConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub action_configuration: Option<PlaybookActionProperties>,
}
impl AutomationRuleRunPlaybookAction {
    pub fn new(automation_rule_action: AutomationRuleAction) -> Self {
        Self {
            automation_rule_action,
            action_configuration: None,
        }
    }
}
#[doc = "Describes automation rule triggering logic."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationRuleTriggeringLogic {
    #[doc = "Determines whether the automation rule is enabled or disabled."]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[doc = "Determines when the automation rule should automatically expire and be disabled."]
    #[serde(rename = "expirationTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_time_utc: Option<time::OffsetDateTime>,
    #[serde(rename = "triggersOn")]
    pub triggers_on: TriggersOn,
    #[serde(rename = "triggersWhen")]
    pub triggers_when: TriggersWhen,
    #[doc = "The conditions to evaluate to determine if the automation rule should be triggered on a given object."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conditions: Vec<AutomationRuleConditionUnion>,
}
impl AutomationRuleTriggeringLogic {
    pub fn new(is_enabled: bool, triggers_on: TriggersOn, triggers_when: TriggersWhen) -> Self {
        Self {
            is_enabled,
            expiration_time_utc: None,
            triggers_on,
            triggers_when,
            conditions: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationRulesList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AutomationRule>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AutomationRulesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AutomationRulesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Connector Availability Status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Availability {
    #[doc = "The connector Availability Status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<availability::Status>,
    #[doc = "Set connector as preview"]
    #[serde(rename = "isPreview", default, skip_serializing_if = "Option::is_none")]
    pub is_preview: Option<bool>,
}
impl Availability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod availability {
    use super::*;
    #[doc = "The connector Availability Status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {}
}
#[doc = "Amazon Web Services CloudTrail requirements check request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudTrailCheckRequirements {}
impl AwsCloudTrailCheckRequirements {
    pub fn new() -> Self {
        Self {}
    }
}
#[doc = "Represents Amazon Web Services CloudTrail data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudTrailDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "Amazon Web Services CloudTrail data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AwsCloudTrailDataConnectorProperties>,
}
impl AwsCloudTrailDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "The available data types for Amazon Web Services CloudTrail data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudTrailDataConnectorDataTypes {
    #[doc = "Logs data type."]
    pub logs: serde_json::Value,
}
impl AwsCloudTrailDataConnectorDataTypes {
    pub fn new(logs: serde_json::Value) -> Self {
        Self { logs }
    }
}
#[doc = "Amazon Web Services CloudTrail data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudTrailDataConnectorProperties {
    #[doc = "The Aws Role Arn (with CloudTrailReadOnly policy) that is used to access the Aws account."]
    #[serde(rename = "awsRoleArn", default, skip_serializing_if = "Option::is_none")]
    pub aws_role_arn: Option<String>,
    #[doc = "The available data types for Amazon Web Services CloudTrail data connector."]
    #[serde(rename = "dataTypes")]
    pub data_types: AwsCloudTrailDataConnectorDataTypes,
}
impl AwsCloudTrailDataConnectorProperties {
    pub fn new(data_types: AwsCloudTrailDataConnectorDataTypes) -> Self {
        Self {
            aws_role_arn: None,
            data_types,
        }
    }
}
#[doc = "Amazon Web Services S3 requirements check request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsS3CheckRequirements {}
impl AwsS3CheckRequirements {
    pub fn new() -> Self {
        Self {}
    }
}
#[doc = "Represents Amazon Web Services S3 data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsS3DataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "Amazon Web Services S3 data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AwsS3DataConnectorProperties>,
}
impl AwsS3DataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "The available data types for Amazon Web Services S3 data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsS3DataConnectorDataTypes {
    #[doc = "Logs data type."]
    pub logs: serde_json::Value,
}
impl AwsS3DataConnectorDataTypes {
    pub fn new(logs: serde_json::Value) -> Self {
        Self { logs }
    }
}
#[doc = "Amazon Web Services S3 data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsS3DataConnectorProperties {
    #[doc = "The logs destination table name in LogAnalytics."]
    #[serde(rename = "destinationTable")]
    pub destination_table: String,
    #[doc = "The AWS sqs urls for the connector."]
    #[serde(rename = "sqsUrls")]
    pub sqs_urls: Vec<String>,
    #[doc = "The Aws Role Arn that is used to access the Aws account."]
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    #[doc = "The available data types for Amazon Web Services S3 data connector."]
    #[serde(rename = "dataTypes")]
    pub data_types: AwsS3DataConnectorDataTypes,
}
impl AwsS3DataConnectorProperties {
    pub fn new(destination_table: String, sqs_urls: Vec<String>, role_arn: String, data_types: AwsS3DataConnectorDataTypes) -> Self {
        Self {
            destination_table,
            sqs_urls,
            role_arn,
            data_types,
        }
    }
}
#[doc = "Resources created in Azure DevOps repository."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsResourceInfo {
    #[doc = "Id of the pipeline created for the source-control."]
    #[serde(rename = "pipelineId", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
    #[doc = "Id of the service-connection created for the source-control."]
    #[serde(rename = "serviceConnectionId", default, skip_serializing_if = "Option::is_none")]
    pub service_connection_id: Option<String>,
}
impl AzureDevOpsResourceInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for an Azure Resource Manager resource with an etag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureEntityResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource Etag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl AzureEntityResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Billing statistic"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingStatistic {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
}
impl BillingStatistic {
    pub fn new() -> Self {
        Self {
            azure_entity_resource: AzureEntityResource::default(),
        }
    }
}
#[doc = "The kind of the billing statistic"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum BillingStatisticUnion {
    SapSolutionUsage(SapSolutionUsageStatistic),
}
#[doc = "The kind of the billing statistic"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BillingStatisticKindEnum")]
pub enum BillingStatisticKindEnum {
    SapSolutionUsage,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BillingStatisticKindEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BillingStatisticKindEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BillingStatisticKindEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SapSolutionUsage => serializer.serialize_unit_variant("BillingStatisticKindEnum", 0u32, "SapSolutionUsage"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "List of all Microsoft Sentinel billing statistics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingStatisticList {
    #[doc = "URL to fetch the next set of billing statistics."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of billing statistics."]
    pub value: Vec<BillingStatisticUnion>,
}
impl azure_core::Continuable for BillingStatisticList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BillingStatisticList {
    pub fn new(value: Vec<BillingStatisticUnion>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Represents a bookmark in Azure Security Insights."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Bookmark {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "Describes bookmark properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BookmarkProperties>,
}
impl Bookmark {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the entity mappings of a single entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BookmarkEntityMappings {
    #[doc = "The entity type"]
    #[serde(rename = "entityType", default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[doc = "Array of fields mapping for that entity type"]
    #[serde(
        rename = "fieldMappings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub field_mappings: Vec<EntityFieldMapping>,
}
impl BookmarkEntityMappings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters required to execute an expand operation on the given bookmark."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BookmarkExpandParameters {
    #[doc = "The end date filter, so the only expansion results returned are before this date."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The Id of the expansion to perform."]
    #[serde(rename = "expansionId", default, skip_serializing_if = "Option::is_none")]
    pub expansion_id: Option<String>,
    #[doc = "The start date filter, so the only expansion results returned are after this date."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
}
impl BookmarkExpandParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The entity expansion result operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BookmarkExpandResponse {
    #[doc = "Expansion result metadata."]
    #[serde(rename = "metaData", default, skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<ExpansionResultsMetadata>,
    #[doc = "The expansion result values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<bookmark_expand_response::Value>,
}
impl BookmarkExpandResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod bookmark_expand_response {
    use super::*;
    #[doc = "The expansion result values."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Value {
        #[doc = "Array of the expansion result entities."]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub entities: Vec<EntityUnion>,
        #[doc = "Array of expansion result connected entities"]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub edges: Vec<ConnectedEntity>,
    }
    impl Value {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "List all the bookmarks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BookmarkList {
    #[doc = "URL to fetch the next set of bookmarks."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of bookmarks."]
    pub value: Vec<Bookmark>,
}
impl azure_core::Continuable for BookmarkList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BookmarkList {
    pub fn new(value: Vec<Bookmark>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Describes bookmark properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BookmarkProperties {
    #[doc = "The time the bookmark was created"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "User information that made some action"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserInfo>,
    #[doc = "The display name of the bookmark"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "List of labels relevant to this bookmark"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub labels: Vec<Label>,
    #[doc = "The notes of the bookmark"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[doc = "The query of the bookmark."]
    pub query: String,
    #[doc = "The query result of the bookmark."]
    #[serde(rename = "queryResult", default, skip_serializing_if = "Option::is_none")]
    pub query_result: Option<String>,
    #[doc = "The last time the bookmark was updated"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub updated: Option<time::OffsetDateTime>,
    #[doc = "User information that made some action"]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<UserInfo>,
    #[doc = "The bookmark event time"]
    #[serde(rename = "eventTime", default, with = "azure_core::date::rfc3339::option")]
    pub event_time: Option<time::OffsetDateTime>,
    #[doc = "The start time for the query"]
    #[serde(rename = "queryStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub query_start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time for the query"]
    #[serde(rename = "queryEndTime", default, with = "azure_core::date::rfc3339::option")]
    pub query_end_time: Option<time::OffsetDateTime>,
    #[doc = "Describes related incident information for the bookmark"]
    #[serde(rename = "incidentInfo", default, skip_serializing_if = "Option::is_none")]
    pub incident_info: Option<IncidentInfo>,
    #[doc = "Describes the entity mappings of the bookmark"]
    #[serde(rename = "entityMappings", default, skip_serializing_if = "Option::is_none")]
    pub entity_mappings: Option<EntityMappingsList>,
    #[doc = "A list of relevant mitre attacks"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tactics: Vec<AttackTactic>,
    #[doc = "A list of relevant mitre techniques"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub techniques: Vec<AttackTechnique>,
}
impl BookmarkProperties {
    pub fn new(display_name: String, query: String) -> Self {
        Self {
            created: None,
            created_by: None,
            display_name,
            labels: Vec::new(),
            notes: None,
            query,
            query_result: None,
            updated: None,
            updated_by: None,
            event_time: None,
            query_start_time: None,
            query_end_time: None,
            incident_info: None,
            entity_mappings: None,
            tactics: Vec::new(),
            techniques: Vec::new(),
        }
    }
}
#[doc = "Represents bookmark timeline item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BookmarkTimelineItem {
    #[doc = "The bookmark azure resource id."]
    #[serde(rename = "azureResourceId")]
    pub azure_resource_id: String,
    #[doc = "The bookmark display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The notes of the bookmark"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[doc = "The bookmark end time."]
    #[serde(rename = "endTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub end_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The bookmark start time."]
    #[serde(rename = "startTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub start_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The bookmark event time."]
    #[serde(rename = "eventTime", default, with = "azure_core::date::rfc3339::option")]
    pub event_time: Option<time::OffsetDateTime>,
    #[doc = "User information that made some action"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserInfo>,
    #[doc = "List of labels relevant to this bookmark"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub labels: Vec<Label>,
}
impl BookmarkTimelineItem {
    pub fn new(azure_resource_id: String) -> Self {
        Self {
            azure_resource_id,
            display_name: None,
            notes: None,
            end_time_utc: None,
            start_time_utc: None,
            event_time: None,
            created_by: None,
            labels: Vec::new(),
        }
    }
}
#[doc = "Describes an automation rule condition that applies a boolean operator (e.g AND, OR) to conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BooleanConditionProperties {
    #[serde(rename = "conditionProperties", default, skip_serializing_if = "Option::is_none")]
    pub condition_properties: Option<AutomationRuleBooleanCondition>,
}
impl BooleanConditionProperties {
    pub fn new() -> Self {
        Self {
            condition_properties: None,
        }
    }
}
#[doc = "Information on the client (user or application) that made some action"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientInfo {
    #[doc = "The email of the client."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The name of the client."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object id of the client."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "The user principal name of the client."]
    #[serde(rename = "userPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}
impl ClientInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response structure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "Error details."]
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
#[doc = "Error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents Codeless API Polling data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodelessApiPollingDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "Represents Codeless API Polling data connector"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiPollingParameters>,
}
impl CodelessApiPollingDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "Describe the authentication properties needed to successfully authenticate with the server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodelessConnectorPollingAuthProperties {
    #[doc = "The authentication type"]
    #[serde(rename = "authType")]
    pub auth_type: String,
    #[doc = "The header name which the token is sent with"]
    #[serde(rename = "apiKeyName", default, skip_serializing_if = "Option::is_none")]
    pub api_key_name: Option<String>,
    #[doc = "A prefix send in the header before the actual token"]
    #[serde(rename = "apiKeyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub api_key_identifier: Option<String>,
    #[doc = "Marks if the key should sent in header"]
    #[serde(rename = "isApiKeyInPostPayload", default, skip_serializing_if = "Option::is_none")]
    pub is_api_key_in_post_payload: Option<String>,
    #[doc = "Describes the flow name, for example 'AuthCode' for Oauth 2.0"]
    #[serde(rename = "flowName", default, skip_serializing_if = "Option::is_none")]
    pub flow_name: Option<String>,
    #[doc = "The endpoint used to issue a token, used in Oauth 2.0 flow"]
    #[serde(rename = "tokenEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub token_endpoint: Option<String>,
    #[doc = "The endpoint used to authorize the user, used in Oauth 2.0 flow"]
    #[serde(rename = "authorizationEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub authorization_endpoint: Option<String>,
    #[doc = "The query parameters used in authorization request, used in Oauth 2.0 flow"]
    #[serde(rename = "authorizationEndpointQueryParameters", default, skip_serializing_if = "Option::is_none")]
    pub authorization_endpoint_query_parameters: Option<serde_json::Value>,
    #[doc = "The redirect endpoint where we will get the authorization code, used in Oauth 2.0 flow"]
    #[serde(rename = "redirectionEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub redirection_endpoint: Option<String>,
    #[doc = "The query headers used in token request, used in Oauth 2.0 flow"]
    #[serde(rename = "tokenEndpointHeaders", default, skip_serializing_if = "Option::is_none")]
    pub token_endpoint_headers: Option<serde_json::Value>,
    #[doc = "The query parameters used in token request, used in Oauth 2.0 flow"]
    #[serde(rename = "tokenEndpointQueryParameters", default, skip_serializing_if = "Option::is_none")]
    pub token_endpoint_query_parameters: Option<serde_json::Value>,
    #[doc = "Marks if we should send the client secret in header or payload, used in Oauth 2.0 flow"]
    #[serde(rename = "isClientSecretInHeader", default, skip_serializing_if = "Option::is_none")]
    pub is_client_secret_in_header: Option<bool>,
    #[doc = "The OAuth token scope"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
impl CodelessConnectorPollingAuthProperties {
    pub fn new(auth_type: String) -> Self {
        Self {
            auth_type,
            api_key_name: None,
            api_key_identifier: None,
            is_api_key_in_post_payload: None,
            flow_name: None,
            token_endpoint: None,
            authorization_endpoint: None,
            authorization_endpoint_query_parameters: None,
            redirection_endpoint: None,
            token_endpoint_headers: None,
            token_endpoint_query_parameters: None,
            is_client_secret_in_header: None,
            scope: None,
        }
    }
}
#[doc = "Config to describe the polling config for API poller connector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodelessConnectorPollingConfigProperties {
    #[doc = "The poller active status"]
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "Describe the authentication properties needed to successfully authenticate with the server"]
    pub auth: CodelessConnectorPollingAuthProperties,
    #[doc = "Describe the request properties needed to successfully pull from the server"]
    pub request: CodelessConnectorPollingRequestProperties,
    #[doc = "Describe the properties needed to make a pagination call"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paging: Option<CodelessConnectorPollingPagingProperties>,
    #[doc = "Describes the response from the external server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<CodelessConnectorPollingResponseProperties>,
}
impl CodelessConnectorPollingConfigProperties {
    pub fn new(auth: CodelessConnectorPollingAuthProperties, request: CodelessConnectorPollingRequestProperties) -> Self {
        Self {
            is_active: None,
            auth,
            request,
            paging: None,
            response: None,
        }
    }
}
#[doc = "Describe the properties needed to make a pagination call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodelessConnectorPollingPagingProperties {
    #[doc = "Describes the type. could be 'None', 'PageToken', 'PageCount', 'TimeStamp'"]
    #[serde(rename = "pagingType")]
    pub paging_type: String,
    #[doc = "Defines the name of a next page attribute"]
    #[serde(rename = "nextPageParaName", default, skip_serializing_if = "Option::is_none")]
    pub next_page_para_name: Option<String>,
    #[doc = "Defines the path to a next page token JSON"]
    #[serde(rename = "nextPageTokenJsonPath", default, skip_serializing_if = "Option::is_none")]
    pub next_page_token_json_path: Option<String>,
    #[doc = "Defines the path to a page count attribute"]
    #[serde(rename = "pageCountAttributePath", default, skip_serializing_if = "Option::is_none")]
    pub page_count_attribute_path: Option<String>,
    #[doc = "Defines the path to a page total count attribute"]
    #[serde(rename = "pageTotalCountAttributePath", default, skip_serializing_if = "Option::is_none")]
    pub page_total_count_attribute_path: Option<String>,
    #[doc = "Defines the path to a paging time stamp attribute"]
    #[serde(rename = "pageTimeStampAttributePath", default, skip_serializing_if = "Option::is_none")]
    pub page_time_stamp_attribute_path: Option<String>,
    #[doc = "Determines whether to search for the latest time stamp in the events list"]
    #[serde(
        rename = "searchTheLatestTimeStampFromEventsList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_the_latest_time_stamp_from_events_list: Option<String>,
    #[doc = "Defines the name of the page size parameter"]
    #[serde(rename = "pageSizeParaName", default, skip_serializing_if = "Option::is_none")]
    pub page_size_para_name: Option<String>,
    #[doc = "Defines the paging size"]
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}
impl CodelessConnectorPollingPagingProperties {
    pub fn new(paging_type: String) -> Self {
        Self {
            paging_type,
            next_page_para_name: None,
            next_page_token_json_path: None,
            page_count_attribute_path: None,
            page_total_count_attribute_path: None,
            page_time_stamp_attribute_path: None,
            search_the_latest_time_stamp_from_events_list: None,
            page_size_para_name: None,
            page_size: None,
        }
    }
}
#[doc = "Describe the request properties needed to successfully pull from the server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodelessConnectorPollingRequestProperties {
    #[doc = "Describe the endpoint we should pull the data from"]
    #[serde(rename = "apiEndpoint")]
    pub api_endpoint: String,
    #[doc = "Defines the rate limit QPS"]
    #[serde(rename = "rateLimitQps", default, skip_serializing_if = "Option::is_none")]
    pub rate_limit_qps: Option<i32>,
    #[doc = "The window interval we will use the pull the data"]
    #[serde(rename = "queryWindowInMin")]
    pub query_window_in_min: i32,
    #[doc = "The http method type we will use in the poll request, GET or POST"]
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    #[doc = "The time format will be used the query events in a specific window"]
    #[serde(rename = "queryTimeFormat")]
    pub query_time_format: String,
    #[doc = "Describe the amount of time we should try and poll the data in case of failure"]
    #[serde(rename = "retryCount", default, skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<i32>,
    #[doc = "The number of seconds we will consider as a request timeout"]
    #[serde(rename = "timeoutInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i32>,
    #[doc = "Describe the headers sent in the poll request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    #[doc = "Describe the query parameters sent in the poll request"]
    #[serde(rename = "queryParameters", default, skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<serde_json::Value>,
    #[doc = "For advanced scenarios for example user name/password embedded in nested JSON payload"]
    #[serde(rename = "queryParametersTemplate", default, skip_serializing_if = "Option::is_none")]
    pub query_parameters_template: Option<String>,
    #[doc = "This will be used the query events from a start of the time window"]
    #[serde(rename = "startTimeAttributeName", default, skip_serializing_if = "Option::is_none")]
    pub start_time_attribute_name: Option<String>,
    #[doc = "This will be used the query events from the end of the time window"]
    #[serde(rename = "endTimeAttributeName", default, skip_serializing_if = "Option::is_none")]
    pub end_time_attribute_name: Option<String>,
}
impl CodelessConnectorPollingRequestProperties {
    pub fn new(api_endpoint: String, query_window_in_min: i32, http_method: String, query_time_format: String) -> Self {
        Self {
            api_endpoint,
            rate_limit_qps: None,
            query_window_in_min,
            http_method,
            query_time_format,
            retry_count: None,
            timeout_in_seconds: None,
            headers: None,
            query_parameters: None,
            query_parameters_template: None,
            start_time_attribute_name: None,
            end_time_attribute_name: None,
        }
    }
}
#[doc = "Describes the response from the external server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodelessConnectorPollingResponseProperties {
    #[doc = "Describes the path we should extract the data in the response"]
    #[serde(rename = "eventsJsonPaths")]
    pub events_json_paths: Vec<String>,
    #[doc = "Describes the path we should extract the status code in the response"]
    #[serde(rename = "successStatusJsonPath", default, skip_serializing_if = "Option::is_none")]
    pub success_status_json_path: Option<String>,
    #[doc = "Describes the path we should extract the status value in the response"]
    #[serde(rename = "successStatusValue", default, skip_serializing_if = "Option::is_none")]
    pub success_status_value: Option<String>,
    #[doc = "Describes if the data in the response is Gzip"]
    #[serde(rename = "isGzipCompressed", default, skip_serializing_if = "Option::is_none")]
    pub is_gzip_compressed: Option<bool>,
}
impl CodelessConnectorPollingResponseProperties {
    pub fn new(events_json_paths: Vec<String>) -> Self {
        Self {
            events_json_paths,
            success_status_json_path: None,
            success_status_value: None,
            is_gzip_compressed: None,
        }
    }
}
#[doc = "Represents Codeless UI data connector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CodelessParameters {
    #[doc = "Config to describe the instructions blade"]
    #[serde(rename = "connectorUiConfig", default, skip_serializing_if = "Option::is_none")]
    pub connector_ui_config: Option<CodelessUiConnectorConfigProperties>,
}
impl CodelessParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Config to describe the instructions blade"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodelessUiConnectorConfigProperties {
    #[doc = "Connector blade title"]
    pub title: String,
    #[doc = "Connector publisher name"]
    pub publisher: String,
    #[doc = "Connector description"]
    #[serde(rename = "descriptionMarkdown")]
    pub description_markdown: String,
    #[doc = "An optional custom image to be used when displaying the connector within Azure Sentinel's connector's gallery"]
    #[serde(rename = "customImage", default, skip_serializing_if = "Option::is_none")]
    pub custom_image: Option<String>,
    #[doc = "Name of the table the connector will insert the data to"]
    #[serde(rename = "graphQueriesTableName")]
    pub graph_queries_table_name: String,
    #[doc = "The graph query to show the current data status"]
    #[serde(rename = "graphQueries")]
    pub graph_queries: Vec<serde_json::Value>,
    #[doc = "The sample queries for the connector"]
    #[serde(rename = "sampleQueries")]
    pub sample_queries: Vec<serde_json::Value>,
    #[doc = "Data types to check for last data received"]
    #[serde(rename = "dataTypes")]
    pub data_types: Vec<serde_json::Value>,
    #[doc = "Define the way the connector check connectivity"]
    #[serde(rename = "connectivityCriteria")]
    pub connectivity_criteria: Vec<serde_json::Value>,
    #[doc = "Connector Availability Status"]
    pub availability: Availability,
    #[doc = "Permissions required for the connector"]
    pub permissions: Permissions,
    #[doc = "Instruction steps to enable the connector"]
    #[serde(rename = "instructionSteps")]
    pub instruction_steps: Vec<serde_json::Value>,
}
impl CodelessUiConnectorConfigProperties {
    pub fn new(
        title: String,
        publisher: String,
        description_markdown: String,
        graph_queries_table_name: String,
        graph_queries: Vec<serde_json::Value>,
        sample_queries: Vec<serde_json::Value>,
        data_types: Vec<serde_json::Value>,
        connectivity_criteria: Vec<serde_json::Value>,
        availability: Availability,
        permissions: Permissions,
        instruction_steps: Vec<serde_json::Value>,
    ) -> Self {
        Self {
            title,
            publisher,
            description_markdown,
            custom_image: None,
            graph_queries_table_name,
            graph_queries,
            sample_queries,
            data_types,
            connectivity_criteria,
            availability,
            permissions,
            instruction_steps,
        }
    }
}
#[doc = "Represents Codeless UI data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodelessUiDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "Represents Codeless UI data connector"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CodelessParameters>,
}
impl CodelessUiDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ConditionType")]
pub enum ConditionType {
    Property,
    PropertyArray,
    PropertyChanged,
    PropertyArrayChanged,
    Boolean,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ConditionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ConditionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ConditionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Property => serializer.serialize_unit_variant("ConditionType", 0u32, "Property"),
            Self::PropertyArray => serializer.serialize_unit_variant("ConditionType", 1u32, "PropertyArray"),
            Self::PropertyChanged => serializer.serialize_unit_variant("ConditionType", 2u32, "PropertyChanged"),
            Self::PropertyArrayChanged => serializer.serialize_unit_variant("ConditionType", 3u32, "PropertyArrayChanged"),
            Self::Boolean => serializer.serialize_unit_variant("ConditionType", 4u32, "Boolean"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Expansion result connected entities"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectedEntity {
    #[doc = "Entity Id of the connected entity"]
    #[serde(rename = "targetEntityId", default, skip_serializing_if = "Option::is_none")]
    pub target_entity_id: Option<String>,
    #[doc = "key-value pairs for a connected entity mapping"]
    #[serde(rename = "additionalData", default, skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<serde_json::Value>,
}
impl ConnectedEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Setting for the connector check connectivity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectivityCriteria {
    #[doc = "type of connectivity"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<connectivity_criteria::Type>,
    #[doc = "Queries for checking connectivity"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<String>,
}
impl ConnectivityCriteria {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod connectivity_criteria {
    use super::*;
    #[doc = "type of connectivity"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        IsConnectedQuery,
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
                Self::IsConnectedQuery => serializer.serialize_unit_variant("Type", 0u32, "IsConnectedQuery"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Instruction step details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectorInstructionModelBase {
    #[doc = "The parameters for the setting"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "The kind of the setting"]
    #[serde(rename = "type")]
    pub type_: connector_instruction_model_base::Type,
}
impl ConnectorInstructionModelBase {
    pub fn new(type_: connector_instruction_model_base::Type) -> Self {
        Self { parameters: None, type_ }
    }
}
pub mod connector_instruction_model_base {
    use super::*;
    #[doc = "The kind of the setting"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        CopyableLabel,
        InstructionStepsGroup,
        InfoMessage,
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
                Self::CopyableLabel => serializer.serialize_unit_variant("Type", 0u32, "CopyableLabel"),
                Self::InstructionStepsGroup => serializer.serialize_unit_variant("Type", 1u32, "InstructionStepsGroup"),
                Self::InfoMessage => serializer.serialize_unit_variant("Type", 2u32, "InfoMessage"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Content section of the recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Content {
    #[doc = "Title of the content."]
    pub title: String,
    #[doc = "Description of the content."]
    pub description: String,
}
impl Content {
    pub fn new(title: String, description: String) -> Self {
        Self { title, description }
    }
}
#[doc = "The content type of a source control path."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ContentType")]
pub enum ContentType {
    AnalyticRule,
    AutomationRule,
    HuntingQuery,
    Parser,
    Playbook,
    Workbook,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ContentType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ContentType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ContentType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AnalyticRule => serializer.serialize_unit_variant("ContentType", 0u32, "AnalyticRule"),
            Self::AutomationRule => serializer.serialize_unit_variant("ContentType", 1u32, "AutomationRule"),
            Self::HuntingQuery => serializer.serialize_unit_variant("ContentType", 2u32, "HuntingQuery"),
            Self::Parser => serializer.serialize_unit_variant("ContentType", 3u32, "Parser"),
            Self::Playbook => serializer.serialize_unit_variant("ContentType", 4u32, "Playbook"),
            Self::Workbook => serializer.serialize_unit_variant("ContentType", 5u32, "Workbook"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specific entity query that supports put requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomEntityQuery {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
}
impl CustomEntityQuery {
    pub fn new() -> Self {
        Self {
            resource_with_etag: ResourceWithEtag::default(),
        }
    }
}
#[doc = "The kind of the entity query that supports put request."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum CustomEntityQueryUnion {
    Activity(ActivityCustomEntityQuery),
}
#[doc = "The kind of the entity query that supports put request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CustomEntityQueryKind")]
pub enum CustomEntityQueryKind {
    Activity,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CustomEntityQueryKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CustomEntityQueryKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CustomEntityQueryKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Activity => serializer.serialize_unit_variant("CustomEntityQueryKind", 0u32, "Activity"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Customs permissions required for the connector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Customs {
    #[serde(flatten)]
    pub customs_permission: CustomsPermission,
}
impl Customs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Customs permissions required for the connector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomsPermission {
    #[doc = "Customs permissions name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Customs permissions description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl CustomsPermission {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration of the destination of the data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DcrConfiguration {
    #[doc = "Represents the data collection ingestion endpoint in log analytics."]
    #[serde(rename = "dataCollectionEndpoint")]
    pub data_collection_endpoint: String,
    #[doc = "The data collection rule immutable id, the rule defines the transformation and data destination."]
    #[serde(rename = "dataCollectionRuleImmutableId")]
    pub data_collection_rule_immutable_id: String,
    #[doc = "The stream we are sending the data to."]
    #[serde(rename = "streamName")]
    pub stream_name: String,
}
impl DcrConfiguration {
    pub fn new(data_collection_endpoint: String, data_collection_rule_immutable_id: String, stream_name: String) -> Self {
        Self {
            data_collection_endpoint,
            data_collection_rule_immutable_id,
            stream_name,
        }
    }
}
#[doc = "Data connector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnector {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
}
impl DataConnector {
    pub fn new() -> Self {
        Self {
            resource_with_etag: ResourceWithEtag::default(),
        }
    }
}
#[doc = "The kind of the data connector"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum DataConnectorUnion {
    AzureActiveDirectory(AadDataConnector),
    AzureAdvancedThreatProtection(AatpDataConnector),
    AzureSecurityCenter(AscDataConnector),
    AmazonWebServicesCloudTrail(AwsCloudTrailDataConnector),
    AmazonWebServicesS3(AwsS3DataConnector),
    #[serde(rename = "APIPolling")]
    ApiPolling(CodelessApiPollingDataConnector),
    #[serde(rename = "GenericUI")]
    GenericUi(CodelessUiDataConnector),
    Dynamics365(Dynamics365DataConnector),
    #[serde(rename = "GCP")]
    Gcp(GcpDataConnector),
    #[serde(rename = "IOT")]
    Iot(IoTDataConnector),
    MicrosoftCloudAppSecurity(McasDataConnector),
    MicrosoftDefenderAdvancedThreatProtection(MdatpDataConnector),
    MicrosoftThreatIntelligence(MstiDataConnector),
    MicrosoftThreatProtection(MtpDataConnector),
    MicrosoftPurviewInformationProtection(MicrosoftPurviewInformationProtectionDataConnector),
    Office365Project(Office365ProjectDataConnector),
    #[serde(rename = "OfficeATP")]
    OfficeAtp(OfficeAtpDataConnector),
    Office365(OfficeDataConnector),
    #[serde(rename = "OfficeIRM")]
    OfficeIrm(OfficeIrmDataConnector),
    #[serde(rename = "OfficePowerBI")]
    OfficePowerBi(OfficePowerBiDataConnector),
    ThreatIntelligence(TiDataConnector),
    ThreatIntelligenceTaxii(TiTaxiiDataConnector),
}
#[doc = "Describes the state of user's authorization for a connector kind."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataConnectorAuthorizationState")]
pub enum DataConnectorAuthorizationState {
    Valid,
    Invalid,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DataConnectorAuthorizationState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DataConnectorAuthorizationState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DataConnectorAuthorizationState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Valid => serializer.serialize_unit_variant("DataConnectorAuthorizationState", 0u32, "Valid"),
            Self::Invalid => serializer.serialize_unit_variant("DataConnectorAuthorizationState", 1u32, "Invalid"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents Codeless API Polling data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataConnectorConnectBody {
    #[doc = "The authentication kind used to poll the data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<data_connector_connect_body::Kind>,
    #[doc = "The API key of the audit server."]
    #[serde(rename = "apiKey", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[doc = "Used in v2 logs connector. Represents the data collection ingestion endpoint in log analytics."]
    #[serde(rename = "dataCollectionEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub data_collection_endpoint: Option<String>,
    #[doc = "Used in v2 logs connector. The data collection rule immutable id, the rule defines the transformation and data destination."]
    #[serde(rename = "dataCollectionRuleImmutableId", default, skip_serializing_if = "Option::is_none")]
    pub data_collection_rule_immutable_id: Option<String>,
    #[doc = "Used in v2 logs connector. The stream we are sending the data to, this is the name of the streamDeclarations defined in the DCR."]
    #[serde(rename = "outputStream", default, skip_serializing_if = "Option::is_none")]
    pub output_stream: Option<String>,
    #[doc = "The client secret of the OAuth 2.0 application."]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[doc = "The client id of the OAuth 2.0 application."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The authorization code used in OAuth 2.0 code flow to issue a token."]
    #[serde(rename = "authorizationCode", default, skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,
    #[doc = "The user name in the audit log server."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "The user password in the audit log server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = ""]
    #[serde(
        rename = "requestConfigUserInputValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub request_config_user_input_values: Vec<serde_json::Value>,
}
impl DataConnectorConnectBody {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_connector_connect_body {
    use super::*;
    #[doc = "The authentication kind used to poll the data"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Basic,
        OAuth2,
        #[serde(rename = "APIKey")]
        ApiKey,
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
                Self::Basic => serializer.serialize_unit_variant("Kind", 0u32, "Basic"),
                Self::OAuth2 => serializer.serialize_unit_variant("Kind", 1u32, "OAuth2"),
                Self::ApiKey => serializer.serialize_unit_variant("Kind", 2u32, "APIKey"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Common field for data type in data connectors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectorDataTypeCommon {
    #[doc = "Describe whether this data type connection is enabled or not."]
    pub state: data_connector_data_type_common::State,
}
impl DataConnectorDataTypeCommon {
    pub fn new(state: data_connector_data_type_common::State) -> Self {
        Self { state }
    }
}
pub mod data_connector_data_type_common {
    use super::*;
    #[doc = "Describe whether this data type connection is enabled or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("State", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("State", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The kind of the data connector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataConnectorKind")]
pub enum DataConnectorKind {
    AzureActiveDirectory,
    AzureSecurityCenter,
    MicrosoftCloudAppSecurity,
    ThreatIntelligence,
    ThreatIntelligenceTaxii,
    Office365,
    #[serde(rename = "OfficeATP")]
    OfficeAtp,
    #[serde(rename = "OfficeIRM")]
    OfficeIrm,
    Office365Project,
    MicrosoftPurviewInformationProtection,
    #[serde(rename = "OfficePowerBI")]
    OfficePowerBi,
    AmazonWebServicesCloudTrail,
    AmazonWebServicesS3,
    AzureAdvancedThreatProtection,
    MicrosoftDefenderAdvancedThreatProtection,
    Dynamics365,
    MicrosoftThreatProtection,
    MicrosoftThreatIntelligence,
    #[serde(rename = "GenericUI")]
    GenericUi,
    #[serde(rename = "APIPolling")]
    ApiPolling,
    #[serde(rename = "IOT")]
    Iot,
    #[serde(rename = "GCP")]
    Gcp,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DataConnectorKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DataConnectorKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DataConnectorKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AzureActiveDirectory => serializer.serialize_unit_variant("DataConnectorKind", 0u32, "AzureActiveDirectory"),
            Self::AzureSecurityCenter => serializer.serialize_unit_variant("DataConnectorKind", 1u32, "AzureSecurityCenter"),
            Self::MicrosoftCloudAppSecurity => serializer.serialize_unit_variant("DataConnectorKind", 2u32, "MicrosoftCloudAppSecurity"),
            Self::ThreatIntelligence => serializer.serialize_unit_variant("DataConnectorKind", 3u32, "ThreatIntelligence"),
            Self::ThreatIntelligenceTaxii => serializer.serialize_unit_variant("DataConnectorKind", 4u32, "ThreatIntelligenceTaxii"),
            Self::Office365 => serializer.serialize_unit_variant("DataConnectorKind", 5u32, "Office365"),
            Self::OfficeAtp => serializer.serialize_unit_variant("DataConnectorKind", 6u32, "OfficeATP"),
            Self::OfficeIrm => serializer.serialize_unit_variant("DataConnectorKind", 7u32, "OfficeIRM"),
            Self::Office365Project => serializer.serialize_unit_variant("DataConnectorKind", 8u32, "Office365Project"),
            Self::MicrosoftPurviewInformationProtection => {
                serializer.serialize_unit_variant("DataConnectorKind", 9u32, "MicrosoftPurviewInformationProtection")
            }
            Self::OfficePowerBi => serializer.serialize_unit_variant("DataConnectorKind", 10u32, "OfficePowerBI"),
            Self::AmazonWebServicesCloudTrail => {
                serializer.serialize_unit_variant("DataConnectorKind", 11u32, "AmazonWebServicesCloudTrail")
            }
            Self::AmazonWebServicesS3 => serializer.serialize_unit_variant("DataConnectorKind", 12u32, "AmazonWebServicesS3"),
            Self::AzureAdvancedThreatProtection => {
                serializer.serialize_unit_variant("DataConnectorKind", 13u32, "AzureAdvancedThreatProtection")
            }
            Self::MicrosoftDefenderAdvancedThreatProtection => {
                serializer.serialize_unit_variant("DataConnectorKind", 14u32, "MicrosoftDefenderAdvancedThreatProtection")
            }
            Self::Dynamics365 => serializer.serialize_unit_variant("DataConnectorKind", 15u32, "Dynamics365"),
            Self::MicrosoftThreatProtection => serializer.serialize_unit_variant("DataConnectorKind", 16u32, "MicrosoftThreatProtection"),
            Self::MicrosoftThreatIntelligence => {
                serializer.serialize_unit_variant("DataConnectorKind", 17u32, "MicrosoftThreatIntelligence")
            }
            Self::GenericUi => serializer.serialize_unit_variant("DataConnectorKind", 18u32, "GenericUI"),
            Self::ApiPolling => serializer.serialize_unit_variant("DataConnectorKind", 19u32, "APIPolling"),
            Self::Iot => serializer.serialize_unit_variant("DataConnectorKind", 20u32, "IOT"),
            Self::Gcp => serializer.serialize_unit_variant("DataConnectorKind", 21u32, "GCP"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the state of user's license for a connector kind."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataConnectorLicenseState")]
pub enum DataConnectorLicenseState {
    Valid,
    Invalid,
    Unknown,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DataConnectorLicenseState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DataConnectorLicenseState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DataConnectorLicenseState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Valid => serializer.serialize_unit_variant("DataConnectorLicenseState", 0u32, "Valid"),
            Self::Invalid => serializer.serialize_unit_variant("DataConnectorLicenseState", 1u32, "Invalid"),
            Self::Unknown => serializer.serialize_unit_variant("DataConnectorLicenseState", 2u32, "Unknown"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "List all the data connectors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectorList {
    #[doc = "URL to fetch the next set of data connectors."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of data connectors."]
    pub value: Vec<DataConnectorUnion>,
}
impl azure_core::Continuable for DataConnectorList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DataConnectorList {
    pub fn new(value: Vec<DataConnectorUnion>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Data connector requirements status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataConnectorRequirementsState {
    #[doc = "Describes the state of user's authorization for a connector kind."]
    #[serde(rename = "authorizationState", default, skip_serializing_if = "Option::is_none")]
    pub authorization_state: Option<DataConnectorAuthorizationState>,
    #[doc = "Describes the state of user's license for a connector kind."]
    #[serde(rename = "licenseState", default, skip_serializing_if = "Option::is_none")]
    pub license_state: Option<DataConnectorLicenseState>,
}
impl DataConnectorRequirementsState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties data connector on tenant level."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectorTenantId {
    #[doc = "The tenant id to connect to, and get the data from."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
}
impl DataConnectorTenantId {
    pub fn new(tenant_id: String) -> Self {
        Self { tenant_id }
    }
}
#[doc = "Data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataConnectorWithAlertsProperties {
    #[doc = "Alerts data type for data connectors."]
    #[serde(rename = "dataTypes", default, skip_serializing_if = "Option::is_none")]
    pub data_types: Option<AlertsDataTypeOfDataConnector>,
}
impl DataConnectorWithAlertsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kind of the data connector"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum DataConnectorsCheckRequirementsUnion {
    AzureActiveDirectory(AadCheckRequirements),
    AzureAdvancedThreatProtection(AatpCheckRequirements),
    AzureSecurityCenter(AscCheckRequirements),
    AmazonWebServicesCloudTrail(AwsCloudTrailCheckRequirements),
    AmazonWebServicesS3(AwsS3CheckRequirements),
    Dynamics365(Dynamics365CheckRequirements),
    #[serde(rename = "IOT")]
    Iot(IoTCheckRequirements),
    MicrosoftCloudAppSecurity(McasCheckRequirements),
    MicrosoftDefenderAdvancedThreatProtection(MdatpCheckRequirements),
    MicrosoftThreatIntelligence(MstiCheckRequirements),
    MicrosoftPurviewInformationProtection(MicrosoftPurviewInformationProtectionCheckRequirements),
    MicrosoftThreatProtection(MtpCheckRequirements),
    Office365Project(Office365ProjectCheckRequirements),
    #[serde(rename = "OfficeATP")]
    OfficeAtp(OfficeAtpCheckRequirements),
    #[serde(rename = "OfficeIRM")]
    OfficeIrm(OfficeIrmCheckRequirements),
    #[serde(rename = "OfficePowerBI")]
    OfficePowerBi(OfficePowerBiCheckRequirements),
    ThreatIntelligence(TiCheckRequirements),
    ThreatIntelligenceTaxii(TiTaxiiCheckRequirements),
}
#[doc = "The data type definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataTypeDefinitions {
    #[doc = "The data type name"]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
}
impl DataTypeDefinitions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description about a deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Deployment {
    #[doc = "Deployment identifier."]
    #[serde(rename = "deploymentId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[doc = "The current state of the deployment."]
    #[serde(rename = "deploymentState", default, skip_serializing_if = "Option::is_none")]
    pub deployment_state: Option<DeploymentState>,
    #[doc = "Status while trying to fetch the deployment information."]
    #[serde(rename = "deploymentResult", default, skip_serializing_if = "Option::is_none")]
    pub deployment_result: Option<DeploymentResult>,
    #[doc = "The time when the deployment finished."]
    #[serde(rename = "deploymentTime", default, with = "azure_core::date::rfc3339::option")]
    pub deployment_time: Option<time::OffsetDateTime>,
    #[doc = "Url to access repository action logs."]
    #[serde(rename = "deploymentLogsUrl", default, skip_serializing_if = "Option::is_none")]
    pub deployment_logs_url: Option<String>,
}
impl Deployment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status while trying to fetch the deployment information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeploymentFetchStatus")]
pub enum DeploymentFetchStatus {
    Success,
    Unauthorized,
    NotFound,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeploymentFetchStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeploymentFetchStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeploymentFetchStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Success => serializer.serialize_unit_variant("DeploymentFetchStatus", 0u32, "Success"),
            Self::Unauthorized => serializer.serialize_unit_variant("DeploymentFetchStatus", 1u32, "Unauthorized"),
            Self::NotFound => serializer.serialize_unit_variant("DeploymentFetchStatus", 2u32, "NotFound"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Information regarding a deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentInfo {
    #[doc = "Status while trying to fetch the deployment information."]
    #[serde(rename = "deploymentFetchStatus", default, skip_serializing_if = "Option::is_none")]
    pub deployment_fetch_status: Option<DeploymentFetchStatus>,
    #[doc = "Description about a deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment: Option<Deployment>,
    #[doc = "Additional details about the deployment that can be shown to the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl DeploymentInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status while trying to fetch the deployment information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeploymentResult")]
pub enum DeploymentResult {
    Success,
    Canceled,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeploymentResult {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeploymentResult {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeploymentResult {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Success => serializer.serialize_unit_variant("DeploymentResult", 0u32, "Success"),
            Self::Canceled => serializer.serialize_unit_variant("DeploymentResult", 1u32, "Canceled"),
            Self::Failed => serializer.serialize_unit_variant("DeploymentResult", 2u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The current state of the deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeploymentState")]
pub enum DeploymentState {
    #[serde(rename = "In_Progress")]
    InProgress,
    Completed,
    Queued,
    Canceling,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeploymentState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeploymentState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeploymentState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::InProgress => serializer.serialize_unit_variant("DeploymentState", 0u32, "In_Progress"),
            Self::Completed => serializer.serialize_unit_variant("DeploymentState", 1u32, "Completed"),
            Self::Queued => serializer.serialize_unit_variant("DeploymentState", 2u32, "Queued"),
            Self::Canceling => serializer.serialize_unit_variant("DeploymentState", 3u32, "Canceling"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents Dynamics365 requirements check request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dynamics365CheckRequirements {
    #[doc = "Dynamics365 requirements check properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Dynamics365CheckRequirementsProperties>,
}
impl Dynamics365CheckRequirements {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "Dynamics365 requirements check properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dynamics365CheckRequirementsProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
}
impl Dynamics365CheckRequirementsProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self { data_connector_tenant_id }
    }
}
#[doc = "Represents Dynamics365 data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dynamics365DataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "Dynamics365 data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Dynamics365DataConnectorProperties>,
}
impl Dynamics365DataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "The available data types for Dynamics365 data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dynamics365DataConnectorDataTypes {
    #[doc = "Common Data Service data type connection."]
    #[serde(rename = "dynamics365CdsActivities")]
    pub dynamics365_cds_activities: serde_json::Value,
}
impl Dynamics365DataConnectorDataTypes {
    pub fn new(dynamics365_cds_activities: serde_json::Value) -> Self {
        Self {
            dynamics365_cds_activities,
        }
    }
}
#[doc = "Dynamics365 data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dynamics365DataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[doc = "The available data types for Dynamics365 data connector."]
    #[serde(rename = "dataTypes")]
    pub data_types: Dynamics365DataConnectorDataTypes,
}
impl Dynamics365DataConnectorProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId, data_types: Dynamics365DataConnectorDataTypes) -> Self {
        Self {
            data_connector_tenant_id,
            data_types,
        }
    }
}
#[doc = "Whois information for a given domain and associated metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrichmentDomainWhois {
    #[doc = "The domain for this whois record"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "The hostname of this registrar's whois server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[doc = "The timestamp at which this record was created"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "The timestamp at which this record was last updated"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub updated: Option<time::OffsetDateTime>,
    #[doc = "The timestamp at which this record will expire"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub expires: Option<time::OffsetDateTime>,
    #[doc = "The whois record for a given domain"]
    #[serde(rename = "parsedWhois", default, skip_serializing_if = "Option::is_none")]
    pub parsed_whois: Option<EnrichmentDomainWhoisDetails>,
}
impl EnrichmentDomainWhois {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An individual contact associated with this domain"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrichmentDomainWhoisContact {
    #[doc = "The name of this contact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The organization for this contact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org: Option<String>,
    #[doc = "A list describing the street address for this contact"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub street: Vec<String>,
    #[doc = "The city for this contact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "The state for this contact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The postal code for this contact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal: Option<String>,
    #[doc = "The country for this contact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "The phone number for this contact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "The fax number for this contact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[doc = "The email address for this contact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
impl EnrichmentDomainWhoisContact {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The set of contacts associated with this domain"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrichmentDomainWhoisContacts {
    #[doc = "An individual contact associated with this domain"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin: Option<EnrichmentDomainWhoisContact>,
    #[doc = "An individual contact associated with this domain"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing: Option<EnrichmentDomainWhoisContact>,
    #[doc = "An individual contact associated with this domain"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registrant: Option<EnrichmentDomainWhoisContact>,
    #[doc = "An individual contact associated with this domain"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tech: Option<EnrichmentDomainWhoisContact>,
}
impl EnrichmentDomainWhoisContacts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The whois record for a given domain"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrichmentDomainWhoisDetails {
    #[doc = "The registrar associated with this domain"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registrar: Option<EnrichmentDomainWhoisRegistrarDetails>,
    #[doc = "The set of contacts associated with this domain"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contacts: Option<EnrichmentDomainWhoisContacts>,
    #[doc = "A list of name servers associated with this domain"]
    #[serde(
        rename = "nameServers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub name_servers: Vec<String>,
    #[doc = "The set of status flags for this whois record"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub statuses: Vec<String>,
}
impl EnrichmentDomainWhoisDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The registrar associated with this domain"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrichmentDomainWhoisRegistrarDetails {
    #[doc = "The name of this registrar"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "This registrar's abuse contact email"]
    #[serde(rename = "abuseContactEmail", default, skip_serializing_if = "Option::is_none")]
    pub abuse_contact_email: Option<String>,
    #[doc = "This registrar's abuse contact phone number"]
    #[serde(rename = "abuseContactPhone", default, skip_serializing_if = "Option::is_none")]
    pub abuse_contact_phone: Option<String>,
    #[doc = "This registrar's Internet Assigned Numbers Authority id"]
    #[serde(rename = "ianaId", default, skip_serializing_if = "Option::is_none")]
    pub iana_id: Option<String>,
    #[doc = "This registrar's URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The hostname of this registrar's whois server"]
    #[serde(rename = "whoisServer", default, skip_serializing_if = "Option::is_none")]
    pub whois_server: Option<String>,
}
impl EnrichmentDomainWhoisRegistrarDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Geodata information for a given IP address"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrichmentIpGeodata {
    #[doc = "The autonomous system number associated with this IP address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asn: Option<String>,
    #[doc = "The name of the carrier for this IP address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[doc = "The city this IP address is located in"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "A numeric rating of confidence that the value in the 'city' field is correct, on a scale of 0-100"]
    #[serde(rename = "cityCf", default, skip_serializing_if = "Option::is_none")]
    pub city_cf: Option<i32>,
    #[doc = "The continent this IP address is located on"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continent: Option<String>,
    #[doc = "The county this IP address is located in"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "A numeric rating of confidence that the value in the 'country' field is correct on a scale of 0-100"]
    #[serde(rename = "countryCf", default, skip_serializing_if = "Option::is_none")]
    pub country_cf: Option<i32>,
    #[doc = "The dotted-decimal or colon-separated string representation of the IP address"]
    #[serde(rename = "ipAddr", default, skip_serializing_if = "Option::is_none")]
    pub ip_addr: Option<String>,
    #[doc = "A description of the connection type of this IP address"]
    #[serde(rename = "ipRoutingType", default, skip_serializing_if = "Option::is_none")]
    pub ip_routing_type: Option<String>,
    #[doc = "The latitude of this IP address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    #[doc = "The longitude of this IP address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
    #[doc = "The name of the organization for this IP address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[doc = "The type of the organization for this IP address"]
    #[serde(rename = "organizationType", default, skip_serializing_if = "Option::is_none")]
    pub organization_type: Option<String>,
    #[doc = "The geographic region this IP address is located in"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "The state this IP address is located in"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "A numeric rating of confidence that the value in the 'state' field is correct on a scale of 0-100"]
    #[serde(rename = "stateCf", default, skip_serializing_if = "Option::is_none")]
    pub state_cf: Option<i32>,
    #[doc = "The abbreviated name for the state this IP address is located in"]
    #[serde(rename = "stateCode", default, skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
}
impl EnrichmentIpGeodata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specific entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    #[serde(flatten)]
    pub resource: Resource,
}
impl Entity {
    pub fn new() -> Self {
        Self {
            resource: Resource::default(),
        }
    }
}
#[doc = "The kind of the entity"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum EntityUnion {
    Bookmark(HuntingBookmark),
    SecurityAlert(SecurityAlert),
}
#[doc = "Settings with single toggle."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityAnalytics {
    #[serde(flatten)]
    pub settings: Settings,
    #[doc = "EntityAnalytics property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EntityAnalyticsProperties>,
}
impl EntityAnalytics {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            properties: None,
        }
    }
}
#[doc = "EntityAnalytics property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityAnalyticsProperties {
    #[doc = "The relevant entity providers that are synced"]
    #[serde(
        rename = "entityProviders",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub entity_providers: Vec<EntityProviders>,
}
impl EntityAnalyticsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Entity common property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityCommonProperties {
    #[doc = "A bag of custom fields that should be part of the entity and will be presented to the user."]
    #[serde(rename = "additionalData", default, skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<serde_json::Value>,
    #[doc = "The graph item display name which is a short humanly readable description of the graph item instance. This property is optional and might be system generated."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl EntityCommonProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The edge that connects the entity to the other entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityEdges {
    #[doc = "The target entity Id."]
    #[serde(rename = "targetEntityId", default, skip_serializing_if = "Option::is_none")]
    pub target_entity_id: Option<String>,
    #[doc = "A bag of custom fields that should be part of the entity and will be presented to the user."]
    #[serde(rename = "additionalData", default, skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<serde_json::Value>,
}
impl EntityEdges {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters required to execute an expand operation on the given entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityExpandParameters {
    #[doc = "The end date filter, so the only expansion results returned are before this date."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The Id of the expansion to perform."]
    #[serde(rename = "expansionId", default, skip_serializing_if = "Option::is_none")]
    pub expansion_id: Option<String>,
    #[doc = "The start date filter, so the only expansion results returned are after this date."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
}
impl EntityExpandParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The entity expansion result operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityExpandResponse {
    #[doc = "Expansion result metadata."]
    #[serde(rename = "metaData", default, skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<ExpansionResultsMetadata>,
    #[doc = "The expansion result values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<entity_expand_response::Value>,
}
impl EntityExpandResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod entity_expand_response {
    use super::*;
    #[doc = "The expansion result values."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Value {
        #[doc = "Array of the expansion result entities."]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub entities: Vec<EntityUnion>,
        #[doc = "Array of edges that connects the entity to the list of entities."]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub edges: Vec<EntityEdges>,
    }
    impl Value {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Map identifiers of a single entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityFieldMapping {
    #[doc = "Alert V3 identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[doc = "The value of the identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl EntityFieldMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters required to execute insights operation on the given entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityGetInsightsParameters {
    #[doc = "The start timeline date, so the results returned are after this date."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "The end timeline date, so the results returned are before this date."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339")]
    pub end_time: time::OffsetDateTime,
    #[doc = "Indicates if query time range should be extended with default time range of the query. Default value is false"]
    #[serde(rename = "addDefaultExtendedTimeRange", default, skip_serializing_if = "Option::is_none")]
    pub add_default_extended_time_range: Option<bool>,
    #[doc = "List of Insights Query Id. If empty, default value is all insights of this entity"]
    #[serde(
        rename = "insightQueryIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub insight_query_ids: Vec<String>,
}
impl EntityGetInsightsParameters {
    pub fn new(start_time: time::OffsetDateTime, end_time: time::OffsetDateTime) -> Self {
        Self {
            start_time,
            end_time,
            add_default_extended_time_range: None,
            insight_query_ids: Vec::new(),
        }
    }
}
#[doc = "The Get Insights result operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityGetInsightsResponse {
    #[doc = "Get Insights result metadata."]
    #[serde(rename = "metaData", default, skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<GetInsightsResultsMetadata>,
    #[doc = "The insights result values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EntityInsightItem>,
}
impl EntityGetInsightsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kind of the entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EntityInnerKind")]
pub enum EntityInnerKind {
    Account,
    Host,
    File,
    AzureResource,
    CloudApplication,
    DnsResolution,
    FileHash,
    Ip,
    Malware,
    Process,
    RegistryKey,
    RegistryValue,
    SecurityGroup,
    Url,
    IoTDevice,
    SecurityAlert,
    Bookmark,
    Mailbox,
    MailCluster,
    MailMessage,
    SubmissionMail,
    Nic,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EntityInnerKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EntityInnerKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EntityInnerKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Account => serializer.serialize_unit_variant("EntityInnerKind", 0u32, "Account"),
            Self::Host => serializer.serialize_unit_variant("EntityInnerKind", 1u32, "Host"),
            Self::File => serializer.serialize_unit_variant("EntityInnerKind", 2u32, "File"),
            Self::AzureResource => serializer.serialize_unit_variant("EntityInnerKind", 3u32, "AzureResource"),
            Self::CloudApplication => serializer.serialize_unit_variant("EntityInnerKind", 4u32, "CloudApplication"),
            Self::DnsResolution => serializer.serialize_unit_variant("EntityInnerKind", 5u32, "DnsResolution"),
            Self::FileHash => serializer.serialize_unit_variant("EntityInnerKind", 6u32, "FileHash"),
            Self::Ip => serializer.serialize_unit_variant("EntityInnerKind", 7u32, "Ip"),
            Self::Malware => serializer.serialize_unit_variant("EntityInnerKind", 8u32, "Malware"),
            Self::Process => serializer.serialize_unit_variant("EntityInnerKind", 9u32, "Process"),
            Self::RegistryKey => serializer.serialize_unit_variant("EntityInnerKind", 10u32, "RegistryKey"),
            Self::RegistryValue => serializer.serialize_unit_variant("EntityInnerKind", 11u32, "RegistryValue"),
            Self::SecurityGroup => serializer.serialize_unit_variant("EntityInnerKind", 12u32, "SecurityGroup"),
            Self::Url => serializer.serialize_unit_variant("EntityInnerKind", 13u32, "Url"),
            Self::IoTDevice => serializer.serialize_unit_variant("EntityInnerKind", 14u32, "IoTDevice"),
            Self::SecurityAlert => serializer.serialize_unit_variant("EntityInnerKind", 15u32, "SecurityAlert"),
            Self::Bookmark => serializer.serialize_unit_variant("EntityInnerKind", 16u32, "Bookmark"),
            Self::Mailbox => serializer.serialize_unit_variant("EntityInnerKind", 17u32, "Mailbox"),
            Self::MailCluster => serializer.serialize_unit_variant("EntityInnerKind", 18u32, "MailCluster"),
            Self::MailMessage => serializer.serialize_unit_variant("EntityInnerKind", 19u32, "MailMessage"),
            Self::SubmissionMail => serializer.serialize_unit_variant("EntityInnerKind", 20u32, "SubmissionMail"),
            Self::Nic => serializer.serialize_unit_variant("EntityInnerKind", 21u32, "Nic"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type of the entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EntityInnerType")]
pub enum EntityInnerType {
    Account,
    Host,
    File,
    AzureResource,
    CloudApplication,
    #[serde(rename = "DNS")]
    Dns,
    FileHash,
    #[serde(rename = "IP")]
    Ip,
    Malware,
    Process,
    RegistryKey,
    RegistryValue,
    SecurityGroup,
    #[serde(rename = "URL")]
    Url,
    IoTDevice,
    SecurityAlert,
    HuntingBookmark,
    MailCluster,
    MailMessage,
    Mailbox,
    SubmissionMail,
    Nic,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EntityInnerType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EntityInnerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EntityInnerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Account => serializer.serialize_unit_variant("EntityInnerType", 0u32, "Account"),
            Self::Host => serializer.serialize_unit_variant("EntityInnerType", 1u32, "Host"),
            Self::File => serializer.serialize_unit_variant("EntityInnerType", 2u32, "File"),
            Self::AzureResource => serializer.serialize_unit_variant("EntityInnerType", 3u32, "AzureResource"),
            Self::CloudApplication => serializer.serialize_unit_variant("EntityInnerType", 4u32, "CloudApplication"),
            Self::Dns => serializer.serialize_unit_variant("EntityInnerType", 5u32, "DNS"),
            Self::FileHash => serializer.serialize_unit_variant("EntityInnerType", 6u32, "FileHash"),
            Self::Ip => serializer.serialize_unit_variant("EntityInnerType", 7u32, "IP"),
            Self::Malware => serializer.serialize_unit_variant("EntityInnerType", 8u32, "Malware"),
            Self::Process => serializer.serialize_unit_variant("EntityInnerType", 9u32, "Process"),
            Self::RegistryKey => serializer.serialize_unit_variant("EntityInnerType", 10u32, "RegistryKey"),
            Self::RegistryValue => serializer.serialize_unit_variant("EntityInnerType", 11u32, "RegistryValue"),
            Self::SecurityGroup => serializer.serialize_unit_variant("EntityInnerType", 12u32, "SecurityGroup"),
            Self::Url => serializer.serialize_unit_variant("EntityInnerType", 13u32, "URL"),
            Self::IoTDevice => serializer.serialize_unit_variant("EntityInnerType", 14u32, "IoTDevice"),
            Self::SecurityAlert => serializer.serialize_unit_variant("EntityInnerType", 15u32, "SecurityAlert"),
            Self::HuntingBookmark => serializer.serialize_unit_variant("EntityInnerType", 16u32, "HuntingBookmark"),
            Self::MailCluster => serializer.serialize_unit_variant("EntityInnerType", 17u32, "MailCluster"),
            Self::MailMessage => serializer.serialize_unit_variant("EntityInnerType", 18u32, "MailMessage"),
            Self::Mailbox => serializer.serialize_unit_variant("EntityInnerType", 19u32, "Mailbox"),
            Self::SubmissionMail => serializer.serialize_unit_variant("EntityInnerType", 20u32, "SubmissionMail"),
            Self::Nic => serializer.serialize_unit_variant("EntityInnerType", 21u32, "Nic"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Entity insight Item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityInsightItem {
    #[doc = "The query id of the insight"]
    #[serde(rename = "queryId", default, skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[doc = "The Time interval that the query actually executed on."]
    #[serde(rename = "queryTimeInterval", default, skip_serializing_if = "Option::is_none")]
    pub query_time_interval: Option<entity_insight_item::QueryTimeInterval>,
    #[doc = "Query results for table insights query."]
    #[serde(rename = "tableQueryResults", default, skip_serializing_if = "Option::is_none")]
    pub table_query_results: Option<InsightsTableResult>,
    #[doc = "Query results for table insights query."]
    #[serde(
        rename = "chartQueryResults",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub chart_query_results: Vec<InsightsTableResult>,
}
impl EntityInsightItem {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod entity_insight_item {
    use super::*;
    #[doc = "The Time interval that the query actually executed on."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct QueryTimeInterval {
        #[doc = "Insight query start time"]
        #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
        pub start_time: Option<time::OffsetDateTime>,
        #[doc = "Insight query end time"]
        #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
        pub end_time: Option<time::OffsetDateTime>,
    }
    impl QueryTimeInterval {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "List of all the entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityList {
    #[doc = "URL to fetch the next set of entities."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of entities."]
    pub value: Vec<EntityUnion>,
}
impl azure_core::Continuable for EntityList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EntityList {
    pub fn new(value: Vec<EntityUnion>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Describes the request body for triggering a playbook on an entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityManualTriggerRequestBody {
    #[doc = "Incident ARM id."]
    #[serde(rename = "incidentArmId", default, skip_serializing_if = "Option::is_none")]
    pub incident_arm_id: Option<String>,
    #[doc = "The tenant id of the playbook resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The resource id of the playbook resource."]
    #[serde(rename = "logicAppsResourceId")]
    pub logic_apps_resource_id: String,
}
impl EntityManualTriggerRequestBody {
    pub fn new(logic_apps_resource_id: String) -> Self {
        Self {
            incident_arm_id: None,
            tenant_id: None,
            logic_apps_resource_id,
        }
    }
}
#[doc = "Single entity mapping for the alert rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityMapping {
    #[doc = "The V3 type of the mapped entity"]
    #[serde(rename = "entityType", default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<EntityMappingType>,
    #[doc = "array of field mappings for the given entity mapping"]
    #[serde(
        rename = "fieldMappings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub field_mappings: Vec<FieldMapping>,
}
impl EntityMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The V3 type of the mapped entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EntityMappingType")]
pub enum EntityMappingType {
    Account,
    Host,
    #[serde(rename = "IP")]
    Ip,
    Malware,
    File,
    Process,
    CloudApplication,
    #[serde(rename = "DNS")]
    Dns,
    AzureResource,
    FileHash,
    RegistryKey,
    RegistryValue,
    SecurityGroup,
    #[serde(rename = "URL")]
    Url,
    Mailbox,
    MailCluster,
    MailMessage,
    SubmissionMail,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EntityMappingType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EntityMappingType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EntityMappingType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Account => serializer.serialize_unit_variant("EntityMappingType", 0u32, "Account"),
            Self::Host => serializer.serialize_unit_variant("EntityMappingType", 1u32, "Host"),
            Self::Ip => serializer.serialize_unit_variant("EntityMappingType", 2u32, "IP"),
            Self::Malware => serializer.serialize_unit_variant("EntityMappingType", 3u32, "Malware"),
            Self::File => serializer.serialize_unit_variant("EntityMappingType", 4u32, "File"),
            Self::Process => serializer.serialize_unit_variant("EntityMappingType", 5u32, "Process"),
            Self::CloudApplication => serializer.serialize_unit_variant("EntityMappingType", 6u32, "CloudApplication"),
            Self::Dns => serializer.serialize_unit_variant("EntityMappingType", 7u32, "DNS"),
            Self::AzureResource => serializer.serialize_unit_variant("EntityMappingType", 8u32, "AzureResource"),
            Self::FileHash => serializer.serialize_unit_variant("EntityMappingType", 9u32, "FileHash"),
            Self::RegistryKey => serializer.serialize_unit_variant("EntityMappingType", 10u32, "RegistryKey"),
            Self::RegistryValue => serializer.serialize_unit_variant("EntityMappingType", 11u32, "RegistryValue"),
            Self::SecurityGroup => serializer.serialize_unit_variant("EntityMappingType", 12u32, "SecurityGroup"),
            Self::Url => serializer.serialize_unit_variant("EntityMappingType", 13u32, "URL"),
            Self::Mailbox => serializer.serialize_unit_variant("EntityMappingType", 14u32, "Mailbox"),
            Self::MailCluster => serializer.serialize_unit_variant("EntityMappingType", 15u32, "MailCluster"),
            Self::MailMessage => serializer.serialize_unit_variant("EntityMappingType", 16u32, "MailMessage"),
            Self::SubmissionMail => serializer.serialize_unit_variant("EntityMappingType", 17u32, "SubmissionMail"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type EntityMappings = Vec<EntityMapping>;
pub type EntityMappingsList = Vec<BookmarkEntityMappings>;
#[doc = "The entity provider that is synced."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EntityProviders")]
pub enum EntityProviders {
    ActiveDirectory,
    AzureActiveDirectory,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EntityProviders {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EntityProviders {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EntityProviders {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ActiveDirectory => serializer.serialize_unit_variant("EntityProviders", 0u32, "ActiveDirectory"),
            Self::AzureActiveDirectory => serializer.serialize_unit_variant("EntityProviders", 1u32, "AzureActiveDirectory"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specific entity query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityQuery {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
}
impl EntityQuery {
    pub fn new() -> Self {
        Self {
            resource_with_etag: ResourceWithEtag::default(),
        }
    }
}
#[doc = "The kind of the entity query"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum EntityQueryUnion {
    Activity(ActivityEntityQuery),
    Expansion(ExpansionEntityQuery),
}
#[doc = "An abstract Query item for entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityQueryItem {
    #[doc = "Query Template ARM ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Query Template ARM Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "ARM Type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl EntityQueryItem {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
        }
    }
}
#[doc = "The kind of the entity query"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum EntityQueryItemUnion {
    Insight(InsightQueryItem),
}
#[doc = "An properties abstract Query item for entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityQueryItemProperties {
    #[doc = "Data types for template"]
    #[serde(
        rename = "dataTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_types: Vec<serde_json::Value>,
    #[doc = "The type of the entity"]
    #[serde(rename = "inputEntityType", default, skip_serializing_if = "Option::is_none")]
    pub input_entity_type: Option<EntityInnerType>,
    #[doc = "Data types for template"]
    #[serde(
        rename = "requiredInputFieldsSets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_input_fields_sets: Vec<Vec<String>>,
    #[doc = "The query applied only to entities matching to all filters"]
    #[serde(rename = "entitiesFilter", default, skip_serializing_if = "Option::is_none")]
    pub entities_filter: Option<serde_json::Value>,
}
impl EntityQueryItemProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kind of the entity query"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EntityQueryKind")]
pub enum EntityQueryKind {
    Expansion,
    Insight,
    Activity,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EntityQueryKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EntityQueryKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EntityQueryKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Expansion => serializer.serialize_unit_variant("EntityQueryKind", 0u32, "Expansion"),
            Self::Insight => serializer.serialize_unit_variant("EntityQueryKind", 1u32, "Insight"),
            Self::Activity => serializer.serialize_unit_variant("EntityQueryKind", 2u32, "Activity"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "List of all the entity queries."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityQueryList {
    #[doc = "URL to fetch the next set of entity queries."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of entity queries."]
    pub value: Vec<EntityQueryUnion>,
}
impl azure_core::Continuable for EntityQueryList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EntityQueryList {
    pub fn new(value: Vec<EntityQueryUnion>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Specific entity query template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityQueryTemplate {
    #[serde(flatten)]
    pub resource: Resource,
}
impl EntityQueryTemplate {
    pub fn new() -> Self {
        Self {
            resource: Resource::default(),
        }
    }
}
#[doc = "The kind of the entity query template."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum EntityQueryTemplateUnion {
    Activity(ActivityEntityQueryTemplate),
}
#[doc = "The kind of the entity query template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EntityQueryTemplateKind")]
pub enum EntityQueryTemplateKind {
    Activity,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EntityQueryTemplateKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EntityQueryTemplateKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EntityQueryTemplateKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Activity => serializer.serialize_unit_variant("EntityQueryTemplateKind", 0u32, "Activity"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "List of all the entity query templates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityQueryTemplateList {
    #[doc = "URL to fetch the next set of entity query templates."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of entity query templates."]
    pub value: Vec<EntityQueryTemplateUnion>,
}
impl azure_core::Continuable for EntityQueryTemplateList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EntityQueryTemplateList {
    pub fn new(value: Vec<EntityQueryTemplateUnion>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "The entity query kind"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum EntityTimelineItemUnion {
    Activity(ActivityTimelineItem),
    Anomaly(AnomalyTimelineItem),
    Bookmark(BookmarkTimelineItem),
    SecurityAlert(SecurityAlertTimelineItem),
}
#[doc = "The entity query kind"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EntityTimelineKind")]
pub enum EntityTimelineKind {
    Activity,
    Bookmark,
    SecurityAlert,
    Anomaly,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EntityTimelineKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EntityTimelineKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EntityTimelineKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Activity => serializer.serialize_unit_variant("EntityTimelineKind", 0u32, "Activity"),
            Self::Bookmark => serializer.serialize_unit_variant("EntityTimelineKind", 1u32, "Bookmark"),
            Self::SecurityAlert => serializer.serialize_unit_variant("EntityTimelineKind", 2u32, "SecurityAlert"),
            Self::Anomaly => serializer.serialize_unit_variant("EntityTimelineKind", 3u32, "Anomaly"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The parameters required to execute s timeline operation on the given entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityTimelineParameters {
    #[doc = "Array of timeline Item kinds."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub kinds: Vec<EntityTimelineKind>,
    #[doc = "The start timeline date, so the results returned are after this date."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "The end timeline date, so the results returned are before this date."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339")]
    pub end_time: time::OffsetDateTime,
    #[doc = "The number of bucket for timeline queries aggregation."]
    #[serde(rename = "numberOfBucket", default, skip_serializing_if = "Option::is_none")]
    pub number_of_bucket: Option<i32>,
}
impl EntityTimelineParameters {
    pub fn new(start_time: time::OffsetDateTime, end_time: time::OffsetDateTime) -> Self {
        Self {
            kinds: Vec::new(),
            start_time,
            end_time,
            number_of_bucket: None,
        }
    }
}
#[doc = "The entity timeline result operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityTimelineResponse {
    #[doc = "Expansion result metadata."]
    #[serde(rename = "metaData", default, skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<TimelineResultsMetadata>,
    #[doc = "The timeline result values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EntityTimelineItemUnion>,
}
impl EntityTimelineResponse {
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "The event grouping aggregation kinds"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EventGroupingAggregationKind")]
pub enum EventGroupingAggregationKind {
    SingleAlert,
    AlertPerResult,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EventGroupingAggregationKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EventGroupingAggregationKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EventGroupingAggregationKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SingleAlert => serializer.serialize_unit_variant("EventGroupingAggregationKind", 0u32, "SingleAlert"),
            Self::AlertPerResult => serializer.serialize_unit_variant("EventGroupingAggregationKind", 1u32, "AlertPerResult"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Event grouping settings property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventGroupingSettings {
    #[doc = "The event grouping aggregation kinds"]
    #[serde(rename = "aggregationKind", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_kind: Option<EventGroupingAggregationKind>,
}
impl EventGroupingSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes expansion entity query properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpansionEntityQueriesProperties {
    #[doc = "List of the data sources that are required to run the query"]
    #[serde(
        rename = "dataSources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_sources: Vec<String>,
    #[doc = "The query display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The type of the entity"]
    #[serde(rename = "inputEntityType", default, skip_serializing_if = "Option::is_none")]
    pub input_entity_type: Option<EntityInnerType>,
    #[doc = "List of the fields of the source entity that are required to run the query"]
    #[serde(
        rename = "inputFields",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub input_fields: Vec<String>,
    #[doc = "List of the desired output types to be constructed from the result"]
    #[serde(
        rename = "outputEntityTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output_entity_types: Vec<EntityInnerType>,
    #[doc = "The template query string to be parsed and formatted"]
    #[serde(rename = "queryTemplate", default, skip_serializing_if = "Option::is_none")]
    pub query_template: Option<String>,
}
impl ExpansionEntityQueriesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents Expansion entity query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpansionEntityQuery {
    #[serde(flatten)]
    pub entity_query: EntityQuery,
    #[doc = "Describes expansion entity query properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExpansionEntityQueriesProperties>,
}
impl ExpansionEntityQuery {
    pub fn new(entity_query: EntityQuery) -> Self {
        Self {
            entity_query,
            properties: None,
        }
    }
}
#[doc = "Information of a specific aggregation in the expansion result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpansionResultAggregation {
    #[doc = "The common type of the aggregation. (for e.g. entity field name)"]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "Total number of aggregations of the given kind (and aggregationType if given) in the expansion result."]
    pub count: i32,
    #[doc = "The display name of the aggregation by type."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The kind of the entity"]
    #[serde(rename = "entityKind")]
    pub entity_kind: EntityInnerKind,
}
impl ExpansionResultAggregation {
    pub fn new(count: i32, entity_kind: EntityInnerKind) -> Self {
        Self {
            aggregation_type: None,
            count,
            display_name: None,
            entity_kind,
        }
    }
}
#[doc = "Expansion result metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpansionResultsMetadata {
    #[doc = "Information of the aggregated nodes in the expansion result."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub aggregations: Vec<ExpansionResultAggregation>,
}
impl ExpansionResultsMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Settings with single toggle."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EyesOn {
    #[serde(flatten)]
    pub settings: Settings,
    #[doc = "EyesOn property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EyesOnSettingsProperties>,
}
impl EyesOn {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            properties: None,
        }
    }
}
#[doc = "EyesOn property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EyesOnSettingsProperties {
    #[doc = "Determines whether the setting is enable or disabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}
impl EyesOnSettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A single field mapping of the mapped entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FieldMapping {
    #[doc = "the V3 identifier of the entity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[doc = "the column name to be mapped to the identifier"]
    #[serde(rename = "columnName", default, skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
}
impl FieldMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a file import in Azure Security Insights."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileImport {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the FileImport's properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FileImportProperties>,
}
impl FileImport {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List all the file imports."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileImportList {
    #[doc = "URL to fetch the next set of file imports."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of file imports."]
    pub value: Vec<FileImport>,
}
impl azure_core::Continuable for FileImportList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl FileImportList {
    pub fn new(value: Vec<FileImport>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Describes the FileImport's properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileImportProperties {
    #[doc = "Describes how to ingest the records in the file."]
    #[serde(rename = "ingestionMode")]
    pub ingestion_mode: file_import_properties::IngestionMode,
    #[doc = "The content type of this file."]
    #[serde(rename = "contentType")]
    pub content_type: file_import_properties::ContentType,
    #[doc = "The time the file was imported."]
    #[serde(rename = "createdTimeUTC", default, with = "azure_core::date::rfc3339::option")]
    pub created_time_utc: Option<time::OffsetDateTime>,
    #[doc = "Represents a file."]
    #[serde(rename = "errorFile", default, skip_serializing_if = "Option::is_none")]
    pub error_file: Option<FileMetadata>,
    #[doc = "An ordered list of some of the errors that were encountered during validation."]
    #[serde(
        rename = "errorsPreview",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors_preview: Vec<ValidationError>,
    #[doc = "Represents a file."]
    #[serde(rename = "importFile")]
    pub import_file: FileMetadata,
    #[doc = "The number of records that have been successfully ingested."]
    #[serde(rename = "ingestedRecordCount", default, skip_serializing_if = "Option::is_none")]
    pub ingested_record_count: Option<i32>,
    #[doc = "The source for the data in the file."]
    pub source: String,
    #[doc = "The state of the file import."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<file_import_properties::State>,
    #[doc = "The number of records in the file."]
    #[serde(rename = "totalRecordCount", default, skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
    #[doc = "The number of records that have passed validation."]
    #[serde(rename = "validRecordCount", default, skip_serializing_if = "Option::is_none")]
    pub valid_record_count: Option<i32>,
    #[doc = "The time the files associated with this import are deleted from the storage account."]
    #[serde(rename = "filesValidUntilTimeUTC", default, with = "azure_core::date::rfc3339::option")]
    pub files_valid_until_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The time the file import record is soft deleted from the database and history."]
    #[serde(rename = "importValidUntilTimeUTC", default, with = "azure_core::date::rfc3339::option")]
    pub import_valid_until_time_utc: Option<time::OffsetDateTime>,
}
impl FileImportProperties {
    pub fn new(
        ingestion_mode: file_import_properties::IngestionMode,
        content_type: file_import_properties::ContentType,
        import_file: FileMetadata,
        source: String,
    ) -> Self {
        Self {
            ingestion_mode,
            content_type,
            created_time_utc: None,
            error_file: None,
            errors_preview: Vec::new(),
            import_file,
            ingested_record_count: None,
            source,
            state: None,
            total_record_count: None,
            valid_record_count: None,
            files_valid_until_time_utc: None,
            import_valid_until_time_utc: None,
        }
    }
}
pub mod file_import_properties {
    use super::*;
    #[doc = "Describes how to ingest the records in the file."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IngestionMode")]
    pub enum IngestionMode {
        IngestOnlyIfAllAreValid,
        IngestAnyValidRecords,
        Unspecified,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IngestionMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IngestionMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IngestionMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IngestOnlyIfAllAreValid => serializer.serialize_unit_variant("IngestionMode", 0u32, "IngestOnlyIfAllAreValid"),
                Self::IngestAnyValidRecords => serializer.serialize_unit_variant("IngestionMode", 1u32, "IngestAnyValidRecords"),
                Self::Unspecified => serializer.serialize_unit_variant("IngestionMode", 2u32, "Unspecified"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The content type of this file."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ContentType")]
    pub enum ContentType {
        BasicIndicator,
        StixIndicator,
        Unspecified,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ContentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ContentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ContentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BasicIndicator => serializer.serialize_unit_variant("ContentType", 0u32, "BasicIndicator"),
                Self::StixIndicator => serializer.serialize_unit_variant("ContentType", 1u32, "StixIndicator"),
                Self::Unspecified => serializer.serialize_unit_variant("ContentType", 2u32, "Unspecified"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The state of the file import."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        FatalError,
        Ingested,
        IngestedWithErrors,
        InProgress,
        Invalid,
        WaitingForUpload,
        Unspecified,
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
                Self::FatalError => serializer.serialize_unit_variant("State", 0u32, "FatalError"),
                Self::Ingested => serializer.serialize_unit_variant("State", 1u32, "Ingested"),
                Self::IngestedWithErrors => serializer.serialize_unit_variant("State", 2u32, "IngestedWithErrors"),
                Self::InProgress => serializer.serialize_unit_variant("State", 3u32, "InProgress"),
                Self::Invalid => serializer.serialize_unit_variant("State", 4u32, "Invalid"),
                Self::WaitingForUpload => serializer.serialize_unit_variant("State", 5u32, "WaitingForUpload"),
                Self::Unspecified => serializer.serialize_unit_variant("State", 6u32, "Unspecified"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents a file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileMetadata {
    #[doc = "The format of the file"]
    #[serde(rename = "fileFormat", default, skip_serializing_if = "Option::is_none")]
    pub file_format: Option<file_metadata::FileFormat>,
    #[doc = "The name of the file."]
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[doc = "The size of the file."]
    #[serde(rename = "fileSize", default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i32>,
    #[doc = "A URI with a valid SAS token to allow uploading / downloading the file."]
    #[serde(rename = "fileContentUri", default, skip_serializing_if = "Option::is_none")]
    pub file_content_uri: Option<String>,
    #[doc = "Indicates whether the file was deleted from the storage account."]
    #[serde(rename = "deleteStatus", default, skip_serializing_if = "Option::is_none")]
    pub delete_status: Option<file_metadata::DeleteStatus>,
}
impl FileMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod file_metadata {
    use super::*;
    #[doc = "The format of the file"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FileFormat")]
    pub enum FileFormat {
        #[serde(rename = "CSV")]
        Csv,
        #[serde(rename = "JSON")]
        Json,
        Unspecified,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FileFormat {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FileFormat {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FileFormat {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Csv => serializer.serialize_unit_variant("FileFormat", 0u32, "CSV"),
                Self::Json => serializer.serialize_unit_variant("FileFormat", 1u32, "JSON"),
                Self::Unspecified => serializer.serialize_unit_variant("FileFormat", 2u32, "Unspecified"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates whether the file was deleted from the storage account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeleteStatus")]
    pub enum DeleteStatus {
        Deleted,
        NotDeleted,
        Unspecified,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeleteStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeleteStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeleteStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Deleted => serializer.serialize_unit_variant("DeleteStatus", 0u32, "Deleted"),
                Self::NotDeleted => serializer.serialize_unit_variant("DeleteStatus", 1u32, "NotDeleted"),
                Self::Unspecified => serializer.serialize_unit_variant("DeleteStatus", 2u32, "Unspecified"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents Fusion alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FusionAlertRule {
    #[serde(flatten)]
    pub alert_rule: AlertRule,
    #[doc = "Fusion alert rule base property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FusionAlertRuleProperties>,
}
impl FusionAlertRule {
    pub fn new(alert_rule: AlertRule) -> Self {
        Self {
            alert_rule,
            properties: None,
        }
    }
}
#[doc = "Fusion alert rule base property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FusionAlertRuleProperties {
    #[doc = "The Name of the alert rule template used to create this rule."]
    #[serde(rename = "alertRuleTemplateName")]
    pub alert_rule_template_name: String,
    #[doc = "The description of the alert rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display name for alerts created by this alert rule."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Determines whether this alert rule is enabled or disabled."]
    pub enabled: bool,
    #[doc = "Configuration for all supported source signals in fusion detection."]
    #[serde(
        rename = "sourceSettings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_settings: Vec<FusionSourceSettings>,
    #[doc = "Configuration to exclude scenarios in fusion detection."]
    #[serde(
        rename = "scenarioExclusionPatterns",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scenario_exclusion_patterns: Vec<FusionScenarioExclusionPattern>,
    #[doc = "The last time that this alert has been modified."]
    #[serde(rename = "lastModifiedUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_utc: Option<time::OffsetDateTime>,
    #[doc = "The severity of the alert"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<AlertSeverityEnum>,
    #[doc = "The tactics of the alert rule"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tactics: Vec<AttackTactic>,
    #[doc = "The techniques of the alert rule"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub techniques: Vec<String>,
}
impl FusionAlertRuleProperties {
    pub fn new(alert_rule_template_name: String, enabled: bool) -> Self {
        Self {
            alert_rule_template_name,
            description: None,
            display_name: None,
            enabled,
            source_settings: Vec::new(),
            scenario_exclusion_patterns: Vec::new(),
            last_modified_utc: None,
            severity: None,
            tactics: Vec::new(),
            techniques: Vec::new(),
        }
    }
}
#[doc = "Represents Fusion alert rule template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FusionAlertRuleTemplate {
    #[serde(flatten)]
    pub alert_rule_template: AlertRuleTemplate,
    #[doc = "Fusion alert rule template properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FusionAlertRuleTemplateProperties>,
}
impl FusionAlertRuleTemplate {
    pub fn new(alert_rule_template: AlertRuleTemplate) -> Self {
        Self {
            alert_rule_template,
            properties: None,
        }
    }
}
#[doc = "Fusion alert rule template properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FusionAlertRuleTemplateProperties {
    #[doc = "the number of alert rules that were created by this template"]
    #[serde(rename = "alertRulesCreatedByTemplateCount", default, skip_serializing_if = "Option::is_none")]
    pub alert_rules_created_by_template_count: Option<i32>,
    #[doc = "The time that this alert rule template has been added."]
    #[serde(rename = "createdDateUTC", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_utc: Option<time::OffsetDateTime>,
    #[doc = "The time that this alert rule template was last updated."]
    #[serde(rename = "lastUpdatedDateUTC", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_date_utc: Option<time::OffsetDateTime>,
    #[doc = "The description of the alert rule template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display name for alert rule template."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The required data connectors for this template"]
    #[serde(
        rename = "requiredDataConnectors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_data_connectors: Vec<AlertRuleTemplateDataSource>,
    #[doc = "The alert rule template status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AlertRuleTemplateStatus>,
    #[doc = "The severity of the alert"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<AlertSeverityEnum>,
    #[doc = "The tactics of the alert rule template"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tactics: Vec<AttackTactic>,
    #[doc = "The techniques of the alert rule"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub techniques: Vec<String>,
    #[doc = "All supported source signal configurations consumed in fusion detection."]
    #[serde(
        rename = "sourceSettings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_settings: Vec<FusionTemplateSourceSetting>,
}
impl FusionAlertRuleTemplateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Fusion scenario exclusion patterns in Fusion detection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FusionScenarioExclusionPattern {
    #[doc = "Scenario exclusion pattern."]
    #[serde(rename = "exclusionPattern")]
    pub exclusion_pattern: String,
    #[doc = "DateTime when scenario exclusion pattern is added in UTC."]
    #[serde(rename = "dateAddedInUTC")]
    pub date_added_in_utc: String,
}
impl FusionScenarioExclusionPattern {
    pub fn new(exclusion_pattern: String, date_added_in_utc: String) -> Self {
        Self {
            exclusion_pattern,
            date_added_in_utc,
        }
    }
}
#[doc = "Represents a supported source signal configuration in Fusion detection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FusionSourceSettings {
    #[doc = "Determines whether this source signal is enabled or disabled in Fusion detection."]
    pub enabled: bool,
    #[doc = "Name of the Fusion source signal. Refer to Fusion alert rule template for supported values."]
    #[serde(rename = "sourceName")]
    pub source_name: String,
    #[doc = "Configuration for all source subtypes under this source signal consumed in fusion detection."]
    #[serde(
        rename = "sourceSubTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_sub_types: Vec<FusionSourceSubTypeSetting>,
}
impl FusionSourceSettings {
    pub fn new(enabled: bool, source_name: String) -> Self {
        Self {
            enabled,
            source_name,
            source_sub_types: Vec::new(),
        }
    }
}
#[doc = "Represents a supported source subtype configuration under a source signal in Fusion detection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FusionSourceSubTypeSetting {
    #[doc = "Determines whether this source subtype under source signal is enabled or disabled in Fusion detection."]
    pub enabled: bool,
    #[doc = "The Name of the source subtype under a given source signal in Fusion detection. Refer to Fusion alert rule template for supported values."]
    #[serde(rename = "sourceSubTypeName")]
    pub source_sub_type_name: String,
    #[doc = "The display name of source subtype under a source signal consumed in Fusion detection."]
    #[serde(rename = "sourceSubTypeDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub source_sub_type_display_name: Option<String>,
    #[doc = "Represents severity configuration for a source subtype consumed in Fusion detection."]
    #[serde(rename = "severityFilters")]
    pub severity_filters: FusionSubTypeSeverityFilter,
}
impl FusionSourceSubTypeSetting {
    pub fn new(enabled: bool, source_sub_type_name: String, severity_filters: FusionSubTypeSeverityFilter) -> Self {
        Self {
            enabled,
            source_sub_type_name,
            source_sub_type_display_name: None,
            severity_filters,
        }
    }
}
#[doc = "Represents severity configuration for a source subtype consumed in Fusion detection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FusionSubTypeSeverityFilter {
    #[doc = "Determines whether this source subtype supports severity configuration or not."]
    #[serde(rename = "isSupported", default, skip_serializing_if = "Option::is_none")]
    pub is_supported: Option<bool>,
    #[doc = "Individual Severity configuration settings for a given source subtype consumed in Fusion detection."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub filters: Vec<FusionSubTypeSeverityFiltersItem>,
}
impl FusionSubTypeSeverityFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Severity filter setting for a given source subtype consumed in Fusion detection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FusionSubTypeSeverityFiltersItem {
    #[doc = "The severity of the alert"]
    pub severity: AlertSeverityEnum,
    #[doc = "Determines whether this severity is enabled or disabled for this source subtype consumed in Fusion detection."]
    pub enabled: bool,
}
impl FusionSubTypeSeverityFiltersItem {
    pub fn new(severity: AlertSeverityEnum, enabled: bool) -> Self {
        Self { severity, enabled }
    }
}
#[doc = "Represents a source signal consumed in Fusion detection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FusionTemplateSourceSetting {
    #[doc = "The name of a source signal consumed in Fusion detection."]
    #[serde(rename = "sourceName")]
    pub source_name: String,
    #[doc = "All supported source subtypes under this source signal consumed in fusion detection."]
    #[serde(
        rename = "sourceSubTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_sub_types: Vec<FusionTemplateSourceSubType>,
}
impl FusionTemplateSourceSetting {
    pub fn new(source_name: String) -> Self {
        Self {
            source_name,
            source_sub_types: Vec::new(),
        }
    }
}
#[doc = "Represents a source subtype under a source signal consumed in Fusion detection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FusionTemplateSourceSubType {
    #[doc = "The name of source subtype under a source signal consumed in Fusion detection."]
    #[serde(rename = "sourceSubTypeName")]
    pub source_sub_type_name: String,
    #[doc = "The display name of source subtype under a source signal consumed in Fusion detection."]
    #[serde(rename = "sourceSubTypeDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub source_sub_type_display_name: Option<String>,
    #[doc = "Represents severity configurations available for a source subtype consumed in Fusion detection."]
    #[serde(rename = "severityFilter")]
    pub severity_filter: FusionTemplateSubTypeSeverityFilter,
}
impl FusionTemplateSourceSubType {
    pub fn new(source_sub_type_name: String, severity_filter: FusionTemplateSubTypeSeverityFilter) -> Self {
        Self {
            source_sub_type_name,
            source_sub_type_display_name: None,
            severity_filter,
        }
    }
}
#[doc = "Represents severity configurations available for a source subtype consumed in Fusion detection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FusionTemplateSubTypeSeverityFilter {
    #[doc = "Determines whether severity configuration is supported for this source subtype consumed in Fusion detection."]
    #[serde(rename = "isSupported")]
    pub is_supported: bool,
    #[doc = "List of all supported severities for this source subtype consumed in Fusion detection."]
    #[serde(
        rename = "severityFilters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub severity_filters: Vec<AlertSeverityEnum>,
}
impl FusionTemplateSubTypeSeverityFilter {
    pub fn new(is_supported: bool) -> Self {
        Self {
            is_supported,
            severity_filters: Vec::new(),
        }
    }
}
#[doc = "Google Cloud Platform auth section properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcpAuthProperties {
    #[doc = "The service account that is used to access the GCP project."]
    #[serde(rename = "serviceAccountEmail")]
    pub service_account_email: String,
    #[doc = "The GCP project number."]
    #[serde(rename = "projectNumber")]
    pub project_number: String,
    #[doc = "The workload identity provider id that is used to gain access to the GCP project."]
    #[serde(rename = "workloadIdentityProviderId")]
    pub workload_identity_provider_id: String,
}
impl GcpAuthProperties {
    pub fn new(service_account_email: String, project_number: String, workload_identity_provider_id: String) -> Self {
        Self {
            service_account_email,
            project_number,
            workload_identity_provider_id,
        }
    }
}
#[doc = "Represents Google Cloud Platform data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcpDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "Google Cloud Platform data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GcpDataConnectorProperties>,
}
impl GcpDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "Google Cloud Platform data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcpDataConnectorProperties {
    #[doc = "The name of the connector definition that represents the UI config."]
    #[serde(rename = "connectorDefinitionName")]
    pub connector_definition_name: String,
    #[doc = "Google Cloud Platform auth section properties."]
    pub auth: GcpAuthProperties,
    #[doc = "Google Cloud Platform request section properties."]
    pub request: GcpRequestProperties,
    #[doc = "The configuration of the destination of the data."]
    #[serde(rename = "dcrConfig", default, skip_serializing_if = "Option::is_none")]
    pub dcr_config: Option<DcrConfiguration>,
}
impl GcpDataConnectorProperties {
    pub fn new(connector_definition_name: String, auth: GcpAuthProperties, request: GcpRequestProperties) -> Self {
        Self {
            connector_definition_name,
            auth,
            request,
            dcr_config: None,
        }
    }
}
#[doc = "Google Cloud Platform request section properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcpRequestProperties {
    #[doc = "The GCP project id."]
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[doc = "The GCP pub/sub subscription names."]
    #[serde(rename = "subscriptionNames")]
    pub subscription_names: Vec<String>,
}
impl GcpRequestProperties {
    pub fn new(project_id: String, subscription_names: Vec<String>) -> Self {
        Self {
            project_id,
            subscription_names,
        }
    }
}
#[doc = "GetInsights Query Errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetInsightsErrorKind {
    #[doc = "the query kind"]
    pub kind: get_insights_error_kind::Kind,
    #[doc = "the query id"]
    #[serde(rename = "queryId", default, skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[doc = "the error message"]
    #[serde(rename = "errorMessage")]
    pub error_message: String,
}
impl GetInsightsErrorKind {
    pub fn new(kind: get_insights_error_kind::Kind, error_message: String) -> Self {
        Self {
            kind,
            query_id: None,
            error_message,
        }
    }
}
pub mod get_insights_error_kind {
    use super::*;
    #[doc = "the query kind"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Insight,
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
                Self::Insight => serializer.serialize_unit_variant("Kind", 0u32, "Insight"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Get Insights result metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetInsightsResultsMetadata {
    #[doc = "the total items found for the insights request"]
    #[serde(rename = "totalCount")]
    pub total_count: i32,
    #[doc = "information about the failed queries"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<GetInsightsErrorKind>,
}
impl GetInsightsResultsMetadata {
    pub fn new(total_count: i32) -> Self {
        Self {
            total_count,
            errors: Vec::new(),
        }
    }
}
#[doc = "Retrieve queries for entity result operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetQueriesResponse {
    #[doc = "The query result values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EntityQueryItemUnion>,
}
impl GetQueriesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resources created in GitHub repository."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubResourceInfo {
    #[doc = "GitHub application installation id."]
    #[serde(rename = "appInstallationId", default, skip_serializing_if = "Option::is_none")]
    pub app_installation_id: Option<String>,
}
impl GitHubResourceInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The graph query to show the current data status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphQueries {
    #[doc = "the metric that the query is checking"]
    #[serde(rename = "metricName", default, skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[doc = "The legend for the graph"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legend: Option<String>,
    #[doc = "The base query for the graph"]
    #[serde(rename = "baseQuery", default, skip_serializing_if = "Option::is_none")]
    pub base_query: Option<String>,
}
impl GraphQueries {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Grouping configuration property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupingConfiguration {
    #[doc = "Grouping enabled"]
    pub enabled: bool,
    #[doc = "Re-open closed matching incidents"]
    #[serde(rename = "reopenClosedIncident")]
    pub reopen_closed_incident: bool,
    #[doc = "Limit the group to alerts created within the lookback duration (in ISO 8601 duration format)"]
    #[serde(rename = "lookbackDuration")]
    pub lookback_duration: String,
    #[doc = "Grouping matching method. When method is Selected at least one of groupByEntities, groupByAlertDetails, groupByCustomDetails must be provided and not empty."]
    #[serde(rename = "matchingMethod")]
    pub matching_method: grouping_configuration::MatchingMethod,
    #[doc = "A list of entity types to group by (when matchingMethod is Selected). Only entities defined in the current alert rule may be used."]
    #[serde(
        rename = "groupByEntities",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_by_entities: Vec<EntityMappingType>,
    #[doc = "A list of alert details to group by (when matchingMethod is Selected)"]
    #[serde(
        rename = "groupByAlertDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_by_alert_details: Vec<String>,
    #[doc = "A list of custom details keys to group by (when matchingMethod is Selected). Only keys defined in the current alert rule may be used."]
    #[serde(
        rename = "groupByCustomDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_by_custom_details: Vec<String>,
}
impl GroupingConfiguration {
    pub fn new(
        enabled: bool,
        reopen_closed_incident: bool,
        lookback_duration: String,
        matching_method: grouping_configuration::MatchingMethod,
    ) -> Self {
        Self {
            enabled,
            reopen_closed_incident,
            lookback_duration,
            matching_method,
            group_by_entities: Vec::new(),
            group_by_alert_details: Vec::new(),
            group_by_custom_details: Vec::new(),
        }
    }
}
pub mod grouping_configuration {
    use super::*;
    #[doc = "Grouping matching method. When method is Selected at least one of groupByEntities, groupByAlertDetails, groupByCustomDetails must be provided and not empty."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MatchingMethod")]
    pub enum MatchingMethod {
        AllEntities,
        AnyAlert,
        Selected,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MatchingMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MatchingMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MatchingMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AllEntities => serializer.serialize_unit_variant("MatchingMethod", 0u32, "AllEntities"),
                Self::AnyAlert => serializer.serialize_unit_variant("MatchingMethod", 1u32, "AnyAlert"),
                Self::Selected => serializer.serialize_unit_variant("MatchingMethod", 2u32, "Selected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents a Hunt in Azure Security Insights."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Hunt {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "Describes hunt properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HuntProperties>,
}
impl Hunt {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Hunt Comment in Azure Security Insights"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HuntComment {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "Describes a hunt comment properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HuntCommentProperties>,
}
impl HuntComment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of all hunt comments"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HuntCommentList {
    #[doc = "URL to fetch the next set of hunt comments."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of hunt comments"]
    pub value: Vec<HuntComment>,
}
impl azure_core::Continuable for HuntCommentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl HuntCommentList {
    pub fn new(value: Vec<HuntComment>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Describes a hunt comment properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HuntCommentProperties {
    #[doc = "The message for the comment"]
    pub message: String,
}
impl HuntCommentProperties {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
#[doc = "List all the hunts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HuntList {
    #[doc = "URL to fetch the next set of hunts."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of hunts."]
    pub value: Vec<Hunt>,
}
impl azure_core::Continuable for HuntList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl HuntList {
    pub fn new(value: Vec<Hunt>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Describes a user that the hunt is assigned to"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HuntOwner {
    #[doc = "The email of the user the hunt is assigned to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The name of the user the hunt is assigned to."]
    #[serde(rename = "assignedTo", default, skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    #[doc = "The object id of the user the hunt is assigned to."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "The user principal name of the user the hunt is assigned to."]
    #[serde(rename = "userPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[doc = "The type of the owner the hunt is assigned to."]
    #[serde(rename = "ownerType", default, skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<hunt_owner::OwnerType>,
}
impl HuntOwner {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hunt_owner {
    use super::*;
    #[doc = "The type of the owner the hunt is assigned to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OwnerType")]
    pub enum OwnerType {
        Unknown,
        User,
        Group,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OwnerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OwnerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OwnerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("OwnerType", 0u32, "Unknown"),
                Self::User => serializer.serialize_unit_variant("OwnerType", 1u32, "User"),
                Self::Group => serializer.serialize_unit_variant("OwnerType", 2u32, "Group"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes hunt properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HuntProperties {
    #[doc = "The display name of the hunt"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The description of the hunt"]
    pub description: String,
    #[doc = "The status of the hunt."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<hunt_properties::Status>,
    #[doc = "The hypothesis status of the hunt."]
    #[serde(rename = "hypothesisStatus", default, skip_serializing_if = "Option::is_none")]
    pub hypothesis_status: Option<hunt_properties::HypothesisStatus>,
    #[doc = "A list of mitre attack tactics the hunt is associated with"]
    #[serde(
        rename = "attackTactics",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub attack_tactics: Vec<AttackTactic>,
    #[doc = "A list of a mitre attack techniques the hunt is associated with"]
    #[serde(
        rename = "attackTechniques",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub attack_techniques: Vec<String>,
    #[doc = "List of labels relevant to this hunt "]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub labels: Vec<Label>,
    #[doc = "Describes a user that the hunt is assigned to"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<HuntOwner>,
}
impl HuntProperties {
    pub fn new(display_name: String, description: String) -> Self {
        Self {
            display_name,
            description,
            status: None,
            hypothesis_status: None,
            attack_tactics: Vec::new(),
            attack_techniques: Vec::new(),
            labels: Vec::new(),
            owner: None,
        }
    }
}
pub mod hunt_properties {
    use super::*;
    #[doc = "The status of the hunt."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        New,
        Active,
        Closed,
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
                Self::New => serializer.serialize_unit_variant("Status", 0u32, "New"),
                Self::Active => serializer.serialize_unit_variant("Status", 1u32, "Active"),
                Self::Closed => serializer.serialize_unit_variant("Status", 2u32, "Closed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::New
        }
    }
    #[doc = "The hypothesis status of the hunt."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HypothesisStatus")]
    pub enum HypothesisStatus {
        Unknown,
        Invalidated,
        Validated,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HypothesisStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HypothesisStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HypothesisStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("HypothesisStatus", 0u32, "Unknown"),
                Self::Invalidated => serializer.serialize_unit_variant("HypothesisStatus", 1u32, "Invalidated"),
                Self::Validated => serializer.serialize_unit_variant("HypothesisStatus", 2u32, "Validated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for HypothesisStatus {
        fn default() -> Self {
            Self::Unknown
        }
    }
}
#[doc = "Represents a Hunt Relation in Azure Security Insights."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HuntRelation {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "Describes hunt relation properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HuntRelationProperties>,
}
impl HuntRelation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of all the hunt relations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HuntRelationList {
    #[doc = "URL to fetch the next set of hunt relations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of hunt relations"]
    pub value: Vec<HuntRelation>,
}
impl azure_core::Continuable for HuntRelationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl HuntRelationList {
    pub fn new(value: Vec<HuntRelation>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Describes hunt relation properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HuntRelationProperties {
    #[doc = "The id of the related resource"]
    #[serde(rename = "relatedResourceId")]
    pub related_resource_id: String,
    #[doc = "The name of the related resource"]
    #[serde(rename = "relatedResourceName", default, skip_serializing_if = "Option::is_none")]
    pub related_resource_name: Option<String>,
    #[doc = "The type of the hunt relation"]
    #[serde(rename = "relationType", default, skip_serializing_if = "Option::is_none")]
    pub relation_type: Option<String>,
    #[doc = "The resource that the relation is related to"]
    #[serde(rename = "relatedResourceKind", default, skip_serializing_if = "Option::is_none")]
    pub related_resource_kind: Option<String>,
    #[doc = "List of labels relevant to this hunt"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub labels: Vec<Label>,
}
impl HuntRelationProperties {
    pub fn new(related_resource_id: String) -> Self {
        Self {
            related_resource_id,
            related_resource_name: None,
            relation_type: None,
            related_resource_kind: None,
            labels: Vec::new(),
        }
    }
}
#[doc = "Represents a Hunting bookmark entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HuntingBookmark {
    #[serde(flatten)]
    pub entity: Entity,
    #[doc = "Describes bookmark properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HuntingBookmarkProperties>,
}
impl HuntingBookmark {
    pub fn new(entity: Entity) -> Self {
        Self { entity, properties: None }
    }
}
#[doc = "Describes bookmark properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HuntingBookmarkProperties {
    #[serde(flatten)]
    pub entity_common_properties: EntityCommonProperties,
    #[doc = "The time the bookmark was created"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "User information that made some action"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserInfo>,
    #[doc = "The display name of the bookmark"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The time of the event"]
    #[serde(rename = "eventTime", default, with = "azure_core::date::rfc3339::option")]
    pub event_time: Option<time::OffsetDateTime>,
    #[doc = "List of labels relevant to this bookmark"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub labels: Vec<Label>,
    #[doc = "The notes of the bookmark"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[doc = "The query of the bookmark."]
    pub query: String,
    #[doc = "The query result of the bookmark."]
    #[serde(rename = "queryResult", default, skip_serializing_if = "Option::is_none")]
    pub query_result: Option<String>,
    #[doc = "The last time the bookmark was updated"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub updated: Option<time::OffsetDateTime>,
    #[doc = "User information that made some action"]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<UserInfo>,
    #[doc = "Describes related incident information for the bookmark"]
    #[serde(rename = "incidentInfo", default, skip_serializing_if = "Option::is_none")]
    pub incident_info: Option<IncidentInfo>,
}
impl HuntingBookmarkProperties {
    pub fn new(display_name: String, query: String) -> Self {
        Self {
            entity_common_properties: EntityCommonProperties::default(),
            created: None,
            created_by: None,
            display_name,
            event_time: None,
            labels: Vec::new(),
            notes: None,
            query,
            query_result: None,
            updated: None,
            updated_by: None,
            incident_info: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Incident {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IncidentProperties>,
}
impl Incident {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Incident additional data property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IncidentAdditionalData {
    #[doc = "The number of alerts in the incident"]
    #[serde(rename = "alertsCount", default, skip_serializing_if = "Option::is_none")]
    pub alerts_count: Option<i32>,
    #[doc = "The number of bookmarks in the incident"]
    #[serde(rename = "bookmarksCount", default, skip_serializing_if = "Option::is_none")]
    pub bookmarks_count: Option<i32>,
    #[doc = "The number of comments in the incident"]
    #[serde(rename = "commentsCount", default, skip_serializing_if = "Option::is_none")]
    pub comments_count: Option<i32>,
    #[doc = "List of product names of alerts in the incident"]
    #[serde(
        rename = "alertProductNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub alert_product_names: Vec<String>,
    #[doc = "The tactics associated with incident"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tactics: Vec<AttackTactic>,
    #[doc = "The techniques associated with incident's tactics"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub techniques: Vec<String>,
    #[doc = "The provider incident url to the incident in Microsoft 365 Defender portal"]
    #[serde(rename = "providerIncidentUrl", default, skip_serializing_if = "Option::is_none")]
    pub provider_incident_url: Option<String>,
}
impl IncidentAdditionalData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of incident alerts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentAlertList {
    #[doc = "Array of incident alerts."]
    pub value: Vec<SecurityAlert>,
}
impl IncidentAlertList {
    pub fn new(value: Vec<SecurityAlert>) -> Self {
        Self { value }
    }
}
#[doc = "List of incident bookmarks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentBookmarkList {
    #[doc = "Array of incident bookmarks."]
    pub value: Vec<HuntingBookmark>,
}
impl IncidentBookmarkList {
    pub fn new(value: Vec<HuntingBookmark>) -> Self {
        Self { value }
    }
}
#[doc = "The reason the incident was closed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IncidentClassificationEnum")]
pub enum IncidentClassificationEnum {
    Undetermined,
    TruePositive,
    BenignPositive,
    FalsePositive,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IncidentClassificationEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IncidentClassificationEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IncidentClassificationEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Undetermined => serializer.serialize_unit_variant("IncidentClassificationEnum", 0u32, "Undetermined"),
            Self::TruePositive => serializer.serialize_unit_variant("IncidentClassificationEnum", 1u32, "TruePositive"),
            Self::BenignPositive => serializer.serialize_unit_variant("IncidentClassificationEnum", 2u32, "BenignPositive"),
            Self::FalsePositive => serializer.serialize_unit_variant("IncidentClassificationEnum", 3u32, "FalsePositive"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The classification reason the incident was closed with"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IncidentClassificationReasonEnum")]
pub enum IncidentClassificationReasonEnum {
    SuspiciousActivity,
    SuspiciousButExpected,
    IncorrectAlertLogic,
    InaccurateData,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IncidentClassificationReasonEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IncidentClassificationReasonEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IncidentClassificationReasonEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SuspiciousActivity => serializer.serialize_unit_variant("IncidentClassificationReasonEnum", 0u32, "SuspiciousActivity"),
            Self::SuspiciousButExpected => {
                serializer.serialize_unit_variant("IncidentClassificationReasonEnum", 1u32, "SuspiciousButExpected")
            }
            Self::IncorrectAlertLogic => serializer.serialize_unit_variant("IncidentClassificationReasonEnum", 2u32, "IncorrectAlertLogic"),
            Self::InaccurateData => serializer.serialize_unit_variant("IncidentClassificationReasonEnum", 3u32, "InaccurateData"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents an incident comment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IncidentComment {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "Incident comment property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IncidentCommentProperties>,
}
impl IncidentComment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentCommentList {
    pub value: Vec<IncidentComment>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IncidentCommentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl IncidentCommentList {
    pub fn new(value: Vec<IncidentComment>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Incident comment property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentCommentProperties {
    #[doc = "The comment message"]
    pub message: String,
    #[doc = "The time the comment was created"]
    #[serde(rename = "createdTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub created_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The time the comment was updated"]
    #[serde(rename = "lastModifiedTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_time_utc: Option<time::OffsetDateTime>,
    #[doc = "Information on the client (user or application) that made some action"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<ClientInfo>,
}
impl IncidentCommentProperties {
    pub fn new(message: String) -> Self {
        Self {
            message,
            created_time_utc: None,
            last_modified_time_utc: None,
            author: None,
        }
    }
}
#[doc = "Incident Configuration property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentConfiguration {
    #[doc = "Create incidents from alerts triggered by this analytics rule"]
    #[serde(rename = "createIncident")]
    pub create_incident: bool,
    #[doc = "Grouping configuration property bag."]
    #[serde(rename = "groupingConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub grouping_configuration: Option<GroupingConfiguration>,
}
impl IncidentConfiguration {
    pub fn new(create_incident: bool) -> Self {
        Self {
            create_incident,
            grouping_configuration: None,
        }
    }
}
#[doc = "The incident related entities response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IncidentEntitiesResponse {
    #[doc = "Array of the incident related entities."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub entities: Vec<EntityUnion>,
    #[doc = "The metadata from the incident related entities results."]
    #[serde(
        rename = "metaData",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub meta_data: Vec<IncidentEntitiesResultsMetadata>,
}
impl IncidentEntitiesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information of a specific aggregation in the incident related entities result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentEntitiesResultsMetadata {
    #[doc = "The kind of the entity"]
    #[serde(rename = "entityKind")]
    pub entity_kind: EntityInnerKind,
    #[doc = "Total number of aggregations of the given kind in the incident related entities result."]
    pub count: i32,
}
impl IncidentEntitiesResultsMetadata {
    pub fn new(entity_kind: EntityInnerKind, count: i32) -> Self {
        Self { entity_kind, count }
    }
}
#[doc = "Describes related incident information for the bookmark"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IncidentInfo {
    #[doc = "Incident Id"]
    #[serde(rename = "incidentId", default, skip_serializing_if = "Option::is_none")]
    pub incident_id: Option<String>,
    #[doc = "The severity of the incident"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<IncidentSeverityEnum>,
    #[doc = "The title of the incident"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Relation Name"]
    #[serde(rename = "relationName", default, skip_serializing_if = "Option::is_none")]
    pub relation_name: Option<String>,
}
impl IncidentInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an incident label"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentLabel {
    #[doc = "The name of the label"]
    #[serde(rename = "labelName")]
    pub label_name: String,
    #[doc = "The type of the label"]
    #[serde(rename = "labelType", default, skip_serializing_if = "Option::is_none")]
    pub label_type: Option<IncidentLabelType>,
}
impl IncidentLabel {
    pub fn new(label_name: String) -> Self {
        Self {
            label_name,
            label_type: None,
        }
    }
}
#[doc = "The type of the label"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IncidentLabelType")]
pub enum IncidentLabelType {
    User,
    AutoAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IncidentLabelType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IncidentLabelType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IncidentLabelType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::User => serializer.serialize_unit_variant("IncidentLabelType", 0u32, "User"),
            Self::AutoAssigned => serializer.serialize_unit_variant("IncidentLabelType", 1u32, "AutoAssigned"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "List all the incidents."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentList {
    pub value: Vec<Incident>,
    #[doc = "URL to fetch the next set of incidents."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IncidentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl IncidentList {
    pub fn new(value: Vec<Incident>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Information on the user an incident is assigned to"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IncidentOwnerInfo {
    #[doc = "The email of the user the incident is assigned to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The name of the user the incident is assigned to."]
    #[serde(rename = "assignedTo", default, skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    #[doc = "The object id of the user the incident is assigned to."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "The user principal name of the user the incident is assigned to."]
    #[serde(rename = "userPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[doc = "The type of the owner the incident is assigned to."]
    #[serde(rename = "ownerType", default, skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<incident_owner_info::OwnerType>,
}
impl IncidentOwnerInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod incident_owner_info {
    use super::*;
    #[doc = "The type of the owner the incident is assigned to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OwnerType")]
    pub enum OwnerType {
        Unknown,
        User,
        Group,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OwnerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OwnerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OwnerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("OwnerType", 0u32, "Unknown"),
                Self::User => serializer.serialize_unit_variant("OwnerType", 1u32, "User"),
                Self::Group => serializer.serialize_unit_variant("OwnerType", 2u32, "Group"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentProperties {
    #[doc = "The title of the incident"]
    pub title: String,
    #[doc = "The description of the incident"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The severity of the incident"]
    pub severity: IncidentSeverityEnum,
    #[doc = "The status of the incident"]
    pub status: IncidentStatusEnum,
    #[doc = "The reason the incident was closed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<IncidentClassificationEnum>,
    #[doc = "The classification reason the incident was closed with"]
    #[serde(rename = "classificationReason", default, skip_serializing_if = "Option::is_none")]
    pub classification_reason: Option<IncidentClassificationReasonEnum>,
    #[doc = "Describes the reason the incident was closed"]
    #[serde(rename = "classificationComment", default, skip_serializing_if = "Option::is_none")]
    pub classification_comment: Option<String>,
    #[doc = "Information on the user an incident is assigned to"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<IncidentOwnerInfo>,
    #[doc = "List of labels relevant to this incident"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub labels: Vec<IncidentLabel>,
    #[doc = "The time of the first activity in the incident"]
    #[serde(rename = "firstActivityTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub first_activity_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The time of the last activity in the incident"]
    #[serde(rename = "lastActivityTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_activity_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The last time the incident was updated"]
    #[serde(rename = "lastModifiedTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The time the incident was created"]
    #[serde(rename = "createdTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub created_time_utc: Option<time::OffsetDateTime>,
    #[doc = "A sequential number"]
    #[serde(rename = "incidentNumber", default, skip_serializing_if = "Option::is_none")]
    pub incident_number: Option<i32>,
    #[doc = "Incident additional data property bag."]
    #[serde(rename = "additionalData", default, skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<IncidentAdditionalData>,
    #[doc = "List of resource ids of Analytic rules related to the incident"]
    #[serde(
        rename = "relatedAnalyticRuleIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub related_analytic_rule_ids: Vec<String>,
    #[doc = "The deep-link url to the incident in Azure portal"]
    #[serde(rename = "incidentUrl", default, skip_serializing_if = "Option::is_none")]
    pub incident_url: Option<String>,
    #[doc = "The name of the source provider that generated the incident"]
    #[serde(rename = "providerName", default, skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[doc = "The incident ID assigned by the incident provider"]
    #[serde(rename = "providerIncidentId", default, skip_serializing_if = "Option::is_none")]
    pub provider_incident_id: Option<String>,
    #[doc = "Describes team information"]
    #[serde(rename = "teamInformation", default, skip_serializing_if = "Option::is_none")]
    pub team_information: Option<TeamInformation>,
}
impl IncidentProperties {
    pub fn new(title: String, severity: IncidentSeverityEnum, status: IncidentStatusEnum) -> Self {
        Self {
            title,
            description: None,
            severity,
            status,
            classification: None,
            classification_reason: None,
            classification_comment: None,
            owner: None,
            labels: Vec::new(),
            first_activity_time_utc: None,
            last_activity_time_utc: None,
            last_modified_time_utc: None,
            created_time_utc: None,
            incident_number: None,
            additional_data: None,
            related_analytic_rule_ids: Vec::new(),
            incident_url: None,
            provider_name: None,
            provider_incident_id: None,
            team_information: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IncidentPropertiesAction {
    #[doc = "The severity of the incident"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<IncidentSeverityEnum>,
    #[doc = "The status of the incident"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<IncidentStatusEnum>,
    #[doc = "The reason the incident was closed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<IncidentClassificationEnum>,
    #[doc = "The classification reason the incident was closed with"]
    #[serde(rename = "classificationReason", default, skip_serializing_if = "Option::is_none")]
    pub classification_reason: Option<IncidentClassificationReasonEnum>,
    #[doc = "Describes the reason the incident was closed."]
    #[serde(rename = "classificationComment", default, skip_serializing_if = "Option::is_none")]
    pub classification_comment: Option<String>,
    #[doc = "Information on the user an incident is assigned to"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<IncidentOwnerInfo>,
    #[doc = "List of labels to add to the incident."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub labels: Vec<IncidentLabel>,
}
impl IncidentPropertiesAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The severity of the incident"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IncidentSeverityEnum")]
pub enum IncidentSeverityEnum {
    High,
    Medium,
    Low,
    Informational,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IncidentSeverityEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IncidentSeverityEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IncidentSeverityEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::High => serializer.serialize_unit_variant("IncidentSeverityEnum", 0u32, "High"),
            Self::Medium => serializer.serialize_unit_variant("IncidentSeverityEnum", 1u32, "Medium"),
            Self::Low => serializer.serialize_unit_variant("IncidentSeverityEnum", 2u32, "Low"),
            Self::Informational => serializer.serialize_unit_variant("IncidentSeverityEnum", 3u32, "Informational"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The status of the incident"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IncidentStatusEnum")]
pub enum IncidentStatusEnum {
    New,
    Active,
    Closed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IncidentStatusEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IncidentStatusEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IncidentStatusEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::New => serializer.serialize_unit_variant("IncidentStatusEnum", 0u32, "New"),
            Self::Active => serializer.serialize_unit_variant("IncidentStatusEnum", 1u32, "Active"),
            Self::Closed => serializer.serialize_unit_variant("IncidentStatusEnum", 2u32, "Closed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTask {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    pub properties: IncidentTaskProperties,
}
impl IncidentTask {
    pub fn new(properties: IncidentTaskProperties) -> Self {
        Self {
            resource_with_etag: ResourceWithEtag::default(),
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IncidentTaskList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<IncidentTask>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IncidentTaskList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl IncidentTaskList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTaskProperties {
    #[doc = "The title of the task"]
    pub title: String,
    #[doc = "The description of the task"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub status: IncidentTaskStatus,
    #[doc = "The time the task was created"]
    #[serde(rename = "createdTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub created_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The last time the task was updated"]
    #[serde(rename = "lastModifiedTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_time_utc: Option<time::OffsetDateTime>,
    #[doc = "Information on the client (user or application) that made some action"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<ClientInfo>,
    #[doc = "Information on the client (user or application) that made some action"]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<ClientInfo>,
}
impl IncidentTaskProperties {
    pub fn new(title: String, status: IncidentTaskStatus) -> Self {
        Self {
            title,
            description: None,
            status,
            created_time_utc: None,
            last_modified_time_utc: None,
            created_by: None,
            last_modified_by: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IncidentTaskStatus")]
pub enum IncidentTaskStatus {
    New,
    Completed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IncidentTaskStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IncidentTaskStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IncidentTaskStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::New => serializer.serialize_unit_variant("IncidentTaskStatus", 0u32, "New"),
            Self::Completed => serializer.serialize_unit_variant("IncidentTaskStatus", 1u32, "Completed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents Insight Query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InsightQueryItem {
    #[serde(flatten)]
    pub entity_query_item: EntityQueryItem,
    #[doc = "Represents Insight Query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InsightQueryItemProperties>,
}
impl InsightQueryItem {
    pub fn new(entity_query_item: EntityQueryItem) -> Self {
        Self {
            entity_query_item,
            properties: None,
        }
    }
}
#[doc = "Represents Insight Query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InsightQueryItemProperties {
    #[serde(flatten)]
    pub entity_query_item_properties: EntityQueryItemProperties,
    #[doc = "The insight display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The insight description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The base query of the insight."]
    #[serde(rename = "baseQuery", default, skip_serializing_if = "Option::is_none")]
    pub base_query: Option<String>,
    #[doc = "The insight table query."]
    #[serde(rename = "tableQuery", default, skip_serializing_if = "Option::is_none")]
    pub table_query: Option<insight_query_item_properties::TableQuery>,
    #[doc = "The insight chart query."]
    #[serde(rename = "chartQuery", default, skip_serializing_if = "Option::is_none")]
    pub chart_query: Option<serde_json::Value>,
    #[doc = "The activity query definitions."]
    #[serde(rename = "additionalQuery", default, skip_serializing_if = "Option::is_none")]
    pub additional_query: Option<insight_query_item_properties::AdditionalQuery>,
    #[doc = "The insight chart query."]
    #[serde(rename = "defaultTimeRange", default, skip_serializing_if = "Option::is_none")]
    pub default_time_range: Option<insight_query_item_properties::DefaultTimeRange>,
    #[doc = "The insight chart query."]
    #[serde(rename = "referenceTimeRange", default, skip_serializing_if = "Option::is_none")]
    pub reference_time_range: Option<insight_query_item_properties::ReferenceTimeRange>,
}
impl InsightQueryItemProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod insight_query_item_properties {
    use super::*;
    #[doc = "The insight table query."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct TableQuery {
        #[doc = "List of insight column definitions."]
        #[serde(
            rename = "columnsDefinitions",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub columns_definitions: Vec<serde_json::Value>,
        #[doc = "List of insight queries definitions."]
        #[serde(
            rename = "queriesDefinitions",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub queries_definitions: Vec<serde_json::Value>,
    }
    impl TableQuery {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The activity query definitions."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct AdditionalQuery {
        #[doc = "The insight query."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub query: Option<String>,
        #[doc = "The insight text."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub text: Option<String>,
    }
    impl AdditionalQuery {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The insight chart query."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DefaultTimeRange {
        #[doc = "The padding for the start time of the query."]
        #[serde(rename = "beforeRange", default, skip_serializing_if = "Option::is_none")]
        pub before_range: Option<String>,
        #[doc = "The padding for the end time of the query."]
        #[serde(rename = "afterRange", default, skip_serializing_if = "Option::is_none")]
        pub after_range: Option<String>,
    }
    impl DefaultTimeRange {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The insight chart query."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ReferenceTimeRange {
        #[doc = "Additional query time for looking back."]
        #[serde(rename = "beforeRange", default, skip_serializing_if = "Option::is_none")]
        pub before_range: Option<String>,
    }
    impl ReferenceTimeRange {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Query results for table insights query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InsightsTableResult {
    #[doc = "Columns Metadata of the table"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub columns: Vec<serde_json::Value>,
    #[doc = "Rows data of the table"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rows: Vec<Vec<String>>,
}
impl InsightsTableResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Instruction steps to enable the connector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstructionSteps {
    #[doc = "Instruction step title"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Instruction step description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Instruction step details"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub instructions: Vec<serde_json::Value>,
}
impl InstructionSteps {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Instructions section of a recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Instructions {
    #[doc = "What actions should be taken to complete the recommendation."]
    #[serde(rename = "actionsToBePerformed")]
    pub actions_to_be_performed: String,
    #[doc = "Explains why the recommendation is important."]
    #[serde(rename = "recommendationImportance")]
    pub recommendation_importance: String,
    #[doc = "How should the user complete the recommendation."]
    #[serde(rename = "howToPerformActionDetails", default, skip_serializing_if = "Option::is_none")]
    pub how_to_perform_action_details: Option<String>,
}
impl Instructions {
    pub fn new(actions_to_be_performed: String, recommendation_importance: String) -> Self {
        Self {
            actions_to_be_performed,
            recommendation_importance,
            how_to_perform_action_details: None,
        }
    }
}
#[doc = "Represents IoT requirements check request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTCheckRequirements {
    #[doc = "IoT requirements check properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTCheckRequirementsProperties>,
}
impl IoTCheckRequirements {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "IoT requirements check properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTCheckRequirementsProperties {
    #[doc = "The subscription id to connect to, and get the data from."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}
impl IoTCheckRequirementsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents IoT data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "IoT data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTDataConnectorProperties>,
}
impl IoTDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "IoT data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_with_alerts_properties: DataConnectorWithAlertsProperties,
    #[doc = "The subscription id to connect to, and get the data from."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}
impl IoTDataConnectorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The assignment job"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Job {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "The job properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobProperties>,
}
impl Job {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of all the jobs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobList {
    #[doc = "URL to fetch the next set of jobs."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of jobs."]
    pub value: Vec<Job>,
}
impl azure_core::Continuable for JobList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl JobList {
    pub fn new(value: Vec<Job>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "The job properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobProperties {
    #[doc = "The time the job completed"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "List of items published by the job"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<JobItem>,
    #[doc = "State of the job"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<job_properties::ProvisioningState>,
    #[doc = "The time the job started"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Message to describe error, if an error exists"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl JobProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_properties {
    use super::*;
    #[doc = "State of the job"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        InProgress,
        Canceled,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 1u32, "InProgress"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type Label = String;
#[doc = "Data type for last data received"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LastDataReceivedDataType {
    #[doc = "Name of the data type to show in the graph. can be use with {{graphQueriesTableName}} placeholder"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Query for indicate last data received"]
    #[serde(rename = "lastDataReceivedQuery", default, skip_serializing_if = "Option::is_none")]
    pub last_data_received_query: Option<String>,
}
impl LastDataReceivedDataType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents MCAS (Microsoft Cloud App Security) requirements check request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct McasCheckRequirements {
    #[doc = "MCAS (Microsoft Cloud App Security) requirements check properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<McasCheckRequirementsProperties>,
}
impl McasCheckRequirements {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "MCAS (Microsoft Cloud App Security) requirements check properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct McasCheckRequirementsProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
}
impl McasCheckRequirementsProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self { data_connector_tenant_id }
    }
}
#[doc = "Represents MCAS (Microsoft Cloud App Security) data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct McasDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "MCAS (Microsoft Cloud App Security) data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<McasDataConnectorProperties>,
}
impl McasDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "The available data types for MCAS (Microsoft Cloud App Security) data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct McasDataConnectorDataTypes {
    #[serde(flatten)]
    pub alerts_data_type_of_data_connector: AlertsDataTypeOfDataConnector,
    #[doc = "Common field for data type in data connectors."]
    #[serde(rename = "discoveryLogs", default, skip_serializing_if = "Option::is_none")]
    pub discovery_logs: Option<DataConnectorDataTypeCommon>,
}
impl McasDataConnectorDataTypes {
    pub fn new(alerts_data_type_of_data_connector: AlertsDataTypeOfDataConnector) -> Self {
        Self {
            alerts_data_type_of_data_connector,
            discovery_logs: None,
        }
    }
}
#[doc = "MCAS (Microsoft Cloud App Security) data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct McasDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[doc = "The available data types for MCAS (Microsoft Cloud App Security) data connector."]
    #[serde(rename = "dataTypes")]
    pub data_types: McasDataConnectorDataTypes,
}
impl McasDataConnectorProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId, data_types: McasDataConnectorDataTypes) -> Self {
        Self {
            data_connector_tenant_id,
            data_types,
        }
    }
}
#[doc = "Represents MDATP (Microsoft Defender Advanced Threat Protection) requirements check request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MdatpCheckRequirements {
    #[doc = "MDATP (Microsoft Defender Advanced Threat Protection) requirements check properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MdatpCheckRequirementsProperties>,
}
impl MdatpCheckRequirements {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "MDATP (Microsoft Defender Advanced Threat Protection) requirements check properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MdatpCheckRequirementsProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
}
impl MdatpCheckRequirementsProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self { data_connector_tenant_id }
    }
}
#[doc = "Represents MDATP (Microsoft Defender Advanced Threat Protection) data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MdatpDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "MDATP (Microsoft Defender Advanced Threat Protection) data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MdatpDataConnectorProperties>,
}
impl MdatpDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "MDATP (Microsoft Defender Advanced Threat Protection) data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MdatpDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[serde(flatten)]
    pub data_connector_with_alerts_properties: DataConnectorWithAlertsProperties,
}
impl MdatpDataConnectorProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self {
            data_connector_tenant_id,
            data_connector_with_alerts_properties: DataConnectorWithAlertsProperties::default(),
        }
    }
}
#[doc = "Represents MLBehaviorAnalytics alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MlBehaviorAnalyticsAlertRule {
    #[serde(flatten)]
    pub alert_rule: AlertRule,
    #[doc = "MLBehaviorAnalytics alert rule base property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MlBehaviorAnalyticsAlertRuleProperties>,
}
impl MlBehaviorAnalyticsAlertRule {
    pub fn new(alert_rule: AlertRule) -> Self {
        Self {
            alert_rule,
            properties: None,
        }
    }
}
#[doc = "MLBehaviorAnalytics alert rule base property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MlBehaviorAnalyticsAlertRuleProperties {
    #[doc = "The Name of the alert rule template used to create this rule."]
    #[serde(rename = "alertRuleTemplateName")]
    pub alert_rule_template_name: String,
    #[doc = "The description of the alert rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display name for alerts created by this alert rule."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Determines whether this alert rule is enabled or disabled."]
    pub enabled: bool,
    #[doc = "The last time that this alert rule has been modified."]
    #[serde(rename = "lastModifiedUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_utc: Option<time::OffsetDateTime>,
    #[doc = "The severity of the alert"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<AlertSeverityEnum>,
    #[doc = "The tactics of the alert rule"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tactics: Vec<AttackTactic>,
    #[doc = "The techniques of the alert rule"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub techniques: Vec<String>,
}
impl MlBehaviorAnalyticsAlertRuleProperties {
    pub fn new(alert_rule_template_name: String, enabled: bool) -> Self {
        Self {
            alert_rule_template_name,
            description: None,
            display_name: None,
            enabled,
            last_modified_utc: None,
            severity: None,
            tactics: Vec::new(),
            techniques: Vec::new(),
        }
    }
}
#[doc = "Represents MLBehaviorAnalytics alert rule template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MlBehaviorAnalyticsAlertRuleTemplate {
    #[serde(flatten)]
    pub alert_rule_template: AlertRuleTemplate,
    #[doc = "MLBehaviorAnalytics alert rule template properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ml_behavior_analytics_alert_rule_template::Properties>,
}
impl MlBehaviorAnalyticsAlertRuleTemplate {
    pub fn new(alert_rule_template: AlertRuleTemplate) -> Self {
        Self {
            alert_rule_template,
            properties: None,
        }
    }
}
pub mod ml_behavior_analytics_alert_rule_template {
    use super::*;
    #[doc = "MLBehaviorAnalytics alert rule template properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(flatten)]
        pub alert_rule_template_with_mitre_properties: AlertRuleTemplateWithMitreProperties,
        #[doc = "The severity of the alert"]
        pub severity: AlertSeverityEnum,
    }
    impl Properties {
        pub fn new(severity: AlertSeverityEnum) -> Self {
            Self {
                alert_rule_template_with_mitre_properties: AlertRuleTemplateWithMitreProperties::default(),
                severity,
            }
        }
    }
}
#[doc = "Represents Microsoft Threat Intelligence requirements check request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MstiCheckRequirements {
    #[doc = "Microsoft Threat Intelligence requirements check properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MstiCheckRequirementsProperties>,
}
impl MstiCheckRequirements {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "Microsoft Threat Intelligence requirements check properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MstiCheckRequirementsProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
}
impl MstiCheckRequirementsProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self { data_connector_tenant_id }
    }
}
#[doc = "Represents Microsoft Threat Intelligence data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MstiDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "Microsoft Threat Intelligence data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MstiDataConnectorProperties>,
}
impl MstiDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "The available data types for Microsoft Threat Intelligence Platforms data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MstiDataConnectorDataTypes {
    #[doc = "Data type for Microsoft Threat Intelligence Platforms data connector."]
    #[serde(rename = "microsoftEmergingThreatFeed")]
    pub microsoft_emerging_threat_feed: msti_data_connector_data_types::MicrosoftEmergingThreatFeed,
}
impl MstiDataConnectorDataTypes {
    pub fn new(microsoft_emerging_threat_feed: msti_data_connector_data_types::MicrosoftEmergingThreatFeed) -> Self {
        Self {
            microsoft_emerging_threat_feed,
        }
    }
}
pub mod msti_data_connector_data_types {
    use super::*;
    #[doc = "Data type for Microsoft Threat Intelligence Platforms data connector."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct MicrosoftEmergingThreatFeed {
        #[serde(flatten)]
        pub data_connector_data_type_common: DataConnectorDataTypeCommon,
        #[doc = "The lookback period for the feed to be imported."]
        #[serde(rename = "lookbackPeriod")]
        pub lookback_period: String,
    }
    impl MicrosoftEmergingThreatFeed {
        pub fn new(data_connector_data_type_common: DataConnectorDataTypeCommon, lookback_period: String) -> Self {
            Self {
                data_connector_data_type_common,
                lookback_period,
            }
        }
    }
}
#[doc = "Microsoft Threat Intelligence data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MstiDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[doc = "The available data types for Microsoft Threat Intelligence Platforms data connector."]
    #[serde(rename = "dataTypes")]
    pub data_types: MstiDataConnectorDataTypes,
}
impl MstiDataConnectorProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId, data_types: MstiDataConnectorDataTypes) -> Self {
        Self {
            data_connector_tenant_id,
            data_types,
        }
    }
}
#[doc = "MTP (Microsoft Threat Protection) requirements check properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MtpCheckRequirementsProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
}
impl MtpCheckRequirementsProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self { data_connector_tenant_id }
    }
}
#[doc = "Represents MTP (Microsoft Threat Protection) data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MtpDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "MTP (Microsoft Threat Protection) data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MtpDataConnectorProperties>,
}
impl MtpDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "The available data types for Microsoft Threat Protection Platforms data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MtpDataConnectorDataTypes {
    #[doc = "Incidents data type for Microsoft Threat Protection Platforms data connector."]
    pub incidents: serde_json::Value,
    #[doc = "Alerts data type for Microsoft Threat Protection Platforms data connector."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alerts: Option<serde_json::Value>,
}
impl MtpDataConnectorDataTypes {
    pub fn new(incidents: serde_json::Value) -> Self {
        Self { incidents, alerts: None }
    }
}
#[doc = "MTP (Microsoft Threat Protection) data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MtpDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[doc = "The available data types for Microsoft Threat Protection Platforms data connector."]
    #[serde(rename = "dataTypes")]
    pub data_types: MtpDataConnectorDataTypes,
    #[doc = "Represents the connector's Filtered providers"]
    #[serde(rename = "filteredProviders", default, skip_serializing_if = "Option::is_none")]
    pub filtered_providers: Option<MtpFilteredProviders>,
}
impl MtpDataConnectorProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId, data_types: MtpDataConnectorDataTypes) -> Self {
        Self {
            data_connector_tenant_id,
            data_types,
            filtered_providers: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManualTriggerRequestBody {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "logicAppsResourceId")]
    pub logic_apps_resource_id: String,
}
impl ManualTriggerRequestBody {
    pub fn new(logic_apps_resource_id: String) -> Self {
        Self {
            tenant_id: None,
            logic_apps_resource_id,
        }
    }
}
#[doc = "List of all the metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataList {
    #[doc = "Array of metadata."]
    pub value: Vec<MetadataModel>,
    #[doc = "URL to fetch the next page of metadata."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MetadataList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl MetadataList {
    pub fn new(value: Vec<MetadataModel>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Metadata resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataModel {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "Metadata property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetadataProperties>,
}
impl MetadataModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents MicrosoftPurviewInformationProtection requirements check request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftPurviewInformationProtectionCheckRequirements {
    #[doc = "MicrosoftPurviewInformationProtection requirements check properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MicrosoftPurviewInformationProtectionCheckRequirementsProperties>,
}
impl MicrosoftPurviewInformationProtectionCheckRequirements {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "MicrosoftPurviewInformationProtection requirements check properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftPurviewInformationProtectionCheckRequirementsProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
}
impl MicrosoftPurviewInformationProtectionCheckRequirementsProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self { data_connector_tenant_id }
    }
}
#[doc = "The available data types for Microsoft Purview Information Protection data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftPurviewInformationProtectionConnectorDataTypes {
    #[doc = "Logs data type."]
    pub logs: serde_json::Value,
}
impl MicrosoftPurviewInformationProtectionConnectorDataTypes {
    pub fn new(logs: serde_json::Value) -> Self {
        Self { logs }
    }
}
#[doc = "Represents Microsoft Purview Information Protection data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftPurviewInformationProtectionDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "Microsoft Purview Information Protection data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MicrosoftPurviewInformationProtectionDataConnectorProperties>,
}
impl MicrosoftPurviewInformationProtectionDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "Microsoft Purview Information Protection data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftPurviewInformationProtectionDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[doc = "The available data types for Microsoft Purview Information Protection data connector."]
    #[serde(rename = "dataTypes")]
    pub data_types: MicrosoftPurviewInformationProtectionConnectorDataTypes,
}
impl MicrosoftPurviewInformationProtectionDataConnectorProperties {
    pub fn new(
        data_connector_tenant_id: DataConnectorTenantId,
        data_types: MicrosoftPurviewInformationProtectionConnectorDataTypes,
    ) -> Self {
        Self {
            data_connector_tenant_id,
            data_types,
        }
    }
}
#[doc = "Represents MicrosoftSecurityIncidentCreation rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftSecurityIncidentCreationAlertRule {
    #[serde(flatten)]
    pub alert_rule: AlertRule,
    #[doc = "MicrosoftSecurityIncidentCreation rule property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MicrosoftSecurityIncidentCreationAlertRuleProperties>,
}
impl MicrosoftSecurityIncidentCreationAlertRule {
    pub fn new(alert_rule: AlertRule) -> Self {
        Self {
            alert_rule,
            properties: None,
        }
    }
}
#[doc = "MicrosoftSecurityIncidentCreation rule common property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftSecurityIncidentCreationAlertRuleCommonProperties {
    #[doc = "the alerts' displayNames on which the cases will be generated"]
    #[serde(
        rename = "displayNamesFilter",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub display_names_filter: Vec<String>,
    #[doc = "the alerts' displayNames on which the cases will not be generated"]
    #[serde(
        rename = "displayNamesExcludeFilter",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub display_names_exclude_filter: Vec<String>,
    #[doc = "The alerts' productName on which the cases will be generated"]
    #[serde(rename = "productFilter")]
    pub product_filter: MicrosoftSecurityProductName,
    #[doc = "the alerts' severities on which the cases will be generated"]
    #[serde(
        rename = "severitiesFilter",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub severities_filter: Vec<AlertSeverityEnum>,
}
impl MicrosoftSecurityIncidentCreationAlertRuleCommonProperties {
    pub fn new(product_filter: MicrosoftSecurityProductName) -> Self {
        Self {
            display_names_filter: Vec::new(),
            display_names_exclude_filter: Vec::new(),
            product_filter,
            severities_filter: Vec::new(),
        }
    }
}
#[doc = "MicrosoftSecurityIncidentCreation rule property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftSecurityIncidentCreationAlertRuleProperties {
    #[serde(flatten)]
    pub microsoft_security_incident_creation_alert_rule_common_properties: MicrosoftSecurityIncidentCreationAlertRuleCommonProperties,
    #[doc = "The Name of the alert rule template used to create this rule."]
    #[serde(rename = "alertRuleTemplateName", default, skip_serializing_if = "Option::is_none")]
    pub alert_rule_template_name: Option<String>,
    #[doc = "The description of the alert rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display name for alerts created by this alert rule."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Determines whether this alert rule is enabled or disabled."]
    pub enabled: bool,
    #[doc = "The last time that this alert has been modified."]
    #[serde(rename = "lastModifiedUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_utc: Option<time::OffsetDateTime>,
}
impl MicrosoftSecurityIncidentCreationAlertRuleProperties {
    pub fn new(
        microsoft_security_incident_creation_alert_rule_common_properties: MicrosoftSecurityIncidentCreationAlertRuleCommonProperties,
        display_name: String,
        enabled: bool,
    ) -> Self {
        Self {
            microsoft_security_incident_creation_alert_rule_common_properties,
            alert_rule_template_name: None,
            description: None,
            display_name,
            enabled,
            last_modified_utc: None,
        }
    }
}
#[doc = "Represents MicrosoftSecurityIncidentCreation rule template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftSecurityIncidentCreationAlertRuleTemplate {
    #[serde(flatten)]
    pub alert_rule_template: AlertRuleTemplate,
    #[doc = "MicrosoftSecurityIncidentCreation rule template properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MicrosoftSecurityIncidentCreationAlertRuleTemplateProperties>,
}
impl MicrosoftSecurityIncidentCreationAlertRuleTemplate {
    pub fn new(alert_rule_template: AlertRuleTemplate) -> Self {
        Self {
            alert_rule_template,
            properties: None,
        }
    }
}
#[doc = "MicrosoftSecurityIncidentCreation rule template properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MicrosoftSecurityIncidentCreationAlertRuleTemplateProperties {
    #[serde(flatten)]
    pub alert_rule_template_properties_base: AlertRuleTemplatePropertiesBase,
    #[doc = "the alerts' displayNames on which the cases will be generated"]
    #[serde(
        rename = "displayNamesFilter",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub display_names_filter: Vec<String>,
    #[doc = "the alerts' displayNames on which the cases will not be generated"]
    #[serde(
        rename = "displayNamesExcludeFilter",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub display_names_exclude_filter: Vec<String>,
    #[doc = "The alerts' productName on which the cases will be generated"]
    #[serde(rename = "productFilter", default, skip_serializing_if = "Option::is_none")]
    pub product_filter: Option<MicrosoftSecurityProductName>,
    #[doc = "the alerts' severities on which the cases will be generated"]
    #[serde(
        rename = "severitiesFilter",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub severities_filter: Vec<AlertSeverityEnum>,
}
impl MicrosoftSecurityIncidentCreationAlertRuleTemplateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The alerts' productName on which the cases will be generated"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MicrosoftSecurityProductName")]
pub enum MicrosoftSecurityProductName {
    #[serde(rename = "Microsoft Cloud App Security")]
    MicrosoftCloudAppSecurity,
    #[serde(rename = "Azure Security Center")]
    AzureSecurityCenter,
    #[serde(rename = "Azure Advanced Threat Protection")]
    AzureAdvancedThreatProtection,
    #[serde(rename = "Azure Active Directory Identity Protection")]
    AzureActiveDirectoryIdentityProtection,
    #[serde(rename = "Azure Security Center for IoT")]
    AzureSecurityCenterForIoT,
    #[serde(rename = "Office 365 Advanced Threat Protection")]
    Office365AdvancedThreatProtection,
    #[serde(rename = "Microsoft Defender Advanced Threat Protection")]
    MicrosoftDefenderAdvancedThreatProtection,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MicrosoftSecurityProductName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MicrosoftSecurityProductName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MicrosoftSecurityProductName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::MicrosoftCloudAppSecurity => {
                serializer.serialize_unit_variant("MicrosoftSecurityProductName", 0u32, "Microsoft Cloud App Security")
            }
            Self::AzureSecurityCenter => serializer.serialize_unit_variant("MicrosoftSecurityProductName", 1u32, "Azure Security Center"),
            Self::AzureAdvancedThreatProtection => {
                serializer.serialize_unit_variant("MicrosoftSecurityProductName", 2u32, "Azure Advanced Threat Protection")
            }
            Self::AzureActiveDirectoryIdentityProtection => {
                serializer.serialize_unit_variant("MicrosoftSecurityProductName", 3u32, "Azure Active Directory Identity Protection")
            }
            Self::AzureSecurityCenterForIoT => {
                serializer.serialize_unit_variant("MicrosoftSecurityProductName", 4u32, "Azure Security Center for IoT")
            }
            Self::Office365AdvancedThreatProtection => {
                serializer.serialize_unit_variant("MicrosoftSecurityProductName", 5u32, "Office 365 Advanced Threat Protection")
            }
            Self::MicrosoftDefenderAdvancedThreatProtection => serializer.serialize_unit_variant(
                "MicrosoftSecurityProductName",
                6u32,
                "Microsoft Defender Advanced Threat Protection",
            ),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents MTP (Microsoft Threat Protection) requirements check request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MtpCheckRequirements {
    #[doc = "MTP (Microsoft Threat Protection) requirements check properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MtpCheckRequirementsProperties>,
}
impl MtpCheckRequirements {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "Represents the connector's Filtered providers"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MtpFilteredProviders {
    #[doc = "Alerts filtered providers. When filters are not applied, all alerts will stream through the MTP pipeline, still in private preview for all products EXCEPT MDA and MDI, which are in GA state."]
    pub alerts: Vec<MtpProvider>,
}
impl MtpFilteredProviders {
    pub fn new(alerts: Vec<MtpProvider>) -> Self {
        Self { alerts }
    }
}
#[doc = "The available data providers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MtpProvider")]
pub enum MtpProvider {
    #[serde(rename = "microsoftDefenderForCloudApps")]
    MicrosoftDefenderForCloudApps,
    #[serde(rename = "microsoftDefenderForIdentity")]
    MicrosoftDefenderForIdentity,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MtpProvider {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MtpProvider {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MtpProvider {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::MicrosoftDefenderForCloudApps => serializer.serialize_unit_variant("MtpProvider", 0u32, "microsoftDefenderForCloudApps"),
            Self::MicrosoftDefenderForIdentity => serializer.serialize_unit_variant("MtpProvider", 1u32, "microsoftDefenderForIdentity"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents NRT alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NrtAlertRule {
    #[serde(flatten)]
    pub alert_rule: AlertRule,
    #[doc = "Nrt alert rule base property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NrtAlertRuleProperties>,
}
impl NrtAlertRule {
    pub fn new(alert_rule: AlertRule) -> Self {
        Self {
            alert_rule,
            properties: None,
        }
    }
}
#[doc = "Nrt alert rule base property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NrtAlertRuleProperties {
    #[doc = "The Name of the alert rule template used to create this rule."]
    #[serde(rename = "alertRuleTemplateName", default, skip_serializing_if = "Option::is_none")]
    pub alert_rule_template_name: Option<String>,
    #[doc = "The version of the alert rule template used to create this rule - in format <a.b.c>, where all are numbers, for example 0 <1.0.2>"]
    #[serde(rename = "templateVersion", default, skip_serializing_if = "Option::is_none")]
    pub template_version: Option<String>,
    #[doc = "The description of the alert rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The query that creates alerts for this rule."]
    pub query: String,
    #[doc = "The tactics of the alert rule"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tactics: Vec<AttackTactic>,
    #[doc = "The techniques of the alert rule"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub techniques: Vec<String>,
    #[doc = "The display name for alerts created by this alert rule."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Determines whether this alert rule is enabled or disabled."]
    pub enabled: bool,
    #[doc = "The last time that this alert rule has been modified."]
    #[serde(rename = "lastModifiedUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_utc: Option<time::OffsetDateTime>,
    #[doc = "The suppression (in ISO 8601 duration format) to wait since last time this alert rule been triggered."]
    #[serde(rename = "suppressionDuration")]
    pub suppression_duration: String,
    #[doc = "Determines whether the suppression for this alert rule is enabled or disabled."]
    #[serde(rename = "suppressionEnabled")]
    pub suppression_enabled: bool,
    #[doc = "The severity of the alert"]
    pub severity: AlertSeverityEnum,
    #[doc = "Incident Configuration property bag."]
    #[serde(rename = "incidentConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub incident_configuration: Option<IncidentConfiguration>,
    #[doc = "Dictionary of string key-value pairs of columns to be attached to the alert"]
    #[serde(rename = "customDetails", default, skip_serializing_if = "Option::is_none")]
    pub custom_details: Option<serde_json::Value>,
    #[doc = "List of entity mappings of the alert rule"]
    #[serde(rename = "entityMappings", default, skip_serializing_if = "Option::is_none")]
    pub entity_mappings: Option<EntityMappings>,
    #[doc = "Settings for how to dynamically override alert static details"]
    #[serde(rename = "alertDetailsOverride", default, skip_serializing_if = "Option::is_none")]
    pub alert_details_override: Option<AlertDetailsOverride>,
    #[doc = "Event grouping settings property bag."]
    #[serde(rename = "eventGroupingSettings", default, skip_serializing_if = "Option::is_none")]
    pub event_grouping_settings: Option<EventGroupingSettings>,
    #[doc = "List of sentinel entity mappings of the alert rule"]
    #[serde(rename = "sentinelEntitiesMappings", default, skip_serializing_if = "Option::is_none")]
    pub sentinel_entities_mappings: Option<SentinelEntitiesMappings>,
}
impl NrtAlertRuleProperties {
    pub fn new(
        query: String,
        display_name: String,
        enabled: bool,
        suppression_duration: String,
        suppression_enabled: bool,
        severity: AlertSeverityEnum,
    ) -> Self {
        Self {
            alert_rule_template_name: None,
            template_version: None,
            description: None,
            query,
            tactics: Vec::new(),
            techniques: Vec::new(),
            display_name,
            enabled,
            last_modified_utc: None,
            suppression_duration,
            suppression_enabled,
            severity,
            incident_configuration: None,
            custom_details: None,
            entity_mappings: None,
            alert_details_override: None,
            event_grouping_settings: None,
            sentinel_entities_mappings: None,
        }
    }
}
#[doc = "Represents NRT alert rule template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NrtAlertRuleTemplate {
    #[serde(flatten)]
    pub alert_rule_template: AlertRuleTemplate,
    #[doc = "NRT alert rule template properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl NrtAlertRuleTemplate {
    pub fn new(alert_rule_template: AlertRuleTemplate) -> Self {
        Self {
            alert_rule_template,
            properties: None,
        }
    }
}
#[doc = "Represents Office365 Project requirements check request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Office365ProjectCheckRequirements {
    #[doc = "Office365 Project requirements check properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Office365ProjectCheckRequirementsProperties>,
}
impl Office365ProjectCheckRequirements {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "Office365 Project requirements check properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Office365ProjectCheckRequirementsProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
}
impl Office365ProjectCheckRequirementsProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self { data_connector_tenant_id }
    }
}
#[doc = "The available data types for Office Microsoft Project data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Office365ProjectConnectorDataTypes {
    #[doc = "Logs data type."]
    pub logs: serde_json::Value,
}
impl Office365ProjectConnectorDataTypes {
    pub fn new(logs: serde_json::Value) -> Self {
        Self { logs }
    }
}
#[doc = "Represents Office Microsoft Project data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Office365ProjectDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "Office Microsoft Project data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Office365ProjectDataConnectorProperties>,
}
impl Office365ProjectDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "Office Microsoft Project data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Office365ProjectDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[doc = "The available data types for Office Microsoft Project data connector."]
    #[serde(rename = "dataTypes")]
    pub data_types: Office365ProjectConnectorDataTypes,
}
impl Office365ProjectDataConnectorProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId, data_types: Office365ProjectConnectorDataTypes) -> Self {
        Self {
            data_connector_tenant_id,
            data_types,
        }
    }
}
#[doc = "Represents OfficeATP (Office 365 Advanced Threat Protection) requirements check request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeAtpCheckRequirements {
    #[doc = "OfficeATP (Office 365 Advanced Threat Protection) requirements check properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OfficeAtpCheckRequirementsProperties>,
}
impl OfficeAtpCheckRequirements {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "OfficeATP (Office 365 Advanced Threat Protection) requirements check properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeAtpCheckRequirementsProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
}
impl OfficeAtpCheckRequirementsProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self { data_connector_tenant_id }
    }
}
#[doc = "Represents OfficeATP (Office 365 Advanced Threat Protection) data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeAtpDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "OfficeATP (Office 365 Advanced Threat Protection) data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OfficeAtpDataConnectorProperties>,
}
impl OfficeAtpDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "OfficeATP (Office 365 Advanced Threat Protection) data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeAtpDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[serde(flatten)]
    pub data_connector_with_alerts_properties: DataConnectorWithAlertsProperties,
}
impl OfficeAtpDataConnectorProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self {
            data_connector_tenant_id,
            data_connector_with_alerts_properties: DataConnectorWithAlertsProperties::default(),
        }
    }
}
#[doc = "Consent for Office365 tenant that already made."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OfficeConsent {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Consent property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OfficeConsentProperties>,
}
impl OfficeConsent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of all the office365 consents."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeConsentList {
    #[doc = "URL to fetch the next set of office consents."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of the consents."]
    pub value: Vec<OfficeConsent>,
}
impl azure_core::Continuable for OfficeConsentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OfficeConsentList {
    pub fn new(value: Vec<OfficeConsent>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Consent property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OfficeConsentProperties {
    #[doc = "The tenantId of the Office365 with the consent."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Help to easily cascade among the data layers."]
    #[serde(rename = "consentId", default, skip_serializing_if = "Option::is_none")]
    pub consent_id: Option<String>,
}
impl OfficeConsentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents office data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "Office data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OfficeDataConnectorProperties>,
}
impl OfficeDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "The available data types for office data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeDataConnectorDataTypes {
    #[doc = "Exchange data type connection."]
    pub exchange: serde_json::Value,
    #[doc = "SharePoint data type connection."]
    #[serde(rename = "sharePoint")]
    pub share_point: serde_json::Value,
    #[doc = "Teams data type connection."]
    pub teams: serde_json::Value,
}
impl OfficeDataConnectorDataTypes {
    pub fn new(exchange: serde_json::Value, share_point: serde_json::Value, teams: serde_json::Value) -> Self {
        Self {
            exchange,
            share_point,
            teams,
        }
    }
}
#[doc = "Office data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[doc = "The available data types for office data connector."]
    #[serde(rename = "dataTypes")]
    pub data_types: OfficeDataConnectorDataTypes,
}
impl OfficeDataConnectorProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId, data_types: OfficeDataConnectorDataTypes) -> Self {
        Self {
            data_connector_tenant_id,
            data_types,
        }
    }
}
#[doc = "Represents OfficeIRM (Microsoft Insider Risk Management) requirements check request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeIrmCheckRequirements {
    #[doc = "OfficeIRM (Microsoft Insider Risk Management) requirements check properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OfficeIrmCheckRequirementsProperties>,
}
impl OfficeIrmCheckRequirements {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "OfficeIRM (Microsoft Insider Risk Management) requirements check properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeIrmCheckRequirementsProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
}
impl OfficeIrmCheckRequirementsProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self { data_connector_tenant_id }
    }
}
#[doc = "Represents OfficeIRM (Microsoft Insider Risk Management) data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeIrmDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "OfficeIRM (Microsoft Insider Risk Management) data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OfficeIrmDataConnectorProperties>,
}
impl OfficeIrmDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "OfficeIRM (Microsoft Insider Risk Management) data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeIrmDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[serde(flatten)]
    pub data_connector_with_alerts_properties: DataConnectorWithAlertsProperties,
}
impl OfficeIrmDataConnectorProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self {
            data_connector_tenant_id,
            data_connector_with_alerts_properties: DataConnectorWithAlertsProperties::default(),
        }
    }
}
#[doc = "Represents Office PowerBI requirements check request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficePowerBiCheckRequirements {
    #[doc = "Office PowerBI requirements check properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OfficePowerBiCheckRequirementsProperties>,
}
impl OfficePowerBiCheckRequirements {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "Office PowerBI requirements check properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficePowerBiCheckRequirementsProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
}
impl OfficePowerBiCheckRequirementsProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self { data_connector_tenant_id }
    }
}
#[doc = "The available data types for Office Microsoft PowerBI data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficePowerBiConnectorDataTypes {
    #[doc = "Logs data type."]
    pub logs: serde_json::Value,
}
impl OfficePowerBiConnectorDataTypes {
    pub fn new(logs: serde_json::Value) -> Self {
        Self { logs }
    }
}
#[doc = "Represents Office Microsoft PowerBI data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficePowerBiDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "Office Microsoft PowerBI data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OfficePowerBiDataConnectorProperties>,
}
impl OfficePowerBiDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "Office Microsoft PowerBI data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficePowerBiDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[doc = "The available data types for Office Microsoft PowerBI data connector."]
    #[serde(rename = "dataTypes")]
    pub data_types: OfficePowerBiConnectorDataTypes,
}
impl OfficePowerBiDataConnectorProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId, data_types: OfficePowerBiConnectorDataTypes) -> Self {
        Self {
            data_connector_tenant_id,
            data_types,
        }
    }
}
#[doc = "Operation provided by provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Properties of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Properties of the operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Description of the operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "Operation name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Provider name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Lists the operations available in the SecurityInsights RP."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsList {
    #[doc = "URL to fetch the next set of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of operations"]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationsList {
    pub fn new(value: Vec<Operation>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Permissions required for the connector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Permissions {
    #[doc = "Resource provider permissions required for the connector"]
    #[serde(
        rename = "resourceProvider",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resource_provider: Vec<serde_json::Value>,
    #[doc = "Customs permissions required for the connector"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub customs: Vec<serde_json::Value>,
}
impl Permissions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlaybookActionProperties {
    #[doc = "The resource id of the playbook resource."]
    #[serde(rename = "logicAppResourceId")]
    pub logic_app_resource_id: String,
    #[doc = "The tenant id of the playbook resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl PlaybookActionProperties {
    pub fn new(logic_app_resource_id: String) -> Self {
        Self {
            logic_app_resource_id,
            tenant_id: None,
        }
    }
}
#[doc = "Describes an automation rule condition that evaluates an array property's value change"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyArrayChangedConditionProperties {
    #[serde(rename = "conditionProperties", default, skip_serializing_if = "Option::is_none")]
    pub condition_properties: Option<AutomationRulePropertyArrayChangedValuesCondition>,
}
impl PropertyArrayChangedConditionProperties {
    pub fn new() -> Self {
        Self {
            condition_properties: None,
        }
    }
}
#[doc = "Describes an automation rule condition that evaluates an array property's value"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyArrayConditionProperties {
    #[serde(rename = "conditionProperties", default, skip_serializing_if = "Option::is_none")]
    pub condition_properties: Option<AutomationRulePropertyArrayValuesCondition>,
}
impl PropertyArrayConditionProperties {
    pub fn new() -> Self {
        Self {
            condition_properties: None,
        }
    }
}
#[doc = "Describes an automation rule condition that evaluates a property's value change"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyChangedConditionProperties {
    #[serde(rename = "conditionProperties", default, skip_serializing_if = "Option::is_none")]
    pub condition_properties: Option<AutomationRulePropertyValuesChangedCondition>,
}
impl PropertyChangedConditionProperties {
    pub fn new() -> Self {
        Self {
            condition_properties: None,
        }
    }
}
#[doc = "Describes an automation rule condition that evaluates a property's value"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyConditionProperties {
    #[serde(rename = "conditionProperties", default, skip_serializing_if = "Option::is_none")]
    pub condition_properties: Option<AutomationRulePropertyValuesCondition>,
}
impl PropertyConditionProperties {
    pub fn new() -> Self {
        Self {
            condition_properties: None,
        }
    }
}
#[doc = "The triggered analytics rule run provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Accepted,
    InProgress,
    Succeeded,
    Failed,
    Canceled,
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
            Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 1u32, "InProgress"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Information regarding pull request for protected branches."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PullRequest {
    #[doc = "URL of pull request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Status of the pull request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<PullRequestState>,
}
impl PullRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status of the pull request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PullRequestState")]
pub enum PullRequestState {
    Open,
    Closed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PullRequestState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PullRequestState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PullRequestState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Open => serializer.serialize_unit_variant("PullRequestState", 0u32, "Open"),
            Self::Closed => serializer.serialize_unit_variant("PullRequestState", 1u32, "Closed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Query based alert rule template base property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryBasedAlertRuleTemplateProperties {
    #[doc = "The query that creates alerts for this rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[doc = "The severity of the alert"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<AlertSeverityEnum>,
    #[doc = "The version of this template - in format <a.b.c>, where all are numbers. For example <1.0.2>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Dictionary of string key-value pairs of columns to be attached to the alert"]
    #[serde(rename = "customDetails", default, skip_serializing_if = "Option::is_none")]
    pub custom_details: Option<serde_json::Value>,
    #[doc = "List of entity mappings of the alert rule"]
    #[serde(rename = "entityMappings", default, skip_serializing_if = "Option::is_none")]
    pub entity_mappings: Option<EntityMappings>,
    #[doc = "Settings for how to dynamically override alert static details"]
    #[serde(rename = "alertDetailsOverride", default, skip_serializing_if = "Option::is_none")]
    pub alert_details_override: Option<AlertDetailsOverride>,
    #[doc = "Event grouping settings property bag."]
    #[serde(rename = "eventGroupingSettings", default, skip_serializing_if = "Option::is_none")]
    pub event_grouping_settings: Option<EventGroupingSettings>,
    #[doc = "List of sentinel entity mappings of the alert rule"]
    #[serde(rename = "sentinelEntitiesMappings", default, skip_serializing_if = "Option::is_none")]
    pub sentinel_entities_mappings: Option<SentinelEntitiesMappings>,
}
impl QueryBasedAlertRuleTemplateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recommendation object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Recommendation {
    #[doc = "id of recommendation."]
    pub id: String,
    #[doc = "Instructions section of a recommendation."]
    pub instructions: Instructions,
    #[doc = "Content section of the recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
    #[doc = "Id of the resource this recommendation refers to."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Collection of additional properties for the recommendation."]
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
    #[doc = "Title of the recommendation."]
    pub title: String,
    #[doc = "Description of the recommendation."]
    pub description: String,
    #[doc = "Title of the recommendation type."]
    #[serde(rename = "recommendationTypeTitle")]
    pub recommendation_type_title: String,
    #[doc = "Id of the recommendation type."]
    #[serde(rename = "recommendationTypeId")]
    pub recommendation_type_id: String,
    #[doc = "Categories of recommendations."]
    pub category: RecommendationCategory,
    #[doc = "Context of recommendation."]
    pub context: RecommendationContext,
    #[doc = "Id of the workspace this recommendation refers to."]
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[doc = "List of actions to take for this recommendation."]
    pub actions: Vec<RecommendedAction>,
    #[doc = "State of recommendation."]
    pub state: RecommendationState,
    #[doc = "Priority of recommendation."]
    pub priority: RecommendationPriority,
    #[doc = "The time stamp (UTC) when the recommendation was last evaluated."]
    #[serde(rename = "lastEvaluatedTimeUtc", with = "azure_core::date::rfc3339")]
    pub last_evaluated_time_utc: time::OffsetDateTime,
    #[doc = "The time stamp (UTC) when the recommendation should be displayed again."]
    #[serde(rename = "hideUntilTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub hide_until_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The timestamp (UTC) after which the recommendation should not be displayed anymore."]
    #[serde(rename = "displayUntilTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub display_until_time_utc: Option<time::OffsetDateTime>,
    #[doc = "Value indicating if the recommendation should be displayed or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}
impl Recommendation {
    pub fn new(
        id: String,
        instructions: Instructions,
        title: String,
        description: String,
        recommendation_type_title: String,
        recommendation_type_id: String,
        category: RecommendationCategory,
        context: RecommendationContext,
        workspace_id: String,
        actions: Vec<RecommendedAction>,
        state: RecommendationState,
        priority: RecommendationPriority,
        last_evaluated_time_utc: time::OffsetDateTime,
    ) -> Self {
        Self {
            id,
            instructions,
            content: None,
            resource_id: None,
            additional_properties: None,
            title,
            description,
            recommendation_type_title,
            recommendation_type_id,
            category,
            context,
            workspace_id,
            actions,
            state,
            priority,
            last_evaluated_time_utc,
            hide_until_time_utc: None,
            display_until_time_utc: None,
            visible: None,
        }
    }
}
#[doc = "Categories of recommendations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RecommendationCategory")]
pub enum RecommendationCategory {
    Onboarding,
    NewFeature,
    SocEfficiency,
    CostOptimization,
    Demo,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RecommendationCategory {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RecommendationCategory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RecommendationCategory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Onboarding => serializer.serialize_unit_variant("RecommendationCategory", 0u32, "Onboarding"),
            Self::NewFeature => serializer.serialize_unit_variant("RecommendationCategory", 1u32, "NewFeature"),
            Self::SocEfficiency => serializer.serialize_unit_variant("RecommendationCategory", 2u32, "SocEfficiency"),
            Self::CostOptimization => serializer.serialize_unit_variant("RecommendationCategory", 3u32, "CostOptimization"),
            Self::Demo => serializer.serialize_unit_variant("RecommendationCategory", 4u32, "Demo"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Context of recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RecommendationContext")]
pub enum RecommendationContext {
    Analytics,
    Incidents,
    Overview,
    None,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RecommendationContext {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RecommendationContext {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RecommendationContext {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Analytics => serializer.serialize_unit_variant("RecommendationContext", 0u32, "Analytics"),
            Self::Incidents => serializer.serialize_unit_variant("RecommendationContext", 1u32, "Incidents"),
            Self::Overview => serializer.serialize_unit_variant("RecommendationContext", 2u32, "Overview"),
            Self::None => serializer.serialize_unit_variant("RecommendationContext", 3u32, "None"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A list of recommendations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendationList {
    #[doc = "An list of recommendations"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Recommendation>,
}
impl RecommendationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recommendation Fields to update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendationPatch {
    #[doc = "State of recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<RecommendationState>,
    #[doc = "The time stamp (UTC) when the recommendation should be displayed again."]
    #[serde(rename = "hideUntilTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub hide_until_time_utc: Option<time::OffsetDateTime>,
}
impl RecommendationPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Priority of recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RecommendationPriority")]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RecommendationPriority {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RecommendationPriority {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RecommendationPriority {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Low => serializer.serialize_unit_variant("RecommendationPriority", 0u32, "Low"),
            Self::Medium => serializer.serialize_unit_variant("RecommendationPriority", 1u32, "Medium"),
            Self::High => serializer.serialize_unit_variant("RecommendationPriority", 2u32, "High"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "State of recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RecommendationState")]
pub enum RecommendationState {
    Active,
    Disabled,
    CompletedByUser,
    CompletedByAction,
    Hidden,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RecommendationState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RecommendationState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RecommendationState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("RecommendationState", 0u32, "Active"),
            Self::Disabled => serializer.serialize_unit_variant("RecommendationState", 1u32, "Disabled"),
            Self::CompletedByUser => serializer.serialize_unit_variant("RecommendationState", 2u32, "CompletedByUser"),
            Self::CompletedByAction => serializer.serialize_unit_variant("RecommendationState", 3u32, "CompletedByAction"),
            Self::Hidden => serializer.serialize_unit_variant("RecommendationState", 4u32, "Hidden"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "What actions should be taken to complete the recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendedAction {
    #[doc = "Text of the link to complete the action."]
    #[serde(rename = "linkText")]
    pub link_text: String,
    #[doc = "The Link to complete the action."]
    #[serde(rename = "linkUrl")]
    pub link_url: String,
    #[doc = "Represents the state the recommendation action is currently in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ActionState>,
}
impl RecommendedAction {
    pub fn new(link_text: String, link_url: String) -> Self {
        Self {
            link_text,
            link_url,
            state: None,
        }
    }
}
#[doc = "Represents a relation between two resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Relation {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "Relation property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RelationProperties>,
}
impl Relation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of relations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationList {
    #[doc = "URL to fetch the next set of relations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of relations."]
    pub value: Vec<Relation>,
}
impl azure_core::Continuable for RelationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RelationList {
    pub fn new(value: Vec<Relation>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Relation property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationProperties {
    #[doc = "The resource ID of the related resource"]
    #[serde(rename = "relatedResourceId")]
    pub related_resource_id: String,
    #[doc = "The name of the related resource"]
    #[serde(rename = "relatedResourceName", default, skip_serializing_if = "Option::is_none")]
    pub related_resource_name: Option<String>,
    #[doc = "The resource type of the related resource"]
    #[serde(rename = "relatedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub related_resource_type: Option<String>,
    #[doc = "The resource kind of the related resource"]
    #[serde(rename = "relatedResourceKind", default, skip_serializing_if = "Option::is_none")]
    pub related_resource_kind: Option<String>,
}
impl RelationProperties {
    pub fn new(related_resource_id: String) -> Self {
        Self {
            related_resource_id,
            related_resource_name: None,
            related_resource_type: None,
            related_resource_kind: None,
        }
    }
}
#[doc = "Represents a repository."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Repo {
    #[doc = "The url to access the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The name of the repository."]
    #[serde(rename = "fullName", default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[doc = "Array of branches."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub branches: Vec<String>,
}
impl Repo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List all the source controls."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepoList {
    #[doc = "URL to fetch the next set of repositories."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of repositories."]
    pub value: Vec<Repo>,
}
impl azure_core::Continuable for RepoList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RepoList {
    pub fn new(value: Vec<Repo>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "The type of repository."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RepoType")]
pub enum RepoType {
    Github,
    AzureDevOps,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RepoType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RepoType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RepoType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Github => serializer.serialize_unit_variant("RepoType", 0u32, "Github"),
            Self::AzureDevOps => serializer.serialize_unit_variant("RepoType", 1u32, "AzureDevOps"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "metadata of a repository."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Repository {
    #[doc = "Url of repository."]
    pub url: String,
    #[doc = "Branch name of repository."]
    pub branch: String,
    #[doc = "Display url of repository."]
    #[serde(rename = "displayUrl", default, skip_serializing_if = "Option::is_none")]
    pub display_url: Option<String>,
    #[doc = "Url to access repository action logs."]
    #[serde(rename = "deploymentLogsUrl", default, skip_serializing_if = "Option::is_none")]
    pub deployment_logs_url: Option<String>,
}
impl Repository {
    pub fn new(url: String, branch: String) -> Self {
        Self {
            url,
            branch,
            display_url: None,
            deployment_logs_url: None,
        }
    }
}
#[doc = "Credentials to access repository."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryAccess {
    #[doc = "The kind of repository access credentials"]
    pub kind: RepositoryAccessKind,
    #[doc = "OAuth Code. Required when `kind` is `OAuth`"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "OAuth State. Required when `kind` is `OAuth`"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "OAuth ClientId. Required when `kind` is `OAuth`"]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "Personal Access Token. Required when `kind` is `PAT`"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[doc = "Application installation ID. Required when `kind` is `App`. Supported by `GitHub` only."]
    #[serde(rename = "installationId", default, skip_serializing_if = "Option::is_none")]
    pub installation_id: Option<String>,
}
impl RepositoryAccess {
    pub fn new(kind: RepositoryAccessKind) -> Self {
        Self {
            kind,
            code: None,
            state: None,
            client_id: None,
            token: None,
            installation_id: None,
        }
    }
}
#[doc = "The kind of repository access credentials"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RepositoryAccessKind")]
pub enum RepositoryAccessKind {
    OAuth,
    #[serde(rename = "PAT")]
    Pat,
    App,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RepositoryAccessKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RepositoryAccessKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RepositoryAccessKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::OAuth => serializer.serialize_unit_variant("RepositoryAccessKind", 0u32, "OAuth"),
            Self::Pat => serializer.serialize_unit_variant("RepositoryAccessKind", 1u32, "PAT"),
            Self::App => serializer.serialize_unit_variant("RepositoryAccessKind", 2u32, "App"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Resources created in user's repository for the source-control."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RepositoryResourceInfo {
    #[doc = "Detail about the webhook object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Webhook>,
    #[doc = "Resources created in GitHub repository."]
    #[serde(rename = "gitHubResourceInfo", default, skip_serializing_if = "Option::is_none")]
    pub git_hub_resource_info: Option<GitHubResourceInfo>,
    #[doc = "Resources created in Azure DevOps repository."]
    #[serde(rename = "azureDevOpsResourceInfo", default, skip_serializing_if = "Option::is_none")]
    pub azure_dev_ops_resource_info: Option<AzureDevOpsResourceInfo>,
}
impl RepositoryResourceInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Required permissions for the connector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequiredPermissions {
    #[doc = "action permission"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<bool>,
    #[doc = "write permission"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write: Option<bool>,
    #[doc = "read permission"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[doc = "delete permission"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
}
impl RequiredPermissions {
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource provider permissions required for the connector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProvider {
    #[doc = "Provider name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<resource_provider::Provider>,
    #[doc = "Permission description text"]
    #[serde(rename = "permissionsDisplayText", default, skip_serializing_if = "Option::is_none")]
    pub permissions_display_text: Option<String>,
    #[doc = "Permission provider display name"]
    #[serde(rename = "providerDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub provider_display_name: Option<String>,
    #[doc = "Permission provider scope"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<resource_provider::Scope>,
    #[doc = "Required permissions for the connector"]
    #[serde(rename = "requiredPermissions", default, skip_serializing_if = "Option::is_none")]
    pub required_permissions: Option<RequiredPermissions>,
}
impl ResourceProvider {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_provider {
    use super::*;
    #[doc = "Provider name"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Provider")]
    pub enum Provider {
        #[serde(rename = "Microsoft.OperationalInsights/solutions")]
        MicrosoftOperationalInsightsSolutions,
        #[serde(rename = "Microsoft.OperationalInsights/workspaces")]
        MicrosoftOperationalInsightsWorkspaces,
        #[serde(rename = "Microsoft.OperationalInsights/workspaces/datasources")]
        MicrosoftOperationalInsightsWorkspacesDatasources,
        #[serde(rename = "microsoft.aadiam/diagnosticSettings")]
        MicrosoftAadiamDiagnosticSettings,
        #[serde(rename = "Microsoft.OperationalInsights/workspaces/sharedKeys")]
        MicrosoftOperationalInsightsWorkspacesSharedKeys,
        #[serde(rename = "Microsoft.Authorization/policyAssignments")]
        MicrosoftAuthorizationPolicyAssignments,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Provider {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Provider {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Provider {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MicrosoftOperationalInsightsSolutions => {
                    serializer.serialize_unit_variant("Provider", 0u32, "Microsoft.OperationalInsights/solutions")
                }
                Self::MicrosoftOperationalInsightsWorkspaces => {
                    serializer.serialize_unit_variant("Provider", 1u32, "Microsoft.OperationalInsights/workspaces")
                }
                Self::MicrosoftOperationalInsightsWorkspacesDatasources => {
                    serializer.serialize_unit_variant("Provider", 2u32, "Microsoft.OperationalInsights/workspaces/datasources")
                }
                Self::MicrosoftAadiamDiagnosticSettings => {
                    serializer.serialize_unit_variant("Provider", 3u32, "microsoft.aadiam/diagnosticSettings")
                }
                Self::MicrosoftOperationalInsightsWorkspacesSharedKeys => {
                    serializer.serialize_unit_variant("Provider", 4u32, "Microsoft.OperationalInsights/workspaces/sharedKeys")
                }
                Self::MicrosoftAuthorizationPolicyAssignments => {
                    serializer.serialize_unit_variant("Provider", 5u32, "Microsoft.Authorization/policyAssignments")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Permission provider scope"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Scope")]
    pub enum Scope {
        ResourceGroup,
        Subscription,
        Workspace,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Scope {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Scope {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Scope {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ResourceGroup => serializer.serialize_unit_variant("Scope", 0u32, "ResourceGroup"),
                Self::Subscription => serializer.serialize_unit_variant("Scope", 1u32, "Subscription"),
                Self::Workspace => serializer.serialize_unit_variant("Scope", 2u32, "Workspace"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An azure resource object with an Etag property"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceWithEtag {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Etag of the azure resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ResourceWithEtag {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The sample queries for the connector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SampleQueries {
    #[doc = "The sample query description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "the sample query"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}
impl SampleQueries {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Billing statistic about the Microsoft Sentinel solution for SAP Usage"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapSolutionUsageStatistic {
    #[serde(flatten)]
    pub billing_statistic: BillingStatistic,
    #[doc = "Properties of the billing statistic about the Microsoft Sentinel solution for SAP usage"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SapSolutionUsageStatisticProperties>,
}
impl SapSolutionUsageStatistic {
    pub fn new(billing_statistic: BillingStatistic) -> Self {
        Self {
            billing_statistic,
            properties: None,
        }
    }
}
#[doc = "Properties of the billing statistic about the Microsoft Sentinel solution for SAP usage"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapSolutionUsageStatisticProperties {
    #[doc = "The latest count of active SAP system IDs under the Microsoft Sentinel solution for SAP Usage"]
    #[serde(rename = "activeSystemIdCount", default, skip_serializing_if = "Option::is_none")]
    pub active_system_id_count: Option<i64>,
}
impl SapSolutionUsageStatisticProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents scheduled alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledAlertRule {
    #[serde(flatten)]
    pub alert_rule: AlertRule,
    #[doc = "Scheduled alert rule base property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduledAlertRuleProperties>,
}
impl ScheduledAlertRule {
    pub fn new(alert_rule: AlertRule) -> Self {
        Self {
            alert_rule,
            properties: None,
        }
    }
}
#[doc = "Scheduled alert rule template property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduledAlertRuleCommonProperties {
    #[doc = "The query that creates alerts for this rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[doc = "The frequency (in ISO 8601 duration format) for this alert rule to run."]
    #[serde(rename = "queryFrequency", default, skip_serializing_if = "Option::is_none")]
    pub query_frequency: Option<String>,
    #[doc = "The period (in ISO 8601 duration format) that this alert rule looks at."]
    #[serde(rename = "queryPeriod", default, skip_serializing_if = "Option::is_none")]
    pub query_period: Option<String>,
    #[doc = "The severity of the alert"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<AlertSeverityEnum>,
    #[doc = "The operation against the threshold that triggers alert rule."]
    #[serde(rename = "triggerOperator", default, skip_serializing_if = "Option::is_none")]
    pub trigger_operator: Option<AlertRuleTriggerOperator>,
    #[doc = "The threshold triggers this alert rule."]
    #[serde(rename = "triggerThreshold", default, skip_serializing_if = "Option::is_none")]
    pub trigger_threshold: Option<i32>,
    #[doc = "Event grouping settings property bag."]
    #[serde(rename = "eventGroupingSettings", default, skip_serializing_if = "Option::is_none")]
    pub event_grouping_settings: Option<EventGroupingSettings>,
    #[doc = "Dictionary of string key-value pairs of columns to be attached to the alert"]
    #[serde(rename = "customDetails", default, skip_serializing_if = "Option::is_none")]
    pub custom_details: Option<serde_json::Value>,
    #[doc = "List of entity mappings of the alert rule"]
    #[serde(rename = "entityMappings", default, skip_serializing_if = "Option::is_none")]
    pub entity_mappings: Option<EntityMappings>,
    #[doc = "Settings for how to dynamically override alert static details"]
    #[serde(rename = "alertDetailsOverride", default, skip_serializing_if = "Option::is_none")]
    pub alert_details_override: Option<AlertDetailsOverride>,
    #[doc = "List of sentinel entity mappings of the alert rule"]
    #[serde(rename = "sentinelEntitiesMappings", default, skip_serializing_if = "Option::is_none")]
    pub sentinel_entities_mappings: Option<SentinelEntitiesMappings>,
}
impl ScheduledAlertRuleCommonProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Scheduled alert rule base property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledAlertRuleProperties {
    #[serde(flatten)]
    pub scheduled_alert_rule_common_properties: ScheduledAlertRuleCommonProperties,
    #[doc = "The Name of the alert rule template used to create this rule."]
    #[serde(rename = "alertRuleTemplateName", default, skip_serializing_if = "Option::is_none")]
    pub alert_rule_template_name: Option<String>,
    #[doc = "The version of the alert rule template used to create this rule - in format <a.b.c>, where all are numbers, for example 0 <1.0.2>"]
    #[serde(rename = "templateVersion", default, skip_serializing_if = "Option::is_none")]
    pub template_version: Option<String>,
    #[doc = "The description of the alert rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display name for alerts created by this alert rule."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Determines whether this alert rule is enabled or disabled."]
    pub enabled: bool,
    #[doc = "The last time that this alert rule has been modified."]
    #[serde(rename = "lastModifiedUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_utc: Option<time::OffsetDateTime>,
    #[doc = "The suppression (in ISO 8601 duration format) to wait since last time this alert rule been triggered."]
    #[serde(rename = "suppressionDuration")]
    pub suppression_duration: String,
    #[doc = "Determines whether the suppression for this alert rule is enabled or disabled."]
    #[serde(rename = "suppressionEnabled")]
    pub suppression_enabled: bool,
    #[doc = "The tactics of the alert rule"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tactics: Vec<AttackTactic>,
    #[doc = "The techniques of the alert rule"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub techniques: Vec<String>,
    #[doc = "Incident Configuration property bag."]
    #[serde(rename = "incidentConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub incident_configuration: Option<IncidentConfiguration>,
}
impl ScheduledAlertRuleProperties {
    pub fn new(display_name: String, enabled: bool, suppression_duration: String, suppression_enabled: bool) -> Self {
        Self {
            scheduled_alert_rule_common_properties: ScheduledAlertRuleCommonProperties::default(),
            alert_rule_template_name: None,
            template_version: None,
            description: None,
            display_name,
            enabled,
            last_modified_utc: None,
            suppression_duration,
            suppression_enabled,
            tactics: Vec::new(),
            techniques: Vec::new(),
            incident_configuration: None,
        }
    }
}
#[doc = "Represents scheduled alert rule template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledAlertRuleTemplate {
    #[serde(flatten)]
    pub alert_rule_template: AlertRuleTemplate,
    #[doc = "Scheduled alert rule template properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduledAlertRuleTemplateProperties>,
}
impl ScheduledAlertRuleTemplate {
    pub fn new(alert_rule_template: AlertRuleTemplate) -> Self {
        Self {
            alert_rule_template,
            properties: None,
        }
    }
}
#[doc = "Scheduled alert rule template properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduledAlertRuleTemplateProperties {
    #[doc = "the number of alert rules that were created by this template"]
    #[serde(rename = "alertRulesCreatedByTemplateCount", default, skip_serializing_if = "Option::is_none")]
    pub alert_rules_created_by_template_count: Option<i32>,
    #[doc = "The time that this alert rule template has been added."]
    #[serde(rename = "createdDateUTC", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_utc: Option<time::OffsetDateTime>,
    #[doc = "The time that this alert rule template was last updated."]
    #[serde(rename = "lastUpdatedDateUTC", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_date_utc: Option<time::OffsetDateTime>,
    #[doc = "The description of the alert rule template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display name for alert rule template."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The required data connectors for this template"]
    #[serde(
        rename = "requiredDataConnectors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_data_connectors: Vec<AlertRuleTemplateDataSource>,
    #[doc = "The alert rule template status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AlertRuleTemplateStatus>,
    #[doc = "The query that creates alerts for this rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[doc = "The frequency (in ISO 8601 duration format) for this alert rule to run."]
    #[serde(rename = "queryFrequency", default, skip_serializing_if = "Option::is_none")]
    pub query_frequency: Option<String>,
    #[doc = "The period (in ISO 8601 duration format) that this alert rule looks at."]
    #[serde(rename = "queryPeriod", default, skip_serializing_if = "Option::is_none")]
    pub query_period: Option<String>,
    #[doc = "The severity of the alert"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<AlertSeverityEnum>,
    #[doc = "The operation against the threshold that triggers alert rule."]
    #[serde(rename = "triggerOperator", default, skip_serializing_if = "Option::is_none")]
    pub trigger_operator: Option<AlertRuleTriggerOperator>,
    #[doc = "The threshold triggers this alert rule."]
    #[serde(rename = "triggerThreshold", default, skip_serializing_if = "Option::is_none")]
    pub trigger_threshold: Option<i32>,
    #[doc = "The tactics of the alert rule template"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tactics: Vec<AttackTactic>,
    #[doc = "The techniques of the alert rule"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub techniques: Vec<String>,
    #[doc = "The version of this template - in format <a.b.c>, where all are numbers. For example <1.0.2>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Event grouping settings property bag."]
    #[serde(rename = "eventGroupingSettings", default, skip_serializing_if = "Option::is_none")]
    pub event_grouping_settings: Option<EventGroupingSettings>,
    #[doc = "Dictionary of string key-value pairs of columns to be attached to the alert"]
    #[serde(rename = "customDetails", default, skip_serializing_if = "Option::is_none")]
    pub custom_details: Option<serde_json::Value>,
    #[doc = "List of entity mappings of the alert rule"]
    #[serde(rename = "entityMappings", default, skip_serializing_if = "Option::is_none")]
    pub entity_mappings: Option<EntityMappings>,
    #[doc = "Settings for how to dynamically override alert static details"]
    #[serde(rename = "alertDetailsOverride", default, skip_serializing_if = "Option::is_none")]
    pub alert_details_override: Option<AlertDetailsOverride>,
    #[doc = "List of sentinel entity mappings of the alert rule"]
    #[serde(rename = "sentinelEntitiesMappings", default, skip_serializing_if = "Option::is_none")]
    pub sentinel_entities_mappings: Option<SentinelEntitiesMappings>,
}
impl ScheduledAlertRuleTemplateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a security alert entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAlert {
    #[serde(flatten)]
    pub entity: Entity,
    #[doc = "SecurityAlert entity property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityAlertProperties>,
}
impl SecurityAlert {
    pub fn new(entity: Entity) -> Self {
        Self { entity, properties: None }
    }
}
#[doc = "SecurityAlert entity property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityAlertProperties {
    #[serde(flatten)]
    pub entity_common_properties: EntityCommonProperties,
    #[doc = "The display name of the alert."]
    #[serde(rename = "alertDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub alert_display_name: Option<String>,
    #[doc = "The type name of the alert."]
    #[serde(rename = "alertType", default, skip_serializing_if = "Option::is_none")]
    pub alert_type: Option<String>,
    #[doc = "Display name of the main entity being reported on."]
    #[serde(rename = "compromisedEntity", default, skip_serializing_if = "Option::is_none")]
    pub compromised_entity: Option<String>,
    #[doc = "The confidence level of this alert."]
    #[serde(rename = "confidenceLevel", default, skip_serializing_if = "Option::is_none")]
    pub confidence_level: Option<security_alert_properties::ConfidenceLevel>,
    #[doc = "The confidence reasons"]
    #[serde(
        rename = "confidenceReasons",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub confidence_reasons: Vec<serde_json::Value>,
    #[doc = "The confidence score of the alert."]
    #[serde(rename = "confidenceScore", default, skip_serializing_if = "Option::is_none")]
    pub confidence_score: Option<f64>,
    #[doc = "The confidence score calculation status, i.e. indicating if score calculation is pending for this alert, not applicable or final."]
    #[serde(rename = "confidenceScoreStatus", default, skip_serializing_if = "Option::is_none")]
    pub confidence_score_status: Option<security_alert_properties::ConfidenceScoreStatus>,
    #[doc = "Alert description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The impact end time of the alert (the time of the last event contributing to the alert)."]
    #[serde(rename = "endTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub end_time_utc: Option<time::OffsetDateTime>,
    #[doc = "Holds the alert intent stage(s) mapping for this alert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<security_alert_properties::Intent>,
    #[doc = "The identifier of the alert inside the product which generated the alert."]
    #[serde(rename = "providerAlertId", default, skip_serializing_if = "Option::is_none")]
    pub provider_alert_id: Option<String>,
    #[doc = "The time the alert was made available for consumption."]
    #[serde(rename = "processingEndTime", default, with = "azure_core::date::rfc3339::option")]
    pub processing_end_time: Option<time::OffsetDateTime>,
    #[doc = "The name of a component inside the product which generated the alert."]
    #[serde(rename = "productComponentName", default, skip_serializing_if = "Option::is_none")]
    pub product_component_name: Option<String>,
    #[doc = "The name of the product which published this alert."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "The version of the product generating the alert."]
    #[serde(rename = "productVersion", default, skip_serializing_if = "Option::is_none")]
    pub product_version: Option<String>,
    #[doc = "Manual action items to take to remediate the alert."]
    #[serde(
        rename = "remediationSteps",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub remediation_steps: Vec<String>,
    #[doc = "The severity of the alert"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<AlertSeverityEnum>,
    #[doc = "The impact start time of the alert (the time of the first event contributing to the alert)."]
    #[serde(rename = "startTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub start_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The lifecycle status of the alert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<security_alert_properties::Status>,
    #[doc = "Holds the product identifier of the alert for the product."]
    #[serde(rename = "systemAlertId", default, skip_serializing_if = "Option::is_none")]
    pub system_alert_id: Option<String>,
    #[doc = "The tactics of the alert"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tactics: Vec<AttackTactic>,
    #[doc = "The time the alert was generated."]
    #[serde(rename = "timeGenerated", default, with = "azure_core::date::rfc3339::option")]
    pub time_generated: Option<time::OffsetDateTime>,
    #[doc = "The name of the vendor that raise the alert."]
    #[serde(rename = "vendorName", default, skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
    #[doc = "The uri link of the alert."]
    #[serde(rename = "alertLink", default, skip_serializing_if = "Option::is_none")]
    pub alert_link: Option<String>,
    #[doc = "The list of resource identifiers of the alert."]
    #[serde(
        rename = "resourceIdentifiers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resource_identifiers: Vec<serde_json::Value>,
}
impl SecurityAlertProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod security_alert_properties {
    use super::*;
    #[doc = "The confidence level of this alert."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConfidenceLevel")]
    pub enum ConfidenceLevel {
        Unknown,
        Low,
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConfidenceLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConfidenceLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConfidenceLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("ConfidenceLevel", 0u32, "Unknown"),
                Self::Low => serializer.serialize_unit_variant("ConfidenceLevel", 1u32, "Low"),
                Self::High => serializer.serialize_unit_variant("ConfidenceLevel", 2u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The confidence score calculation status, i.e. indicating if score calculation is pending for this alert, not applicable or final."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConfidenceScoreStatus")]
    pub enum ConfidenceScoreStatus {
        NotApplicable,
        InProcess,
        NotFinal,
        Final,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConfidenceScoreStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConfidenceScoreStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConfidenceScoreStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotApplicable => serializer.serialize_unit_variant("ConfidenceScoreStatus", 0u32, "NotApplicable"),
                Self::InProcess => serializer.serialize_unit_variant("ConfidenceScoreStatus", 1u32, "InProcess"),
                Self::NotFinal => serializer.serialize_unit_variant("ConfidenceScoreStatus", 2u32, "NotFinal"),
                Self::Final => serializer.serialize_unit_variant("ConfidenceScoreStatus", 3u32, "Final"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Holds the alert intent stage(s) mapping for this alert."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Intent")]
    pub enum Intent {
        Unknown,
        Probing,
        Exploitation,
        Persistence,
        PrivilegeEscalation,
        DefenseEvasion,
        CredentialAccess,
        Discovery,
        LateralMovement,
        Execution,
        Collection,
        Exfiltration,
        CommandAndControl,
        Impact,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Intent {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Intent {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Intent {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("Intent", 0u32, "Unknown"),
                Self::Probing => serializer.serialize_unit_variant("Intent", 1u32, "Probing"),
                Self::Exploitation => serializer.serialize_unit_variant("Intent", 2u32, "Exploitation"),
                Self::Persistence => serializer.serialize_unit_variant("Intent", 3u32, "Persistence"),
                Self::PrivilegeEscalation => serializer.serialize_unit_variant("Intent", 4u32, "PrivilegeEscalation"),
                Self::DefenseEvasion => serializer.serialize_unit_variant("Intent", 5u32, "DefenseEvasion"),
                Self::CredentialAccess => serializer.serialize_unit_variant("Intent", 6u32, "CredentialAccess"),
                Self::Discovery => serializer.serialize_unit_variant("Intent", 7u32, "Discovery"),
                Self::LateralMovement => serializer.serialize_unit_variant("Intent", 8u32, "LateralMovement"),
                Self::Execution => serializer.serialize_unit_variant("Intent", 9u32, "Execution"),
                Self::Collection => serializer.serialize_unit_variant("Intent", 10u32, "Collection"),
                Self::Exfiltration => serializer.serialize_unit_variant("Intent", 11u32, "Exfiltration"),
                Self::CommandAndControl => serializer.serialize_unit_variant("Intent", 12u32, "CommandAndControl"),
                Self::Impact => serializer.serialize_unit_variant("Intent", 13u32, "Impact"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The lifecycle status of the alert."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Unknown,
        New,
        Resolved,
        Dismissed,
        InProgress,
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
                Self::Unknown => serializer.serialize_unit_variant("Status", 0u32, "Unknown"),
                Self::New => serializer.serialize_unit_variant("Status", 1u32, "New"),
                Self::Resolved => serializer.serialize_unit_variant("Status", 2u32, "Resolved"),
                Self::Dismissed => serializer.serialize_unit_variant("Status", 3u32, "Dismissed"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 4u32, "InProgress"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents security alert timeline item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAlertTimelineItem {
    #[doc = "The alert azure resource id."]
    #[serde(rename = "azureResourceId")]
    pub azure_resource_id: String,
    #[doc = "The alert product name."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "The alert description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The alert name."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The severity of the alert"]
    pub severity: AlertSeverityEnum,
    #[doc = "The alert end time."]
    #[serde(rename = "endTimeUtc", with = "azure_core::date::rfc3339")]
    pub end_time_utc: time::OffsetDateTime,
    #[doc = "The alert start time."]
    #[serde(rename = "startTimeUtc", with = "azure_core::date::rfc3339")]
    pub start_time_utc: time::OffsetDateTime,
    #[doc = "The alert generated time."]
    #[serde(rename = "timeGenerated", with = "azure_core::date::rfc3339")]
    pub time_generated: time::OffsetDateTime,
    #[doc = "The name of the alert type."]
    #[serde(rename = "alertType")]
    pub alert_type: String,
    #[doc = "The intent of the alert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<security_alert_timeline_item::Intent>,
    #[doc = "The techniques of the alert."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub techniques: Vec<String>,
}
impl SecurityAlertTimelineItem {
    pub fn new(
        azure_resource_id: String,
        display_name: String,
        severity: AlertSeverityEnum,
        end_time_utc: time::OffsetDateTime,
        start_time_utc: time::OffsetDateTime,
        time_generated: time::OffsetDateTime,
        alert_type: String,
    ) -> Self {
        Self {
            azure_resource_id,
            product_name: None,
            description: None,
            display_name,
            severity,
            end_time_utc,
            start_time_utc,
            time_generated,
            alert_type,
            intent: None,
            techniques: Vec::new(),
        }
    }
}
pub mod security_alert_timeline_item {
    use super::*;
    #[doc = "The intent of the alert."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Intent")]
    pub enum Intent {
        Unknown,
        Probing,
        Exploitation,
        Persistence,
        PrivilegeEscalation,
        DefenseEvasion,
        CredentialAccess,
        Discovery,
        LateralMovement,
        Execution,
        Collection,
        Exfiltration,
        CommandAndControl,
        Impact,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Intent {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Intent {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Intent {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("Intent", 0u32, "Unknown"),
                Self::Probing => serializer.serialize_unit_variant("Intent", 1u32, "Probing"),
                Self::Exploitation => serializer.serialize_unit_variant("Intent", 2u32, "Exploitation"),
                Self::Persistence => serializer.serialize_unit_variant("Intent", 3u32, "Persistence"),
                Self::PrivilegeEscalation => serializer.serialize_unit_variant("Intent", 4u32, "PrivilegeEscalation"),
                Self::DefenseEvasion => serializer.serialize_unit_variant("Intent", 5u32, "DefenseEvasion"),
                Self::CredentialAccess => serializer.serialize_unit_variant("Intent", 6u32, "CredentialAccess"),
                Self::Discovery => serializer.serialize_unit_variant("Intent", 7u32, "Discovery"),
                Self::LateralMovement => serializer.serialize_unit_variant("Intent", 8u32, "LateralMovement"),
                Self::Execution => serializer.serialize_unit_variant("Intent", 9u32, "Execution"),
                Self::Collection => serializer.serialize_unit_variant("Intent", 10u32, "Collection"),
                Self::Exfiltration => serializer.serialize_unit_variant("Intent", 11u32, "Exfiltration"),
                Self::CommandAndControl => serializer.serialize_unit_variant("Intent", 12u32, "CommandAndControl"),
                Self::Impact => serializer.serialize_unit_variant("Intent", 13u32, "Impact"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Security ML Analytics Setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMlAnalyticsSetting {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
}
impl SecurityMlAnalyticsSetting {
    pub fn new() -> Self {
        Self {
            resource_with_etag: ResourceWithEtag::default(),
        }
    }
}
#[doc = "The kind of security ML analytics settings"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum SecurityMlAnalyticsSettingUnion {
    Anomaly(AnomalySecurityMlAnalyticsSettings),
}
#[doc = "security ml analytics settings data sources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityMlAnalyticsSettingsDataSource {
    #[doc = "The connector id that provides the following data types"]
    #[serde(rename = "connectorId", default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[doc = "The data types used by the security ml analytics settings"]
    #[serde(
        rename = "dataTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_types: Vec<String>,
}
impl SecurityMlAnalyticsSettingsDataSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kind of security ML analytics settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SecurityMlAnalyticsSettingsKindEnum")]
pub enum SecurityMlAnalyticsSettingsKindEnum {
    Anomaly,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SecurityMlAnalyticsSettingsKindEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SecurityMlAnalyticsSettingsKindEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SecurityMlAnalyticsSettingsKindEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Anomaly => serializer.serialize_unit_variant("SecurityMlAnalyticsSettingsKindEnum", 0u32, "Anomaly"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "List all the SecurityMLAnalyticsSettings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMlAnalyticsSettingsList {
    #[doc = "URL to fetch the next set of SecurityMLAnalyticsSettings."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of SecurityMLAnalyticsSettings"]
    pub value: Vec<SecurityMlAnalyticsSettingUnion>,
}
impl azure_core::Continuable for SecurityMlAnalyticsSettingsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SecurityMlAnalyticsSettingsList {
    pub fn new(value: Vec<SecurityMlAnalyticsSettingUnion>) -> Self {
        Self { next_link: None, value }
    }
}
pub type SentinelEntitiesMappings = Vec<SentinelEntityMapping>;
#[doc = "A single sentinel entity mapping"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SentinelEntityMapping {
    #[doc = "the column name to be mapped to the SentinelEntities"]
    #[serde(rename = "columnName", default, skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
}
impl SentinelEntityMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sentinel onboarding state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SentinelOnboardingState {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "The Sentinel onboarding state properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SentinelOnboardingStateProperties>,
}
impl SentinelOnboardingState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Sentinel onboarding state properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SentinelOnboardingStateProperties {
    #[doc = "Flag that indicates the status of the CMK setting"]
    #[serde(rename = "customerManagedKey", default, skip_serializing_if = "Option::is_none")]
    pub customer_managed_key: Option<bool>,
}
impl SentinelOnboardingStateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of the Sentinel onboarding states"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentinelOnboardingStatesList {
    #[doc = "Array of Sentinel onboarding states"]
    pub value: Vec<SentinelOnboardingState>,
}
impl SentinelOnboardingStatesList {
    pub fn new(value: Vec<SentinelOnboardingState>) -> Self {
        Self { value }
    }
}
#[doc = "Service principal metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicePrincipal {
    #[doc = "Id of service principal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Tenant id of service principal."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "App id of service principal."]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}
impl ServicePrincipal {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of all the settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SettingList {
    #[doc = "Array of settings."]
    pub value: Vec<SettingsUnion>,
}
impl SettingList {
    pub fn new(value: Vec<SettingsUnion>) -> Self {
        Self { value }
    }
}
#[doc = "The Setting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
}
impl Settings {
    pub fn new() -> Self {
        Self {
            resource_with_etag: ResourceWithEtag::default(),
        }
    }
}
#[doc = "The kind of the setting"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum SettingsUnion {
    Anomalies(Anomalies),
    EntityAnalytics(EntityAnalytics),
    EyesOn(EyesOn),
    Ueba(Ueba),
}
#[doc = "Represents a SourceControl in Azure Security Insights."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControl {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "Describes source control properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SourceControlProperties>,
}
impl SourceControl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List all the source controls."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceControlList {
    #[doc = "URL to fetch the next set of source controls."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of source controls."]
    pub value: Vec<SourceControl>,
}
impl azure_core::Continuable for SourceControlList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SourceControlList {
    pub fn new(value: Vec<SourceControl>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Describes source control properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceControlProperties {
    #[doc = "The id (a Guid) of the source control"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The version of the source control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
    #[doc = "The display name of the source control"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "A description of the source control"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The type of repository."]
    #[serde(rename = "repoType")]
    pub repo_type: RepoType,
    #[doc = "Array of source control content types."]
    #[serde(rename = "contentTypes")]
    pub content_types: Vec<ContentType>,
    #[doc = "metadata of a repository."]
    pub repository: Repository,
    #[doc = "Service principal metadata."]
    #[serde(rename = "servicePrincipal", default, skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<ServicePrincipal>,
    #[doc = "Credentials to access repository."]
    #[serde(rename = "repositoryAccess", default, skip_serializing_if = "Option::is_none")]
    pub repository_access: Option<RepositoryAccess>,
    #[doc = "Resources created in user's repository for the source-control."]
    #[serde(rename = "repositoryResourceInfo", default, skip_serializing_if = "Option::is_none")]
    pub repository_resource_info: Option<RepositoryResourceInfo>,
    #[doc = "Information regarding a deployment."]
    #[serde(rename = "lastDeploymentInfo", default, skip_serializing_if = "Option::is_none")]
    pub last_deployment_info: Option<DeploymentInfo>,
    #[doc = "Information regarding pull request for protected branches."]
    #[serde(rename = "pullRequest", default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
}
impl SourceControlProperties {
    pub fn new(display_name: String, repo_type: RepoType, content_types: Vec<ContentType>, repository: Repository) -> Self {
        Self {
            id: None,
            version: None,
            display_name,
            description: None,
            repo_type,
            content_types,
            repository,
            service_principal: None,
            repository_access: None,
            repository_resource_info: None,
            last_deployment_info: None,
            pull_request: None,
        }
    }
}
#[doc = "Threat Intelligence Platforms data connector check requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiCheckRequirements {
    #[doc = "Threat Intelligence Platforms data connector required properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TiCheckRequirementsProperties>,
}
impl TiCheckRequirements {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "Threat Intelligence Platforms data connector required properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiCheckRequirementsProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
}
impl TiCheckRequirementsProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self { data_connector_tenant_id }
    }
}
#[doc = "Represents threat intelligence data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "TI (Threat Intelligence) data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TiDataConnectorProperties>,
}
impl TiDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "The available data types for TI (Threat Intelligence) data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiDataConnectorDataTypes {
    #[doc = "Data type for indicators connection."]
    pub indicators: serde_json::Value,
}
impl TiDataConnectorDataTypes {
    pub fn new(indicators: serde_json::Value) -> Self {
        Self { indicators }
    }
}
#[doc = "TI (Threat Intelligence) data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[doc = "The lookback period for the feed to be imported."]
    #[serde(rename = "tipLookbackPeriod", default, with = "azure_core::date::rfc3339::option")]
    pub tip_lookback_period: Option<time::OffsetDateTime>,
    #[doc = "The available data types for TI (Threat Intelligence) data connector."]
    #[serde(rename = "dataTypes")]
    pub data_types: TiDataConnectorDataTypes,
}
impl TiDataConnectorProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId, data_types: TiDataConnectorDataTypes) -> Self {
        Self {
            data_connector_tenant_id,
            tip_lookback_period: None,
            data_types,
        }
    }
}
#[doc = "Describes team information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamInformation {
    #[doc = "Team ID"]
    #[serde(rename = "teamId", default, skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[doc = "The primary channel URL of the team"]
    #[serde(rename = "primaryChannelUrl", default, skip_serializing_if = "Option::is_none")]
    pub primary_channel_url: Option<String>,
    #[doc = "The time the team was created"]
    #[serde(rename = "teamCreationTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub team_creation_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The name of the team"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The description of the team"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl TeamInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes team properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamProperties {
    #[doc = "The name of the team"]
    #[serde(rename = "teamName")]
    pub team_name: String,
    #[doc = "The description of the team"]
    #[serde(rename = "teamDescription", default, skip_serializing_if = "Option::is_none")]
    pub team_description: Option<String>,
    #[doc = "List of group IDs to add their members to the team"]
    #[serde(
        rename = "groupIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_ids: Vec<String>,
    #[doc = "List of member IDs to add to the team"]
    #[serde(
        rename = "memberIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub member_ids: Vec<String>,
}
impl TeamProperties {
    pub fn new(team_name: String) -> Self {
        Self {
            team_name,
            team_description: None,
            group_ids: Vec::new(),
            member_ids: Vec::new(),
        }
    }
}
#[doc = "Represents Threat Intelligence alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreatIntelligenceAlertRule {
    #[serde(flatten)]
    pub alert_rule: AlertRule,
    #[doc = "Threat Intelligence alert rule base property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ThreatIntelligenceAlertRuleProperties>,
}
impl ThreatIntelligenceAlertRule {
    pub fn new(alert_rule: AlertRule) -> Self {
        Self {
            alert_rule,
            properties: None,
        }
    }
}
#[doc = "Threat Intelligence alert rule base property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreatIntelligenceAlertRuleProperties {
    #[doc = "The Name of the alert rule template used to create this rule."]
    #[serde(rename = "alertRuleTemplateName")]
    pub alert_rule_template_name: String,
    #[doc = "The description of the alert rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display name for alerts created by this alert rule."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Determines whether this alert rule is enabled or disabled."]
    pub enabled: bool,
    #[doc = "The last time that this alert has been modified."]
    #[serde(rename = "lastModifiedUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_utc: Option<time::OffsetDateTime>,
    #[doc = "The severity of the alert"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<AlertSeverityEnum>,
    #[doc = "The tactics of the alert rule"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tactics: Vec<AttackTactic>,
    #[doc = "The techniques of the alert rule"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub techniques: Vec<String>,
}
impl ThreatIntelligenceAlertRuleProperties {
    pub fn new(alert_rule_template_name: String, enabled: bool) -> Self {
        Self {
            alert_rule_template_name,
            description: None,
            display_name: None,
            enabled,
            last_modified_utc: None,
            severity: None,
            tactics: Vec::new(),
            techniques: Vec::new(),
        }
    }
}
#[doc = "Represents Threat Intelligence alert rule template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreatIntelligenceAlertRuleTemplate {
    #[serde(flatten)]
    pub alert_rule_template: AlertRuleTemplate,
    #[doc = "Threat Intelligence alert rule template properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<threat_intelligence_alert_rule_template::Properties>,
}
impl ThreatIntelligenceAlertRuleTemplate {
    pub fn new(alert_rule_template: AlertRuleTemplate) -> Self {
        Self {
            alert_rule_template,
            properties: None,
        }
    }
}
pub mod threat_intelligence_alert_rule_template {
    use super::*;
    #[doc = "Threat Intelligence alert rule template properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(flatten)]
        pub alert_rule_template_with_mitre_properties: AlertRuleTemplateWithMitreProperties,
        #[doc = "The severity of the alert"]
        pub severity: AlertSeverityEnum,
    }
    impl Properties {
        pub fn new(severity: AlertSeverityEnum) -> Self {
            Self {
                alert_rule_template_with_mitre_properties: AlertRuleTemplateWithMitreProperties::default(),
                severity,
            }
        }
    }
}
#[doc = "Array of tags to be appended to the threat intelligence indicator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThreatIntelligenceAppendTags {
    #[doc = "List of tags to be appended."]
    #[serde(
        rename = "threatIntelligenceTags",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub threat_intelligence_tags: Vec<String>,
}
impl ThreatIntelligenceAppendTags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes external reference"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThreatIntelligenceExternalReference {
    #[doc = "External reference description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "External reference ID"]
    #[serde(rename = "externalId", default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[doc = "External reference source name"]
    #[serde(rename = "sourceName", default, skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[doc = "External reference URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "External reference hashes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hashes: Option<serde_json::Value>,
}
impl ThreatIntelligenceExternalReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Filtering criteria for querying threat intelligence indicators."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThreatIntelligenceFilteringCriteria {
    #[doc = "Page size"]
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[doc = "Minimum confidence."]
    #[serde(rename = "minConfidence", default, skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<i32>,
    #[doc = "Maximum confidence."]
    #[serde(rename = "maxConfidence", default, skip_serializing_if = "Option::is_none")]
    pub max_confidence: Option<i32>,
    #[doc = "Start time for ValidUntil filter."]
    #[serde(rename = "minValidUntil", default, skip_serializing_if = "Option::is_none")]
    pub min_valid_until: Option<String>,
    #[doc = "End time for ValidUntil filter."]
    #[serde(rename = "maxValidUntil", default, skip_serializing_if = "Option::is_none")]
    pub max_valid_until: Option<String>,
    #[doc = "Parameter to include/exclude disabled indicators."]
    #[serde(rename = "includeDisabled", default, skip_serializing_if = "Option::is_none")]
    pub include_disabled: Option<bool>,
    #[doc = "Columns to sort by and sorting order"]
    #[serde(
        rename = "sortBy",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sort_by: Vec<ThreatIntelligenceSortingCriteria>,
    #[doc = "Sources of threat intelligence indicators"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sources: Vec<String>,
    #[doc = "Pattern types"]
    #[serde(
        rename = "patternTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pattern_types: Vec<String>,
    #[doc = "Threat types of threat intelligence indicators"]
    #[serde(
        rename = "threatTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub threat_types: Vec<String>,
    #[doc = "Ids of threat intelligence indicators"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ids: Vec<String>,
    #[doc = "Keywords for searching threat intelligence indicators"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub keywords: Vec<String>,
    #[doc = "Skip token."]
    #[serde(rename = "skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
}
impl ThreatIntelligenceFilteringCriteria {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes threat granular marking model entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThreatIntelligenceGranularMarkingModel {
    #[doc = "Language granular marking model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[doc = "marking reference granular marking model"]
    #[serde(rename = "markingRef", default, skip_serializing_if = "Option::is_none")]
    pub marking_ref: Option<i32>,
    #[doc = "granular marking model selectors"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub selectors: Vec<String>,
}
impl ThreatIntelligenceGranularMarkingModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Threat intelligence indicator entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreatIntelligenceIndicatorModel {
    #[serde(flatten)]
    pub threat_intelligence_information: ThreatIntelligenceInformation,
    #[doc = "Describes threat intelligence entity properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ThreatIntelligenceIndicatorProperties>,
}
impl ThreatIntelligenceIndicatorModel {
    pub fn new(threat_intelligence_information: ThreatIntelligenceInformation) -> Self {
        Self {
            threat_intelligence_information,
            properties: None,
        }
    }
}
#[doc = "Describes threat intelligence entity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThreatIntelligenceIndicatorProperties {
    #[serde(flatten)]
    pub entity_common_properties: EntityCommonProperties,
    #[doc = "List of tags"]
    #[serde(
        rename = "threatIntelligenceTags",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub threat_intelligence_tags: Vec<String>,
    #[doc = "Last updated time in UTC"]
    #[serde(rename = "lastUpdatedTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time_utc: Option<String>,
    #[doc = "Source of a threat intelligence entity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Display name of a threat intelligence entity"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Description of a threat intelligence entity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Indicator types of threat intelligence entities"]
    #[serde(
        rename = "indicatorTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub indicator_types: Vec<String>,
    #[doc = "Pattern of a threat intelligence entity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[doc = "Pattern type of a threat intelligence entity"]
    #[serde(rename = "patternType", default, skip_serializing_if = "Option::is_none")]
    pub pattern_type: Option<String>,
    #[doc = "Pattern version of a threat intelligence entity"]
    #[serde(rename = "patternVersion", default, skip_serializing_if = "Option::is_none")]
    pub pattern_version: Option<String>,
    #[doc = "Kill chain phases"]
    #[serde(
        rename = "killChainPhases",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub kill_chain_phases: Vec<ThreatIntelligenceKillChainPhase>,
    #[doc = "Parsed patterns"]
    #[serde(
        rename = "parsedPattern",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parsed_pattern: Vec<ThreatIntelligenceParsedPattern>,
    #[doc = "External ID of threat intelligence entity"]
    #[serde(rename = "externalId", default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[doc = "Created by reference of threat intelligence entity"]
    #[serde(rename = "createdByRef", default, skip_serializing_if = "Option::is_none")]
    pub created_by_ref: Option<String>,
    #[doc = "Is threat intelligence entity defanged"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defanged: Option<bool>,
    #[doc = "External last updated time in UTC"]
    #[serde(rename = "externalLastUpdatedTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub external_last_updated_time_utc: Option<String>,
    #[doc = "External References"]
    #[serde(
        rename = "externalReferences",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub external_references: Vec<ThreatIntelligenceExternalReference>,
    #[doc = "Granular Markings"]
    #[serde(
        rename = "granularMarkings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub granular_markings: Vec<ThreatIntelligenceGranularMarkingModel>,
    #[doc = "Labels  of threat intelligence entity"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub labels: Vec<String>,
    #[doc = "Is threat intelligence entity revoked"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked: Option<bool>,
    #[doc = "Confidence of threat intelligence entity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i32>,
    #[doc = "Threat intelligence entity object marking references"]
    #[serde(
        rename = "objectMarkingRefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub object_marking_refs: Vec<String>,
    #[doc = "Language of threat intelligence entity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[doc = "Threat types"]
    #[serde(
        rename = "threatTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub threat_types: Vec<String>,
    #[doc = "Valid from"]
    #[serde(rename = "validFrom", default, skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    #[doc = "Valid until"]
    #[serde(rename = "validUntil", default, skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<String>,
    #[doc = "Created by"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[doc = "Modified by"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
    #[doc = "Extensions map"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<serde_json::Value>,
}
impl ThreatIntelligenceIndicatorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Threat intelligence information object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreatIntelligenceInformation {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
}
impl ThreatIntelligenceInformation {
    pub fn new() -> Self {
        Self {
            resource_with_etag: ResourceWithEtag::default(),
        }
    }
}
#[doc = "The kind of the threat intelligence entity"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ThreatIntelligenceInformationUnion {
    #[serde(rename = "indicator")]
    Indicator(ThreatIntelligenceIndicatorModel),
}
#[doc = "List of all the threat intelligence information objects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreatIntelligenceInformationList {
    #[doc = "URL to fetch the next set of information objects."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of threat intelligence information objects."]
    pub value: Vec<ThreatIntelligenceInformationUnion>,
}
impl azure_core::Continuable for ThreatIntelligenceInformationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ThreatIntelligenceInformationList {
    pub fn new(value: Vec<ThreatIntelligenceInformationUnion>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Describes threat kill chain phase entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThreatIntelligenceKillChainPhase {
    #[doc = "Kill chainName name"]
    #[serde(rename = "killChainName", default, skip_serializing_if = "Option::is_none")]
    pub kill_chain_name: Option<String>,
    #[doc = "Phase name"]
    #[serde(rename = "phaseName", default, skip_serializing_if = "Option::is_none")]
    pub phase_name: Option<String>,
}
impl ThreatIntelligenceKillChainPhase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes threat intelligence metric"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThreatIntelligenceMetric {
    #[doc = "Last updated indicator metric"]
    #[serde(rename = "lastUpdatedTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time_utc: Option<String>,
    #[doc = "Threat type metrics"]
    #[serde(
        rename = "threatTypeMetrics",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub threat_type_metrics: Vec<ThreatIntelligenceMetricEntity>,
    #[doc = "Pattern type metrics"]
    #[serde(
        rename = "patternTypeMetrics",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pattern_type_metrics: Vec<ThreatIntelligenceMetricEntity>,
    #[doc = "Source metrics"]
    #[serde(
        rename = "sourceMetrics",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_metrics: Vec<ThreatIntelligenceMetricEntity>,
}
impl ThreatIntelligenceMetric {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes threat intelligence metric entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThreatIntelligenceMetricEntity {
    #[doc = "Metric name"]
    #[serde(rename = "metricName", default, skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[doc = "Metric value"]
    #[serde(rename = "metricValue", default, skip_serializing_if = "Option::is_none")]
    pub metric_value: Option<i32>,
}
impl ThreatIntelligenceMetricEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Threat intelligence metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThreatIntelligenceMetrics {
    #[doc = "Describes threat intelligence metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ThreatIntelligenceMetric>,
}
impl ThreatIntelligenceMetrics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of all the threat intelligence metric fields (type/threat type/source)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreatIntelligenceMetricsList {
    #[doc = "Array of threat intelligence metric fields (type/threat type/source)."]
    pub value: Vec<ThreatIntelligenceMetrics>,
}
impl ThreatIntelligenceMetricsList {
    pub fn new(value: Vec<ThreatIntelligenceMetrics>) -> Self {
        Self { value }
    }
}
#[doc = "Describes parsed pattern entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThreatIntelligenceParsedPattern {
    #[doc = "Pattern type key"]
    #[serde(rename = "patternTypeKey", default, skip_serializing_if = "Option::is_none")]
    pub pattern_type_key: Option<String>,
    #[doc = "Pattern type keys"]
    #[serde(
        rename = "patternTypeValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pattern_type_values: Vec<ThreatIntelligenceParsedPatternTypeValue>,
}
impl ThreatIntelligenceParsedPattern {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes threat kill chain phase entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThreatIntelligenceParsedPatternTypeValue {
    #[doc = "Type of the value"]
    #[serde(rename = "valueType", default, skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
    #[doc = "Value of parsed pattern"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ThreatIntelligenceParsedPatternTypeValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kind of the threat intelligence entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ThreatIntelligenceResourceInnerKind")]
pub enum ThreatIntelligenceResourceInnerKind {
    #[serde(rename = "indicator")]
    Indicator,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ThreatIntelligenceResourceInnerKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ThreatIntelligenceResourceInnerKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ThreatIntelligenceResourceInnerKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Indicator => serializer.serialize_unit_variant("ThreatIntelligenceResourceInnerKind", 0u32, "indicator"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "List of available columns for sorting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThreatIntelligenceSortingCriteria {
    #[doc = "Column name"]
    #[serde(rename = "itemKey", default, skip_serializing_if = "Option::is_none")]
    pub item_key: Option<String>,
    #[doc = "Sorting order (ascending/descending/unsorted)."]
    #[serde(rename = "sortOrder", default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<ThreatIntelligenceSortingOrder>,
}
impl ThreatIntelligenceSortingCriteria {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sorting order (ascending/descending/unsorted)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ThreatIntelligenceSortingOrder")]
pub enum ThreatIntelligenceSortingOrder {
    #[serde(rename = "unsorted")]
    Unsorted,
    #[serde(rename = "ascending")]
    Ascending,
    #[serde(rename = "descending")]
    Descending,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ThreatIntelligenceSortingOrder {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ThreatIntelligenceSortingOrder {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ThreatIntelligenceSortingOrder {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unsorted => serializer.serialize_unit_variant("ThreatIntelligenceSortingOrder", 0u32, "unsorted"),
            Self::Ascending => serializer.serialize_unit_variant("ThreatIntelligenceSortingOrder", 1u32, "ascending"),
            Self::Descending => serializer.serialize_unit_variant("ThreatIntelligenceSortingOrder", 2u32, "descending"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Threat Intelligence TAXII data connector check requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiTaxiiCheckRequirements {
    #[doc = "Threat Intelligence TAXII data connector required properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TiTaxiiCheckRequirementsProperties>,
}
impl TiTaxiiCheckRequirements {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
#[doc = "Threat Intelligence TAXII data connector required properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiTaxiiCheckRequirementsProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
}
impl TiTaxiiCheckRequirementsProperties {
    pub fn new(data_connector_tenant_id: DataConnectorTenantId) -> Self {
        Self { data_connector_tenant_id }
    }
}
#[doc = "Data connector to pull Threat intelligence data from TAXII 2.0/2.1 server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiTaxiiDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[doc = "Threat Intelligence TAXII data connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TiTaxiiDataConnectorProperties>,
}
impl TiTaxiiDataConnector {
    pub fn new(data_connector: DataConnector) -> Self {
        Self {
            data_connector,
            properties: None,
        }
    }
}
#[doc = "The available data types for Threat Intelligence TAXII data connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiTaxiiDataConnectorDataTypes {
    #[doc = "Data type for TAXII connector."]
    #[serde(rename = "taxiiClient")]
    pub taxii_client: serde_json::Value,
}
impl TiTaxiiDataConnectorDataTypes {
    pub fn new(taxii_client: serde_json::Value) -> Self {
        Self { taxii_client }
    }
}
#[doc = "Threat Intelligence TAXII data connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiTaxiiDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[doc = "The workspace id."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "The friendly name for the TAXII server."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The API root for the TAXII server."]
    #[serde(rename = "taxiiServer", default, skip_serializing_if = "Option::is_none")]
    pub taxii_server: Option<String>,
    #[doc = "The collection id of the TAXII server."]
    #[serde(rename = "collectionId", default, skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[doc = "The userName for the TAXII server."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "The password for the TAXII server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "The lookback period for the TAXII server."]
    #[serde(rename = "taxiiLookbackPeriod", default, with = "azure_core::date::rfc3339::option")]
    pub taxii_lookback_period: Option<time::OffsetDateTime>,
    #[doc = "The polling frequency for the TAXII server."]
    #[serde(rename = "pollingFrequency")]
    pub polling_frequency: ti_taxii_data_connector_properties::PollingFrequency,
    #[doc = "The available data types for Threat Intelligence TAXII data connector."]
    #[serde(rename = "dataTypes")]
    pub data_types: TiTaxiiDataConnectorDataTypes,
}
impl TiTaxiiDataConnectorProperties {
    pub fn new(
        data_connector_tenant_id: DataConnectorTenantId,
        polling_frequency: ti_taxii_data_connector_properties::PollingFrequency,
        data_types: TiTaxiiDataConnectorDataTypes,
    ) -> Self {
        Self {
            data_connector_tenant_id,
            workspace_id: None,
            friendly_name: None,
            taxii_server: None,
            collection_id: None,
            user_name: None,
            password: None,
            taxii_lookback_period: None,
            polling_frequency,
            data_types,
        }
    }
}
pub mod ti_taxii_data_connector_properties {
    use super::*;
    #[doc = "The polling frequency for the TAXII server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PollingFrequency")]
    pub enum PollingFrequency {
        OnceAMinute,
        OnceAnHour,
        OnceADay,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PollingFrequency {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PollingFrequency {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PollingFrequency {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::OnceAMinute => serializer.serialize_unit_variant("PollingFrequency", 0u32, "OnceAMinute"),
                Self::OnceAnHour => serializer.serialize_unit_variant("PollingFrequency", 1u32, "OnceAnHour"),
                Self::OnceADay => serializer.serialize_unit_variant("PollingFrequency", 2u32, "OnceADay"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "timeline aggregation information per kind"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimelineAggregation {
    #[doc = "the total items found for a kind"]
    pub count: i32,
    #[doc = "The entity query kind"]
    pub kind: EntityTimelineKind,
}
impl TimelineAggregation {
    pub fn new(count: i32, kind: EntityTimelineKind) -> Self {
        Self { count, kind }
    }
}
#[doc = "Timeline Query Errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimelineError {
    #[doc = "The entity query kind"]
    pub kind: EntityTimelineKind,
    #[doc = "the query id"]
    #[serde(rename = "queryId", default, skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[doc = "the error message"]
    #[serde(rename = "errorMessage")]
    pub error_message: String,
}
impl TimelineError {
    pub fn new(kind: EntityTimelineKind, error_message: String) -> Self {
        Self {
            kind,
            query_id: None,
            error_message,
        }
    }
}
#[doc = "Expansion result metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimelineResultsMetadata {
    #[doc = "the total items found for the timeline request"]
    #[serde(rename = "totalCount")]
    pub total_count: i32,
    #[doc = "timeline aggregation per kind"]
    pub aggregations: Vec<TimelineAggregation>,
    #[doc = "information about the failure queries"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<TimelineError>,
}
impl TimelineResultsMetadata {
    pub fn new(total_count: i32, aggregations: Vec<TimelineAggregation>) -> Self {
        Self {
            total_count,
            aggregations,
            errors: Vec::new(),
        }
    }
}
#[doc = "The triggered analytics rule run"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggeredAnalyticsRuleRun {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "The triggered analytics rule run Properties"]
    pub properties: TriggeredAnalyticsRuleRunProperties,
}
impl TriggeredAnalyticsRuleRun {
    pub fn new(properties: TriggeredAnalyticsRuleRunProperties) -> Self {
        Self {
            resource_with_etag: ResourceWithEtag::default(),
            properties,
        }
    }
}
#[doc = "The triggered analytics rule run Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggeredAnalyticsRuleRunProperties {
    #[serde(rename = "executionTimeUtc", with = "azure_core::date::rfc3339")]
    pub execution_time_utc: time::OffsetDateTime,
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    #[serde(rename = "triggeredAnalyticsRuleRunId")]
    pub triggered_analytics_rule_run_id: String,
    #[doc = "The triggered analytics rule run provisioning state"]
    #[serde(rename = "provisioningState")]
    pub provisioning_state: ProvisioningState,
    #[serde(rename = "ruleRunAdditionalData", default, skip_serializing_if = "Option::is_none")]
    pub rule_run_additional_data: Option<serde_json::Value>,
}
impl TriggeredAnalyticsRuleRunProperties {
    pub fn new(
        execution_time_utc: time::OffsetDateTime,
        rule_id: String,
        triggered_analytics_rule_run_id: String,
        provisioning_state: ProvisioningState,
    ) -> Self {
        Self {
            execution_time_utc,
            rule_id,
            triggered_analytics_rule_run_id,
            provisioning_state,
            rule_run_additional_data: None,
        }
    }
}
#[doc = "The triggered analytics rule run array"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggeredAnalyticsRuleRuns {
    pub value: Vec<TriggeredAnalyticsRuleRun>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TriggeredAnalyticsRuleRuns {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TriggeredAnalyticsRuleRuns {
    pub fn new(value: Vec<TriggeredAnalyticsRuleRun>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Settings with single toggle."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ueba {
    #[serde(flatten)]
    pub settings: Settings,
    #[doc = "Ueba property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UebaProperties>,
}
impl Ueba {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            properties: None,
        }
    }
}
#[doc = "The data source that enriched by ueba."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UebaDataSources")]
pub enum UebaDataSources {
    AuditLogs,
    AzureActivity,
    SecurityEvent,
    SigninLogs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UebaDataSources {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UebaDataSources {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UebaDataSources {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AuditLogs => serializer.serialize_unit_variant("UebaDataSources", 0u32, "AuditLogs"),
            Self::AzureActivity => serializer.serialize_unit_variant("UebaDataSources", 1u32, "AzureActivity"),
            Self::SecurityEvent => serializer.serialize_unit_variant("UebaDataSources", 2u32, "SecurityEvent"),
            Self::SigninLogs => serializer.serialize_unit_variant("UebaDataSources", 3u32, "SigninLogs"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Ueba property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UebaProperties {
    #[doc = "The relevant data sources that enriched by ueba"]
    #[serde(
        rename = "dataSources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_sources: Vec<UebaDataSources>,
}
impl UebaProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User information that made some action"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserInfo {
    #[doc = "The email of the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The name of the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object id of the user."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}
impl UserInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an error encountered in the file during validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidationError {
    #[doc = "The number of the record that has the error."]
    #[serde(rename = "recordIndex", default, skip_serializing_if = "Option::is_none")]
    pub record_index: Option<i32>,
    #[doc = "A list of descriptions of the error."]
    #[serde(
        rename = "errorMessages",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub error_messages: Vec<String>,
}
impl ValidationError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The version of the source control."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Version")]
pub enum Version {
    V1,
    V2,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Version {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::V1 => serializer.serialize_unit_variant("Version", 0u32, "V1"),
            Self::V2 => serializer.serialize_unit_variant("Version", 1u32, "V2"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Warning response structure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Warning {
    #[doc = "Warning details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning: Option<WarningBody>,
}
impl Warning {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Warning details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WarningBody {
    #[doc = "The type of repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<WarningCode>,
    #[doc = "A message describing the warning, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<WarningBody>,
}
impl WarningBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of repository."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WarningCode")]
pub enum WarningCode {
    #[serde(rename = "SourceControlWarning_DeleteServicePrincipal")]
    SourceControlWarningDeleteServicePrincipal,
    #[serde(rename = "SourceControlWarning_DeletePipelineFromAzureDevOps")]
    SourceControlWarningDeletePipelineFromAzureDevOps,
    #[serde(rename = "SourceControlWarning_DeleteWorkflowAndSecretFromGitHub")]
    SourceControlWarningDeleteWorkflowAndSecretFromGitHub,
    #[serde(rename = "SourceControlWarning_DeleteRoleAssignment")]
    SourceControlWarningDeleteRoleAssignment,
    #[serde(rename = "SourceControl_DeletedWithWarnings")]
    SourceControlDeletedWithWarnings,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WarningCode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WarningCode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WarningCode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SourceControlWarningDeleteServicePrincipal => {
                serializer.serialize_unit_variant("WarningCode", 0u32, "SourceControlWarning_DeleteServicePrincipal")
            }
            Self::SourceControlWarningDeletePipelineFromAzureDevOps => {
                serializer.serialize_unit_variant("WarningCode", 1u32, "SourceControlWarning_DeletePipelineFromAzureDevOps")
            }
            Self::SourceControlWarningDeleteWorkflowAndSecretFromGitHub => {
                serializer.serialize_unit_variant("WarningCode", 2u32, "SourceControlWarning_DeleteWorkflowAndSecretFromGitHub")
            }
            Self::SourceControlWarningDeleteRoleAssignment => {
                serializer.serialize_unit_variant("WarningCode", 3u32, "SourceControlWarning_DeleteRoleAssignment")
            }
            Self::SourceControlDeletedWithWarnings => {
                serializer.serialize_unit_variant("WarningCode", 4u32, "SourceControl_DeletedWithWarnings")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents a Watchlist in Azure Security Insights."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Watchlist {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "Describes watchlist properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WatchlistProperties>,
}
impl Watchlist {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Watchlist item in Azure Security Insights."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WatchlistItem {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "Describes watchlist item properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WatchlistItemProperties>,
}
impl WatchlistItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List all the watchlist items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistItemList {
    #[doc = "URL to fetch the next set of watchlist item."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of watchlist items."]
    pub value: Vec<WatchlistItem>,
}
impl azure_core::Continuable for WatchlistItemList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WatchlistItemList {
    pub fn new(value: Vec<WatchlistItem>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Describes watchlist item properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistItemProperties {
    #[doc = "The type of the watchlist item"]
    #[serde(rename = "watchlistItemType", default, skip_serializing_if = "Option::is_none")]
    pub watchlist_item_type: Option<String>,
    #[doc = "The id (a Guid) of the watchlist item"]
    #[serde(rename = "watchlistItemId", default, skip_serializing_if = "Option::is_none")]
    pub watchlist_item_id: Option<String>,
    #[doc = "The tenantId to which the watchlist item belongs to"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "A flag that indicates if the watchlist item is deleted or not"]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "The time the watchlist item was created"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "The last time the watchlist item was updated"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub updated: Option<time::OffsetDateTime>,
    #[doc = "User information that made some action"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserInfo>,
    #[doc = "User information that made some action"]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<UserInfo>,
    #[doc = "key-value pairs for a watchlist item"]
    #[serde(rename = "itemsKeyValue")]
    pub items_key_value: serde_json::Value,
    #[doc = "key-value pairs for a watchlist item entity mapping"]
    #[serde(rename = "entityMapping", default, skip_serializing_if = "Option::is_none")]
    pub entity_mapping: Option<serde_json::Value>,
}
impl WatchlistItemProperties {
    pub fn new(items_key_value: serde_json::Value) -> Self {
        Self {
            watchlist_item_type: None,
            watchlist_item_id: None,
            tenant_id: None,
            is_deleted: None,
            created: None,
            updated: None,
            created_by: None,
            updated_by: None,
            items_key_value,
            entity_mapping: None,
        }
    }
}
#[doc = "List all the watchlists."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistList {
    #[doc = "URL to fetch the next set of watchlists."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of watchlist."]
    pub value: Vec<Watchlist>,
}
impl azure_core::Continuable for WatchlistList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WatchlistList {
    pub fn new(value: Vec<Watchlist>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Describes watchlist properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistProperties {
    #[doc = "The id (a Guid) of the watchlist"]
    #[serde(rename = "watchlistId", default, skip_serializing_if = "Option::is_none")]
    pub watchlist_id: Option<String>,
    #[doc = "The display name of the watchlist"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The provider of the watchlist"]
    pub provider: String,
    #[doc = "The filename of the watchlist, called 'source'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "The sourceType of the watchlist"]
    #[serde(rename = "sourceType", default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<watchlist_properties::SourceType>,
    #[doc = "The time the watchlist was created"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "The last time the watchlist was updated"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub updated: Option<time::OffsetDateTime>,
    #[doc = "User information that made some action"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserInfo>,
    #[doc = "User information that made some action"]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<UserInfo>,
    #[doc = "A description of the watchlist"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The type of the watchlist"]
    #[serde(rename = "watchlistType", default, skip_serializing_if = "Option::is_none")]
    pub watchlist_type: Option<String>,
    #[doc = "The alias of the watchlist"]
    #[serde(rename = "watchlistAlias", default, skip_serializing_if = "Option::is_none")]
    pub watchlist_alias: Option<String>,
    #[doc = "A flag that indicates if the watchlist is deleted or not"]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "List of labels relevant to this watchlist"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub labels: Vec<Label>,
    #[doc = "The default duration of a watchlist (in ISO 8601 duration format)"]
    #[serde(rename = "defaultDuration", default, skip_serializing_if = "Option::is_none")]
    pub default_duration: Option<String>,
    #[doc = "The tenantId where the watchlist belongs to"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The number of lines in a csv/tsv content to skip before the header"]
    #[serde(rename = "numberOfLinesToSkip", default, skip_serializing_if = "Option::is_none")]
    pub number_of_lines_to_skip: Option<i32>,
    #[doc = "The raw content that represents to watchlist items to create. In case of csv/tsv content type, it's the content of the file that will parsed by the endpoint"]
    #[serde(rename = "rawContent", default, skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    #[doc = "The search key is used to optimize query performance when using watchlists for joins with other data. For example, enable a column with IP addresses to be the designated SearchKey field, then use this field as the key field when joining to other event data by IP address."]
    #[serde(rename = "itemsSearchKey")]
    pub items_search_key: String,
    #[doc = "The content type of the raw content. Example : text/csv or text/tsv "]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "The status of the Watchlist upload : New, InProgress or Complete. Pls note : When a Watchlist upload status is equal to InProgress, the Watchlist cannot be deleted"]
    #[serde(rename = "uploadStatus", default, skip_serializing_if = "Option::is_none")]
    pub upload_status: Option<String>,
}
impl WatchlistProperties {
    pub fn new(display_name: String, provider: String, items_search_key: String) -> Self {
        Self {
            watchlist_id: None,
            display_name,
            provider,
            source: None,
            source_type: None,
            created: None,
            updated: None,
            created_by: None,
            updated_by: None,
            description: None,
            watchlist_type: None,
            watchlist_alias: None,
            is_deleted: None,
            labels: Vec::new(),
            default_duration: None,
            tenant_id: None,
            number_of_lines_to_skip: None,
            raw_content: None,
            items_search_key,
            content_type: None,
            upload_status: None,
        }
    }
}
pub mod watchlist_properties {
    use super::*;
    #[doc = "The sourceType of the watchlist"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SourceType")]
    pub enum SourceType {
        #[serde(rename = "Local file")]
        LocalFile,
        #[serde(rename = "Remote storage")]
        RemoteStorage,
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
                Self::LocalFile => serializer.serialize_unit_variant("SourceType", 0u32, "Local file"),
                Self::RemoteStorage => serializer.serialize_unit_variant("SourceType", 1u32, "Remote storage"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Detail about the webhook object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Webhook {
    #[doc = "Unique identifier for the webhook."]
    #[serde(rename = "webhookId", default, skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
    #[doc = "URL that gets invoked by the webhook."]
    #[serde(rename = "webhookUrl", default, skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[doc = "Time when the webhook secret was updated."]
    #[serde(rename = "webhookSecretUpdateTime", default, with = "azure_core::date::rfc3339::option")]
    pub webhook_secret_update_time: Option<time::OffsetDateTime>,
    #[doc = "A flag to instruct the backend service to rotate webhook secret."]
    #[serde(rename = "rotateWebhookSecret", default, skip_serializing_if = "Option::is_none")]
    pub rotate_webhook_secret: Option<bool>,
}
impl Webhook {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workspace manager assignment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceManagerAssignment {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    #[doc = "The workspace manager assignment properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceManagerAssignmentProperties>,
}
impl WorkspaceManagerAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of all the workspace manager assignments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceManagerAssignmentList {
    #[doc = "URL to fetch the next set of workspace manager assignments."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of workspace manager assignments."]
    pub value: Vec<WorkspaceManagerAssignment>,
}
impl azure_core::Continuable for WorkspaceManagerAssignmentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkspaceManagerAssignmentList {
    pub fn new(value: Vec<WorkspaceManagerAssignment>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "The workspace manager assignment properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceManagerAssignmentProperties {
    #[doc = "The resource name of the workspace manager group targeted by the workspace manager assignment"]
    #[serde(rename = "targetResourceName")]
    pub target_resource_name: String,
    #[doc = "The time the last job associated to this assignment ended at"]
    #[serde(rename = "lastJobEndTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_job_end_time: Option<time::OffsetDateTime>,
    #[doc = "State of the last job associated to this assignment"]
    #[serde(rename = "lastJobProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub last_job_provisioning_state: Option<workspace_manager_assignment_properties::LastJobProvisioningState>,
    #[doc = "List of resources included in this workspace manager assignment"]
    pub items: Vec<AssignmentItem>,
}
impl WorkspaceManagerAssignmentProperties {
    pub fn new(target_resource_name: String, items: Vec<AssignmentItem>) -> Self {
        Self {
            target_resource_name,
            last_job_end_time: None,
            last_job_provisioning_state: None,
            items,
        }
    }
}
pub mod workspace_manager_assignment_properties {
    use super::*;
    #[doc = "State of the last job associated to this assignment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastJobProvisioningState")]
    pub enum LastJobProvisioningState {
        Succeeded,
        InProgress,
        Canceled,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastJobProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastJobProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastJobProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("LastJobProvisioningState", 0u32, "Succeeded"),
                Self::InProgress => serializer.serialize_unit_variant("LastJobProvisioningState", 1u32, "InProgress"),
                Self::Canceled => serializer.serialize_unit_variant("LastJobProvisioningState", 2u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("LastJobProvisioningState", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The workspace manager configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceManagerConfiguration {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    #[doc = "The workspace manager configuration properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceManagerConfigurationProperties>,
}
impl WorkspaceManagerConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List all the workspace manager configurations for the workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceManagerConfigurationList {
    #[doc = "URL to fetch the next set of workspace manager configurations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of workspace manager configurations."]
    pub value: Vec<WorkspaceManagerConfiguration>,
}
impl azure_core::Continuable for WorkspaceManagerConfigurationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkspaceManagerConfigurationList {
    pub fn new(value: Vec<WorkspaceManagerConfiguration>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "The workspace manager configuration properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceManagerConfigurationProperties {
    #[doc = "The current mode of the workspace manager configuration"]
    pub mode: workspace_manager_configuration_properties::Mode,
}
impl WorkspaceManagerConfigurationProperties {
    pub fn new(mode: workspace_manager_configuration_properties::Mode) -> Self {
        Self { mode }
    }
}
pub mod workspace_manager_configuration_properties {
    use super::*;
    #[doc = "The current mode of the workspace manager configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Mode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Mode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Mode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("Mode", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Mode", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The workspace manager group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceManagerGroup {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    #[doc = "The workspace manager group properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceManagerGroupProperties>,
}
impl WorkspaceManagerGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of all the workspace manager groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceManagerGroupList {
    #[doc = "URL to fetch the next set of workspace manager groups."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of workspace manager groups."]
    pub value: Vec<WorkspaceManagerGroup>,
}
impl azure_core::Continuable for WorkspaceManagerGroupList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkspaceManagerGroupList {
    pub fn new(value: Vec<WorkspaceManagerGroup>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "The workspace manager group properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceManagerGroupProperties {
    #[doc = "The description of the workspace manager group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display name of the workspace manager group"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The names of the workspace manager members participating in this group."]
    #[serde(rename = "memberResourceNames")]
    pub member_resource_names: Vec<String>,
}
impl WorkspaceManagerGroupProperties {
    pub fn new(display_name: String, member_resource_names: Vec<String>) -> Self {
        Self {
            description: None,
            display_name,
            member_resource_names,
        }
    }
}
#[doc = "The workspace manager member"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceManagerMember {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    #[doc = "The workspace manager member properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceManagerMemberProperties>,
}
impl WorkspaceManagerMember {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workspace manager member properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceManagerMemberProperties {
    #[doc = "Fully qualified resource ID of the target Sentinel workspace joining the given Sentinel workspace manager"]
    #[serde(rename = "targetWorkspaceResourceId")]
    pub target_workspace_resource_id: String,
    #[doc = "Tenant id of the target Sentinel workspace joining the given Sentinel workspace manager"]
    #[serde(rename = "targetWorkspaceTenantId")]
    pub target_workspace_tenant_id: String,
}
impl WorkspaceManagerMemberProperties {
    pub fn new(target_workspace_resource_id: String, target_workspace_tenant_id: String) -> Self {
        Self {
            target_workspace_resource_id,
            target_workspace_tenant_id,
        }
    }
}
#[doc = "List of workspace manager members"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceManagerMembersList {
    #[doc = "URL to fetch the next set of workspace manager members"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of workspace manager members"]
    pub value: Vec<WorkspaceManagerMember>,
}
impl azure_core::Continuable for WorkspaceManagerMembersList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkspaceManagerMembersList {
    pub fn new(value: Vec<WorkspaceManagerMember>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "An entity describing a content item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentItem {
    #[doc = "The resource id of the content item"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl AssignmentItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error description for why a publication failed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[doc = "The member resource name for which the publication error occured"]
    #[serde(rename = "memberResourceName")]
    pub member_resource_name: String,
    #[doc = "The error message"]
    #[serde(rename = "errorMessage")]
    pub error_message: String,
}
impl Error {
    pub fn new(member_resource_name: String, error_message: String) -> Self {
        Self {
            member_resource_name,
            error_message,
        }
    }
}
pub type FirstPublishDate = String;
#[doc = "An entity describing the publish status of a content item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobItem {
    #[doc = "The resource id of the content item"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Status of the item publication"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<job_item::Status>,
    #[doc = "The time the item publishing was completed"]
    #[serde(rename = "executionTime", default, with = "azure_core::date::rfc3339::option")]
    pub execution_time: Option<time::OffsetDateTime>,
    #[doc = "The list of error descriptions if the item publication fails."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<Error>,
}
impl JobItem {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_item {
    use super::*;
    #[doc = "Status of the item publication"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Succeeded,
        Failed,
        InProgress,
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
                Self::Succeeded => serializer.serialize_unit_variant("Status", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 1u32, "Failed"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 2u32, "InProgress"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type LastPublishDate = String;
#[doc = "Publisher or creator of the content item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataAuthor {
    #[doc = "Name of the author. Company or person."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Email of author contact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Link for author/vendor page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}
impl MetadataAuthor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ies for the solution content item"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataCategories {
    #[doc = "domain for the solution content item"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub domains: Vec<String>,
    #[doc = "Industry verticals for the solution content item"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub verticals: Vec<String>,
}
impl MetadataCategories {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type MetadataContentId = String;
pub type MetadataContentSchemaVersion = String;
pub type MetadataCustomVersion = String;
#[doc = "Dependencies for the content item, what other content items it requires to work.  Can describe more complex dependencies using a recursive/nested structure. For a single dependency an id/kind/version can be supplied or operator/criteria for complex dependencies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataDependencies {
    #[doc = "Static ID for the content.  Used to identify dependencies and content from solutions or community.  Hard-coded/static for out of the box content and solutions. Can be optionally set for user created content to define dependencies.  If an active content item is made from a metadata, both will have the same contentId."]
    #[serde(rename = "contentId", default, skip_serializing_if = "Option::is_none")]
    pub content_id: Option<MetadataContentId>,
    #[doc = "The kind of content the metadata is for."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MetadataKind>,
    #[doc = "Version of the content.  Default and recommended format is numeric (e.g. 1, 1.0, 1.0.0, 1.0.0.0), following ARM template best practices.  Can also be any string, but then we cannot guarantee any version checks"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<MetadataVersion>,
    #[doc = "Name of the content item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Operator used for list of dependencies in criteria array."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<metadata_dependencies::Operator>,
    #[doc = "This is the list of dependencies we must fulfill, according to the AND/OR operator"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub criteria: Vec<MetadataDependencies>,
}
impl MetadataDependencies {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod metadata_dependencies {
    use super::*;
    #[doc = "Operator used for list of dependencies in criteria array."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        #[serde(rename = "AND")]
        And,
        #[serde(rename = "OR")]
        Or,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::And => serializer.serialize_unit_variant("Operator", 0u32, "AND"),
                Self::Or => serializer.serialize_unit_variant("Operator", 1u32, "OR"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type MetadataDisplayName = String;
pub type MetadataFirstPublishDate = String;
pub type MetadataIcon = String;
#[doc = "The kind of content the metadata is for."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MetadataKind")]
pub enum MetadataKind {
    DataConnector,
    DataType,
    Workbook,
    WorkbookTemplate,
    Playbook,
    PlaybookTemplate,
    AnalyticsRuleTemplate,
    AnalyticsRule,
    HuntingQuery,
    InvestigationQuery,
    Parser,
    Watchlist,
    WatchlistTemplate,
    Solution,
    AzureFunction,
    LogicAppsCustomConnector,
    AutomationRule,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MetadataKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MetadataKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MetadataKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DataConnector => serializer.serialize_unit_variant("MetadataKind", 0u32, "DataConnector"),
            Self::DataType => serializer.serialize_unit_variant("MetadataKind", 1u32, "DataType"),
            Self::Workbook => serializer.serialize_unit_variant("MetadataKind", 2u32, "Workbook"),
            Self::WorkbookTemplate => serializer.serialize_unit_variant("MetadataKind", 3u32, "WorkbookTemplate"),
            Self::Playbook => serializer.serialize_unit_variant("MetadataKind", 4u32, "Playbook"),
            Self::PlaybookTemplate => serializer.serialize_unit_variant("MetadataKind", 5u32, "PlaybookTemplate"),
            Self::AnalyticsRuleTemplate => serializer.serialize_unit_variant("MetadataKind", 6u32, "AnalyticsRuleTemplate"),
            Self::AnalyticsRule => serializer.serialize_unit_variant("MetadataKind", 7u32, "AnalyticsRule"),
            Self::HuntingQuery => serializer.serialize_unit_variant("MetadataKind", 8u32, "HuntingQuery"),
            Self::InvestigationQuery => serializer.serialize_unit_variant("MetadataKind", 9u32, "InvestigationQuery"),
            Self::Parser => serializer.serialize_unit_variant("MetadataKind", 10u32, "Parser"),
            Self::Watchlist => serializer.serialize_unit_variant("MetadataKind", 11u32, "Watchlist"),
            Self::WatchlistTemplate => serializer.serialize_unit_variant("MetadataKind", 12u32, "WatchlistTemplate"),
            Self::Solution => serializer.serialize_unit_variant("MetadataKind", 13u32, "Solution"),
            Self::AzureFunction => serializer.serialize_unit_variant("MetadataKind", 14u32, "AzureFunction"),
            Self::LogicAppsCustomConnector => serializer.serialize_unit_variant("MetadataKind", 15u32, "LogicAppsCustomConnector"),
            Self::AutomationRule => serializer.serialize_unit_variant("MetadataKind", 16u32, "AutomationRule"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type MetadataLastPublishDate = String;
#[doc = "The package kind"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MetadataPackageKind")]
pub enum MetadataPackageKind {
    Solution,
    Standalone,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MetadataPackageKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MetadataPackageKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MetadataPackageKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Solution => serializer.serialize_unit_variant("MetadataPackageKind", 0u32, "Solution"),
            Self::Standalone => serializer.serialize_unit_variant("MetadataPackageKind", 1u32, "Standalone"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type MetadataParentId = String;
#[doc = "Metadata patch request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataPatch {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "Metadata property bag for patch requests.  This is the same as the MetadataProperties, but with nothing required"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetadataPropertiesPatch>,
}
impl MetadataPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type MetadataPreviewImages = Vec<String>;
pub type MetadataPreviewImagesDark = Vec<String>;
#[doc = "Metadata property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataProperties {
    #[doc = "Static ID for the content.  Used to identify dependencies and content from solutions or community.  Hard-coded/static for out of the box content and solutions. Can be optionally set for user created content to define dependencies.  If an active content item is made from a template, both will have the same contentId."]
    #[serde(rename = "contentId", default, skip_serializing_if = "Option::is_none")]
    pub content_id: Option<MetadataContentId>,
    #[doc = "Full parent resource ID of the content item the metadata is for.  This is the full resource ID including the scope (subscription and resource group)"]
    #[serde(rename = "parentId")]
    pub parent_id: MetadataParentId,
    #[doc = "Version of the content.  Default and recommended format is numeric (e.g. 1, 1.0, 1.0.0, 1.0.0.0), following ARM template best practices.  Can also be any string, but then we cannot guarantee any version checks"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<MetadataVersion>,
    #[doc = "The kind of content the metadata is for."]
    pub kind: MetadataKind,
    #[doc = "The original source of the content item, where it comes from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<MetadataSource>,
    #[doc = "Publisher or creator of the content item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<MetadataAuthor>,
    #[doc = "Support information for the content item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support: Option<MetadataSupport>,
    #[doc = "Dependencies for the content item, what other content items it requires to work.  Can describe more complex dependencies using a recursive/nested structure. For a single dependency an id/kind/version can be supplied or operator/criteria for complex dependencies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<MetadataDependencies>,
    #[doc = "ies for the solution content item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<MetadataCategories>,
    #[doc = "Providers for the solution content item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub providers: Option<MetadataProviders>,
    #[doc = "first publish date of solution content item"]
    #[serde(rename = "firstPublishDate", default, skip_serializing_if = "Option::is_none")]
    pub first_publish_date: Option<FirstPublishDate>,
    #[doc = "last publish date of solution content item"]
    #[serde(rename = "lastPublishDate", default, skip_serializing_if = "Option::is_none")]
    pub last_publish_date: Option<LastPublishDate>,
    #[doc = "The custom version of the content. A optional free text"]
    #[serde(rename = "customVersion", default, skip_serializing_if = "Option::is_none")]
    pub custom_version: Option<MetadataCustomVersion>,
    #[doc = "Schema version of the content. Can be used to distinguish between different flow based on the schema version"]
    #[serde(rename = "contentSchemaVersion", default, skip_serializing_if = "Option::is_none")]
    pub content_schema_version: Option<MetadataContentSchemaVersion>,
    #[doc = "the icon identifier. this id can later be fetched from the solution template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<MetadataIcon>,
    #[doc = "the tactics the resource covers"]
    #[serde(rename = "threatAnalysisTactics", default, skip_serializing_if = "Option::is_none")]
    pub threat_analysis_tactics: Option<MetadataThreatAnalysisTactics>,
    #[doc = "the techniques the resource covers, these have to be aligned with the tactics being used"]
    #[serde(rename = "threatAnalysisTechniques", default, skip_serializing_if = "Option::is_none")]
    pub threat_analysis_techniques: Option<MetadataThreatAnalysisTechniques>,
    #[doc = "preview image file names. These will be taken from the solution artifacts"]
    #[serde(rename = "previewImages", default, skip_serializing_if = "Option::is_none")]
    pub preview_images: Option<MetadataPreviewImages>,
    #[doc = "preview image file names. These will be taken from the solution artifacts. used for dark theme support"]
    #[serde(rename = "previewImagesDark", default, skip_serializing_if = "Option::is_none")]
    pub preview_images_dark: Option<MetadataPreviewImagesDark>,
}
impl MetadataProperties {
    pub fn new(parent_id: MetadataParentId, kind: MetadataKind) -> Self {
        Self {
            content_id: None,
            parent_id,
            version: None,
            kind,
            source: None,
            author: None,
            support: None,
            dependencies: None,
            categories: None,
            providers: None,
            first_publish_date: None,
            last_publish_date: None,
            custom_version: None,
            content_schema_version: None,
            icon: None,
            threat_analysis_tactics: None,
            threat_analysis_techniques: None,
            preview_images: None,
            preview_images_dark: None,
        }
    }
}
#[doc = "Metadata property bag for patch requests.  This is the same as the MetadataProperties, but with nothing required"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataPropertiesPatch {
    #[doc = "Static ID for the content.  Used to identify dependencies and content from solutions or community.  Hard-coded/static for out of the box content and solutions. Can be optionally set for user created content to define dependencies.  If an active content item is made from a template, both will have the same contentId."]
    #[serde(rename = "contentId", default, skip_serializing_if = "Option::is_none")]
    pub content_id: Option<MetadataContentId>,
    #[doc = "Full parent resource ID of the content item the metadata is for.  This is the full resource ID including the scope (subscription and resource group)"]
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<MetadataParentId>,
    #[doc = "Version of the content.  Default and recommended format is numeric (e.g. 1, 1.0, 1.0.0, 1.0.0.0), following ARM template best practices.  Can also be any string, but then we cannot guarantee any version checks"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<MetadataVersion>,
    #[doc = "The kind of content the metadata is for."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MetadataKind>,
    #[doc = "The original source of the content item, where it comes from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<MetadataSource>,
    #[doc = "Publisher or creator of the content item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<MetadataAuthor>,
    #[doc = "Support information for the content item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support: Option<MetadataSupport>,
    #[doc = "Dependencies for the content item, what other content items it requires to work.  Can describe more complex dependencies using a recursive/nested structure. For a single dependency an id/kind/version can be supplied or operator/criteria for complex dependencies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<MetadataDependencies>,
    #[doc = "ies for the solution content item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<MetadataCategories>,
    #[doc = "Providers for the solution content item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub providers: Option<MetadataProviders>,
    #[doc = "first publish date of solution content item"]
    #[serde(rename = "firstPublishDate", default, skip_serializing_if = "Option::is_none")]
    pub first_publish_date: Option<FirstPublishDate>,
    #[doc = "last publish date of solution content item"]
    #[serde(rename = "lastPublishDate", default, skip_serializing_if = "Option::is_none")]
    pub last_publish_date: Option<LastPublishDate>,
    #[doc = "The custom version of the content. A optional free text"]
    #[serde(rename = "customVersion", default, skip_serializing_if = "Option::is_none")]
    pub custom_version: Option<MetadataCustomVersion>,
    #[doc = "Schema version of the content. Can be used to distinguish between different flow based on the schema version"]
    #[serde(rename = "contentSchemaVersion", default, skip_serializing_if = "Option::is_none")]
    pub content_schema_version: Option<MetadataContentSchemaVersion>,
    #[doc = "the icon identifier. this id can later be fetched from the solution template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<MetadataIcon>,
    #[doc = "the tactics the resource covers"]
    #[serde(rename = "threatAnalysisTactics", default, skip_serializing_if = "Option::is_none")]
    pub threat_analysis_tactics: Option<MetadataThreatAnalysisTactics>,
    #[doc = "the techniques the resource covers, these have to be aligned with the tactics being used"]
    #[serde(rename = "threatAnalysisTechniques", default, skip_serializing_if = "Option::is_none")]
    pub threat_analysis_techniques: Option<MetadataThreatAnalysisTechniques>,
    #[doc = "preview image file names. These will be taken from the solution artifacts"]
    #[serde(rename = "previewImages", default, skip_serializing_if = "Option::is_none")]
    pub preview_images: Option<MetadataPreviewImages>,
    #[doc = "preview image file names. These will be taken from the solution artifacts. used for dark theme support"]
    #[serde(rename = "previewImagesDark", default, skip_serializing_if = "Option::is_none")]
    pub preview_images_dark: Option<MetadataPreviewImagesDark>,
}
impl MetadataPropertiesPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type MetadataProviders = Vec<String>;
#[doc = "The original source of the content item, where it comes from."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataSource {
    #[doc = "Source type of the content"]
    pub kind: metadata_source::Kind,
    #[doc = "Name of the content source.  The repo name, solution name, LA workspace name etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "ID of the content source.  The solution ID, workspace ID, etc"]
    #[serde(rename = "sourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
}
impl MetadataSource {
    pub fn new(kind: metadata_source::Kind) -> Self {
        Self {
            kind,
            name: None,
            source_id: None,
        }
    }
}
pub mod metadata_source {
    use super::*;
    #[doc = "Source type of the content"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        LocalWorkspace,
        Community,
        Solution,
        SourceRepository,
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
                Self::LocalWorkspace => serializer.serialize_unit_variant("Kind", 0u32, "LocalWorkspace"),
                Self::Community => serializer.serialize_unit_variant("Kind", 1u32, "Community"),
                Self::Solution => serializer.serialize_unit_variant("Kind", 2u32, "Solution"),
                Self::SourceRepository => serializer.serialize_unit_variant("Kind", 3u32, "SourceRepository"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Support information for the content item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataSupport {
    #[doc = "Type of support for content item"]
    pub tier: metadata_support::Tier,
    #[doc = "Name of the support contact. Company or person."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Email of support contact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Link for support help, like to support page to open a ticket etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}
impl MetadataSupport {
    pub fn new(tier: metadata_support::Tier) -> Self {
        Self {
            tier,
            name: None,
            email: None,
            link: None,
        }
    }
}
pub mod metadata_support {
    use super::*;
    #[doc = "Type of support for content item"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Microsoft,
        Partner,
        Community,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Microsoft => serializer.serialize_unit_variant("Tier", 0u32, "Microsoft"),
                Self::Partner => serializer.serialize_unit_variant("Tier", 1u32, "Partner"),
                Self::Community => serializer.serialize_unit_variant("Tier", 2u32, "Community"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type MetadataThreatAnalysisTactics = Vec<String>;
pub type MetadataThreatAnalysisTechniques = Vec<String>;
#[doc = "The boolean value the metadata is for."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MetadataTrueFalseFlag")]
pub enum MetadataTrueFalseFlag {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MetadataTrueFalseFlag {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MetadataTrueFalseFlag {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MetadataTrueFalseFlag {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::True => serializer.serialize_unit_variant("MetadataTrueFalseFlag", 0u32, "true"),
            Self::False => serializer.serialize_unit_variant("MetadataTrueFalseFlag", 1u32, "false"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type MetadataVersion = String;
#[doc = "List available packages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageList {
    #[doc = "URL to fetch the next set of packages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of packages."]
    pub value: Vec<PackageModel>,
}
impl azure_core::Continuable for PackageList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PackageList {
    pub fn new(value: Vec<PackageModel>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Represents a Package in Azure Security Insights."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageModel {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "Describes package properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PackageProperties>,
}
impl PackageModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes package properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageProperties {
    #[doc = "Static ID for the content.  Used to identify dependencies and content from solutions or community.  Hard-coded/static for out of the box content and solutions. Can be optionally set for user created content to define dependencies.  If an active content item is made from a metadata, both will have the same contentId."]
    #[serde(rename = "contentId")]
    pub content_id: MetadataContentId,
    #[doc = "The package kind"]
    #[serde(rename = "contentKind")]
    pub content_kind: MetadataPackageKind,
    #[doc = "Version of the content.  Default and recommended format is numeric (e.g. 1, 1.0, 1.0.0, 1.0.0.0), following ARM template best practices.  Can also be any string, but then we cannot guarantee any version checks"]
    #[serde(rename = "contentSchemaVersion", default, skip_serializing_if = "Option::is_none")]
    pub content_schema_version: Option<MetadataVersion>,
    #[doc = "The boolean value the metadata is for."]
    #[serde(rename = "isNew", default, skip_serializing_if = "Option::is_none")]
    pub is_new: Option<MetadataTrueFalseFlag>,
    #[doc = "The boolean value the metadata is for."]
    #[serde(rename = "isPreview", default, skip_serializing_if = "Option::is_none")]
    pub is_preview: Option<MetadataTrueFalseFlag>,
    #[doc = "The boolean value the metadata is for."]
    #[serde(rename = "isFeatured", default, skip_serializing_if = "Option::is_none")]
    pub is_featured: Option<MetadataTrueFalseFlag>,
    #[doc = "Version of the content.  Default and recommended format is numeric (e.g. 1, 1.0, 1.0.0, 1.0.0.0), following ARM template best practices.  Can also be any string, but then we cannot guarantee any version checks"]
    pub version: MetadataVersion,
    #[doc = "DisplayName of the content."]
    #[serde(rename = "displayName")]
    pub display_name: MetadataDisplayName,
    #[doc = "The description of the package"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "DisplayName of the content."]
    #[serde(rename = "publisherDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_display_name: Option<MetadataDisplayName>,
    #[doc = "The original source of the content item, where it comes from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<MetadataSource>,
    #[doc = "Publisher or creator of the content item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<MetadataAuthor>,
    #[doc = "Support information for the content item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support: Option<MetadataSupport>,
    #[doc = "Dependencies for the content item, what other content items it requires to work.  Can describe more complex dependencies using a recursive/nested structure. For a single dependency an id/kind/version can be supplied or operator/criteria for complex dependencies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<MetadataDependencies>,
    #[doc = "Providers for the solution content item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub providers: Option<MetadataProviders>,
    #[doc = "first publish date of solution content item"]
    #[serde(rename = "firstPublishDate", default, skip_serializing_if = "Option::is_none")]
    pub first_publish_date: Option<MetadataFirstPublishDate>,
    #[doc = "last publish date of solution content item"]
    #[serde(rename = "lastPublishDate", default, skip_serializing_if = "Option::is_none")]
    pub last_publish_date: Option<MetadataLastPublishDate>,
    #[doc = "ies for the solution content item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<MetadataCategories>,
    #[doc = "the tactics the resource covers"]
    #[serde(rename = "threatAnalysisTactics", default, skip_serializing_if = "Option::is_none")]
    pub threat_analysis_tactics: Option<MetadataThreatAnalysisTactics>,
    #[doc = "the techniques the resource covers, these have to be aligned with the tactics being used"]
    #[serde(rename = "threatAnalysisTechniques", default, skip_serializing_if = "Option::is_none")]
    pub threat_analysis_techniques: Option<MetadataThreatAnalysisTechniques>,
    #[doc = "the icon identifier. this id can later be fetched from the metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<MetadataIcon>,
}
impl PackageProperties {
    pub fn new(
        content_id: MetadataContentId,
        content_kind: MetadataPackageKind,
        version: MetadataVersion,
        display_name: MetadataDisplayName,
    ) -> Self {
        Self {
            content_id,
            content_kind,
            content_schema_version: None,
            is_new: None,
            is_preview: None,
            is_featured: None,
            version,
            display_name,
            description: None,
            publisher_display_name: None,
            source: None,
            author: None,
            support: None,
            dependencies: None,
            providers: None,
            first_publish_date: None,
            last_publish_date: None,
            categories: None,
            threat_analysis_tactics: None,
            threat_analysis_techniques: None,
            icon: None,
        }
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
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
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreatedByType")]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreatedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreatedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreatedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("CreatedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("CreatedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("CreatedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("CreatedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastModifiedByType")]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastModifiedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastModifiedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastModifiedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("LastModifiedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("LastModifiedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("LastModifiedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("LastModifiedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of all the template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateList {
    #[doc = "Array of templates."]
    pub value: Vec<TemplateModel>,
    #[doc = "URL to fetch the next page of template."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TemplateList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TemplateList {
    pub fn new(value: Vec<TemplateModel>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Template resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TemplateModel {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[doc = "Template property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TemplateProperties>,
}
impl TemplateModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Template property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateProperties {
    #[doc = "Static ID for the content.  Used to identify dependencies and content from solutions or community.  Hard-coded/static for out of the box content and solutions. Can be optionally set for user created content to define dependencies.  If an active content item is made from a metadata, both will have the same contentId."]
    #[serde(rename = "contentId")]
    pub content_id: MetadataContentId,
    #[doc = "Full parent resource ID of the content item the metadata is for.  This is the full resource ID including the scope (subscription and resource group)"]
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<MetadataParentId>,
    #[doc = "Version of the content.  Default and recommended format is numeric (e.g. 1, 1.0, 1.0.0, 1.0.0.0), following ARM template best practices.  Can also be any string, but then we cannot guarantee any version checks"]
    pub version: MetadataVersion,
    #[doc = "DisplayName of the content."]
    #[serde(rename = "displayName")]
    pub display_name: MetadataDisplayName,
    #[doc = "The kind of content the metadata is for."]
    #[serde(rename = "contentKind")]
    pub content_kind: MetadataKind,
    #[doc = "The original source of the content item, where it comes from."]
    pub source: MetadataSource,
    #[doc = "Publisher or creator of the content item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<MetadataAuthor>,
    #[doc = "Support information for the content item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support: Option<MetadataSupport>,
    #[doc = "Dependencies for the content item, what other content items it requires to work.  Can describe more complex dependencies using a recursive/nested structure. For a single dependency an id/kind/version can be supplied or operator/criteria for complex dependencies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<MetadataDependencies>,
    #[doc = "ies for the solution content item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<MetadataCategories>,
    #[doc = "Providers for the solution content item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub providers: Option<MetadataProviders>,
    #[doc = "first publish date of solution content item"]
    #[serde(rename = "firstPublishDate", default, skip_serializing_if = "Option::is_none")]
    pub first_publish_date: Option<MetadataFirstPublishDate>,
    #[doc = "last publish date of solution content item"]
    #[serde(rename = "lastPublishDate", default, skip_serializing_if = "Option::is_none")]
    pub last_publish_date: Option<MetadataLastPublishDate>,
    #[doc = "The custom version of the content. A optional free text"]
    #[serde(rename = "customVersion", default, skip_serializing_if = "Option::is_none")]
    pub custom_version: Option<MetadataCustomVersion>,
    #[doc = "Schema version of the content. Can be used to distinguish between different flow based on the schema version"]
    #[serde(rename = "contentSchemaVersion", default, skip_serializing_if = "Option::is_none")]
    pub content_schema_version: Option<MetadataContentSchemaVersion>,
    #[doc = "the icon identifier. this id can later be fetched from the metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<MetadataIcon>,
    #[doc = "the tactics the resource covers"]
    #[serde(rename = "threatAnalysisTactics", default, skip_serializing_if = "Option::is_none")]
    pub threat_analysis_tactics: Option<MetadataThreatAnalysisTactics>,
    #[doc = "the techniques the resource covers, these have to be aligned with the tactics being used"]
    #[serde(rename = "threatAnalysisTechniques", default, skip_serializing_if = "Option::is_none")]
    pub threat_analysis_techniques: Option<MetadataThreatAnalysisTechniques>,
    #[doc = "preview image file names. These will be taken from the solution artifacts"]
    #[serde(rename = "previewImages", default, skip_serializing_if = "Option::is_none")]
    pub preview_images: Option<MetadataPreviewImages>,
    #[doc = "preview image file names. These will be taken from the solution artifacts. used for dark theme support"]
    #[serde(rename = "previewImagesDark", default, skip_serializing_if = "Option::is_none")]
    pub preview_images_dark: Option<MetadataPreviewImagesDark>,
    #[doc = "Static ID for the content.  Used to identify dependencies and content from solutions or community.  Hard-coded/static for out of the box content and solutions. Can be optionally set for user created content to define dependencies.  If an active content item is made from a metadata, both will have the same contentId."]
    #[serde(rename = "packageId", default, skip_serializing_if = "Option::is_none")]
    pub package_id: Option<MetadataContentId>,
    #[doc = "The package kind"]
    #[serde(rename = "packageKind", default, skip_serializing_if = "Option::is_none")]
    pub package_kind: Option<MetadataPackageKind>,
    #[doc = "DisplayName of the content."]
    #[serde(rename = "packageName", default, skip_serializing_if = "Option::is_none")]
    pub package_name: Option<MetadataDisplayName>,
    #[doc = "the json to deploy"]
    #[serde(rename = "packagedContent", default, skip_serializing_if = "Option::is_none")]
    pub packaged_content: Option<serde_json::Value>,
}
impl TemplateProperties {
    pub fn new(
        content_id: MetadataContentId,
        version: MetadataVersion,
        display_name: MetadataDisplayName,
        content_kind: MetadataKind,
        source: MetadataSource,
    ) -> Self {
        Self {
            content_id,
            parent_id: None,
            version,
            display_name,
            content_kind,
            source,
            author: None,
            support: None,
            dependencies: None,
            categories: None,
            providers: None,
            first_publish_date: None,
            last_publish_date: None,
            custom_version: None,
            content_schema_version: None,
            icon: None,
            threat_analysis_tactics: None,
            threat_analysis_techniques: None,
            preview_images: None,
            preview_images_dark: None,
            package_id: None,
            package_kind: None,
            package_name: None,
            packaged_content: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TriggersOn")]
pub enum TriggersOn {
    Incidents,
    Alerts,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TriggersOn {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TriggersOn {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TriggersOn {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Incidents => serializer.serialize_unit_variant("TriggersOn", 0u32, "Incidents"),
            Self::Alerts => serializer.serialize_unit_variant("TriggersOn", 1u32, "Alerts"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TriggersWhen")]
pub enum TriggersWhen {
    Created,
    Updated,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TriggersWhen {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TriggersWhen {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TriggersWhen {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Created => serializer.serialize_unit_variant("TriggersWhen", 0u32, "Created"),
            Self::Updated => serializer.serialize_unit_variant("TriggersWhen", 1u32, "Updated"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
