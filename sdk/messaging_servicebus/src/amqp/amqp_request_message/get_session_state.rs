use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::OrderedMap;

use crate::amqp::{
    amqp_response_message::get_session_state::GetSessionStateResponse,
    management_constants::{operations::GET_SESSION_STATE_OPERATION, properties::SESSION_ID},
};

type GetSessionStateRequestBody = OrderedMap<String, String>;

pub(crate) struct GetSessionStateRequest<'a> {
    server_timeout: Option<u32>,
    associated_link_name: Option<&'a str>,
    body: GetSessionStateRequestBody,
}

impl<'a> GetSessionStateRequest<'a> {
    pub fn new(session_id: &str, associated_link_name: Option<&'a str>) -> Self {
        let mut body = OrderedMap::new();
        body.insert(SESSION_ID.into(), session_id.into());
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

impl<'a> Request for GetSessionStateRequest<'a> {
    const OPERATION: &'static str = GET_SESSION_STATE_OPERATION;

    type Response = GetSessionStateResponse;

    type Body = GetSessionStateRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a, 'b> Request for &'a mut GetSessionStateRequest<'b> {
    const OPERATION: &'static str = GET_SESSION_STATE_OPERATION;

    type Response = GetSessionStateResponse;

    type Body = &'a GetSessionStateRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a, 'b> Request for &'a GetSessionStateRequest<'b> {
    const OPERATION: &'static str = GET_SESSION_STATE_OPERATION;

    type Response = GetSessionStateResponse;

    type Body = &'a GetSessionStateRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
