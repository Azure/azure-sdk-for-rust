// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Criterion benchmark comparing item-read body deserialization strategies:
//!
//! - **text** — `serde_json::from_slice::<T>` on a UTF-8 JSON body.
//! - **binary_value** — `binary_json::decode` → `serde_json::from_value::<T>`,
//!   the two-stage path that materializes an intermediate `serde_json::Value`
//!   tree before building the typed value.
//! - **binary_native** — `binary_json::from_slice::<T>`, the native serde
//!   deserializer that drives `T::deserialize` straight off the binary buffer
//!   with no intermediate `Value`.
//!
//! Both a typed target (`LogEntry`) and a `serde_json::Value` target are
//! measured, on a small and a large (~1.7 MB) item.
//!
//! ```text
//! cargo bench -p azure_data_cosmos_benchmarks --bench binary_decode
//! ```

use azure_data_cosmos_driver::binary_json;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use serde::{Deserialize, Serialize};

/// A log-entry-shaped item, mirroring the shape used by the SDK samples.
#[derive(Serialize, Deserialize)]
struct LogEntry {
    id: String,
    pk: String,
    level: String,
    message: String,
    counter: u64,
    tags: Vec<String>,
}

impl LogEntry {
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

/// Two-stage binary decode: bytes → `serde_json::Value` → `T`.
fn decode_via_value<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> T {
    let value = binary_json::decode(bytes).expect("decode");
    serde_json::from_value(value).expect("from_value")
}

fn bench_binary_decode(c: &mut Criterion) {
    let cases = [("small", 64_usize), ("large_1_7mb", 1_740 * 1024)];

    let mut group = c.benchmark_group("binary_decode");

    for (label, payload_size) in cases {
        let item = LogEntry::new(payload_size);
        let text = serde_json::to_vec(&item).expect("to_vec");
        let binary = binary_json::to_vec(&item).expect("to_vec");
        group.throughput(Throughput::Bytes(text.len() as u64));

        // ---- typed target: LogEntry ----
        group.bench_with_input(BenchmarkId::new("text/typed", label), &text, |b, text| {
            b.iter(|| serde_json::from_slice::<LogEntry>(text).expect("from_slice"));
        });
        group.bench_with_input(
            BenchmarkId::new("binary_value/typed", label),
            &binary,
            |b, binary| b.iter(|| decode_via_value::<LogEntry>(binary)),
        );
        group.bench_with_input(
            BenchmarkId::new("binary_native/typed", label),
            &binary,
            |b, binary| b.iter(|| binary_json::from_slice::<LogEntry>(binary).expect("from_slice")),
        );

        // ---- dynamic target: serde_json::Value ----
        group.bench_with_input(BenchmarkId::new("text/value", label), &text, |b, text| {
            b.iter(|| serde_json::from_slice::<serde_json::Value>(text).expect("from_slice"));
        });
        group.bench_with_input(
            BenchmarkId::new("binary_value/value", label),
            &binary,
            |b, binary| b.iter(|| decode_via_value::<serde_json::Value>(binary)),
        );
        group.bench_with_input(
            BenchmarkId::new("binary_native/value", label),
            &binary,
            |b, binary| {
                b.iter(|| binary_json::from_slice::<serde_json::Value>(binary).expect("from_slice"))
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_binary_decode);
criterion_main!(benches);
