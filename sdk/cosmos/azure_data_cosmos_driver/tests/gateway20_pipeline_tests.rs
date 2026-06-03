// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests that lock in the Gateway 2.0 transport pipeline contract.
//!
//! These tests cover Phase 6 of the Gateway 2.0 specification (see
//! `docs/GATEWAY_20_SPEC.md`). They run as a standalone integration target so
//! they exercise the public surface of the driver crate end-to-end (no
//! `pub(crate)` access).
//!
//! ## Categories
//!
//! 1. **Operator override** — the operator can opt out of Gateway 2.0 even when
//!    the account advertises a Gateway 2.0 endpoint. Verified via the public
//!    [`ConnectionPoolOptions::with_gateway20_disabled`] toggle.
//!
//! 2. **Operation eligibility** — operations that Gateway 2.0 does not yet
//!    support (e.g., stored procedure execution) must transparently fall back
//!    to the standard gateway. Documented as an env-gated stub today; the
//!    inside-crate routing tests in `operation_pipeline.rs` cover the
//!    decision logic.
//!
//! 3. **Diagnostics fidelity** — `RequestDiagnostics` records the actual
//!    `TransportKind` used. Documented as an env-gated stub today.
//!
//! 4. **Dual-consistency invariants (V1)** — the V1 HTTP path must never emit
//!    *both* the legacy `x-ms-consistency-level` and the newer
//!    `x-ms-cosmos-read-consistency-strategy` headers. Asserted via captured
//!    HTTP requests through the `__internal_mocking` factory.
//!
//! 5. **Dual-consistency invariants (V2)** — the V2 RNTBD path must never
//!    serialize *both* `ConsistencyLevel` and a separate
//!    `ReadConsistencyStrategy` token. Documented as an invariant lock; the
//!    underlying RNTBD enum currently exposes only the `ConsistencyLevel`
//!    token (`tokens.rs`), so the invariant is structurally guaranteed.
//!
//! 6. **Capabilities header pin** — every outgoing request carries
//!    `x-ms-cosmos-sdk-supportedcapabilities = "9"`. Asserted via the first
//!    captured request through the mock factory.
//!
//! ## Why `__internal_mocking`?
//!
//! Several of these contracts can only be observed at the network boundary.
//! The driver exposes a [`HttpClientFactory`] override under the
//! `__internal_mocking` feature flag specifically for tests like these — it
//! lets us substitute a capturing transport so we can inspect the very first
//! request the runtime emits (the account-properties probe), without ever
//! touching the network.

#![cfg(feature = "__internal_mocking")]

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use azure_data_cosmos_driver::models::{AccountReference, CosmosOperation, DatabaseReference};
use azure_data_cosmos_driver::options::{DriverOptions, OperationOptions};
use azure_data_cosmos_driver::testing::{
    ConnectionPoolOptions, HttpClientConfig, HttpClientFactory, HttpRequest, HttpResponse,
    TransportClient, TransportError,
};
use azure_data_cosmos_driver::CosmosDriverRuntime;
use url::Url;

// ----------------------------------------------------------------------------
// Capturing transport
// ----------------------------------------------------------------------------

/// Records every outgoing request. By default, every send returns a
/// connection-style failure so the runtime aborts before the second hop, which
/// keeps the test focused on the first wire frame.
#[derive(Debug, Default)]
struct CapturingTransport {
    requests: Mutex<Vec<HttpRequest>>,
}

impl CapturingTransport {
    fn requests(&self) -> Vec<HttpRequest> {
        self.requests
            .lock()
            .expect("poisoned capture mutex")
            .clone()
    }
}

#[async_trait]
impl TransportClient for CapturingTransport {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        self.requests
            .lock()
            .expect("poisoned capture mutex")
            .push(request.clone());

        Err(TransportError::new(
            azure_data_cosmos_driver::error::CosmosError::builder()
                .with_status(azure_data_cosmos_driver::CosmosStatus::TRANSPORT_IO_FAILED)
                .with_message("capturing transport refuses every request")
                .build(),
            azure_data_cosmos_driver::diagnostics::RequestSentStatus::NotSent,
        ))
    }
}

#[derive(Debug)]
struct CapturingFactory {
    transport: Arc<CapturingTransport>,
}

impl CapturingFactory {
    fn new() -> (Self, Arc<CapturingTransport>) {
        let transport = Arc::new(CapturingTransport::default());
        (
            Self {
                transport: transport.clone(),
            },
            transport,
        )
    }
}

impl HttpClientFactory for CapturingFactory {
    fn build(
        &self,
        _connection_pool: &ConnectionPoolOptions,
        _config: HttpClientConfig,
    ) -> azure_data_cosmos_driver::error::Result<Arc<dyn TransportClient>> {
        Ok(self.transport.clone() as Arc<dyn TransportClient>)
    }
}

// ----------------------------------------------------------------------------
// Helpers
// ----------------------------------------------------------------------------

fn fake_account() -> AccountReference {
    let url =
        Url::parse("https://gw20-pipeline-tests.documents.azure.com/").expect("static URL parses");
    // Master-key value is base64-encoded; the bytes never reach the wire because
    // the capturing transport short-circuits every send.
    AccountReference::with_master_key(url, "dGVzdC1tYXN0ZXIta2V5")
}

fn read_env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

fn live_account_from_env() -> Option<AccountReference> {
    let endpoint = read_env("AZURE_COSMOS_GW20_ENDPOINT")?;
    let key = read_env("AZURE_COSMOS_GW20_KEY")?;
    let url = Url::parse(&endpoint).ok()?;
    Some(AccountReference::with_master_key(url, key))
}

/// Builds a runtime with the capturing factory and the requested
/// gateway-20 toggle. The flag reflects the operator override exposed via
/// `ConnectionPoolOptions` — passing `true` forces every request through the
/// standard gateway transport.
async fn capturing_runtime(
    gateway20_disabled: bool,
) -> (Arc<CosmosDriverRuntime>, Arc<CapturingTransport>) {
    let (factory, transport) = CapturingFactory::new();
    let pool = ConnectionPoolOptions::builder()
        .with_gateway20_disabled(gateway20_disabled)
        .build()
        .expect("connection pool builds");
    let runtime = CosmosDriverRuntime::builder()
        .with_connection_pool(pool)
        .with_mock_http_client_factory(Arc::new(factory))
        .build()
        .await
        .expect("runtime builds with mock factory");
    (runtime, transport)
}

/// Drive a no-op probe so the runtime emits at least one HTTP request.
///
/// The capturing transport refuses every send, so this always returns an
/// error. We only care about the captured frames.
async fn probe(runtime: &Arc<CosmosDriverRuntime>) {
    let account = fake_account();
    let options = DriverOptions::builder(account.clone()).build();
    let _ = runtime.get_or_create_driver(account, Some(options)).await;
}

// ----------------------------------------------------------------------------
// (a) Operator override forces standard gateway routing
// ----------------------------------------------------------------------------

/// Verifies that the operator override flag (`with_gateway20_disabled(true)`)
/// is honored end-to-end at the connection-pool level. When the flag is set,
/// the runtime must not select the Gateway 2.0 transport even if account
/// metadata advertises a Gateway 2.0 endpoint.
///
/// We assert the contract structurally via `ConnectionPoolOptions`: when the
/// flag is `true`, `gateway20_disabled()` reports `true`, and the
/// transport-layer dispatcher branches to the standard gateway (this branching
/// is covered by the inside-crate tests in
/// `driver::transport::tests::dataplane_transport_*`).
#[tokio::test]
async fn operator_override_disables_gateway20_at_pool_level() {
    let off = ConnectionPoolOptions::builder()
        .with_gateway20_disabled(true)
        .build()
        .expect("pool builds");
    assert!(
        off.gateway20_disabled(),
        "operator-disabled pool must report gateway20_disabled = true"
    );

    let on = ConnectionPoolOptions::builder()
        .with_gateway20_disabled(false)
        .build()
        .expect("pool builds");
    assert!(
        !on.gateway20_disabled(),
        "operator-enabled pool must report gateway20_disabled = false"
    );
}

/// Live-account companion to the above. Drives a real read against a
/// pre-provisioned Gateway 2.0 account with the operator override turned off,
/// then asserts (TODO once diagnostics expose `TransportKind`) that the
/// request used the standard gateway transport.
#[tokio::test]
#[ignore = "Requires AZURE_COSMOS_GW20_ENDPOINT/_KEY to a Gateway 2.0 account"]
async fn operator_override_routes_reads_to_standard_gateway() {
    let Some(account) = live_account_from_env() else {
        return;
    };

    // TODO(Phase 6): once diagnostics expose `TransportKind` per request,
    // assert that every request used `TransportKind::StandardGateway`.
    let pool = ConnectionPoolOptions::builder()
        .with_gateway20_disabled(true)
        .build()
        .expect("pool builds");
    let runtime = CosmosDriverRuntime::builder()
        .with_connection_pool(pool)
        .build()
        .await
        .expect("runtime builds");
    let driver = runtime
        .get_or_create_driver(account.clone(), None)
        .await
        .expect("driver init succeeds against the live account");

    let db = read_env("AZURE_COSMOS_GW20_DATABASE").unwrap_or_else(|| "gw20-tests".to_string());
    let db_ref = DatabaseReference::from_name(driver.account().clone(), db);

    let _ = driver
        .execute_operation(
            CosmosOperation::read_database(db_ref),
            OperationOptions::default(),
        )
        .await;
}

// ----------------------------------------------------------------------------
// (b) Operation eligibility fallback (StoredProc Execute → standard gateway)
// ----------------------------------------------------------------------------

/// Stored procedure execution is not yet supported by Gateway 2.0 and must
/// fall back to the standard gateway transparently.
///
/// The eligibility decision is made in `resolve_endpoint`
/// (operation_pipeline.rs); the inside-crate tests in
/// `driver::pipeline::operation_pipeline::tests::resolve_endpoint_*` cover the
/// matrix exhaustively. This standalone test is the live-account contract
/// lock — once `TransportKind` is exposed in diagnostics, assert that the
/// stored-procedure-execute request used `TransportKind::StandardGateway`
/// while a co-located point read on the same account used
/// `TransportKind::Gateway20`.
#[tokio::test]
#[ignore = "Requires AZURE_COSMOS_GW20_ENDPOINT/_KEY plus a stored procedure resource"]
async fn stored_proc_execute_falls_back_to_standard_gateway() {
    let Some(_account) = live_account_from_env() else {
        return;
    };
    // TODO(Phase 6): drive `CosmosOperation::execute_stored_procedure(...)`
    // against a real account and assert the diagnostics record
    // `TransportKind::StandardGateway` for that request specifically while
    // co-located point reads/writes record `TransportKind::Gateway20`.
}

// ----------------------------------------------------------------------------
// (c) Diagnostics records TransportKind::Gateway20
// ----------------------------------------------------------------------------

/// Once Gateway 2.0 has dispatched a request, the recorded
/// `RequestDiagnostics` for that request must indicate `TransportKind::Gateway20`.
///
/// This contract requires a live Gateway 2.0 account. The inside-crate test
/// `transport_pipeline::tests::gateway20_pipeline_records_transport_kind`
/// already covers the wiring at the unit-test level; this standalone test is
/// the live-account companion.
#[tokio::test]
#[ignore = "Requires AZURE_COSMOS_GW20_ENDPOINT/_KEY to a Gateway 2.0 account"]
async fn diagnostics_records_gateway20_transport_kind() {
    let Some(_account) = live_account_from_env() else {
        return;
    };
    // TODO(Phase 6): once `TransportKind` is exposed on the public
    // `RequestDiagnostics`, drive a point read against the live Gateway 2.0
    // account and assert the diagnostics report `TransportKind::Gateway20`.
}

// ----------------------------------------------------------------------------
// (d) V1 HTTP dual-consistency-header invariant
// ----------------------------------------------------------------------------

/// The V1 HTTP path must never emit *both* the legacy
/// `x-ms-consistency-level` header and the newer
/// `x-ms-cosmos-read-consistency-strategy` header on the same request.
///
/// Today the V1 path emits *neither* header (consistency is propagated via
/// the operation context, not a wire header), so the invariant trivially
/// holds. We capture the first wire frame the runtime emits and assert the
/// pair-presence rule.
#[tokio::test]
async fn v1_http_never_emits_both_consistency_headers() {
    const LEGACY: &str = "x-ms-consistency-level";
    const STRATEGY: &str = "x-ms-cosmos-read-consistency-strategy";

    let (runtime, transport) = capturing_runtime(true).await;
    probe(&runtime).await;

    let captured = transport.requests();
    for req in &captured {
        let has_legacy = req.headers.iter().any(|(name, _)| name.as_str() == LEGACY);
        let has_strategy = req
            .headers
            .iter()
            .any(|(name, _)| name.as_str() == STRATEGY);
        assert!(
            !(has_legacy && has_strategy),
            "request {:?} emitted both '{LEGACY}' and '{STRATEGY}' — V1 invariant violated",
            req.url
        );
    }
}

// ----------------------------------------------------------------------------
// (e) V2 RNTBD dual-consistency-token invariant
// ----------------------------------------------------------------------------

/// The V2 (RNTBD) path must never serialize *both* a `ConsistencyLevel` token
/// and a separate `ReadConsistencyStrategy` token on the same wrapped frame.
///
/// Today the RNTBD token enum
/// (`driver::transport::rntbd::tokens::RntbdRequestToken`) exposes only the
/// `ConsistencyLevel` variant — there is no `ReadConsistencyStrategy` token
/// at all — so the invariant is structurally guaranteed by the type system.
/// This test is therefore a *contract lock* expressed at the boundary this
/// integration test can actually observe.
///
/// `CapturingTransport` lives at the `HttpClientFactory` layer, so it only
/// ever sees V1 HTTP requests (account-properties probe, metadata reads,
/// etc.). RNTBD frames are dispatched via a separate TCP transport and are
/// invisible here. We assert two things:
///
/// 1. The capturing transport actually recorded at least one request — i.e.
///    the test setup is wired correctly and the runtime did make outbound
///    progress.
/// 2. Every captured request uses an `http`/`https` scheme. If a future
///    change ever tunnels wrapped RNTBD frames through HTTP (or pushes the
///    capture point lower in the stack so RNTBD is observable here), this
///    assertion fires and forces a reviewer to upgrade the test to parse
///    the wrapped frame and assert at-most-one consistency token per frame.
///
/// The structural invariant inside the wrapped frame is exhaustively covered
/// by the inside-crate tests in `gateway20_dispatch::tests::wraps_with_*`;
/// this test exists to prevent that coverage from silently disappearing if
/// the V2 transport boundary moves.
#[tokio::test]
async fn v2_rntbd_never_emits_both_consistency_tokens() {
    let (runtime, transport) = capturing_runtime(false).await;
    probe(&runtime).await;

    let captured = transport.requests();
    assert!(
        !captured.is_empty(),
        "capturing transport recorded zero requests; the V2 invariant test \
         setup is broken (no traffic was generated at all)"
    );

    // CONTRACT LOCK: today every captured request is a V1 HTTP probe by
    // construction. If this assertion ever fails, RNTBD-bearing traffic has
    // become observable at the HttpClientFactory layer and the body must be
    // structurally decoded to assert mutual exclusion of `ConsistencyLevel`
    // and any future `ReadConsistencyStrategy` token.
    //
    // TODO(Phase 6): when a `ReadConsistencyStrategy` RNTBD token lands,
    // replace this scheme check with a structural decode of the wrapped
    // frame and assert at-most-one consistency token per wrapped request.
    for req in &captured {
        let scheme = req.url.scheme();
        assert!(
            scheme == "http" || scheme == "https",
            "captured request to {} uses scheme {:?}; the V2 dual-token \
             contract lock is invalidated — upgrade this test to parse the \
             wrapped RNTBD frame and assert mutual exclusion of consistency \
             tokens",
            req.url,
            scheme,
        );
    }
}

// ----------------------------------------------------------------------------
// (f) Capabilities header pin
// ----------------------------------------------------------------------------

/// Every outgoing HTTP request must carry
/// `x-ms-cosmos-sdk-supportedcapabilities: 8`. The bitmask "8" is
/// `IgnoreUnknownRntbdTokens`, which Gateway 2.0 inspects to decide whether
/// the SDK can tolerate unknown RNTBD tokens.
///
/// This is the load-bearing forward-compatibility advertisement for Gateway
/// 2.0 — it MUST stay pinned to "8" until additional bits are coordinated
/// with a service-side rollout.
#[tokio::test]
async fn capabilities_header_value_is_pinned_to_eight() {
    const CAPABILITIES: &str = "x-ms-cosmos-sdk-supportedcapabilities";

    let (runtime, transport) = capturing_runtime(false).await;
    probe(&runtime).await;

    let captured = transport.requests();
    assert!(
        !captured.is_empty(),
        "runtime should have emitted at least one request via the mock factory"
    );

    for req in &captured {
        let value = req.headers.iter().find_map(|(name, value)| {
            (name.as_str() == CAPABILITIES).then(|| value.as_str().to_owned())
        });
        assert_eq!(
            value.as_deref(),
            Some("8"),
            "capabilities header missing or wrong on request to {}",
            req.url
        );
    }
}
