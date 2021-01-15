use crate::prelude::*;
use crate::resources::stored_procedure::Parameters;
use crate::responses::ExecuteStoredProcedureResponse;
use azure_core::prelude::*;
use bytes::Bytes;
use http::StatusCode;
use serde::de::DeserializeOwned;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ExecuteStoredProcedureBuilder<'a, 'b> {
    stored_procedure_client: &'a StoredProcedureClient,
    parameters: Option<Parameters>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: TenativeWritesAllowance,
    partition_keys: Option<&'b PartitionKeys>,
}

static EMPTY_LIST: &[u8; 2] = b"[]";

impl<'a, 'b> ExecuteStoredProcedureBuilder<'a, 'b> {
    pub(crate) fn new(stored_procedure_client: &'a StoredProcedureClient) -> Self {
        Self {
            stored_procedure_client,
            parameters: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            allow_tentative_writes: TenativeWritesAllowance::Deny,
            partition_keys: None,
        }
    }

    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        allow_tentative_writes: TenativeWritesAllowance,
        partition_keys: &'b PartitionKeys => Some(partition_keys),
    }

    pub fn parameters<P: Into<Parameters>>(self, p: P) -> Self {
        Self {
            parameters: Some(p.into()),
            ..self
        }
    }

    pub async fn execute<T>(&self) -> Result<ExecuteStoredProcedureResponse<T>, CosmosError>
    where
        T: DeserializeOwned,
    {
        trace!("ExecuteStoredProcedureBuilder::execute called");

        let request = self
            .stored_procedure_client
            .prepare_request_with_stored_procedure_name(http::Method::POST);

        // add trait headers
        let request = azure_core::headers::add_optional_header(&self.user_agent, request);
        let request = azure_core::headers::add_optional_header(&self.activity_id, request);
        let request = azure_core::headers::add_optional_header(&self.consistency_level, request);
        let request =
            azure_core::headers::add_mandatory_header(&self.allow_tentative_writes, request);
        let request = azure_core::headers::add_optional_header(&self.partition_keys, request);

        let request = request.header(http::header::CONTENT_TYPE, "application/json");

        let body = if let Some(parameters) = self.parameters.as_ref() {
            Bytes::from(parameters.to_json())
        } else {
            Bytes::from_static(EMPTY_LIST)
        };

        let request = request.body(body)?;

        Ok(self
            .stored_procedure_client
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()?)
    }
}
