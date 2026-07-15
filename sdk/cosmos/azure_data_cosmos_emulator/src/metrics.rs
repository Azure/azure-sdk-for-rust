// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Default)]
pub(crate) struct HostMetrics {
    connectivity_probes: AtomicUsize,
    gateway20_requests: AtomicUsize,
}

impl HostMetrics {
    pub(crate) fn record_connectivity_probe(&self) {
        self.connectivity_probes.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn record_gateway20_request(&self) {
        self.gateway20_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn connectivity_probes(&self) -> usize {
        self.connectivity_probes.load(Ordering::Relaxed)
    }

    pub(crate) fn gateway20_requests(&self) -> usize {
        self.gateway20_requests.load(Ordering::Relaxed)
    }
}
