use std::marker::PhantomData;

use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::stored_procedure::Parameters;

use azure_core::headers::session_token_from_headers;
use azure_core::prelude::*;
use azure_core::{Response as HttpResponse, SessionToken};
use bytes::Bytes;
use serde::de::DeserializeOwned;
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct ExecuteStoredProcedureBuilder<T> {
    client: StoredProcedureClient,
    parameters: Option<Parameters>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: TentativeWritesAllowance,
    partition_key: Option<String>,
    context: Context,
    _stored_proc: PhantomData<T>,
}

static EMPTY_LIST: &[u8; 2] = b"[]";

impl<T> ExecuteStoredProcedureBuilder<T> {
    pub(crate) fn new(client: StoredProcedureClient) -> Self {
        Self {
            client,
            parameters: None,
            consistency_level: None,
            allow_tentative_writes: TentativeWritesAllowance::Deny,
            partition_key: None,
            context: Context::new(),
            _stored_proc: PhantomData,
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        allow_tentative_writes: TentativeWritesAllowance,
        parameters: Parameters => Some(parameters),
        context: Context,
    }

    pub fn partition_key<PK: serde::Serialize>(self, pk: &PK) -> azure_core::Result<Self> {
        Ok(Self {
            partition_key: Some(crate::cosmos_entity::serialize_partition_key(pk)?),
            ..self
        })
    }
}

impl<T: DeserializeOwned + Send> ExecuteStoredProcedureBuilder<T> {
    pub fn into_future(self) -> ExecuteStoredProcedure<T> {
        Box::pin(async move {
            let mut request = self
                .client
                .stored_procedure_request(azure_core::Method::Post);

            if let Some(pk) = self.partition_key.as_ref() {
                crate::cosmos_entity::add_as_partition_key_header_serialized(pk, &mut request)
            }

            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }
            request.insert_headers(&self.allow_tentative_writes);

            let body = if let Some(parameters) = self.parameters.as_ref() {
                Bytes::from(parameters.to_json())
            } else {
                Bytes::from_static(EMPTY_LIST)
            };

            request.set_body(body);

            let response = self
                .client
                .cosmos_client()
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::StoredProcedures),
                    &mut request,
                )
                .await?;

            ExecuteStoredProcedureResponse::try_from(response).await
        })
    }
}

azure_core::future!(ExecuteStoredProcedure<T>);

#[cfg(feature = "into_future")]
impl<T: DeserializeOwned + Send> std::future::IntoFuture for ExecuteStoredProcedureBuilder<T> {
    type IntoFuture = ExecuteStoredProcedure<T>;
    type Output = <ExecuteStoredProcedure<T> as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
#[derive(Debug, Clone)]
pub struct ExecuteStoredProcedureResponse<T>
where
    T: DeserializeOwned,
{
    pub payload: T,

    pub last_state_change: OffsetDateTime,
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
    pub cosmos_quorum_acked_llsn: Option<u64>,
    pub session_token: SessionToken,
    pub charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: OffsetDateTime,
}

impl<T> ExecuteStoredProcedureResponse<T>
where
    T: DeserializeOwned,
{
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        Ok(Self {
            payload: serde_json::from_slice(&body)?,
            last_state_change: last_state_change_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?,
            alt_content_path: alt_content_path_from_headers(&headers)?,
            content_path: content_path_from_headers(&headers)?,
            quorum_acked_lsn: quorum_acked_lsn_from_headers(&headers)?,
            current_write_quorum: current_write_quorum_from_headers(&headers)?,
            current_replica_set_size: current_replica_set_size_from_headers(&headers)?,
            role: role_from_headers(&headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            transport_request_id: transport_request_id_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            cosmos_quorum_acked_llsn: cosmos_quorum_acked_llsn_from_headers_optional(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            charge: request_charge_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?,
            date: date_from_headers(&headers)?,
        })
    }
}
