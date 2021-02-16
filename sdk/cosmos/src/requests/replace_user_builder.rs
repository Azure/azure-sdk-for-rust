use crate::prelude::*;
use crate::responses::CreateUserResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ReplaceUserBuilder<'a, 'b> {
    user_client: &'a UserClient,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> ReplaceUserBuilder<'a, 'b> {
    pub(crate) fn new(user_client: &'a UserClient) -> ReplaceUserBuilder<'a, 'b> {
        Self {
            user_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b> ReplaceUserBuilder<'a, 'b> {
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }
}

impl<'a, 'b> ReplaceUserBuilder<'a, 'b> {
    pub async fn execute<U: AsRef<str>>(
        &self,
        user_name: U,
    ) -> Result<Option<CreateUserResponse>, CosmosError> {
        trace!("ReplaceUserBuilder::execute called");

        let req = self
            .user_client
            .prepare_request_with_user_name(http::Method::PUT);

        let req = azure_core::headers::add_optional_header(&self.user_agent, req);
        let req = azure_core::headers::add_optional_header(&self.activity_id, req);
        let req = azure_core::headers::add_optional_header(&self.consistency_level, req);

        #[derive(Serialize)]
        struct RequestBody<'x> {
            id: &'x str,
        }
        let request_body = RequestBody {
            id: user_name.as_ref(),
        };
        let request_body = azure_core::to_json(&request_body)?;

        let req = req.body(request_body)?;
        debug!("\nreq == {:?}", req);

        let response = self
            .user_client
            .http_client()
            .execute_request_check_statuses(req, &[StatusCode::OK, StatusCode::NOT_FOUND])
            .await?;

        match response.status() {
            StatusCode::NOT_FOUND => Ok(None),
            StatusCode::OK => Ok(Some(response.try_into()?)),
            _ => unreachable!(),
        }
    }
}
