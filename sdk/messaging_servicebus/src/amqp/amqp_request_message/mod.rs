use std::time::Duration as StdDuration;

use fe2o3_amqp_types::{messaging::ApplicationProperties, primitives::OrderedMap};
use serde_amqp::Value;

use super::management_constants::properties::SERVER_TIMEOUT;

pub(crate) mod add_rule;
pub(crate) mod cancel_scheduled_message;
pub(crate) mod enumerate_rules;
pub mod error;
pub(crate) mod get_session_state;
pub(crate) mod peek_message;
pub(crate) mod peek_session_pessage;
pub(crate) mod receive_by_sequence_number;
pub(crate) mod remove_rule;
pub(crate) mod renew_lock;
pub(crate) mod renew_session_lock;
pub(crate) mod schedule_message;
pub(crate) mod set_session_state;
pub(crate) mod update_disposition;

fn encode_server_timeout_as_application_properties(
    server_timeout: Option<u32>,
) -> Option<ApplicationProperties> {
    let server_timeout = server_timeout?;
    Some(
        ApplicationProperties::builder()
            .insert(SERVER_TIMEOUT, server_timeout)
            .build(),
    )
}
