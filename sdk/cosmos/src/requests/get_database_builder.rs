use crate::prelude::*;
use crate::responses::GetDatabaseResponse;
use azure_core::prelude::*;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetDatabaseBuilder<'a, 'b> {
    database_client: &'a DatabaseClient,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
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

    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }

    pub async fn execute(&self) -> Result<GetDatabaseResponse, CosmosError> {
        trace!("GetDatabaseResponse::execute called");

        let request = self
            .database_client
            .prepare_request_with_database_name(http::Method::GET);

        let request = crate::headers::add_header(self.user_agent, request);
        let request = crate::headers::add_header(self.activity_id, request);
        let request = crate::headers::add_header(self.consistency_level.clone(), request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        trace!("request prepared == {:?}", request);

        Ok(self
            .database_client
            .http_client()
            .execute_request_check_status(request, http::StatusCode::OK)
            .await?
            .try_into()?)
    }
}
