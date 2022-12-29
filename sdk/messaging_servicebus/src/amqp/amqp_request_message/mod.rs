use fe2o3_amqp_types::messaging::ApplicationProperties;

use super::management_constants::{properties::SERVER_TIMEOUT, request::ASSOCIATED_LINK_NAME};

pub(crate) mod add_rule;
pub(crate) mod cancel_scheduled_message;
pub(crate) mod enumerate_rules;
pub(crate) mod get_session_state;
pub(crate) mod peek_message;
pub(crate) mod peek_session_message;
pub(crate) mod receive_by_sequence_number;
pub(crate) mod remove_rule;
pub(crate) mod renew_lock;
pub(crate) mod renew_session_lock;
pub(crate) mod schedule_message;
pub(crate) mod set_session_state;
pub(crate) mod update_disposition;

fn encode_application_properties(
    server_timeout: Option<u32>,
    associated_link_name: Option<String>,
) -> Option<ApplicationProperties> {
    let mut props = None;
    if let Some(server_timeout) = server_timeout {
        props
            .get_or_insert(ApplicationProperties::default())
            .insert(SERVER_TIMEOUT.into(), server_timeout.into());
    }
    if let Some(associated_link_name) = associated_link_name {
        props
            .get_or_insert(ApplicationProperties::default())
            .insert(ASSOCIATED_LINK_NAME.into(), associated_link_name.into());
    }
    props
}
