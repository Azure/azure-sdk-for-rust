use azure_core::http::{
    headers::{HeaderName, HeaderValue, Headers},
    new_http_client, Method, Request, StatusCode, Url,
};

use azure_core::error::ErrorKind;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

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

fn http_request_creation_benchmark(c: &mut Criterion) {
    fn create_request(c: usize) {
        let mut url = String::new();
        url.push_str("https://example.com/path/resource?param=value");

        for i in 0..c {
            url.push_str(&format!("&param{}=value{}", i, i));
        }
        let mut req = Request::new(Url::parse(&url).unwrap(), Method::Get);
        for i in 0..c {
            let header_name = format!("x-ms-header-{}", i);
            let header_value = format!("value_{}", i);
            req.insert_header(
                HeaderName::from(header_name),
                HeaderValue::from(header_value),
            );
        }
    }

    // Benchmark creating requests with many headers
    for num in PARAMS {
        c.bench_function(&format!("create_request_{}", num), |b| {
            b.iter(|| {
                black_box(create_request(*num));
            })
        });
    }

    fn create_request_with_headers(c: usize) {
        for i in 0..c {
            let mut req = Request::new(Url::parse("https://example.com").unwrap(), Method::Get);
            let header_name = format!("x-ms-header-{}", i);
            let header_value = format!("value_{}", i);
            req.insert_header(
                HeaderName::from(header_name),
                HeaderValue::from(header_value),
            );
        }
    }
    // Benchmark creating requests with headers
    for num in PARAMS {
        c.bench_function(&format!("create_request_with_headers_{}", num), |b| {
            b.iter(|| {
                black_box(create_request_with_headers(*num));
            })
        });
    }

    // Benchmark with request body
    for size in PARAMS {
        let body = "0".repeat(*size);
        c.bench_function(&format!("create_request_with_body_{}", size), |b| {
            b.iter(|| {
                black_box({
                    let mut req =
                        Request::new(Url::parse("https://example.com").unwrap(), Method::Put);
                    req.set_body(body.as_bytes().to_vec());
                })
            })
        });
    }
}

fn header_operations_benchmark(c: &mut Criterion) {
    fn header_operations(c: usize) {
        let mut headers = Headers::new();
        for (i, j) in (0..c).zip(0..c) {
            headers.insert(
                {
                    let header_name = format!("x-ms-header-{}", i);
                    HeaderName::from(header_name)
                },
                {
                    let header_value = format!("value_{}", j);
                    HeaderValue::from(header_value)
                },
            );
        }
        for i in 0..c {
            // Read headers
            let header_name = format!("x-ms-header-{}", i);
            let _ = headers.get_str(&HeaderName::from(header_name));
        }
    }
    // Benchmark header operations
    for num in PARAMS {
        c.bench_function(&format!("header_operations_{}", num), |b| {
            b.iter(|| {
                black_box(header_operations(*num));
            })
        });
    }
}

fn http_transport_test(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    async fn make_get_request() -> azure_core::Result<StatusCode> {
        let req = Request::new(
            Url::parse("http://127.0.0.1:5000/Admin/isAlive").unwrap(),
            Method::Get,
        );
        let http_client = new_http_client();
        let resp = http_client.execute_request(&req).await?;

        if resp.status() != StatusCode::Ok {
            let status_code = resp.status();

            return Err(azure_core::Error::message(
                ErrorKind::http_response(
                    status_code,
                    Some(status_code.canonical_reason().to_string()),
                ),
                format!("{status_code} response from server"),
            ));
        }

        let status = resp.status();
        Ok(status)
    }

    async fn make_post_request() -> azure_core::Result<StatusCode> {
        let mut req = Request::new(
            Url::parse("http://127.0.0.1:5000/Admin/setRecordingOptions").unwrap(),
            Method::Post,
        );
        let body = "{ \"HandleRedirects\": \"true\"}".to_string();
        req.set_body(body.as_bytes().to_vec());
        req.insert_header(
            HeaderName::from("content-type"),
            HeaderValue::from("application/json"),
        );
        let http_client = new_http_client();
        let resp = http_client.execute_request(&req).await?;

        if resp.status() != StatusCode::Ok {
            let status_code = resp.status();

            return Err(azure_core::Error::message(
                ErrorKind::http_response(
                    status_code,
                    Some(status_code.canonical_reason().to_string()),
                ),
                format!("{status_code} response from server"),
            ));
        }

        let status = resp.status();
        Ok(status)
    }

    c.bench_function("http_transport_get_async", |b| {
        b.to_async(&rt).iter(|| async {
            make_get_request().await.unwrap_or_else(|err| {
                panic!("Failed to make GET request {err}");
            })
        });
    });

    c.bench_function("http_transport_post_async", |b| {
        b.to_async(&rt).iter(|| async {
            make_post_request().await.unwrap_or_else(|err| {
                panic!("Failed to make POST request {err}");
            })
        });
    });
}
// Main benchmark configuration
criterion_group! {
    name = benchmarks;
    config = Criterion::default();
    targets = url_parsing_benchmark,
        http_request_creation_benchmark,
        header_operations_benchmark,
        http_transport_test
}
criterion_main!(benchmarks);
