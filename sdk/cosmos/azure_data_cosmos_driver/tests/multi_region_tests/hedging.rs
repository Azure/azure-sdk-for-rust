// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Multi-region live-account integration tests for cross-region request
//! hedging (Spec 0009: Cross-region hedging).
//!
//! **Prereqs**: ≥2-region read account. The `test-resources.bicep` template
//! provisions East US 2 + West US 3 when `enableMultipleRegions=true`. A
//! third region is required to exercise §15.3 `hedging_read_cross_region`
//! and the §6.7 fallback path; the two tests below run on the 2-region
//! topology.
//!
//! These tests use fault injection to introduce a deterministic delay on
//! reads in the primary region. The alternate region — selected by the
//! hedging eligibility evaluator from the driver's `preferred_regions` —
//! produces the winning response, and the returned `HedgeDiagnostics`
//! must classify the race per the §10.1 / §10.1.1 terminal-state taxonomy.
//!
//! Gated by `test_category = "multi_region"` — requires a live multi-region
//! Cosmos DB account, not the local emulator.

#![cfg(feature = "fault_injection")]

use crate::framework::DriverTestClient;
use azure_core::http::StatusCode;
use azure_data_cosmos_driver::diagnostics::{DiagnosticsContext, HedgeTerminalState};
use azure_data_cosmos_driver::error::CosmosError;
use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionResultBuilder, FaultInjectionRuleBuilder,
    FaultOperationType,
};
use azure_data_cosmos_driver::models::{
    ContainerReference, CosmosOperation, CosmosResponse, FeedRange, ItemReference,
    MaxItemCountHint, PartitionKey, ResponseBody,
};
use azure_data_cosmos_driver::options::{
    AvailabilityStrategy, ExcludedRegions, HedgeThreshold, HedgingStrategy, OperationOptions,
    OperationOptionsBuilder, PlanOptions, Region,
};
use azure_data_cosmos_driver::CosmosDriver;
use std::error::Error;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;

/// Primary region — the first entry in the `preferred_regions` list passed
/// to the driver. The hedging evaluator targets this region for the primary
/// leg and selects the next eligible entry as the alternate.
///
/// Defaults to the topology `test-resources.bicep` provisions (East US 2 +
/// West US 3, so the primary is West US 3). Set `AZURE_COSMOS_PRIMARY_REGION`
/// to run against an account in other regions — hard-coding a local topology
/// here would break the CI leg.
fn primary_region() -> Region {
    region_from_env("AZURE_COSMOS_PRIMARY_REGION", Region::WEST_US_3)
}

/// Alternate region — second entry in `preferred_regions`. Spec §6.2:
/// `regions[1]` is the deterministic alternate-region target. Override with
/// `AZURE_COSMOS_ALTERNATE_REGION`.
fn alternate_region() -> Region {
    region_from_env("AZURE_COSMOS_ALTERNATE_REGION", Region::EAST_US_2)
}

/// Reads a region name from `var`, falling back to the CI-provisioned default.
/// `Region::new` normalizes spacing and case, so "West US 2", "WestUS2", and
/// "westus2" are equivalent.
fn region_from_env(var: &str, default: Region) -> Region {
    match std::env::var(var) {
        Ok(name) if !name.trim().is_empty() => Region::new(name.trim().to_owned()),
        _ => default,
    }
}

/// Threshold short enough that any added delay on the primary leg fires
/// the alternate-region hedge well before the operation deadline. Per
/// §5.2 the driver default is `min(1000ms, request_timeout / 2)`; this is
/// explicit so the test does not depend on `request_timeout`.
///
/// 200 ms assumes an in-region test agent. From a developer machine the round
/// trip alone can exceed it, which spawns an alternate on a perfectly healthy
/// account and turns the zero-overhead happy-path test into a false failure.
/// Override with `AZURE_COSMOS_HEDGE_THRESHOLD_MS`.
const DEFAULT_HEDGE_THRESHOLD: Duration = Duration::from_millis(200);

fn hedge_threshold() -> Duration {
    match std::env::var("AZURE_COSMOS_HEDGE_THRESHOLD_MS") {
        Ok(raw) => match raw.trim().parse::<u64>() {
            Ok(ms) if ms > 0 => Duration::from_millis(ms),
            _ => DEFAULT_HEDGE_THRESHOLD,
        },
        Err(_) => DEFAULT_HEDGE_THRESHOLD,
    }
}

/// Delay injected on the primary leg's read. Defaults to `threshold + 1500ms`
/// so raising the threshold for a high-latency client keeps the "comfortably
/// past the threshold" invariant instead of silently inverting it.
///
/// That default only gives the alternate leg `delay - threshold` to finish.
/// The hedge leg's first request to a region pays a TLS handshake, so on a
/// distant client that window can expire for reasons that have nothing to do
/// with hedging. `AZURE_COSMOS_PRIMARY_DELAY_MS` widens it independently.
fn primary_read_delay() -> Duration {
    match std::env::var("AZURE_COSMOS_PRIMARY_DELAY_MS") {
        Ok(raw) => match raw.trim().parse::<u64>() {
            Ok(ms) if ms > 0 => return Duration::from_millis(ms),
            _ => {}
        },
        Err(_) => {}
    }
    hedge_threshold() + Duration::from_millis(1500)
}

/// How long the alternate leg actually gets to finish.
///
/// The secondary is not launched until the threshold elapses, so it races
/// against `delay - threshold` — never the delay itself. Reported in failure
/// messages because that window, not the delay, is what a distant client runs
/// out of.
fn hedge_delay_margin() -> Duration {
    primary_read_delay().saturating_sub(hedge_threshold())
}

/// Per-request `region=status` trail.
///
/// A hedge leg that fails fast (for example 404/1002 against a region the
/// seeded write has not replicated to yet) resolves the race identically to
/// one that was merely slower, so the terminal state alone cannot explain an
/// unexpected winner. The statuses can.
fn request_trail(diagnostics: &DiagnosticsContext) -> String {
    diagnostics
        .requests()
        .iter()
        .map(|request| {
            format!(
                "{}={:?}",
                request.region().map_or("<none>", Region::as_str),
                request.status()
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Builds a runtime-level [`OperationOptions`] that enables hedging with a
/// custom threshold. Tests pass this into the harness so the
/// driver-runtime layer of the option-resolution view
/// (per Spec 0002: Hierarchical configuration model)
/// carries the strategy for every
/// operation issued by the test.
fn hedging_operation_options(threshold: Duration) -> OperationOptions {
    let threshold = HedgeThreshold::new(threshold)
        .expect("threshold must be non-zero per HedgeThreshold::new contract");
    let strategy = AvailabilityStrategy::Hedging(HedgingStrategy::new(threshold));
    OperationOptionsBuilder::new()
        .with_availability_strategy(strategy)
        .build()
}

// ────────────────────────────────────────────────────────────────────────────
// Query continuation fixtures.
// ────────────────────────────────────────────────────────────────────────────

/// Logical partition every seeded document shares, so the query stays
/// single-partition and therefore hedge-eligible (`is_trivial()`).
const QUERY_PARTITION_KEY: &str = "hedge-query-pk";

/// Documents seeded into [`QUERY_PARTITION_KEY`].
const QUERY_DOCUMENT_IDS: [&str; 4] = ["q-1", "q-2", "q-3", "q-4"];

/// One document per page, so a four-document partition is guaranteed to mint a
/// continuation and exercise a cross-region resume rather than returning
/// everything in the first page.
const QUERY_PAGE_SIZE: u32 = 1;

/// Builds the single-partition query under test.
fn single_partition_query(container: &ContainerReference) -> CosmosOperation {
    CosmosOperation::query_items(
        container.clone(),
        Some(FeedRange::for_partition(
            PartitionKey::from(QUERY_PARTITION_KEY),
            container.partition_key_definition(),
        )),
    )
    .with_body(br#"{"query":"SELECT * FROM c","parameters":[]}"#.to_vec())
    .with_max_item_count(MaxItemCountHint::Limit(
        NonZeroU32::new(QUERY_PAGE_SIZE).expect("page size is non-zero"),
    ))
}

/// Options that make a region unreachable for routing, pinning the request to
/// whatever remains. With a two-region account, excluding one region also
/// drops the applicable-region count below two, so `should_hedge` returns
/// `false` and the page is served by exactly one region.
fn excluding(regions: &[Region]) -> OperationOptions {
    let mut options = OperationOptions::default();
    options.excluded_regions = Some(ExcludedRegions::from_iter(regions.iter().cloned()));
    options
}

fn page_document_ids(response: CosmosResponse) -> Vec<String> {
    fn parse(bytes: &[u8]) -> serde_json::Value {
        if azure_data_cosmos_driver::binary_json::is_binary(bytes) {
            azure_data_cosmos_driver::binary_json::decode(bytes)
                .expect("binary query response should decode")
        } else {
            serde_json::from_slice(bytes).expect("text query response should decode")
        }
    }

    let documents: Vec<serde_json::Value> = match response.into_body() {
        ResponseBody::NoPayload => Vec::new(),
        ResponseBody::Items(items) => items.iter().map(|item| parse(item)).collect(),
        ResponseBody::Bytes(bytes) => parse(&bytes)["Documents"]
            .as_array()
            .cloned()
            .unwrap_or_default(),
    };

    documents
        .iter()
        .map(|document| {
            document["id"]
                .as_str()
                .expect("seeded query document must carry an id")
                .to_owned()
        })
        .collect()
}

fn sorted_expected_ids() -> Vec<String> {
    let mut expected: Vec<String> = QUERY_DOCUMENT_IDS
        .iter()
        .map(|id| (*id).to_owned())
        .collect();
    expected.sort();
    expected
}

/// Bound on waiting for seeded data to become readable in a region. Seeds are
/// written by throwaway drivers, so later reads carry no session token to wait on.
const REPLICATION_WAIT: Duration = Duration::from_secs(30);

/// Polls `visible` until it returns `true`, failing after [`REPLICATION_WAIT`].
/// A 404 also means not yet: a new container can lag in a read region too.
async fn wait_for_replication(
    what: &str,
    mut visible: impl AsyncFnMut() -> Result<bool, Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    let deadline = std::time::Instant::now() + REPLICATION_WAIT;
    loop {
        let pending = match visible().await {
            Ok(true) => return Ok(()),
            Ok(false) => "incomplete".to_owned(),
            Err(error)
                if error
                    .downcast_ref::<CosmosError>()
                    .is_some_and(|error| error.status().status_code() == StatusCode::NotFound) =>
            {
                error.to_string()
            }
            Err(error) => return Err(error),
        };
        if std::time::Instant::now() >= deadline {
            return Err(format!("{what} not visible after {REPLICATION_WAIT:?}: {pending}").into());
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
}

/// Point-reads `item` in the region `options` pins until it succeeds, which
/// also warms that region's connection on `driver`.
async fn wait_for_seeded_item(
    driver: &CosmosDriver,
    item: &ItemReference,
    region: &Region,
    options: &OperationOptions,
) -> Result<(), Box<dyn Error>> {
    wait_for_replication(&format!("seeded item in {}", region.as_str()), async || {
        driver
            .execute_singleton_operation(CosmosOperation::read_item(item.clone()), options.clone())
            .await?;
        Ok(true)
    })
    .await
}

/// Drains [`single_partition_query`] in the region `options` pins until it
/// returns every seeded document, which also warms that region's connection.
async fn wait_for_seeded_documents(
    driver: &CosmosDriver,
    container: &ContainerReference,
    region: &Region,
    options: &OperationOptions,
) -> Result<(), Box<dyn Error>> {
    wait_for_replication(
        &format!("seeded query documents in {}", region.as_str()),
        async || {
            let mut plan = driver
                .plan_operation(
                    single_partition_query(container),
                    options,
                    None,
                    &PlanOptions::default(),
                )
                .await?;
            let mut ids = Vec::new();
            while let Some(page) = driver
                .execute_plan(&mut plan, Some(container.clone()), options.clone())
                .await?
            {
                ids.extend(page_document_ids(page));
            }
            ids.sort();
            Ok(ids == sorted_expected_ids())
        },
    )
    .await
}

/// Seeds [`QUERY_DOCUMENT_IDS`] into a fresh container's single logical
/// partition.
async fn seed_query_partition(
    context: &crate::framework::DriverTestRunContext,
    database: &azure_data_cosmos_driver::models::DatabaseReference,
) -> Result<ContainerReference, Box<dyn Error>> {
    let container_name = context.unique_container_name();
    let container = context
        .create_container(database, &container_name, "/pk")
        .await?;

    for id in QUERY_DOCUMENT_IDS {
        let body =
            format!(r#"{{"id":"{id}","pk":"{QUERY_PARTITION_KEY}","value":"continuation-test"}}"#);
        context
            .create_seed_item_with_pk(&container, QUERY_PARTITION_KEY, body.as_bytes())
            .await?;
    }

    Ok(container)
}

// ────────────────────────────────────────────────────────────────────────────
// Test 1: primary slow → alternate wins the race.
// ────────────────────────────────────────────────────────────────────────────

/// Read latency on the primary region exceeds the hedge threshold → the
/// alternate region must produce the winning response.
///
/// Validates the end-to-end §6.1 race + §10.1 diagnostics shape against a
/// real account: the diagnostics returned to the caller must record
/// `terminal_state == AlternateWon` and the alternate as `response_region`.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_region"),
    ignore = "requires test_category 'multi_region'"
)]
pub async fn hedging_primary_slow_alternate_wins() -> Result<(), Box<dyn Error>> {
    let primary = primary_region();
    let alternate = alternate_region();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "hedging-primary-slow",
            FaultInjectionResultBuilder::new()
                .with_delay(primary_read_delay())
                .with_probability(1.0)
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::ReadItem)
                .with_region(primary.clone())
                .build(),
        )
        .build(),
    );

    DriverTestClient::run_with_unique_db_and_hedging(
        vec![Arc::clone(&rule)],
        hedging_operation_options(hedge_threshold()),
        vec![primary.clone(), alternate.clone()],
        async |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            // Seed an item to read. The seeding write itself is not hedged
            // (writes are §5.1-excluded) and is unaffected by the
            // read-only fault rule above.
            let item_id = "hedge-item-1";
            let body = format!(r#"{{"id": "{item_id}", "pk": "pk1", "value": "hedge-test"}}"#);
            context
                .create_item_with_pk(&container, "pk1", body.as_bytes())
                .await?;
            let driver = context
                .runtime()
                .create_driver(context.driver_options()?)
                .await?;
            let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), item_id);

            // Warm the alternate region on the measured driver before the race.
            // The hedge leg's first request to a region pays a TLS handshake; if
            // that outlasts the primary's injected delay the alternate loses for
            // reasons unrelated to hedging. Scoped to the alternate so the
            // primary-region fault rule does not fire and `hits` stays clean.
            let alternate_only = excluding(std::slice::from_ref(&primary));
            wait_for_seeded_item(&driver, &item, &alternate, &alternate_only).await?;

            let started = std::time::Instant::now();
            let response = driver
                .execute_singleton_operation(
                    CosmosOperation::read_item(item),
                    OperationOptions::default(),
                )
                .await
                .expect("read should succeed via the hedged alternate region");
            let elapsed = started.elapsed();

            let diagnostics = response.diagnostics();
            let hedge_diag = diagnostics.hedge_diagnostics().expect(
                "hedge diagnostics must be attached when hedging is configured \
                 and at least one leg returns a Final response (spec §10.1)",
            );
            let trail = request_trail(&diagnostics);
            let hits = rule.hit_count();
            let delay = primary_read_delay();

            // Checked before the race outcome: if the delay never landed, the
            // primary legitimately won and the assertions below would be
            // blaming hedging for a fault-injection miss.
            assert!(
                hits > 0,
                "fault rule never fired, so the primary was never slowed and this \
                 test proves nothing about hedging; trail = [{trail}]",
            );

            assert_eq!(
                hedge_diag.terminal_state(),
                HedgeTerminalState::AlternateWon,
                "alternate must win when the primary is delayed past the threshold. \
                 hits={hits} (more than one per read means both legs matched the \
                 primary-region condition, i.e. the hedge leg did not route to the \
                 alternate); elapsed={elapsed:?} against injected delay {delay:?}. \
                 The alternate is only spawned at the threshold, so it had \
                 {margin:?} to finish a cold request to the alternate region — raise \
                 AZURE_COSMOS_PRIMARY_DELAY_MS if that is simply too short from \
                 this client; hedge_diag = {hedge_diag:?}; trail = [{trail}]",
                margin = hedge_delay_margin(),
            );
            assert!(
                matches!(
                    hedge_diag.terminal_state(),
                    HedgeTerminalState::AlternateWon
                ),
                "alternate-won invariant: terminal_state must be AlternateWon \
                 (spec §10.1); hedge_diag = {hedge_diag:?}",
            );
            assert_eq!(
                hedge_diag.response_region(),
                Some(&alternate),
                "winning response must come from the alternate region; \
                 hedge_diag = {hedge_diag:?}",
            );
            assert_eq!(
                hedge_diag.primary_region(),
                &primary,
                "primary_region must record the region that lost the race; \
                 hedge_diag = {hedge_diag:?}",
            );
            assert_eq!(
                hedge_diag.alternate_region(),
                Some(&alternate),
                "alternate_region must be populated once the threshold elapses \
                 and a hedge is spawned; hedge_diag = {hedge_diag:?}",
            );

            Ok(())
        },
    )
    .await
}

// ────────────────────────────────────────────────────────────────────────────
// Test 2: primary fast → zero-overhead happy path, no alternate spawned.
// ────────────────────────────────────────────────────────────────────────────

/// No fault injection → the primary returns within the threshold window and
/// the alternate is never spawned.
///
/// `HedgeDiagnostics` is still attached on the zero-overhead happy path
/// (spec §10.1: a synthetic "primary-only" record lets callers distinguish
/// *"hedging was selected but never fanned out"* from *"hedging was not
/// selected"*). The terminal state must classify as
/// `PrimaryWonPreThreshold` and only the primary leg launched.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_region"),
    ignore = "requires test_category 'multi_region'"
)]
pub async fn hedging_primary_fast_no_alternate() -> Result<(), Box<dyn Error>> {
    let primary = primary_region();
    let alternate = alternate_region();
    DriverTestClient::run_with_unique_db_and_hedging(
        vec![],
        hedging_operation_options(hedge_threshold()),
        vec![primary.clone(), alternate.clone()],
        async |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item_id = "hedge-item-fast";
            let body = format!(r#"{{"id": "{item_id}", "pk": "pk1", "value": "fast"}}"#);
            context
                .create_item_with_pk(&container, "pk1", body.as_bytes())
                .await?;
            let driver = context
                .runtime()
                .create_driver(context.driver_options()?)
                .await?;
            let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), item_id);

            // Warm the primary on the measured driver: the read below must beat the
            // threshold, and a cold handshake alone can exceed it. Pinned so the
            // warm-up isn't hedged; a losing primary leg is dropped mid-handshake.
            let primary_only = excluding(std::slice::from_ref(&alternate));
            wait_for_seeded_item(&driver, &item, &primary, &primary_only).await?;

            let response = driver
                .execute_singleton_operation(
                    CosmosOperation::read_item(item),
                    OperationOptions::default(),
                )
                .await?;
            let diagnostics = response.diagnostics();
            let hedge_diag = diagnostics.hedge_diagnostics().expect(
                "hedge diagnostics must be attached even on the zero-overhead happy \
                 path so consumers can distinguish hedge-selected-but-not-fanned-out \
                 from hedge-not-selected (spec §10.1)",
            );
            let trail = request_trail(&diagnostics);

            assert_eq!(
                hedge_diag.terminal_state(),
                HedgeTerminalState::PrimaryWonPreThreshold,
                "healthy account → primary must win before the threshold fires. If the \
                 client sits far from the primary region, raise \
                 AZURE_COSMOS_HEDGE_THRESHOLD_MS above the observed round trip; \
                 hedge_diag = {hedge_diag:?}; trail = [{trail}]",
            );
            assert!(
                !matches!(
                    hedge_diag.terminal_state(),
                    HedgeTerminalState::AlternateWon
                ),
                "terminal_state must NOT be AlternateWon when no alternate produced the response; \
                 hedge_diag = {hedge_diag:?}",
            );
            assert_eq!(
                hedge_diag.alternate_region(),
                None,
                "only the primary leg should be launched on the zero-overhead happy \
                 path (spec §6.5 invariant #3); hedge_diag = {hedge_diag:?}",
            );
            assert_eq!(
                hedge_diag.response_region(),
                Some(&primary),
                "primary owns the response on the zero-overhead happy path; \
                 hedge_diag = {hedge_diag:?}",
            );

            Ok(())
        },
    )
    .await
}

// ────────────────────────────────────────────────────────────────────────────
// Test 3: a continuation minted by one region resumes correctly in another.
// ────────────────────────────────────────────────────────────────────────────

/// Page-level query hedging is only sound if a continuation produced by the
/// region that served page N can be honored by a *different* region on page
/// N+1 — a hedged page can be won by either region, so the token crosses
/// regions whenever the winner flips.
///
/// This is the assumption that distinguishes queries from change feed, whose
/// continuation is a region-affine ETag and which is therefore excluded from
/// `HEDGEABLE_PAIRS`. Query continuations are expected to resume on globally
/// stable document rids, but that is a service contract, so assert it against
/// a real account rather than inferring it.
///
/// Hedging is configured but inert here: excluding one region of a two-region
/// account drops the applicable count below two, so each page is deterministically
/// served by exactly one region and the test isolates continuation portability
/// from the race.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_region"),
    ignore = "requires test_category 'multi_region'"
)]
pub async fn query_continuation_minted_in_primary_resumes_in_alternate(
) -> Result<(), Box<dyn Error>> {
    let primary = primary_region();
    let alternate = alternate_region();
    DriverTestClient::run_with_unique_db_and_hedging(
        vec![],
        hedging_operation_options(hedge_threshold()),
        vec![primary.clone(), alternate.clone()],
        async |context, database| {
            let container = seed_query_partition(&context, &database).await?;
            let driver = context
                .runtime()
                .create_driver(context.driver_options()?)
                .await?;
            let primary_only = excluding(std::slice::from_ref(&alternate));
            let alternate_only = excluding(std::slice::from_ref(&primary));
            wait_for_seeded_documents(&driver, &container, &primary, &primary_only).await?;
            wait_for_seeded_documents(&driver, &container, &alternate, &alternate_only).await?;

            // Page 1 on the primary region.
            let mut plan = driver
                .plan_operation(
                    single_partition_query(&container),
                    &primary_only,
                    None,
                    &PlanOptions::default(),
                )
                .await?;
            let first = driver
                .execute_plan(&mut plan, Some(container.clone()), primary_only.clone())
                .await?
                .ok_or("single-partition query must emit a first page")?;
            let first_regions = first.diagnostics().regions_contacted();
            assert!(
                !first_regions.contains(&alternate),
                "page 1 must be served by the primary region only; contacted = {first_regions:?}",
            );
            let mut seen = page_document_ids(first);
            assert!(
                !seen.is_empty(),
                "page 1 must return at least one document or there is no continuation to port",
            );

            let token = plan.to_continuation_token()?;

            // Every remaining page on the alternate region, resuming the
            // primary-minted token.
            let mut resumed = driver
                .plan_operation(
                    single_partition_query(&container),
                    &alternate_only,
                    Some(&token),
                    &PlanOptions::default(),
                )
                .await?;
            let mut resumed_pages = 0usize;
            while let Some(page) = driver
                .execute_plan(
                    &mut resumed,
                    Some(container.clone()),
                    alternate_only.clone(),
                )
                .await?
            {
                let regions = page.diagnostics().regions_contacted();
                assert!(
                    !regions.contains(&primary),
                    "resumed pages must avoid the excluded primary region; contacted = {regions:?}",
                );
                resumed_pages += 1;
                seen.extend(page_document_ids(page));
            }

            assert!(
                resumed_pages > 0,
                "the resumed plan must produce at least one page in the alternate region",
            );

            seen.sort();
            assert_eq!(
                seen,
                sorted_expected_ids(),
                "a continuation minted in {primary:?} must resume in {alternate:?} \
                 returning every document exactly once — no gaps, no duplicates",
            );

            Ok(())
        },
    )
    .await
}

// ────────────────────────────────────────────────────────────────────────────
// Test 4: hedged paged query end-to-end under a slow primary.
// ────────────────────────────────────────────────────────────────────────────

/// The feature as a customer sees it: a single-partition query paging through
/// a degraded primary region must stay correct and complete while the
/// alternate region absorbs the slow pages.
///
/// Asserts both halves of the contract — that at least one page was actually
/// won by the alternate (otherwise the test proves nothing about hedging), and
/// that the union of pages is still exactly the seeded set despite pages being
/// served by different regions.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_region"),
    ignore = "requires test_category 'multi_region'"
)]
pub async fn hedged_paged_query_survives_slow_primary() -> Result<(), Box<dyn Error>> {
    let primary = primary_region();
    let alternate = alternate_region();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "hedging-primary-slow-query",
            FaultInjectionResultBuilder::new()
                .with_delay(primary_read_delay())
                .with_probability(1.0)
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::QueryItem)
                .with_region(primary.clone())
                .build(),
        )
        .build(),
    );

    DriverTestClient::run_with_unique_db_and_hedging(
        vec![Arc::clone(&rule)],
        hedging_operation_options(hedge_threshold()),
        vec![primary.clone(), alternate.clone()],
        async |context, database| {
            let container = seed_query_partition(&context, &database).await?;
            let driver = context
                .runtime()
                .create_driver(context.driver_options()?)
                .await?;

            let options = OperationOptions::default();

            // Warm the alternate region first — see the matching note in
            // `hedging_primary_slow_alternate_wins`. The session token it captures
            // also stops a lagging primary from serving a short page.
            let warm = excluding(std::slice::from_ref(&primary));
            wait_for_seeded_documents(&driver, &container, &alternate, &warm).await?;

            let mut plan = driver
                .plan_operation(
                    single_partition_query(&container),
                    &options,
                    None,
                    &PlanOptions::default(),
                )
                .await?;

            let mut seen = Vec::new();
            let mut alternate_wins = 0usize;
            let mut pages = Vec::new();
            let started = std::time::Instant::now();
            while let Some(page) = driver
                .execute_plan(&mut plan, Some(container.clone()), options.clone())
                .await?
            {
                let diagnostics = page.diagnostics();
                let state = diagnostics.hedge_diagnostics().map(|d| d.terminal_state());
                pages.push(format!("{state:?} [{}]", request_trail(&diagnostics)));
                if state == Some(HedgeTerminalState::AlternateWon) {
                    alternate_wins += 1;
                }
                seen.extend(page_document_ids(page));
            }
            let elapsed = started.elapsed();

            let hits = rule.hit_count();
            assert!(
                hits > 0,
                "the query fault rule must have fired on the primary region",
            );
            assert!(
                alternate_wins > 0,
                "a primary delayed past the hedge threshold must lose at least one page \
                 to the alternate region; otherwise single-partition query hedging is \
                 not actually engaging. A PrimaryWonAfterHedge page means the hedge did \
                 spawn but the alternate leg lost or failed. hits={hits} across \
                 {page_count} pages (more than one per page means both legs matched the \
                 primary-region condition, i.e. the hedge leg did not route to the \
                 alternate); elapsed={elapsed:?} against injected delay {delay:?} per \
                 page, leaving the alternate {margin:?} per page to finish; \
                 pages = {pages:?}",
                page_count = pages.len(),
                delay = primary_read_delay(),
                margin = hedge_delay_margin(),
            );

            seen.sort();
            assert_eq!(
                seen,
                sorted_expected_ids(),
                "a hedged paged query must return every document exactly once even though \
                 pages were served by different regions",
            );

            Ok(())
        },
    )
    .await
}
