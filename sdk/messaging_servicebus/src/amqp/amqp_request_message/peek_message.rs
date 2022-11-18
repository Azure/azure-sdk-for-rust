use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::OrderedMap;
use serde_amqp::Value;

use crate::amqp::{management_constants::{properties::{FROM_SEQUENCE_NUMBER, MESSAGE_COUNT}, operations::PEEK_MESSAGE_OPERATION}, amqp_response_message::peek_message::PeekMessageResponse};


pub struct PeekMessageRequest {
    server_timeout: Option<u32>,
    body: OrderedMap<String, Value>,
}

impl PeekMessageRequest {
    pub fn new(from_sequence_number: i64, message_count: i32) -> Self {
        let mut body = OrderedMap::with_capacity(2);
        body.insert(FROM_SEQUENCE_NUMBER.into(), Value::Long(from_sequence_number));
        body.insert(MESSAGE_COUNT.into(), Value::Int(message_count));
        Self {
            server_timeout: None,
            body,
        }
    }

    pub fn set_server_timeout(&mut self, server_timeout: Option<u32>) {
        self.server_timeout = server_timeout;
    }
}

impl Request for PeekMessageRequest {
    const OPERATION: &'static str = PEEK_MESSAGE_OPERATION;

    type Response = PeekMessageResponse;

    type Body = OrderedMap<String, Value>;

    fn encode_application_properties(&mut self) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a> Request for &'a PeekMessageRequest {
    const OPERATION: &'static str = PEEK_MESSAGE_OPERATION;

    type Response = PeekMessageResponse;

    type Body = &'a OrderedMap<String, Value>;

    fn encode_application_properties(&mut self) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}


