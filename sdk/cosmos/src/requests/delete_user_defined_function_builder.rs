use crate::prelude::*;
use crate::responses::DeleteUserDefinedFunctionResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteUserDefinedFunctionBuilder<'a, 'b> {
    user_defined_function_client: &'a UserDefinedFunctionClient,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
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
}

impl<'a, 'b> DeleteUserDefinedFunctionBuilder<'a, 'b> {
    fn user_defined_function_client(&self) -> &'a UserDefinedFunctionClient {
        self.user_defined_function_client
    }
}

impl<'a, 'b> DeleteUserDefinedFunctionBuilder<'a, 'b> {
    fn user_agent(&self) -> Option<azure_core::UserAgent<'b>> {
        self.user_agent
    }
}

impl<'a, 'b> DeleteUserDefinedFunctionBuilder<'a, 'b> {
    fn activity_id(&self) -> Option<azure_core::ActivityId<'b>> {
        self.activity_id
    }
}

impl<'a, 'b> DeleteUserDefinedFunctionBuilder<'a, 'b> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b> DeleteUserDefinedFunctionBuilder<'a, 'b> {
    pub fn with_user_agent(self, user_agent: &'b str) -> Self {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }
}

impl<'a, 'b> ActivityIdSupport<'b> for DeleteUserDefinedFunctionBuilder<'a, 'b> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(azure_core::ActivityId::new(activity_id)),
            ..self
        }
    }
}

impl<'a, 'b> ConsistencyLevelSupport<'b> for DeleteUserDefinedFunctionBuilder<'a, 'b> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> DeleteUserDefinedFunctionBuilder<'a, 'b> {
    pub async fn execute(&self) -> Result<DeleteUserDefinedFunctionResponse, CosmosError> {
        trace!("DeleteUserDefinedFunctionBuilder::execute called");

        let request = self
            .user_defined_function_client
            .prepare_request_with_user_defined_function_name(http::Method::DELETE);

        // add trait headers
        let request = crate::headers::add_header(self.user_agent(), request);
        let request = crate::headers::add_header(self.activity_id(), request);
        let request = crate::headers::add_header(self.consistency_level(), request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        Ok(self
            .user_defined_function_client()
            .http_client()
            .execute_request_check_status(request, StatusCode::NO_CONTENT)
            .await?
            .try_into()?)
    }
}
