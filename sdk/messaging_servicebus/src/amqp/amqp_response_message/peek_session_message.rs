use fe2o3_amqp_management::response::Response;
use fe2o3_amqp_types::messaging::{message::__private::Deserializable, Body, Message};
use serde_amqp::Value;

use crate::{
    amqp::management_constants::properties::MESSAGES,
    primitives::service_bus_peeked_message::ServiceBusPeekedMessage,
};

type PeekSessionMessageResponseBody = super::peek_message::PeekMessageResponseBody;

pub(crate) struct PeekSessionMessageResponse {
    pub _has_more_messages: bool,
    pub messages: Vec<Vec<u8>>,
}

impl PeekSessionMessageResponse {
    pub fn into_peeked_messages(self) -> Result<Vec<ServiceBusPeekedMessage>, serde_amqp::Error> {
        self.messages
            .into_iter()
            .map(|buf| {
                let raw_amqp_message: Deserializable<Message<Body<Value>>> =
                    serde_amqp::from_slice(&buf)?;
                let message = ServiceBusPeekedMessage {
                    raw_amqp_message: raw_amqp_message.0,
                };
                Ok(message)
            })
            .collect()
    }
}

impl Response for PeekSessionMessageResponse {
    const STATUS_CODE: u16 = super::HTTP_STATUS_CODE_OK;

    type Body = Option<PeekSessionMessageResponseBody>;

    type Error = super::ManagementError;

    fn verify_status_code(
        message: &mut fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<fe2o3_amqp_management::status::StatusCode, Self::Error> {
        super::verify_ok_or_no_content_status_code(message)
    }

    fn decode_message(
        mut message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        let status_code = Self::verify_status_code(&mut message)?;

        let has_more_messages = match status_code.0.get() {
            super::HTTP_STATUS_CODE_OK => true,
            super::HTTP_STATUS_CODE_NO_CONTENT => false,
            _ => unreachable!(),
        };

        let body = message.body.ok_or(Self::Error::DecodeError(None))?;
        let messages = super::peek_message::get_messages_from_body(body)
            .ok_or_else(|| super::InvalidType {
                expected: MESSAGES.to_string(),
                actual: "None".to_string(),
            })?
            .collect();

        Ok(Self {
            _has_more_messages: has_more_messages,
            messages,
        })
    }

    fn from_message(
        message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        Self::decode_message(message)
    }
}
