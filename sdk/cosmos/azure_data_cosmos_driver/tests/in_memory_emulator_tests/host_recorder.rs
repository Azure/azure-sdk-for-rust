// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Shared `RequestObserver` that records the host of every dispatched
//! request along with whether the request was an account-topology fetch
//! (`GET /`).
//!
//! Tests filter out the `GET /` requests when asserting where data-plane
//! and metadata-CRUD traffic landed, because account-topology fetches
//! bypass the routing path under test (they legitimately target the
//! global endpoint during bootstrap and periodic refresh).
//!
//! Lifted from `excluded_regions_fallback.rs` so both that file and the
//! `regional_gateway_unreachable.rs` tests (and any future
//! observer-driven tests) share a single implementation.

use std::sync::{Arc, Mutex};

use azure_core::http::{Method, Request};
use azure_data_cosmos_driver::in_memory_emulator::RequestObserver;

/// Tuple stored per observation: `(host, is_account_GET_/)`.
type Observation = (String, bool);

/// Records the host of every request observed by the in-memory emulator.
#[derive(Debug, Default)]
pub struct HostRecorder {
    requests: Mutex<Vec<Observation>>,
}

impl HostRecorder {
    /// Creates an empty recorder behind an `Arc` (the form required by
    /// `InMemoryEmulatorHttpClient::with_request_observer`).
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Hosts of all requests EXCEPT account-topology reads (`GET /`).
    /// Topology fetches legitimately target the global endpoint during
    /// bootstrap; tests filter them out when asserting data-plane
    /// landing sites.
    pub fn data_plane_hosts(&self) -> Vec<String> {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, is_account)| !*is_account)
            .map(|(host, _)| host.clone())
            .collect()
    }

    /// Total number of observations, including `GET /` topology fetches.
    #[allow(dead_code)]
    pub fn total_count(&self) -> usize {
        self.requests.lock().unwrap().len()
    }

    /// Number of `GET /` account-topology fetches captured. Tests use
    /// this as the signal that the SDK triggered an account refresh
    /// (typically after a topology-changing failure like 403/1008 or
    /// 403/3); call `clear()` first to scope the count to the
    /// post-setup window.
    #[allow(dead_code)]
    pub fn account_read_count(&self) -> usize {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, is_account)| *is_account)
            .count()
    }

    /// Hosts of `GET /` topology fetches only.
    #[allow(dead_code)]
    pub fn topology_hosts(&self) -> Vec<String> {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, is_account)| *is_account)
            .map(|(host, _)| host.clone())
            .collect()
    }

    /// Drops every captured observation. Tests call this after seeding
    /// or warm-up to isolate the post-setup assertions.
    pub fn clear(&self) {
        self.requests.lock().unwrap().clear();
    }
}

impl RequestObserver for HostRecorder {
    fn on_request(&self, request: &Request) {
        let host = request.url().host_str().unwrap_or_default().to_string();
        let is_account_read = request.method() == Method::Get && request.url().path() == "/";
        self.requests.lock().unwrap().push((host, is_account_read));
    }
}
