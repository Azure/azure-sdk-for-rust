//! The set of annotations for service responses associated with an AMQP messages and
//! entities.

use super::amqp_response_status_code::AmqpResponseStatusCode;

/// The annotation that identifies the code of a response status
pub const STATUS_CODE: &str = "status-code";

/// The annotation that identifies the description of a response status
pub const STATUS_DESCRIPTION: &str = "status-description";

/// The annotation that identifies an error response
pub const ERROR_CONDITION: &str = "error-condition";

/// Determines whether the given AMQP status code value should be considered a successful
/// request.
pub fn is_success_status(status_code: AmqpResponseStatusCode) -> bool {
    match status_code {
        AmqpResponseStatusCode::OK => true,
        AmqpResponseStatusCode::Accepted => true,
        _ => false,
    }
}
