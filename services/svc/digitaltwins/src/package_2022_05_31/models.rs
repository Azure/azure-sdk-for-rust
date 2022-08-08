#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A model definition and metadata for that model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalTwinsModelData {
    #[doc = "A language map that contains the localized display names as specified in the model definition."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<serde_json::Value>,
    #[doc = "A language map that contains the localized descriptions as specified in the model definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<serde_json::Value>,
    #[doc = "The id of the model as specified in the model definition."]
    pub id: String,
    #[doc = "The time the model was uploaded to the service."]
    #[serde(rename = "uploadTime", with = "azure_core::date::rfc3339::option")]
    pub upload_time: Option<time::OffsetDateTime>,
    #[doc = "Indicates if the model is decommissioned. Decommissioned models cannot be referenced by newly created digital twins."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decommissioned: Option<bool>,
    #[doc = "The model definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<serde_json::Value>,
}
impl DigitalTwinsModelData {
    pub fn new(id: String) -> Self {
        Self {
            display_name: None,
            description: None,
            id,
            upload_time: None,
            decommissioned: None,
            model: None,
        }
    }
}
#[doc = "Error definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "Service specific error code which serves as the substatus for the HTTP error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A human-readable representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Internal error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<Error>,
    #[doc = "A more specific error description than was provided by the containing error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Box<Option<InnerError>>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[doc = "Error definition."]
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
#[doc = "A route which directs notification and telemetry events to an endpoint. Endpoints are a destination outside of Azure Digital Twins such as an EventHub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventRoute {
    #[doc = "The id of the event route."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the endpoint this event route is bound to."]
    #[serde(rename = "endpointName")]
    pub endpoint_name: String,
    #[doc = "An expression which describes the events which are routed to the endpoint."]
    pub filter: String,
}
impl EventRoute {
    pub fn new(endpoint_name: String, filter: String) -> Self {
        Self {
            id: None,
            endpoint_name,
            filter,
        }
    }
}
#[doc = "A collection of EventRoute objects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventRouteCollection {
    #[doc = "The EventRoute objects."]
    pub value: Vec<EventRoute>,
    #[doc = "A URI to retrieve the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EventRouteCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EventRouteCollection {
    pub fn new(value: Vec<EventRoute>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "An incoming relationship."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IncomingRelationship {
    #[doc = "A user-provided string representing the id of this relationship, unique in the context of the source digital twin, i.e. sourceId + relationshipId is unique in the context of the service."]
    #[serde(rename = "$relationshipId", default, skip_serializing_if = "Option::is_none")]
    pub relationship_id: Option<String>,
    #[doc = "The id of the source digital twin."]
    #[serde(rename = "$sourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[doc = "The name of the relationship."]
    #[serde(rename = "$relationshipName", default, skip_serializing_if = "Option::is_none")]
    pub relationship_name: Option<String>,
    #[doc = "Link to the relationship, to be used for deletion."]
    #[serde(rename = "$relationshipLink", default, skip_serializing_if = "Option::is_none")]
    pub relationship_link: Option<String>,
}
impl IncomingRelationship {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of incoming relationships which relate digital twins together."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncomingRelationshipCollection {
    pub value: Vec<IncomingRelationship>,
    #[doc = "A URI to retrieve the next page of objects."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IncomingRelationshipCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IncomingRelationshipCollection {
    pub fn new(value: Vec<IncomingRelationship>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A more specific error description than was provided by the containing error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerError {
    #[doc = "A more specific error code than was provided by the containing error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A more specific error description than was provided by the containing error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Box<Option<InnerError>>,
}
impl InnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type NonPagedDigitalTwinsModelDataCollection = Vec<DigitalTwinsModelData>;
#[doc = "A collection of DigitalTwinsModelData objects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedDigitalTwinsModelDataCollection {
    #[doc = "The DigitalTwinsModelData objects."]
    pub value: Vec<DigitalTwinsModelData>,
    #[doc = "A URI to retrieve the next page of objects."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedDigitalTwinsModelDataCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PagedDigitalTwinsModelDataCollection {
    pub fn new(value: Vec<DigitalTwinsModelData>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The results of a query operation and an optional continuation token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResult {
    #[doc = "The query results."]
    pub value: Vec<serde_json::Value>,
    #[doc = "A token which can be used to construct a new QuerySpecification to retrieve the next set of results."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl QueryResult {
    pub fn new(value: Vec<serde_json::Value>) -> Self {
        Self {
            value,
            continuation_token: None,
        }
    }
}
#[doc = "A query specification containing either a query statement or a continuation token from a previous query result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuerySpecification {
    #[doc = "The query to execute. This value is ignored if a continuation token is provided."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[doc = "A token which is used to retrieve the next set of results from a previous query."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl QuerySpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of relationships which relate digital twins together."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipCollection {
    #[doc = "The relationship objects."]
    pub value: Vec<serde_json::Value>,
    #[doc = "A URI to retrieve the next page of objects."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RelationshipCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RelationshipCollection {
    pub fn new(value: Vec<serde_json::Value>) -> Self {
        Self { value, next_link: None }
    }
}
