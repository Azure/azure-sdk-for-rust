use crate::prelude::*;
use crate::responses::CreateDatabaseResponse;
use crate::ResourceType;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateDatabaseBuilder<'a> {
    cosmos_client: &'a CosmosClient,
    database_name: &'a dyn DatabaseName,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a> CreateDatabaseBuilder<'a> {
    pub(crate) fn new(
        cosmos_client: &'a CosmosClient,
        database_name: &'a dyn DatabaseName,
    ) -> Self {
        Self {
            cosmos_client,
            database_name,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }

    pub fn with_database_name(self, database_name: &'a dyn DatabaseName) -> Self {
        Self {
            database_name,
            ..self
        }
    }

    pub fn with_user_agent(self, user_agent: &'a str) -> Self {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }

    pub fn with_activity_id(self, activity_id: &'a str) -> Self {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }

    pub fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }

    // methods callable only when every mandatory field has been filled
    pub async fn execute(&self) -> Result<CreateDatabaseResponse, CosmosError> {
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
            http::Method::POST,
            ResourceType::Databases,
        );

        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

        let request = request.body(req.as_bytes())?; // todo: set content-length here and elsewhere without builders

        debug!("create database request prepared == {:?}", request);

        Ok(self
            .cosmos_client()
            .http_client()
            .execute_request_check_status(request, StatusCode::CREATED)
            .await?
            .try_into()?)
    }
}

impl<'a> CosmosClientRequired<'a> for CreateDatabaseBuilder<'a> {
    fn cosmos_client(&self) -> &'a CosmosClient {
        self.cosmos_client
    }
}

impl<'a> DatabaseNameRequired<'a> for CreateDatabaseBuilder<'a> {
    fn database_name(&self) -> &'a dyn DatabaseName {
        self.database_name
    }
}

impl<'a> UserAgentOption<'a> for CreateDatabaseBuilder<'a> {
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a> ActivityIdOption<'a> for CreateDatabaseBuilder<'a> {
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a> ConsistencyLevelOption<'a> for CreateDatabaseBuilder<'a> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}
