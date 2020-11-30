//! Utilities for interacting with [`Trigger`]s.

/// A piece of logic that can be executed before or after creating, deleting, & replacing a document.
///
/// You can learn more about triggers [here](https://docs.microsoft.com/en-us/rest/api/cosmos-db/triggers).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Trigger {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_ts")]
    pub ts: u64,
    pub _self: String,
    #[serde(rename = "_etag")]
    pub etag: String,

    pub id: String,
    #[serde(rename = "triggerOperation")]
    pub trigger_operation: TriggerOperation,
    #[serde(rename = "triggerType")]
    pub trigger_type: TriggerType,
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
