use crate::prelude::*;
use crate::responses::CreateStoredProcedureResponse;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use http::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateStoredProcedureBuilder<'a, 'b, BodySet>
where
    BodySet: ToAssign,
{
    stored_procedure_client: &'a StoredProcedureClient,
    p_body: PhantomData<BodySet>,
    body: Option<&'a str>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> CreateStoredProcedureBuilder<'a, 'b, No> {
    pub(crate) fn new(stored_procedure_client: &'a StoredProcedureClient) -> Self {
        Self {
            stored_procedure_client,
            p_body: PhantomData,
            body: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, BodySet> CreateStoredProcedureBuilder<'a, 'b, BodySet>
where
    BodySet: ToAssign,
{
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }
}

impl<'a, 'b> CreateStoredProcedureBuilder<'a, 'b, Yes> {
    fn body(&self) -> &'a str {
        self.body.unwrap()
    }
}

impl<'a, 'b> CreateStoredProcedureBuilder<'a, 'b, No> {
    pub fn body(self, body: &'a str) -> CreateStoredProcedureBuilder<'a, 'b, Yes> {
        CreateStoredProcedureBuilder {
            body: Some(body),
            stored_procedure_client: self.stored_procedure_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            p_body: PhantomData,
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> CreateStoredProcedureBuilder<'a, 'b, Yes> {
    pub async fn execute(&self) -> Result<CreateStoredProcedureResponse, CosmosError> {
        trace!("CreateStoredProcedureBuilder::execute called");

        let req = self
            .stored_procedure_client
            .prepare_request(http::Method::POST);

        // add trait headers
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
            body: self.body(),
            id: self.stored_procedure_client.stored_procedure_name(),
        };

        let request = serde_json::to_string(&request)?;
        let request = req.body(request.as_bytes())?;

        Ok(self
            .stored_procedure_client
            .http_client()
            .execute_request_check_status(request, StatusCode::CREATED)
            .await?
            .try_into()?)
    }
}
