use fe2o3_amqp_management::error::Error as ManagementError;
use fe2o3_amqp_types::messaging::Body;
use serde_amqp::Value;

pub(crate) struct CancelScheduledMessageResponse {}

impl fe2o3_amqp_management::response::Response for CancelScheduledMessageResponse {
    const STATUS_CODE: u16 = 200;

    type Body = Body<Value>; // Nothing will be carried in the body, so anything is fine

    type Error = ManagementError;

    fn decode_message(
        _: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}
