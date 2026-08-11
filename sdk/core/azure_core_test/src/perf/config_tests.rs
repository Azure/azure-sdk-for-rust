// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Tests for configuration of the performance test runner.

use clap::Args;
use futures::FutureExt;

use super::*;

#[derive(Subcommand, Debug, Clone)]
enum ConfigTests {
    Basic(BasicArgs),
}

impl PerfTestFactory for ConfigTests {
    fn create_test(&self) -> CreatePerfTestReturn {
        async move { Ok(Box::new(NoopPerfTest) as Box<dyn PerfTest>) }.boxed()
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Basic(_) => "basic",
        }
    }
}

#[derive(Args, Debug, Clone, Default)]
struct BasicArgs {}

struct NoopPerfTest;

#[async_trait::async_trait]
impl PerfTest for NoopPerfTest {
    async fn setup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        Ok(())
    }

    async fn run(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        Ok(())
    }

    async fn cleanup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        Ok(())
    }
}

#[test]
fn perf_runner_parses_default_options() {
    let runner = PerfRunner::<ConfigTests>::with_command_line(
        env!("CARGO_MANIFEST_DIR"),
        file!(),
        vec!["perf-tests", "basic"],
    )
    .unwrap();

    assert_eq!(runner.options.iterations, 1);
    assert_eq!(runner.options.parallel, 1);
    assert_eq!(runner.options.duration, Duration::seconds(30));
    assert_eq!(runner.options.warmup, Duration::seconds(5));
    assert_eq!(runner.options.test_results_filename, "./results.json");
    assert_eq!(runner.options.results_file, "");
    assert!(!runner.options.no_cleanup);
    assert!(!runner.options.disable_progress);
    assert!(!runner.options.latency);
    assert!(runner.options.test_proxy.is_none());
    assert!(matches!(runner.options.subcommand, ConfigTests::Basic(_)));
}

#[test]
fn perf_runner_parses_custom_options() {
    let runner = PerfRunner::<ConfigTests>::with_command_line(
        env!("CARGO_MANIFEST_DIR"),
        file!(),
        vec![
            "perf-tests",
            "--iterations",
            "10",
            "--parallel",
            "5",
            "--duration",
            "60",
            "--warmup",
            "10",
            "--no-cleanup",
            "--no-progress",
            "--latency",
            "--test-results",
            "/tmp/results.json",
            "--results-file",
            "/tmp/latencies.json",
            "--test-proxy",
            "https://example.com",
            "basic",
        ],
    )
    .unwrap();

    assert_eq!(runner.options.iterations, 10);
    assert_eq!(runner.options.parallel, 5);
    assert_eq!(runner.options.duration, Duration::seconds(60));
    assert_eq!(runner.options.warmup, Duration::seconds(10));
    assert_eq!(runner.options.test_results_filename, "/tmp/results.json");
    assert_eq!(runner.options.results_file, "/tmp/latencies.json");
    assert!(runner.options.no_cleanup);
    assert!(runner.options.disable_progress);
    assert!(runner.options.latency);
    assert_eq!(
        runner.options.test_proxy.as_ref().map(Url::as_str),
        Some("https://example.com/")
    );
}

#[test]
fn perf_runner_rejects_invalid_duration() {
    let error = PerfRunner::<ConfigTests>::with_command_line(
        env!("CARGO_MANIFEST_DIR"),
        file!(),
        vec!["perf-tests", "--duration", "invalid", "basic"],
    )
    .unwrap_err();

    assert_eq!(error.kind(), clap::error::ErrorKind::ValueValidation);
}
