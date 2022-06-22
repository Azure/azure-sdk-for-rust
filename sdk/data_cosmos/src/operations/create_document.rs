use crate::cosmos_entity::{add_as_partition_key_header_serialized, serialize_partition_key};
use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::document::DocumentAttributes;
use crate::ResourceQuota;
use azure_core::headers::{etag_from_headers, session_token_from_headers};
use azure_core::prelude::*;
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::Serialize;
use std::convert::TryFrom;

use azure_core::{collect_pinned_stream, Response as HttpResponse};

#[derive(Debug, Clone)]
pub struct CreateDocumentBuilder<D> {
    client: CollectionClient,
    is_upsert: IsUpsert,
    indexing_directive: IndexingDirective,
    if_match_condition: Option<IfMatchCondition>,
    if_modified_since: Option<IfModifiedSince>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: TentativeWritesAllowance,
    partition_key: Option<String>,
    document: D,
    context: Context,
}

impl<D: Serialize + CosmosEntity + Send + 'static> CreateDocumentBuilder<D> {
    pub(crate) fn new(client: CollectionClient, document: D) -> Self {
        Self {
            client,
            is_upsert: IsUpsert::No,
            indexing_directive: IndexingDirective::Default,
            if_match_condition: None,
            if_modified_since: None,
            consistency_level: None,
            allow_tentative_writes: TentativeWritesAllowance::Deny,
            partition_key: None,
            document,
            context: Context::new(),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        if_modified_since: DateTime<Utc> => Some(IfModifiedSince::new(if_modified_since)),
        allow_tentative_writes: TentativeWritesAllowance,
        is_upsert: bool => if is_upsert { IsUpsert::Yes } else { IsUpsert::No },
        indexing_directive: IndexingDirective,
        context: Context => context,
    }

    pub fn partition_key<PK: Serialize>(mut self, partition_key: &PK) -> azure_core::Result<Self> {
        self.partition_key = Some(serialize_partition_key(partition_key)?);
        Ok(self)
    }

    pub fn into_future(self) -> CreateDocument {
        Box::pin(async move {
            let document = self.document;
            let serialized = serde_json::to_string(&document)?;
            let partition_key = match self.partition_key {
                Some(s) => s,
                None => serialize_partition_key(&document.partition_key())?,
            };
            let mut request = self.client.prepare_doc_request_pipeline(http::Method::POST);

            add_as_partition_key_header_serialized(&partition_key, &mut request);
            request.insert_headers(&self.if_match_condition);
            request.insert_headers(&self.if_modified_since);
            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }
            request.insert_headers(&self.is_upsert);
            request.insert_headers(&self.indexing_directive);
            request.insert_headers(&self.allow_tentative_writes);

            request.set_body(serialized);
            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Documents),
                    &mut request,
                )
                .await?;

            CreateDocumentResponse::try_from(response).await
        })
    }
}

/// The future returned by calling `into_future` on the builder.
pub type CreateDocument =
    futures::future::BoxFuture<'static, azure_core::Result<CreateDocumentResponse>>;

#[cfg(feature = "into_future")]
impl<D: Serialize + CosmosEntity + Send + 'static> std::future::IntoFuture
    for CreateDocumentBuilder<D>
{
    type IntoFuture = CreateDocument;
    type Output = <CreateDocument as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct CreateDocumentResponse {
    pub document_attributes: DocumentAttributes,
    pub is_update: bool,
    pub last_state_change: DateTime<Utc>,
    pub etag: String,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
    pub schema_version: String,
    pub alt_content_path: String,
    pub content_path: String,
    pub quorum_acked_lsn: u64,
    pub current_write_quorum: u64,
    pub current_replica_set_size: u64,
    pub role: u32,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_quorum_acked_llsn: u64,
    pub session_token: String,
    pub charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: DateTime<Utc>,
}

impl CreateDocumentResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

        Ok(CreateDocumentResponse {
            is_update: status_code == StatusCode::OK,

            last_state_change: last_state_change_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            lsn: lsn_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(&headers)?.to_owned(),
            content_path: content_path_from_headers(&headers)?.to_owned(),
            quorum_acked_lsn: quorum_acked_lsn_from_headers(&headers)?,
            current_write_quorum: current_write_quorum_from_headers(&headers)?,
            current_replica_set_size: current_replica_set_size_from_headers(&headers)?,
            role: role_from_headers(&headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            transport_request_id: transport_request_id_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            cosmos_quorum_acked_llsn: cosmos_quorum_acked_llsn_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            charge: request_charge_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?.to_owned(),
            activity_id: activity_id_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?.to_owned(),
            date: date_from_headers(&headers)?,

            document_attributes: DocumentAttributes::try_from(&body)?,
        })
    }
}
