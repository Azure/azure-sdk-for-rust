// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Generates the two real example outputs used in the diagnostics-capture PR description.
//!
//! Each test drives the gated-capture front-end (the same API the driver uses on the hot path),
//! lets the op-end gate build the **canonical** [`DiagnosticsContext`], and prints its detailed
//! JSON. Run them with output visible to regenerate the examples:
//!
//! ```text
//! cargo test -p azure_data_cosmos_driver --all-features --test diagnostics_examples -- --nocapture
//! ```

use azure_data_cosmos_driver::diagnostics::capture::{
    finish, AttemptRecord, DiagnosticsPolicy, DiagnosticsRecorder, HedgeOutcome, LogPool, Outcome,
};
use azure_data_cosmos_driver::diagnostics::ExecutionContext;
use azure_data_cosmos_driver::options::DiagnosticsOptions;
use azure_data_cosmos_driver::{DiagnosticsEncoding, DiagnosticsVerbosity};
use std::sync::Arc;
use std::time::Duration;

fn options() -> Arc<DiagnosticsOptions> {
    Arc::new(DiagnosticsOptions::default())
}

/// Example: a typical operation that retries a throttled (429) attempt and then succeeds (200).
#[test]
fn example_typical_operation() {
    let pool = LogPool::new();
    let mut rec = DiagnosticsRecorder::start(
        &pool,
        "read_item",
        "https://acct.documents.azure.com/",
        "a1b2c3d4-0000-1111-2222-typical000001",
    );
    rec.record_attempt(
        AttemptRecord::new(
            ExecutionContext::Initial,
            "East US",
            "https://acct-eastus.documents.azure.com/",
            429,
        )
        .with_service_request_id("req-eastus-429")
        .with_request_charge(4.2)
        .with_sub_status(3200)
        .with_duration_ns(3_200_000),
    );
    rec.record_attempt(
        AttemptRecord::new(
            ExecutionContext::Retry,
            "East US",
            "https://acct-eastus.documents.azure.com/",
            200,
        )
        .with_service_request_id("req-eastus-200")
        .with_request_charge(4.2)
        .with_duration_ns(4_100_000),
    );
    rec.record_end(Outcome::Success, 2, 200, None, Some(7_300_000));

    // Always-build so the example is deterministic regardless of wall-clock timing.
    let ctx = finish(rec, &DiagnosticsPolicy::always(), options())
        .expect("a context is built in Always mode");

    assert_eq!(ctx.request_count(), 2);
    assert_eq!(ctx.status().map(|s| u16::from(s.status_code())), Some(200));
    assert!(ctx.hedge_diagnostics().is_none());

    println!("=== Example: diagnostics for a typical operation ===");
    println!(
        "{}",
        ctx.to_json_string(Some(DiagnosticsVerbosity::Detailed))
    );
}

/// Example: a hedged multi-region operation where the alternate region (West US) wins the race.
#[test]
fn example_hedged_operation() {
    let pool = LogPool::new();
    let mut rec = DiagnosticsRecorder::start(
        &pool,
        "read_item",
        "https://acct.documents.azure.com/",
        "a1b2c3d4-0000-1111-2222-hedged0000001",
    );
    // Primary leg (East US) is slow and never responds before the alternate wins.
    rec.record_attempt(
        AttemptRecord::new(
            ExecutionContext::Hedging,
            "East US",
            "https://acct-eastus.documents.azure.com/",
            0,
        )
        .with_request_sent("sent")
        .with_duration_ns(8_500_000),
    );
    // Alternate leg (West US) returns first and wins the race.
    rec.record_attempt(
        AttemptRecord::new(
            ExecutionContext::Hedging,
            "West US",
            "https://acct-westus.documents.azure.com/",
            200,
        )
        .with_service_request_id("req-westus-200")
        .with_request_charge(3.1)
        .with_duration_ns(4_300_000),
    );
    rec.record_hedge_outcome(
        HedgeOutcome::AlternateWon,
        Duration::from_millis(500),
        "East US",
        Some("West US"),
        Some("West US"),
    );
    rec.record_end(Outcome::Success, 2, 200, None, Some(9_000_000));

    let ctx = finish(rec, &DiagnosticsPolicy::always(), options())
        .expect("a context is built in Always mode");

    assert_eq!(ctx.request_count(), 2);
    let hedge = ctx.hedge_diagnostics().expect("hedge diagnostics attached");
    assert_eq!(
        hedge.terminal_state(),
        azure_data_cosmos_driver::diagnostics::HedgeTerminalState::AlternateWon
    );
    assert_eq!(hedge.response_region().map(|r| r.as_str()), Some("westus"));

    println!("=== Example: diagnostics for a hedged operation ===");
    println!(
        "{}",
        ctx.to_json_string(Some(DiagnosticsVerbosity::Detailed))
    );
    // The hedge race outcome (terminal state + winning region) is not part of the request-level
    // JSON, so surface it explicitly for the example.
    println!(
        "hedge: terminal_state={:?} primary_region={} alternate_region={:?} winning_region={:?}",
        hedge.terminal_state(),
        hedge.primary_region().as_str(),
        hedge.alternate_region().map(|r| r.as_str()),
        hedge.response_region().map(|r| r.as_str()),
    );
}

/// Example: the top-level `summary` block (computed at finalization) and the three encoding modes.
#[test]
fn example_summary_and_encoding() {
    let pool = LogPool::new();
    let mut rec = DiagnosticsRecorder::start(
        &pool,
        "create_item",
        "https://acct.documents.azure.com/",
        "a1b2c3d4-0000-1111-2222-summary00enc01",
    );
    rec.record_attempt(
        AttemptRecord::new(
            ExecutionContext::Initial,
            "East US",
            "https://acct-eastus.documents.azure.com/",
            429,
        )
        .with_service_request_id("req-eastus-429")
        .with_request_charge(4.2)
        .with_sub_status(3200)
        .with_duration_ns(3_200_000),
    );
    rec.record_attempt(
        AttemptRecord::new(
            ExecutionContext::Retry,
            "East US",
            "https://acct-eastus.documents.azure.com/",
            200,
        )
        .with_service_request_id("req-eastus-200")
        .with_request_charge(4.2)
        .with_duration_ns(4_100_000),
    );
    rec.record_end(Outcome::Success, 2, 200, None, Some(7_300_000));

    let ctx = finish(rec, &DiagnosticsPolicy::always(), options())
        .expect("a context is built in Always mode");

    // The summary aggregates over the requests; it exists only on a built context.
    let summary = ctx.summary();
    assert_eq!(summary.request_count(), 2);
    assert_eq!(summary.retry_count(), 1);
    assert_eq!(summary.throttled_count(), 1);

    println!("=== Example: top-level summary block ===");
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::to_value(summary).unwrap()).unwrap()
    );

    // The three encoding modes for the same context, with their sizes.
    let json = ctx.encode(DiagnosticsEncoding::Json);
    let compact = ctx.encode(DiagnosticsEncoding::Compact);
    let encoded = ctx.encode(DiagnosticsEncoding::Encoded);
    println!("=== Example: encoding modes (same context) ===");
    println!("Json    ({} bytes, pretty)", json.len());
    println!("Compact ({} bytes): {}", compact.len(), compact);
    println!("Encoded ({} bytes, base64): {}", encoded.len(), encoded);
}
