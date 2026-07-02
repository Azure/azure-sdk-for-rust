// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HTTP/2 connectivity probe for Gateway 2.0 (thin-client) proxy endpoints.
//!
//! After the SDK discovers thin-client endpoints from a `getDatabaseAccount`
//! response (`thinClientWritableLocations` / `thinClientReadableLocations`),
//! it issues a `POST /connectivity-probe` to every discovered proxy endpoint
//! before sending real RNTBD traffic. The probe proves TCP + TLS + HTTP/2
//! reachability to port 10250 (firewall / NSG / Private Endpoint
//! misconfigurations otherwise surface as confusing timeout errors).
//!
//! ## Contract
//!
//! The wire contract is:
//!
//! * **Path**: `POST /connectivity-probe`
//! * **Body**: empty on request, empty on response
//! * **Protocol**: HTTP/2 required (the proxy is `Nghttp2ProxyProtocolHandler`)
//! * **Responses**:
//!   * `200 OK` — probe enabled, proxy ready
//!   * `503 Service Unavailable` — `enableConnectivityProbe` flag is OFF on
//!     this federation; proxy is reachable but the feature is not yet
//!     active for data plane routing
//!   * any other status, connection refused, or timeout — proxy unreachable
//!
//! ## SDK gating policy
//!
//! The Rust SDK applies a **strict, all-or-nothing** policy:
//!
//! 1. **Strict**: only `200` counts as success. A `503` (feature disabled)
//!    fails the probe. The federation has not opted in to Gateway 2.0 yet,
//!    so the data plane stays on Gateway V1.
//! 2. **All-or-nothing**: if any probed region returns non-200, drop
//!    Gateway 2.0 for *all* regions and use Gateway V1 everywhere until a
//!    subsequent probe pass succeeds on every region.
//! 3. **No opt-in**: the probe runs whenever `thinClient*Locations` are
//!    advertised. There is no SDK-side opt-out — the federation flag is the
//!    operator-facing kill switch.

use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use azure_core::http::{headers::Headers, Method};
use futures::future::join_all;
use url::Url;

use crate::options::Region;

use super::cosmos_transport_client::{HttpRequest, HttpResponse, TransportClient, TransportError};

/// Wire path used for the connectivity probe POST request. Defined on the
/// proxy side in `Nghttp2ProxyProtocolHandler.h::ConnectivityProbePath`.
pub(crate) const CONNECTIVITY_PROBE_PATH: &str = "/connectivity-probe";

/// Default per-probe deadline. Probes are intentionally cheap (no body, no
/// auth, no backend routing) so a short timeout is safe and keeps the gating
/// loop responsive.
pub(crate) const DEFAULT_PROBE_TIMEOUT: Duration = Duration::from_secs(5);

/// Reason a single probe call did not return `200`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ProbeFailure {
    /// Proxy responded with a non-200 status. `503` typically indicates the
    /// `enableConnectivityProbe` federation flag is off; everything else
    /// indicates a real protocol-level mismatch.
    Status(u16),
    /// Network-level failure (connection refused, TLS handshake failed,
    /// timeout, etc.). The message preserves the underlying transport error
    /// text for diagnostics.
    Network(String),
}

impl std::fmt::Display for ProbeFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProbeFailure::Status(code) => write!(f, "status {code}"),
            ProbeFailure::Network(msg) => write!(f, "network: {msg}"),
        }
    }
}

/// Result of probing every advertised Gateway 2.0 endpoint.
///
/// `AllHealthy` requires every probed URL to return `200`. Any other outcome
/// (including a single 503 or a single timeout) collapses into `Failed` so
/// the all-or-nothing gate can be applied uniformly across regions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ProbeOutcome {
    AllHealthy,
    Failed {
        /// Per-region attribution for the failing endpoints. When the same
        /// URL is advertised for multiple regions (e.g. listed in both
        /// readable and writable lists), each `(region, role)` pair appears
        /// here even though only one network call was made.
        failures: Vec<(Region, ProbeFailure)>,
    },
}

/// Trait abstracting the connectivity probe so tests can inject canned
/// outcomes without standing up an HTTP server.
#[async_trait::async_trait]
pub(crate) trait ConnectivityProbe: Send + Sync + std::fmt::Debug {
    /// Probes every supplied endpoint concurrently and returns the
    /// all-or-nothing aggregate outcome.
    ///
    /// `endpoints` is a flat list of `(region, role, base_url)` triples;
    /// duplicates by URL are deduplicated so each unique URL is probed once.
    async fn probe_endpoints(&self, endpoints: Vec<(Region, ProbeRole, Url)>) -> ProbeOutcome;
}

/// Whether an advertised proxy URL came from `thinClientReadableLocations`
/// or `thinClientWritableLocations`. Used only for diagnostics on a failed
/// probe — the gating decision treats both roles identically.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ProbeRole {
    Read,
    Write,
}

impl std::fmt::Display for ProbeRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProbeRole::Read => f.write_str("read"),
            ProbeRole::Write => f.write_str("write"),
        }
    }
}

/// HTTP/2-only probe backed by a [`TransportClient`] built from the same
/// `HttpClientFactory` the data plane uses for Gateway 2.0 traffic.
///
/// Constructing the probe through the Gateway 2.0 config (rather than the
/// metadata config) guarantees the probe negotiates HTTP/2 with prior
/// knowledge — matching the protocol the data plane will use for real
/// traffic. The metadata transport may be HTTP/1.1 and would fail
/// against the proxy.
pub(crate) struct Http2ConnectivityProbe {
    transport: Arc<dyn TransportClient>,
    timeout: Duration,
}

impl std::fmt::Debug for Http2ConnectivityProbe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Http2ConnectivityProbe")
            .field("timeout", &self.timeout)
            .finish_non_exhaustive()
    }
}

impl Http2ConnectivityProbe {
    pub(crate) fn new(transport: Arc<dyn TransportClient>) -> Self {
        Self {
            transport,
            timeout: DEFAULT_PROBE_TIMEOUT,
        }
    }

    /// Builds the probe URL: `{base}/connectivity-probe` preserving the
    /// scheme, host, and port advertised by the account topology response.
    /// The base URL must include the scheme + authority; any path/query on
    /// the base is overwritten so the probe always targets exactly
    /// `/connectivity-probe`.
    fn probe_url(base: &Url) -> Result<Url, url::ParseError> {
        let mut probe = base.clone();
        probe.set_path(CONNECTIVITY_PROBE_PATH);
        probe.set_query(None);
        probe.set_fragment(None);
        Ok(probe)
    }

    async fn probe_one(&self, url: Url) -> Result<(), ProbeFailure> {
        let request = HttpRequest {
            url,
            method: Method::Post,
            headers: Headers::new(),
            body: None,
            timeout: Some(self.timeout),
            #[cfg(feature = "fault_injection")]
            evaluation_collector: None,
        };

        let started = Instant::now();
        let result = self.transport.send(&request).await;
        let elapsed = started.elapsed();

        match result {
            Ok(HttpResponse { status: 200, .. }) => {
                tracing::debug!(
                    url = %request.url,
                    status = 200,
                    elapsed_ms = elapsed.as_millis() as u64,
                    "connectivity probe healthy"
                );
                Ok(())
            }
            Ok(HttpResponse { status, .. }) => {
                tracing::info!(
                    url = %request.url,
                    status,
                    elapsed_ms = elapsed.as_millis() as u64,
                    "connectivity probe returned non-200 status (gating Gateway 2.0 off)"
                );
                Err(ProbeFailure::Status(status))
            }
            Err(TransportError { error, .. }) => {
                let message = error.to_string();
                tracing::info!(
                    url = %request.url,
                    error = %message,
                    elapsed_ms = elapsed.as_millis() as u64,
                    "connectivity probe network failure (gating Gateway 2.0 off)"
                );
                Err(ProbeFailure::Network(message))
            }
        }
    }
}

#[async_trait::async_trait]
impl ConnectivityProbe for Http2ConnectivityProbe {
    async fn probe_endpoints(&self, endpoints: Vec<(Region, ProbeRole, Url)>) -> ProbeOutcome {
        if endpoints.is_empty() {
            // No advertised G2 endpoints — nothing to gate.
            return ProbeOutcome::AllHealthy;
        }

        // Deduplicate by URL so we send one POST per unique proxy endpoint,
        // even when the same URL appears in both readable and writable
        // location lists. Attribution is preserved so a failure can be
        // reported against every (region, role) pair that advertised the URL.
        let mut url_to_attribution: HashMap<Url, Vec<(Region, ProbeRole)>> = HashMap::new();
        let mut ordered_urls: Vec<Url> = Vec::new();
        for (region, role, base) in endpoints {
            let url = match Self::probe_url(&base) {
                Ok(u) => u,
                Err(e) => {
                    // Should not happen — base URL is a Url already.
                    return ProbeOutcome::Failed {
                        failures: vec![(region, ProbeFailure::Network(e.to_string()))],
                    };
                }
            };
            url_to_attribution
                .entry(url.clone())
                .or_insert_with(|| {
                    ordered_urls.push(url.clone());
                    Vec::new()
                })
                .push((region, role));
        }

        let probes = ordered_urls
            .iter()
            .cloned()
            .map(|url| async move { (url.clone(), self.probe_one(url).await) });

        let results = join_all(probes).await;

        let mut failures: Vec<(Region, ProbeFailure)> = Vec::new();
        for (url, outcome) in results {
            if let Err(failure) = outcome {
                let attributions = url_to_attribution.get(&url).cloned().unwrap_or_default();
                for (region, _role) in attributions {
                    failures.push((region, failure.clone()));
                }
            }
        }

        if failures.is_empty() {
            ProbeOutcome::AllHealthy
        } else {
            ProbeOutcome::Failed { failures }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::RequestSentStatus;
    use crate::error::{CosmosError, CosmosStatus};
    use async_trait::async_trait;
    use std::sync::Mutex;

    /// Mock TransportClient that returns canned responses keyed by URL host.
    #[derive(Debug)]
    struct MockTransport {
        responses: Mutex<HashMap<String, Result<u16, String>>>,
        calls: Mutex<Vec<Url>>,
    }

    impl MockTransport {
        fn new() -> Self {
            Self {
                responses: Mutex::new(HashMap::new()),
                calls: Mutex::new(Vec::new()),
            }
        }

        fn with_status(self, host: &str, status: u16) -> Self {
            self.responses
                .lock()
                .unwrap()
                .insert(host.to_owned(), Ok(status));
            self
        }

        fn with_network_error(self, host: &str, message: &str) -> Self {
            self.responses
                .lock()
                .unwrap()
                .insert(host.to_owned(), Err(message.to_owned()));
            self
        }

        fn call_count(&self) -> usize {
            self.calls.lock().unwrap().len()
        }

        fn called_urls(&self) -> Vec<Url> {
            self.calls.lock().unwrap().clone()
        }
    }

    #[async_trait]
    impl TransportClient for MockTransport {
        async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            self.calls.lock().unwrap().push(request.url.clone());
            let host = request.url.host_str().unwrap_or_default().to_owned();
            match self.responses.lock().unwrap().get(&host).cloned() {
                Some(Ok(status)) => Ok(HttpResponse {
                    status,
                    headers: Headers::new(),
                    body: Vec::new(),
                }),
                Some(Err(message)) => Err(TransportError::new(
                    CosmosError::builder()
                        .with_status(CosmosStatus::TRANSPORT_CONNECTION_FAILED)
                        .with_message(message)
                        .build(),
                    RequestSentStatus::NotSent,
                )),
                None => panic!("no response configured for host {host}"),
            }
        }
    }

    fn region(name: &'static str) -> Region {
        Region::from(name)
    }

    fn endpoint(host: &str) -> Url {
        Url::parse(&format!("https://{host}:444/")).unwrap()
    }

    #[tokio::test]
    async fn empty_endpoint_list_is_healthy() {
        let transport: Arc<dyn TransportClient> = Arc::new(MockTransport::new());
        let probe = Http2ConnectivityProbe::new(transport);
        let outcome = probe.probe_endpoints(Vec::new()).await;
        assert_eq!(outcome, ProbeOutcome::AllHealthy);
    }

    #[tokio::test]
    async fn all_200s_is_healthy() {
        let mock = Arc::new(
            MockTransport::new()
                .with_status("eastus-thin", 200)
                .with_status("westus-thin", 200),
        );
        let probe = Http2ConnectivityProbe::new(mock.clone());

        let outcome = probe
            .probe_endpoints(vec![
                (region("eastus"), ProbeRole::Write, endpoint("eastus-thin")),
                (region("westus"), ProbeRole::Read, endpoint("westus-thin")),
            ])
            .await;

        assert_eq!(outcome, ProbeOutcome::AllHealthy);
        assert_eq!(mock.call_count(), 2);
        for url in mock.called_urls() {
            assert_eq!(url.path(), CONNECTIVITY_PROBE_PATH);
            assert!(url.query().is_none());
        }
    }

    #[tokio::test]
    async fn strict_503_fails_the_probe() {
        // Per the cross-SDK decision: 503 (proxy reachable but
        // enableConnectivityProbe flag is OFF) is treated as failure so the
        // SDK stays on Gateway V1 until the federation opts in.
        let mock = Arc::new(
            MockTransport::new()
                .with_status("eastus-thin", 200)
                .with_status("westus-thin", 503),
        );
        let probe = Http2ConnectivityProbe::new(mock);

        let outcome = probe
            .probe_endpoints(vec![
                (region("eastus"), ProbeRole::Write, endpoint("eastus-thin")),
                (region("westus"), ProbeRole::Read, endpoint("westus-thin")),
            ])
            .await;

        match outcome {
            ProbeOutcome::Failed { failures } => {
                assert_eq!(failures.len(), 1);
                assert_eq!(failures[0].0, region("westus"));
                assert_eq!(failures[0].1, ProbeFailure::Status(503));
            }
            other => panic!("expected Failed, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn network_error_fails_the_probe() {
        let mock = Arc::new(
            MockTransport::new()
                .with_status("eastus-thin", 200)
                .with_network_error("westus-thin", "connection refused"),
        );
        let probe = Http2ConnectivityProbe::new(mock);

        let outcome = probe
            .probe_endpoints(vec![
                (region("eastus"), ProbeRole::Write, endpoint("eastus-thin")),
                (region("westus"), ProbeRole::Read, endpoint("westus-thin")),
            ])
            .await;

        match outcome {
            ProbeOutcome::Failed { failures } => {
                assert_eq!(failures.len(), 1);
                assert_eq!(failures[0].0, region("westus"));
                assert!(matches!(failures[0].1, ProbeFailure::Network(_)));
            }
            other => panic!("expected Failed, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn all_or_nothing_collapses_multiple_failures() {
        let mock = Arc::new(
            MockTransport::new()
                .with_status("eastus-thin", 500)
                .with_status("westus-thin", 503)
                .with_status("centralus-thin", 200),
        );
        let probe = Http2ConnectivityProbe::new(mock);

        let outcome = probe
            .probe_endpoints(vec![
                (region("eastus"), ProbeRole::Write, endpoint("eastus-thin")),
                (region("westus"), ProbeRole::Read, endpoint("westus-thin")),
                (
                    region("centralus"),
                    ProbeRole::Read,
                    endpoint("centralus-thin"),
                ),
            ])
            .await;

        match outcome {
            ProbeOutcome::Failed { failures } => {
                assert_eq!(failures.len(), 2);
                let mut by_region: HashMap<Region, ProbeFailure> = failures.into_iter().collect();
                assert_eq!(
                    by_region.remove(&region("eastus")),
                    Some(ProbeFailure::Status(500))
                );
                assert_eq!(
                    by_region.remove(&region("westus")),
                    Some(ProbeFailure::Status(503))
                );
            }
            other => panic!("expected Failed, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn dedupes_by_url_and_attributes_to_both_roles() {
        // The same URL is advertised for both read and write roles. The
        // mock asserts only one network call is made, but a failure is
        // attributed to both (region, role) pairs.
        let mock = Arc::new(MockTransport::new().with_status("eastus-thin", 503));
        let probe = Http2ConnectivityProbe::new(mock.clone());

        let url = endpoint("eastus-thin");
        let outcome = probe
            .probe_endpoints(vec![
                (region("eastus"), ProbeRole::Read, url.clone()),
                (region("eastus"), ProbeRole::Write, url),
            ])
            .await;

        // One network call was made.
        assert_eq!(mock.call_count(), 1);

        match outcome {
            ProbeOutcome::Failed { failures } => {
                // Two attribution entries — one per advertised role.
                assert_eq!(failures.len(), 2);
                for (_, failure) in failures {
                    assert_eq!(failure, ProbeFailure::Status(503));
                }
            }
            other => panic!("expected Failed, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn probe_url_overwrites_path_and_query() {
        // Service-advertised endpoints carry a trailing slash and no query,
        // but if a future contract change adds a path or query, the probe
        // path must take precedence so we always target the exact endpoint.
        let base =
            Url::parse("https://eastus-thin.documents.azure.com:444/garbage?foo=bar").unwrap();
        let probe = Http2ConnectivityProbe::probe_url(&base).unwrap();
        assert_eq!(probe.path(), CONNECTIVITY_PROBE_PATH);
        assert!(probe.query().is_none());
        assert_eq!(probe.host_str(), Some("eastus-thin.documents.azure.com"));
        assert_eq!(probe.port(), Some(444));
        assert_eq!(probe.scheme(), "https");
    }
}
