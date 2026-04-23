// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Tests for functioning of the stress test runner.
//!
//! These tests cover various scenarios for running the `StressRunner` with different options and measurements.
//!

use async_trait::async_trait;
use clap::Args;

use super::*;

#[derive(Debug, Subcommand)]
enum MockStressTestFactory {
    TestOne(MockArgsOne),
    TestTwo(MockArgsTwo),
}

impl StressTestFactory for MockStressTestFactory {
    fn build_test(&self) -> Result<Box<dyn StressTest>> {
        Ok(match self {
            Self::TestOne(_args) => Box::new(MockStressTestOne {}),
            Self::TestTwo(_args) => Box::new(MockStressTestTwo {}),
        })
    }
}

impl std::fmt::Display for MockStressTestFactory {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

struct MockStressTestOne {}
#[derive(Args, Debug)]
struct MockArgsOne {}
#[async_trait]
impl StressTest for MockStressTestOne {
    async fn global_setup(&self) -> Result<()> {
        Ok(())
    }
    async fn get_operation(&self) -> Result<Box<dyn StressTestOperation>> {
        Ok(Box::new(MockOperationOne {}))
    }
    async fn global_cleanup(&self) -> Result<()> {
        Ok(())
    }
}
struct MockOperationOne {}
#[async_trait]
impl StressTestOperation for MockOperationOne {
    async fn run(&mut self, _result_sender: UnboundedSender<StressRunOutput>) {
        // todo!()
    }
}

struct MockStressTestTwo {}
#[derive(Args, Debug)]
struct MockArgsTwo {
    #[arg(short, long)]
    a_mandatory: usize,
    #[arg(short, long, default_value_t = 1)]
    b_has_default: usize,
    #[arg(short, long)]
    c_optional: Option<usize>,
    #[arg(short, long)]
    d_flag: bool,
}
#[async_trait]
impl StressTest for MockStressTestTwo {
    async fn global_setup(&self) -> Result<()> {
        Ok(())
    }
    async fn get_operation(&self) -> Result<Box<dyn StressTestOperation>> {
        Ok(Box::new(MockOperationTwo {}))
    }
    async fn global_cleanup(&self) -> Result<()> {
        Ok(())
    }
}
struct MockOperationTwo {}
#[async_trait]
impl StressTestOperation for MockOperationTwo {
    async fn run(&mut self, _result_sender: UnboundedSender<StressRunOutput>) {
        // todo!()
    }
}

#[tokio::test]
async fn test_parse_minimum_runner_args() {
    let args = vec!["stress_test", "--output", "", "test-one"];
    let result =
        StressRunner::<MockStressTestFactory>::from_args(env!("CARGO_MANIFEST_DIR"), file!(), args);

    println!("Result: {:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_parse_missing_required_runner_arg() {
    let args = vec!["stress_test", "test-one"];
    let result =
        StressRunner::<MockStressTestFactory>::from_args(env!("CARGO_MANIFEST_DIR"), file!(), args);

    println!("Result: {:?}", result);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_parse_unrecognized_runner_arg() {
    let args = vec!["stress_test", "-o", "", "--bad-arg", "test-one"];
    let result =
        StressRunner::<MockStressTestFactory>::from_args(env!("CARGO_MANIFEST_DIR"), file!(), args);

    println!("Result: {:?}", result);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_parse_all_runner_args() {
    let args = vec![
        "stress_test",
        "--parallel",
        "5",
        "--duration",
        "10",
        "--timeout",
        "60",
        "--output",
        "",
        "test-one",
    ];
    let result =
        StressRunner::<MockStressTestFactory>::from_args(env!("CARGO_MANIFEST_DIR"), file!(), args);

    println!("Result: {:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_parse_missing_subcommand() {
    let args = vec!["stress_test", "--output", ""];
    let result =
        StressRunner::<MockStressTestFactory>::from_args(env!("CARGO_MANIFEST_DIR"), file!(), args);

    println!("Result: {:?}", result);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_parse_subcommand_minimum_args() {
    let args = vec!["stress_test", "--output", "", "test-two", "-a", "5"];
    let result =
        StressRunner::<MockStressTestFactory>::from_args(env!("CARGO_MANIFEST_DIR"), file!(), args);

    println!("Result: {:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_parse_subcommand_all_args() {
    let args = vec![
        "stress_test",
        "--output",
        "",
        "test-two",
        "-a",
        "5",
        "-b",
        "10",
        "-c",
        "15",
        "-d",
    ];
    let result =
        StressRunner::<MockStressTestFactory>::from_args(env!("CARGO_MANIFEST_DIR"), file!(), args);

    println!("Result: {:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_parse_subcommand_unrecognized_arg() {
    let args = vec![
        "stress_test",
        "--output",
        "",
        "test-two",
        "-a",
        "5",
        "--bad-arg",
        "foo",
    ];
    let result =
        StressRunner::<MockStressTestFactory>::from_args(env!("CARGO_MANIFEST_DIR"), file!(), args);

    println!("Result: {:?}", result);
    assert!(result.is_err());
}
