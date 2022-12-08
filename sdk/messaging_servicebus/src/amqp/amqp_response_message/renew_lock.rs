use fe2o3_amqp_management::error::Error as ManagementError;
use fe2o3_amqp_management::response::Response;
use fe2o3_amqp_types::primitives::{Array, OrderedMap, Timestamp};

use crate::amqp::management_constants::properties::EXPIRATIONS;

type RenewLockResponseBody = OrderedMap<String, Array<Timestamp>>;

pub(crate) struct RenewLockResponse {
    pub expirations: Array<Timestamp>,
}

impl Response for RenewLockResponse {
    const STATUS_CODE: u16 = 200;

    type Body = Option<RenewLockResponseBody>;

    type Error = ManagementError;

    fn decode_message(
        message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        let expirations = message
            .body
            .and_then(|mut map| map.remove(EXPIRATIONS))
            .ok_or(ManagementError::DecodeError(None))?;
        Ok(Self { expirations })
    }
}
