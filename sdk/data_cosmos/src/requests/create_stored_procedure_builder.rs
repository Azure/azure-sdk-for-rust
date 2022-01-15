use crate::prelude::*;
use crate::responses::CreateStoredProcedureResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateStoredProcedureBuilder<'a, 'b> {
    stored_procedure_client: &'a StoredProcedureClient,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> CreateStoredProcedureBuilder<'a, 'b> {
    pub(crate) fn new(stored_procedure_client: &'a StoredProcedureClient) -> Self {
        Self {
            stored_procedure_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b> CreateStoredProcedureBuilder<'a, 'b> {
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }
}

impl<'a, 'b> CreateStoredProcedureBuilder<'a, 'b> {
    pub async fn execute<B: AsRef<str>>(
        &self,
        body: B,
    ) -> crate::Result<CreateStoredProcedureResponse> {
        trace!("CreateStoredProcedureBuilder::execute called");

        let req = self
            .stored_procedure_client
            .prepare_request(http::Method::POST);

        let req = azure_core::headers::add_optional_header(&self.user_agent, req);
        let req = azure_core::headers::add_optional_header(&self.activity_id, req);
        let req = azure_core::headers::add_optional_header(&self.consistency_level, req);

        let req = req.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Debug, Serialize)]
        struct Request<'a> {
            body: &'a str,
            id: &'a str,
        }
        let request = Request {
            body: body.as_ref(),
            id: self.stored_procedure_client.stored_procedure_name(),
        };

        let request = azure_core::to_json(&request)?;
        let request = req.body(request)?;

        Ok(self
            .stored_procedure_client
            .http_client()
            .execute_request_check_status(request, StatusCode::CREATED)
            .await?
            .try_into()?)
    }
}
