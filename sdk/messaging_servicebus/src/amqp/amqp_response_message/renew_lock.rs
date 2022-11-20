use fe2o3_amqp_management::error::Error as ManagementError;
use fe2o3_amqp_management::response::Response;
use fe2o3_amqp_types::primitives::{Array, Timestamp};

pub struct RenewLockResponse {
    pub expirations: Array<Timestamp>,
}

impl Response for RenewLockResponse {
    const STATUS_CODE: u16 = 200;

    type Body = Array<Timestamp>;

    type Error = ManagementError;

    fn decode_message(
        message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            expirations: message.body,
        })
    }
}
