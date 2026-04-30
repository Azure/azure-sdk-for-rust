// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end tests for the Gateway 2.0 transport, exercised through the
//! `azure_data_cosmos` SDK surface (not the underlying driver crate).
//!
//! These tests run against a pre-provisioned Gateway 2.0 ("thin client")
//! account. The endpoint and primary key are read from the
//! `AZURE_COSMOS_GW20_ENDPOINT` and `AZURE_COSMOS_GW20_KEY` environment
//! variables and gated by the `gateway20` test category. They are skipped by
//! default; the dedicated `ci-gateway20.yml` pipeline sets the matrix entry's
//! `testCategory` to `gateway20` (or `gateway20_multi_region`) so the tests
//! run in CI against the live account.
//!
//! ## Current state
//!
//! Every test in this file is a placeholder stub:
//!
//! * [`CosmosClientOptions`](azure_data_cosmos::CosmosClientOptions) does not
//!   yet expose a public Gateway 2.0 enable/disable setter. Without it, the
//!   SDK cannot deterministically opt a client into Gateway 2.0 from outside
//!   the crate.
//! * The driver-level toggle
//!   (`ConnectionPoolOptions::with_is_gateway20_allowed`) is not wired through
//!   to `CosmosClientOptions`.
//!
//! Once the SDK exposes a public `with_gateway20_disabled` (or equivalent)
//! setter, fill in each test body. Until then, the bodies are intentionally
//! empty so the file compiles and the test names lock in the contract.

#![cfg(feature = "key_auth")]

fn read_env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

/// Returns `Some((endpoint, key))` only when both env vars are set.
fn live_credentials() -> Option<(String, String)> {
    Some((
        read_env("AZURE_COSMOS_GW20_ENDPOINT")?,
        read_env("AZURE_COSMOS_GW20_KEY")?,
    ))
}

/// Drives a point CRUD round-trip (create → read → replace → delete) against
/// the live Gateway 2.0 account.
///
/// TODO(Phase 6): implement once `CosmosClientOptions` exposes a Gateway 2.0
/// toggle. Build a client with that toggle enabled, drive the CRUD operations,
/// and assert each request reports `TransportKind::Gateway20` in diagnostics.
#[tokio::test]
#[cfg_attr(
    not(test_category = "gateway20"),
    ignore = "requires test_category 'gateway20' and AZURE_COSMOS_GW20_ENDPOINT/_KEY"
)]
pub async fn gateway20_point_crud_round_trip() {
    let Some((_endpoint, _key)) = live_credentials() else {
        return;
    };
    // TODO(Phase 6): build a CosmosClient with Gateway 2.0 enabled and run a
    // create/read/replace/delete cycle on a single item, asserting each
    // response surfaces `TransportKind::Gateway20` in diagnostics.
}

/// Runs a SQL query through Gateway 2.0 and asserts the streamed pages all
/// route through the thin-client transport.
///
/// TODO(Phase 6): implement once `CosmosClientOptions` exposes a Gateway 2.0
/// toggle. Open a query feed, iterate every page, and assert the diagnostics
/// for each page record `TransportKind::Gateway20`.
#[tokio::test]
#[cfg_attr(
    not(test_category = "gateway20"),
    ignore = "requires test_category 'gateway20' and AZURE_COSMOS_GW20_ENDPOINT/_KEY"
)]
pub async fn gateway20_query_streams_through_thin_client() {
    let Some((_endpoint, _key)) = live_credentials() else {
        return;
    };
    // TODO(Phase 6): drive a multi-page SELECT query and assert every page's
    // diagnostics report `TransportKind::Gateway20`.
}

/// Runs a transactional batch through Gateway 2.0.
///
/// TODO(Phase 6): implement once `CosmosClientOptions` exposes a Gateway 2.0
/// toggle. Submit a batch with mixed create/upsert/delete operations and
/// assert it routes through Gateway 2.0 end-to-end.
#[tokio::test]
#[cfg_attr(
    not(test_category = "gateway20"),
    ignore = "requires test_category 'gateway20' and AZURE_COSMOS_GW20_ENDPOINT/_KEY"
)]
pub async fn gateway20_transactional_batch() {
    let Some((_endpoint, _key)) = live_credentials() else {
        return;
    };
    // TODO(Phase 6): submit a mixed-op transactional batch and assert the
    // diagnostics report `TransportKind::Gateway20`.
}

/// Drives a `LatestVersion` change feed iterator through Gateway 2.0.
///
/// TODO(Phase 6): implement once `CosmosClientOptions` exposes a Gateway 2.0
/// toggle. Open a change feed iterator with `mode = LatestVersion` and assert
/// pages are served via Gateway 2.0.
#[tokio::test]
#[cfg_attr(
    not(test_category = "gateway20"),
    ignore = "requires test_category 'gateway20' and AZURE_COSMOS_GW20_ENDPOINT/_KEY"
)]
pub async fn gateway20_change_feed_latest_version() {
    let Some((_endpoint, _key)) = live_credentials() else {
        return;
    };
    // TODO(Phase 6): consume a LatestVersion change feed and assert the
    // diagnostics report `TransportKind::Gateway20`.
}

/// Verifies that `RequestDiagnostics` correctly reports
/// `TransportKind::Gateway20` for SDK-issued requests.
///
/// TODO(Phase 6): implement once `CosmosClientOptions` exposes a Gateway 2.0
/// toggle and the SDK surfaces `TransportKind` on its public diagnostics
/// type.
#[tokio::test]
#[cfg_attr(
    not(test_category = "gateway20"),
    ignore = "requires test_category 'gateway20' and AZURE_COSMOS_GW20_ENDPOINT/_KEY"
)]
pub async fn gateway20_diagnostics_validation() {
    let Some((_endpoint, _key)) = live_credentials() else {
        return;
    };
    // TODO(Phase 6): perform a single read against the live account and
    // assert the resulting diagnostics record `TransportKind::Gateway20`.
}

/// Verifies the operator override at the SDK boundary: when the operator
/// disables Gateway 2.0 via the public client option, every request must
/// route through the standard gateway even though the account advertises a
/// thin-client endpoint.
///
/// TODO(Phase 6): implement once `CosmosClientOptions` exposes a public
/// Gateway 2.0 toggle. Build a client with the toggle disabled, drive a
/// point read, and assert diagnostics report `TransportKind::StandardGateway`.
#[tokio::test]
#[cfg_attr(
    not(test_category = "gateway20"),
    ignore = "requires test_category 'gateway20' and AZURE_COSMOS_GW20_ENDPOINT/_KEY"
)]
pub async fn gateway20_operator_override_at_sdk_boundary() {
    let Some((_endpoint, _key)) = live_credentials() else {
        return;
    };
    // TODO(Phase 6): build a CosmosClient with Gateway 2.0 explicitly
    // disabled, drive a point read, and assert the diagnostics report
    // `TransportKind::StandardGateway`.
}
