//! Utilities for interacting with [`Trigger`]s.

#![allow(missing_docs)]

/// A piece of logic that can be executed before or after creating, deleting, & replacing a document.
///
/// You can learn more about triggers [here](https://docs.microsoft.com/rest/api/cosmos-db/triggers).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Trigger {
    /// The trigger id
    pub id: String,
    /// The resource id
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_ts")]
    /// The last updated timestamp
    pub ts: u64,
    /// The resource's uri
    pub _self: String,
    /// The resource's etag used for concurrency control
    #[serde(rename = "_etag")]
    pub etag: String,
    /// The trigger operation
    #[serde(rename = "triggerOperation")]
    pub trigger_operation: TriggerOperation,
    #[serde(rename = "triggerType")]
    /// The trigger type
    pub trigger_type: TriggerType,
    /// The trigger body
    pub body: String,
}

create_enum!(
    TriggerOperation,
    (All, "All"),
    (Create, "Create"),
    (Replace, "Replace"),
    (Delete, "Delete")
);

create_enum!(TriggerType, (Pre, "Pre"), (Post, "Post"));
