use azure_core::prelude::IfMatchCondition;
use http::response::Response;

/// A document's attributes
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DocumentAttributes {
    #[serde(rename = "_rid")]
    rid: String,
    #[serde(rename = "_ts")]
    ts: u64,
    _self: String,
    #[serde(rename = "_etag")]
    etag: String,
    #[serde(rename = "_attachments")]
    attachments: String,
}

impl DocumentAttributes {
    /// a unique identifier that is also hierarchical per the resource
    /// stack on the resource model.
    pub fn rid(&self) -> &str {
        &self.rid
    }

    /// the last updated timestamp of the resource.
    pub fn ts(&self) -> u64 {
        self.ts
    }

    ///  the unique addressable URI for the resource
    pub fn _self(&self) -> &str {
        &self._self
    }

    /// resource etag required for optimistic concurrency control
    pub fn etag(&self) -> &str {
        &self.etag
    }

    /// the addressable path for the attachments resource
    pub fn attachments(&self) -> &str {
        &self.attachments
    }
}

impl std::convert::TryFrom<Response<bytes::Bytes>> for DocumentAttributes {
    type Error = crate::Error;

    fn try_from(response: Response<bytes::Bytes>) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(response.body())?)
    }
}

impl std::convert::TryFrom<bytes::Bytes> for DocumentAttributes {
    type Error = crate::Error;

    fn try_from(body: bytes::Bytes) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(&body)?)
    }
}

impl<'a> std::convert::From<&'a DocumentAttributes> for IfMatchCondition<'a> {
    fn from(document_attributes: &'a DocumentAttributes) -> Self {
        IfMatchCondition::Match(&document_attributes.etag)
    }
}
