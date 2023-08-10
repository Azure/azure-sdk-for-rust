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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
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
        #[doc = "Title description of the availability status."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[doc = "Summary description of the availability status."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub summary: Option<String>,
        #[doc = "Details of the availability status."]
        #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
        pub detailed_status: Option<String>,
        #[doc = "When the resource's availabilityState is Unavailable, it describes where the health impacting event was originated. Examples are planned, unplanned, user initiated or an outage etc."]
        #[serde(rename = "reasonType", default, skip_serializing_if = "Option::is_none")]
        pub reason_type: Option<String>,
        #[doc = "When an event is created, it can either be triggered by a customer or the platform of the resource and this field will illustrate that. This field is connected to the category field in this object."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub context: Option<String>,
        #[doc = "When a context field is set to Platform, this field will reflect if the event was planned or unplanned. If the context field does not have a value of Platform, then this field will be ignored."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub category: Option<String>,
        #[doc = "The Article Id"]
        #[serde(rename = "articleId", default, skip_serializing_if = "Option::is_none")]
        pub article_id: Option<String>,
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
        #[serde(rename = "recentlyResolved", default, skip_serializing_if = "Option::is_none")]
        pub recently_resolved: Option<properties::RecentlyResolved>,
        #[doc = "Lists actions the user can take based on the current availabilityState of the resource."]
        #[serde(
            rename = "recommendedActions",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub recommended_actions: Vec<RecommendedAction>,
        #[doc = "Lists the service impacting events that may be affecting the health of the resource."]
        #[serde(
            rename = "serviceImpactingEvents",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
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
            #[serde(rename = "unavailableOccuredTime", default, with = "azure_core::date::rfc3339::option")]
            pub unavailable_occured_time: Option<time::OffsetDateTime>,
            #[doc = "Timestamp when the availabilityState changes to Available."]
            #[serde(rename = "resolvedTime", default, with = "azure_core::date::rfc3339::option")]
            pub resolved_time: Option<time::OffsetDateTime>,
            #[doc = "Brief description of cause of the resource becoming unavailable."]
            #[serde(rename = "unavailableSummary", default, skip_serializing_if = "Option::is_none")]
            pub unavailable_summary: Option<String>,
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
#[doc = "Service health event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Event {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
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
        #[doc = "The id of the Incident"]
        #[serde(rename = "externalIncidentId", default, skip_serializing_if = "Option::is_none")]
        pub external_incident_id: Option<String>,
        #[doc = "The reason for the Incident"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub reason: Option<String>,
        #[doc = "Article of event."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub article: Option<properties::Article>,
        #[doc = "Useful links of event."]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub links: Vec<Link>,
        #[doc = "It provides the Timestamp for when the health impacting event started."]
        #[serde(rename = "impactStartTime", default, with = "azure_core::date::rfc3339::option")]
        pub impact_start_time: Option<time::OffsetDateTime>,
        #[doc = "It provides the Timestamp for when the health impacting event resolved."]
        #[serde(rename = "impactMitigationTime", default, with = "azure_core::date::rfc3339::option")]
        pub impact_mitigation_time: Option<time::OffsetDateTime>,
        #[doc = "List services impacted by the service health event."]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub impact: Vec<Impact>,
        #[doc = "Recommended actions of event."]
        #[serde(rename = "recommendedActions", default, skip_serializing_if = "Option::is_none")]
        pub recommended_actions: Option<properties::RecommendedActions>,
        #[doc = "Frequently asked questions for the service health event."]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
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
        pub priority: Option<i32>,
        #[doc = "It provides the Timestamp for when the health impacting event was last updated."]
        #[serde(rename = "lastUpdateTime", default, with = "azure_core::date::rfc3339::option")]
        pub last_update_time: Option<time::OffsetDateTime>,
        #[doc = "Stage for HIR Document"]
        #[serde(rename = "hirStage", default, skip_serializing_if = "Option::is_none")]
        pub hir_stage: Option<String>,
        #[doc = "Additional information"]
        #[serde(rename = "additionalInformation", default, skip_serializing_if = "Option::is_none")]
        pub additional_information: Option<properties::AdditionalInformation>,
        #[doc = "duration in seconds"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub duration: Option<i32>,
        #[doc = "The type of the impact"]
        #[serde(rename = "impactType", default, skip_serializing_if = "Option::is_none")]
        pub impact_type: Option<String>,
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
            #[doc = "Article Id"]
            #[serde(rename = "articleId", default, skip_serializing_if = "Option::is_none")]
            pub article_id: Option<String>,
            #[doc = "It provides a map of parameter name and value"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub parameters: Option<serde_json::Value>,
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
            #[serde(
                default,
                deserialize_with = "azure_core::util::deserialize_null_as_default",
                skip_serializing_if = "Vec::is_empty"
            )]
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
        #[doc = "Additional information"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct AdditionalInformation {
            #[doc = "Additional information Message"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub message: Option<String>,
        }
        impl AdditionalInformation {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}
#[doc = "Impacted resource for an event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventImpactedResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of impacted resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<event_impacted_resource::Properties>,
}
impl EventImpactedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod event_impacted_resource {
    use super::*;
    #[doc = "Properties of impacted resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Resource type within Microsoft cloud."]
        #[serde(rename = "targetResourceType", default, skip_serializing_if = "Option::is_none")]
        pub target_resource_type: Option<String>,
        #[doc = "Identity for resource within Microsoft cloud."]
        #[serde(rename = "targetResourceId", default, skip_serializing_if = "Option::is_none")]
        pub target_resource_id: Option<String>,
        #[doc = "Impacted resource region name."]
        #[serde(rename = "targetRegion", default, skip_serializing_if = "Option::is_none")]
        pub target_region: Option<String>,
        #[doc = "Additional information."]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub info: Vec<KeyValueItem>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The List of eventImpactedResources operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventImpactedResourceListResult {
    #[doc = "The list of eventImpactedResources."]
    pub value: Vec<EventImpactedResource>,
    #[doc = "The URI to fetch the next page of events. Call ListNext() with this URI to fetch the next page of impacted resource."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EventImpactedResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EventImpactedResourceListResult {
    pub fn new(value: Vec<EventImpactedResource>) -> Self {
        Self { value, next_link: None }
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
    #[serde(
        rename = "impactedRegions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "impactedResource with health status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImpactedResourceStatus {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of impacted resource status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<impacted_resource_status::Properties>,
}
impl ImpactedResourceStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod impacted_resource_status {
    use super::*;
    #[doc = "Properties of impacted resource status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Impacted resource status of the resource."]
        #[serde(rename = "availabilityState", default, skip_serializing_if = "Option::is_none")]
        pub availability_state: Option<properties::AvailabilityState>,
        #[doc = "Title description of the impacted resource status."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[doc = "Summary description of the impacted resource status."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub summary: Option<String>,
        #[doc = "When the resource's availabilityState is Unavailable, it describes where the health impacting event was originated."]
        #[serde(rename = "reasonType", default, skip_serializing_if = "Option::is_none")]
        pub reason_type: Option<properties::ReasonType>,
        #[doc = "Timestamp for when last change in health status occurred."]
        #[serde(rename = "occurredTime", default, with = "azure_core::date::rfc3339::option")]
        pub occurred_time: Option<time::OffsetDateTime>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Impacted resource status of the resource."]
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
        #[doc = "When the resource's availabilityState is Unavailable, it describes where the health impacting event was originated."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "ReasonType")]
        pub enum ReasonType {
            Unplanned,
            Planned,
            UserInitiated,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for ReasonType {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for ReasonType {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for ReasonType {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Unplanned => serializer.serialize_unit_variant("ReasonType", 0u32, "Unplanned"),
                    Self::Planned => serializer.serialize_unit_variant("ReasonType", 1u32, "Planned"),
                    Self::UserInitiated => serializer.serialize_unit_variant("ReasonType", 2u32, "UserInitiated"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
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
    #[serde(
        rename = "impactedSubscriptions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub impacted_subscriptions: Vec<String>,
    #[doc = "List tenant impacted by the service health event."]
    #[serde(
        rename = "impactedTenants",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub impacted_tenants: Vec<String>,
    #[doc = "It provides the Timestamp for when the last update for the service health event."]
    #[serde(rename = "lastUpdateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_update_time: Option<time::OffsetDateTime>,
    #[doc = "List of updates for given service health event."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "Key value tuple."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyValueItem {
    #[doc = "Key of tuple."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "Value of tuple."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl KeyValueItem {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "the comment for the Action"]
    #[serde(rename = "_ActionUrl.Comment", default, skip_serializing_if = "Option::is_none")]
    pub action_url_comment: Option<String>,
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
