use crate::prelude::*;
use crate::responses::CreateUserResponse;
use azure_core::errors::{extract_status_headers_and_body, AzureError, UnexpectedHTTPResult};
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReplaceUserBuilder<'a, 'b, UserNameSet>
where
    UserNameSet: ToAssign,
{
    user_client: &'a UserClient,
    p_user_name: PhantomData<UserNameSet>,
    user_name: Option<&'a dyn UserName>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
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

impl<'a, 'b, UserNameSet> UserClientRequired<'a> for ReplaceUserBuilder<'a, 'b, UserNameSet>
where
    UserNameSet: ToAssign,
{
    fn user_client(&self) -> &'a UserClient {
        self.user_client
    }
}

impl<'a, 'b> UserNameRequired<'a> for ReplaceUserBuilder<'a, 'b, Yes> {
    fn user_name(&self) -> &'a dyn UserName {
        self.user_name.unwrap()
    }
}

impl<'a, 'b, UserNameSet> UserAgentOption<'b> for ReplaceUserBuilder<'a, 'b, UserNameSet>
where
    UserNameSet: ToAssign,
{
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, UserNameSet> ActivityIdOption<'b> for ReplaceUserBuilder<'a, 'b, UserNameSet>
where
    UserNameSet: ToAssign,
{
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, UserNameSet> ConsistencyLevelOption<'b> for ReplaceUserBuilder<'a, 'b, UserNameSet>
where
    UserNameSet: ToAssign,
{
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b> UserNameSupport<'a> for ReplaceUserBuilder<'a, 'b, No> {
    type O = ReplaceUserBuilder<'a, 'b, Yes>;

    fn with_user_name(self, user_name: &'a dyn UserName) -> Self::O {
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
            user_agent: Some(user_agent),
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
            activity_id: Some(activity_id),
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
    pub async fn execute(&self) -> Result<Option<CreateUserResponse>, AzureError> {
        trace!("ReplaceUserBuilder::execute called");

        let req = self
            .user_client
            .prepare_request_with_user_name(hyper::Method::PUT);

        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);

        #[derive(Serialize, Deserialize)]
        struct RequestBody<'x> {
            id: &'x str,
        }
        let request_body = RequestBody {
            id: self.user_name().id(),
        };
        let request_body = serde_json::to_string(&request_body)?;

        let req = req.body(hyper::Body::from(request_body))?;
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
