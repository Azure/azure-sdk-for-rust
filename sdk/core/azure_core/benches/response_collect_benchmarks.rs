// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{headers::Headers, AsyncRawResponse, AsyncResponseBody, StatusCode},
    Bytes,
};
use bytes::BytesMut;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use futures::{stream, StreamExt};

/// The PR #3879 two-pass approach: collect all chunks into a Vec<Bytes>,
/// sum total length, allocate exact BytesMut, then copy everything in.
async fn collect_two_pass(mut body: AsyncResponseBody) -> Bytes {
    let mut chunks: Vec<Bytes> = Vec::new();
    let mut total_length = 0usize;
    while let Some(res) = body.next().await {
        let chunk = res.unwrap();
        total_length += chunk.len();
        chunks.push(chunk);
    }
    let mut result = BytesMut::with_capacity(total_length);
    for chunk in chunks {
        result.extend(&chunk);
    }
    result.into()
}

/// The current single-pass approach: reserve+extend directly into a growing BytesMut.
async fn collect_single_pass(body: AsyncResponseBody) -> Bytes {
    body.collect().await.unwrap()
}

fn make_body(chunk_size: usize, chunk_count: usize) -> AsyncResponseBody {
    let chunks: Vec<azure_core::Result<Bytes>> = (0..chunk_count)
        .map(|_| Ok(Bytes::from(vec![0u8; chunk_size])))
        .collect();
    AsyncRawResponse::new(StatusCode::Ok, Headers::new(), stream::iter(chunks).boxed()).into_body()
}

fn response_collect_benchmarks(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    // (chunk_size_bytes, chunk_count, label)
    let cases: &[(usize, usize, &str)] = &[
        (1_024, 10, "10x1KB"),
        (1_024, 100, "100x1KB"),
        (64 * 1_024, 10, "10x64KB"),
        (1_024 * 1_024, 10, "10x1MB"),
    ];

    let mut group = c.benchmark_group("response_collect");
    for &(chunk_size, chunk_count, label) in cases {
        group.bench_with_input(
            BenchmarkId::new("two_pass_pr3879", label),
            &(chunk_size, chunk_count),
            |b, &(cs, cc)| {
                b.to_async(&rt).iter(|| async move {
                    let body = make_body(cs, cc);
                    std::hint::black_box(collect_two_pass(body).await);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("single_pass_current", label),
            &(chunk_size, chunk_count),
            |b, &(cs, cc)| {
                b.to_async(&rt).iter(|| async move {
                    let body = make_body(cs, cc);
                    std::hint::black_box(collect_single_pass(body).await);
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, response_collect_benchmarks);
criterion_main!(benches);
