use crate::DocumentAttributes;
use crate::{number_of_read_regions_from_headers, request_charge_from_headers};
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::session_token_from_headers;
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
    #[inline]
    pub fn id(&self) -> &str {
        self.document_attributes.id()
    }

    pub fn new(id: String, t: T) -> Self {
        let mut document_attributes = DocumentAttributes::default();
        document_attributes.id = id;

        Self {
            document_attributes,
            document: t,
        }
    }
}

impl<T> DocumentName for Document<T>
where
    T: std::fmt::Debug,
{
    fn name(&self) -> &str {
        self.id()
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

#[derive(Debug, Clone)]
pub struct DocumentAdditionalHeaders {
    pub charge: f64,
    pub session_token: String,
    pub number_of_read_regions: u32,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for DocumentAdditionalHeaders {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        DocumentAdditionalHeaders::try_from(value.0)
    }
}

impl std::convert::TryFrom<&HeaderMap> for DocumentAdditionalHeaders {
    type Error = AzureError;
    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        debug!("headers == {:?}", headers);
        let dah = DocumentAdditionalHeaders {
            charge: request_charge_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(headers)?,
        };

        debug!("dah == {:?}", dah);
        Ok(dah)
    }
}
