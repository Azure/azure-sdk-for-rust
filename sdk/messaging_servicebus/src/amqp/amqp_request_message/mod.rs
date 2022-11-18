use std::time::Duration as StdDuration;

use fe2o3_amqp_types::{primitives::OrderedMap, messaging::ApplicationProperties};
use serde_amqp::Value;

use super::management_constants::properties::SERVER_TIMEOUT;

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
// pub(crate) trait IntoMessage {
//     fn into_message(self) -> Message<Body<Value>>;
// }

pub(crate) struct AmqpRequestMessage<'a> {
    pub operation: &'a str, // All possible operations should be constants that are defined in the crate
    pub map: OrderedMap<String, Value>,
    pub timeout: StdDuration,
}

fn encode_server_timeout_as_application_properties(server_timeout: Option<u32>) -> Option<ApplicationProperties> {
    let server_timeout = server_timeout?;
    Some(
        ApplicationProperties::builder()
            .insert(SERVER_TIMEOUT, server_timeout)
            .build(),
    )
}
