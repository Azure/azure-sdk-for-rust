// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Shared `RequestObserver` for hosts, excluding metadata fetches from routing assertions.

use std::sync::{Arc, Mutex};

use azure_core::http::{Method, Request};
use azure_data_cosmos_driver::in_memory_emulator::RequestObserver;

static IS_QUERY: azure_core::http::headers::HeaderName =
    azure_core::http::headers::HeaderName::from_static("x-ms-documentdb-isquery");
static IS_QUERY_PLAN: azure_core::http::headers::HeaderName =
    azure_core::http::headers::HeaderName::from_static("x-ms-cosmos-is-query-plan-request");

/// Classification of a recorded request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RequestKind {
    /// Account topology fetch (`GET /`).
    Topology,
    /// Routing metadata (e.g., `GET .../pkranges`).
    RoutingMetadata,
    /// Gateway query-plan request.
    QueryPlan,
    /// Query execution request.
    DocumentQuery,
    /// Actual data-plane operation (item CRUD).
    DataPlane,
}

/// Tuple stored per observation: `(host, kind)`.
type Observation = (String, RequestKind);

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

    /// Hosts of actual data-plane operations (item CRUD), excluding topology
    /// fetches (`GET /`) and routing metadata (`GET .../pkranges`).
    pub fn data_plane_hosts(&self) -> Vec<String> {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, kind)| *kind == RequestKind::DataPlane)
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
            .filter(|(_, kind)| *kind == RequestKind::Topology)
            .count()
    }

    pub fn query_plan_count(&self) -> usize {
        self.count(RequestKind::QueryPlan)
    }

    #[cfg_attr(feature = "__internal_native_query_plan", allow(dead_code))]
    pub fn document_query_count(&self) -> usize {
        self.count(RequestKind::DocumentQuery)
    }

    #[cfg_attr(feature = "__internal_native_query_plan", allow(dead_code))]
    pub fn routing_metadata_count(&self) -> usize {
        self.count(RequestKind::RoutingMetadata)
    }

    fn count(&self, kind: RequestKind) -> usize {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, observed)| *observed == kind)
            .count()
    }

    /// Hosts of `GET /` topology fetches only.
    #[cfg_attr(not(feature = "fault_injection"), allow(dead_code))]
    pub fn topology_hosts(&self) -> Vec<String> {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, kind)| *kind == RequestKind::Topology)
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
        let path = request.url().path();
        let method = request.method();

        let is_true = |name| {
            request
                .headers()
                .get_optional_str(name)
                .is_some_and(|value| value.eq_ignore_ascii_case("true"))
        };
        let kind = if method == Method::Get && path == "/" {
            RequestKind::Topology
        } else if method == Method::Get && path.ends_with("/pkranges") {
            RequestKind::RoutingMetadata
        } else if is_true(&IS_QUERY_PLAN) {
            RequestKind::QueryPlan
        } else if is_true(&IS_QUERY) {
            RequestKind::DocumentQuery
        } else {
            RequestKind::DataPlane
        };

        self.requests.lock().unwrap().push((host, kind));
    }
}
