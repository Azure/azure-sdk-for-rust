// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Tests for functioning of the performance test runner.
//!
//! These tests cover various scenarios for running the `PerfRunner` with different options and measurements.
//!
use futures::FutureExt;

use super::*;
use std::boxed::Box;

fn create_fibonacci1_test(runner: PerfRunner) -> CreatePerfTestReturn {
    struct Fibonacci1Test {
        count: u32,
    }

    impl Fibonacci1Test {
        fn fibonacci(n: u32) -> u32 {
            if n <= 1 {
                n
            } else {
                Self::fibonacci(n - 1) + Self::fibonacci(n - 2)
            }
        }
    }

    #[async_trait::async_trait]
    impl PerfTest for Fibonacci1Test {
        async fn setup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
            Ok(())
        }
        async fn run(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
            let _result = Self::fibonacci(self.count);
            // This is a CPU bound test, so yield to allow other tasks to run. Otherwise we jam the tokio scheduler.
            // Note that this significantly reduces the performance of the test, but it is necessary to allow parallelism.
            //
            // In a real-world scenario, the test would be doing async work (e.g. network I/O) which would yield naturally.
            tokio::task::yield_now().await;
            Ok(())
        }
        async fn cleanup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
            Ok(())
        }
    }

    // Return a pinned future that creates the test.
    async move {
        Ok(Box::new(Fibonacci1Test {
            count: runner.try_get_test_arg("count")?.unwrap(),
        }) as Box<dyn PerfTest>)
    }
    .boxed()
}

#[tokio::test]
async fn test_perf_runner_with_single_test() {
    let args = vec![
        "perf_test",
        "--iterations",
        "1",
        "--parallel",
        "30",
        "--duration",
        "10",
        "--test-results",
        "",
        "--warmup",
        "1",
        "fibonacci1",
        "-c",
        "10",
    ];
    let runner = PerfRunner::with_command_line(
        env!("CARGO_MANIFEST_DIR"),
        file!(),
        vec![PerfTestMetadata {
            name: "fibonacci1",
            description: "A basic test for testing purposes",
            options: vec![PerfTestOption {
                name: "count",
                mandatory: true,
                short_activator: Some('c'),
                expected_args_len: 1,
                display_message: "The Fibonacci number to compute",
                option_type: PerfTestOptionKind::Uint32,
                ..Default::default()
            }],
            create_test: create_fibonacci1_test,
        }],
        args,
    )
    .unwrap();

    let result = runner.run().await;
    println!("Result: {:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_latency_collection_returns_values() {
    let args = vec![
        "perf_test",
        "--parallel",
        "2",
        "--duration",
        "1",
        "--warmup",
        "0",
        "--test-results",
        "",
        "--latency",
        "fibonacci1",
        "-c",
        "5",
    ];
    let runner = PerfRunner::with_command_line(
        env!("CARGO_MANIFEST_DIR"),
        file!(),
        vec![PerfTestMetadata {
            name: "fibonacci1",
            description: "A basic test for testing purposes",
            options: vec![PerfTestOption {
                name: "count",
                mandatory: true,
                short_activator: Some('c'),
                expected_args_len: 1,
                display_message: "The Fibonacci number to compute",
                option_type: PerfTestOptionKind::Uint32,
                ..Default::default()
            }],
            create_test: create_fibonacci1_test,
        }],
        args,
    )
    .unwrap();

    // Run directly via run_test_for to inspect the returned latencies.
    let test_mode = crate::TestMode::current_opt()
        .unwrap()
        .unwrap_or(crate::TestMode::Live);
    let mut test_instances: Vec<Arc<dyn PerfTest>> = Vec::new();
    let mut test_contexts: Vec<Arc<TestContext>> = Vec::new();
    for _ in 0..runner.options.parallel {
        let instance = (runner.tests[0].create_test)(runner.clone()).await.unwrap();
        let instance: Arc<dyn PerfTest> = Arc::from(instance);
        let context = Arc::new(
            crate::recorded::start(
                test_mode,
                runner.package_dir,
                runner.module_name,
                runner.tests[0].name,
                None,
            )
            .await
            .unwrap(),
        );
        instance.setup(context.clone()).await.unwrap();
        test_instances.push(instance);
        test_contexts.push(context);
    }

    let (ops_per_second, latencies) = runner
        .run_test_for(&test_instances, &test_contexts, Duration::seconds(1), true)
        .await
        .unwrap();

    assert!(
        ops_per_second > 0.0,
        "Should have completed some operations"
    );
    assert!(
        !latencies.is_empty(),
        "Latencies should be collected when track_latency is true"
    );

    // Every latency should be non-zero.
    for lat in &latencies {
        assert!(
            !lat.is_zero(),
            "Each latency measurement should be non-zero"
        );
    }
}
