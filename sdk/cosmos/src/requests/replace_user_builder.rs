use crate::prelude::*;
use crate::responses::CreateUserResponse;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use http::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReplaceUserBuilder<'a, 'b, UserNameSet>
where
    UserNameSet: ToAssign,
{
    user_client: &'a UserClient,
    p_user_name: PhantomData<UserNameSet>,
    user_name: Option<&'a str>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> ReplaceUserBuilder<'a, 'b, No> {
    pub(crate) fn new(user_client: &'a UserClient) -> ReplaceUserBuilder<'a, 'b, No> {
        Self {
            user_client,
            p_user_name: PhantomData,
            user_name: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, UserNameSet> ReplaceUserBuilder<'a, 'b, UserNameSet>
where
    UserNameSet: ToAssign,
{
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }
}

impl<'a, 'b> ReplaceUserBuilder<'a, 'b, Yes> {
    pub async fn execute(&self) -> Result<Option<CreateUserResponse>, CosmosError> {
        trace!("ReplaceUserBuilder::execute called");

        let req = self
            .user_client
            .prepare_request_with_user_name(http::Method::PUT);

        let req = azure_core::headers::add_optional_header(&self.user_agent, req);
        let req = azure_core::headers::add_optional_header(&self.activity_id, req);
        let req = azure_core::headers::add_optional_header(&self.consistency_level, req);

        #[derive(Serialize, Deserialize)]
        struct RequestBody<'x> {
            id: &'x str,
        }
        let request_body = RequestBody {
            id: self.user_name.unwrap(),
        };
        let request_body = serde_json::to_string(&request_body)?;

        let req = req.body(request_body.as_bytes())?;
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

impl<'a, 'b> ReplaceUserBuilder<'a, 'b, No> {
    pub fn with_user_name(self, user_name: &'a str) -> ReplaceUserBuilder<'a, 'b, Yes> {
        ReplaceUserBuilder {
            user_name: Some(user_name),
            user_client: self.user_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            p_user_name: PhantomData,
        }
    }
}
