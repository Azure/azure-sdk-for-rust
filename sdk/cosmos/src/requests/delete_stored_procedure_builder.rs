use crate::prelude::*;
use crate::responses::DeleteStoredProcedureResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteStoredProcedureBuilder<'a, 'b> {
    stored_procedure_client: &'a StoredProcedureClient,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> DeleteStoredProcedureBuilder<'a, 'b> {
    pub(crate) fn new(stored_procedure_client: &'a StoredProcedureClient) -> Self {
        Self {
            stored_procedure_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }

    pub fn with_user_agent(self, user_agent: &'b str) -> Self {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }

    pub fn with_activity_id(self, activity_id: &'b str) -> Self {
        Self {
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

    pub async fn execute(&self) -> Result<DeleteStoredProcedureResponse, CosmosError> {
        trace!("DeleteStoredProcedureBuilder::execute called");

        let request = self
            .stored_procedure_client
            .prepare_request_with_stored_procedure_name(http::Method::DELETE);

        // add trait headers
        let request = azure_core::headers::add_optional_header(&self.user_agent(), request);
        let request = azure_core::headers::add_optional_header(&self.activity_id(), request);
        let request = azure_core::headers::add_optional_header(&self.consistency_level(), request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        Ok(self
            .stored_procedure_client()
            .http_client()
            .execute_request_check_status(request, StatusCode::NO_CONTENT)
            .await?
            .try_into()?)
    }

    fn stored_procedure_client(&self) -> &'a StoredProcedureClient {
        self.stored_procedure_client
    }

    fn user_agent(&self) -> Option<azure_core::UserAgent<'b>> {
        self.user_agent
    }

    fn activity_id(&self) -> Option<azure_core::ActivityId<'b>> {
        self.activity_id
    }

    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}
