use crate::prelude::*;
use crate::responses::DeleteUserResponse;
use azure_core::prelude::*;

use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteUserBuilder<'a, 'b> {
    user_client: &'a UserClient,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> DeleteUserBuilder<'a, 'b> {
    pub(crate) fn new(user_client: &'a UserClient) -> DeleteUserBuilder<'a, 'b> {
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

    pub async fn execute(&self) -> Result<DeleteUserResponse, CosmosError> {
        trace!("DeleteUserBuilder::execute called");

        let req = self
            .user_client
            .prepare_request_with_user_name(http::Method::DELETE);

        let req = azure_core::headers::add_optional_header(&self.user_agent, req);
        let req = azure_core::headers::add_optional_header(&self.activity_id, req);
        let req = azure_core::headers::add_optional_header(&self.consistency_level, req);

        let req = req.body(bytes::Bytes::from_static(EMPTY_BODY))?;
        debug!("\nreq == {:?}", req);

        Ok(self
            .user_client
            .http_client()
            .execute_request_check_status(req, StatusCode::NO_CONTENT)
            .await?
            .try_into()?)
    }
}
