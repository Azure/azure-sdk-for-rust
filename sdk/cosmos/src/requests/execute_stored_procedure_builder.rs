use crate::prelude::*;
use crate::responses::ExecuteStoredProcedureResponse;
use crate::stored_procedure::Parameters;
use azure_core::prelude::*;
use http::StatusCode;
use serde::de::DeserializeOwned;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    stored_procedure_client: &'a dyn StoredProcedureClient<C, D, COLL>,
    parameters: Option<&'b Parameters>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
    allow_tentative_writes: bool,
    partition_keys: Option<&'b PartitionKeys>,
}

impl<'a, 'b, C, D, COLL> ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    pub(crate) fn new(
        stored_procedure_client: &'a dyn StoredProcedureClient<C, D, COLL>,
    ) -> ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL> {
        ExecuteStoredProcedureBuilder {
            stored_procedure_client,
            parameters: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            allow_tentative_writes: false,
            partition_keys: None,
        }
    }
}

impl<'a, 'b, C, D, COLL> StoredProcedureClientRequired<'a, C, D, COLL>
    for ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>
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
impl<'a, 'b, C, D, COLL> ParametersOption<'b> for ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn parameters(&self) -> Option<&'b Parameters> {
        self.parameters
    }
}

impl<'a, 'b, C, D, COLL> UserAgentOption<'b> for ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>
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

impl<'a, 'b, C, D, COLL> ActivityIdOption<'b> for ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>
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
    for ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>
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

impl<'a, 'b, C, D, COLL> AllowTentativeWritesOption
    for ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn allow_tentative_writes(&self) -> bool {
        self.allow_tentative_writes
    }
}

impl<'a, 'b, C, D, COLL> PartitionKeysOption<'b>
    for ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn partition_keys(&self) -> Option<&'b PartitionKeys> {
        self.partition_keys
    }
}

impl<'a, 'b, C, D, COLL> ParametersSupport<'b> for ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>;

    #[inline]
    fn with_parameters(self, parameters: &'b Parameters) -> Self::O {
        ExecuteStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            parameters: Some(parameters),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
            partition_keys: self.partition_keys,
        }
    }
}

impl<'a, 'b, C, D, COLL> UserAgentSupport<'b> for ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        ExecuteStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            parameters: self.parameters,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
            partition_keys: self.partition_keys,
        }
    }
}

impl<'a, 'b, C, D, COLL> ActivityIdSupport<'b> for ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        ExecuteStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            parameters: self.parameters,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
            partition_keys: self.partition_keys,
        }
    }
}

impl<'a, 'b, C, D, COLL> ConsistencyLevelSupport<'b>
    for ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        ExecuteStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            parameters: self.parameters,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
            allow_tentative_writes: self.allow_tentative_writes,
            partition_keys: self.partition_keys,
        }
    }
}

impl<'a, 'b, C, D, COLL> AllowTentativeWritesSupport
    for ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>;

    #[inline]
    fn with_allow_tentative_writes(self, allow_tentative_writes: bool) -> Self::O {
        ExecuteStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            parameters: self.parameters,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes,
            partition_keys: self.partition_keys,
        }
    }
}

impl<'a, 'b, C, D, COLL> PartitionKeysSupport<'b>
    for ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>;

    #[inline]
    fn with_partition_keys(self, partition_keys: &'b PartitionKeys) -> Self::O {
        ExecuteStoredProcedureBuilder {
            stored_procedure_client: self.stored_procedure_client,
            parameters: self.parameters,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
            partition_keys: Some(partition_keys),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C, D, COLL> ExecuteStoredProcedureBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    pub async fn execute<T>(&self) -> Result<ExecuteStoredProcedureResponse<T>, CosmosError>
    where
        T: DeserializeOwned,
    {
        trace!("ExecuteStoredProcedureBuilder::execute called");

        let request = self
            .stored_procedure_client()
            .prepare_request_with_stored_procedure_name(http::Method::POST);

        // add trait headers
        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);
        let request = AllowTentativeWritesOption::add_header(self, request);
        let request = PartitionKeysOption::add_header(self, request);

        let request = request.header(http::header::CONTENT_TYPE, "application/json");

        let body = ParametersOption::generate_body(self);

        let request = request.body(body.as_bytes())?;

        Ok(self
            .stored_procedure_client()
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()?)
    }
}
