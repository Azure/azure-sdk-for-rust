use azure_core::http::Url;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

const INPUTS: &[usize] = &[1, 10, 100];

fn url_parsing_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("url_parsing");
    //group.measurement_time(Duration::from_secs(10));
    for num in INPUTS {
        let mut url = String::new();
        url.push_str("https://example.com/path/resource?param=value");

        for i in 0..*num {
            url.push_str(&format!("&param{}=value{}", i, i));
        }
        group.bench_function(BenchmarkId::new("parse_basic_url", num), |b| {
            b.iter(|| {
                let _url = Url::parse(&url).unwrap();
            })
        });
    }
    group.finish();
}

// Main benchmark group
criterion_group!{
    name = url_benches;
    config = Criterion::default().sample_size(200).warm_up_time(Duration::from_secs(5)); // default is 100 samples, warmup 3, just an example
    targets = url_parsing_benchmark
}
criterion_main!(url_benches);
