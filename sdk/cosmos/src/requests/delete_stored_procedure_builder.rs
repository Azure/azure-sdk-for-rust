use crate::prelude::*;
use crate::responses::DeleteStoredProcedureResponse;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    stored_procedure_client: &'a dyn StoredProcedureClient<C, D, COLL>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, C, D, COLL> DeleteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    pub(crate) fn new(
        stored_procedure_client: &'a dyn StoredProcedureClient<C, D, COLL>,
    ) -> DeleteStoredProcedureBuilder<'a, 'b, C, D, COLL> {
        DeleteStoredProcedureBuilder {
            stored_procedure_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, C, D, COLL> StoredProcedureClientRequired<'a, C, D, COLL>
    for DeleteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
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
impl<'a, 'b, C, D, COLL> UserAgentOption<'b> for DeleteStoredProcedureBuilder<'a, 'b, C, D, COLL>
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

impl<'a, 'b, C, D, COLL> ActivityIdOption<'b> for DeleteStoredProcedureBuilder<'a, 'b, C, D, COLL>
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
    for DeleteStoredProcedureBuilder<'a, 'b, C, D, COLL>
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

impl<'a, 'b, C, D, COLL> UserAgentSupport<'b> for DeleteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = DeleteStoredProcedureBuilder<'a, 'b, C, D, COLL>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        DeleteStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, COLL> ActivityIdSupport<'b> for DeleteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = DeleteStoredProcedureBuilder<'a, 'b, C, D, COLL>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        DeleteStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, COLL> ConsistencyLevelSupport<'b>
    for DeleteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = DeleteStoredProcedureBuilder<'a, 'b, C, D, COLL>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        DeleteStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C, D, COLL> DeleteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    pub async fn execute(&self) -> Result<DeleteStoredProcedureResponse, AzureError> {
        trace!("DeleteStoredProcedureBuilder::execute called");

        let request = self
            .stored_procedure_client
            .prepare_request_with_stored_procedure_name(http::Method::DELETE);

        // add trait headers
        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        let (headers, body) = check_status_extract_headers_and_body(
            self.stored_procedure_client()
                .http_client()
                .request(request),
            StatusCode::NO_CONTENT,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
