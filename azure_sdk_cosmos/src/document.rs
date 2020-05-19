use crate::DocumentAttributes;
use azure_sdk_core::errors::AzureError;
use hyper::header::HeaderMap;
use serde::de::DeserializeOwned;

pub trait DocumentName: std::fmt::Debug {
    fn name(&self) -> &str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document<T> {
    #[serde(flatten)]
    pub document_attributes: DocumentAttributes,
    #[serde(flatten)]
    pub document: T, // raw, id not included
}

impl<T> Document<T> {
    pub fn new(t: T) -> Self {
        let document_attributes = DocumentAttributes::default();

        Self {
            document_attributes,
            document: t,
        }
    }
}

impl DocumentName for &str {
    fn name(&self) -> &str {
        self
    }
}

impl DocumentName for String {
    fn name(&self) -> &str {
        self.as_ref()
    }
}

impl DocumentName for std::borrow::Cow<'_, str> {
    fn name(&self) -> &str {
        self.as_ref()
    }
}

impl<T> std::convert::TryFrom<(&HeaderMap, &[u8])> for Document<T>
where
    T: DeserializeOwned,
{
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let _headers = value.0;
        let body = value.1;

        Ok(serde_json::from_slice(body)?)
    }
}
