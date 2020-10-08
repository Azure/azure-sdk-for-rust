use crate::from_headers::*;
use crate::ResourceQuota;
use crate::{Document, DocumentAttributes};
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{
    continuation_token_from_headers_optional, session_token_from_headers, SessionToken,
};
use chrono::{DateTime, Utc};
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
pub struct ListDocumentsResponse<T> {
    pub rid: String,
    pub documents: Vec<Document<T>>,

    pub content_location: String,
    pub last_state_change: DateTime<Utc>,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
    pub item_count: u32,
    pub schema_version: String,
    pub alt_content_path: String,
    pub content_path: String,
    pub quorum_acked_lsn: Option<u64>,
    pub current_write_quorum: Option<u64>,
    pub current_replica_set_size: Option<u64>,
    pub role: u32,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_quorum_acked_llsn: Option<u64>,
    pub session_token: SessionToken,
    pub charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: DateTime<Utc>,
    pub continuation_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListDocumentsResponseEntities<T> {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "Documents")]
    pub entities: Vec<T>,
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

        debug!("headers == {:#?}", headers);

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
            content_location: content_location_from_headers(headers)?.to_owned(),
            last_state_change: last_state_change_from_headers(headers)?,
            resource_quota: resource_quota_from_headers(headers)?,
            resource_usage: resource_usage_from_headers(headers)?,
            lsn: lsn_from_headers(headers)?,
            item_count: item_count_from_headers(headers)?,
            schema_version: schema_version_from_headers(headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(headers)?.to_owned(),
            content_path: content_path_from_headers(headers)?.to_owned(),
            quorum_acked_lsn: quorum_acked_lsn_from_headers_optional(headers)?,
            current_write_quorum: current_write_quorum_from_headers_optional(headers)?,
            current_replica_set_size: current_replica_set_size_from_headers_optional(headers)?,
            role: role_from_headers(headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(headers)?,
            transport_request_id: transport_request_id_from_headers(headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(headers)?,
            cosmos_quorum_acked_llsn: cosmos_quorum_acked_llsn_from_headers_optional(headers)?,
            session_token: session_token_from_headers(headers)?,
            charge: request_charge_from_headers(headers)?,
            service_version: service_version_from_headers(headers)?.to_owned(),
            activity_id: activity_id_from_headers(headers)?,
            gateway_version: gateway_version_from_headers(headers)?.to_owned(),
            continuation_token: continuation_token_from_headers_optional(headers)?,
            date: date_from_headers(headers)?,
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
