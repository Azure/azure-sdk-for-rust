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
            Ok(())
        }
        async fn cleanup(&self, _context: &TestContext) -> azure_core::Result<()> {
            Ok(())
        }
    }
    // Manually handle the Result instead of using ? because this function does not return a Result.
    let count: Option<&String> = match runner.try_get_test_arg("count") {
        Ok(v) => v,
        Err(e) => {
            // Return a future that immediately yields the error.
            return Box::pin(async move { Err(e) });
        }
    };
    println!("Fibonacci1Test with count: {:?}", count);
    let count = count.expect("count argument is mandatory");
    let count = match count.parse::<u32>() {
        Ok(v) => v,
        Err(e) => {
            let err = azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("invalid count argument: {}", e),
            );
            return Box::pin(async move { Err(err) });
        }
    };
    Box::pin(async move { Ok(Box::new(Fibonacci1Test { count }) as Box<dyn PerfTest>) })
}

#[tokio::test]
async fn test_perf_runner_with_single_test() {
    let args = vec![
        "perf_test",
        "--iterations",
        "1",
        "--parallel",
        "10",
        "--duration",
        "1",
        "--warmup",
        "1",
        "basic_test",
        "-c",
        "10",
    ];
    let runner = PerfRunner::with_command_line(
        env!("CARGO_MANIFEST_DIR"),
        file!(),
        vec![TestMetadata {
            name: "basic_test",
            description: "A basic test for testing purposes",
            options: vec![TestOption {
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
