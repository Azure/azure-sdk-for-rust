use fe2o3_amqp_management::{Request, Response, error::Error as ManagementError};
use fe2o3_amqp_types::messaging::Body;
use serde_amqp::Value;

use super::READ_OPERATION_VALUE;

pub(crate) struct EventHubPropertiesRequest {

}

impl Request for EventHubPropertiesRequest {
    const OPERATION: &'static str = READ_OPERATION_VALUE;

    type Response = EventHubPropertiesResponse;

    type Body = ();

    fn encode_body(self) -> Self::Body {
        todo!()
    }
}

pub(crate) struct EventHubPropertiesResponse {

}

impl Response for EventHubPropertiesResponse {
    type Body = Body<Value>;

    const STATUS_CODE: u16 = 200;

    type Error = ManagementError;

    fn decode_message(message: fe2o3_amqp_types::messaging::Message<Self::Body>) -> Result<Self, Self::Error> {
        todo!()
    }

}
