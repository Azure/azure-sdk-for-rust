use crate::prelude::*;
use crate::resources::stored_procedure::Parameters;
use crate::responses::ExecuteStoredProcedureResponse;
use http::StatusCode;
use serde::de::DeserializeOwned;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ExecuteStoredProcedureBuilder<'a, 'b> {
    stored_procedure_client: &'a StoredProcedureClient,
    parameters: Option<&'b Parameters>,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
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

    fn stored_procedure_client(&self) -> &'a StoredProcedureClient {
        self.stored_procedure_client
    }

    fn parameters(&self) -> Option<&'b Parameters> {
        self.parameters
    }

    fn user_agent(&self) -> Option<azure_core::UserAgent<'b>> {
        self.user_agent
    }

    fn activity_id(&self) -> Option<azure_core::ActivityId<'b>> {
        self.activity_id
    }

    fn partition_keys(&self) -> Option<&'b PartitionKeys> {
        self.partition_keys
    }

    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }

    fn allow_tentative_writes(&self) -> TenativeWritesAllowance {
        self.allow_tentative_writes
    }

    pub fn with_parameters(self, parameters: &'b Parameters) -> Self {
        Self {
            parameters: Some(parameters),
            ..self
        }
    }

    pub fn with_user_agent(self, user_agent: &'b str) -> Self {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }

    pub fn with_activity_id(self, activity_id: &'b str) -> Self {
        ExecuteStoredProcedureBuilder {
            activity_id: Some(azure_core::ActivityId::new(activity_id)),
            ..self
        }
    }

    pub fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }

    pub fn with_allow_tentative_writes(
        self,
        allow_tentative_writes: TenativeWritesAllowance,
    ) -> Self {
        Self {
            allow_tentative_writes,
            ..self
        }
    }

    pub fn with_partition_keys(self, partition_keys: &'b PartitionKeys) -> Self {
        Self {
            partition_keys: Some(partition_keys),
            ..self
        }
    }

    pub async fn execute<T>(&self) -> Result<ExecuteStoredProcedureResponse<T>, CosmosError>
    where
        T: DeserializeOwned,
    {
        trace!("ExecuteStoredProcedureBuilder::execute called");

        let request = self
            .stored_procedure_client()
            .prepare_request_with_stored_procedure_name(http::Method::POST);

        // add trait headers
        let request = crate::headers::add_optional_header(self.user_agent(), request);
        let request = crate::headers::add_optional_header(self.activity_id(), request);
        let request = crate::headers::add_optional_header(self.consistency_level(), request);
        let request =
            crate::headers::add_optional_header(Some(self.allow_tentative_writes()), request);
        let request = crate::headers::add_optional_header(self.partition_keys(), request);

        let request = request.header(http::header::CONTENT_TYPE, "application/json");

        let body = if let Some(parameters) = self.parameters() {
            parameters.to_json()
        } else {
            String::from("[]")
        };

        let request = request.body(body.as_bytes())?;

        Ok(self
            .stored_procedure_client()
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()?)
    }
}
