// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Transport tracking for request lifecycle events.
//!
//! This module provides utilities for tracking the lifecycle of HTTP requests,
//! including determining whether a request was sent on the wire before an error
//! occurred. This information is critical for retry safety decisions.

use crate::diagnostics::{RequestEvent, RequestEventType, RequestSentStatus};
use std::sync::mpsc;

/// A channel sender for publishing request events.
///
/// This is cloned and passed through the pipeline to allow different stages
/// to emit events about the request lifecycle.
pub type EventSender = mpsc::Sender<RequestEvent>;

/// A channel receiver for consuming request events.
pub type EventReceiver = mpsc::Receiver<RequestEvent>;

/// Creates a new event channel for tracking request lifecycle events.
///
/// Returns a sender/receiver pair. The sender can be cloned and passed to
/// transport components, while the receiver is used to collect events after
/// the request completes (or fails).
pub fn event_channel() -> (EventSender, EventReceiver) {
    mpsc::channel()
}

/// Context extension for carrying event sender through the pipeline.
///
/// This is inserted into the `Context` before sending a request, allowing
/// any policy or transport component to emit events.
#[derive(Clone)]
pub(crate) struct EventEmitter {
    sender: EventSender,
}

impl std::fmt::Debug for EventEmitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventEmitter").finish_non_exhaustive()
    }
}

impl EventEmitter {
    /// Creates a new event emitter wrapping the given sender.
    pub(crate) fn new(sender: EventSender) -> Self {
        Self { sender }
    }

    /// Emits a request event.
    ///
    /// If the channel is closed, the event is silently dropped.
    pub(crate) fn emit(&self, event: RequestEvent) {
        let _ = self.sender.send(event);
    }

    /// Emits a simple event with just the type.
    pub(crate) fn emit_type(&self, event_type: RequestEventType) {
        self.emit(RequestEvent::new(event_type));
    }

    /// Emits an event with details.
    pub(crate) fn emit_with_details(
        &self,
        event_type: RequestEventType,
        details: impl Into<String>,
    ) {
        self.emit(RequestEvent::new(event_type).with_details(details));
    }
}

/// Tracked request state that can be checked after request completion/failure.
///
/// This struct collects events from the request lifecycle and provides
/// methods to determine retry safety based on which events occurred.
pub(crate) struct TrackedRequestState {
    events: Vec<RequestEvent>,
}

impl TrackedRequestState {
    /// Creates a new tracked request state by collecting all events from the receiver.
    ///
    /// This drains the receiver, so it should only be called after the request
    /// has completed or failed.
    pub(crate) fn collect(receiver: EventReceiver) -> Self {
        let mut events = Vec::new();
        while let Ok(event) = receiver.try_recv() {
            events.push(event);
        }
        Self { events }
    }

    /// Consumes this state and returns the events.
    pub(crate) fn into_events(self) -> Vec<RequestEvent> {
        self.events
    }

    /// Returns the request sent status using both events and error analysis.
    ///
    /// This provides the most accurate determination by combining:
    /// 1. Event-based tracking (most reliable when available)
    /// 2. Error type heuristics (for errors that definitively indicate pre-send failure)
    ///
    /// # Returns
    /// - `Sent`: We received response headers (definitive)
    /// - `NotSent`: Error type indicates failure before transmission (e.g., DNS, connect)
    /// - `Unknown`: Cannot determine from available information
    pub fn request_sent_status_with_error(&self, error: &azure_core::Error) -> RequestSentStatus {
        // If any event indicates request was sent, it was definitely sent
        if self
            .events
            .iter()
            .any(|e| e.event_type().indicates_request_sent())
        {
            return RequestSentStatus::Sent;
        }

        // Use error heuristics to determine if request was definitely not sent
        error.request_sent_status()
    }
}

/// Extension trait for determining request sent status from errors.
///
/// This provides methods to extract send-status from transport errors using
/// heuristics based on error types.
pub(crate) trait RequestSentExt {
    /// Returns the request sent status based on error analysis.
    ///
    /// This uses heuristics based on the error type:
    /// - Connection refused, DNS errors: `NotSent` (definitely not sent)
    /// - Decode, redirect, status errors: `Sent` (response was received)
    /// - Other errors: `Unknown` (can't determine)
    fn request_sent_status(&self) -> RequestSentStatus;
}

impl RequestSentExt for azure_core::Error {
    fn request_sent_status(&self) -> RequestSentStatus {
        use azure_core::error::ErrorKind;

        match self.kind() {
            // IO errors - check message for pre-send failures
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
                // Other IO errors - can't determine
                RequestSentStatus::Unknown
            }
            // Credential/auth errors happen before sending
            ErrorKind::Credential => RequestSentStatus::NotSent,
            // Data conversion errors happen during request building
            ErrorKind::DataConversion => RequestSentStatus::NotSent,
            // HTTP status errors mean we got a response
            ErrorKind::HttpResponse { .. } => RequestSentStatus::Sent,
            // Other error types - can't determine
            _ => RequestSentStatus::Unknown,
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
        // Timeout without "connect" in message - can't determine
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
        // Other errors - can't determine
        assert_eq!(err.request_sent_status(), RequestSentStatus::Unknown);
    }

    #[test]
    fn events_with_error_dns_failure() {
        let (sender, receiver) = event_channel();
        let emitter = EventEmitter::new(sender);

        emitter.emit_type(RequestEventType::TransportStart);
        emitter.emit_type(RequestEventType::TransportFailed);

        let state = TrackedRequestState::collect(receiver);
        let err = azure_core::Error::new(ErrorKind::Io, "dns resolution failed");

        // With DNS error, we know it wasn't sent
        assert_eq!(
            state.request_sent_status_with_error(&err),
            RequestSentStatus::NotSent
        );
    }

    #[test]
    fn events_with_error_connect_refused() {
        let (sender, receiver) = event_channel();
        let emitter = EventEmitter::new(sender);

        emitter.emit_type(RequestEventType::TransportStart);
        emitter.emit_type(RequestEventType::TransportFailed);

        let state = TrackedRequestState::collect(receiver);
        let err = azure_core::Error::new(ErrorKind::Io, "connection refused");

        assert_eq!(
            state.request_sent_status_with_error(&err),
            RequestSentStatus::NotSent
        );
    }

    #[test]
    fn event_channel_works() {
        let (sender, receiver) = event_channel();
        let emitter = EventEmitter::new(sender);

        emitter.emit_type(RequestEventType::TransportStart);
        emitter.emit_type(RequestEventType::ResponseHeadersReceived);
        emitter.emit_type(RequestEventType::TransportComplete);

        let state = TrackedRequestState::collect(receiver);
        let events = state.into_events();
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn response_headers_received_indicates_sent() {
        let (sender, receiver) = event_channel();
        let emitter = EventEmitter::new(sender);

        emitter.emit_type(RequestEventType::TransportStart);
        emitter.emit_type(RequestEventType::ResponseHeadersReceived);
        // Simulate body buffering failure - no TransportComplete

        let state = TrackedRequestState::collect(receiver);
        // Even without TransportComplete, ResponseHeadersReceived means request was sent
        let err = azure_core::Error::new(ErrorKind::Other, "test error");
        assert_eq!(state.request_sent_status_with_error(&err), RequestSentStatus::Sent);
    }

    #[test]
    fn transport_failed_is_unknown() {
        let (sender, receiver) = event_channel();
        let emitter = EventEmitter::new(sender);

        emitter.emit_type(RequestEventType::TransportStart);
        emitter.emit_type(RequestEventType::TransportFailed);

        let state = TrackedRequestState::collect(receiver);
        // TransportFailed without headers received - we don't know if request was sent
        let err = azure_core::Error::new(ErrorKind::Other, "test error");
        assert_eq!(state.request_sent_status_with_error(&err), RequestSentStatus::Unknown);
    }

    #[test]
    fn empty_events_is_unknown() {
        let (_sender, receiver) = event_channel();
        // Don't emit any events
        let state = TrackedRequestState::collect(receiver);
        let err = azure_core::Error::new(ErrorKind::Other, "test error");
        assert_eq!(state.request_sent_status_with_error(&err), RequestSentStatus::Unknown);
    }

    #[test]
    fn headers_received_overrides_error_heuristics() {
        let (sender, receiver) = event_channel();
        let emitter = EventEmitter::new(sender);

        emitter.emit_type(RequestEventType::TransportStart);
        emitter.emit_type(RequestEventType::ResponseHeadersReceived);

        let state = TrackedRequestState::collect(receiver);
        // Even with an error that would normally indicate NotSent,
        // if we have ResponseHeadersReceived, we know it was sent
        let err = azure_core::Error::new(ErrorKind::Io, "connection refused");
        assert_eq!(
            state.request_sent_status_with_error(&err),
            RequestSentStatus::Sent
        );
    }
}
