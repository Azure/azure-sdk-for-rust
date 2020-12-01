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
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> ReplaceUserBuilder<'a, 'b, No> {
    pub(crate) fn new(user_client: &'a UserClient) -> ReplaceUserBuilder<'a, 'b, No> {
        Self {
            user_client,
            p_user_name: PhantomData {},
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
    pub fn user_client(&self) -> &'a UserClient {
        self.user_client
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
}

impl<'a, 'b> UserNameRequired<'a> for ReplaceUserBuilder<'a, 'b, Yes> {
    fn user_name(&self) -> &'a str {
        self.user_name.unwrap()
    }
}

impl<'a, 'b> UserNameSupport<'a> for ReplaceUserBuilder<'a, 'b, No> {
    type O = ReplaceUserBuilder<'a, 'b, Yes>;

    fn with_user_name(self, user_name: &'a str) -> Self::O {
        ReplaceUserBuilder {
            user_client: self.user_client,
            p_user_name: PhantomData {},
            user_name: Some(user_name),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, UserNameSet> UserAgentSupport<'b> for ReplaceUserBuilder<'a, 'b, UserNameSet>
where
    UserNameSet: ToAssign,
{
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }
}

impl<'a, 'b, UserNameSet> ActivityIdSupport<'b> for ReplaceUserBuilder<'a, 'b, UserNameSet>
where
    UserNameSet: ToAssign,
{
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(azure_core::ActivityId::new(activity_id)),
            ..self
        }
    }
}

impl<'a, 'b, UserNameSet> ConsistencyLevelSupport<'b> for ReplaceUserBuilder<'a, 'b, UserNameSet>
where
    UserNameSet: ToAssign,
{
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> ReplaceUserBuilder<'a, 'b, Yes> {
    pub async fn execute(&self) -> Result<Option<CreateUserResponse>, CosmosError> {
        trace!("ReplaceUserBuilder::execute called");

        let req = self
            .user_client
            .prepare_request_with_user_name(http::Method::PUT);

        let req = crate::headers::add_header(self.user_agent(), req);
        let req = crate::headers::add_header(self.activity_id(), req);
        let req = crate::headers::add_header(self.consistency_level(), req);

        #[derive(Serialize, Deserialize)]
        struct RequestBody<'x> {
            id: &'x str,
        }
        let request_body = RequestBody {
            id: self.user_name(),
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
