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

pub(crate) trait RequestSentExt {
    fn request_sent_status(&self) -> RequestSentStatus;
}

impl RequestSentExt for azure_core::Error {
    fn request_sent_status(&self) -> RequestSentStatus {
        use azure_core::error::ErrorKind;

        match self.kind() {
            ErrorKind::Io => {
                let msg = self.to_string().to_lowercase();
                if msg.contains("dns")
                    || msg.contains("resolve")
                    || msg.contains("connection refused")
                    || msg.contains("no route to host")
                    || msg.contains("network unreachable")
                    || msg.contains("connection reset")
                        && (msg.contains("before") || msg.contains("establish"))
                {
                    return RequestSentStatus::NotSent;
                }
                RequestSentStatus::Unknown
            }
            ErrorKind::Credential => RequestSentStatus::NotSent,
            ErrorKind::DataConversion => RequestSentStatus::NotSent,
            ErrorKind::HttpResponse { .. } => RequestSentStatus::Sent,
            _ => RequestSentStatus::Unknown,
        }
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
                let sent_status = error.request_sent_status();
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
    fn dns_error_not_sent() {
        let err = azure_core::Error::new(ErrorKind::Io, "dns resolution failed");
        assert_eq!(err.request_sent_status(), RequestSentStatus::NotSent);
    }

    #[test]
    fn connection_refused_not_sent() {
        let err = azure_core::Error::new(ErrorKind::Io, "connection refused");
        assert_eq!(err.request_sent_status(), RequestSentStatus::NotSent);
    }

    #[test]
    fn timeout_is_unknown() {
        let err = azure_core::Error::new(ErrorKind::Io, "operation timed out");
        assert_eq!(err.request_sent_status(), RequestSentStatus::Unknown);
    }

    #[test]
    fn credential_error_not_sent() {
        let err = azure_core::Error::new(ErrorKind::Credential, "invalid token");
        assert_eq!(err.request_sent_status(), RequestSentStatus::NotSent);
    }

    #[test]
    fn data_conversion_error_not_sent() {
        let err = azure_core::Error::new(ErrorKind::DataConversion, "serialization failed");
        assert_eq!(err.request_sent_status(), RequestSentStatus::NotSent);
    }

    #[test]
    fn unknown_error_is_unknown() {
        let err = azure_core::Error::new(ErrorKind::Other, "something went wrong");
        assert_eq!(err.request_sent_status(), RequestSentStatus::Unknown);
    }
}
