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
        // Classifying these as `NotSent` is what lets retry policies for
        // non-idempotent writes (Create / Replace / PATCH) safely retry.
        // Generic `TRANSPORT_IO_FAILED` is deliberately *not* included —
        // it can fire mid-stream after request bytes left the socket and
        // so must stay `Unknown`.
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
    use crate::models::CosmosStatus;

    fn transport_err(status: CosmosStatus) -> Error {
        Error::builder(Kind::Transport)
            .with_status(status)
            .with_message("synthetic")
            .build()
    }

    #[test]
    fn connection_failed_not_sent() {
        let err = transport_err(CosmosStatus::TRANSPORT_CONNECTION_FAILED);
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::NotSent);
    }

    #[test]
    fn dns_failed_not_sent() {
        let err = transport_err(CosmosStatus::TRANSPORT_DNS_FAILED);
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::NotSent);
    }

    #[test]
    fn http2_incompatible_not_sent() {
        let err = transport_err(CosmosStatus::TRANSPORT_HTTP2_INCOMPATIBLE);
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::NotSent);
    }

    #[test]
    fn generic_transport_io_is_unknown() {
        let err = transport_err(CosmosStatus::TRANSPORT_IO_FAILED);
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::Unknown);
    }

    #[test]
    fn client_error_is_unknown() {
        let err = Error::builder(Kind::Client)
            .with_message("bad input")
            .build();
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::Unknown);
    }

    #[test]
    fn serialization_error_is_unknown() {
        let err = Error::builder(Kind::Serialization)
            .with_message("bad json")
            .with_source(std::io::Error::other("stub"))
            .build();
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::Unknown);
    }

    #[test]
    fn authentication_error_not_sent() {
        let err = Error::builder(Kind::Authentication)
            .with_message("invalid token")
            .build();
        assert_eq!(err.kind(), Kind::Authentication);
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::NotSent);
    }
}
