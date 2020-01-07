use crate::clients::{CosmosUriBuilder, StoredProcedureClient};
use crate::prelude::*;
use crate::responses::ExecuteStoredProcedureResponse;
use crate::stored_procedure::Parameters;
use crate::StoredProcedureBuilderTrait;
use crate::StoredProcedureClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use hyper::StatusCode;
use serde::de::DeserializeOwned;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ExecuteStoredProcedureBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    stored_procedure_client: &'a StoredProcedureClient<'a, CUB>,
    parameters: Option<&'b Parameters>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
    allow_tentative_writes: bool,
}

impl<'a, 'b, CUB> ExecuteStoredProcedureBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        stored_procedure_client: &'a StoredProcedureClient<'a, CUB>,
    ) -> ExecuteStoredProcedureBuilder<'a, 'b, CUB> {
        ExecuteStoredProcedureBuilder {
            stored_procedure_client,
            parameters: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            allow_tentative_writes: false,
        }
    }
}

impl<'a, 'b, CUB> StoredProcedureClientRequired<'a, CUB>
    for ExecuteStoredProcedureBuilder<'a, 'b, CUB>
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
impl<'a, 'b, CUB> ParametersOption<'b> for ExecuteStoredProcedureBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn parameters(&self) -> Option<&'b Parameters> {
        self.parameters
    }
}

impl<'a, 'b, CUB> UserAgentOption<'b> for ExecuteStoredProcedureBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, CUB> ActivityIdOption<'b> for ExecuteStoredProcedureBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, CUB> ConsistencyLevelOption<'b> for ExecuteStoredProcedureBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level
    }
}

impl<'a, 'b, CUB> AllowTentativeWritesOption for ExecuteStoredProcedureBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn allow_tentative_writes(&self) -> bool {
        self.allow_tentative_writes
    }
}

impl<'a, 'b, CUB> ParametersSupport<'b> for ExecuteStoredProcedureBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ExecuteStoredProcedureBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_parameters(self, parameters: &'b Parameters) -> Self::O {
        ExecuteStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            parameters: Some(parameters),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, CUB> UserAgentSupport<'b> for ExecuteStoredProcedureBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ExecuteStoredProcedureBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        ExecuteStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            parameters: self.parameters,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, CUB> ActivityIdSupport<'b> for ExecuteStoredProcedureBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ExecuteStoredProcedureBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        ExecuteStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            parameters: self.parameters,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, CUB> ConsistencyLevelSupport<'b> for ExecuteStoredProcedureBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ExecuteStoredProcedureBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        ExecuteStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            parameters: self.parameters,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, CUB> AllowTentativeWritesSupport for ExecuteStoredProcedureBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ExecuteStoredProcedureBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_allow_tentative_writes(self, allow_tentative_writes: bool) -> Self::O {
        ExecuteStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            parameters: self.parameters,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes,
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, CUB> ExecuteStoredProcedureBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute<T>(&self) -> Result<ExecuteStoredProcedureResponse<T>, AzureError>
    where
        T: DeserializeOwned,
    {
        trace!("ExecuteStoredProcedureBuilder::execute called");

        let mut req = self
            .stored_procedure_client()
            .prepare_request(hyper::Method::POST);

        //let mut req = self.stored_procedure_client.main_client().prepare_request(
        //    &format!(
        //        "dbs/{}/colls/{}/sprocs/{}",
        //        self.stored_procedure_client.database_name().name(),
        //        self.stored_procedure_client.collection_name().name(),
        //        self.stored_procedure_client.stored_procedure_name()
        //    ),
        //    hyper::Method::POST,
        //    ResourceType::StoredProcedures,
        //);

        req = req.header(http::header::CONTENT_TYPE, "application/json");

        let body = ParametersOption::generate_body(self);

        let req = req.body(hyper::Body::from(body))?;

        let (headers, body) = check_status_extract_headers_and_body(
            self.stored_procedure_client().hyper_client().request(req),
            StatusCode::OK,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
