use azure_core::{prelude::IfMatchCondition, CollectedResponse};

/// A document's attributes
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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

impl std::convert::TryFrom<CollectedResponse> for DocumentAttributes {
    type Error = azure_core::error::Error;

    fn try_from(response: CollectedResponse) -> Result<Self, Self::Error> {
        let body = response.body();
        body.try_into()
    }
}

impl std::convert::TryFrom<&bytes::Bytes> for DocumentAttributes {
    type Error = azure_core::error::Error;

    fn try_from(body: &bytes::Bytes) -> Result<Self, Self::Error> {
        let str = std::str::from_utf8(body).unwrap_or("<NON-UTF8>");
        serde_json::from_slice(body).map_err(|e| {
            azure_core::error::Error::full(
                azure_core::error::ErrorKind::DataConversion,
                e,
                format!("failed to convert json '{}' into DocumentAttributes", str),
            )
        })
    }
}

impl<'a> std::convert::From<&'a DocumentAttributes> for IfMatchCondition {
    fn from(document_attributes: &'a DocumentAttributes) -> Self {
        IfMatchCondition::Match(document_attributes.etag.clone())
    }
}
