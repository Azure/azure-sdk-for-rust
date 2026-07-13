// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration coverage for per-partition hub-region caching via HTTP-transport
//! fault injection.
//!
//! Targets a real, multi-region, single-master Cosmos DB account. Gated by
//! `test_category = "multi_region"` (set via RUSTFLAGS by the
//! `Session SingleWrite MultiRegion PartitionFailover` live-test matrix entry
//! — see `sdk/cosmos/live-platform-matrix.json` and
//! `sdk/cosmos/test-resources.bicep`), so per-PR builds skip it entirely.
//!
//! The test does not require the live account to advertise PPAF natively;
//! it runs both reads through a single persistent driver and calls
//! `context.force_ppaf_enabled(&driver)` to set the in-memory partition flag
//! so the hub-region latch arms regardless of account capability. Sharing one
//! driver across both reads is what lets the warm (R2) read observe the cache
//! entry populated by the cold (R1) read. Inside the live leg the framework
//! reads `AZURE_COSMOS_CONNECTION_STRING` exported by the bicep template; if it
//! is unset the test skips cleanly via `DriverTestClient::run_with_*`.

#![cfg(feature = "fault_injection")]
#![cfg(feature = "__internal_testing")]

use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos_driver::options::{AvailabilityStrategy, OperationOptions, Region};
use std::error::Error;
use std::sync::Arc;

// Framework module shared across test binaries — only a subset of its
// exports are used here.
#[allow(dead_code, unused_imports)]
mod framework;

use framework::DriverTestClient;

/// Hub / write region of the live-test account (failover priority 0 in
/// `test-resources.bicep`).
const HUB_REGION: Region = Region::EAST_US_2;

/// Preferred read region of the live-test account (failover priority 1).
const PREFERRED_READ_REGION: Region = Region::WEST_US_3;

/// Host fragment expected in the cached hub-region endpoint URL. Regional
/// gateway URLs are of the form `https://<account>-<region-slug>.documents...`
/// where the slug is the lowercased short region name.
const HUB_REGION_HOST_FRAGMENT: &str = "eastus2";

/// Drives two sequential reads of the same item to exercise per-partition
/// hub-region caching: the cold (R1) path completes the hub-region discovery
/// rotation and writes a single cache entry pointing at the hub; the warm
/// (R2) path short-circuits the discovery probe via that cache entry.
///
/// Cold-path assertions (R1) prove the discovery converged and the
/// hub-region request header was attached on retries (without the header,
/// the non-hub region would not return real `403/3` and the cache would not
/// converge on the hub).
///
/// Warm-path assertions (R2) prove the cache hit: the hub-region 403 rule
/// is NOT consumed again and the cache snapshot is byte-for-byte unchanged
/// from R1.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_region"),
    ignore = "requires test_category 'multi_region'"
)]
pub async fn read_hub_region_caching_cold_then_warm() -> Result<(), Box<dyn Error>> {
    // Two 1002 rules on the preferred read region. The fault injection
    // runtime does not filter on request headers, so a single rule with
    // `hit_limit = 2` would also fire on R1's discovery rotation revisit
    // to the read region and suppress the real backend `403/3` the test
    // depends on. Splitting into r1 (always on, exhausted by R1) and r2
    // (initially disabled, enabled before R2) keeps each read's initial
    // attempt faulted while leaving the intermediate revisits live.
    let rule_1002_r1 = Arc::new(
        FaultInjectionRuleBuilder::new(
            "hub-cache-read-region-1002-r1",
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
        .with_hit_limit(1)
        .build(),
    );

    let rule_1002_r2 = Arc::new(
        FaultInjectionRuleBuilder::new(
            "hub-cache-read-region-1002-r2",
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
        .with_hit_limit(1)
        .build(),
    );
    rule_1002_r2.disable();

    // 403/3 on the hub region fires once on R1's first hub probe to force
    // the discovery rotation. After R1 populates the cache, R2 short-circuits
    // via the cache hit and this rule never fires again — that is the
    // cache-utilization signal we assert on the warm path.
    let rule_403 = Arc::new(
        FaultInjectionRuleBuilder::new(
            "hub-cache-hub-region-403-3",
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

    let rules = vec![
        Arc::clone(&rule_1002_r1),
        Arc::clone(&rule_1002_r2),
        Arc::clone(&rule_403),
    ];

    // Suppress hedging so the discovery rotation drives the wire flow
    // deterministically. With hedging enabled, the post-1002 hedge race
    // can win in `terminal_state: AlternateWon` and preempt hub-region
    // discovery before the cache is populated.
    let mut op_options = OperationOptions::default();
    op_options.availability_strategy = Some(AvailabilityStrategy::Disabled);

    DriverTestClient::run_with_unique_db_and_hedging(
        rules,
        op_options,
        // Pre-warm the preferred-regions list so the initial read targets
        // the satellite region (where the 1002 injection lives) rather
        // than the account-default first preference.
        vec![PREFERRED_READ_REGION, HUB_REGION],
        async |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item_body = br#"{"id": "hub-cache-item", "pk": "pk1", "value": "seed"}"#;
            context
                .create_item_with_pk(&container, "pk1", item_body)
                .await
                .expect("seed CreateItem should succeed");

            // Every hub-region cache assertion below must observe the same
            // driver across both reads, so create one persistent driver and
            // issue every read through it. The per-operation helpers each
            // build a throwaway driver and would never share cache state.
            let driver = context.create_persistent_driver().await?;

            // Force PPAF on the in-memory partition state. The bicep-
            // provisioned test account does not advertise PPAF natively,
            // which would leave the hub-region latch in
            // `build_session_retry_state` permanently disarmed. Harmless
            // no-op on accounts that already advertise PPAF.
            context.force_ppaf_enabled(&driver);

            let cache_before = context.hub_region_cache_snapshot(&driver);
            assert!(
                cache_before.is_empty(),
                "hub-region cache should be empty before any read, got {:?}",
                cache_before
            );

            // ── R1 (cold cache) ────────────────────────────────────────
            let r1 = context
                .read_item_on(&driver, &container, "hub-cache-item", "pk1")
                .await
                .expect("R1 (cold) ReadItem should succeed via hub-region discovery");
            let r1_diag = r1.diagnostics();

            assert_eq!(
                rule_1002_r1.hit_count(),
                1,
                "R1: read-region 1002-r1 rule should fire exactly once, got {}",
                rule_1002_r1.hit_count()
            );
            assert_eq!(
                rule_1002_r2.hit_count(),
                0,
                "R1: read-region 1002-r2 rule should not fire on R1 (still disabled), got {}",
                rule_1002_r2.hit_count()
            );
            assert_eq!(
                rule_403.hit_count(),
                1,
                "R1: hub-region 403/3 rule should fire exactly once, got {}",
                rule_403.hit_count()
            );

            let r1_count = r1_diag.request_count();
            assert!(
                r1_count >= 4,
                "R1 cold path should issue at least 4 wire attempts, got {}",
                r1_count
            );
            let r1_regions = r1_diag.regions_contacted();
            assert!(
                r1_regions.contains(&PREFERRED_READ_REGION),
                "R1: regions_contacted should include {:?}, got {:?}",
                PREFERRED_READ_REGION,
                r1_regions
            );
            assert!(
                r1_regions.contains(&HUB_REGION),
                "R1: regions_contacted should include hub {:?}, got {:?}",
                HUB_REGION,
                r1_regions
            );

            // Cache must contain exactly one partition entry pointing at
            // the hub. This is the strongest single-test signal that the
            // hub-region header was attached on retries — without the
            // header the cache would converge on the satellite region.
            let cache_after_r1 = context.hub_region_cache_snapshot(&driver);
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
                endpoint_url
                    .to_lowercase()
                    .contains(HUB_REGION_HOST_FRAGMENT),
                "R1: cached endpoint URL {endpoint_url:?} should point at the hub region \
                 (host fragment {HUB_REGION_HOST_FRAGMENT:?})"
            );

            // ── R2 (warm cache) ────────────────────────────────────────
            // Enable the second 1002 rule so the warm-path initial attempt
            // re-arms the hub-region latch (latch starts OFF on each fresh
            // operation; the cache lookup gate only consults the partition
            // entry once the latch is back ON).
            rule_1002_r2.enable();

            let r2 = context
                .read_item_on(&driver, &container, "hub-cache-item", "pk1")
                .await
                .expect("R2 (warm) ReadItem should succeed via cache short-circuit");
            let r2_diag = r2.diagnostics();

            assert_eq!(
                rule_1002_r1.hit_count(),
                1,
                "R2: read-region 1002-r1 rule should still be at 1 (exhausted on R1), got {}",
                rule_1002_r1.hit_count()
            );
            assert_eq!(
                rule_1002_r2.hit_count(),
                1,
                "R2: read-region 1002-r2 rule should fire exactly once on the warm-path \
                 initial attempt, got {}",
                rule_1002_r2.hit_count()
            );
            assert_eq!(
                rule_403.hit_count(),
                1,
                "R2: hub-region 403/3 rule should NOT have fired again \
                 (cache HIT short-circuits the discovery probe), got {}",
                rule_403.hit_count()
            );

            let r2_count = r2_diag.request_count();
            assert_eq!(
                r2_count, 2,
                "R2 warm path should issue exactly 2 wire attempts \
                 (read-region 1002 → hub-region cache-hit success), got {}",
                r2_count
            );
            let r2_regions = r2_diag.regions_contacted();
            assert!(
                r2_regions.contains(&PREFERRED_READ_REGION),
                "R2: regions_contacted should include {:?}, got {:?}",
                PREFERRED_READ_REGION,
                r2_regions
            );
            assert!(
                r2_regions.contains(&HUB_REGION),
                "R2: regions_contacted should include hub {:?}, got {:?}",
                HUB_REGION,
                r2_regions
            );

            // Cache snapshot must be byte-for-byte unchanged — the warm
            // path's success only refreshed `current_endpoint` to the
            // same hub value (cache_hub_region is idempotent).
            let cache_after_r2 = context.hub_region_cache_snapshot(&driver);
            assert_eq!(
                cache_after_r2, cache_after_r1,
                "R2: hub-region cache snapshot should be unchanged from R1"
            );

            Ok(())
        },
    )
    .await
}
