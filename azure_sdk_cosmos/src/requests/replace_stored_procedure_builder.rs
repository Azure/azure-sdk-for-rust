use crate::clients::{CosmosUriBuilder, ResourceType};
use crate::prelude::*;
use crate::responses::ReplaceStoredProcedureResponse;
use crate::StoredProcedureClient;
use crate::StoredProcedureClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReplaceStoredProcedureBuilder<'a, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    stored_procedure_client: &'a StoredProcedureClient<'a, CUB>,
    p_body: PhantomData<BodySet>,
    body: Option<&'a str>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel<'a>>,
}

impl<'a, CUB> ReplaceStoredProcedureBuilder<'a, CUB, No>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        stored_procedure_client: &'a StoredProcedureClient<'a, CUB>,
    ) -> ReplaceStoredProcedureBuilder<'a, CUB, No> {
        ReplaceStoredProcedureBuilder {
            stored_procedure_client,
            p_body: PhantomData {},
            body: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, CUB, BodySet> StoredProcedureClientRequired<'a, CUB>
    for ReplaceStoredProcedureBuilder<'a, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn stored_procedure_client(&self) -> &'a StoredProcedureClient<'a, CUB> {
        self.stored_procedure_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, CUB> StoredProcedureBodyRequired<'a> for ReplaceStoredProcedureBuilder<'a, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn body(&self) -> &'a str {
        self.body.unwrap()
    }
}

impl<'a, CUB, BodySet> UserAgentOption<'a> for ReplaceStoredProcedureBuilder<'a, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, CUB, BodySet> ActivityIdOption<'a> for ReplaceStoredProcedureBuilder<'a, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, CUB, BodySet> ConsistencyLevelOption<'a>
    for ReplaceStoredProcedureBuilder<'a, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'a>> {
        self.consistency_level.clone()
    }
}

impl<'a, CUB> StoredProcedureBodySupport<'a> for ReplaceStoredProcedureBuilder<'a, CUB, No>
where
    CUB: CosmosUriBuilder,
{
    type O = ReplaceStoredProcedureBuilder<'a, CUB, Yes>;

    #[inline]
    fn with_body(self, body: &'a str) -> Self::O {
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

impl<'a, CUB, BodySet> UserAgentSupport<'a> for ReplaceStoredProcedureBuilder<'a, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = ReplaceStoredProcedureBuilder<'a, CUB, BodySet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        ReplaceStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            p_body: PhantomData {},
            body: self.body,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, BodySet> ActivityIdSupport<'a> for ReplaceStoredProcedureBuilder<'a, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = ReplaceStoredProcedureBuilder<'a, CUB, BodySet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        ReplaceStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            p_body: PhantomData {},
            body: self.body,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, BodySet> ConsistencyLevelSupport<'a>
    for ReplaceStoredProcedureBuilder<'a, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = ReplaceStoredProcedureBuilder<'a, CUB, BodySet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'a>) -> Self::O {
        ReplaceStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            p_body: PhantomData {},
            body: self.body,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> ReplaceStoredProcedureBuilder<'a, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<ReplaceStoredProcedureResponse, AzureError> {
        trace!("ReplaceStoredProcedureBuilder::execute called");

        let req = self.stored_procedure_client.main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/sprocs/{}",
                self.stored_procedure_client.database_name().name(),
                self.stored_procedure_client.collection_name().name(),
                self.stored_procedure_client.stored_procedure_name().name(),
            ),
            hyper::Method::PUT,
            ResourceType::StoredProcedures,
        );

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
            id: self.stored_procedure_client.stored_procedure_name().name(),
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
