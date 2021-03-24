use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntityMetadata {
    #[serde(rename = "odata.metadata")]
    pub metadata: String,
    #[serde(rename = "odata.type")]
    pub odata_type: String,
    #[serde(rename = "odata.id")]
    pub id: String,
    #[serde(rename = "odata.etag")]
    pub etag: String,
    #[serde(rename = "odata.editLink")]
    pub edit_link: String,
}
