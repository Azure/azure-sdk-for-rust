use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::Document;
use crate::ResourceQuota;
use azure_core::headers::{etag_from_headers, session_token_from_headers};
use azure_core::prelude::*;
use azure_core::SessionToken;
use azure_core::{collect_pinned_stream, Request as HttpRequest, Response as HttpResponse};
use chrono::{DateTime, Utc};
use http::{HeaderMap, StatusCode};
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct GetDocumentOptions<'a, 'b> {
    document_client: &'a DocumentClient,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<IfModifiedSince<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> GetDocumentOptions<'a, 'b> {
    pub fn new(document_client: &'a DocumentClient) -> Self {
        Self {
            document_client,
            if_match_condition: None,
            if_modified_since: None,
            activity_id: None,
            consistency_level: None,
        }
    }

    setters! {
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'b> => Some(if_match_condition),
        if_modified_since: &'b DateTime<Utc> => Some(IfModifiedSince::new(if_modified_since)),
    }

    pub(crate) fn decorate_request(&self, request: &mut HttpRequest) -> Result<(), crate::Error> {
        // add trait headers
        azure_core::headers::add_optional_header2(&self.if_match_condition, request)?;
        azure_core::headers::add_optional_header2(&self.if_modified_since, request)?;
        azure_core::headers::add_optional_header2(&self.activity_id, request)?;
        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;

        crate::cosmos_entity::add_as_partition_key_header_serialized2(
            self.document_client.partition_key_serialized(),
            request,
        );

        request.set_body(bytes::Bytes::from_static(EMPTY_BODY).into());

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum GetDocumentResponse<T> {
    Found(Box<FoundDocumentResponse<T>>),
    NotFound(Box<NotFoundDocumentResponse>),
}

impl<T> GetDocumentResponse<T>
where
    T: DeserializeOwned,
{
    pub async fn try_from(response: HttpResponse) -> Result<Self, crate::Error> {
        let (status_code, headers, pinned_stream) = response.deconstruct();

        let has_been_found =
            status_code == StatusCode::OK || status_code == StatusCode::NOT_MODIFIED;

        let body = collect_pinned_stream(pinned_stream).await?;

        if has_been_found {
            Ok(GetDocumentResponse::Found(Box::new(
                FoundDocumentResponse::try_from(&headers, body).await?,
            )))
        } else {
            Ok(GetDocumentResponse::NotFound(Box::new(
                NotFoundDocumentResponse::try_from(&headers).await?,
            )))
        }
    }
}

#[derive(Debug, Clone)]
pub struct FoundDocumentResponse<T> {
    pub document: Document<T>,
    pub content_location: String,
    pub last_state_change: DateTime<Utc>,
    pub etag: String,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
    pub schema_version: String,
    pub alt_content_path: String,
    pub content_path: String,
    pub role: u32,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub item_lsn: u64,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_item_llsn: u64,
    pub session_token: SessionToken,
    pub charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: DateTime<Utc>,
}

impl<T> FoundDocumentResponse<T>
where
    T: DeserializeOwned,
{
    async fn try_from(headers: &HeaderMap, body: bytes::Bytes) -> Result<Self, crate::Error> {
        Ok(Self {
            document: serde_json::from_slice(&body)?,

            content_location: content_location_from_headers(headers)?.to_owned(),
            last_state_change: last_state_change_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            resource_quota: resource_quota_from_headers(headers)?,
            resource_usage: resource_usage_from_headers(headers)?,
            lsn: lsn_from_headers(headers)?,
            schema_version: schema_version_from_headers(headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(headers)?.to_owned(),
            content_path: content_path_from_headers(headers)?.to_owned(),
            role: role_from_headers(headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(headers)?,
            item_lsn: item_lsn_from_headers(headers)?,
            transport_request_id: transport_request_id_from_headers(headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(headers)?,
            cosmos_item_llsn: cosmos_item_llsn_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            charge: request_charge_from_headers(headers)?,
            service_version: service_version_from_headers(headers)?.to_owned(),
            activity_id: activity_id_from_headers(headers)?,
            gateway_version: gateway_version_from_headers(headers)?.to_owned(),
            date: date_from_headers(headers)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct NotFoundDocumentResponse {
    pub content_location: String,
    pub last_state_change: DateTime<Utc>,
    pub lsn: u64,
    pub schema_version: String,
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
}

impl NotFoundDocumentResponse {
    async fn try_from(headers: &HeaderMap) -> Result<Self, crate::Error> {
        Ok(Self {
            content_location: content_location_from_headers(headers)?.to_owned(),
            last_state_change: last_state_change_from_headers(headers)?,
            lsn: lsn_from_headers(headers)?,
            schema_version: schema_version_from_headers(headers)?.to_owned(),
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
            date: date_from_headers(headers)?,
        })
    }
}
