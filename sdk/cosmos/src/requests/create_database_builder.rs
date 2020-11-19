use crate::prelude::*;
use crate::responses::CreateDatabaseResponse;
use crate::ResourceType;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateDatabaseBuilder<'a, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
{
    cosmos_client: &'a CosmosClient,
    p_database_name: PhantomData<DatabaseNameSet>,
    database_name: Option<&'a dyn DatabaseName>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a> CreateDatabaseBuilder<'a, No> {
    pub(crate) fn new(cosmos_client: &'a CosmosClient) -> Self {
        Self {
            cosmos_client,
            p_database_name: PhantomData,
            database_name: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, DatabaseNameSet> CosmosClientRequired<'a> for CreateDatabaseBuilder<'a, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
{
    fn cosmos_client(&self) -> &'a CosmosClient {
        self.cosmos_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a> DatabaseNameRequired<'a> for CreateDatabaseBuilder<'a, Yes> {
    fn database_name(&self) -> &'a dyn DatabaseName {
        self.database_name.unwrap()
    }
}

impl<'a, DatabaseNameSet> UserAgentOption<'a> for CreateDatabaseBuilder<'a, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
{
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, DatabaseNameSet> ActivityIdOption<'a> for CreateDatabaseBuilder<'a, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
{
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, DatabaseNameSet> ConsistencyLevelOption<'a> for CreateDatabaseBuilder<'a, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
{
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a> DatabaseNameSupport<'a> for CreateDatabaseBuilder<'a, No> {
    type O = CreateDatabaseBuilder<'a, Yes>;

    fn with_database_name(self, database_name: &'a dyn DatabaseName) -> Self::O {
        CreateDatabaseBuilder {
            cosmos_client: self.cosmos_client,
            p_database_name: PhantomData {},
            database_name: Some(database_name),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, DatabaseNameSet> UserAgentSupport<'a> for CreateDatabaseBuilder<'a, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
{
    type O = CreateDatabaseBuilder<'a, DatabaseNameSet>;

    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        CreateDatabaseBuilder {
            cosmos_client: self.cosmos_client,
            p_database_name: PhantomData {},
            database_name: self.database_name,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, DatabaseNameSet> ActivityIdSupport<'a> for CreateDatabaseBuilder<'a, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
{
    type O = CreateDatabaseBuilder<'a, DatabaseNameSet>;

    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        CreateDatabaseBuilder {
            cosmos_client: self.cosmos_client,
            p_database_name: PhantomData {},
            database_name: self.database_name,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, DatabaseNameSet> ConsistencyLevelSupport<'a> for CreateDatabaseBuilder<'a, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
{
    type O = CreateDatabaseBuilder<'a, DatabaseNameSet>;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        CreateDatabaseBuilder {
            cosmos_client: self.cosmos_client,
            p_database_name: PhantomData {},
            database_name: self.database_name,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a> CreateDatabaseBuilder<'a, Yes> {
    pub async fn execute(&self) -> Result<CreateDatabaseResponse, AzureError> {
        trace!("CreateDatabaseBuilder::execute called");

        #[derive(Serialize, Debug)]
        struct CreateDatabaseRequest<'a> {
            pub id: &'a str,
        }

        let req = serde_json::to_string(&CreateDatabaseRequest {
            id: self.database_name().name(),
        })?;

        let request = self.cosmos_client().prepare_request(
            "dbs",
            hyper::Method::POST,
            ResourceType::Databases,
        );

        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

        let request = request.body(hyper::Body::from(req))?; // todo: set content-length here and elsewhere without builders

        debug!("create database request prepared == {:?}", request);

        let future_response = self.cosmos_client().hyper_client().request(request);
        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::CREATED).await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
