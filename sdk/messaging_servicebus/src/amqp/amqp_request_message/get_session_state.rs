use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::OrderedMap;

use crate::amqp::{
    amqp_response_message::get_session_state::GetSessionStateResponse,
    management_constants::{operations::GET_SESSION_STATE_OPERATION, properties::SESSION_ID},
};

type GetSessionStateRequestBody = OrderedMap<String, String>;

pub(crate) struct GetSessionStateRequest {
    server_timeout: Option<u32>,
    body: GetSessionStateRequestBody,
}

impl GetSessionStateRequest {
    pub fn new(session_id: String) -> Self {
        let mut body = OrderedMap::new();
        body.insert(SESSION_ID.into(), session_id);
        Self {
            server_timeout: None,
            body,
        }
    }

    pub fn set_server_timeout(&mut self, server_timeout: Option<u32>) {
        self.server_timeout = server_timeout;
    }
}

impl Request for GetSessionStateRequest {
    const OPERATION: &'static str = GET_SESSION_STATE_OPERATION;

    type Response = GetSessionStateResponse;

    type Body = GetSessionStateRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a> Request for &'a mut GetSessionStateRequest {
    const OPERATION: &'static str = GET_SESSION_STATE_OPERATION;

    type Response = GetSessionStateResponse;

    type Body = &'a GetSessionStateRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a> Request for &'a GetSessionStateRequest {
    const OPERATION: &'static str = GET_SESSION_STATE_OPERATION;

    type Response = GetSessionStateResponse;

    type Body = &'a GetSessionStateRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
