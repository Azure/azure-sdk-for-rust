use crate::clients::{CosmosUriBuilder, ResourceType};
use crate::prelude::*;
use crate::responses::DeleteStoredProcedureResponse;
use crate::StoredProcedureClient;
use crate::StoredProcedureClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteStoredProcedureBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    stored_procedure_client: &'a StoredProcedureClient<'a, CUB>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel<'a>>,
}

impl<'a, CUB> DeleteStoredProcedureBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        stored_procedure_client: &'a StoredProcedureClient<'a, CUB>,
    ) -> DeleteStoredProcedureBuilder<'a, CUB> {
        DeleteStoredProcedureBuilder {
            stored_procedure_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, CUB> StoredProcedureClientRequired<'a, CUB> for DeleteStoredProcedureBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn stored_procedure_client(&self) -> &'a StoredProcedureClient<'a, CUB> {
        self.stored_procedure_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, CUB> UserAgentOption<'a> for DeleteStoredProcedureBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, CUB> ActivityIdOption<'a> for DeleteStoredProcedureBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, CUB> ConsistencyLevelOption<'a> for DeleteStoredProcedureBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'a>> {
        self.consistency_level
    }
}

impl<'a, CUB> UserAgentSupport<'a> for DeleteStoredProcedureBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = DeleteStoredProcedureBuilder<'a, CUB>;

    #[inline]
    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        DeleteStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB> ActivityIdSupport<'a> for DeleteStoredProcedureBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = DeleteStoredProcedureBuilder<'a, CUB>;

    #[inline]
    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        DeleteStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB> ConsistencyLevelSupport<'a> for DeleteStoredProcedureBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = DeleteStoredProcedureBuilder<'a, CUB>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'a>) -> Self::O {
        DeleteStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> DeleteStoredProcedureBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<DeleteStoredProcedureResponse, AzureError> {
        trace!("DeleteStoredProcedureBuilder::execute called");

        let req = self.stored_procedure_client.main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/sprocs/{}",
                self.stored_procedure_client.database_name().name(),
                self.stored_procedure_client.collection_name().name(),
                self.stored_procedure_client.stored_procedure_name().name()
            ),
            hyper::Method::DELETE,
            ResourceType::StoredProcedures,
        );

        // add trait headers
        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);

        let req = req.body(hyper::Body::empty())?;

        let (headers, body) = check_status_extract_headers_and_body(
            self.stored_procedure_client().hyper_client().request(req),
            StatusCode::NO_CONTENT,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
