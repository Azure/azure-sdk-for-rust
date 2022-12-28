use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::OrderedMap;
use serde_amqp::Value;

use crate::amqp::{
    amqp_response_message::peek_message::PeekMessageResponse,
    management_constants::{
        operations::PEEK_MESSAGE_OPERATION,
        properties::{FROM_SEQUENCE_NUMBER, MESSAGE_COUNT},
    },
};

type PeekMessageRequestBody = OrderedMap<String, Value>;

pub(crate) struct PeekMessageRequest {
    server_timeout: Option<u32>,
    associated_link_name: Option<String>,
    body: OrderedMap<String, Value>,
}

impl PeekMessageRequest {
    pub fn new(
        from_sequence_number: i64,
        message_count: i32,
        associated_link_name: Option<String>,
    ) -> Self {
        let mut body = OrderedMap::with_capacity(2);
        body.insert(
            FROM_SEQUENCE_NUMBER.into(),
            Value::Long(from_sequence_number),
        );
        body.insert(MESSAGE_COUNT.into(), Value::Int(message_count));
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

impl Request for PeekMessageRequest {
    const OPERATION: &'static str = PEEK_MESSAGE_OPERATION;

    type Response = PeekMessageResponse;

    type Body = PeekMessageRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        // TODO: reduce clones?
        super::encode_application_properties(self.server_timeout, self.associated_link_name.clone())
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a> Request for &'a mut PeekMessageRequest {
    const OPERATION: &'static str = PEEK_MESSAGE_OPERATION;

    type Response = PeekMessageResponse;

    type Body = &'a PeekMessageRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        // TODO: reduce clones?
        super::encode_application_properties(self.server_timeout, self.associated_link_name.clone())
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a> Request for &'a PeekMessageRequest {
    const OPERATION: &'static str = PEEK_MESSAGE_OPERATION;

    type Response = PeekMessageResponse;

    type Body = &'a PeekMessageRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        // TODO: reduce clones?
        super::encode_application_properties(self.server_timeout, self.associated_link_name.clone())
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
