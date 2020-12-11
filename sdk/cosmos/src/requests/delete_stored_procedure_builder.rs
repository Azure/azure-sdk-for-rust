use crate::prelude::*;
use crate::responses::DeleteStoredProcedureResponse;
use azure_core::prelude::*;

use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteStoredProcedureBuilder<'a, 'b> {
    stored_procedure_client: &'a StoredProcedureClient,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
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

    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }

    pub async fn execute(&self) -> Result<DeleteStoredProcedureResponse, CosmosError> {
        trace!("DeleteStoredProcedureBuilder::execute called");

        let request = self
            .stored_procedure_client
            .prepare_request_with_stored_procedure_name(http::Method::DELETE);

        // add trait headers
        let request = azure_core::headers::add_optional_header(&self.user_agent, request);
        let request = azure_core::headers::add_optional_header(&self.activity_id, request);
        let request = azure_core::headers::add_optional_header(&self.consistency_level, request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        Ok(self
            .stored_procedure_client
            .http_client()
            .execute_request_check_status(request, StatusCode::NO_CONTENT)
            .await?
            .try_into()?)
    }
}
