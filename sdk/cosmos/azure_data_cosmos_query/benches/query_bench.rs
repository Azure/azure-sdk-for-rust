// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn bench_parse_simple(c: &mut Criterion) {
    c.bench_function("parse_select_star", |b| {
        b.iter(|| azure_data_cosmos_query::parse(black_box("SELECT * FROM c")))
    });
}

fn bench_parse_complex(c: &mut Criterion) {
    let sql = "SELECT DISTINCT TOP 10 c.name, c.age AS a, COUNT(1) \
               FROM c JOIN t IN c.tags \
               WHERE c.pk = 'hello' AND c.age > 21 AND CONTAINS(c.name, 'test') \
               GROUP BY c.name \
               ORDER BY c.age DESC \
               OFFSET 5 LIMIT 20";
    c.bench_function("parse_complex_query", |b| {
        b.iter(|| azure_data_cosmos_query::parse(black_box(sql)))
    });
}

fn bench_plan_generation(c: &mut Criterion) {
    let program = azure_data_cosmos_query::parse(
        "SELECT * FROM c WHERE c.pk = 'hello' AND c.age > 21 ORDER BY c.age DESC",
    )
    .unwrap();
    c.bench_function("generate_query_plan", |b| {
        b.iter(|| {
            azure_data_cosmos_query::plan::generate_query_plan(
                black_box(&program.query),
                black_box(&["/pk"]),
            )
        })
    });
}

fn bench_eval_match(c: &mut Criterion) {
    let program =
        azure_data_cosmos_query::parse("SELECT * FROM c WHERE c.age > 21 AND c.name = 'Alice'")
            .unwrap();
    let doc = serde_json::json!({"name": "Alice", "age": 30, "pk": "x"});
    c.bench_function("eval_matches_query", |b| {
        b.iter(|| {
            azure_data_cosmos_query::eval::matches_query(
                black_box(&doc),
                black_box(&program.query),
                black_box(&[]),
            )
        })
    });
}

fn bench_eval_project(c: &mut Criterion) {
    let program = azure_data_cosmos_query::parse(
        "SELECT c.name, c.age AS years, UPPER(c.city) AS city FROM c",
    )
    .unwrap();
    let doc = serde_json::json!({"name": "Alice", "age": 30, "city": "seattle", "extra": true});
    c.bench_function("eval_project", |b| {
        b.iter(|| {
            azure_data_cosmos_query::eval::project(
                black_box(&doc),
                black_box(&program.query),
                black_box(&[]),
            )
        })
    });
}

fn bench_lexer_tokenize(c: &mut Criterion) {
    let sql = "SELECT c.name, c.age FROM c WHERE c.pk = 'hello' AND c.age > 21 ORDER BY c.age";
    c.bench_function("lexer_tokenize", |b| {
        b.iter(|| azure_data_cosmos_query::lexer::Lexer::tokenize(black_box(sql)))
    });
}

fn bench_like_pattern(c: &mut Criterion) {
    let program = azure_data_cosmos_query::parse(
        "SELECT * FROM c WHERE c.name LIKE '%alice%wonderland%through%looking%glass%'",
    )
    .unwrap();
    let doc = serde_json::json!({"name": "this is a very long string that does not match the pattern at all"});
    c.bench_function("eval_like_worst_case", |b| {
        b.iter(|| {
            azure_data_cosmos_query::eval::matches_query(
                black_box(&doc),
                black_box(&program.query),
                black_box(&[]),
            )
        })
    });
}

criterion_group!(
    benches,
    bench_parse_simple,
    bench_parse_complex,
    bench_plan_generation,
    bench_eval_match,
    bench_eval_project,
    bench_lexer_tokenize,
    bench_like_pattern,
);
criterion_main!(benches);
