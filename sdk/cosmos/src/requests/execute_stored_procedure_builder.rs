use crate::prelude::*;
use crate::resources::stored_procedure::Parameters;
use crate::responses::ExecuteStoredProcedureResponse;
use azure_core::prelude::*;
use http::StatusCode;
use serde::de::DeserializeOwned;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ExecuteStoredProcedureBuilder<'a, 'b> {
    stored_procedure_client: &'a StoredProcedureClient,
    parameters: Option<&'b Parameters>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: TenativeWritesAllowance,
    partition_keys: Option<&'b PartitionKeys>,
}

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
        parameters: &'b Parameters => Some(parameters),
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
        let request = crate::headers::add_header(self.user_agent, request);
        let request = crate::headers::add_header(self.activity_id, request);
        let request = crate::headers::add_header(self.consistency_level.clone(), request);
        let request = crate::headers::add_header(Some(self.allow_tentative_writes), request);
        let request = crate::headers::add_header(self.partition_keys, request);

        let request = request.header(http::header::CONTENT_TYPE, "application/json");

        let body = if let Some(parameters) = self.parameters {
            parameters.to_json()
        } else {
            String::from("[]")
        };

        let request = request.body(body.as_bytes())?;

        Ok(self
            .stored_procedure_client
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()?)
    }
}
