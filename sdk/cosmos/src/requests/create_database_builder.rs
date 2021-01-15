use crate::prelude::*;
use crate::resources::ResourceType;
use crate::responses::CreateDatabaseResponse;
use azure_core::{ActivityId, UserAgent};
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateDatabaseBuilder<'a> {
    cosmos_client: &'a CosmosClient,
    user_agent: Option<UserAgent<'a>>,
    activity_id: Option<ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a> CreateDatabaseBuilder<'a> {
    pub(crate) fn new(cosmos_client: &'a CosmosClient) -> Self {
        Self {
            cosmos_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a> CreateDatabaseBuilder<'a> {
    setters! {
        user_agent: &'a str => Some(UserAgent::new(user_agent)),
        activity_id: &'a str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }
}

impl<'a> CreateDatabaseBuilder<'a> {
    pub async fn execute<D: AsRef<str>>(
        &self,
        database_name: D,
    ) -> Result<CreateDatabaseResponse, CosmosError> {
        trace!("CreateDatabaseBuilder::execute called");

        #[derive(Serialize, Debug)]
        struct CreateDatabaseRequest<'a> {
            pub id: &'a str,
        }

        let req = serde_json::to_string(&CreateDatabaseRequest {
            id: database_name.as_ref(),
        })?;

        let request =
            self.cosmos_client
                .prepare_request("dbs", http::Method::POST, ResourceType::Databases);

        let request = azure_core::headers::add_optional_header(&self.user_agent, request);
        let request = azure_core::headers::add_optional_header(&self.activity_id, request);
        let request = azure_core::headers::add_optional_header(&self.consistency_level, request);

        let request = request.body(req)?; // todo: set content-length here and elsewhere without builders

        debug!("create database request prepared == {:?}", request);

        Ok(self
            .cosmos_client
            .http_client()
            .execute_request_check_status(request, StatusCode::CREATED)
            .await?
            .try_into()?)
    }
}
