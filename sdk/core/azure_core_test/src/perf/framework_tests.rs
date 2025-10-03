// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Tests for functioning of the performance test runner.
//!
//! These tests cover various scenarios for running the `PerfRunner` with different options and measurements.
//!
use super::*;
use std::boxed::Box;

#[tokio::test]
async fn test_perf_runner_with_no_tests() {
    let args = vec!["perf_test", "--iterations", "1", "--duration", "1"];
    let runner =
        PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), vec![], args).unwrap();

    let result = runner.run().await;
    assert!(result.is_err());
}

fn create_fibonacci1_test(runner: &PerfRunner) -> CreatePerfTestReturn {
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
        async fn setup(&self, _context: &TestContext) -> azure_core::Result<()> {
            Ok(())
        }
        async fn run(&self /*, _context: &TestContext*/) -> azure_core::Result<()> {
            let _result = Self::fibonacci(self.count);
            // This is a CPU bound test, so yield to allow other tasks to run. Otherwise we jam the tokio scheduler.
            // Note that this significantly reduces the performance of the test, but it is necessary to allow parallelism.
            //
            // In a real-world scenario, the test would be doing async work (e.g. network I/O) which would yield naturally.
            tokio::task::yield_now().await;
            Ok(())
        }
        async fn cleanup(&self, _context: &TestContext) -> azure_core::Result<()> {
            Ok(())
        }
    }

    // Helper function to handle the async creation of the test.
    async fn create_test(runner: PerfRunner) -> Result<Box<dyn PerfTest>> {
        let count: Option<&String> = runner.try_get_test_arg("count")?;

        println!("Fibonacci1Test with count: {:?}", count);
        let count = count.expect("count argument is mandatory");
        let count = count.parse::<u32>().map_err(|e| {
            azure_core::Error::with_error(
                azure_core::error::ErrorKind::Other,
                e,
                "Invalid count argument",
            )
        })?;
        Ok(Box::new(Fibonacci1Test { count }) as Box<dyn PerfTest>)
    }

    // Return a pinned future that creates the test.
    Box::pin(create_test(runner.clone()))
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
                short_activator: 'c',
                expected_args_len: 1,
                display_message: "The Fibonacci number to compute",
                ..Default::default()
            }],
            create_test: create_fibonacci1_test,
        }],
        args,
    )
    .unwrap();

    let result = runner.run().await;
    assert!(result.is_ok());
    println!("Result: {:?}", result);
}
