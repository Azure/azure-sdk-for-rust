use fe2o3_amqp_types::primitives::OrderedMap;
use serde_amqp::Value;

pub(crate) mod add_rule;
pub(crate) mod cancel_scheduled_message;
pub(crate) mod enumerate_rules;
pub(crate) mod get_session_state;
pub(crate) mod peek_message;
pub(crate) mod receive_by_sequence_number;
pub(crate) mod remove_rule;
pub(crate) mod renew_lock;
pub(crate) mod renew_session_lock;
pub(crate) mod schedule_message;
pub(crate) mod set_session_state;
pub(crate) mod update_disposition;

// /// TODO: This is a temporary solution to get the request/response working, as it seems like there
// /// is some inconsistency between the cbs field constants vs the management field constants.
// pub(crate) trait FromMessage {
//     type Error;

//     fn from_message(message: Message<Body<Value>>) -> Result<Self, Self::Error>
//     where
//         Self: Sized;
// }

pub(crate) struct AmqpResponseMessage {
    pub map: OrderedMap<String, Value>,
}
