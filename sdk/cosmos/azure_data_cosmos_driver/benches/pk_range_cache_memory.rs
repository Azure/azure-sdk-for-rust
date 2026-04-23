//! Rust analog of the Python tracemalloc PKR-cache micro-benchmark from PR #46297.
//!
//! Builds 100,000 synthetic PartitionKeyRange entries in three variants and reports
//! retained heap bytes via dhat (Rust's closest analog to Python's tracemalloc).
//!
//! Variants:
//!   A. Current — exact struct from sdk/cosmos/azure_data_cosmos/src/routing/partition_key_range.rs
//!   B. Stripped — id, min, max, parents only (Python PKRange-equivalent)
//!   C. Stripped + 2 — adds status + throughputFraction (cross-SDK middle ground)
//!
//! Run:
//!   cargo run --release -- --variant a   # or b, c, all
//!
//! Apples-to-apples with Python's `tracemalloc.get_traced_memory()` "current" reading.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

const N_RANGES: usize = 100_000;


// Driver-equivalent newtypes: bytes-for-bytes layout match for memory accounting.
// `EtagLocal` mirrors `azure_data_cosmos_driver::models::ETag(Cow<'static, str>)`.
// `EpkLocal` mirrors `azure_data_cosmos_driver::models::EffectivePartitionKey(String)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtagLocal(pub Cow<'static, str>);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpkLocal(pub String);

// =========================================================================
// Variant A — current PartitionKeyRange (verbatim from azure_data_cosmos)
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PartitionKeyRangeStatus {
    #[default]
    Online,
    Splitting,
    Offline,
    Split,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionKeyRangeFull {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "_rid", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "_self", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    #[serde(rename = "_etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<EtagLocal>,
    #[serde(rename = "_ts", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(rename = "minInclusive")]
    pub min_inclusive: EpkLocal,
    #[serde(rename = "maxExclusive")]
    pub max_exclusive: EpkLocal,
    #[serde(rename = "ridPrefix", skip_serializing_if = "Option::is_none")]
    pub rid_prefix: Option<i32>,
    #[serde(rename = "throughputFraction", default)]
    pub throughput_fraction: f64,
    #[serde(rename = "targetThroughput", skip_serializing_if = "Option::is_none")]
    pub target_throughput: Option<f64>,
    #[serde(rename = "status", default)]
    pub status: PartitionKeyRangeStatus,
    #[serde(rename = "_lsn", default)]
    pub lsn: i64,
    #[serde(rename = "parents", skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<String>>,
    #[serde(
        rename = "ownedArchivalPKRangeIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub owned_archival_pk_range_ids: Option<Vec<String>>,
}

// =========================================================================
// Variant B — stripped (Python PKRange shape: id + min + max + parents)
// =========================================================================
#[derive(Debug, Clone)]
pub struct PartitionKeyRangeStripped {
    pub id: String,
    pub min_inclusive: String,
    pub max_exclusive: String,
    pub parents: Option<Vec<String>>,
}

// =========================================================================
// Variant C — stripped + status + throughputFraction (cross-SDK middle ground)
// =========================================================================
#[derive(Debug, Clone)]
pub struct PartitionKeyRangeStrippedKeep2 {
    pub id: String,
    pub min_inclusive: String,
    pub max_exclusive: String,
    pub parents: Option<Vec<String>>,
    pub status: PartitionKeyRangeStatus,
    pub throughput_fraction: f64,
}

// =========================================================================
// Synthetic generator — realistic field shapes mirroring service responses
// =========================================================================

/// Hex-style 16-char EPK (matches typical Cosmos effective-partition-key width).
fn epk_for(i: usize) -> String {
    // 16 hex chars; deterministic but distinct per-range so strings can't share storage.
    format!("{:016X}", (i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15))
}

/// 24-char base64-like rid string (mirrors Cosmos rid widths).
fn rid_for(i: usize) -> String {
    // Deterministic, 24 chars. Ends in == like real base64 padding.
    format!("3FIlAOzjvyg{:010X}AA==", i as u64)
}

/// Build N full-shape PartitionKeyRange entries with realistic field values.
fn gen_full(n: usize) -> Vec<PartitionKeyRangeFull> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        let min_inc = if i == 0 { String::new() } else { epk_for(i) };
        let max_exc = if i == n - 1 { "FF".to_string() } else { epk_for(i + 1) };
        let rid = rid_for(i);
        v.push(PartitionKeyRangeFull {
            id: i.to_string(),
            resource_id: Some(rid.clone()),
            self_link: Some(format!(
                "dbs/3FIlAA==/colls/3FIlAOzjvyg=/pkranges/{}/",
                rid
            )),
            etag: Some(EtagLocal(Cow::Owned(format!("\"{:08x}-0000-0800-0000-{:012x}0000\"", i, i)))),
            timestamp: Some(1_700_000_000 + i as i64),
            min_inclusive: EpkLocal(min_inc),
            max_exclusive: EpkLocal(max_exc),
            rid_prefix: Some((i % 256) as i32),
            throughput_fraction: 1.0 / n as f64,
            target_throughput: Some(400.0),
            status: PartitionKeyRangeStatus::Online,
            lsn: 100 + i as i64,
            parents: if i % 2 == 0 {
                None
            } else {
                Some(vec![(i / 2).to_string()])
            },
            owned_archival_pk_range_ids: None,
        });
    }
    v
}

/// Convert a full-shape vec into the stripped variant — same string contents
/// would be allocated freshly anyway in the wire-deserialization path, so we
/// just rebuild from scratch for an isolated measurement.
fn gen_stripped(n: usize) -> Vec<PartitionKeyRangeStripped> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        let min_inc = if i == 0 { String::new() } else { epk_for(i) };
        let max_exc = if i == n - 1 { "FF".to_string() } else { epk_for(i + 1) };
        v.push(PartitionKeyRangeStripped {
            id: i.to_string(),
            min_inclusive: min_inc,
            max_exclusive: max_exc,
            parents: if i % 2 == 0 {
                None
            } else {
                Some(vec![(i / 2).to_string()])
            },
        });
    }
    v
}

fn gen_stripped_keep2(n: usize) -> Vec<PartitionKeyRangeStrippedKeep2> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        let min_inc = if i == 0 { String::new() } else { epk_for(i) };
        let max_exc = if i == n - 1 { "FF".to_string() } else { epk_for(i + 1) };
        v.push(PartitionKeyRangeStrippedKeep2 {
            id: i.to_string(),
            min_inclusive: min_inc,
            max_exclusive: max_exc,
            parents: if i % 2 == 0 {
                None
            } else {
                Some(vec![(i / 2).to_string()])
            },
            status: PartitionKeyRangeStatus::Online,
            throughput_fraction: 1.0 / n as f64,
        });
    }
    v
}

// =========================================================================
// Measurement helpers
// =========================================================================

#[derive(Debug, Clone, Copy)]
struct Snapshot {
    curr_bytes: u64,
    curr_blocks: u64,
    max_bytes: u64,
}

fn snapshot() -> Snapshot {
    let s = dhat::HeapStats::get();
    Snapshot {
        curr_bytes: s.curr_bytes as u64,
        curr_blocks: s.curr_blocks as u64,
        max_bytes: s.max_bytes as u64,
    }
}

fn delta(before: Snapshot, after: Snapshot) -> (i64, i64, i64) {
    (
        after.curr_bytes as i64 - before.curr_bytes as i64,
        after.curr_blocks as i64 - before.curr_blocks as i64,
        after.max_bytes as i64 - before.max_bytes as i64,
    )
}

fn mib(bytes: i64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}

#[derive(Debug)]
struct Result {
    label: &'static str,
    retained_bytes: i64,
    retained_blocks: i64,
    peak_delta_bytes: i64,
    n_entries: usize,
}

fn measure_full(n: usize) -> Result {
    // Pre-snapshot baseline — only the cache vec contributes to the delta.
    let before = snapshot();
    let v = gen_full(n);
    let after = snapshot();
    let (rb, blocks, peak) = delta(before, after);
    // Keep alive until we've snapshot.
    std::hint::black_box(&v);
    drop(v);
    Result {
        label: "A: current (all 13 fields)",
        retained_bytes: rb,
        retained_blocks: blocks,
        peak_delta_bytes: peak,
        n_entries: n,
    }
}

fn measure_stripped(n: usize) -> Result {
    let before = snapshot();
    let v = gen_stripped(n);
    let after = snapshot();
    let (rb, blocks, peak) = delta(before, after);
    std::hint::black_box(&v);
    drop(v);
    Result {
        label: "B: stripped (id, min, max, parents)",
        retained_bytes: rb,
        retained_blocks: blocks,
        peak_delta_bytes: peak,
        n_entries: n,
    }
}

fn measure_stripped_keep2(n: usize) -> Result {
    let before = snapshot();
    let v = gen_stripped_keep2(n);
    let after = snapshot();
    let (rb, blocks, peak) = delta(before, after);
    std::hint::black_box(&v);
    drop(v);
    Result {
        label: "C: stripped + status + throughputFraction",
        retained_bytes: rb,
        retained_blocks: blocks,
        peak_delta_bytes: peak,
        n_entries: n,
    }
}

fn print_results(results: &[Result]) {
    println!();
    println!("# Rust PKR-cache memory micro-benchmark");
    println!();
    println!("Tool: dhat-rs (Rust analog of Python tracemalloc, retained-heap delta)");
    println!("Entries per scenario: {}", N_RANGES);
    println!();
    println!("| Variant | Retained MiB | Bytes / entry | Heap blocks | Peak delta MiB |");
    println!("|---------|-------------:|--------------:|------------:|---------------:|");
    for r in results {
        println!(
            "| {} | {:.2} | {:.0} | {} | {:.2} |",
            r.label,
            mib(r.retained_bytes),
            r.retained_bytes as f64 / r.n_entries as f64,
            r.retained_blocks,
            mib(r.peak_delta_bytes),
        );
    }
    println!();
    if results.len() >= 2 {
        let baseline = results[0].retained_bytes;
        println!("## Reductions vs Variant A");
        println!();
        for r in &results[1..] {
            let saved = baseline - r.retained_bytes;
            let pct = (saved as f64 / baseline as f64) * 100.0;
            println!(
                "- {}: {:.2} MiB saved per cache ({:.1}% reduction)",
                r.label,
                mib(saved),
                pct
            );
        }
        println!();
        println!("## 150-cache cluster projection (mirrors Python PR #46297)");
        println!();
        println!("| Scenario | Total memory |");
        println!("|----------|-------------:|");
        for r in results {
            let total = (r.retained_bytes as f64) * 150.0;
            println!(
                "| 150 caches × per-cache: {} | {:.2} GiB |",
                r.label,
                total / (1024.0 * 1024.0 * 1024.0)
            );
        }
        println!(
            "| Single shared cache (any variant) | {:.2} MiB |",
            mib(baseline)
        );
    }
}

fn main() {
    let _profiler = dhat::Profiler::builder().testing().build();

    // Run each variant in its own scope, dropping between to isolate deltas.
    println!("Generating Variant A (current)...");
    let r_a = measure_full(N_RANGES);
    println!("  retained: {:.2} MiB", mib(r_a.retained_bytes));

    println!("Generating Variant B (stripped)...");
    let r_b = measure_stripped(N_RANGES);
    println!("  retained: {:.2} MiB", mib(r_b.retained_bytes));

    println!("Generating Variant C (stripped + 2)...");
    let r_c = measure_stripped_keep2(N_RANGES);
    println!("  retained: {:.2} MiB", mib(r_c.retained_bytes));

    print_results(&[r_a, r_b, r_c]);
}
