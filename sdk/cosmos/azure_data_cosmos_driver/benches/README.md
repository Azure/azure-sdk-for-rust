# `pk_range_cache_memory` — PartitionKeyRange cache memory micro-benchmark

> ⚠️ **DO NOT MERGE / DO NOT REVIEW.** This branch exists solely to collect
> retained-heap numbers comparable to the Python PR
> [Azure/azure-sdk-for-python#46297](https://github.com/Azure/azure-sdk-for-python/pull/46297),
> which strips unused fields from the Python PKR cache.

## What it measures

Retained-heap delta after building a 100,000-entry partition-key-range vector,
mirroring the `tracemalloc` methodology used in the Python PR. Variants:

| Variant | Fields retained |
|---------|-----------------|
| **A** | All 13 fields currently on `PartitionKeyRange` |
| **B** | Minimum: `id`, `min_inclusive`, `max_exclusive`, `parents` |
| **C** | B + `status` + `throughputFraction` (cross-SDK recommendation) |

Field shapes use **bytes-for-bytes equivalents** of the driver crate's types:
`Option<ETag>` (`Cow<'static, str>` newtype) and `EffectivePartitionKey`
(`String` newtype). The bench declares local newtype copies because
`PartitionKeyRange` is `pub(crate)` in the driver.

## Tool

[`dhat-rs`](https://docs.rs/dhat) is the closest Rust analog of Python's
`tracemalloc` — it reports retained bytes and live heap blocks via a global
allocator wrapper.

## Run

```bash
cargo bench --bench pk_range_cache_memory -p azure_data_cosmos_driver -- --nocapture
```

A `dhat-heap.json` viewer file is dropped in the repo root.

## Latest results (M-series Mac, release build)

| Variant | Retained MiB | Bytes/entry | Reduction vs A |
|---------|-------------:|------------:|---------------:|
| A: current (all 13 fields) | 43.90 | 460 | — |
| B: stripped (id/min/max/parents) | 14.05 | 147 | −68.0% |
| C: stripped + status + throughputFraction | 15.57 | 163 | −64.5% |

These align (within methodology error) with Python PR #46297 results
(854 → 452 B/entry, −47%) — the larger Rust reduction reflects the heavier
struct on Rust today (more `Option<String>` fields populated).

## Why a benchmark, not a code change

This branch is intentionally measurement-only. The actual field-stripping
decision should land as a separate, reviewable PR after we agree on which
fields to drop. The cross-SDK analysis (Java drops these fields entirely;
.NET v3 inherits them but doesn't read them on PKR) supports variant **C**
as the safest cut.
