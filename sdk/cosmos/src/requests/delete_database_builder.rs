use crate::prelude::*;
use crate::responses::DeleteDatabaseResponse;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteDatabaseBuilder<'a> {
    database_client: &'a DatabaseClient,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
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

impl<'a> DatabaseClientRequired<'a> for DeleteDatabaseBuilder<'a> {
    fn database_client(&self) -> &'a DatabaseClient {
        self.database_client
    }
}

impl<'a> UserAgentOption<'a> for DeleteDatabaseBuilder<'a> {
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a> ActivityIdOption<'a> for DeleteDatabaseBuilder<'a> {
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a> ConsistencyLevelOption<'a> for DeleteDatabaseBuilder<'a> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a> UserAgentSupport<'a> for DeleteDatabaseBuilder<'a> {
    type O = Self;

    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a> ActivityIdSupport<'a> for DeleteDatabaseBuilder<'a> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a> ConsistencyLevelSupport<'a> for DeleteDatabaseBuilder<'a> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        DeleteDatabaseBuilder {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a> DeleteDatabaseBuilder<'a> {
    pub async fn execute(&self) -> Result<DeleteDatabaseResponse, AzureError> {
        trace!("DeleteDatabaseResponse::execute called");

        let request = self
            .database_client()
            .prepare_request_with_database_name(hyper::Method::DELETE);

        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

        let request = request.body(hyper::Body::empty())?;

        trace!("request prepared == {:?}", request);

        let future_response = self.database_client().hyper_client().request(request);
        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::NO_CONTENT).await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
