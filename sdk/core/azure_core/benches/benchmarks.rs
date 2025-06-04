use azure_core::http::{
    headers::Headers, HttpClient, Method, RawResponse, Request, StatusCode, Url,
};
use azure_core_test::http::MockHttpClient;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use futures::FutureExt;
use std::sync::Arc;

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

fn http_transport_test(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    // client to be used in the benchmark
    let mock_client = Arc::new(MockHttpClient::new(move |_| {
        async move {
            Ok(RawResponse::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                vec![],
            ))
        }
        .boxed()
    })) as Arc<dyn HttpClient>;

    // requests to be used in the benchmark
    let get_req = Request::new(Url::parse("https://localhost").unwrap(), Method::Get);
    let mut post_req = Request::new(Url::parse("https://localhost").unwrap(), Method::Post);
    post_req.set_body("test body");

    // Benchmark GET and POST requests
    c.bench_function("http_transport_get_async", |b| {
        b.to_async(&rt).iter(|| async {
            let _ = mock_client.execute_request(&get_req).await;
            black_box(());
        });
    });

    c.bench_function("http_transport_post_async", |b| {
        b.to_async(&rt).iter(|| async {
            let _ = mock_client.execute_request(&post_req).await;
            black_box(());
        });
    });
}
// Main benchmark configuration
criterion_group! {
    name = benchmarks;
    config = Criterion::default();
    targets = url_parsing_benchmark, http_transport_test
}
criterion_main!(benchmarks);
