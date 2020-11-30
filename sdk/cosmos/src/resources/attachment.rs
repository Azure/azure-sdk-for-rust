/// Special documents that contain references and associated metadata to an external blob or media file.
///
/// You can find more information about attachments in Cosmos [here](https://docs.microsoft.com/en-us/rest/api/cosmos-db/attachments)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Attachment {
    pub id: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub media: String,
    #[serde(rename = "_rid")]
    rid: String,
    #[serde(rename = "_ts")]
    pub timestamp: u64,
    #[serde(rename = "_self")]
    pub url: String,
    #[serde(rename = "_etag")]
    pub etag: String,
}
