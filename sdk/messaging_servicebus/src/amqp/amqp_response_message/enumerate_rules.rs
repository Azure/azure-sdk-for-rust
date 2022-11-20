use fe2o3_amqp_management::response::Response;
use serde_amqp::{DeserializeComposite, SerializeComposite};

pub(crate) struct EnumerateRulesResponse {}

impl Response for EnumerateRulesResponse {
    const STATUS_CODE: u16 = 200;

    type Body = ();

    type Error = super::ManagementError;

    fn decode_message(
        message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}
