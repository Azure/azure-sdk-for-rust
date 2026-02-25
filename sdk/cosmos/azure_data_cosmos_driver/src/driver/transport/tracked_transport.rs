// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Transport send-status inference utilities.
//!
//! This module determines whether a request was definitely sent on the wire before
//! a transport error occurred. The information is used by retry safety gates.

use crate::diagnostics::{RequestEvent, RequestEventType};
use azure_core::http::{
    policies::{Policy, PolicyResult},
    Context, Request, Transport,
};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RequestSentStatus {
    Sent,
    NotSent,
    Unknown,
}

impl RequestSentStatus {
    pub(crate) fn definitely_not_sent(self) -> bool {
        matches!(self, RequestSentStatus::NotSent)
    }
}

/// Infers from the error whether the request was definitely sent, not sent, or unknown.
pub(crate) fn infer_request_sent_status(error: &azure_core::Error) -> RequestSentStatus {
    use azure_core::error::ErrorKind;

    match error.kind() {
        // ErrorKind::Connection means the transport could not establish a
        // connection at all (DNS failure, connection refused, etc.).  The
        // request was never sent, so it is unconditionally safe to retry.
        // The reqwest transport maps `reqwest::Error::is_connect()` to this
        // variant, so no brittle string matching on Io messages is needed.
        ErrorKind::Connection | ErrorKind::Credential => RequestSentStatus::NotSent,
        // DataConversion could originate from request serialization (not sent)
        // or response deserialization (sent), so it is ambiguous.
        ErrorKind::DataConversion => RequestSentStatus::Unknown,
        ErrorKind::HttpResponse { .. } => RequestSentStatus::Sent,
        _ => RequestSentStatus::Unknown,
    }
}

pub(crate) trait RequestAttemptTelemetrySink: Send + Sync {
    fn mark_reached_transport(&self);
    fn set_request_sent_status(&self, request_sent_status: RequestSentStatus);
    fn record_event(&self, event: RequestEvent);
}

#[derive(Clone)]
pub(crate) struct RequestAttemptTelemetryContext(Arc<dyn RequestAttemptTelemetrySink>);

impl RequestAttemptTelemetryContext {
    pub(crate) fn new(sink: Arc<dyn RequestAttemptTelemetrySink>) -> Self {
        Self(sink)
    }

    pub(crate) fn sink(&self) -> &dyn RequestAttemptTelemetrySink {
        self.0.as_ref()
    }
}

#[derive(Debug)]
pub(crate) struct TrackedTransportPolicy {
    transport: Transport,
}

impl TrackedTransportPolicy {
    pub(crate) fn new(transport: Transport) -> Self {
        Self { transport }
    }
}

#[async_trait::async_trait]
impl Policy for TrackedTransportPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        assert_eq!(
            0,
            next.len(),
            "TrackedTransportPolicy must be the last policy"
        );

        if let Some(telemetry) = ctx.value::<RequestAttemptTelemetryContext>() {
            telemetry.sink().mark_reached_transport();
            telemetry
                .sink()
                .record_event(RequestEvent::new(RequestEventType::TransportStart));
        }

        match self.transport.send(ctx, request).await {
            Ok(response) => {
                if let Some(telemetry) = ctx.value::<RequestAttemptTelemetryContext>() {
                    telemetry
                        .sink()
                        .set_request_sent_status(RequestSentStatus::Sent);
                    telemetry
                        .sink()
                        .record_event(RequestEvent::new(RequestEventType::ResponseHeadersReceived));
                }
                Ok(response)
            }
            Err(error) => {
                let sent_status = infer_request_sent_status(&error);
                if let Some(telemetry) = ctx.value::<RequestAttemptTelemetryContext>() {
                    telemetry.sink().set_request_sent_status(sent_status);
                    telemetry.sink().record_event(
                        RequestEvent::new(RequestEventType::TransportFailed)
                            .with_details(error.to_string()),
                    );
                }
                Err(error)
            }
        }
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
        // DataConversion can happen during request serialization (not sent)
        // or response deserialization (sent), so the status is ambiguous.
        let err = azure_core::Error::new(ErrorKind::DataConversion, "serialization failed");
        assert_eq!(
            infer_request_sent_status(&err),
            RequestSentStatus::Unknown
        );
    }

    #[test]
    fn io_error_is_unknown() {
        // With reqwest, connection failures use ErrorKind::Connection, not Io.
        // A generic Io error has ambiguous send status.
        let err = azure_core::Error::new(ErrorKind::Io, "operation timed out");
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::Unknown);
    }

    #[test]
    fn unknown_error_is_unknown() {
        let err = azure_core::Error::new(ErrorKind::Other, "something went wrong");
        assert_eq!(infer_request_sent_status(&err), RequestSentStatus::Unknown);
    }
}
