use fe2o3_amqp_management::error::Error as ManagementError;
use fe2o3_amqp_management::response::Response;
use fe2o3_amqp_types::primitives::{OrderedMap, Timestamp};

use crate::amqp::management_constants::properties::EXPIRATION;

type RenewSessionLockResponseBody = OrderedMap<String, Timestamp>;

pub(crate) struct RenewSessionLockResponse {
    pub _has_more_messages: bool,
    pub expiration: Timestamp,
}

impl Response for RenewSessionLockResponse {
    const STATUS_CODE: u16 = super::HTTP_STATUS_CODE_OK; // This will be ignored

    type Body = Option<RenewSessionLockResponseBody>;

    type Error = ManagementError;

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

        let mut body = message.body.ok_or(Self::Error::DecodeError(None))?;
        let expiration =
            body.remove(EXPIRATION)
                .ok_or_else(|| fe2o3_amqp_management::error::InvalidType {
                    expected: EXPIRATION.to_string(),
                    actual: "None".to_string(),
                })?;

        Ok(Self {
            _has_more_messages: has_more_messages,
            expiration,
        })
    }

    fn from_message(
        message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        Self::decode_message(message)
    }
}
