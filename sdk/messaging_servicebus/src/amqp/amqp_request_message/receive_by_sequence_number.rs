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
        properties::{RECEIVER_SETTLE_MODE, SEQUENCE_NUMBERS},
    },
};

type ReceiveBySequenceNumberRequestBody = OrderedMap<String, Value>;

pub struct ReceiveBySequenceNumberRequest {
    server_timeout: Option<u32>,
    body: ReceiveBySequenceNumberRequestBody,
}

impl ReceiveBySequenceNumberRequest {
    pub fn new(
        sequence_numbers: impl Into<Array<i64>>,
        receiver_settle_mode: ReceiverSettleMode,
    ) -> Self {
        let sequence_numbers = sequence_numbers.into();
        let mut body = OrderedMap::new();
        body.insert(SEQUENCE_NUMBERS.into(), sequence_numbers.into());
        body.insert(
            RECEIVER_SETTLE_MODE.into(),
            Value::UByte(receiver_settle_mode.into()),
        );
        Self {
            server_timeout: None,
            body,
        }
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
