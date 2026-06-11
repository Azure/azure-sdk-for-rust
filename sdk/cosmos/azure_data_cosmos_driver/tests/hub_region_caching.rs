// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration test for per-partition **hub-region caching** on a
//! PPAF-enabled single-master account.
//!
//! Mirrors the .NET coverage from PR #5648 (`CosmosItemIntegrationTests
//! .ReadItemAsync_HubRegionCaching_DiscoveryThenCacheHit_LiveAccount`)
//! within the constraints of the 2-region Rust live-test matrix (`East US 2`
//! hub + `West US 3` read region; see `sdk/cosmos/test-resources.bicep`).
//!
//! # What this test proves
//!
//! Two sequential reads of the same item:
//!
//! * **R1 (cold cache)** — exercises the full hub-region discovery
//!   pipeline. With `preferred_regions = [WUS3, EUS2]` and an injected
//!   `404/1002` on WUS3 plus an injected `403/3` on EUS2, the operation
//!   pipeline must (a) flip the hub-region-processing-only latch on the
//!   `1002`, (b) attach the `x-ms-cosmos-hub-region-processing-only`
//!   request header on every subsequent attempt, (c) consume the injected
//!   `403/3` on the first hub probe, and (d) drive the per-partition
//!   discovery rotation until it converges on the real hub (EUS2). On
//!   success the routing system writes a single entry into
//!   `PartitionEndpointState::failover_overrides` whose
//!   `current_endpoint` points at the EUS2 hub.
//!
//! * **R2 (warm cache)** — re-reads the same item. The cache entry from
//!   R1 persists, so once the second `1002` re-latches the hub gate, the
//!   cache lookup at `operation_pipeline.rs:1128-1135` short-circuits the
//!   discovery probe. The wire signature is just `WUS3 (1002) → EUS2`,
//!   the cache snapshot is unchanged, and the `403/3` rule does NOT fire
//!   a second time.
//!
//! # The 5-hop cold trace
//!
//! With `preferred_regions = [WUS3, EUS2]`, single-master account
//! (`preferred_write_endpoints = [EUS2]`), `max_failover_retries = 3` and
//! the two fault rules in this file:
//!
//! 1. `WUS3` — initial attempt, no hub header. Injected `404/1002` →
//!    `SessionRetry`. `build_session_retry_state` flips the latch ON and
//!    sets routing = `PreferredWriteEndpoints` (see
//!    `retry_evaluation.rs:517`).
//! 2. `EUS2` — routed via `PreferredWriteEndpoints[0]`; the hub-region
//!    header is now attached (`apply_hub_region_header`,
//!    `operation_pipeline.rs:1549`). Injected `403/3` →
//!    `AdvanceHubRegionDiscovery{failed=EUS2}` + `FailoverRetry`. Buffered
//!    effect creates the cache entry `{current=WUS3, failed={EUS2}}`.
//! 3. `WUS3` — routed via cache HIT (entry.current). Hub header set.
//!    Live backend rejects with **real** `403/3` because the request
//!    targets a non-hub region with the hub-only header set. New effect
//!    rotates the entry, but with `next_endpoints = [WUS3, EUS2]`,
//!    `WUS3 == current` and `EUS2 ∈ failed`, so
//!    `try_move_next_endpoint` returns `false` and the entry is
//!    **removed** (see `routing_systems.rs:465-489`).
//! 4. `WUS3` — cache MISS; falls back to default routing
//!    `preferred_read[0] = WUS3` again. Hub header set. **Real** `403/3`
//!    again. The buffered effect re-creates the entry via `or_insert_with`,
//!    inserts `WUS3` into `failed_endpoints`, and the next-endpoint walk
//!    now picks `EUS2` (the only non-failed candidate) — final state
//!    `{current=EUS2, failed={WUS3}}`.
//! 5. `EUS2` — cache HIT (entry.current). Hub header set. Live backend
//!    returns **real** `200 OK`. The success path emits
//!    `CacheHubRegion{endpoint=EUS2}` which is a no-op against the
//!    already-EUS2 entry (`cache_hub_region` at
//!    `routing_systems.rs:508-548` preserves `failed_endpoints`).
//!
//! Net cold flow: **5 wire attempts**, both fault rules each fired once,
//! one cache entry pointing at EUS2.
//!
//! # The 2-hop warm trace
//!
//! 1. `WUS3` — fresh operation, latch starts OFF (cache gate skipped).
//!    Routing default → `WUS3`. Injected `404/1002` (rule's 2nd hit) →
//!    `SessionRetry` re-latches.
//! 2. `EUS2` — latch ON, cache HIT against the entry from R1 routes
//!    directly to `current=EUS2`. Real `200 OK`. The `403/3` rule does
//!    not fire (cache short-circuited the discovery probe). Cache entry
//!    is refreshed in place (no change).
//!
//! # How the test proves the hub-region header is attached
//!
//! Direct unit-level coverage of the header emission lives in
//! `operation_pipeline.rs` (tests `T-S11`
//! `apply_hub_region_header_emits_when_only_shared_latch_set` and `T-S12`
//! `apply_hub_region_header_omits_when_shared_latch_present_but_false`).
//! The integration test contributes **end-to-end evidence**: the cold
//! trace can only converge on EUS2 in the cache if WUS3 returns a real
//! `403/3` on attempts 3 and 4. Live Cosmos backends only reject a read
//! with `403/3` on a non-hub region when the
//! `x-ms-cosmos-hub-region-processing-only: True` header is present —
//! without the header, those attempts would succeed with `200 OK` and the
//! cache would converge on the wrong region (or fail to converge at all).
//!
//! Note: on 2 regions the `PreferredWriteEndpoints` fallback path
//! naturally routes to EUS2 on R2 attempt 1 even without a cache hit
//! (since `preferred_write_endpoints = [EUS2]`), so R2's wire signature
//! alone is not unique evidence of cache utilization. The strong cache
//! signal in this test is the **persisted cache snapshot** between R1 and
//! R2 plus the cold-flow convergence on EUS2.
//!
//! # Gating
//!
//! - `#[cfg(feature = "fault_injection")]` — fault injection is opt-in.
//! - `#[cfg_attr(not(test_category = "multi_region"), ignore = ...)]` — only
//!   runs in the `Session SingleWrite MultiRegion PartitionFailover` live
//!   matrix entry (per `sdk/cosmos/live-platform-matrix.json`).
//! - `#[ignore = "Requires PPAF enabled account with active failover from
//!   backend"]` — must be invoked explicitly with `--ignored` against a
//!   PPAF-enabled account.
//!
//! Run with:
//!
//! ```text
//! cargo test -p azure_data_cosmos_driver \
//!   --features "fault_injection,__internal_testing" \
//!   --test hub_region_caching -- --ignored --nocapture
//! ```

#![cfg(feature = "fault_injection")]
#![cfg(feature = "__internal_testing")]

use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos_driver::options::{OperationOptions, Region};
use std::error::Error;
use std::sync::Arc;

// Framework module shared across test binaries — only a subset of its
// exports are used here.
#[allow(dead_code, unused_imports)]
mod framework;

use framework::DriverTestClient;

/// Account hub / write region for the 2-region live-test account
/// (`East US 2`, failover priority 0 in `test-resources.bicep`).
const HUB_REGION: Region = Region::EAST_US_2;

/// Preferred read region for the 2-region live-test account (`West US 3`,
/// failover priority 1).
const PREFERRED_READ_REGION: Region = Region::WEST_US_3;

/// Host fragment expected in the cached hub-region endpoint URL.
///
/// The SDK builds regional gateway URLs of the form
/// `https://<account>-<region-slug>.documents.azure.com:443/` (see
/// `account_metadata_cache.rs`), where the region slug is the lowercased
/// `Region` short name. For [`Region::EAST_US_2`] that is `eastus2`.
const HUB_REGION_HOST_FRAGMENT: &str = "eastus2";

/// End-to-end coverage of per-partition hub-region caching against a
/// PPAF-enabled single-master account.
///
/// Performs two sequential reads of the same item with
/// `preferred_regions = [WEST_US_3, EAST_US_2]` and two fault rules:
///
/// * `wus3-1002` on `WEST_US_3` + `ReadItem` + `ReadSessionNotAvailable`
///   (`404/1002`), `hit_limit = 2` — fires on both R1 and R2 to flip the
///   hub-region-processing-only latch.
/// * `eus2-403-3` on `EAST_US_2` + `ReadItem` + `WriteForbidden`
///   (`403/3`), `hit_limit = 1` — fires only on R1 to drive the first
///   hub-discovery rotation away from EUS2.
///
/// Asserts (cold path, R1):
///   1. The `1002` rule fired exactly once.
///   2. The `403/3` rule fired exactly once.
///   3. The read issued at least 4 wire attempts (5 in the typical
///      deterministic trace — see the module-level docs for the full
///      hop-by-hop derivation).
///   4. Both regions appear in the contacted set.
///   5. After R1, `PartitionEndpointState::failover_overrides` contains
///      exactly one entry whose endpoint URL points at EUS2.
///
/// Asserts (warm path, R2):
///   6. The `1002` rule fired a second time (`hit_count == 2`).
///   7. The `403/3` rule was NOT consumed again (`hit_count == 1`),
///      proving the cache HIT short-circuited the discovery probe.
///   8. The read issued exactly 2 wire attempts.
///   9. The cache snapshot is byte-for-byte identical to R1's final
///      snapshot.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_region"),
    ignore = "requires test_category 'multi_region'"
)]
#[cfg_attr(
    test_category = "multi_region",
    ignore = "Requires PPAF enabled account with active failover from backend"
)]
pub async fn read_hub_region_caching_cold_then_warm() -> Result<(), Box<dyn Error>> {
    // 1002 on the preferred read region — fires twice: once on R1's
    // initial attempt to set the latch and start discovery, once on R2's
    // initial attempt to re-latch so the cache lookup gate is active.
    let rule_1002 = Arc::new(
        FaultInjectionRuleBuilder::new(
            "hub-cache-wus3-1002",
            FaultInjectionResultBuilder::new()
                .with_error(FaultInjectionErrorType::ReadSessionNotAvailable)
                .with_probability(1.0)
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::ReadItem)
                .with_region(PREFERRED_READ_REGION)
                .build(),
        )
        .with_hit_limit(2)
        .build(),
    );

    // 403/3 on the hub region — fires once on R1's first hub probe to
    // force the discovery rotation. After R1 populates the cache, R2
    // short-circuits via the cache hit and this rule never fires again,
    // which is exactly the cache-utilization signal we assert below.
    let rule_403 = Arc::new(
        FaultInjectionRuleBuilder::new(
            "hub-cache-eus2-403-3",
            FaultInjectionResultBuilder::new()
                .with_error(FaultInjectionErrorType::WriteForbidden)
                .with_probability(1.0)
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::ReadItem)
                .with_region(HUB_REGION)
                .build(),
        )
        .with_hit_limit(1)
        .build(),
    );

    let rules = vec![Arc::clone(&rule_1002), Arc::clone(&rule_403)];

    DriverTestClient::run_with_unique_db_and_hedging(
        rules,
        OperationOptions::default(),
        // Explicit preferred-regions list — pre-warms the driver cache so
        // initial reads target WEST_US_3 (where the 1002 injection lives)
        // rather than EUS2 (the account default first preference).
        vec![PREFERRED_READ_REGION, HUB_REGION],
        async |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            // Seed an item to read.
            let item_body = br#"{"id": "hub-cache-item", "pk": "pk1", "value": "seed"}"#;
            context
                .create_item_with_pk(&container, "pk1", item_body)
                .await
                .expect("seed CreateItem should succeed");

            // Cache must start empty.
            let cache_before = context.hub_region_cache_snapshot().await?;
            assert!(
                cache_before.is_empty(),
                "Hub-region cache should be empty before any read, got {:?}",
                cache_before
            );

            // ── R1 (cold cache) ─────────────────────────────────────────
            //
            // The hub-region discovery pipeline drives the full rotation:
            // WUS3 (1002) → EUS2 (injected 403/3) → WUS3 (real 403/3) →
            // WUS3 (real 403/3) → EUS2 (real 200). See the module-level
            // docs for the per-hop derivation.
            let r1 = context
                .read_item(&container, "hub-cache-item", "pk1")
                .await
                .expect("R1 (cold) ReadItem should succeed via hub-region discovery");

            assert_eq!(
                rule_1002.hit_count(),
                1,
                "R1: WUS3 1002 rule should fire exactly once, got {}",
                rule_1002.hit_count()
            );
            assert_eq!(
                rule_403.hit_count(),
                1,
                "R1: EUS2 403/3 rule should fire exactly once, got {}",
                rule_403.hit_count()
            );

            let r1_diag = r1.diagnostics();
            let r1_count = r1_diag.request_count();
            assert!(
                r1_count >= 4,
                "R1 cold path should issue at least 4 wire attempts \
                 (deterministic trace is 5: WUS3 1002, EUS2 403/3-inject, \
                 WUS3 real-403/3, WUS3 real-403/3, EUS2 200), got {}",
                r1_count
            );
            let r1_regions = r1_diag.regions_contacted();
            assert!(
                r1_regions.iter().any(|r| *r == PREFERRED_READ_REGION),
                "R1: regions_contacted should include {:?}, got {:?}",
                PREFERRED_READ_REGION,
                r1_regions
            );
            assert!(
                r1_regions.iter().any(|r| *r == HUB_REGION),
                "R1: regions_contacted should include hub {:?}, got {:?}",
                HUB_REGION,
                r1_regions
            );

            // Cache populated — exactly one partition entry pointing at
            // the hub. This is the strongest single-test signal that
            // (a) the discovery converged, AND (b) the hub-region header
            // was attached on retries: without the header, WUS3 would
            // have returned 200 OK on attempts 3/4 and the cache would
            // have ended up pointing at WUS3 instead of EUS2.
            let cache_after_r1 = context.hub_region_cache_snapshot().await?;
            assert_eq!(
                cache_after_r1.len(),
                1,
                "R1: hub-region cache should contain exactly 1 entry, got {:?}",
                cache_after_r1
            );
            let (pk_range_id, endpoint_url) = &cache_after_r1[0];
            assert!(
                !pk_range_id.is_empty(),
                "R1: cached pk_range_id should not be empty"
            );
            assert!(
                endpoint_url.to_lowercase().contains(HUB_REGION_HOST_FRAGMENT),
                "R1: cached endpoint URL {endpoint_url:?} should point at the \
                 hub region (host fragment {HUB_REGION_HOST_FRAGMENT:?})"
            );

            // ── R2 (warm cache) ─────────────────────────────────────────
            //
            // Same item, fresh operation. Latch starts OFF so the initial
            // attempt targets WUS3 (which hits the 2nd 1002), re-latches,
            // and then the cache lookup gate routes attempt 2 directly to
            // the cached EUS2. The 403/3 rule never fires.
            let r2 = context
                .read_item(&container, "hub-cache-item", "pk1")
                .await
                .expect("R2 (warm) ReadItem should succeed via cache short-circuit");

            assert_eq!(
                rule_1002.hit_count(),
                2,
                "R2: WUS3 1002 rule should fire a second time, got {}",
                rule_1002.hit_count()
            );
            assert_eq!(
                rule_403.hit_count(),
                1,
                "R2: EUS2 403/3 rule should NOT have fired again \
                 (cache HIT short-circuits the discovery probe), got {}",
                rule_403.hit_count()
            );

            let r2_diag = r2.diagnostics();
            let r2_count = r2_diag.request_count();
            assert_eq!(
                r2_count, 2,
                "R2 warm path should issue exactly 2 wire attempts \
                 (WUS3 1002 → EUS2 cache-hit success), got {}",
                r2_count
            );
            let r2_regions = r2_diag.regions_contacted();
            assert!(
                r2_regions.iter().any(|r| *r == PREFERRED_READ_REGION),
                "R2: regions_contacted should include {:?}, got {:?}",
                PREFERRED_READ_REGION,
                r2_regions
            );
            assert!(
                r2_regions.iter().any(|r| *r == HUB_REGION),
                "R2: regions_contacted should include hub {:?}, got {:?}",
                HUB_REGION,
                r2_regions
            );

            // Cache snapshot is byte-for-byte unchanged — the warm-path
            // success only refreshed the entry's `current_endpoint` to
            // the same EUS2 value (`cache_hub_region` is idempotent and
            // preserves `failed_endpoints`).
            let cache_after_r2 = context.hub_region_cache_snapshot().await?;
            assert_eq!(
                cache_after_r2, cache_after_r1,
                "R2: hub-region cache snapshot should be unchanged from R1"
            );

            Ok(())
        },
    )
    .await
}
