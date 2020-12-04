use crate::prelude::*;
use crate::responses::CreateUserResponse;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateUserBuilder<'a, 'b> {
    user_client: &'a UserClient,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> CreateUserBuilder<'a, 'b> {
    pub(crate) fn new(user_client: &'a UserClient) -> Self {
        Self {
            user_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }

    pub fn user_client(&self) -> &'a UserClient {
        self.user_client
    }

    pub fn with_user_agent(self, user_agent: &'b str) -> Self {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }

    pub fn with_activity_id(self, activity_id: &'b str) -> Self {
        Self {
            activity_id: Some(azure_core::ActivityId::new(activity_id)),
            ..self
        }
    }

    pub fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }

    pub async fn execute(&self) -> Result<CreateUserResponse, CosmosError> {
        trace!("CreateUserBuilder::execute called");

        let req = self.user_client.prepare_request(http::Method::POST);

        let req = azure_core::headers::add_optional_header(&self.user_agent(), req);
        let req = azure_core::headers::add_optional_header(&self.activity_id(), req);
        let req = azure_core::headers::add_optional_header(&self.consistency_level(), req);

        let req = req.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Serialize, Deserialize)]
        struct RequestBody<'x> {
            id: &'x str,
        }
        let request_body = RequestBody {
            id: self.user_client().user_name(),
        };
        let request_body = serde_json::to_string(&request_body)?;

        let req = req.body(request_body.as_bytes())?;
        debug!("\nreq == {:?}", req);

        Ok(self
            .user_client
            .http_client()
            .execute_request_check_status(req, StatusCode::CREATED)
            .await?
            .try_into()?)
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
