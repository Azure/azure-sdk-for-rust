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

#[derive(Debug, Clone)]
pub struct ReceiveBySequenceNumberRequest<'a> {
    server_timeout: Option<u32>,
    associated_link_name: Option<&'a str>,
    body: ReceiveBySequenceNumberRequestBody,
}

impl<'a> ReceiveBySequenceNumberRequest<'a> {
    pub fn new(
        sequence_numbers: Array<i64>,
        receiver_settle_mode: ReceiverSettleMode,
        associated_link_name: Option<&'a str>,
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
            associated_link_name,
            body,
        }
    }

    pub fn set_server_timeout(&mut self, server_timeout: Option<u32>) {
        self.server_timeout = server_timeout;
    }
}

impl<'a> Request for ReceiveBySequenceNumberRequest<'a> {
    const OPERATION: &'static str = RECEIVE_BY_SEQUENCE_NUMBER_OPERATION;

    type Response = ReceiveBySequenceNumberResponse;

    type Body = ReceiveBySequenceNumberRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a, 'b> Request for &'a mut ReceiveBySequenceNumberRequest<'b> {
    const OPERATION: &'static str = RECEIVE_BY_SEQUENCE_NUMBER_OPERATION;

    type Response = ReceiveBySequenceNumberResponse;

    type Body = &'a ReceiveBySequenceNumberRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a, 'b> Request for &'a ReceiveBySequenceNumberRequest<'b> {
    const OPERATION: &'static str = RECEIVE_BY_SEQUENCE_NUMBER_OPERATION;

    type Response = ReceiveBySequenceNumberResponse;

    type Body = &'a ReceiveBySequenceNumberRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

#[cfg(test)]
mod tests {
    use fe2o3_amqp_management::request::Request;
    use fe2o3_amqp_types::messaging::{
        message::__private::{Deserializable, Serializable},
        AmqpValue, Body, Message,
    };
    use serde_amqp::{from_slice, to_vec, Value};

    #[test]
    fn encode_reqeust() {
        let request = super::ReceiveBySequenceNumberRequest::new(
            vec![389].into(),
            fe2o3_amqp_types::definitions::ReceiverSettleMode::First,
            Some("link-name"),
            Some("session-id".into()),
        );
        println!("request {:?}", request);

        let message = request.into_message();
        println!("message {:?}", message);

        let encoded = to_vec(&Serializable(message.map_body(AmqpValue))).unwrap();
        println!("encoded {:#x?}", encoded);

        let decoded: Deserializable<Message<Body<Value>>> = from_slice(&encoded).unwrap();
        println!("decoded {:?}", decoded);
    }

    #[test]
    fn decode_buf_from_azure_amqp() {
        const BUF: &[u8] = &[
            0x0, 0x53, 0x74, 0xc1, 0xcd, 0x8, 0xa1, 0x9, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69,
            0x6f, 0x6e, 0xa1, 0x28, 0x63, 0x6f, 0x6d, 0x2e, 0x6d, 0x69, 0x63, 0x72, 0x6f, 0x73,
            0x6f, 0x66, 0x74, 0x3a, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x2d, 0x62, 0x79,
            0x2d, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x2d, 0x6e, 0x75, 0x6d, 0x62,
            0x65, 0x72, 0xa1, 0x1c, 0x63, 0x6f, 0x6d, 0x2e, 0x6d, 0x69, 0x63, 0x72, 0x6f, 0x73,
            0x6f, 0x66, 0x74, 0x3a, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2d, 0x74, 0x69, 0x6d,
            0x65, 0x6f, 0x75, 0x74, 0x70, 0x0, 0x0, 0xea, 0x60, 0xa1, 0x19, 0x63, 0x6f, 0x6d, 0x2e,
            0x6d, 0x69, 0x63, 0x72, 0x6f, 0x73, 0x6f, 0x66, 0x74, 0x3a, 0x74, 0x72, 0x61, 0x63,
            0x6b, 0x69, 0x6e, 0x67, 0x2d, 0x69, 0x64, 0xa1, 0x24, 0x64, 0x32, 0x39, 0x31, 0x33,
            0x35, 0x33, 0x61, 0x2d, 0x61, 0x66, 0x64, 0x36, 0x2d, 0x34, 0x66, 0x35, 0x62, 0x2d,
            0x61, 0x35, 0x38, 0x39, 0x2d, 0x65, 0x30, 0x62, 0x66, 0x30, 0x38, 0x33, 0x38, 0x30,
            0x64, 0x34, 0x62, 0xa1, 0x14, 0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65,
            0x64, 0x2d, 0x6c, 0x69, 0x6e, 0x6b, 0x2d, 0x6e, 0x61, 0x6d, 0x65, 0xa1, 0x1b, 0x72,
            0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x72, 0x2d, 0x36, 0x33, 0x38, 0x30, 0x34, 0x38,
            0x30, 0x38, 0x39, 0x34, 0x30, 0x38, 0x38, 0x38, 0x30, 0x30, 0x35, 0x36, 0x0, 0x53,
            0x77, 0xc1, 0x4b, 0x6, 0xa1, 0x10, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65,
            0x2d, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x73, 0xe0, 0xa, 0x1, 0x81, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x1, 0x87, 0xa1, 0x14, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x72,
            0x2d, 0x73, 0x65, 0x74, 0x74, 0x6c, 0x65, 0x2d, 0x6d, 0x6f, 0x64, 0x65, 0x43, 0xa1,
            0xa, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2d, 0x69, 0x64, 0xa1, 0x7, 0x73, 0x65,
            0x73, 0x73, 0x69, 0x6f, 0x6e,
        ];
        let message: Deserializable<Message<Body<Value>>> = from_slice(BUF).unwrap();
        println!("decoded {:?}", message.0);
    }
}
