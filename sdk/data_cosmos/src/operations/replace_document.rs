use crate::cosmos_entity::{add_as_partition_key_header_serialized, serialize_partition_key};
use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::document::DocumentAttributes;
use crate::ResourceQuota;

use azure_core::headers::session_token_from_headers;
use azure_core::prelude::*;
use azure_core::SessionToken;
use azure_core::{collect_pinned_stream, Response as HttpResponse};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct ReplaceDocumentBuilder<D> {
    client: DocumentClient,
    document: D,
    partition_key: Option<String>,
    indexing_directive: IndexingDirective,
    if_match_condition: Option<IfMatchCondition>,
    if_modified_since: Option<IfModifiedSince>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: TentativeWritesAllowance,
    context: Context,
}

impl<D: Serialize + Send + 'static> ReplaceDocumentBuilder<D> {
    pub(crate) fn new(client: DocumentClient, document: D) -> Self {
        Self {
            client,
            document,
            partition_key: None,
            indexing_directive: IndexingDirective::Default,
            if_match_condition: None,
            if_modified_since: None,
            consistency_level: None,
            allow_tentative_writes: TentativeWritesAllowance::Deny,
            context: Context::new(),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        if_modified_since: DateTime<Utc> => Some(IfModifiedSince::new(if_modified_since)),
        allow_tentative_writes: TentativeWritesAllowance,
        indexing_directive: IndexingDirective,
        context: Context => context,
    }

    pub fn partition_key<T: Serialize>(&mut self, partition_key: &T) -> azure_core::Result<()> {
        self.partition_key = Some(serialize_partition_key(partition_key)?);
        Ok(())
    }

    pub fn into_future(self) -> ReplaceDocument {
        Box::pin(async move {
            let mut request = self.client.document_request(azure_core::Method::Put);

            let partition_key = self
                .partition_key
                .as_deref()
                .unwrap_or_else(|| self.client.partition_key_serialized());
            add_as_partition_key_header_serialized(partition_key, &mut request);

            request.insert_headers(&self.indexing_directive);
            request.insert_headers(&self.if_match_condition);
            request.insert_headers(&self.if_modified_since);
            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }
            request.insert_headers(&self.allow_tentative_writes);

            let serialized = azure_core::to_json(&self.document)?;
            request.set_body(serialized);

            let response = self
                .client
                .cosmos_client()
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Documents),
                    &mut request,
                )
                .await?;

            ReplaceDocumentResponse::try_from(response).await
        })
    }
}

/// The future returned by calling `into_future` on the builder.
pub type ReplaceDocument =
    futures::future::BoxFuture<'static, azure_core::Result<ReplaceDocumentResponse>>;

#[cfg(feature = "into_future")]
impl<D: Serialize + Send + 'static> std::future::IntoFuture for ReplaceDocumentBuilder<D> {
    type IntoFuture = ReplaceDocument;
    type Output = <ReplaceDocument as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct ReplaceDocumentResponse {
    pub document_attributes: DocumentAttributes,
    pub content_location: String,
    pub last_state_change: DateTime<Utc>,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
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
}

impl ReplaceDocumentResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;
        let document_attributes = serde_json::from_slice(&*body)?;

        Ok(Self {
            content_location: content_location_from_headers(&headers)?.to_owned(),
            last_state_change: last_state_change_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            lsn: lsn_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(&headers)?.to_owned(),
            content_path: content_path_from_headers(&headers)?.to_owned(),
            quorum_acked_lsn: quorum_acked_lsn_from_headers_optional(&headers)?,
            current_write_quorum: current_write_quorum_from_headers_optional(&headers)?,
            current_replica_set_size: current_replica_set_size_from_headers_optional(&headers)?,
            role: role_from_headers(&headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            transport_request_id: transport_request_id_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            cosmos_quorum_acked_llsn: cosmos_quorum_acked_llsn_from_headers_optional(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            charge: request_charge_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?.to_owned(),
            activity_id: activity_id_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?.to_owned(),
            date: date_from_headers(&headers)?,
            document_attributes,
        })
    }
}
