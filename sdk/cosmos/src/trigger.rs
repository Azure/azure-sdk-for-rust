create_enum!(
    TriggerOperation,
    (All, "All"),
    (Create, "Create"),
    (Replace, "Replace"),
    (Delete, "Delete")
);

create_enum!(TriggerType, (Pre, "Pre"), (Post, "Post"));

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
