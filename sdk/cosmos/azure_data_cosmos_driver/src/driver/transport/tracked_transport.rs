// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Transport send-status inference utilities.

use crate::diagnostics::RequestSentStatus;

/// Infers from the error whether the request was definitely sent, not sent, or unknown.
pub(crate) fn infer_request_sent_status(error: &azure_core::Error) -> RequestSentStatus {
    use azure_core::error::ErrorKind;

    match error.kind() {
        // Connection means the transport could not establish a connection.
        ErrorKind::Connection | ErrorKind::Credential => RequestSentStatus::NotSent,
        // DataConversion can happen before send (serialization) or after send (deserialization).
        ErrorKind::DataConversion => RequestSentStatus::Unknown,
        ErrorKind::HttpResponse { .. } => RequestSentStatus::Sent,
        _ => RequestSentStatus::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::error::ErrorKind;

    #[test]
    fn connection_error_not_sent() {
        let err = azure_core::Error::with_message(ErrorKind::Connection, "connection refused");
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::NotSent);
    }

    #[test]
    fn credential_error_not_sent() {
        let err = azure_core::Error::new(ErrorKind::Credential, "invalid token");
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::NotSent);
    }

    #[test]
    fn data_conversion_error_is_unknown() {
        let err = azure_core::Error::new(ErrorKind::DataConversion, "serialization failed");
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::Unknown);
    }

    #[test]
    fn io_error_is_unknown() {
        let err = azure_core::Error::new(ErrorKind::Io, "operation timed out");
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::Unknown);
    }

    #[test]
    fn unknown_error_is_unknown() {
        let err = azure_core::Error::new(ErrorKind::Other, "something went wrong");
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::Unknown);
    }
}
