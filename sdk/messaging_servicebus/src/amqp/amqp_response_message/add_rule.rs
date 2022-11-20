use fe2o3_amqp_management::response::Response;
use fe2o3_amqp_types::messaging::Body;
use serde_amqp::Value;

pub(crate) struct AddRuleResponse {}

impl Response for AddRuleResponse {
    const STATUS_CODE: u16 = 200;

    type Body = Body<Value>;

    type Error = super::ManagementError;

    fn decode_message(
        _message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}
