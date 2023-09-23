#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
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
#[doc = "Details of a monitor service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorServiceDetails {
    #[doc = "Monitor service name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Monitor service display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl MonitorServiceDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Monitor service details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorServiceList {
    #[serde(flatten)]
    pub alerts_meta_data_properties: AlertsMetaDataProperties,
    #[doc = "Array of operations"]
    pub data: Vec<MonitorServiceDetails>,
}
impl MonitorServiceList {
    pub fn new(alerts_meta_data_properties: AlertsMetaDataProperties, data: Vec<MonitorServiceDetails>) -> Self {
        Self {
            alerts_meta_data_properties,
            data,
        }
    }
}
#[doc = "Prometheus enrichment object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrometheusEnrichmentItem {
    #[serde(flatten)]
    pub alert_enrichment_item: AlertEnrichmentItem,
    #[doc = "Link to Prometheus query API (Url format)."]
    #[serde(rename = "linkToApi")]
    pub link_to_api: String,
    #[doc = "An array of the azure monitor workspace resource ids."]
    pub datasources: Vec<String>,
    #[doc = "Partial link to the Grafana explore API."]
    #[serde(rename = "grafanaExplorePath")]
    pub grafana_explore_path: String,
    #[doc = "The Prometheus expression query."]
    pub query: String,
}
impl PrometheusEnrichmentItem {
    pub fn new(
        alert_enrichment_item: AlertEnrichmentItem,
        link_to_api: String,
        datasources: Vec<String>,
        grafana_explore_path: String,
        query: String,
    ) -> Self {
        Self {
            alert_enrichment_item,
            link_to_api,
            datasources,
            grafana_explore_path,
            query,
        }
    }
}
#[doc = "Prometheus instant query enrichment object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrometheusInstantQuery {
    #[serde(flatten)]
    pub prometheus_enrichment_item: PrometheusEnrichmentItem,
    #[doc = "The date and the time of the evaluation."]
    pub time: String,
}
impl PrometheusInstantQuery {
    pub fn new(prometheus_enrichment_item: PrometheusEnrichmentItem, time: String) -> Self {
        Self {
            prometheus_enrichment_item,
            time,
        }
    }
}
#[doc = "Prometheus instant query enrichment object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrometheusRangeQuery {
    #[serde(flatten)]
    pub prometheus_enrichment_item: PrometheusEnrichmentItem,
    #[doc = "The start evaluation date and time in ISO8601 format."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub start: time::OffsetDateTime,
    #[doc = "The end evaluation date and time in ISO8601 format."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub end: time::OffsetDateTime,
    #[doc = "Query resolution step width in ISO8601 format."]
    pub step: String,
}
impl PrometheusRangeQuery {
    pub fn new(
        prometheus_enrichment_item: PrometheusEnrichmentItem,
        start: time::OffsetDateTime,
        end: time::OffsetDateTime,
        step: String,
    ) -> Self {
        Self {
            prometheus_enrichment_item,
            start,
            end,
            step,
        }
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Action status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionStatus {
    #[doc = "Value indicating whether alert is suppressed."]
    #[serde(rename = "isSuppressed", default, skip_serializing_if = "Option::is_none")]
    pub is_suppressed: Option<bool>,
}
impl ActionStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An alert created in alert management service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Alert {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Alert property bag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProperties>,
}
impl Alert {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information specific to the monitor service that gives more contextual details about the alert."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertContext {}
impl AlertContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert enrichment item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertEnrichmentItem {
    #[doc = "The enrichment title."]
    pub title: String,
    #[doc = "The enrichment description."]
    pub description: String,
    #[doc = "The status of the evaluation of the enrichment."]
    pub status: alert_enrichment_item::Status,
    #[doc = "The error message. Will be present only if the status is 'Failed'."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The enrichment type."]
    #[serde(rename = "type")]
    pub type_: alert_enrichment_item::Type,
}
impl AlertEnrichmentItem {
    pub fn new(title: String, description: String, status: alert_enrichment_item::Status, type_: alert_enrichment_item::Type) -> Self {
        Self {
            title,
            description,
            status,
            error_message: None,
            type_,
        }
    }
}
pub mod alert_enrichment_item {
    use super::*;
    #[doc = "The status of the evaluation of the enrichment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Succeeded,
        Failed,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The enrichment type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        PrometheusInstantQuery,
        PrometheusRangeQuery,
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
                Self::PrometheusInstantQuery => serializer.serialize_unit_variant("Type", 0u32, "PrometheusInstantQuery"),
                Self::PrometheusRangeQuery => serializer.serialize_unit_variant("Type", 1u32, "PrometheusRangeQuery"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AlertEnrichmentItemUnion {}
#[doc = "Properties of the alert enrichment item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertEnrichmentProperties {
    #[doc = "Unique Id (GUID) of the alert for which the enrichments are being retrieved."]
    #[serde(rename = "alertId", default, skip_serializing_if = "Option::is_none")]
    pub alert_id: Option<String>,
    #[doc = "Enrichment details"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enrichments: Vec<AlertEnrichmentItemUnion>,
}
impl AlertEnrichmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The alert's enrichments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertEnrichmentResponse {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the alert enrichment item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertEnrichmentProperties>,
}
impl AlertEnrichmentResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List the alert's enrichments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertEnrichmentsList {
    #[doc = "List the alert's enrichments"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AlertEnrichmentResponse>,
    #[doc = "Request URL that can be used to query next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AlertEnrichmentsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AlertEnrichmentsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert Modification details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertModification {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the alert modification item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertModificationProperties>,
}
impl AlertModification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert modification item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertModificationItem {
    #[doc = "Reason for the modification"]
    #[serde(rename = "modificationEvent", default, skip_serializing_if = "Option::is_none")]
    pub modification_event: Option<alert_modification_item::ModificationEvent>,
    #[doc = "Old value"]
    #[serde(rename = "oldValue", default, skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
    #[doc = "New value"]
    #[serde(rename = "newValue", default, skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,
    #[doc = "Modified date and time"]
    #[serde(rename = "modifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[doc = "Modified user details (Principal client name)"]
    #[serde(rename = "modifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    #[doc = "Modification comments"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[doc = "Description of the modification"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AlertModificationItem {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod alert_modification_item {
    use super::*;
    #[doc = "Reason for the modification"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ModificationEvent {
        AlertCreated,
        StateChange,
        MonitorConditionChange,
        SeverityChange,
        ActionRuleTriggered,
        ActionRuleSuppressed,
        ActionsTriggered,
        ActionsSuppressed,
        ActionsFailed,
    }
}
#[doc = "Properties of the alert modification item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertModificationProperties {
    #[doc = "Unique Id of the alert for which the history is being retrieved"]
    #[serde(rename = "alertId", default, skip_serializing_if = "Option::is_none")]
    pub alert_id: Option<String>,
    #[doc = "Modification details"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub modifications: Vec<AlertModificationItem>,
}
impl AlertModificationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert property bag"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertProperties {
    #[doc = "This object contains consistent fields across different monitor services."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub essentials: Option<Essentials>,
    #[doc = "Information specific to the monitor service that gives more contextual details about the alert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<AlertContext>,
    #[doc = "Config which would be used for displaying the data in portal."]
    #[serde(rename = "egressConfig", default, skip_serializing_if = "Option::is_none")]
    pub egress_config: Option<EgressConfig>,
}
impl AlertProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List the alerts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertsList {
    #[doc = "URL to fetch the next set of alerts."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of alerts"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Alert>,
}
impl azure_core::Continuable for AlertsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AlertsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "alert meta data information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertsMetaData {
    #[doc = "alert meta data property bag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertsMetaDataPropertiesUnion>,
}
impl AlertsMetaData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "alert meta data property bag"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsMetaDataProperties {
    #[doc = "Identification of the information to be retrieved by API call"]
    #[serde(rename = "metadataIdentifier")]
    pub metadata_identifier: alerts_meta_data_properties::MetadataIdentifier,
}
impl AlertsMetaDataProperties {
    pub fn new(metadata_identifier: alerts_meta_data_properties::MetadataIdentifier) -> Self {
        Self { metadata_identifier }
    }
}
pub mod alerts_meta_data_properties {
    use super::*;
    #[doc = "Identification of the information to be retrieved by API call"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MetadataIdentifier")]
    pub enum MetadataIdentifier {
        MonitorServiceList,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MetadataIdentifier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MetadataIdentifier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MetadataIdentifier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MonitorServiceList => serializer.serialize_unit_variant("MetadataIdentifier", 0u32, "MonitorServiceList"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "metadataIdentifier")]
pub enum AlertsMetaDataPropertiesUnion {}
#[doc = "Summary of alerts based on the input filters and 'groupby' parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertsSummary {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Group the result set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertsSummaryGroup>,
}
impl AlertsSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Group the result set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertsSummaryGroup {
    #[doc = "Total count of the result set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[doc = "Total count of the smart groups."]
    #[serde(rename = "smartGroupsCount", default, skip_serializing_if = "Option::is_none")]
    pub smart_groups_count: Option<i64>,
    #[doc = "Name of the field aggregated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groupedby: Option<String>,
    #[doc = "List of the items"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<AlertsSummaryGroupItem>,
}
impl AlertsSummaryGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alerts summary group item"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertsSummaryGroupItem {
    #[doc = "Value of the aggregated field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Count of the aggregated field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Name of the field aggregated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groupedby: Option<String>,
    #[doc = "List of the items"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<AlertsSummaryGroupItem>,
}
impl AlertsSummaryGroupItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Change alert state reason"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Comments {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}
impl Comments {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Config which would be used for displaying the data in portal."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EgressConfig {}
impl EgressConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Details of error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseBody>,
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
#[doc = "Details of error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseBody {
    #[doc = "Error code, intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Description of the error, intended for display in user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Target of the particular error, for example name of the property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorResponseBody>,
}
impl ErrorResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This object contains consistent fields across different monitor services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Essentials {
    #[doc = "Severity of alert Sev0 being highest and Sev4 being lowest."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<essentials::Severity>,
    #[doc = "The type of signal the alert is based on, which could be metrics, logs or activity logs."]
    #[serde(rename = "signalType", default, skip_serializing_if = "Option::is_none")]
    pub signal_type: Option<essentials::SignalType>,
    #[doc = "Alert object state, which can be modified by the user."]
    #[serde(rename = "alertState", default, skip_serializing_if = "Option::is_none")]
    pub alert_state: Option<essentials::AlertState>,
    #[doc = "Condition of the rule at the monitor service. It represents whether the underlying conditions have crossed the defined alert rule thresholds."]
    #[serde(rename = "monitorCondition", default, skip_serializing_if = "Option::is_none")]
    pub monitor_condition: Option<essentials::MonitorCondition>,
    #[doc = "Target ARM resource, on which alert got created."]
    #[serde(rename = "targetResource", default, skip_serializing_if = "Option::is_none")]
    pub target_resource: Option<String>,
    #[doc = "Name of the target ARM resource name, on which alert got created."]
    #[serde(rename = "targetResourceName", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_name: Option<String>,
    #[doc = "Resource group of target ARM resource, on which alert got created."]
    #[serde(rename = "targetResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_group: Option<String>,
    #[doc = "Resource type of target ARM resource, on which alert got created."]
    #[serde(rename = "targetResourceType", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<String>,
    #[doc = "Monitor service on which the rule(monitor) is set."]
    #[serde(rename = "monitorService", default, skip_serializing_if = "Option::is_none")]
    pub monitor_service: Option<essentials::MonitorService>,
    #[doc = "Rule(monitor) which fired alert instance. Depending on the monitor service,  this would be ARM id or name of the rule."]
    #[serde(rename = "alertRule", default, skip_serializing_if = "Option::is_none")]
    pub alert_rule: Option<String>,
    #[doc = "Unique Id created by monitor service for each alert instance. This could be used to track the issue at the monitor service, in case of Nagios, Zabbix, SCOM etc."]
    #[serde(rename = "sourceCreatedId", default, skip_serializing_if = "Option::is_none")]
    pub source_created_id: Option<String>,
    #[doc = "Unique Id of the smart group"]
    #[serde(rename = "smartGroupId", default, skip_serializing_if = "Option::is_none")]
    pub smart_group_id: Option<String>,
    #[doc = "Verbose reason describing the reason why this alert instance is added to a smart group"]
    #[serde(rename = "smartGroupingReason", default, skip_serializing_if = "Option::is_none")]
    pub smart_grouping_reason: Option<String>,
    #[doc = "Creation time(ISO-8601 format) of alert instance."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "Last modification time(ISO-8601 format) of alert instance."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Resolved time(ISO-8601 format) of alert instance. This will be updated when monitor service resolves the alert instance because the rule condition is no longer met."]
    #[serde(rename = "monitorConditionResolvedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub monitor_condition_resolved_date_time: Option<time::OffsetDateTime>,
    #[doc = "User who last modified the alert, in case of monitor service updates user would be 'system', otherwise name of the user."]
    #[serde(rename = "lastModifiedUserName", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_user_name: Option<String>,
    #[doc = "Action status"]
    #[serde(rename = "actionStatus", default, skip_serializing_if = "Option::is_none")]
    pub action_status: Option<ActionStatus>,
    #[doc = "Alert description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl Essentials {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod essentials {
    use super::*;
    #[doc = "Severity of alert Sev0 being highest and Sev4 being lowest."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Severity")]
    pub enum Severity {
        Sev0,
        Sev1,
        Sev2,
        Sev3,
        Sev4,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Severity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Severity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Severity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Sev0 => serializer.serialize_unit_variant("Severity", 0u32, "Sev0"),
                Self::Sev1 => serializer.serialize_unit_variant("Severity", 1u32, "Sev1"),
                Self::Sev2 => serializer.serialize_unit_variant("Severity", 2u32, "Sev2"),
                Self::Sev3 => serializer.serialize_unit_variant("Severity", 3u32, "Sev3"),
                Self::Sev4 => serializer.serialize_unit_variant("Severity", 4u32, "Sev4"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of signal the alert is based on, which could be metrics, logs or activity logs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SignalType")]
    pub enum SignalType {
        Metric,
        Log,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SignalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SignalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SignalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Metric => serializer.serialize_unit_variant("SignalType", 0u32, "Metric"),
                Self::Log => serializer.serialize_unit_variant("SignalType", 1u32, "Log"),
                Self::Unknown => serializer.serialize_unit_variant("SignalType", 2u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Alert object state, which can be modified by the user."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AlertState")]
    pub enum AlertState {
        New,
        Acknowledged,
        Closed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AlertState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AlertState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AlertState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::New => serializer.serialize_unit_variant("AlertState", 0u32, "New"),
                Self::Acknowledged => serializer.serialize_unit_variant("AlertState", 1u32, "Acknowledged"),
                Self::Closed => serializer.serialize_unit_variant("AlertState", 2u32, "Closed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Condition of the rule at the monitor service. It represents whether the underlying conditions have crossed the defined alert rule thresholds."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MonitorCondition")]
    pub enum MonitorCondition {
        Fired,
        Resolved,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MonitorCondition {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MonitorCondition {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MonitorCondition {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Fired => serializer.serialize_unit_variant("MonitorCondition", 0u32, "Fired"),
                Self::Resolved => serializer.serialize_unit_variant("MonitorCondition", 1u32, "Resolved"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Monitor service on which the rule(monitor) is set."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MonitorService")]
    pub enum MonitorService {
        #[serde(rename = "Application Insights")]
        ApplicationInsights,
        #[serde(rename = "ActivityLog Administrative")]
        ActivityLogAdministrative,
        #[serde(rename = "ActivityLog Security")]
        ActivityLogSecurity,
        #[serde(rename = "ActivityLog Recommendation")]
        ActivityLogRecommendation,
        #[serde(rename = "ActivityLog Policy")]
        ActivityLogPolicy,
        #[serde(rename = "ActivityLog Autoscale")]
        ActivityLogAutoscale,
        #[serde(rename = "Log Analytics")]
        LogAnalytics,
        Nagios,
        Platform,
        #[serde(rename = "SCOM")]
        Scom,
        ServiceHealth,
        SmartDetector,
        #[serde(rename = "VM Insights")]
        VmInsights,
        Zabbix,
        #[serde(rename = "Resource Health")]
        ResourceHealth,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MonitorService {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MonitorService {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MonitorService {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ApplicationInsights => serializer.serialize_unit_variant("MonitorService", 0u32, "Application Insights"),
                Self::ActivityLogAdministrative => serializer.serialize_unit_variant("MonitorService", 1u32, "ActivityLog Administrative"),
                Self::ActivityLogSecurity => serializer.serialize_unit_variant("MonitorService", 2u32, "ActivityLog Security"),
                Self::ActivityLogRecommendation => serializer.serialize_unit_variant("MonitorService", 3u32, "ActivityLog Recommendation"),
                Self::ActivityLogPolicy => serializer.serialize_unit_variant("MonitorService", 4u32, "ActivityLog Policy"),
                Self::ActivityLogAutoscale => serializer.serialize_unit_variant("MonitorService", 5u32, "ActivityLog Autoscale"),
                Self::LogAnalytics => serializer.serialize_unit_variant("MonitorService", 6u32, "Log Analytics"),
                Self::Nagios => serializer.serialize_unit_variant("MonitorService", 7u32, "Nagios"),
                Self::Platform => serializer.serialize_unit_variant("MonitorService", 8u32, "Platform"),
                Self::Scom => serializer.serialize_unit_variant("MonitorService", 9u32, "SCOM"),
                Self::ServiceHealth => serializer.serialize_unit_variant("MonitorService", 10u32, "ServiceHealth"),
                Self::SmartDetector => serializer.serialize_unit_variant("MonitorService", 11u32, "SmartDetector"),
                Self::VmInsights => serializer.serialize_unit_variant("MonitorService", 12u32, "VM Insights"),
                Self::Zabbix => serializer.serialize_unit_variant("MonitorService", 13u32, "Zabbix"),
                Self::ResourceHealth => serializer.serialize_unit_variant("MonitorService", 14u32, "Resource Health"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Operation provided by provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "Origin of the operation"]
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
    #[doc = "Properties of the operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Provider name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of the operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Lists the operations available in the AlertsManagement RP."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsList {
    #[doc = "URL to fetch the next set of alerts."]
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
