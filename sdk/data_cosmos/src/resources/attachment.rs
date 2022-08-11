/// Special documents that contain references and associated metadata to an external blob or media file.
///
/// You can find more information about attachments in Cosmos [here](https://docs.microsoft.com/rest/api/cosmos-db/attachments)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Attachment {
    /// The attachment id
    pub id: String,
    /// The content type
    #[serde(rename = "contentType")]
    pub content_type: String,
    /// The media url
    pub media: String,
    /// The resource id
    #[serde(rename = "_rid")]
    rid: String,
    /// The last updated timestamp
    #[serde(rename = "_ts")]
    pub timestamp: u64,
    /// The resource's url
    #[serde(rename = "_self")]
    pub url: String,
    /// The resource's etag used for concurrency control
    #[serde(rename = "_etag")]
    pub etag: String,
}
