use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::{Binary, OrderedMap};
use serde_amqp::Value;

use crate::amqp::{
    amqp_response_message::set_session_state::SetSessionStateResponse,
    management_constants::{
        operations::SET_SESSION_STATE_OPERATION,
        properties::{SESSION_ID, SESSION_STATE},
    },
};

type SetSessionStateRequestBody = OrderedMap<String, Value>;

pub(crate) struct SetSessionStateRequest<'a> {
    server_timeout: Option<u32>,
    associated_link_name: Option<&'a str>,
    body: SetSessionStateRequestBody,
}

impl<'a> SetSessionStateRequest<'a> {
    pub fn new(
        session_id: String,
        session_state: Binary,
        associated_link_name: Option<&'a str>,
    ) -> Self {
        let mut body = OrderedMap::new();
        body.insert(SESSION_ID.into(), session_id.into());
        body.insert(SESSION_STATE.into(), session_state.into());
        Self {
            server_timeout: None,
            associated_link_name,
            body,
        }
    }

    pub fn set_server_timeout(&mut self, server_timeout: Option<u32>) {
        self.server_timeout = server_timeout;
    }
}

impl<'a> Request for SetSessionStateRequest<'a> {
    const OPERATION: &'static str = SET_SESSION_STATE_OPERATION;

    type Response = SetSessionStateResponse;

    type Body = SetSessionStateRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a, 'b> Request for &'a mut SetSessionStateRequest<'b> {
    const OPERATION: &'static str = SET_SESSION_STATE_OPERATION;

    type Response = SetSessionStateResponse;

    type Body = &'a SetSessionStateRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a, 'b> Request for &'a SetSessionStateRequest<'b> {
    const OPERATION: &'static str = SET_SESSION_STATE_OPERATION;

    type Response = SetSessionStateResponse;

    type Body = &'a SetSessionStateRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
