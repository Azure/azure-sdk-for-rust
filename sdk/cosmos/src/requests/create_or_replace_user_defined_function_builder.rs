use crate::prelude::*;
use crate::responses::CreateUserDefinedFunctionResponse;
use azure_core::UserAgent;
use azure_core::{No, ToAssign, Yes};
use http::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, BodySet>
where
    BodySet: ToAssign,
{
    user_defined_function_client: &'a UserDefinedFunctionClient,
    is_create: bool,
    body: Option<&'b str>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
    p_body: PhantomData<BodySet>,
}

impl<'a, 'b> CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, No> {
    pub(crate) fn new(
        user_defined_function_client: &'a UserDefinedFunctionClient,
        is_create: bool,
    ) -> Self {
        Self {
            user_defined_function_client,
            is_create,
            body: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            p_body: PhantomData {},
        }
    }
}

impl<'a, 'b, BodySet> CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, BodySet>
where
    BodySet: ToAssign,
{
    pub fn with_user_agent(self, user_agent: &'b str) -> Self {
        Self {
            user_agent: Some(UserAgent::new(user_agent)),
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

    fn user_defined_function_client(&self) -> &'a UserDefinedFunctionClient {
        self.user_defined_function_client
    }

    fn user_agent(&self) -> Option<UserAgent<'b>> {
        self.user_agent
    }

    fn activity_id(&self) -> Option<azure_core::ActivityId<'b>> {
        self.activity_id
    }

    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b> CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, Yes> {
    fn body(&self) -> &'b str {
        self.body.unwrap()
    }
}

impl<'a, 'b> CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, No> {
    pub fn with_body(
        self,
        body: &'b str,
    ) -> CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, Yes> {
        CreateOrReplaceUserDefinedFunctionBuilder {
            body: Some(body),
            user_defined_function_client: self.user_defined_function_client,
            is_create: self.is_create,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            p_body: PhantomData {},
        }
    }
}

impl<'a, 'b> CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b, Yes> {
    pub async fn execute(&self) -> Result<CreateUserDefinedFunctionResponse, CosmosError> {
        trace!("CreateOrReplaceUserDefinedFunctionBuilder::execute called");

        // Create is POST with no name in the URL. Expected return is CREATED.
        // See https://docs.microsoft.com/en-us/rest/api/cosmos-db/create-a-user-defined-function
        // Replace is PUT with name appended to the URL. Expected return is OK.
        // See: https://docs.microsoft.com/en-us/rest/api/cosmos-db/replace-a-user-defined-function
        let req = match self.is_create {
            true => self
                .user_defined_function_client
                .prepare_request(http::Method::POST),
            false => self
                .user_defined_function_client
                .prepare_request_with_user_defined_function_name(http::Method::PUT),
        };

        // add trait headers
        let req = azure_core::headers::add_optional_header(&self.user_agent(), req);
        let req = azure_core::headers::add_optional_header(&self.activity_id(), req);
        let req = azure_core::headers::add_optional_header(&self.consistency_level(), req);

        let req = req.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Debug, Serialize)]
        struct Request<'a> {
            body: &'a str,
            id: &'a str,
        }
        let request = Request {
            body: self.body(),
            id: self
                .user_defined_function_client
                .user_defined_function_name(),
        };

        let request = serde_json::to_string(&request)?;
        let request = req.body(request.as_bytes())?;

        Ok(if self.is_create {
            self.user_defined_function_client()
                .http_client()
                .execute_request_check_status(request, StatusCode::CREATED)
                .await?
                .try_into()?
        } else {
            self.user_defined_function_client()
                .http_client()
                .execute_request_check_status(request, StatusCode::OK)
                .await?
                .try_into()?
        })
    }
}
