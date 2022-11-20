use fe2o3_amqp_management::response::Response;

use crate::amqp::management_constants::properties::MESSAGES;

type PeekSessionMessageResponseBody = super::peek_message::PeekMessageResponseBody;

pub struct PeekSessionMessageResponse {
    pub has_more_messages: bool,
    pub messages: Vec<Vec<u8>>,
}

impl PeekSessionMessageResponse {
    pub fn into_messages(self) -> Vec<Vec<u8>> {
        self.messages
    }
}

impl Response for PeekSessionMessageResponse {
    const STATUS_CODE: u16 = super::HTTP_STATUS_CODE_OK;

    type Body = PeekSessionMessageResponseBody;

    type Error = super::MgmtError;

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

        let messages = super::peek_message::get_messages_from_body(message.body)
            .ok_or_else(|| super::InvalidType {
                expected: MESSAGES.to_string(),
                actual: "None".to_string(),
            })?
            .collect();

        Ok(Self {
            has_more_messages,
            messages,
        })
    }

    fn from_message(
        message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        Self::decode_message(message)
    }
}
