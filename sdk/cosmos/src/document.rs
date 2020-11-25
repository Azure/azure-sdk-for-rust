use crate::{CosmosError, DocumentAttributes};
use http::header::HeaderMap;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document<T> {
    #[serde(flatten)]
    pub document_attributes: DocumentAttributes,
    #[serde(flatten)]
    pub document: T, // raw, id not included
}

impl<T> Document<T> {
    pub fn new(document: T) -> Self {
        let document_attributes = DocumentAttributes::default();

        Self {
            document_attributes,
            document,
        }
    }
}

impl<T> std::convert::TryFrom<(&HeaderMap, &[u8])> for Document<T>
where
    T: DeserializeOwned,
{
    type Error = CosmosError;
    fn try_from((_, body): (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(body)?)
    }
}
