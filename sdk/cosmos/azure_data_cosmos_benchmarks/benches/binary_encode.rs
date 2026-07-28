// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Criterion benchmark comparing item-write body serialization strategies:
//!
//! - **text** — `serde_json::to_vec` (the UTF-8 JSON path).
//! - **binary_v1** — `T` → [`serde_json::Value`] → `binary_json::encode`, the
//!   original two-pass Cosmos binary JSON path that materializes an
//!   intermediate `Value` tree.
//! - **binary_v2** — `binary_json::to_vec`, the native serde serializer that
//!   encodes `T` straight to Cosmos binary JSON with no intermediate `Value`.
//!
//! The point is to quantify the `Value`-elision win of v2 over v1 on a large,
//! realistic item (~1.7 MB), plus a small item to show fixed overhead.
//!
//! ```text
//! cargo bench -p azure_data_cosmos_benchmarks --bench binary_encode
//! ```

use azure_data_cosmos_driver::binary_json;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use serde::Serialize;

/// A log-entry-shaped item, mirroring the shape used by the SDK write samples.
#[derive(Serialize)]
struct LogEntry {
    id: String,
    pk: String,
    level: String,
    message: String,
    counter: u64,
    tags: Vec<String>,
}

impl LogEntry {
    /// Builds an entry whose `message` field is inflated to roughly
    /// `payload_size` bytes so the serialized document lands near that size.
    fn new(payload_size: usize) -> Self {
        let chunk = "log-payload-0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ ";
        let mut message = String::with_capacity(payload_size + chunk.len());
        while message.len() < payload_size {
            message.push_str(chunk);
        }
        message.truncate(payload_size);
        Self {
            id: "dynamic-00000000-0000-0000-0000-000000000000".to_owned(),
            pk: "INFO".to_owned(),
            level: "INFO".to_owned(),
            message,
            counter: 42,
            tags: vec!["alpha".to_owned(), "beta".to_owned(), "gamma".to_owned()],
        }
    }
}

/// v1 encode path: `T` → `serde_json::Value` → binary.
fn encode_v1<T: Serialize>(item: &T) -> Vec<u8> {
    let value = serde_json::to_value(item).expect("to_value");
    binary_json::encode(&value)
}

fn bench_binary_encode(c: &mut Criterion) {
    // A small item (fixed overhead) and a large ~1.7 MB item (throughput).
    let cases = [("small", 64_usize), ("large_1_7mb", 1_740 * 1024)];

    let mut group = c.benchmark_group("binary_encode");

    for (label, payload_size) in cases {
        let item = LogEntry::new(payload_size);
        // Report throughput against the produced text-JSON body size so the
        // three strategies are comparable on a common denominator.
        let text_len = serde_json::to_vec(&item).expect("to_vec").len();
        group.throughput(Throughput::Bytes(text_len as u64));

        group.bench_with_input(BenchmarkId::new("text", label), &item, |b, item| {
            b.iter(|| serde_json::to_vec(item).expect("to_vec"));
        });
        group.bench_with_input(BenchmarkId::new("binary_v1", label), &item, |b, item| {
            b.iter(|| encode_v1(item));
        });
        group.bench_with_input(BenchmarkId::new("binary_v2", label), &item, |b, item| {
            b.iter(|| binary_json::to_vec(item).expect("to_vec"));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_binary_encode);
criterion_main!(benches);
