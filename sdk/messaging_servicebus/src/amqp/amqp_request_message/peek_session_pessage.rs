use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::{messaging::ApplicationProperties, primitives::OrderedMap};
use serde_amqp::SerializeComposite;

use crate::amqp::{
    amqp_response_message::peek_session_message::PeekSessionMessageResponse,
    management_constants::{
        operations::PEEK_MESSAGE_OPERATION,
        properties::{FROM_SEQUENCE_NUMBER, MESSAGE_COUNT, SESSION_ID},
    },
};

type PeekSessionMessageRequestBody = OrderedMap<String, serde_amqp::Value>;

pub struct PeekSessionMessageRequest {
    server_timeout: Option<u32>,
    body: PeekSessionMessageRequestBody,
}

impl PeekSessionMessageRequest {
    pub fn new(from_sequence_number: i64, message_count: i32, session_id: String) -> Self {
        let mut body = OrderedMap::with_capacity(3);
        body.insert(FROM_SEQUENCE_NUMBER.into(), from_sequence_number.into());
        body.insert(MESSAGE_COUNT.into(), message_count.into());
        body.insert(SESSION_ID.into(), session_id.into());
        Self {
            server_timeout: None,
            body,
        }
    }

    pub fn set_server_timeout(&mut self, server_timeout: Option<u32>) {
        self.server_timeout = server_timeout;
    }
}

impl Request for PeekSessionMessageRequest {
    const OPERATION: &'static str = PEEK_MESSAGE_OPERATION;

    type Response = PeekSessionMessageResponse;

    type Body = PeekSessionMessageRequestBody;

    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a> Request for &'a mut PeekSessionMessageRequest {
    const OPERATION: &'static str = PEEK_MESSAGE_OPERATION;

    type Response = PeekSessionMessageResponse;

    type Body = &'a PeekSessionMessageRequestBody;

    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a> Request for &'a PeekSessionMessageRequest {
    const OPERATION: &'static str = PEEK_MESSAGE_OPERATION;

    type Response = PeekSessionMessageResponse;

    type Body = &'a PeekSessionMessageRequestBody;

    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
