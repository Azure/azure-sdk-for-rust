use crate::prelude::*;
use crate::responses::CreateUserResponse;
use azure_core::errors::UnexpectedHTTPResult;
use azure_core::errors::{extract_status_headers_and_body, AzureError};
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetUserBuilder<'a, 'b> {
    user_client: &'a UserClient,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> GetUserBuilder<'a, 'b> {
    pub(crate) fn new(user_client: &'a UserClient) -> GetUserBuilder<'a, 'b> {
        Self {
            user_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b> UserClientRequired<'a> for GetUserBuilder<'a, 'b> {
    fn user_client(&self) -> &'a UserClient {
        self.user_client
    }
}

impl<'a, 'b> UserAgentOption<'b> for GetUserBuilder<'a, 'b> {
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b> ActivityIdOption<'b> for GetUserBuilder<'a, 'b> {
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b> ConsistencyLevelOption<'b> for GetUserBuilder<'a, 'b> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b> UserAgentSupport<'b> for GetUserBuilder<'a, 'b> {
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a, 'b> ActivityIdSupport<'b> for GetUserBuilder<'a, 'b> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a, 'b> ConsistencyLevelSupport<'b> for GetUserBuilder<'a, 'b> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> GetUserBuilder<'a, 'b> {
    pub async fn execute(&self) -> Result<Option<CreateUserResponse>, AzureError> {
        trace!("GetUserBuilder::execute called");

        let req = self
            .user_client
            .prepare_request_with_user_name(hyper::Method::GET);

        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);

        let req = req.body(hyper::Body::empty())?;
        debug!("\nreq == {:?}", req);

        let (status_code, headers, body) =
            extract_status_headers_and_body(self.user_client.hyper_client().request(req)).await?;

        match status_code {
            StatusCode::NOT_FOUND => Ok(None),
            StatusCode::OK => Ok(Some((&headers, &body as &[u8]).try_into()?)),
            _ => Err(UnexpectedHTTPResult::new_multiple(
                vec![StatusCode::OK, StatusCode::NOT_FOUND],
                status_code,
                std::str::from_utf8(&body)?,
            )
            .into()),
        }
    }
}
