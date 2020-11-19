use crate::prelude::*;
use crate::responses::ReplaceStoredProcedureResponse;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReplaceStoredProcedureBuilder<'a, 'b, BodySet>
where
    BodySet: ToAssign,
{
    stored_procedure_client: &'a StoredProcedureClient,
    p_body: PhantomData<BodySet>,
    body: Option<&'b str>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> ReplaceStoredProcedureBuilder<'a, 'b, No> {
    pub(crate) fn new(stored_procedure_client: &'a StoredProcedureClient) -> Self {
        Self {
            stored_procedure_client,
            p_body: PhantomData {},
            body: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, BodySet> StoredProcedureClientRequired<'a>
    for ReplaceStoredProcedureBuilder<'a, 'b, BodySet>
where
    BodySet: ToAssign,
{
    fn stored_procedure_client(&self) -> &'a StoredProcedureClient {
        self.stored_procedure_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b> StoredProcedureBodyRequired<'b> for ReplaceStoredProcedureBuilder<'a, 'b, Yes> {
    fn body(&self) -> &'b str {
        self.body.unwrap()
    }
}

impl<'a, 'b, BodySet> UserAgentOption<'b> for ReplaceStoredProcedureBuilder<'a, 'b, BodySet>
where
    BodySet: ToAssign,
{
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, BodySet> ActivityIdOption<'b> for ReplaceStoredProcedureBuilder<'a, 'b, BodySet>
where
    BodySet: ToAssign,
{
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, BodySet> ConsistencyLevelOption<'b> for ReplaceStoredProcedureBuilder<'a, 'b, BodySet>
where
    BodySet: ToAssign,
{
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b> StoredProcedureBodySupport<'b> for ReplaceStoredProcedureBuilder<'a, 'b, No> {
    type O = ReplaceStoredProcedureBuilder<'a, 'b, Yes>;

    fn with_body(self, body: &'b str) -> Self::O {
        ReplaceStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            p_body: PhantomData {},
            body: Some(body),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, BodySet> UserAgentSupport<'b> for ReplaceStoredProcedureBuilder<'a, 'b, BodySet>
where
    BodySet: ToAssign,
{
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a, 'b, BodySet> ActivityIdSupport<'b> for ReplaceStoredProcedureBuilder<'a, 'b, BodySet>
where
    BodySet: ToAssign,
{
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a, 'b, BodySet> ConsistencyLevelSupport<'b> for ReplaceStoredProcedureBuilder<'a, 'b, BodySet>
where
    BodySet: ToAssign,
{
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> ReplaceStoredProcedureBuilder<'a, 'b, Yes> {
    pub async fn execute(&self) -> Result<ReplaceStoredProcedureResponse, AzureError> {
        trace!("ReplaceStoredProcedureBuilder::execute called");

        let req = self
            .stored_procedure_client
            .prepare_request_with_stored_procedure_name(hyper::Method::PUT);

        // add trait headers
        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);

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
        let request = req.body(hyper::Body::from(request))?;

        let (headers, body) = check_status_extract_headers_and_body(
            self.stored_procedure_client()
                .hyper_client()
                .request(request),
            StatusCode::OK,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
