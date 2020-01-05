use crate::{
    number_of_read_regions_from_headers, request_charge_from_headers,
    request_item_count_from_headers, Document, DocumentAttributes,
};
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{
    continuation_token_from_headers_optional, etag_from_headers_optional,
    session_token_from_headers, SessionToken,
};
use hyper::header::HeaderMap;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDocumentsResponseAttributes {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "Documents")]
    pub documents: Vec<DocumentAttributes>,
}

#[derive(Debug, Clone)]
pub struct ListDocumentsResponseAdditionalHeaders {
    pub continuation_token: Option<String>,
    pub charge: f64,
    pub etag: Option<String>,
    pub session_token: SessionToken,
    pub item_count: u64,
    pub number_of_read_regions: u32,
}

#[derive(Debug, Clone)]
pub struct ListDocumentsResponse<T> {
    pub rid: String,
    pub documents: Vec<Document<T>>,
    pub additional_headers: ListDocumentsResponseAdditionalHeaders,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListDocumentsResponseEntities<T> {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "Documents")]
    pub entities: Vec<T>,
}

impl std::convert::TryFrom<&HeaderMap> for ListDocumentsResponseAdditionalHeaders {
    type Error = AzureError;
    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        debug!("headers == {:?}", headers);

        let ado = ListDocumentsResponseAdditionalHeaders {
            continuation_token: continuation_token_from_headers_optional(headers)?,
            charge: request_charge_from_headers(headers)?,
            etag: etag_from_headers_optional(headers)?,
            session_token: session_token_from_headers(headers)?,
            item_count: request_item_count_from_headers(headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(headers)?,
        };
        debug!("ado == {:?}", ado);

        Ok(ado)
    }
}

impl std::convert::TryFrom<&[u8]> for ListDocumentsResponseAttributes {
    type Error = AzureError;
    fn try_from(body: &[u8]) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(body)?)
    }
}

impl<T> std::convert::TryFrom<&[u8]> for ListDocumentsResponseEntities<T>
where
    T: DeserializeOwned,
{
    type Error = AzureError;
    fn try_from(body: &[u8]) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(body)?)
    }
}

impl<T> std::convert::TryFrom<(&HeaderMap, &[u8])> for ListDocumentsResponse<T>
where
    T: DeserializeOwned,
{
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;
        debug!("headers == {:?}", headers);

        let ado = ListDocumentsResponseAdditionalHeaders::try_from(headers)?;

        // we will proceed in three steps:
        // 1- Deserialize the result as DocumentAttributes. The extra field will be ignored.
        // 2- Deserialize the result a type T. The extra fields will be ignored.
        // 3- Zip 1 and 2 in the resulting structure.
        // There is a lot of data movement here, let's hope the compiler is smarter than me :)
        let document_attributes = ListDocumentsResponseAttributes::try_from(body)?;
        debug!("document_attributes == {:?}", document_attributes);
        let entries = ListDocumentsResponseEntities::try_from(body)?;

        let documents = document_attributes
            .documents
            .into_iter()
            .zip(entries.entities.into_iter())
            .map(|(da, e)| Document {
                document_attributes: da,
                document: e,
            })
            .collect();

        Ok(ListDocumentsResponse {
            rid: document_attributes.rid,
            documents,
            additional_headers: ado,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: &'static str = "
{
    \"_rid\": \"3iNTAJKxVCk=\",
    \"Documents\": [
        {
            \"color\": \"red\",
            \"myvalue\": \"#f00\",
            \"id\": \"c5d11a65-2e5a-3d9f-4de8-2447259dff38\",
            \"_rid\": \"3iNTAJKxVCkBAAAAAAAAAA==\",
            \"_self\": \"dbs/3iNTAA==/colls/3iNTAJKxVCk=/docs/3iNTAJKxVCkBAAAAAAAAAA==/\",
            \"_etag\": \"\\\"0100eb0a-0000-0c00-0000-5ded4fe30000\\\"\",
            \"_attachments\": \"attachments/\",
            \"_ts\": 1575833571
        },
        {
            \"color\": \"yellow\",
            \"myvalue\": \"#ff0\",
            \"id\": \"894dd5ff-573e-f38a-b8c4-5eae5071c900\",
            \"_rid\": \"3iNTAJKxVCkCAAAAAAAAAA==\",
            \"_self\": \"dbs/3iNTAA==/colls/3iNTAJKxVCk=/docs/3iNTAJKxVCkCAAAAAAAAAA==/\",
            \"_etag\": \"\\\"0100ec0a-0000-0c00-0000-5ded4fe30000\\\"\",
            \"_attachments\": \"attachments/\",
            \"_ts\": 1575833571
        }
    ],
    \"_count\": 7
}";

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct MyStruct {
        id: String,
        color: String,
        myvalue: String,
    }

    #[test]
    fn test_list_document() {
        let _document_attributes =
            serde_json::from_slice::<ListDocumentsResponseAttributes>(BODY.as_bytes()).unwrap();
        let _entries =
            serde_json::from_slice::<ListDocumentsResponseEntities<MyStruct>>(BODY.as_bytes())
                .unwrap();
    }
}
