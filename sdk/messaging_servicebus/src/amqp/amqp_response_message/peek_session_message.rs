use fe2o3_amqp_management::response::Response;

type PeekSessionMessageResponseBody = super::peek_message::PeekMessageResponseBody;

pub struct PeekSessionMessageResponse {
    pub has_more_messages: bool,
    pub body: PeekSessionMessageResponseBody,
}

impl PeekSessionMessageResponse {
    pub fn into_messages(self) -> Option<impl Iterator<Item = Vec<u8>>> {
        super::peek_message::get_messages_from_body(self.body)
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
