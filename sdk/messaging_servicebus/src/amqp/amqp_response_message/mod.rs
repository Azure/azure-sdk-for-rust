use fe2o3_amqp_management::error::Error as MgmtError;
use fe2o3_amqp_management::{
    error::{InvalidType, StatusError},
    mgmt_ext::AmqpMessageManagementExt,
    status::StatusCode,
};
use fe2o3_amqp_types::{messaging::Message, primitives::OrderedMap};
use serde_amqp::Value;

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

const HTTP_STATUS_CODE_OK: u16 = 200;
const HTTP_STATUS_CODE_NO_CONTENT: u16 = 204;

fn verify_ok_or_no_content_status_code<B>(
    message: &mut Message<B>,
) -> Result<StatusCode, MgmtError> {
    let status_code = match message
        .remove_status_code()
        .ok_or(MgmtError::StatusCodeNotFound)?
    {
        Ok(code) => code,
        Err(err) => {
            return Err(InvalidType {
                expected: "u16".to_string(),
                actual: format!("{:?}", err),
            }
            .into())
        }
    };
    match status_code.0.get() {
        HTTP_STATUS_CODE_OK | HTTP_STATUS_CODE_NO_CONTENT => Ok(status_code),
        _ => {
            let description = match message.remove_status_description() {
                Some(Ok(status_description)) => Some(status_description),
                Some(Err(err)) => {
                    return Err(InvalidType {
                        expected: "String".to_string(),
                        actual: format!("{:?}", err),
                    }
                    .into())
                }
                None => None,
            };
            Err(StatusError {
                code: status_code,
                description: description.map(Into::into),
            }
            .into())
        }
    }
}
