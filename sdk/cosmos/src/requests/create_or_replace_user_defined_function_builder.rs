use crate::prelude::*;
use crate::responses::CreateUserDefinedFunctionResponse;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, C, D, COLL, BodySet>
where
    BodySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    user_defined_function_client: &'a dyn UserDefinedFunctionClient<C, D, COLL>,
    is_create: bool,
    p_body: PhantomData<BodySet>,
    body: Option<&'b str>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, C, D, COLL> CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, C, D, COLL, No>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    pub(crate) fn new(
        user_defined_function_client: &'a dyn UserDefinedFunctionClient<C, D, COLL>,
        is_create: bool,
    ) -> CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, C, D, COLL, No> {
        CreateOrReplaceUserDefinedFunctionBuilder {
            user_defined_function_client,
            is_create,
            p_body: PhantomData {},
            body: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, C, D, COLL, BodySet> UserDefinedFunctionClientRequired<'a, C, D, COLL>
    for CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, C, D, COLL, BodySet>
where
    BodySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn user_defined_function_client(&self) -> &'a dyn UserDefinedFunctionClient<C, D, COLL> {
        self.user_defined_function_client
    }
}

//set mandatory no traits methods
impl<'a, 'b, C, D, COLL> UserDefinedFunctionBodyRequired<'b>
    for CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, C, D, COLL, Yes>
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
    for CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, C, D, COLL, BodySet>
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
    for CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, C, D, COLL, BodySet>
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
    for CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, C, D, COLL, BodySet>
where
    BodySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, C, D, COLL> UserDefinedFunctionBodySupport<'b>
    for CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, C, D, COLL, No>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, C, D, COLL, Yes>;

    #[inline]
    fn with_body(self, body: &'b str) -> Self::O {
        CreateOrReplaceUserDefinedFunctionBuilder {
            user_defined_function_client: self.user_defined_function_client,
            is_create: self.is_create,
            p_body: PhantomData {},
            body: Some(body),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, COLL, BodySet> UserAgentSupport<'b>
    for CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, C, D, COLL, BodySet>
where
    BodySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, C, D, COLL, BodySet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        CreateOrReplaceUserDefinedFunctionBuilder {
            user_defined_function_client: self.user_defined_function_client,
            is_create: self.is_create,
            p_body: PhantomData {},
            body: self.body,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, COLL, BodySet> ActivityIdSupport<'b>
    for CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, C, D, COLL, BodySet>
where
    BodySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, C, D, COLL, BodySet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        CreateOrReplaceUserDefinedFunctionBuilder {
            user_defined_function_client: self.user_defined_function_client,
            is_create: self.is_create,
            p_body: PhantomData {},
            body: self.body,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, COLL, BodySet> ConsistencyLevelSupport<'b>
    for CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, C, D, COLL, BodySet>
where
    BodySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, C, D, COLL, BodySet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        CreateOrReplaceUserDefinedFunctionBuilder {
            user_defined_function_client: self.user_defined_function_client,
            is_create: self.is_create,
            p_body: PhantomData {},
            body: self.body,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C, D, COLL> CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, C, D, COLL, Yes>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    pub async fn execute(&self) -> Result<CreateUserDefinedFunctionResponse, AzureError> {
        trace!("CreateOrReplaceUserDefinedFunctionBuilder::execute called");

        // Create is POST with no name in the URL. Expected return is CREATED.
        // See https://docs.microsoft.com/en-us/rest/api/cosmos-db/create-a-user-defined-function
        // Replace is PUT with name appended to the URL. Expected return is OK.
        // See: https://docs.microsoft.com/en-us/rest/api/cosmos-db/replace-a-user-defined-function
        let req = match self.is_create {
            true => self
                .user_defined_function_client
                .prepare_request(hyper::Method::POST),
            false => self
                .user_defined_function_client
                .prepare_request_with_user_defined_function_name(hyper::Method::PUT),
        };

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
            id: self
                .user_defined_function_client
                .user_defined_function_name(),
        };

        let request = serde_json::to_string(&request)?;
        let request = req.body(hyper::Body::from(request))?;

        let (headers, body) = check_status_extract_headers_and_body(
            self.user_defined_function_client()
                .http_client()
                .request(request),
            match self.is_create {
                true => StatusCode::CREATED,
                false => StatusCode::OK,
            },
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
