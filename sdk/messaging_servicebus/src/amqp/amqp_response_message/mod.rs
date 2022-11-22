use fe2o3_amqp_management::error::Error as ManagementError;
use fe2o3_amqp_management::{
    error::{InvalidType, StatusError},
    mgmt_ext::AmqpMessageManagementExt,
    status::StatusCode,
};
use fe2o3_amqp_types::messaging::Message;

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

const HTTP_STATUS_CODE_OK: u16 = 200;
const HTTP_STATUS_CODE_NO_CONTENT: u16 = 204;

fn verify_ok_or_no_content_status_code<B>(
    message: &mut Message<B>,
) -> Result<StatusCode, ManagementError> {
    let status_code = match message
        .remove_status_code()
        .ok_or(ManagementError::StatusCodeNotFound)?
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
