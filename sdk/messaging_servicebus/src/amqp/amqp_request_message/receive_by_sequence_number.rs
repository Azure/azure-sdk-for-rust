use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::{
    definitions::ReceiverSettleMode,
    primitives::{Array, OrderedMap},
};
use serde_amqp::Value;

use crate::amqp::{
    amqp_response_message::receive_by_sequence_number::ReceiveBySequenceNumberResponse,
    management_constants::{
        operations::RECEIVE_BY_SEQUENCE_NUMBER_OPERATION,
        properties::{RECEIVER_SETTLE_MODE, SEQUENCE_NUMBERS, SESSION_ID},
    },
};

type ReceiveBySequenceNumberRequestBody = OrderedMap<String, Value>;

pub struct ReceiveBySequenceNumberRequest {
    server_timeout: Option<u32>,
    body: ReceiveBySequenceNumberRequestBody,
}

impl ReceiveBySequenceNumberRequest {
    pub fn new(
        sequence_numbers: Vec<i64>,
        receiver_settle_mode: ReceiverSettleMode,
        session_id: Option<String>,
    ) -> Self {
        let mut body = OrderedMap::new();
        body.insert(SEQUENCE_NUMBERS.into(), sequence_numbers.into());
        body.insert(
            RECEIVER_SETTLE_MODE.into(),
            Value::UInt(u8::from(receiver_settle_mode) as u32),
        );
        if let Some(session_id) = session_id {
            body.insert(SESSION_ID.into(), session_id.into());
        }
        Self {
            server_timeout: None,
            body,
        }
    }

    pub fn set_server_timeout(&mut self, server_timeout: Option<u32>) {
        self.server_timeout = server_timeout;
    }
}

impl Request for ReceiveBySequenceNumberRequest {
    const OPERATION: &'static str = RECEIVE_BY_SEQUENCE_NUMBER_OPERATION;

    type Response = ReceiveBySequenceNumberResponse;

    type Body = ReceiveBySequenceNumberRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a> Request for &'a mut ReceiveBySequenceNumberRequest {
    const OPERATION: &'static str = RECEIVE_BY_SEQUENCE_NUMBER_OPERATION;

    type Response = ReceiveBySequenceNumberResponse;

    type Body = &'a ReceiveBySequenceNumberRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a> Request for &'a ReceiveBySequenceNumberRequest {
    const OPERATION: &'static str = RECEIVE_BY_SEQUENCE_NUMBER_OPERATION;

    type Response = ReceiveBySequenceNumberResponse;

    type Body = &'a ReceiveBySequenceNumberRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
