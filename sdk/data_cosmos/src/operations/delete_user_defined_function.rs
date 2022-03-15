use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::ResourceQuota;

use azure_core::headers::session_token_from_headers;
use azure_core::prelude::*;
use azure_core::Response as HttpResponse;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct DeleteUserDefinedFunctionBuilder {
    client: UserDefinedFunctionClient,
    consistency_level: Option<ConsistencyLevel>,
    context: Context,
}

impl DeleteUserDefinedFunctionBuilder {
    pub(crate) fn new(client: UserDefinedFunctionClient) -> Self {
        Self {
            client,
            consistency_level: None,
            context: Context::new(),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        context: Context => context,
    }

    pub fn into_future(self) -> DeleteUserDefinedFunction {
        Box::pin(async move {
            let mut request = self
                .client
                .prepare_pipeline_with_user_defined_function_name(http::Method::DELETE);

            azure_core::headers::add_optional_header2(&self.consistency_level, &mut request)?;

            let response = self
                .client
                .pipeline()
                .send(
                    self.context
                        .clone()
                        .insert(ResourceType::UserDefinedFunctions),
                    &mut request,
                )
                .await?;

            DeleteUserDefinedFunctionResponse::try_from(response).await
        })
    }
}

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for DeleteUserDefinedFunctionBuilder {
    type IntoFuture = DeleteUserDefinedFunction;
    type Output = <DeleteUserDefinedFunction as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

/// The future returned by calling `into_future` on the builder.
pub type DeleteUserDefinedFunction =
    futures::future::BoxFuture<'static, crate::Result<DeleteUserDefinedFunctionResponse>>;

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteUserDefinedFunctionResponse {
    pub content_location: String,
    pub server: String,
    pub last_state_change: DateTime<Utc>,
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

impl DeleteUserDefinedFunctionResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(Self {
            content_location: content_location_from_headers(&headers)?.to_owned(),
            server: server_from_headers(&headers)?.to_owned(),
            last_state_change: last_state_change_from_headers(&headers)?,
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
        })
    }
}
