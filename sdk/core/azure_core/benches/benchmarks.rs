use azure_core::http::Url;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
// how many params to add to the url
const PARAMS: &[usize] = &[1, 10, 100, 1000];

fn url_parsing_benchmark(c: &mut Criterion) {
    for num in PARAMS {
        let mut url = String::new();
        url.push_str("https://example.com/path/resource?param=value");

        for i in 0..*num {
            url.push_str(&format!("&param{}=value{}", i, i));
        }
        c.bench_function(&format!("parse_basic_url_{}", num), |b| {
            b.iter(|| {
                black_box(Url::parse(&url).unwrap());
            })
        });
    }
}

// Main benchmark configuration
criterion_group! {
    name = benchmarks;
    config = Criterion::default().sample_size(200).warm_up_time(Duration::from_secs(5));
    targets = url_parsing_benchmark
}
criterion_main!(benchmarks);
