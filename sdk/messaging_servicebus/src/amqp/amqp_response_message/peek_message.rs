use fe2o3_amqp_management::error::Error as MgmtError;
use fe2o3_amqp_management::{
    response::Response,
    status::StatusCode,
};
use fe2o3_amqp_types::primitives::{Array, OrderedMap};

use crate::amqp::management_constants::properties::{MESSAGE, MESSAGES};

type EncodedMessage = Array<u8>;
type EncodedMessages = Vec<OrderedMap<String, EncodedMessage>>;
type PeekMessageResponseBody = OrderedMap<String, EncodedMessages>;

pub struct PeekMessageResponse {
    pub has_more_messages: bool,
    pub body: PeekMessageResponseBody,
}

impl PeekMessageResponse {
    const STATUS_CODE_OK: u16 = super::HTTP_STATUS_CODE_OK;
    const STATUS_CODE_NO_CONTENT: u16 = super::HTTP_STATUS_CODE_NO_CONTENT;

    pub fn into_messages(mut self) -> Option<impl Iterator<Item = Vec<u8>>> {
        let messages = self.body.remove(MESSAGES)?;

        let messages = messages
            .into_iter()
            .filter_map(|mut map| map.remove(MESSAGE).map(|arr| arr.into_inner()));

        Some(messages)
    }
}

impl Response for PeekMessageResponse {
    const STATUS_CODE: u16 = Self::STATUS_CODE_OK; //

    type Body = PeekMessageResponseBody;

    type Error = MgmtError;

    fn verify_status_code(
        message: &mut fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<StatusCode, Self::Error> {
        super::verify_ok_or_no_content_status_code(message)
    }

    fn decode_message(
        mut message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        let status_code = Self::verify_status_code(&mut message)?;

        let has_more_messages = match status_code.0.get() {
            Self::STATUS_CODE_OK => true,
            Self::STATUS_CODE_NO_CONTENT => false,
            _ => unreachable!(),
        };

        Ok(Self {
            has_more_messages,
            body: message.body,
        })
    }

    fn from_message(
        message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        Self::decode_message(message)
    }
}
