#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An Application Insights component API Key creation request definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiKeyRequest {
    #[doc = "The name of the API Key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The read access rights of this API Key."]
    #[serde(rename = "linkedReadProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub linked_read_properties: Vec<String>,
    #[doc = "The write access rights of this API Key."]
    #[serde(rename = "linkedWriteProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub linked_write_properties: Vec<String>,
}
impl ApiKeyRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Annotation associated with an application insights resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Annotation {
    #[doc = "Name of annotation"]
    #[serde(rename = "AnnotationName", default, skip_serializing_if = "Option::is_none")]
    pub annotation_name: Option<String>,
    #[doc = "Category of annotation, free form"]
    #[serde(rename = "Category", default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Time when event occurred"]
    #[serde(rename = "EventTime", with = "azure_core::date::rfc3339::option")]
    pub event_time: Option<time::OffsetDateTime>,
    #[doc = "Unique Id for annotation"]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Serialized JSON object for detailed properties"]
    #[serde(rename = "Properties", default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
    #[doc = "Related parent annotation if any"]
    #[serde(rename = "RelatedAnnotation", default, skip_serializing_if = "Option::is_none")]
    pub related_annotation: Option<String>,
}
impl Annotation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error associated with trying to create annotation with Id that already exist"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnnotationError {
    #[doc = "Error detail code and explanation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Inner error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<InnerError>,
}
impl azure_core::Continuable for AnnotationError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AnnotationError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Annotations list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnnotationsListResult {
    #[doc = "An array of annotations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Annotation>,
}
impl azure_core::Continuable for AnnotationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AnnotationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Application Insights component definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponent {
    #[serde(flatten)]
    pub components_resource: ComponentsResource,
    #[doc = "The kind of application that this component refers to, used to customize UI. This value is a freeform string, values should typically be one of the following: web, ios, other, store, java, phone."]
    pub kind: String,
    #[doc = "Resource etag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Properties that define an Application Insights component resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationInsightsComponentProperties>,
}
impl ApplicationInsightsComponent {
    pub fn new(components_resource: ComponentsResource, kind: String) -> Self {
        Self {
            components_resource,
            kind,
            etag: None,
            properties: None,
        }
    }
}
#[doc = "Properties that define an API key of an Application Insights Component."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsComponentApiKey {
    #[doc = "The unique ID of the API key inside an Application Insights component. It is auto generated when the API key is created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The API key value. It will be only return once when the API Key was created."]
    #[serde(rename = "apiKey", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[doc = "The create date of this API key."]
    #[serde(rename = "createdDate", default, skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[doc = "The name of the API key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The read access rights of this API Key."]
    #[serde(rename = "linkedReadProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub linked_read_properties: Vec<String>,
    #[doc = "The write access rights of this API Key."]
    #[serde(rename = "linkedWriteProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub linked_write_properties: Vec<String>,
}
impl ApplicationInsightsComponentApiKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the list of API Keys of an Application Insights Component."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentApiKeyListResult {
    #[doc = "List of API Key definitions."]
    pub value: Vec<ApplicationInsightsComponentApiKey>,
}
impl azure_core::Continuable for ApplicationInsightsComponentApiKeyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ApplicationInsightsComponentApiKeyListResult {
    pub fn new(value: Vec<ApplicationInsightsComponentApiKey>) -> Self {
        Self { value }
    }
}
#[doc = "Properties that define an Analytics item that is associated to an Application Insights component."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsComponentAnalyticsItem {
    #[doc = "Internally assigned unique id of the item definition."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The user-defined name of the item."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The content of this item"]
    #[serde(rename = "Content", default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "This instance's version of the data model. This can change as new features are added."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Enum indicating if this item definition is owned by a specific user or is shared between all users with access to the Application Insights component."]
    #[serde(rename = "Scope", default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<application_insights_component_analytics_item::Scope>,
    #[doc = "Enum indicating the type of the Analytics item."]
    #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<application_insights_component_analytics_item::Type>,
    #[doc = "Date and time in UTC when this item was created."]
    #[serde(rename = "TimeCreated", default, skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    #[doc = "Date and time in UTC of the last modification that was made to this item."]
    #[serde(rename = "TimeModified", default, skip_serializing_if = "Option::is_none")]
    pub time_modified: Option<String>,
    #[doc = "A set of properties that can be defined in the context of a specific item type. Each type may have its own properties."]
    #[serde(rename = "Properties", default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationInsightsComponentAnalyticsItemProperties>,
}
impl ApplicationInsightsComponentAnalyticsItem {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod application_insights_component_analytics_item {
    use super::*;
    #[doc = "Enum indicating if this item definition is owned by a specific user or is shared between all users with access to the Application Insights component."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Scope")]
    pub enum Scope {
        #[serde(rename = "shared")]
        Shared,
        #[serde(rename = "user")]
        User,
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
                Self::Shared => serializer.serialize_unit_variant("Scope", 0u32, "shared"),
                Self::User => serializer.serialize_unit_variant("Scope", 1u32, "user"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Enum indicating the type of the Analytics item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "query")]
        Query,
        #[serde(rename = "recent")]
        Recent,
        #[serde(rename = "function")]
        Function,
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
                Self::None => serializer.serialize_unit_variant("Type", 0u32, "none"),
                Self::Query => serializer.serialize_unit_variant("Type", 1u32, "query"),
                Self::Recent => serializer.serialize_unit_variant("Type", 2u32, "recent"),
                Self::Function => serializer.serialize_unit_variant("Type", 3u32, "function"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A set of properties that can be defined in the context of a specific item type. Each type may have its own properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsComponentAnalyticsItemProperties {
    #[doc = "A function alias, used when the type of the item is Function"]
    #[serde(rename = "functionAlias", default, skip_serializing_if = "Option::is_none")]
    pub function_alias: Option<String>,
}
impl ApplicationInsightsComponentAnalyticsItemProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Application Insights component available features."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsComponentAvailableFeatures {
    #[doc = "A list of Application Insights component feature."]
    #[serde(rename = "Result", default, skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<ApplicationInsightsComponentFeature>,
}
impl ApplicationInsightsComponentAvailableFeatures {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Application Insights component billing features"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsComponentBillingFeatures {
    #[doc = "An Application Insights component daily data volume cap"]
    #[serde(rename = "DataVolumeCap", default, skip_serializing_if = "Option::is_none")]
    pub data_volume_cap: Option<ApplicationInsightsComponentDataVolumeCap>,
    #[doc = "Current enabled pricing plan. When the component is in the Enterprise plan, this will list both 'Basic' and 'Application Insights Enterprise'."]
    #[serde(rename = "CurrentBillingFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub current_billing_features: Vec<String>,
}
impl ApplicationInsightsComponentBillingFeatures {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Application Insights component daily data volume cap"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsComponentDataVolumeCap {
    #[doc = "Daily data volume cap in GB."]
    #[serde(rename = "Cap", default, skip_serializing_if = "Option::is_none")]
    pub cap: Option<f64>,
    #[doc = "Daily data volume cap UTC reset hour."]
    #[serde(rename = "ResetTime", default, skip_serializing_if = "Option::is_none")]
    pub reset_time: Option<i64>,
    #[doc = "Reserved, not used for now."]
    #[serde(rename = "WarningThreshold", default, skip_serializing_if = "Option::is_none")]
    pub warning_threshold: Option<i64>,
    #[doc = "Reserved, not used for now."]
    #[serde(rename = "StopSendNotificationWhenHitThreshold", default, skip_serializing_if = "Option::is_none")]
    pub stop_send_notification_when_hit_threshold: Option<bool>,
    #[doc = "Do not send a notification email when the daily data volume cap is met."]
    #[serde(rename = "StopSendNotificationWhenHitCap", default, skip_serializing_if = "Option::is_none")]
    pub stop_send_notification_when_hit_cap: Option<bool>,
    #[doc = "Maximum daily data volume cap that the user can set for this component."]
    #[serde(rename = "MaxHistoryCap", default, skip_serializing_if = "Option::is_none")]
    pub max_history_cap: Option<f64>,
}
impl ApplicationInsightsComponentDataVolumeCap {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties that define a Continuous Export configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsComponentExportConfiguration {
    #[doc = "The unique ID of the export configuration inside an Application Insights component. It is auto generated when the Continuous Export configuration is created."]
    #[serde(rename = "ExportId", default, skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
    #[doc = "The instrumentation key of the Application Insights component."]
    #[serde(rename = "InstrumentationKey", default, skip_serializing_if = "Option::is_none")]
    pub instrumentation_key: Option<String>,
    #[doc = "This comma separated list of document types that will be exported. The possible values include 'Requests', 'Event', 'Exceptions', 'Metrics', 'PageViews', 'PageViewPerformance', 'Rdd', 'PerformanceCounters', 'Availability', 'Messages'."]
    #[serde(rename = "RecordTypes", default, skip_serializing_if = "Option::is_none")]
    pub record_types: Option<String>,
    #[doc = "The name of the Application Insights component."]
    #[serde(rename = "ApplicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[doc = "The subscription of the Application Insights component."]
    #[serde(rename = "SubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The resource group of the Application Insights component."]
    #[serde(rename = "ResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "The destination storage account subscription ID."]
    #[serde(rename = "DestinationStorageSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub destination_storage_subscription_id: Option<String>,
    #[doc = "The destination account location ID."]
    #[serde(rename = "DestinationStorageLocationId", default, skip_serializing_if = "Option::is_none")]
    pub destination_storage_location_id: Option<String>,
    #[doc = "The name of destination account."]
    #[serde(rename = "DestinationAccountId", default, skip_serializing_if = "Option::is_none")]
    pub destination_account_id: Option<String>,
    #[doc = "The destination type."]
    #[serde(rename = "DestinationType", default, skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
    #[doc = "This will be 'true' if the Continuous Export configuration is enabled, otherwise it will be 'false'."]
    #[serde(rename = "IsUserEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_user_enabled: Option<String>,
    #[doc = "Last time the Continuous Export configuration was updated."]
    #[serde(rename = "LastUserUpdate", default, skip_serializing_if = "Option::is_none")]
    pub last_user_update: Option<String>,
    #[doc = "Deprecated"]
    #[serde(rename = "NotificationQueueEnabled", default, skip_serializing_if = "Option::is_none")]
    pub notification_queue_enabled: Option<String>,
    #[doc = "This indicates current Continuous Export configuration status. The possible values are 'Preparing', 'Success', 'Failure'."]
    #[serde(rename = "ExportStatus", default, skip_serializing_if = "Option::is_none")]
    pub export_status: Option<String>,
    #[doc = "The last time data was successfully delivered to the destination storage container for this Continuous Export configuration."]
    #[serde(rename = "LastSuccessTime", default, skip_serializing_if = "Option::is_none")]
    pub last_success_time: Option<String>,
    #[doc = "The last time the Continuous Export configuration started failing."]
    #[serde(rename = "LastGapTime", default, skip_serializing_if = "Option::is_none")]
    pub last_gap_time: Option<String>,
    #[doc = "This is the reason the Continuous Export configuration started failing. It can be 'AzureStorageNotFound' or 'AzureStorageAccessDenied'."]
    #[serde(rename = "PermanentErrorReason", default, skip_serializing_if = "Option::is_none")]
    pub permanent_error_reason: Option<String>,
    #[doc = "The name of the destination storage account."]
    #[serde(rename = "StorageName", default, skip_serializing_if = "Option::is_none")]
    pub storage_name: Option<String>,
    #[doc = "The name of the destination storage container."]
    #[serde(rename = "ContainerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
}
impl ApplicationInsightsComponentExportConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ApplicationInsightsComponentExportConfigurationListResult = Vec<ApplicationInsightsComponentExportConfiguration>;
#[doc = "An Application Insights component Continuous Export configuration request definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsComponentExportRequest {
    #[doc = "The document types to be exported, as comma separated values. Allowed values include 'Requests', 'Event', 'Exceptions', 'Metrics', 'PageViews', 'PageViewPerformance', 'Rdd', 'PerformanceCounters', 'Availability', 'Messages'."]
    #[serde(rename = "RecordTypes", default, skip_serializing_if = "Option::is_none")]
    pub record_types: Option<String>,
    #[doc = "The Continuous Export destination type. This has to be 'Blob'."]
    #[serde(rename = "DestinationType", default, skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
    #[doc = "The SAS URL for the destination storage container. It must grant write permission."]
    #[serde(rename = "DestinationAddress", default, skip_serializing_if = "Option::is_none")]
    pub destination_address: Option<String>,
    #[doc = "Set to 'true' to create a Continuous Export configuration as enabled, otherwise set it to 'false'."]
    #[serde(rename = "IsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<String>,
    #[doc = "Deprecated"]
    #[serde(rename = "NotificationQueueEnabled", default, skip_serializing_if = "Option::is_none")]
    pub notification_queue_enabled: Option<String>,
    #[doc = "Deprecated"]
    #[serde(rename = "NotificationQueueUri", default, skip_serializing_if = "Option::is_none")]
    pub notification_queue_uri: Option<String>,
    #[doc = "The subscription ID of the destination storage container."]
    #[serde(rename = "DestinationStorageSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub destination_storage_subscription_id: Option<String>,
    #[doc = "The location ID of the destination storage container."]
    #[serde(rename = "DestinationStorageLocationId", default, skip_serializing_if = "Option::is_none")]
    pub destination_storage_location_id: Option<String>,
    #[doc = "The name of destination storage account."]
    #[serde(rename = "DestinationAccountId", default, skip_serializing_if = "Option::is_none")]
    pub destination_account_id: Option<String>,
}
impl ApplicationInsightsComponentExportRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties that define a favorite that is associated to an Application Insights component."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsComponentFavorite {
    #[doc = "The user-defined name of the favorite."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Configuration of this particular favorite, which are driven by the Azure portal UX. Configuration data is a string containing valid JSON"]
    #[serde(rename = "Config", default, skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    #[doc = "This instance's version of the data model. This can change as new features are added that can be marked favorite. Current examples include MetricsExplorer (ME) and Search."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Internally assigned unique id of the favorite definition."]
    #[serde(rename = "FavoriteId", default, skip_serializing_if = "Option::is_none")]
    pub favorite_id: Option<String>,
    #[doc = "Enum indicating if this favorite definition is owned by a specific user or is shared between all users with access to the Application Insights component."]
    #[serde(rename = "FavoriteType", default, skip_serializing_if = "Option::is_none")]
    pub favorite_type: Option<application_insights_component_favorite::FavoriteType>,
    #[doc = "The source of the favorite definition."]
    #[serde(rename = "SourceType", default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[doc = "Date and time in UTC of the last modification that was made to this favorite definition."]
    #[serde(rename = "TimeModified", default, skip_serializing_if = "Option::is_none")]
    pub time_modified: Option<String>,
    #[doc = "A list of 0 or more tags that are associated with this favorite definition"]
    #[serde(rename = "Tags", default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[doc = "Favorite category, as defined by the user at creation time."]
    #[serde(rename = "Category", default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Flag denoting wether or not this favorite was generated from a template."]
    #[serde(rename = "IsGeneratedFromTemplate", default, skip_serializing_if = "Option::is_none")]
    pub is_generated_from_template: Option<bool>,
    #[doc = "Unique user id of the specific user that owns this favorite."]
    #[serde(rename = "UserId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}
impl ApplicationInsightsComponentFavorite {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod application_insights_component_favorite {
    use super::*;
    #[doc = "Enum indicating if this favorite definition is owned by a specific user or is shared between all users with access to the Application Insights component."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FavoriteType {
        #[serde(rename = "shared")]
        Shared,
        #[serde(rename = "user")]
        User,
    }
}
#[doc = "An Application Insights component daily data volume cap status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsComponentFeature {
    #[doc = "The pricing feature name."]
    #[serde(rename = "FeatureName", default, skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<String>,
    #[doc = "The meter id used for the feature."]
    #[serde(rename = "MeterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "The meter rate for the feature's meter."]
    #[serde(rename = "MeterRateFrequency", default, skip_serializing_if = "Option::is_none")]
    pub meter_rate_frequency: Option<String>,
    #[doc = "Reserved, not used now."]
    #[serde(rename = "ResouceId", default, skip_serializing_if = "Option::is_none")]
    pub resouce_id: Option<String>,
    #[doc = "Reserved, not used now."]
    #[serde(rename = "IsHidden", default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[doc = "A list of Application Insights component feature capability."]
    #[serde(rename = "Capabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<ApplicationInsightsComponentFeatureCapability>,
    #[doc = "Display name of the feature."]
    #[serde(rename = "Title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Whether can apply addon feature on to it."]
    #[serde(rename = "IsMainFeature", default, skip_serializing_if = "Option::is_none")]
    pub is_main_feature: Option<bool>,
    #[doc = "The add on features on main feature."]
    #[serde(rename = "SupportedAddonFeatures", default, skip_serializing_if = "Option::is_none")]
    pub supported_addon_features: Option<String>,
}
impl ApplicationInsightsComponentFeature {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Application Insights component feature capabilities"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsComponentFeatureCapabilities {
    #[doc = "Whether allow to use continuous export feature."]
    #[serde(rename = "SupportExportData", default, skip_serializing_if = "Option::is_none")]
    pub support_export_data: Option<bool>,
    #[doc = "Reserved, not used now."]
    #[serde(rename = "BurstThrottlePolicy", default, skip_serializing_if = "Option::is_none")]
    pub burst_throttle_policy: Option<String>,
    #[doc = "Reserved, not used now."]
    #[serde(rename = "MetadataClass", default, skip_serializing_if = "Option::is_none")]
    pub metadata_class: Option<String>,
    #[doc = "Reserved, not used now."]
    #[serde(rename = "LiveStreamMetrics", default, skip_serializing_if = "Option::is_none")]
    pub live_stream_metrics: Option<bool>,
    #[doc = "Reserved, not used now."]
    #[serde(rename = "ApplicationMap", default, skip_serializing_if = "Option::is_none")]
    pub application_map: Option<bool>,
    #[doc = "Whether allow to use work item integration feature."]
    #[serde(rename = "WorkItemIntegration", default, skip_serializing_if = "Option::is_none")]
    pub work_item_integration: Option<bool>,
    #[doc = "Reserved, not used now."]
    #[serde(rename = "PowerBIIntegration", default, skip_serializing_if = "Option::is_none")]
    pub power_bi_integration: Option<bool>,
    #[doc = "Reserved, not used now."]
    #[serde(rename = "OpenSchema", default, skip_serializing_if = "Option::is_none")]
    pub open_schema: Option<bool>,
    #[doc = "Reserved, not used now."]
    #[serde(rename = "ProactiveDetection", default, skip_serializing_if = "Option::is_none")]
    pub proactive_detection: Option<bool>,
    #[doc = "Reserved, not used now."]
    #[serde(rename = "AnalyticsIntegration", default, skip_serializing_if = "Option::is_none")]
    pub analytics_integration: Option<bool>,
    #[doc = "Whether allow to use multiple steps web test feature."]
    #[serde(rename = "MultipleStepWebTest", default, skip_serializing_if = "Option::is_none")]
    pub multiple_step_web_test: Option<bool>,
    #[doc = "Reserved, not used now."]
    #[serde(rename = "ApiAccessLevel", default, skip_serializing_if = "Option::is_none")]
    pub api_access_level: Option<String>,
    #[doc = "The application insights component used tracking type."]
    #[serde(rename = "TrackingType", default, skip_serializing_if = "Option::is_none")]
    pub tracking_type: Option<String>,
    #[doc = "Daily data volume cap in GB."]
    #[serde(rename = "DailyCap", default, skip_serializing_if = "Option::is_none")]
    pub daily_cap: Option<f64>,
    #[doc = "Daily data volume cap UTC reset hour."]
    #[serde(rename = "DailyCapResetTime", default, skip_serializing_if = "Option::is_none")]
    pub daily_cap_reset_time: Option<f64>,
    #[doc = "Reserved, not used now."]
    #[serde(rename = "ThrottleRate", default, skip_serializing_if = "Option::is_none")]
    pub throttle_rate: Option<f64>,
}
impl ApplicationInsightsComponentFeatureCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Application Insights component feature capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsComponentFeatureCapability {
    #[doc = "The name of the capability."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The description of the capability."]
    #[serde(rename = "Description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The value of the capability."]
    #[serde(rename = "Value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The unit of the capability."]
    #[serde(rename = "Unit", default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The meter used for the capability."]
    #[serde(rename = "MeterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "The meter rate of the meter."]
    #[serde(rename = "MeterRateFrequency", default, skip_serializing_if = "Option::is_none")]
    pub meter_rate_frequency: Option<String>,
}
impl ApplicationInsightsComponentFeatureCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the list of Application Insights Resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentListResult {
    #[doc = "List of Application Insights component definitions."]
    pub value: Vec<ApplicationInsightsComponent>,
    #[doc = "The URI to get the next set of Application Insights component definitions if too many components where returned in the result set."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApplicationInsightsComponentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ApplicationInsightsComponentListResult {
    pub fn new(value: Vec<ApplicationInsightsComponent>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties that define a ProactiveDetection configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsComponentProactiveDetectionConfiguration {
    #[doc = "The rule name"]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A flag that indicates whether this rule is enabled by the user"]
    #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "A flag that indicated whether notifications on this rule should be sent to subscription owners"]
    #[serde(rename = "SendEmailsToSubscriptionOwners", default, skip_serializing_if = "Option::is_none")]
    pub send_emails_to_subscription_owners: Option<bool>,
    #[doc = "Custom email addresses for this rule notifications"]
    #[serde(rename = "CustomEmails", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_emails: Vec<String>,
    #[doc = "The last time this rule was updated"]
    #[serde(rename = "LastUpdatedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    #[doc = "Static definitions of the ProactiveDetection configuration rule (same values for all components)."]
    #[serde(rename = "RuleDefinitions", default, skip_serializing_if = "Option::is_none")]
    pub rule_definitions: Option<application_insights_component_proactive_detection_configuration::RuleDefinitions>,
}
impl ApplicationInsightsComponentProactiveDetectionConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod application_insights_component_proactive_detection_configuration {
    use super::*;
    #[doc = "Static definitions of the ProactiveDetection configuration rule (same values for all components)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct RuleDefinitions {
        #[doc = "The rule name"]
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[doc = "The rule name as it is displayed in UI"]
        #[serde(rename = "DisplayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "The rule description"]
        #[serde(rename = "Description", default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "URL which displays additional info about the proactive detection rule"]
        #[serde(rename = "HelpUrl", default, skip_serializing_if = "Option::is_none")]
        pub help_url: Option<String>,
        #[doc = "A flag indicating whether the rule is hidden (from the UI)"]
        #[serde(rename = "IsHidden", default, skip_serializing_if = "Option::is_none")]
        pub is_hidden: Option<bool>,
        #[doc = "A flag indicating whether the rule is enabled by default"]
        #[serde(rename = "IsEnabledByDefault", default, skip_serializing_if = "Option::is_none")]
        pub is_enabled_by_default: Option<bool>,
        #[doc = "A flag indicating whether the rule is in preview"]
        #[serde(rename = "IsInPreview", default, skip_serializing_if = "Option::is_none")]
        pub is_in_preview: Option<bool>,
        #[doc = "A flag indicating whether email notifications are supported for detections for this rule"]
        #[serde(rename = "SupportsEmailNotifications", default, skip_serializing_if = "Option::is_none")]
        pub supports_email_notifications: Option<bool>,
    }
    impl RuleDefinitions {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
pub type ApplicationInsightsComponentProactiveDetectionConfigurationListResult =
    Vec<ApplicationInsightsComponentProactiveDetectionConfiguration>;
#[doc = "Properties that define an Application Insights component resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentProperties {
    #[doc = "The unique ID of your application. This field mirrors the 'Name' field and cannot be changed."]
    #[serde(rename = "ApplicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[doc = "Application Insights Unique ID for your Application."]
    #[serde(rename = "AppId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "Application name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of application being monitored."]
    #[serde(rename = "Application_Type")]
    pub application_type: application_insights_component_properties::ApplicationType,
    #[doc = "Used by the Application Insights system to determine what kind of flow this component was created by. This is to be set to 'Bluefield' when creating/updating a component via the REST API."]
    #[serde(rename = "Flow_Type", default, skip_serializing_if = "Option::is_none")]
    pub flow_type: Option<application_insights_component_properties::FlowType>,
    #[doc = "Describes what tool created this Application Insights component. Customers using this API should set this to the default 'rest'."]
    #[serde(rename = "Request_Source", default, skip_serializing_if = "Option::is_none")]
    pub request_source: Option<application_insights_component_properties::RequestSource>,
    #[doc = "Application Insights Instrumentation key. A read-only value that applications can use to identify the destination for all telemetry sent to Azure Application Insights. This value will be supplied upon construction of each new Application Insights component."]
    #[serde(rename = "InstrumentationKey", default, skip_serializing_if = "Option::is_none")]
    pub instrumentation_key: Option<String>,
    #[doc = "Creation Date for the Application Insights component, in ISO 8601 format."]
    #[serde(rename = "CreationDate", with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "Azure Tenant Id."]
    #[serde(rename = "TenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The unique application ID created when a new application is added to HockeyApp, used for communications with HockeyApp."]
    #[serde(rename = "HockeyAppId", default, skip_serializing_if = "Option::is_none")]
    pub hockey_app_id: Option<String>,
    #[doc = "Token used to authenticate communications with between Application Insights and HockeyApp."]
    #[serde(rename = "HockeyAppToken", default, skip_serializing_if = "Option::is_none")]
    pub hockey_app_token: Option<String>,
    #[doc = "Current state of this component: whether or not is has been provisioned within the resource group it is defined. Users cannot change this value but are able to read from it. Values will include Succeeded, Deploying, Canceled, and Failed."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Percentage of the data produced by the application being monitored that is being sampled for Application Insights telemetry."]
    #[serde(rename = "SamplingPercentage", default, skip_serializing_if = "Option::is_none")]
    pub sampling_percentage: Option<f64>,
    #[doc = "Application Insights component connection string."]
    #[serde(rename = "ConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    #[doc = "Retention period in days."]
    #[serde(rename = "RetentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i64>,
    #[doc = "Disable IP masking."]
    #[serde(rename = "DisableIpMasking", default, skip_serializing_if = "Option::is_none")]
    pub disable_ip_masking: Option<bool>,
    #[doc = "Purge data immediately after 30 days."]
    #[serde(rename = "ImmediatePurgeDataOn30Days", default, skip_serializing_if = "Option::is_none")]
    pub immediate_purge_data_on30_days: Option<bool>,
    #[doc = "Resource Id of the log analytics workspace which the data will be ingested to. This property is required to create an application with this API version. Applications from older versions will not have this property."]
    #[serde(rename = "WorkspaceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_resource_id: Option<String>,
    #[doc = "The date which the component got migrated to LA, in ISO 8601 format."]
    #[serde(rename = "LaMigrationDate", with = "azure_core::date::rfc3339::option")]
    pub la_migration_date: Option<time::OffsetDateTime>,
    #[doc = "List of linked private link scope resources."]
    #[serde(rename = "PrivateLinkScopedResources", default, skip_serializing_if = "Vec::is_empty")]
    pub private_link_scoped_resources: Vec<PrivateLinkScopedResource>,
    #[doc = "The network access type for operating on the Application Insights Component. By default it is Enabled"]
    #[serde(rename = "publicNetworkAccessForIngestion", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access_for_ingestion: Option<PublicNetworkAccessType>,
    #[doc = "The network access type for operating on the Application Insights Component. By default it is Enabled"]
    #[serde(rename = "publicNetworkAccessForQuery", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access_for_query: Option<PublicNetworkAccessType>,
    #[doc = "Indicates the flow of the ingestion."]
    #[serde(rename = "IngestionMode", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_mode: Option<application_insights_component_properties::IngestionMode>,
    #[doc = "Disable Non-AAD based Auth."]
    #[serde(rename = "DisableLocalAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_local_auth: Option<bool>,
    #[doc = "Force users to create their own storage account for profiler and debugger."]
    #[serde(rename = "ForceCustomerStorageForProfiler", default, skip_serializing_if = "Option::is_none")]
    pub force_customer_storage_for_profiler: Option<bool>,
}
impl ApplicationInsightsComponentProperties {
    pub fn new(application_type: application_insights_component_properties::ApplicationType) -> Self {
        Self {
            application_id: None,
            app_id: None,
            name: None,
            application_type,
            flow_type: None,
            request_source: None,
            instrumentation_key: None,
            creation_date: None,
            tenant_id: None,
            hockey_app_id: None,
            hockey_app_token: None,
            provisioning_state: None,
            sampling_percentage: None,
            connection_string: None,
            retention_in_days: None,
            disable_ip_masking: None,
            immediate_purge_data_on30_days: None,
            workspace_resource_id: None,
            la_migration_date: None,
            private_link_scoped_resources: Vec::new(),
            public_network_access_for_ingestion: None,
            public_network_access_for_query: None,
            ingestion_mode: None,
            disable_local_auth: None,
            force_customer_storage_for_profiler: None,
        }
    }
}
pub mod application_insights_component_properties {
    use super::*;
    #[doc = "Type of application being monitored."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ApplicationType")]
    pub enum ApplicationType {
        #[serde(rename = "web")]
        Web,
        #[serde(rename = "other")]
        Other,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ApplicationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ApplicationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ApplicationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Web => serializer.serialize_unit_variant("ApplicationType", 0u32, "web"),
                Self::Other => serializer.serialize_unit_variant("ApplicationType", 1u32, "other"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for ApplicationType {
        fn default() -> Self {
            Self::Web
        }
    }
    #[doc = "Used by the Application Insights system to determine what kind of flow this component was created by. This is to be set to 'Bluefield' when creating/updating a component via the REST API."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FlowType")]
    pub enum FlowType {
        Bluefield,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FlowType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FlowType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FlowType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Bluefield => serializer.serialize_unit_variant("FlowType", 0u32, "Bluefield"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for FlowType {
        fn default() -> Self {
            Self::Bluefield
        }
    }
    #[doc = "Describes what tool created this Application Insights component. Customers using this API should set this to the default 'rest'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RequestSource")]
    pub enum RequestSource {
        #[serde(rename = "rest")]
        Rest,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RequestSource {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RequestSource {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RequestSource {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Rest => serializer.serialize_unit_variant("RequestSource", 0u32, "rest"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for RequestSource {
        fn default() -> Self {
            Self::Rest
        }
    }
    #[doc = "Indicates the flow of the ingestion."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IngestionMode")]
    pub enum IngestionMode {
        ApplicationInsights,
        ApplicationInsightsWithDiagnosticSettings,
        LogAnalytics,
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
                Self::ApplicationInsights => serializer.serialize_unit_variant("IngestionMode", 0u32, "ApplicationInsights"),
                Self::ApplicationInsightsWithDiagnosticSettings => {
                    serializer.serialize_unit_variant("IngestionMode", 1u32, "ApplicationInsightsWithDiagnosticSettings")
                }
                Self::LogAnalytics => serializer.serialize_unit_variant("IngestionMode", 2u32, "LogAnalytics"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for IngestionMode {
        fn default() -> Self {
            Self::LogAnalytics
        }
    }
}
#[doc = "An Application Insights component daily data volume cap status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsComponentQuotaStatus {
    #[doc = "The Application ID for the Application Insights component."]
    #[serde(rename = "AppId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "The daily data volume cap is met, and data ingestion will be stopped."]
    #[serde(rename = "ShouldBeThrottled", default, skip_serializing_if = "Option::is_none")]
    pub should_be_throttled: Option<bool>,
    #[doc = "Date and time when the daily data volume cap will be reset, and data ingestion will resume."]
    #[serde(rename = "ExpirationTime", default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
}
impl ApplicationInsightsComponentQuotaStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties that define a web test location available to an Application Insights Component."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsComponentWebTestLocation {
    #[doc = "The display name of the web test location."]
    #[serde(rename = "DisplayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Internally defined geographic location tag."]
    #[serde(rename = "Tag", default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}
impl ApplicationInsightsComponentWebTestLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the list of web test locations available to an Application Insights Component."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsWebTestLocationsListResult {
    #[doc = "List of web test locations."]
    pub value: Vec<ApplicationInsightsComponentWebTestLocation>,
}
impl azure_core::Continuable for ApplicationInsightsWebTestLocationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ApplicationInsightsWebTestLocationsListResult {
    pub fn new(value: Vec<ApplicationInsightsComponentWebTestLocation>) -> Self {
        Self { value }
    }
}
#[doc = "An Application Insights component linked storage accounts"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentLinkedStorageAccounts {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "An Application Insights component linked storage account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LinkedStorageAccountsProperties>,
}
impl ComponentLinkedStorageAccounts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Application Insights component linked storage accounts patch"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentLinkedStorageAccountsPatch {
    #[doc = "An Application Insights component linked storage account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LinkedStorageAccountsProperties>,
}
impl ComponentLinkedStorageAccountsPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the body of a purge request for an App Insights component"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentPurgeBody {
    #[doc = "Table from which to purge data."]
    pub table: String,
    #[doc = "The set of columns and filters (queries) to run over them to purge the resulting data."]
    pub filters: Vec<ComponentPurgeBodyFilters>,
}
impl ComponentPurgeBody {
    pub fn new(table: String, filters: Vec<ComponentPurgeBodyFilters>) -> Self {
        Self { table, filters }
    }
}
#[doc = "User-defined filters to return data which will be purged from the table."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentPurgeBodyFilters {
    #[doc = "The column of the table over which the given query should run"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<String>,
    #[doc = "A query operator to evaluate over the provided column and value(s). Supported operators are ==, =~, in, in~, >, >=, <, <=, between, and have the same behavior as they would in a KQL query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[doc = "the value for the operator to function over. This can be a number (e.g., > 100), a string (timestamp >= '2017-09-01') or array of values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[doc = "When filtering over custom dimensions, this key will be used as the name of the custom dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
impl ComponentPurgeBodyFilters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response containing operationId for a specific purge action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentPurgeResponse {
    #[doc = "Id to use when querying for status for a particular purge operation."]
    #[serde(rename = "operationId")]
    pub operation_id: String,
}
impl ComponentPurgeResponse {
    pub fn new(operation_id: String) -> Self {
        Self { operation_id }
    }
}
#[doc = "Response containing status for a specific purge operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentPurgeStatusResponse {
    #[doc = "Status of the operation represented by the requested Id."]
    pub status: component_purge_status_response::Status,
}
impl ComponentPurgeStatusResponse {
    pub fn new(status: component_purge_status_response::Status) -> Self {
        Self { status }
    }
}
pub mod component_purge_status_response {
    use super::*;
    #[doc = "Status of the operation represented by the requested Id."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "completed")]
        Completed,
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
                Self::Pending => serializer.serialize_unit_variant("Status", 0u32, "pending"),
                Self::Completed => serializer.serialize_unit_variant("Status", 1u32, "completed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentsResource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ComponentsResource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[doc = "Error definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDefinition {
    #[doc = "Service specific error code which serves as the substatus for the HTTP error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Internal error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<serde_json::Value>,
}
impl ErrorDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates Insights service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseComponents {
    #[doc = "Error response indicates Insights service is not able to process the incoming request. The reason is provided in the error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response_components::Error>,
}
impl azure_core::Continuable for ErrorResponseComponents {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponseComponents {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod error_response_components {
    use super::*;
    #[doc = "Error response indicates Insights service is not able to process the incoming request. The reason is provided in the error message."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Error code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Error message indicating why the operation failed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseLinkedStorage {
    #[doc = "Error response indicates Insights service is not able to process the incoming request. The reason is provided in the error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response_linked_storage::Error>,
}
impl ErrorResponseLinkedStorage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod error_response_linked_storage {
    use super::*;
    #[doc = "Error response indicates Insights service is not able to process the incoming request. The reason is provided in the error message."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Error code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Error message indicating why the operation failed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Inner error"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerError {
    #[doc = "Provides correlation for request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnosticcontext: Option<String>,
    #[doc = "Request time"]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub time: Option<time::OffsetDateTime>,
}
impl InnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerErrorTrace {
    #[doc = "detailed error trace"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trace: Vec<String>,
}
impl InnerErrorTrace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Application Insights component linked storage account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkedStorageAccountsProperties {
    #[doc = "Linked storage account resource ID"]
    #[serde(rename = "linkedStorageAccount", default, skip_serializing_if = "Option::is_none")]
    pub linked_storage_account: Option<String>,
}
impl LinkedStorageAccountsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a live token query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveTokenResponse {
    #[doc = "JWT token for accessing live metrics stream data."]
    #[serde(rename = "liveToken", default, skip_serializing_if = "Option::is_none")]
    pub live_token: Option<String>,
}
impl LiveTokenResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed service identity (system assigned and/or user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
    #[serde(rename = "type")]
    pub type_: ManagedServiceIdentityType,
    #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: ManagedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
#[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedServiceIdentityType")]
pub enum ManagedServiceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned,UserAssigned")]
    SystemAssignedUserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedServiceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedServiceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedServiceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedServiceIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 1u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 2u32, "UserAssigned"),
            Self::SystemAssignedUserAssigned => {
                serializer.serialize_unit_variant("ManagedServiceIdentityType", 3u32, "SystemAssigned,UserAssigned")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An Application Insights private workbook definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MyWorkbook {
    #[serde(flatten)]
    pub my_workbook_resource: MyWorkbookResource,
    #[doc = "The kind of workbook. Choices are user and shared."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<my_workbook::Kind>,
    #[doc = "Properties that contain a private workbook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MyWorkbookProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl MyWorkbook {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod my_workbook {
    use super::*;
    #[doc = "The kind of workbook. Choices are user and shared."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "shared")]
        Shared,
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
                Self::User => serializer.serialize_unit_variant("Kind", 0u32, "user"),
                Self::Shared => serializer.serialize_unit_variant("Kind", 1u32, "shared"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MyWorkbookError {
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
}
impl azure_core::Continuable for MyWorkbookError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MyWorkbookError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Customer Managed Identity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MyWorkbookManagedIdentity {
    #[doc = "Customer Managed Identity"]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<MyWorkbookUserAssignedIdentities>,
    #[doc = "The identity type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<my_workbook_managed_identity::Type>,
}
impl MyWorkbookManagedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod my_workbook_managed_identity {
    use super::*;
    #[doc = "The identity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        UserAssigned,
        None,
    }
}
#[doc = "Properties that contain a private workbook."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MyWorkbookProperties {
    #[doc = "The user-defined name of the private workbook."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Configuration of this particular private workbook. Configuration data is a string containing valid JSON"]
    #[serde(rename = "serializedData")]
    pub serialized_data: String,
    #[doc = "This instance's version of the data model. This can change as new features are added that can be marked private workbook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Date and time in UTC of the last modification that was made to this private workbook definition."]
    #[serde(rename = "timeModified", default, skip_serializing_if = "Option::is_none")]
    pub time_modified: Option<String>,
    #[doc = "Workbook category, as defined by the user at creation time."]
    pub category: String,
    #[doc = "A list of 0 or more tags that are associated with this private workbook definition"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[doc = "Unique user id of the specific user that owns this private workbook."]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "Optional resourceId for a source resource."]
    #[serde(rename = "sourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[doc = "BYOS Storage Account URI"]
    #[serde(rename = "storageUri", default, skip_serializing_if = "Option::is_none")]
    pub storage_uri: Option<String>,
}
impl MyWorkbookProperties {
    pub fn new(display_name: String, serialized_data: String, category: String) -> Self {
        Self {
            display_name,
            serialized_data,
            version: None,
            time_modified: None,
            category,
            tags: Vec::new(),
            user_id: None,
            source_id: None,
            storage_uri: None,
        }
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MyWorkbookResource {
    #[doc = "Customer Managed Identity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<MyWorkbookManagedIdentity>,
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Resource etag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<serde_json::Value>,
}
impl MyWorkbookResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Customer Managed Identity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MyWorkbookUserAssignedIdentities {
    #[doc = "The principal ID of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl MyWorkbookUserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workbook list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MyWorkbooksListResult {
    #[doc = "An array of private workbooks."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MyWorkbook>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MyWorkbooksListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MyWorkbooksListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CDN REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.Cdn"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Profile, endpoint, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Information about an operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationInfo {
    #[doc = "Name of the provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Name of the resource type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list CDN operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of CDN operations supported by the CDN resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
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
#[doc = "Represents an operation returned by the GetOperations request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationLive {
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Information about an operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationInfo>,
    #[doc = "Origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Properties of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OperationLive {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the List Operations operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsListResult {
    #[doc = "A collection of operations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationLive>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The private link scope resource reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkScopedResource {
    #[doc = "The full resource Id of the private link scope resource."]
    #[serde(rename = "ResourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The private link scope unique Identifier."]
    #[serde(rename = "ScopeId", default, skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}
impl PrivateLinkScopedResource {
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
#[doc = "The network access type for operating on the Application Insights Component. By default it is Enabled"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PublicNetworkAccessType")]
pub enum PublicNetworkAccessType {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PublicNetworkAccessType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PublicNetworkAccessType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PublicNetworkAccessType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccessType", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccessType", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for PublicNetworkAccessType {
    fn default() -> Self {
        Self::Enabled
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
#[doc = "A container holding only the Tags for a resource, allowing the user to update the tags on a WebTest instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsResource {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TagsResource {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User assigned identity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Application Insights web test definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebTest {
    #[serde(flatten)]
    pub webtests_resource: WebtestsResource,
    #[doc = "The kind of web test that this web test watches. Choices are ping and multistep."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<web_test::Kind>,
    #[doc = "Metadata describing a web test for an Azure resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebTestProperties>,
}
impl WebTest {
    pub fn new(webtests_resource: WebtestsResource) -> Self {
        Self {
            webtests_resource,
            kind: None,
            properties: None,
        }
    }
}
pub mod web_test {
    use super::*;
    #[doc = "The kind of web test that this web test watches. Choices are ping and multistep."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "ping")]
        Ping,
        #[serde(rename = "multistep")]
        Multistep,
    }
    impl Default for Kind {
        fn default() -> Self {
            Self::Ping
        }
    }
}
#[doc = "Geo-physical location to run a web test from. You must specify one or more locations for the test to run from."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebTestGeolocation {
    #[doc = "Location ID for the webtest to run from."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl WebTestGeolocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata describing a web test for an Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebTestProperties {
    #[doc = "Unique ID of this WebTest. This is typically the same value as the Name field."]
    #[serde(rename = "SyntheticMonitorId")]
    pub synthetic_monitor_id: String,
    #[doc = "User defined name if this WebTest."]
    #[serde(rename = "Name")]
    pub name: String,
    #[doc = "Purpose/user defined descriptive test for this WebTest."]
    #[serde(rename = "Description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Is the test actively being monitored."]
    #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Interval in seconds between test runs for this WebTest. Default value is 300."]
    #[serde(rename = "Frequency", default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i32>,
    #[doc = "Seconds until this WebTest will timeout and fail. Default value is 30."]
    #[serde(rename = "Timeout", default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[doc = "The kind of web test this is, valid choices are ping and multistep."]
    #[serde(rename = "Kind")]
    pub kind: web_test_properties::Kind,
    #[doc = "Allow for retries should this WebTest fail."]
    #[serde(rename = "RetryEnabled", default, skip_serializing_if = "Option::is_none")]
    pub retry_enabled: Option<bool>,
    #[doc = "A list of where to physically run the tests from to give global coverage for accessibility of your application."]
    #[serde(rename = "Locations")]
    pub locations: Vec<WebTestGeolocation>,
    #[doc = "An XML configuration specification for a WebTest."]
    #[serde(rename = "Configuration", default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<web_test_properties::Configuration>,
    #[doc = "Current state of this component, whether or not is has been provisioned within the resource group it is defined. Users cannot change this value but are able to read from it. Values will include Succeeded, Deploying, Canceled, and Failed."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl WebTestProperties {
    pub fn new(synthetic_monitor_id: String, name: String, kind: web_test_properties::Kind, locations: Vec<WebTestGeolocation>) -> Self {
        Self {
            synthetic_monitor_id,
            name,
            description: None,
            enabled: None,
            frequency: None,
            timeout: None,
            kind,
            retry_enabled: None,
            locations,
            configuration: None,
            provisioning_state: None,
        }
    }
}
pub mod web_test_properties {
    use super::*;
    #[doc = "The kind of web test this is, valid choices are ping and multistep."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "ping")]
        Ping,
        #[serde(rename = "multistep")]
        Multistep,
    }
    impl Default for Kind {
        fn default() -> Self {
            Self::Ping
        }
    }
    #[doc = "An XML configuration specification for a WebTest."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Configuration {
        #[doc = "The XML specification of a WebTest to run against an application."]
        #[serde(rename = "WebTest", default, skip_serializing_if = "Option::is_none")]
        pub web_test: Option<String>,
    }
    impl Configuration {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebtestsResource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl WebtestsResource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[doc = "Work item configuration associated with an application insights resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemConfiguration {
    #[doc = "Connector identifier where work item is created"]
    #[serde(rename = "ConnectorId", default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[doc = "Configuration friendly name"]
    #[serde(rename = "ConfigDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub config_display_name: Option<String>,
    #[doc = "Boolean value indicating whether configuration is default"]
    #[serde(rename = "IsDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "Unique Id for work item"]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Serialized JSON object for detailed properties"]
    #[serde(rename = "ConfigProperties", default, skip_serializing_if = "Option::is_none")]
    pub config_properties: Option<String>,
}
impl WorkItemConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error associated with trying to get work item configuration or configurations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemConfigurationError {
    #[doc = "Error detail code and explanation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Inner error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<InnerError>,
}
impl azure_core::Continuable for WorkItemConfigurationError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl WorkItemConfigurationError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Work item configuration list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemConfigurationsListResult {
    #[doc = "An array of work item configurations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkItemConfiguration>,
}
impl azure_core::Continuable for WorkItemConfigurationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl WorkItemConfigurationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Work item configuration creation payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemCreateConfiguration {
    #[doc = "Unique connector id"]
    #[serde(rename = "ConnectorId", default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[doc = "Serialized JSON object for detailed properties"]
    #[serde(rename = "ConnectorDataConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub connector_data_configuration: Option<String>,
    #[doc = "Boolean indicating validate only"]
    #[serde(rename = "ValidateOnly", default, skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<bool>,
    #[doc = "Custom work item properties"]
    #[serde(rename = "WorkItemProperties", default, skip_serializing_if = "Option::is_none")]
    pub work_item_properties: Option<serde_json::Value>,
}
impl WorkItemCreateConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A workbook definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workbook {
    #[serde(flatten)]
    pub workbook_resource: WorkbookResource,
    #[doc = "Properties that contain a workbook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkbookProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Workbook {
    pub fn new(workbook_resource: WorkbookResource) -> Self {
        Self {
            workbook_resource,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkbookError {
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<WorkbookErrorDefinition>,
}
impl azure_core::Continuable for WorkbookError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl WorkbookError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkbookErrorDefinition {
    #[doc = "Service specific error code which serves as the substatus for the HTTP error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Internal error details."]
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<serde_json::Value>,
}
impl WorkbookErrorDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkbookInnerErrorTrace {
    #[doc = "detailed error trace"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trace: Vec<String>,
}
impl WorkbookInnerErrorTrace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties that contain a workbook."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookProperties {
    #[doc = "The user-defined name (display name) of the workbook."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Configuration of this particular workbook. Configuration data is a string containing valid JSON"]
    #[serde(rename = "serializedData")]
    pub serialized_data: String,
    #[doc = "Workbook schema version format, like 'Notebook/1.0', which should match the workbook in serializedData"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Date and time in UTC of the last modification that was made to this workbook definition."]
    #[serde(rename = "timeModified", with = "azure_core::date::rfc3339::option")]
    pub time_modified: Option<time::OffsetDateTime>,
    #[doc = "Workbook category, as defined by the user at creation time."]
    pub category: String,
    #[doc = "Being deprecated, please use the other tags field"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[doc = "Unique user id of the specific user that owns this workbook."]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "ResourceId for a source resource."]
    #[serde(rename = "sourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[doc = "The resourceId to the storage account when bring your own storage is used"]
    #[serde(rename = "storageUri", default, skip_serializing_if = "Option::is_none")]
    pub storage_uri: Option<String>,
    #[doc = "The description of the workbook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The unique revision id for this workbook definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}
impl WorkbookProperties {
    pub fn new(display_name: String, serialized_data: String, category: String) -> Self {
        Self {
            display_name,
            serialized_data,
            version: None,
            time_modified: None,
            category,
            tags: Vec::new(),
            user_id: None,
            source_id: None,
            storage_uri: None,
            description: None,
            revision: None,
        }
    }
}
#[doc = "Properties that contain a workbook for PATCH operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkbookPropertiesUpdateParameters {
    #[doc = "The user-defined name (display name) of the workbook."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Configuration of this particular workbook. Configuration data is a string containing valid JSON"]
    #[serde(rename = "serializedData", default, skip_serializing_if = "Option::is_none")]
    pub serialized_data: Option<String>,
    #[doc = "Workbook category, as defined by the user at creation time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "A list of 0 or more tags that are associated with this workbook definition"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[doc = "The description of the workbook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The unique revision id for this workbook definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}
impl WorkbookPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Identity used for BYOS"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<serde_json::Value>,
    #[doc = "The kind of workbook. Only valid value is shared."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<workbook_resource::Kind>,
    #[doc = "Resource etag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl WorkbookResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
            kind: None,
            etag: None,
        }
    }
}
pub mod workbook_resource {
    use super::*;
    #[doc = "The kind of workbook. Only valid value is shared."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "shared")]
        Shared,
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
                Self::Shared => serializer.serialize_unit_variant("Kind", 0u32, "shared"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An Application Insights workbook template definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookTemplate {
    #[serde(flatten)]
    pub workbook_template_resource: WorkbookTemplateResource,
    #[doc = "Properties that contain a workbook template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkbookTemplateProperties>,
}
impl WorkbookTemplate {
    pub fn new(workbook_template_resource: WorkbookTemplateResource) -> Self {
        Self {
            workbook_template_resource,
            properties: None,
        }
    }
}
#[doc = "Error message that will indicate why the operation failed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkbookTemplateError {
    #[doc = "Error message body that will indicate why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<WorkbookTemplateErrorBody>,
}
impl azure_core::Continuable for WorkbookTemplateError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl WorkbookTemplateError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error message body that will indicate why the operation failed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkbookTemplateErrorBody {
    #[doc = "Service-defined error code. This code serves as a sub-status for the HTTP error code specified in the response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Human-readable representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The list of invalid fields send in request, in case of validation error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<WorkbookTemplateErrorFieldContract>,
}
impl WorkbookTemplateErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error Field contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkbookTemplateErrorFieldContract {
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
impl WorkbookTemplateErrorFieldContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gallery information for a workbook template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkbookTemplateGallery {
    #[doc = "Name of the workbook template in the gallery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Category for the gallery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Type of workbook supported by the workbook template."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Order of the template within the gallery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[doc = "Azure resource type supported by the gallery."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}
impl WorkbookTemplateGallery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Localized template data and gallery information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkbookTemplateLocalizedGallery {
    #[doc = "Valid JSON object containing workbook template payload."]
    #[serde(rename = "templateData", default, skip_serializing_if = "Option::is_none")]
    pub template_data: Option<serde_json::Value>,
    #[doc = "Workbook galleries supported by the template."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub galleries: Vec<WorkbookTemplateGallery>,
}
impl WorkbookTemplateLocalizedGallery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties that contain a workbook template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookTemplateProperties {
    #[doc = "Priority of the template. Determines which template to open when a workbook gallery is opened in viewer mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "Information about the author of the workbook template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[doc = "Valid JSON object containing workbook template payload."]
    #[serde(rename = "templateData")]
    pub template_data: serde_json::Value,
    #[doc = "Workbook galleries supported by the template."]
    pub galleries: Vec<WorkbookTemplateGallery>,
    #[doc = "Key value pair of localized gallery. Each key is the locale code of languages supported by the Azure portal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localized: Option<serde_json::Value>,
}
impl WorkbookTemplateProperties {
    pub fn new(template_data: serde_json::Value, galleries: Vec<WorkbookTemplateGallery>) -> Self {
        Self {
            priority: None,
            author: None,
            template_data,
            galleries,
            localized: None,
        }
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookTemplateResource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl WorkbookTemplateResource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[doc = "The parameters that can be provided when updating workbook template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkbookTemplateUpdateParameters {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties that contain a workbook template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkbookTemplateProperties>,
}
impl WorkbookTemplateUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "WorkbookTemplate list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkbookTemplatesListResult {
    #[doc = "An array of workbook templates."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkbookTemplate>,
}
impl azure_core::Continuable for WorkbookTemplatesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl WorkbookTemplatesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters that can be provided when updating workbook properties properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkbookUpdateParameters {
    #[doc = "The kind of workbook. Only valid value is shared."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<workbook_update_parameters::Kind>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties that contain a workbook for PATCH operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkbookPropertiesUpdateParameters>,
}
impl WorkbookUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workbook_update_parameters {
    use super::*;
    #[doc = "The kind of workbook. Only valid value is shared."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "shared")]
        Shared,
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
                Self::Shared => serializer.serialize_unit_variant("Kind", 0u32, "shared"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Workbook list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkbooksListResult {
    #[doc = "An array of workbooks."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workbook>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkbooksListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkbooksListResult {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
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
#[doc = "A list of 0 or more Application Insights web test definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebTestListResult {
    #[doc = "Set of Application Insights web test definitions."]
    pub value: Vec<WebTest>,
    #[doc = "The link to get the next part of the returned list of web tests, should the return set be too large for a single request. May be null."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WebTestListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WebTestListResult {
    pub fn new(value: Vec<WebTest>) -> Self {
        Self { value, next_link: None }
    }
}
