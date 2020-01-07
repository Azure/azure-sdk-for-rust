use crate::document_attributes::DocumentAttributes;
use crate::{activity_id_from_headers, request_charge_from_headers};
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{continuation_token_from_headers_optional, session_token_from_headers};
use hyper::header::HeaderMap;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::convert::TryInto;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResult<T> {
    #[serde(flatten)]
    pub document_attributes: DocumentAttributes,
    #[serde(flatten)]
    pub result: T,
}

impl<T> std::convert::TryFrom<(&HeaderMap, &[u8])> for QueryResult<T>
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

#[derive(Debug, Clone, PartialEq)]
pub struct QueryDocumentsResponseAdditonalHeaders {
    pub continuation_token: Option<String>,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for QueryDocumentsResponseAdditonalHeaders {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let _body = value.1;

        Ok(QueryDocumentsResponseAdditonalHeaders {
            continuation_token: continuation_token_from_headers_optional(headers)?,
            charge: request_charge_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct QueryResponseMeta {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_count")]
    pub count: u64,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for QueryResponseMeta {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let _headers = value.0;
        let body = value.1;

        Ok(serde_json::from_slice(body)?)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QueryDocumentsResponse<T> {
    pub query_response_meta: QueryResponseMeta,
    pub results: Vec<QueryResult<T>>,
    pub additional_headers: QueryDocumentsResponseAdditonalHeaders,
}

impl<T> std::convert::TryFrom<(&HeaderMap, &[u8])> for QueryDocumentsResponse<T>
where
    T: DeserializeOwned,
{
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        debug!("headers == {:?}", headers);
        debug!("body == {}", std::str::from_utf8(body)?);

        let inner: Value = serde_json::from_slice(body)?;
        let mut results = Vec::new();
        if let Value::Array(documents) = &inner["Documents"] {
            for doc in documents {
                let document_attributes: DocumentAttributes =
                    serde_json::from_value(doc.to_owned())?;
                let result: T = serde_json::from_value(doc.to_owned())?;
                results.push(QueryResult {
                    document_attributes: document_attributes,
                    result,
                });
            }
        }

        Ok(QueryDocumentsResponse {
            query_response_meta: value.try_into()?,
            results,
            additional_headers: value.try_into()?,
        })
    }
}
