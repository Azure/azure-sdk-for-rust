// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Tests for configuration of the performance test runner.
//!
//! These tests cover various scenarios for initializing the `PerfRunner` with different sets of
//! command-line arguments and test metadata. They ensure that the runner correctly parses
//! arguments, handles defaults, and manages errors appropriately.
//!
use super::*;
use std::{env, error::Error};

fn create_failed_test(_runner: &PerfRunner) -> CreatePerfTestReturn {
    Box::pin(async {
        Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "Intentional failure to create test instance",
        ))
    })
}

// Helper function to create a basic test metadata for testing
fn create_basic_test_metadata() -> PerfTestMetadata {
    PerfTestMetadata {
        name: "basic_test",
        description: "A basic test for testing purposes",
        options: vec![PerfTestOption {
            name: "test-option",
            short_activator: 't',
            long_activator: "test-option",
            display_message: "Test option for basic test",
            expected_args_len: 1,
            mandatory: false,
            sensitive: false,
        }],
        create_test: create_failed_test,
    }
}

// Helper function to create test metadata with multiple options
fn create_complex_test_metadata() -> PerfTestMetadata {
    PerfTestMetadata {
        name: "complex_test",
        description: "A complex test with multiple options",
        options: vec![
            PerfTestOption {
                name: "mandatory-option",
                short_activator: 'm',
                long_activator: "mandatory",
                display_message: "Mandatory option",
                expected_args_len: 1,
                mandatory: true,
                sensitive: false,
            },
            PerfTestOption {
                name: "sensitive-option",
                short_activator: 's',
                long_activator: "sensitive",
                display_message: "Sensitive option",
                expected_args_len: 1,
                mandatory: false,
                sensitive: true,
            },
            PerfTestOption {
                name: "flag-option",
                short_activator: 'f',
                long_activator: "flag",
                display_message: "Flag option",
                ..Default::default()
            },
        ],
        create_test: create_failed_test,
    }
}

// Helper function to create test metadata without short activators
fn create_no_short_activator_test_metadata() -> PerfTestMetadata {
    PerfTestMetadata {
        name: "no_short_test",
        description: "Test without short activators",
        options: vec![PerfTestOption {
            name: "long-only",
            short_activator: '\0',
            long_activator: "long-only",
            display_message: "Long activator only",
            expected_args_len: 1,
            mandatory: false,
            sensitive: false,
        }],
        create_test: create_failed_test,
    }
}

#[test]
fn test_perf_runner_new_with_empty_tests() {
    let tests = vec![];
    let result = PerfRunner::with_command_line(
        env!("CARGO_MANIFEST_DIR"),
        file!(),
        tests,
        vec!["perf-tests"],
    );

    assert!(
        result.is_ok(),
        "PerfRunner::new should succeed with empty tests"
    );
    let runner = result.unwrap();

    // Test default values
    assert_eq!(runner.options.iterations, 1);
    assert_eq!(runner.options.parallel, 1);
    assert_eq!(runner.options.duration, Duration::seconds(30));
    assert_eq!(runner.options.warmup, Duration::seconds(5));
    assert_eq!(runner.options.test_results_filename, "./tests/results.json");
    assert!(!runner.options.no_cleanup);
}

#[test]
fn test_perf_runner_new_with_single_test() {
    let tests = vec![create_basic_test_metadata()];
    let result = PerfRunner::with_command_line(
        env!("CARGO_MANIFEST_DIR"),
        file!(),
        tests,
        vec!["perf-tests"],
    );

    assert!(
        result.is_ok(),
        "PerfRunner::new should succeed with single test"
    );
    let runner = result.unwrap();

    // Verify default values are set
    assert_eq!(runner.options.iterations, 1);
    assert_eq!(runner.options.parallel, 1);
    assert_eq!(runner.options.duration, Duration::seconds(30));
    assert_eq!(runner.options.warmup, Duration::seconds(5));
}

#[test]
fn test_perf_runner_new_with_multiple_tests() {
    let tests = vec![
        create_basic_test_metadata(),
        create_complex_test_metadata(),
        create_no_short_activator_test_metadata(),
    ];
    let result = PerfRunner::with_command_line(
        env!("CARGO_MANIFEST_DIR"),
        file!(),
        tests,
        vec!["perf-tests"],
    );

    assert!(
        result.is_ok(),
        "PerfRunner::new should succeed with multiple tests"
    );
    let _runner = result.unwrap();
}

#[test]
fn test_perf_runner_with_command_line_default_args() {
    let tests = vec![create_basic_test_metadata()];
    let args = vec!["perf-tests"];

    let result = PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args);
    assert!(
        result.is_ok(),
        "PerfRunner::with_command_line should succeed with default args"
    );

    let runner = result.unwrap();
    assert_eq!(runner.options.iterations, 1);
    assert_eq!(runner.options.parallel, 1);
    assert_eq!(runner.options.duration, Duration::seconds(30));
    assert_eq!(runner.options.warmup, Duration::seconds(5));
    assert!(!runner.options.no_cleanup);
}

#[test]
fn test_perf_runner_with_command_line_custom_iterations() {
    let tests = vec![create_basic_test_metadata()];
    let args = vec!["perf-tests", "--iterations", "10"];

    let result = PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args);
    assert!(
        result.is_ok(),
        "PerfRunner::with_command_line should succeed with custom iterations"
    );

    let runner = result.unwrap();
    assert_eq!(runner.options.iterations, 10);
}

#[test]
fn test_perf_runner_with_command_line_custom_parallel() {
    let tests = vec![create_basic_test_metadata()];
    let args = vec!["perf-tests", "--parallel", "5"];

    let result = PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args);
    assert!(
        result.is_ok(),
        "PerfRunner::with_command_line should succeed with custom parallel"
    );

    let runner = result.unwrap();
    assert_eq!(runner.options.parallel, 5);
}

#[test]
fn test_perf_runner_with_command_line_custom_duration() {
    let tests = vec![create_basic_test_metadata()];
    let args = vec!["perf-tests", "--duration", "60"];

    let result = PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args);
    assert!(
        result.is_ok(),
        "PerfRunner::with_command_line should succeed with custom duration"
    );

    let runner = result.unwrap();
    assert_eq!(runner.options.duration, Duration::seconds(60));
}

#[test]
fn test_perf_runner_with_command_line_custom_warmup() {
    let tests = vec![create_basic_test_metadata()];
    let args = vec!["perf-tests", "--warmup", "10"];

    let result = PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args);
    assert!(
        result.is_ok(),
        "PerfRunner::with_command_line should succeed with custom warmup"
    );

    let runner = result.unwrap();
    assert_eq!(runner.options.warmup, Duration::seconds(10));
}

#[test]
fn test_perf_runner_with_command_line_test_results_file() {
    let tests = vec![create_basic_test_metadata()];
    let args = vec!["perf-tests", "--test-results", "/tmp/results.json"];

    let result = PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args);
    assert!(
        result.is_ok(),
        "PerfRunner::with_command_line should succeed with custom test results file"
    );

    let runner = result.unwrap();
    assert_eq!(runner.options.test_results_filename, "/tmp/results.json");
}

#[test]
fn test_perf_runner_with_command_line_no_cleanup() {
    let tests = vec![create_basic_test_metadata()];
    let args = vec!["perf-tests", "--no-cleanup"];

    let result = PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args);
    assert!(
        result.is_ok(),
        "PerfRunner::with_command_line should succeed with no-cleanup flag"
    );

    let runner = result.unwrap();
    assert!(runner.options.no_cleanup);
}

#[test]
fn test_perf_runner_with_command_line_all_options() {
    let tests = vec![create_basic_test_metadata()];
    let args = vec![
        "perf-tests",
        "--iterations",
        "20",
        "--parallel",
        "8",
        "--duration",
        "120",
        "--warmup",
        "15",
        "--test-results",
        "/custom/results.json",
        "--no-cleanup",
    ];

    let result = PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args);
    assert!(
        result.is_ok(),
        "PerfRunner::with_command_line should succeed with all options"
    );

    let runner = result.unwrap();
    assert_eq!(runner.options.iterations, 20);
    assert_eq!(runner.options.parallel, 8);
    assert_eq!(runner.options.duration, Duration::seconds(120));
    assert_eq!(runner.options.warmup, Duration::seconds(15));
    assert_eq!(runner.options.test_results_filename, "/custom/results.json");
    assert!(runner.options.no_cleanup);
}

#[test]
fn test_perf_runner_command_line_help() {
    let tests = vec![create_basic_test_metadata()];
    let args = vec!["perf-tests", "--help"];

    let result = PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args);
    assert!(
        result.is_err(),
        "PerfRunner::with_command_line should fail with help flag"
    );

    println!("{}", result.as_ref().err().unwrap().source().unwrap());

    let error = result.err().unwrap();
    assert_eq!(error.kind(), &azure_core::error::ErrorKind::Other);
    assert!(error.to_string().contains("Failed to parse"));
}

#[test]
fn test_perf_runner_with_subcommand() {
    let tests = vec![create_basic_test_metadata()];
    let args = vec!["perf-tests", "basic_test", "--test-option", "value"];

    let result = PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args);
    assert!(
        result.is_ok(),
        "PerfRunner::with_command_line should succeed with subcommand"
    );

    let runner = result.unwrap();

    let selected_test = runner
        .get_selected_test_name()
        .expect("A test should be selected");
    assert_eq!(selected_test, "basic_test");
    let option_value: Option<&String> = runner.try_get_test_arg("test-option").ok().flatten();
    assert!(option_value.is_some());
    assert_eq!(option_value.unwrap(), "value");
}

#[test]
fn test_perf_runner_with_subcommand_short_activator() {
    let tests = vec![create_basic_test_metadata()];
    let args = vec!["perf-tests", "basic_test", "-t", "short_value"];

    let result = PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args);
    assert!(
        result.is_ok(),
        "PerfRunner::with_command_line should succeed with short activator"
    );

    let runner = result.unwrap();
    let option_value: Option<&String> = runner.try_get_test_arg("test-option").ok().flatten();
    assert!(option_value.is_some());
    assert_eq!(option_value.unwrap(), "short_value");
}

#[test]
fn test_perf_runner_with_complex_subcommand() {
    let tests = vec![create_complex_test_metadata()];
    let args = vec![
        "perf-tests",
        "complex_test",
        "--mandatory",
        "required_value",
        "--sensitive",
        "secret_value",
        "--flag",
    ];

    println!(
        "Help: {}",
        PerfRunner::with_command_line(
            env!("CARGO_MANIFEST_DIR"),
            file!(),
            tests.clone(),
            vec!["perf-tests", "--help"]
        )
        .unwrap_err()
        .source()
        .unwrap()
    );
    println!(
        "Help2 : {}",
        PerfRunner::with_command_line(
            env!("CARGO_MANIFEST_DIR"),
            file!(),
            tests.clone(),
            vec!["perf-tests", "complex_test", "--help"]
        )
        .unwrap_err()
        .source()
        .unwrap()
    );

    let result = PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args);
    assert!(
        result.is_ok(),
        "PerfRunner::with_command_line should succeed with complex subcommand"
    );

    let runner = result.unwrap();

    let mandatory_value: Result<Option<&String>> = runner.try_get_test_arg("mandatory-option");
    println!("{:?}", mandatory_value);
    assert!(mandatory_value.is_ok());
    let mandatory_value = mandatory_value.unwrap();
    assert!(mandatory_value.is_some());
    assert_eq!(mandatory_value.unwrap(), "required_value");

    let sensitive_value: Option<&String> =
        runner.try_get_test_arg("sensitive-option").ok().flatten();
    assert!(sensitive_value.is_some());
    assert_eq!(sensitive_value.unwrap(), "secret_value");

    let flag_value = runner.try_get_test_arg("flag-option").ok().flatten();
    assert!(flag_value.is_some());
    let flag_value: bool = *flag_value.unwrap();
    assert!(flag_value);
}

#[test]
fn test_perf_runner_with_no_short_activator() {
    let tests = vec![create_no_short_activator_test_metadata()];
    let args = vec!["perf-tests", "no_short_test", "--long-only", "value"];

    let result = PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args);
    assert!(
        result.is_ok(),
        "PerfRunner::with_command_line should succeed with long-only activator"
    );

    let runner = result.unwrap();
    let option_value: Option<&String> = runner.try_get_test_arg("long-only").ok().flatten();
    assert!(option_value.is_some());
    assert_eq!(option_value.unwrap(), "value");
}

#[test]
fn test_perf_runner_get_one_nonexistent() {
    let tests = vec![create_basic_test_metadata()];
    let args = vec!["perf-tests"];

    let runner =
        PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args).unwrap();
    let result: Result<Option<&String>> = runner.try_get_global_arg("nonexistent");
    assert!(result.is_err());
}

#[test]
fn test_perf_runner_get_one_different_types() {
    let tests = vec![create_basic_test_metadata()];
    let args = vec!["perf-tests", "--iterations", "42"];

    let runner =
        PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args).unwrap();

    // Test getting u32 value
    let iterations: Option<&u32> = runner.try_get_global_arg("iterations").ok().flatten();
    assert!(iterations.is_some());
    assert_eq!(*iterations.unwrap(), 42);

    // Test getting wrong type returns None
    let iterations_as_string: Option<&String> =
        runner.try_get_global_arg("iterations").ok().flatten();
    assert!(iterations_as_string.is_none());
}

#[test]
fn test_perf_runner_options_debug() {
    let tests = vec![create_basic_test_metadata()];
    let args = vec!["perf-tests", "--iterations", "5"];

    let runner =
        PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args).unwrap();

    // Test that Debug is implemented for PerfRunner
    let debug_output = format!("{:?}", runner);
    assert!(debug_output.contains("PerfRunner"));
    assert!(debug_output.contains("options"));

    // Test that PerfRunnerOptions Debug works
    let options_debug = format!("{:?}", runner.options);
    assert!(options_debug.contains("PerfRunnerOptions"));
    assert!(options_debug.contains("iterations: 5"));

    let options = PerfRunnerOptions::from(&runner.arguments);
    assert_eq!(options.iterations, 5);
}

#[test]
fn test_test_option_debug_and_default() {
    let option = PerfTestOption::default();

    // Test default values
    assert_eq!(option.name, "");
    assert_eq!(option.short_activator, '\0');
    assert_eq!(option.long_activator, "");
    assert_eq!(option.display_message, "");
    assert_eq!(option.expected_args_len, 0);
    assert!(!option.mandatory);
    assert!(!option.sensitive);

    // Test Debug implementation
    let debug_output = format!("{:?}", option);
    assert!(debug_output.contains("TestOption"));
}

#[test]
fn test_perf_runner_with_invalid_numeric_value() {
    let tests = vec![create_basic_test_metadata()];
    let args = vec!["perf-tests", "--iterations", "not_a_number"];

    let result = PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args);
    assert!(
        result.is_err(),
        "PerfRunner::with_command_line should fail with invalid numeric value"
    );
}

#[test]
fn test_perf_runner_with_missing_mandatory_option() {
    let tests = vec![create_complex_test_metadata()];
    let args = vec!["perf-tests", "complex_test"]; // Missing mandatory option

    let result = PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args);
    assert!(
        result.is_err(),
        "PerfRunner::with_command_line should fail with missing mandatory option"
    );
}

#[test]
fn test_perf_runner_with_multiple_tests_and_subcommands() {
    let tests = vec![create_basic_test_metadata(), create_complex_test_metadata()];

    // Test with first subcommand
    let args = vec!["perf-tests", "basic_test", "--test-option", "value1"];
    let result =
        PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests.clone(), args);
    assert!(result.is_ok());

    let runner = result.unwrap();
    let option_value: Option<&String> = runner.try_get_test_arg("test-option").ok().flatten();
    assert_eq!(option_value.unwrap(), "value1");

    // Test with second subcommand
    let args = vec!["perf-tests", "complex_test", "--mandatory", "required"];
    let result = PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args);
    assert!(result.is_ok());

    let runner = result.unwrap();
    let mandatory_value: Option<&String> =
        runner.try_get_test_arg("mandatory-option").ok().flatten();
    assert_eq!(mandatory_value.unwrap(), "required");
}

struct ComplexTest {}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl PerfTest for ComplexTest {
    async fn setup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        println!("Setting up ComplexTest...");
        // Simulate some async setup work
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        Ok(())
    }

    async fn cleanup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        println!("Cleaning up ComplexTest...");
        // Simulate some async cleanup work
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        Ok(())
    }

    async fn run(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        // Simulate some async test work
        println!("Running ComplexTest...");
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        Ok(())
    }
}

fn complex_test_create(_runner: &PerfRunner) -> CreatePerfTestReturn {
    Box::pin(async { Ok(Box::new(ComplexTest {}) as Box<dyn PerfTest>) })
}

#[tokio::test]
async fn test_perf_runner_with_test_functions() {
    let tests = vec![PerfTestMetadata {
        name: "complex_test",
        description: "A complex test with multiple options",
        options: vec![
            PerfTestOption {
                name: "mandatory-option",
                short_activator: 'm',
                long_activator: "mandatory",
                display_message: "Mandatory option",
                expected_args_len: 1,
                mandatory: true,
                sensitive: false,
            },
            PerfTestOption {
                name: "sensitive-option",
                short_activator: 's',
                long_activator: "sensitive",
                display_message: "Sensitive option",
                expected_args_len: 1,
                mandatory: false,
                sensitive: true,
            },
            PerfTestOption {
                name: "flag-option",
                short_activator: 'f',
                long_activator: "flag",
                display_message: "Flag option",
                expected_args_len: 0,
                mandatory: false,
                sensitive: false,
            },
        ],
        create_test: complex_test_create,
    }];
    let args = vec![
        "perf-tests",
        "complex_test",
        "--mandatory",
        "required_value",
        "--sensitive",
        "secret_value",
        "--flag",
    ];

    let result = PerfRunner::with_command_line(env!("CARGO_MANIFEST_DIR"), file!(), tests, args);
    assert!(
        result.is_ok(),
        "PerfRunner::with_command_line should succeed with complex subcommand"
    );

    let runner = result.unwrap();

    let mandatory_value: Result<Option<&String>> = runner.try_get_test_arg("mandatory-option");
    println!("{:?}", mandatory_value);
    assert!(mandatory_value.is_ok());
    let mandatory_value = mandatory_value.unwrap();
    assert!(mandatory_value.is_some());
    assert_eq!(mandatory_value.unwrap(), "required_value");

    let sensitive_value: Option<&String> =
        runner.try_get_test_arg("sensitive-option").ok().flatten();
    assert!(sensitive_value.is_some());
    assert_eq!(sensitive_value.unwrap(), "secret_value");

    let flag_value = runner.try_get_test_arg("flag-option").ok().flatten();
    assert!(flag_value.is_some());
    let flag_value: bool = *flag_value.unwrap();
    assert!(flag_value);

    let perf_tests_impl = (runner.tests[0].create_test)(&runner)
        .await
        .expect("Failed to create test instance");

    let crate_dir = env!("CARGO_MANIFEST_DIR");

    let test_context = Arc::new(
        TestContext::new(crate_dir, crate_dir, runner.tests[0].name)
            .expect("Failed to create TestContext"),
    );

    perf_tests_impl
        .setup(test_context.clone())
        .await
        .expect("Setup failed");
    perf_tests_impl
        .run(test_context.clone())
        .await
        .expect("Run failed");
    perf_tests_impl
        .cleanup(test_context.clone())
        .await
        .expect("Cleanup failed");
}
