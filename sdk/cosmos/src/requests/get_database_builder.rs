use crate::prelude::*;
use crate::responses::GetDatabaseResponse;
use azure_core::prelude::*;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetDatabaseBuilder<'a, 'b> {
    database_client: &'a DatabaseClient,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> GetDatabaseBuilder<'a, 'b> {
    pub(crate) fn new(database_client: &'a DatabaseClient) -> Self {
        Self {
            database_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }

    pub fn database_client(&self) -> &'a DatabaseClient {
        self.database_client
    }

    fn user_agent(&self) -> Option<azure_core::UserAgent<'b>> {
        self.user_agent
    }

    fn activity_id(&self) -> Option<azure_core::ActivityId<'b>> {
        self.activity_id
    }

    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }

    pub fn with_user_agent(self, user_agent: &'b str) -> Self {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }

    pub fn with_activity_id(self, activity_id: &'b str) -> Self {
        Self {
            activity_id: Some(azure_core::ActivityId::new(activity_id)),
            ..self
        }
    }

    pub fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }

    pub async fn execute(&self) -> Result<GetDatabaseResponse, CosmosError> {
        trace!("GetDatabaseResponse::execute called");

        let request = self
            .database_client()
            .prepare_request_with_database_name(http::Method::GET);

        let request = azure_core::headers::add_optional_header(&self.user_agent(), request);
        let request = azure_core::headers::add_optional_header(&self.activity_id(), request);
        let request = azure_core::headers::add_optional_header(&self.consistency_level(), request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        trace!("request prepared == {:?}", request);

        Ok(self
            .database_client()
            .http_client()
            .execute_request_check_status(request, http::StatusCode::OK)
            .await?
            .try_into()?)
    }
}
