// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::atomic::{AtomicUsize, Ordering};

use azure_data_cosmos_driver::options::ReadConsistencyStrategy;

#[derive(Debug, Default)]
pub(crate) struct HostMetrics {
    binary_negotiated_requests: AtomicUsize,
    binary_payload_requests: AtomicUsize,
    binary_response_payloads: AtomicUsize,
    wire_default_consistency_requests: AtomicUsize,
    wire_eventual_consistency_requests: AtomicUsize,
    wire_session_consistency_requests: AtomicUsize,
    wire_latest_committed_consistency_requests: AtomicUsize,
    wire_global_strong_consistency_requests: AtomicUsize,
    connectivity_probes: AtomicUsize,
    gateway_requests: AtomicUsize,
    gateway20_requests: AtomicUsize,
}

impl HostMetrics {
    pub(crate) fn record_binary_request(&self, negotiated: bool, binary_payload: bool) {
        if negotiated {
            self.binary_negotiated_requests
                .fetch_add(1, Ordering::Relaxed);
        }
        if binary_payload {
            self.binary_payload_requests.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub(crate) fn record_binary_response(&self, binary_payload: bool) {
        if binary_payload {
            self.binary_response_payloads
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    pub(crate) fn binary_negotiated_requests(&self) -> usize {
        self.binary_negotiated_requests.load(Ordering::Relaxed)
    }

    pub(crate) fn binary_payload_requests(&self) -> usize {
        self.binary_payload_requests.load(Ordering::Relaxed)
    }

    pub(crate) fn binary_response_payloads(&self) -> usize {
        self.binary_response_payloads.load(Ordering::Relaxed)
    }

    pub(crate) fn record_read_consistency_strategy(
        &self,
        strategy: Option<ReadConsistencyStrategy>,
    ) {
        let counter = match strategy.unwrap_or(ReadConsistencyStrategy::Default) {
            ReadConsistencyStrategy::Default => &self.wire_default_consistency_requests,
            ReadConsistencyStrategy::Eventual => &self.wire_eventual_consistency_requests,
            ReadConsistencyStrategy::Session => &self.wire_session_consistency_requests,
            ReadConsistencyStrategy::LatestCommitted => {
                &self.wire_latest_committed_consistency_requests
            }
            ReadConsistencyStrategy::GlobalStrong => &self.wire_global_strong_consistency_requests,
            _ => return,
        };
        counter.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn wire_default_consistency_requests(&self) -> usize {
        self.wire_default_consistency_requests
            .load(Ordering::Relaxed)
    }

    pub(crate) fn wire_eventual_consistency_requests(&self) -> usize {
        self.wire_eventual_consistency_requests
            .load(Ordering::Relaxed)
    }

    pub(crate) fn wire_session_consistency_requests(&self) -> usize {
        self.wire_session_consistency_requests
            .load(Ordering::Relaxed)
    }

    pub(crate) fn wire_latest_committed_consistency_requests(&self) -> usize {
        self.wire_latest_committed_consistency_requests
            .load(Ordering::Relaxed)
    }

    pub(crate) fn wire_global_strong_consistency_requests(&self) -> usize {
        self.wire_global_strong_consistency_requests
            .load(Ordering::Relaxed)
    }

    pub(crate) fn record_connectivity_probe(&self) {
        self.connectivity_probes.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn record_gateway20_request(&self) {
        self.gateway20_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn record_gateway_request(&self) {
        self.gateway_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn connectivity_probes(&self) -> usize {
        self.connectivity_probes.load(Ordering::Relaxed)
    }

    pub(crate) fn gateway20_requests(&self) -> usize {
        self.gateway20_requests.load(Ordering::Relaxed)
    }

    pub(crate) fn gateway_requests(&self) -> usize {
        self.gateway_requests.load(Ordering::Relaxed)
    }
}
