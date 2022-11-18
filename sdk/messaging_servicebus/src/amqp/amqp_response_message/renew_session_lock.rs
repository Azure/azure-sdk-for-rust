use fe2o3_amqp_management::response::Response;
use fe2o3_amqp_types::primitives::{OrderedMap, Timestamp};
use fe2o3_amqp_management::error::Error as MgmtError;

use crate::amqp::management_constants::properties::EXPIRATION;

type RenewSessionLockResponseBody = OrderedMap<String, Timestamp>;

pub(crate) struct RenewSessionLockResponse {
    pub has_more_messages: bool,
    pub body: RenewSessionLockResponseBody
}

impl RenewSessionLockResponse {
    const STATUS_CODE_OK: u16 = super::HTTP_STATUS_CODE_OK;
    const STATUS_CODE_NO_CONTENT: u16 = super::HTTP_STATUS_CODE_NO_CONTENT;

    pub fn into_expiration(mut self) -> Option<Timestamp> {
        self.body.remove(EXPIRATION)
    }
}

impl Response for RenewSessionLockResponse {
    const STATUS_CODE: u16 = Self::STATUS_CODE_OK; // This will be ignored

    type Body = RenewSessionLockResponseBody;

    type Error = MgmtError;

    fn verify_status_code(message: &mut fe2o3_amqp_types::messaging::Message<Self::Body>) -> Result<fe2o3_amqp_management::status::StatusCode, Self::Error> {
        super::verify_ok_or_no_content_status_code(message)
    }

    fn decode_message(mut message: fe2o3_amqp_types::messaging::Message<Self::Body>) -> Result<Self, Self::Error> {
        let status_code = Self::verify_status_code(&mut message)?;

        let has_more_messages = match status_code.0.get() {
            Self::STATUS_CODE_OK => true,
            Self::STATUS_CODE_NO_CONTENT => false,
            _ => unreachable!()
        };

        Ok(Self {
            has_more_messages,
            body: message.body,
        })
    }

    fn from_message(message: fe2o3_amqp_types::messaging::Message<Self::Body>) -> Result<Self, Self::Error> {
        Self::decode_message(message)
    }
}
