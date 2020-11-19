use crate::prelude::*;
use crate::responses::ReplaceStoredProcedureResponse;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use http::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReplaceStoredProcedureBuilder<'a, 'b, C, D, COLL, BodySet>
where
    BodySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    stored_procedure_client: &'a dyn StoredProcedureClient<C, D, COLL>,
    p_body: PhantomData<BodySet>,
    body: Option<&'b str>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b, C, D, COLL> ReplaceStoredProcedureBuilder<'a, 'b, C, D, COLL, No>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    pub(crate) fn new(
        stored_procedure_client: &'a dyn StoredProcedureClient<C, D, COLL>,
    ) -> ReplaceStoredProcedureBuilder<'a, 'b, C, D, COLL, No> {
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

impl<'a, 'b, C, D, COLL, BodySet> StoredProcedureClientRequired<'a, C, D, COLL>
    for ReplaceStoredProcedureBuilder<'a, 'b, C, D, COLL, BodySet>
where
    BodySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn stored_procedure_client(&self) -> &'a dyn StoredProcedureClient<C, D, COLL> {
        self.stored_procedure_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, C, D, COLL> StoredProcedureBodyRequired<'b>
    for ReplaceStoredProcedureBuilder<'a, 'b, C, D, COLL, Yes>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn body(&self) -> &'b str {
        self.body.unwrap()
    }
}

impl<'a, 'b, C, D, COLL, BodySet> UserAgentOption<'b>
    for ReplaceStoredProcedureBuilder<'a, 'b, C, D, COLL, BodySet>
where
    BodySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, C, D, COLL, BodySet> ActivityIdOption<'b>
    for ReplaceStoredProcedureBuilder<'a, 'b, C, D, COLL, BodySet>
where
    BodySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, C, D, COLL, BodySet> ConsistencyLevelOption<'b>
    for ReplaceStoredProcedureBuilder<'a, 'b, C, D, COLL, BodySet>
where
    BodySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, C, D, COLL> StoredProcedureBodySupport<'b>
    for ReplaceStoredProcedureBuilder<'a, 'b, C, D, COLL, No>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = ReplaceStoredProcedureBuilder<'a, 'b, C, D, COLL, Yes>;

    #[inline]
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

impl<'a, 'b, C, D, COLL, BodySet> UserAgentSupport<'b>
    for ReplaceStoredProcedureBuilder<'a, 'b, C, D, COLL, BodySet>
where
    BodySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = ReplaceStoredProcedureBuilder<'a, 'b, C, D, COLL, BodySet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
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

impl<'a, 'b, C, D, COLL, BodySet> ActivityIdSupport<'b>
    for ReplaceStoredProcedureBuilder<'a, 'b, C, D, COLL, BodySet>
where
    BodySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = ReplaceStoredProcedureBuilder<'a, 'b, C, D, COLL, BodySet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
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

impl<'a, 'b, C, D, COLL, BodySet> ConsistencyLevelSupport<'b>
    for ReplaceStoredProcedureBuilder<'a, 'b, C, D, COLL, BodySet>
where
    BodySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = ReplaceStoredProcedureBuilder<'a, 'b, C, D, COLL, BodySet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
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
impl<'a, 'b, C, D, COLL> ReplaceStoredProcedureBuilder<'a, 'b, C, D, COLL, Yes>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    pub async fn execute(&self) -> Result<ReplaceStoredProcedureResponse, CosmosError> {
        trace!("ReplaceStoredProcedureBuilder::execute called");

        let req = self
            .stored_procedure_client
            .prepare_request_with_stored_procedure_name(http::Method::PUT);

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
        let request = req.body(request.as_bytes())?;

        Ok(self
            .stored_procedure_client()
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()?)
    }
}
