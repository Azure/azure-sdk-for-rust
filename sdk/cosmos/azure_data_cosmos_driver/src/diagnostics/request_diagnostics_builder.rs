// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(feature = "fault_injection")]
use crate::fault_injection::FaultInjectionEvaluation;
use crate::{
    diagnostics::{
        DiagnosticsContextBuilder, FailedTransportShardDiagnostics, RequestEvent, RequestHandle,
        RequestSentStatus, TransportShardDiagnostics,
    },
    models::{CosmosResponseHeaders, CosmosStatus},
};
use azure_core::http::StatusCode;

/// Updates one request's diagnostics within its operation context.
pub(crate) struct RequestDiagnosticsBuilder<'a> {
    handle: RequestHandle,
    context: &'a mut DiagnosticsContextBuilder,
}

impl<'a> RequestDiagnosticsBuilder<'a> {
    /// Binds an existing request handle to the context that created it.
    pub fn new(handle: RequestHandle, context: &'a mut DiagnosticsContextBuilder) -> Self {
        Self { handle, context }
    }

    /// Appends an event to this request's timeline.
    pub fn add_event(&mut self, event: RequestEvent) {
        self.context.add_event(self.handle, event);
    }

    /// Records response metadata and completes this request.
    pub fn record_response(&mut self, status: StatusCode, headers: &CosmosResponseHeaders) {
        self.context.record_response(self.handle, status, headers);
    }

    /// Completes this request with a transport error and its sent status.
    pub fn fail_transport_request(
        &mut self,
        error: impl Into<String>,
        request_sent: RequestSentStatus,
        status: CosmosStatus,
    ) {
        self.context
            .fail_transport_request(self.handle, error, request_sent, status);
    }

    /// Marks this request as timed out and copies its terminal record to the hedge journal.
    pub fn timeout_request(&mut self) {
        self.context.timeout_request(self.handle);
    }

    /// Records the shard used by this request.
    pub fn set_transport_shard(&mut self, shard: TransportShardDiagnostics) {
        self.context.set_transport_shard(self.handle, shard);
    }

    /// Adds a prior failed shard to this request's retry history.
    pub fn add_failed_transport_shard(&mut self, shard: FailedTransportShardDiagnostics) {
        self.context.add_failed_transport_shard(self.handle, shard);
    }

    /// Counts a local shard retry preceding this request.
    pub fn increment_local_shard_retry_count(&mut self) {
        self.context.increment_local_shard_retry_count(self.handle);
    }

    /// Returns whether fault-injection evaluations should be collected for this request.
    #[cfg(feature = "fault_injection")]
    pub fn fault_injection_enabled(&self) -> bool {
        self.context.fault_injection_enabled()
    }

    /// Attaches evaluations without changing the already-recorded terminal hedge-journal entry.
    #[cfg(feature = "fault_injection")]
    pub fn set_fault_injection_evaluations(&mut self, evaluations: Vec<FaultInjectionEvaluation>) {
        self.context
            .set_fault_injection_evaluations(self.handle, evaluations);
    }
}
