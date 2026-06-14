// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Shared `RequestObserver` for hosts, excluding account-topology `GET /` fetches from routing assertions.

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

    /// Hosts of all non-topology requests; `GET /` may legitimately target the global endpoint.
    pub fn data_plane_hosts(&self) -> Vec<String> {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, is_account)| !*is_account)
            .map(|(host, _)| host.clone())
            .collect()
    }

    /// Number of `GET /` account-topology fetches; call `clear()` to scope the count.
    #[cfg_attr(not(feature = "fault_injection"), allow(dead_code))]
    pub fn account_read_count(&self) -> usize {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, is_account)| *is_account)
            .count()
    }

    /// Hosts of `GET /` topology fetches only.
    #[cfg_attr(not(feature = "fault_injection"), allow(dead_code))]
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
