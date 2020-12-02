use crate::prelude::*;
use crate::responses::DeleteDatabaseResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteDatabaseBuilder<'a> {
    database_client: &'a DatabaseClient,
    user_agent: Option<azure_core::UserAgent<'a>>,
    activity_id: Option<azure_core::ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a> DeleteDatabaseBuilder<'a> {
    pub(crate) fn new(database_client: &'a DatabaseClient) -> Self {
        Self {
            database_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a> DeleteDatabaseBuilder<'a> {
    pub fn database_client(&self) -> &'a DatabaseClient {
        self.database_client
    }
}

impl<'a> DeleteDatabaseBuilder<'a> {
    fn user_agent(&self) -> Option<azure_core::UserAgent<'a>> {
        self.user_agent
    }
}

impl<'a> DeleteDatabaseBuilder<'a> {
    fn activity_id(&self) -> Option<azure_core::ActivityId<'a>> {
        self.activity_id
    }
}

impl<'a> DeleteDatabaseBuilder<'a> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a> DeleteDatabaseBuilder<'a> {
    pub fn with_user_agent(self, user_agent: &'a str) -> Self {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }
}

impl<'a> DeleteDatabaseBuilder<'a> {
    pub fn with_activity_id(self, activity_id: &'a str) -> Self {
        Self {
            activity_id: Some(azure_core::ActivityId::new(activity_id)),
            ..self
        }
    }
}

impl<'a> DeleteDatabaseBuilder<'a> {
    pub fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self {
        DeleteDatabaseBuilder {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a> DeleteDatabaseBuilder<'a> {
    pub async fn execute(&self) -> Result<DeleteDatabaseResponse, CosmosError> {
        trace!("DeleteDatabaseResponse::execute called");

        let request = self
            .database_client()
            .prepare_request_with_database_name(http::Method::DELETE);

        let request = crate::headers::add_header(self.user_agent(), request);
        let request = crate::headers::add_header(self.activity_id(), request);
        let request = crate::headers::add_header(self.consistency_level(), request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        trace!("request prepared == {:?}", request);

        Ok(self
            .database_client()
            .http_client()
            .execute_request_check_status(request, StatusCode::NO_CONTENT)
            .await?
            .try_into()?)
    }
}
