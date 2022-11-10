use fe2o3_amqp_types::messaging::{Body, Message};
use serde_amqp::Value;

/// TODO: This is a temporary solution to get the request/response working, as it seems like there
/// is some inconsistency between the cbs field constants vs the management field constants.
pub(crate) trait IntoMessage {
    fn into_message(self) -> Message<Body<Value>>;
}
