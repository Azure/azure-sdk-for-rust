use crate::prelude::*;
use crate::resources::ResourceType;
use crate::responses::CreateDatabaseResponse;
use azure_core::{ActivityId, No, ToAssign, UserAgent, Yes};
use http::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateDatabaseBuilder<'a, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
{
    cosmos_client: &'a CosmosClient,
    database_name: Option<&'a str>,
    user_agent: Option<azure_core::UserAgent<'a>>,
    activity_id: Option<azure_core::ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
    p_database_name: PhantomData<DatabaseNameSet>,
}

impl<'a> CreateDatabaseBuilder<'a, No> {
    pub(crate) fn new(cosmos_client: &'a CosmosClient) -> Self {
        Self {
            cosmos_client,
            database_name: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            p_database_name: PhantomData,
        }
    }
}

impl<'a, DatabaseNameSet> CreateDatabaseBuilder<'a, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
{
    setters! {
        user_agent:? &'a str => UserAgent::new,
        activity_id:? &'a str => ActivityId::new,
        consistency_level:? ConsistencyLevel,
    }
}

impl<'a> CreateDatabaseBuilder<'a, No> {
    pub fn with_database_name(self, database_name: &'a str) -> CreateDatabaseBuilder<'a, Yes> {
        CreateDatabaseBuilder {
            database_name: Some(database_name),
            cosmos_client: self.cosmos_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            p_database_name: PhantomData,
        }
    }
}

impl<'a> CreateDatabaseBuilder<'a, Yes> {
    pub async fn execute(&self) -> Result<CreateDatabaseResponse, CosmosError> {
        trace!("CreateDatabaseBuilder::execute called");

        #[derive(Serialize, Debug)]
        struct CreateDatabaseRequest<'a> {
            pub id: &'a str,
        }

        let req = serde_json::to_string(&CreateDatabaseRequest {
            id: self.database_name.unwrap(),
        })?;

        let request =
            self.cosmos_client
                .prepare_request("dbs", http::Method::POST, ResourceType::Databases);

        let request = crate::headers::add_header(self.user_agent, request);
        let request = crate::headers::add_header(self.activity_id, request);
        let request = crate::headers::add_header(self.consistency_level.clone(), request);

        let request = request.body(req.as_bytes())?; // todo: set content-length here and elsewhere without builders

        debug!("create database request prepared == {:?}", request);

        Ok(self
            .cosmos_client
            .http_client()
            .execute_request_check_status(request, StatusCode::CREATED)
            .await?
            .try_into()?)
    }
}
