use crate::prelude::*;
use crate::responses::CreateUserDefinedFunctionResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b> {
    user_defined_function_client: &'a UserDefinedFunctionClient,
    is_create: bool,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b> {
    pub(crate) fn new(
        user_defined_function_client: &'a UserDefinedFunctionClient,
        is_create: bool,
    ) -> Self {
        Self {
            user_defined_function_client,
            is_create,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b> CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b> {
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }
}

impl<'a, 'b> CreateOrReplaceUserDefinedFunctionBuilder<'a, 'b> {
    pub async fn execute<B: AsRef<str>>(
        &self,
        body: B,
    ) -> Result<CreateUserDefinedFunctionResponse, CosmosError> {
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
        let req = azure_core::headers::add_optional_header(&self.user_agent, req);
        let req = azure_core::headers::add_optional_header(&self.activity_id, req);
        let req = azure_core::headers::add_optional_header(&self.consistency_level, req);

        let req = req.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Debug, Serialize)]
        struct Request<'a> {
            body: &'a str,
            id: &'a str,
        }
        let request = Request {
            body: body.as_ref(),
            id: self
                .user_defined_function_client
                .user_defined_function_name(),
        };

        let request = azure_core::to_json(&request)?;
        let request = req.body(request)?;

        let result = if self.is_create {
            self.user_defined_function_client
                .http_client()
                .execute_request_check_status(request, StatusCode::CREATED)
                .await?
        } else {
            self.user_defined_function_client
                .http_client()
                .execute_request_check_status(request, StatusCode::OK)
                .await?
        };
        Ok(result.try_into()?)
    }
}
