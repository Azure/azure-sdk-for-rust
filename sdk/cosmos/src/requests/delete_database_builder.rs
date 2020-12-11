use crate::prelude::*;
use crate::responses::DeleteDatabaseResponse;
use azure_core::prelude::*;

use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteDatabaseBuilder<'a> {
    database_client: &'a DatabaseClient,
    user_agent: Option<UserAgent<'a>>,
    activity_id: Option<ActivityId<'a>>,
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

    setters! {
        user_agent: &'a str => Some(UserAgent::new(user_agent)),
        activity_id: &'a str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }
}

impl<'a> DeleteDatabaseBuilder<'a> {
    pub async fn execute(&self) -> Result<DeleteDatabaseResponse, CosmosError> {
        trace!("DeleteDatabaseResponse::execute called");

        let request = self
            .database_client
            .prepare_request_with_database_name(http::Method::DELETE);

        let request = azure_core::headers::add_optional_header(&self.user_agent, request);
        let request = azure_core::headers::add_optional_header(&self.activity_id, request);
        let request = azure_core::headers::add_optional_header(&self.consistency_level, request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        trace!("request prepared == {:?}", request);

        Ok(self
            .database_client
            .http_client()
            .execute_request_check_status(request, StatusCode::NO_CONTENT)
            .await?
            .try_into()?)
    }
}
