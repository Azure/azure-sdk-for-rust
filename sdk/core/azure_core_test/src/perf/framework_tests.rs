// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Tests for functioning of the performance test runner.
//!
//! These tests cover various scenarios for running the `PerfRunner` with different options and measurements.
//!
use clap::Args;
use futures::FutureExt;

use super::*;
use std::boxed::Box;

#[derive(Subcommand, Debug, Clone)]
enum FrameworkTests {
    Fibonacci1(Fibonacci1Args),
}

impl PerfTestFactory for FrameworkTests {
    fn create_test(&self) -> CreatePerfTestReturn {
        match self {
            FrameworkTests::Fibonacci1(args) => create_fibonacci1_test(args.clone()),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            FrameworkTests::Fibonacci1(_) => "fibonacci1",
        }
    }
}
#[derive(Args, Debug, Clone)]
struct Fibonacci1Args {
    #[arg(short, long)]
    count: u32,
}

fn create_fibonacci1_test(args: Fibonacci1Args) -> CreatePerfTestReturn {
    struct Fibonacci1Test {
        args: Fibonacci1Args,
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
            let _result = Self::fibonacci(self.args.count);
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
    async move { Ok(Box::new(Fibonacci1Test { args }) as Box<dyn PerfTest>) }.boxed()
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
    let runner =
        PerfRunner::<FrameworkTests>::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), args)
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
    let runner =
        PerfRunner::<FrameworkTests>::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), args)
            .unwrap();

    // Run directly via run_test_for to inspect the returned latencies.
    let test_mode = crate::TestMode::current_opt()
        .unwrap()
        .unwrap_or(crate::TestMode::Live);
    let mut test_instances: Vec<Arc<dyn PerfTest>> = Vec::new();
    let mut test_contexts: Vec<Arc<TestContext>> = Vec::new();
    for _ in 0..runner.options.parallel {
        let instance = runner.options.subcommand.create_test().await.unwrap();
        let instance: Arc<dyn PerfTest> = Arc::from(instance);
        let context = Arc::new(
            crate::recorded::start(
                test_mode,
                runner.package_dir,
                runner.module_name,
                runner.options.subcommand.name(),
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
