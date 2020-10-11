use crate::prelude::*;
use crate::responses::DeleteUserDefinedFunctionResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteUserDefinedFunctionBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    user_defined_function_client: &'a dyn UserDefinedFunctionClient<C, D, COLL>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, C, D, COLL> DeleteUserDefinedFunctionBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    pub(crate) fn new(
        user_defined_function_client: &'a dyn UserDefinedFunctionClient<C, D, COLL>,
    ) -> DeleteUserDefinedFunctionBuilder<'a, 'b, C, D, COLL> {
        DeleteUserDefinedFunctionBuilder {
            user_defined_function_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, C, D, COLL> UserDefinedFunctionClientRequired<'a, C, D, COLL>
    for DeleteUserDefinedFunctionBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn user_defined_function_client(&self) -> &'a dyn UserDefinedFunctionClient<C, D, COLL> {
        self.user_defined_function_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, C, D, COLL> UserAgentOption<'b>
    for DeleteUserDefinedFunctionBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, C, D, COLL> ActivityIdOption<'b>
    for DeleteUserDefinedFunctionBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, C, D, COLL> ConsistencyLevelOption<'b>
    for DeleteUserDefinedFunctionBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, C, D, COLL> UserAgentSupport<'b>
    for DeleteUserDefinedFunctionBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = DeleteUserDefinedFunctionBuilder<'a, 'b, C, D, COLL>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        DeleteUserDefinedFunctionBuilder {
            user_defined_function_client: self.user_defined_function_client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, COLL> ActivityIdSupport<'b>
    for DeleteUserDefinedFunctionBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = DeleteUserDefinedFunctionBuilder<'a, 'b, C, D, COLL>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        DeleteUserDefinedFunctionBuilder {
            user_defined_function_client: self.user_defined_function_client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, COLL> ConsistencyLevelSupport<'b>
    for DeleteUserDefinedFunctionBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = DeleteUserDefinedFunctionBuilder<'a, 'b, C, D, COLL>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        DeleteUserDefinedFunctionBuilder {
            user_defined_function_client: self.user_defined_function_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C, D, COLL> DeleteUserDefinedFunctionBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    pub async fn execute(&self) -> Result<DeleteUserDefinedFunctionResponse, AzureError> {
        trace!("DeleteUserDefinedFunctionBuilder::execute called");

        let request = self
            .user_defined_function_client
            .prepare_request_with_user_defined_function_name(hyper::Method::DELETE);

        // add trait headers
        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

        let request = request.body(hyper::Body::empty())?;

        let (headers, body) = check_status_extract_headers_and_body(
            self.user_defined_function_client()
                .hyper_client()
                .request(request),
            StatusCode::NO_CONTENT,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
