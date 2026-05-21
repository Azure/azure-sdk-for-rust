// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Transport send-status inference utilities.

use crate::diagnostics::RequestSentStatus;
use crate::error::{Error, Kind};
use crate::models::SubStatusCode;

/// Infers from a typed Cosmos error whether the request was definitely sent,
/// not sent, or unknown.
///
/// Discrimination is done on the categorical [`Kind`] and Cosmos sub-status
/// minted by the boundary mapper in [`crate::error`], so the predicate works
/// regardless of whether the underlying failure originated in `azure_core`,
/// `reqwest`, or somewhere else.
pub(crate) fn infer_request_sent_status(error: &Error) -> RequestSentStatus {
    match error.kind() {
        // Pre-flight: never reached the wire.
        Kind::Authentication => RequestSentStatus::NotSent,
        Kind::Transport
            if matches!(
                error.sub_status(),
                Some(SubStatusCode::TRANSPORT_CONNECTION_FAILED)
            ) =>
        {
            RequestSentStatus::NotSent
        }
        // A real HTTP response came back.
        Kind::Service => RequestSentStatus::Sent,
        // Everything else (generic transport I/O, serialization, client,
        // configuration) could go either way at this point.
        _ => RequestSentStatus::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::error::ErrorKind;

    fn cosmos_from(az: azure_core::Error) -> Error {
        Error::from(az)
    }

    #[test]
    fn connection_error_not_sent() {
        let err = cosmos_from(azure_core::Error::with_message(
            ErrorKind::Connection,
            "connection refused",
        ));
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::NotSent);
    }

    #[test]
    fn credential_error_not_sent() {
        let err = cosmos_from(azure_core::Error::new(
            ErrorKind::Credential,
            "invalid token",
        ));
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::NotSent);
    }

    #[test]
    fn data_conversion_error_is_unknown() {
        let err = cosmos_from(azure_core::Error::new(
            ErrorKind::DataConversion,
            "serialization failed",
        ));
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::Unknown);
    }

    #[test]
    fn io_error_is_unknown() {
        let err = cosmos_from(azure_core::Error::new(ErrorKind::Io, "operation timed out"));
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::Unknown);
    }

    #[test]
    fn unknown_error_is_unknown() {
        let err = cosmos_from(azure_core::Error::new(
            ErrorKind::Other,
            "something went wrong",
        ));
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::Unknown);
    }
}
