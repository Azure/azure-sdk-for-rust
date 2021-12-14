use crate::prelude::*;
use crate::responses::DeleteUserDefinedFunctionResponse;
use azure_core::prelude::*;

use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteUserDefinedFunctionBuilder<'a, 'b> {
    user_defined_function_client: &'a UserDefinedFunctionClient,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> DeleteUserDefinedFunctionBuilder<'a, 'b> {
    pub(crate) fn new(user_defined_function_client: &'a UserDefinedFunctionClient) -> Self {
        Self {
            user_defined_function_client,
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

    pub async fn execute(&self) -> crate::Result<DeleteUserDefinedFunctionResponse> {
        trace!("DeleteUserDefinedFunctionBuilder::execute called");

        let request = self
            .user_defined_function_client
            .prepare_request_with_user_defined_function_name(http::Method::DELETE);

        // add trait headers
        let request = azure_core::headers::add_optional_header(&self.user_agent, request);
        let request = azure_core::headers::add_optional_header(&self.activity_id, request);
        let request = azure_core::headers::add_optional_header(&self.consistency_level, request);

        let request = request.body(azure_core::EMPTY_BODY)?;

        Ok(self
            .user_defined_function_client
            .http_client()
            .execute_request_check_status(request, StatusCode::NO_CONTENT)
            .await?
            .try_into()?)
    }
}
