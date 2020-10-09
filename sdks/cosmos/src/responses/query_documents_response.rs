use crate::document_attributes::DocumentAttributes;
use crate::errors::ConversionToDocumentError;
use crate::from_headers::*;
use crate::ResourceQuota;
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{
    continuation_token_from_headers_optional, session_token_from_headers, SessionToken,
};
use chrono::{DateTime, Utc};
use hyper::header::HeaderMap;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::convert::TryInto;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentQueryResult<T> {
    #[serde(flatten)]
    pub document_attributes: DocumentAttributes,
    #[serde(flatten)]
    pub result: T,
}

impl<T> std::convert::TryFrom<(&HeaderMap, &[u8])> for DocumentQueryResult<T>
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum QueryResult<T> {
    Document(DocumentQueryResult<T>),
    Raw(T),
}

#[derive(Debug, Clone, PartialEq)]
pub struct QueryDocumentsResponse<T> {
    pub query_response_meta: QueryResponseMeta,
    pub results: Vec<QueryResult<T>>,

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

impl<T> QueryDocumentsResponse<T> {
    pub fn into_raw(self) -> QueryDocumentsResponseRaw<T> {
        self.into()
    }

    pub fn into_documents(
        self,
    ) -> Result<QueryDocumentsResponseDocuments<T>, ConversionToDocumentError> {
        self.try_into()
    }
}

impl<T> std::convert::TryFrom<(&HeaderMap, &[u8])> for QueryDocumentsResponse<T>
where
    T: DeserializeOwned,
{
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        debug!("headers == {:#?}", headers);
        debug!("body == {}", std::str::from_utf8(body)?);

        let inner: Value = serde_json::from_slice(body)?;
        let mut results = Vec::new();
        if let Value::Array(documents) = &inner["Documents"] {
            for doc in documents {
                let result: T = serde_json::from_value(doc.to_owned())?;
                // If we have all the necessary fields to construct a
                // DocumentQueryResult we use it, otherwise we just add a raw
                // struct.
                // If I can ascertain that we receive *either* QueryResults
                // or a raw documents - but not a mix of the two -
                // we might want to avoid a discriminated union
                // to be handled at runtime.
                match serde_json::from_value(doc.to_owned()) {
                    Ok(document_attributes) => {
                        results.push(QueryResult::Document(DocumentQueryResult {
                            document_attributes,
                            result,
                        }))
                    }
                    Err(error) => {
                        warn!("{:#?}", error);
                        results.push(QueryResult::Raw(result));
                    }
                }
            }
        }

        Ok(QueryDocumentsResponse {
            query_response_meta: value.try_into()?,
            results,
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

#[derive(Debug, Clone, PartialEq)]
pub struct QueryDocumentsResponseRaw<T> {
    pub query_response_meta: QueryResponseMeta,
    pub results: Vec<T>,

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

impl<T> std::convert::From<QueryDocumentsResponse<T>> for QueryDocumentsResponseRaw<T> {
    #[inline]
    fn from(q: QueryDocumentsResponse<T>) -> Self {
        Self {
            query_response_meta: q.query_response_meta,
            results: q
                .results
                .into_iter()
                .map(|r| match r {
                    QueryResult::Document(document) => document.result,
                    QueryResult::Raw(raw) => raw,
                })
                .collect(),
            last_state_change: q.last_state_change,
            resource_quota: q.resource_quota,
            resource_usage: q.resource_usage,
            lsn: q.lsn,
            item_count: q.item_count,
            schema_version: q.schema_version,
            alt_content_path: q.alt_content_path,
            content_path: q.content_path,
            quorum_acked_lsn: q.quorum_acked_lsn,
            current_write_quorum: q.current_write_quorum,
            current_replica_set_size: q.current_replica_set_size,
            role: q.role,
            global_committed_lsn: q.global_committed_lsn,
            number_of_read_regions: q.number_of_read_regions,
            transport_request_id: q.transport_request_id,
            cosmos_llsn: q.cosmos_llsn,
            cosmos_quorum_acked_llsn: q.cosmos_quorum_acked_llsn,
            session_token: q.session_token,
            charge: q.charge,
            service_version: q.service_version,
            activity_id: q.activity_id,
            gateway_version: q.gateway_version,
            continuation_token: q.continuation_token,
            date: q.date,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QueryDocumentsResponseDocuments<T> {
    pub query_response_meta: QueryResponseMeta,
    pub results: Vec<DocumentQueryResult<T>>,

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

impl<T> std::convert::TryFrom<QueryDocumentsResponse<T>> for QueryDocumentsResponseDocuments<T> {
    type Error = ConversionToDocumentError;

    #[inline]
    fn try_from(q: QueryDocumentsResponse<T>) -> Result<Self, Self::Error> {
        // first check if there is a Raw document. In case we bail out
        if let Some(_) = q.results.iter().find(|r| match r {
            QueryResult::Document(_) => false,
            QueryResult::Raw(_) => true,
        }) {
            return Err(ConversionToDocumentError::RawElementFound {});
        }

        Ok(Self {
            query_response_meta: q.query_response_meta,
            results: q
                .results
                .into_iter()
                .map(|r| match r {
                    QueryResult::Document(document) => document,
                    QueryResult::Raw(_) => {
                        panic!("this should have been caugth by the previous check")
                    }
                })
                .collect(),
            last_state_change: q.last_state_change,
            resource_quota: q.resource_quota,
            resource_usage: q.resource_usage,
            lsn: q.lsn,
            item_count: q.item_count,
            schema_version: q.schema_version,
            alt_content_path: q.alt_content_path,
            content_path: q.content_path,
            quorum_acked_lsn: q.quorum_acked_lsn,
            current_write_quorum: q.current_write_quorum,
            current_replica_set_size: q.current_replica_set_size,
            role: q.role,
            global_committed_lsn: q.global_committed_lsn,
            number_of_read_regions: q.number_of_read_regions,
            transport_request_id: q.transport_request_id,
            cosmos_llsn: q.cosmos_llsn,
            cosmos_quorum_acked_llsn: q.cosmos_quorum_acked_llsn,
            session_token: q.session_token,
            charge: q.charge,
            service_version: q.service_version,
            activity_id: q.activity_id,
            gateway_version: q.gateway_version,
            continuation_token: q.continuation_token,
            date: q.date,
        })
    }
}
