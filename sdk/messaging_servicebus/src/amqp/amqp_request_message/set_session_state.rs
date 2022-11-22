use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::{Array, Binary, OrderedMap};
use serde_amqp::Value;

use crate::amqp::{
    amqp_response_message::set_session_state::SetSessionStateResponse,
    management_constants::{
        operations::SET_SESSION_STATE_OPERATION,
        properties::{SESSION_ID, SESSION_STATE},
    },
};

type SetSessionStateRequestBody = OrderedMap<String, Value>;

pub(crate) struct SetSessionStateRequest {
    server_timeout: Option<u32>,
    body: SetSessionStateRequestBody,
}

impl SetSessionStateRequest {
    pub fn new(session_id: String, session_state: Binary) -> Self {
        let mut body = OrderedMap::new();
        body.insert(SESSION_ID.into(), session_id.into());
        body.insert(SESSION_STATE.into(), session_state.into());
        Self {
            server_timeout: None,
            body,
        }
    }

    pub fn set_server_timeout(&mut self, server_timeout: Option<u32>) {
        self.server_timeout = server_timeout;
    }
}

impl Request for SetSessionStateRequest {
    const OPERATION: &'static str = SET_SESSION_STATE_OPERATION;

    type Response = SetSessionStateResponse;

    type Body = SetSessionStateRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a> Request for &'a mut SetSessionStateRequest {
    const OPERATION: &'static str = SET_SESSION_STATE_OPERATION;

    type Response = SetSessionStateResponse;

    type Body = &'a SetSessionStateRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a> Request for &'a SetSessionStateRequest {
    const OPERATION: &'static str = SET_SESSION_STATE_OPERATION;

    type Response = SetSessionStateResponse;

    type Body = &'a SetSessionStateRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
