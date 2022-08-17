#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
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
#[doc = "availabilityStatus of a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityStatus {
    #[doc = "Azure Resource Manager Identity for the availabilityStatuses resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "current."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Microsoft.ResourceHealth/AvailabilityStatuses."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Azure Resource Manager geo location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties of availability state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<availability_status::Properties>,
}
impl AvailabilityStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod availability_status {
    use super::*;
    #[doc = "Properties of availability state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Availability status of the resource. When it is null, this availabilityStatus object represents an availability impacting event"]
        #[serde(rename = "availabilityState", default, skip_serializing_if = "Option::is_none")]
        pub availability_state: Option<properties::AvailabilityState>,
        #[doc = "Summary description of the availability status."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub summary: Option<String>,
        #[doc = "Details of the availability status."]
        #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
        pub detailed_status: Option<String>,
        #[doc = "When the resource's availabilityState is Unavailable, it describes where the health impacting event was originated. Examples are planned, unplanned, user initiated or an outage etc."]
        #[serde(rename = "reasonType", default, skip_serializing_if = "Option::is_none")]
        pub reason_type: Option<String>,
        #[doc = "When the resource's availabilityState is Unavailable, it provides the Timestamp for when the health impacting event was received."]
        #[serde(rename = "rootCauseAttributionTime", default, with = "azure_core::date::rfc3339::option")]
        pub root_cause_attribution_time: Option<time::OffsetDateTime>,
        #[doc = "In case of an availability impacting event, it describes when the health impacting event was originated. Examples are Lifecycle, Downtime, Fault Analysis etc."]
        #[serde(rename = "healthEventType", default, skip_serializing_if = "Option::is_none")]
        pub health_event_type: Option<String>,
        #[doc = "In case of an availability impacting event, it describes where the health impacting event was originated. Examples are PlatformInitiated, UserInitiated etc."]
        #[serde(rename = "healthEventCause", default, skip_serializing_if = "Option::is_none")]
        pub health_event_cause: Option<String>,
        #[doc = "In case of an availability impacting event, it describes the category of a PlatformInitiated health impacting event. Examples are Planned, Unplanned etc."]
        #[serde(rename = "healthEventCategory", default, skip_serializing_if = "Option::is_none")]
        pub health_event_category: Option<String>,
        #[doc = "It is a unique Id that identifies the event"]
        #[serde(rename = "healthEventId", default, skip_serializing_if = "Option::is_none")]
        pub health_event_id: Option<String>,
        #[doc = "When the resource's availabilityState is Unavailable and the reasonType is not User Initiated, it provides the date and time for when the issue is expected to be resolved."]
        #[serde(rename = "resolutionETA", default, with = "azure_core::date::rfc3339::option")]
        pub resolution_eta: Option<time::OffsetDateTime>,
        #[doc = "Timestamp for when last change in health status occurred."]
        #[serde(rename = "occuredTime", default, with = "azure_core::date::rfc3339::option")]
        pub occured_time: Option<time::OffsetDateTime>,
        #[doc = "Chronicity of the availability transition."]
        #[serde(rename = "reasonChronicity", default, skip_serializing_if = "Option::is_none")]
        pub reason_chronicity: Option<properties::ReasonChronicity>,
        #[doc = "Timestamp for when the health was last checked. "]
        #[serde(rename = "reportedTime", default, with = "azure_core::date::rfc3339::option")]
        pub reported_time: Option<time::OffsetDateTime>,
        #[doc = "An annotation describing a change in the availabilityState to Available from Unavailable with a reasonType of type Unplanned"]
        #[serde(rename = "recentlyResolvedState", default, skip_serializing_if = "Option::is_none")]
        pub recently_resolved_state: Option<properties::RecentlyResolvedState>,
        #[doc = "Lists actions the user can take based on the current availabilityState of the resource."]
        #[serde(rename = "recommendedActions", default, skip_serializing_if = "Vec::is_empty")]
        pub recommended_actions: Vec<RecommendedAction>,
        #[doc = "Lists the service impacting events that may be affecting the health of the resource."]
        #[serde(rename = "serviceImpactingEvents", default, skip_serializing_if = "Vec::is_empty")]
        pub service_impacting_events: Vec<ServiceImpactingEvent>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Availability status of the resource. When it is null, this availabilityStatus object represents an availability impacting event"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum AvailabilityState {
            Available,
            Unavailable,
            Unknown,
        }
        #[doc = "Chronicity of the availability transition."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ReasonChronicity {
            Transient,
            Persistent,
        }
        #[doc = "An annotation describing a change in the availabilityState to Available from Unavailable with a reasonType of type Unplanned"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct RecentlyResolvedState {
            #[doc = "Timestamp for when the availabilityState changed to Unavailable"]
            #[serde(rename = "unavailableOccurredTime", default, with = "azure_core::date::rfc3339::option")]
            pub unavailable_occurred_time: Option<time::OffsetDateTime>,
            #[doc = "Timestamp when the availabilityState changes to Available."]
            #[serde(rename = "resolvedTime", default, with = "azure_core::date::rfc3339::option")]
            pub resolved_time: Option<time::OffsetDateTime>,
            #[doc = "Brief description of cause of the resource becoming unavailable."]
            #[serde(rename = "unavailabilitySummary", default, skip_serializing_if = "Option::is_none")]
            pub unavailability_summary: Option<String>,
        }
        impl RecentlyResolvedState {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}
#[doc = "The List availabilityStatus operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailabilityStatusListResult {
    #[doc = "The list of availabilityStatuses."]
    pub value: Vec<AvailabilityStatus>,
    #[doc = "The URI to fetch the next page of availabilityStatuses. Call ListNext() with this URI to fetch the next page of availabilityStatuses."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AvailabilityStatusListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AvailabilityStatusListResult {
    pub fn new(value: Vec<AvailabilityStatus>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "On-going emerging issue from azure status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmergingIssue {
    #[doc = "Timestamp for when last time refreshed for ongoing emerging issue."]
    #[serde(rename = "refreshTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub refresh_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The list of emerging issues of banner type."]
    #[serde(rename = "statusBanners", default, skip_serializing_if = "Vec::is_empty")]
    pub status_banners: Vec<StatusBanner>,
    #[doc = "The list of emerging issues of active event type."]
    #[serde(rename = "statusActiveEvents", default, skip_serializing_if = "Vec::is_empty")]
    pub status_active_events: Vec<StatusActiveEvent>,
}
impl EmergingIssue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object of the emerging issue impact on services and regions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmergingIssueImpact {
    #[doc = "The impacted service id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The impacted service name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The list of impacted regions for corresponding emerging issues."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regions: Vec<ImpactedRegion>,
}
impl EmergingIssueImpact {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of emerging issues."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmergingIssueListResult {
    #[doc = "The list of emerging issues."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EmergingIssuesGetResult>,
    #[doc = "The link used to get the next page of emerging issues."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EmergingIssueListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EmergingIssueListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Get EmergingIssues operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmergingIssuesGetResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "On-going emerging issue from azure status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EmergingIssue>,
}
impl EmergingIssuesGetResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object of impacted region."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImpactedRegion {
    #[doc = "The impacted region id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The impacted region name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ImpactedRegion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation available in the resourcehealth resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties of the operation."]
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
    #[doc = "Properties of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Provider name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Lists the operations response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[doc = "List of operations available in the resourcehealth resource provider."]
    pub value: Vec<Operation>,
}
impl OperationListResult {
    pub fn new(value: Vec<Operation>) -> Self {
        Self { value }
    }
}
#[doc = "Lists actions the user can take based on the current availabilityState of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedAction {
    #[doc = "Recommended action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "Link to the action"]
    #[serde(rename = "actionUrl", default, skip_serializing_if = "Option::is_none")]
    pub action_url: Option<String>,
    #[doc = "Substring of action, it describes which text should host the action url."]
    #[serde(rename = "actionUrlText", default, skip_serializing_if = "Option::is_none")]
    pub action_url_text: Option<String>,
}
impl RecommendedAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lists the service impacting events that may be affecting the health of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceImpactingEvent {
    #[doc = "Timestamp for when the event started."]
    #[serde(rename = "eventStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub event_start_time: Option<time::OffsetDateTime>,
    #[doc = "Timestamp for when event was submitted/detected."]
    #[serde(rename = "eventStatusLastModifiedTime", default, with = "azure_core::date::rfc3339::option")]
    pub event_status_last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Correlation id for the event"]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "Status of the service impacting event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<service_impacting_event::Status>,
    #[doc = "Properties of the service impacting event."]
    #[serde(rename = "incidentProperties", default, skip_serializing_if = "Option::is_none")]
    pub incident_properties: Option<service_impacting_event::IncidentProperties>,
}
impl ServiceImpactingEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod service_impacting_event {
    use super::*;
    #[doc = "Status of the service impacting event."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Status {
        #[doc = "Current status of the event"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    impl Status {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Properties of the service impacting event."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct IncidentProperties {
        #[doc = "Title of the incident."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[doc = "Service impacted by the event."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub service: Option<String>,
        #[doc = "Region impacted by the event."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[doc = "Type of Event."]
        #[serde(rename = "incidentType", default, skip_serializing_if = "Option::is_none")]
        pub incident_type: Option<String>,
    }
    impl IncidentProperties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Active event type of emerging issue."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusActiveEvent {
    #[doc = "The active event title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The details of active event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The tracking id of this active event."]
    #[serde(rename = "trackingId", default, skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[doc = "The impact start time on this active event."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The cloud type of this active event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<String>,
    #[doc = "The severity level of this active event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<status_active_event::Severity>,
    #[doc = "The stage of this active event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<status_active_event::Stage>,
    #[doc = "The boolean value of this active event if published or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    #[doc = "The last time modified on this banner."]
    #[serde(rename = "lastModifiedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "The list of emerging issues impacts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub impacts: Vec<EmergingIssueImpact>,
}
impl StatusActiveEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod status_active_event {
    use super::*;
    #[doc = "The severity level of this active event."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Severity")]
    pub enum Severity {
        Information,
        Warning,
        Error,
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
                Self::Information => serializer.serialize_unit_variant("Severity", 0u32, "Information"),
                Self::Warning => serializer.serialize_unit_variant("Severity", 1u32, "Warning"),
                Self::Error => serializer.serialize_unit_variant("Severity", 2u32, "Error"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The stage of this active event."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Stage")]
    pub enum Stage {
        Active,
        Resolve,
        Archived,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Stage {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Stage {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Stage {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Active => serializer.serialize_unit_variant("Stage", 0u32, "Active"),
                Self::Resolve => serializer.serialize_unit_variant("Stage", 1u32, "Resolve"),
                Self::Archived => serializer.serialize_unit_variant("Stage", 2u32, "Archived"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Banner type of emerging issue."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusBanner {
    #[doc = "The banner title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The details of banner."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The cloud type of this banner."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<String>,
    #[doc = "The last time modified on this banner."]
    #[serde(rename = "lastModifiedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
}
impl StatusBanner {
    pub fn new() -> Self {
        Self::default()
    }
}
