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
        // Failure modes that provably precede any request bytes going onto
        // the wire:
        //
        // * `TRANSPORT_CONNECTION_FAILED` — TCP connect refused / reset
        //   before the HTTP layer.
        // * `TRANSPORT_DNS_FAILED` — name resolution failed; no socket was
        //   ever opened to send anything on.
        // * `TRANSPORT_HTTP2_INCOMPATIBLE` — HTTP/2 protocol negotiation
        //   was rejected (e.g. `HTTP_1_1_REQUIRED`) during the preface
        //   exchange, before the request frame is emitted.
        //
        // Classifying these as `NotSent` preserves the pre-refactor
        // contract that callers (notably retry policies for non-idempotent
        // writes like Create / Replace / PATCH) used to rely on under
        // `azure_core::ErrorKind::Connection`. Generic
        // `TRANSPORT_IO_FAILED` is deliberately *not* included — it can
        // fire mid-stream after request bytes left the socket and so must
        // stay `Unknown`.
        Kind::Transport
            if matches!(
                error.sub_status(),
                Some(SubStatusCode::TRANSPORT_CONNECTION_FAILED)
                    | Some(SubStatusCode::TRANSPORT_DNS_FAILED)
                    | Some(SubStatusCode::TRANSPORT_HTTP2_INCOMPATIBLE)
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
    fn dns_error_not_sent() {
        // DNS resolution provably precedes wire I/O. The boundary mapper
        // reclassifies an `io::ErrorKind::NotFound` inside an `Io` chain
        // to `TRANSPORT_DNS_FAILED`; the contract here is that retry
        // policies for non-idempotent writes see `NotSent` and may
        // safely retry.
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "dns lookup failed");
        let err = cosmos_from(azure_core::Error::new(ErrorKind::Io, io_err));
        assert_eq!(
            err.sub_status(),
            Some(SubStatusCode::TRANSPORT_DNS_FAILED),
            "boundary mapper must classify NotFound IO as DNS"
        );
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::NotSent);
    }

    #[cfg(feature = "reqwest")]
    #[test]
    fn http2_error_not_sent() {
        // HTTP/2 protocol negotiation (e.g. `HTTP_1_1_REQUIRED`) fails
        // during the preface exchange, before the request frame goes out
        // — same `NotSent` semantics as a pre-connect failure.
        let h2_err: h2::Error = h2::Reason::HTTP_1_1_REQUIRED.into();
        let err = cosmos_from(azure_core::Error::new(ErrorKind::Io, h2_err));
        assert_eq!(
            err.sub_status(),
            Some(SubStatusCode::TRANSPORT_HTTP2_INCOMPATIBLE),
            "boundary mapper must classify h2 protocol errors"
        );
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::NotSent);
    }

    #[test]
    fn generic_io_error_stays_unknown() {
        // Generic `TRANSPORT_IO_FAILED` (no DNS / HTTP2 refinement) can
        // fire mid-stream after request bytes already left the socket,
        // so it must remain `Unknown` — retry policies for non-idempotent
        // writes need to fall back to idempotency-token handling.
        let io_err = std::io::Error::other("mid-stream read failed");
        let err = cosmos_from(azure_core::Error::new(ErrorKind::Io, io_err));
        assert_eq!(
            err.sub_status(),
            Some(SubStatusCode::TRANSPORT_IO_FAILED),
            "boundary mapper must keep generic IO as IO_FAILED"
        );
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
