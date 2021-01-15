use crate::prelude::*;
use crate::responses::CreateUserResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetUserBuilder<'a, 'b> {
    user_client: &'a UserClient,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
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

    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }

    pub async fn execute(&self) -> Result<Option<CreateUserResponse>, CosmosError> {
        trace!("GetUserBuilder::execute called");

        let req = self
            .user_client
            .prepare_request_with_user_name(http::Method::GET);

        let req = azure_core::headers::add_optional_header(&self.user_agent, req);
        let req = azure_core::headers::add_optional_header(&self.activity_id, req);
        let req = azure_core::headers::add_optional_header(&self.consistency_level, req);

        let req = req.body(bytes::Bytes::from_static(EMPTY_BODY))?;
        debug!("\nreq == {:?}", req);

        let response = self
            .user_client
            .http_client()
            .execute_request_check_statuses(req, &[StatusCode::NOT_FOUND, StatusCode::OK])
            .await?;

        match response.status() {
            StatusCode::NOT_FOUND => Ok(None),
            StatusCode::OK => Ok(Some(response.try_into()?)),
            _ => unreachable!(),
        }
    }
}
