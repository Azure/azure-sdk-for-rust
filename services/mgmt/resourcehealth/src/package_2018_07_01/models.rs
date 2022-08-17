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
#[doc = "The metadata entity contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataEntity {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The metadata entity properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetadataEntityProperties>,
}
impl MetadataEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of metadata entities"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataEntityListResult {
    #[doc = "The list of metadata entities."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MetadataEntity>,
    #[doc = "The link used to get the next page of metadata."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MetadataEntityListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MetadataEntityListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The metadata entity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataEntityProperties {
    #[doc = "The display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The list of keys on which this entity depends on."]
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    #[doc = "The list of scenarios applicable to this metadata entity."]
    #[serde(rename = "applicableScenarios", default, skip_serializing_if = "Vec::is_empty")]
    pub applicable_scenarios: Vec<String>,
    #[doc = "The list of supported values."]
    #[serde(rename = "supportedValues", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_values: Vec<MetadataSupportedValueDetail>,
}
impl MetadataEntityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The metadata supported value detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataSupportedValueDetail {
    #[doc = "The id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl MetadataSupportedValueDetail {
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
        #[serde(rename = "occurredTime", default, with = "azure_core::date::rfc3339::option")]
        pub occurred_time: Option<time::OffsetDateTime>,
        #[doc = "Chronicity of the availability transition."]
        #[serde(rename = "reasonChronicity", default, skip_serializing_if = "Option::is_none")]
        pub reason_chronicity: Option<properties::ReasonChronicity>,
        #[doc = "Timestamp for when the health was last checked. "]
        #[serde(rename = "reportedTime", default, with = "azure_core::date::rfc3339::option")]
        pub reported_time: Option<time::OffsetDateTime>,
        #[doc = "An annotation describing a change in the availabilityState to Available from Unavailable with a reasonType of type Unplanned"]
        #[serde(rename = "recentlyResolved", default, skip_serializing_if = "Option::is_none")]
        pub recently_resolved: Option<properties::RecentlyResolved>,
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
        #[serde(remote = "AvailabilityState")]
        pub enum AvailabilityState {
            Available,
            Unavailable,
            Degraded,
            Unknown,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for AvailabilityState {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for AvailabilityState {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for AvailabilityState {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Available => serializer.serialize_unit_variant("AvailabilityState", 0u32, "Available"),
                    Self::Unavailable => serializer.serialize_unit_variant("AvailabilityState", 1u32, "Unavailable"),
                    Self::Degraded => serializer.serialize_unit_variant("AvailabilityState", 2u32, "Degraded"),
                    Self::Unknown => serializer.serialize_unit_variant("AvailabilityState", 3u32, "Unknown"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "Chronicity of the availability transition."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "ReasonChronicity")]
        pub enum ReasonChronicity {
            Transient,
            Persistent,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for ReasonChronicity {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for ReasonChronicity {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for ReasonChronicity {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Transient => serializer.serialize_unit_variant("ReasonChronicity", 0u32, "Transient"),
                    Self::Persistent => serializer.serialize_unit_variant("ReasonChronicity", 1u32, "Persistent"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "An annotation describing a change in the availabilityState to Available from Unavailable with a reasonType of type Unplanned"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct RecentlyResolved {
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
        impl RecentlyResolved {
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
#[doc = "Service health event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Event {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<event::Properties>,
}
impl Event {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod event {
    use super::*;
    #[doc = "Properties of event."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Type of event."]
        #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
        pub event_type: Option<properties::EventType>,
        #[doc = "Source of event."]
        #[serde(rename = "eventSource", default, skip_serializing_if = "Option::is_none")]
        pub event_source: Option<properties::EventSource>,
        #[doc = "Current status of event."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<properties::Status>,
        #[doc = "Title text of event."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[doc = "Summary text of event."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub summary: Option<String>,
        #[doc = "Header text of event."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub header: Option<String>,
        #[doc = "Level of insight."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub level: Option<properties::Level>,
        #[doc = "Level of event."]
        #[serde(rename = "eventLevel", default, skip_serializing_if = "Option::is_none")]
        pub event_level: Option<properties::EventLevel>,
        #[doc = "Article of event."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub article: Option<properties::Article>,
        #[doc = "Useful links of event."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub links: Vec<Link>,
        #[doc = "It provides the Timestamp for when the health impacting event started."]
        #[serde(rename = "impactStartTime", default, with = "azure_core::date::rfc3339::option")]
        pub impact_start_time: Option<time::OffsetDateTime>,
        #[doc = "It provides the Timestamp for when the health impacting event resolved."]
        #[serde(rename = "impactMitigationTime", default, with = "azure_core::date::rfc3339::option")]
        pub impact_mitigation_time: Option<time::OffsetDateTime>,
        #[doc = "List services impacted by the service health event."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub impact: Vec<Impact>,
        #[doc = "Recommended actions of event."]
        #[serde(rename = "recommendedActions", default, skip_serializing_if = "Option::is_none")]
        pub recommended_actions: Option<properties::RecommendedActions>,
        #[doc = "Frequently asked questions for the service health event."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub faqs: Vec<Faq>,
        #[doc = "It provides information if the event is High incident rate event or not."]
        #[serde(rename = "isHIR", default, skip_serializing_if = "Option::is_none")]
        pub is_hir: Option<bool>,
        #[doc = "Tells if we want to enable or disable Microsoft Support for this event."]
        #[serde(rename = "enableMicrosoftSupport", default, skip_serializing_if = "Option::is_none")]
        pub enable_microsoft_support: Option<bool>,
        #[doc = "Contains the communication message for the event, that could include summary, root cause and other details."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "Is true if the event is platform initiated."]
        #[serde(rename = "platformInitiated", default, skip_serializing_if = "Option::is_none")]
        pub platform_initiated: Option<bool>,
        #[doc = "Tells if we want to enable or disable Microsoft Support for this event."]
        #[serde(rename = "enableChatWithUs", default, skip_serializing_if = "Option::is_none")]
        pub enable_chat_with_us: Option<bool>,
        #[doc = "Priority level of the event. Has value from 0 to 23. 0 is the highest priority. Service issue events have higher priority followed by planned maintenance and health advisory. Critical events have higher priority followed by error, warning and informational. Furthermore, active events have higher priority than resolved."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub priority: Option<i64>,
        #[doc = "It provides the Timestamp for when the health impacting event was last updated."]
        #[serde(rename = "lastUpdateTime", default, with = "azure_core::date::rfc3339::option")]
        pub last_update_time: Option<time::OffsetDateTime>,
        #[doc = "Stage for HIR Document"]
        #[serde(rename = "hirStage", default, skip_serializing_if = "Option::is_none")]
        pub hir_stage: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Type of event."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "EventType")]
        pub enum EventType {
            ServiceIssue,
            PlannedMaintenance,
            HealthAdvisory,
            #[serde(rename = "RCA")]
            Rca,
            EmergingIssues,
            SecurityAdvisory,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for EventType {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for EventType {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for EventType {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::ServiceIssue => serializer.serialize_unit_variant("EventType", 0u32, "ServiceIssue"),
                    Self::PlannedMaintenance => serializer.serialize_unit_variant("EventType", 1u32, "PlannedMaintenance"),
                    Self::HealthAdvisory => serializer.serialize_unit_variant("EventType", 2u32, "HealthAdvisory"),
                    Self::Rca => serializer.serialize_unit_variant("EventType", 3u32, "RCA"),
                    Self::EmergingIssues => serializer.serialize_unit_variant("EventType", 4u32, "EmergingIssues"),
                    Self::SecurityAdvisory => serializer.serialize_unit_variant("EventType", 5u32, "SecurityAdvisory"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "Source of event."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "EventSource")]
        pub enum EventSource {
            ResourceHealth,
            ServiceHealth,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for EventSource {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for EventSource {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for EventSource {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::ResourceHealth => serializer.serialize_unit_variant("EventSource", 0u32, "ResourceHealth"),
                    Self::ServiceHealth => serializer.serialize_unit_variant("EventSource", 1u32, "ServiceHealth"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "Current status of event."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Status")]
        pub enum Status {
            Active,
            Resolved,
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
                    Self::Active => serializer.serialize_unit_variant("Status", 0u32, "Active"),
                    Self::Resolved => serializer.serialize_unit_variant("Status", 1u32, "Resolved"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "Level of insight."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Level")]
        pub enum Level {
            Critical,
            Warning,
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
                    Self::Critical => serializer.serialize_unit_variant("Level", 0u32, "Critical"),
                    Self::Warning => serializer.serialize_unit_variant("Level", 1u32, "Warning"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "Level of event."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "EventLevel")]
        pub enum EventLevel {
            Critical,
            Error,
            Warning,
            Informational,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for EventLevel {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for EventLevel {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for EventLevel {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Critical => serializer.serialize_unit_variant("EventLevel", 0u32, "Critical"),
                    Self::Error => serializer.serialize_unit_variant("EventLevel", 1u32, "Error"),
                    Self::Warning => serializer.serialize_unit_variant("EventLevel", 2u32, "Warning"),
                    Self::Informational => serializer.serialize_unit_variant("EventLevel", 3u32, "Informational"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "Article of event."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Article {
            #[doc = "Article content of event."]
            #[serde(rename = "articleContent", default, skip_serializing_if = "Option::is_none")]
            pub article_content: Option<String>,
        }
        impl Article {
            pub fn new() -> Self {
                Self::default()
            }
        }
        #[doc = "Recommended actions of event."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct RecommendedActions {
            #[doc = "Recommended action title for the service health event."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub message: Option<String>,
            #[doc = "Recommended actions for the service health event."]
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub actions: Vec<serde_json::Value>,
            #[doc = "Recommended action locale for the service health event."]
            #[serde(rename = "localeCode", default, skip_serializing_if = "Option::is_none")]
            pub locale_code: Option<String>,
        }
        impl RecommendedActions {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}
#[doc = "The List events operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Events {
    #[doc = "The list of event."]
    pub value: Vec<Event>,
    #[doc = "The URI to fetch the next page of events. Call ListNext() with this URI to fetch the next page of events."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for Events {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl Events {
    pub fn new(value: Vec<Event>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Frequently asked question for the service health event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Faq {
    #[doc = "FAQ question for the service health event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    #[doc = "FAQ answer for the service health event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    #[doc = "FAQ locale for the service health event."]
    #[serde(rename = "localeCode", default, skip_serializing_if = "Option::is_none")]
    pub locale_code: Option<String>,
}
impl Faq {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure service impacted by the service health event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Impact {
    #[doc = "Impacted service name."]
    #[serde(rename = "impactedService", default, skip_serializing_if = "Option::is_none")]
    pub impacted_service: Option<String>,
    #[doc = "List regions impacted by the service health event."]
    #[serde(rename = "impactedRegions", default, skip_serializing_if = "Vec::is_empty")]
    pub impacted_regions: Vec<ImpactedServiceRegion>,
}
impl Impact {
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
#[doc = "Azure region impacted by the service health event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImpactedServiceRegion {
    #[doc = "Impacted region name."]
    #[serde(rename = "impactedRegion", default, skip_serializing_if = "Option::is_none")]
    pub impacted_region: Option<String>,
    #[doc = "Current status of event in the region."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<impacted_service_region::Status>,
    #[doc = "List subscription impacted by the service health event."]
    #[serde(rename = "impactedSubscriptions", default, skip_serializing_if = "Vec::is_empty")]
    pub impacted_subscriptions: Vec<String>,
    #[doc = "It provides the Timestamp for when the last update for the service health event."]
    #[serde(rename = "lastUpdateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_update_time: Option<time::OffsetDateTime>,
    #[doc = "List of updates for given service health event."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub updates: Vec<Update>,
}
impl ImpactedServiceRegion {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod impacted_service_region {
    use super::*;
    #[doc = "Current status of event in the region."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Active,
        Resolved,
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
                Self::Active => serializer.serialize_unit_variant("Status", 0u32, "Active"),
                Self::Resolved => serializer.serialize_unit_variant("Status", 1u32, "Resolved"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Useful links for service health event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Link {
    #[doc = "Type of link."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<link::Type>,
    #[doc = "Display text of link."]
    #[serde(rename = "displayText", default, skip_serializing_if = "Option::is_none")]
    pub display_text: Option<link::DisplayText>,
    #[doc = "It provides the name of portal extension to produce link for given service health event."]
    #[serde(rename = "extensionName", default, skip_serializing_if = "Option::is_none")]
    pub extension_name: Option<String>,
    #[doc = "It provides the name of portal extension blade to produce link for given service health event."]
    #[serde(rename = "bladeName", default, skip_serializing_if = "Option::is_none")]
    pub blade_name: Option<String>,
    #[doc = "It provides a map of parameter name and value for portal extension blade to produce lik for given service health event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl Link {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod link {
    use super::*;
    #[doc = "Type of link."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Button,
        Hyperlink,
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
                Self::Button => serializer.serialize_unit_variant("Type", 0u32, "Button"),
                Self::Hyperlink => serializer.serialize_unit_variant("Type", 1u32, "Hyperlink"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Display text of link."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DisplayText {
        #[doc = "Display text of link."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
        #[doc = "Localized display text of link."]
        #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
        pub localized_value: Option<String>,
    }
    impl DisplayText {
        pub fn new() -> Self {
            Self::default()
        }
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
#[doc = "Update for service health event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Update {
    #[doc = "Summary text for the given update for the service health event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "It provides the Timestamp for the given update for the service health event."]
    #[serde(rename = "updateDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub update_date_time: Option<time::OffsetDateTime>,
}
impl Update {
    pub fn new() -> Self {
        Self::default()
    }
}
