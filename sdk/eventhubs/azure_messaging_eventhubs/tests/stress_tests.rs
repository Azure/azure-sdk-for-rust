// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Main stress test runner for Azure Event Hubs
//!
//! This binary runs stress tests with a custom harness (harness = false). Each
//! stress test registers its own CLI options and runner so adding new tests
//! only requires updating the registry below.

use clap::{ArgMatches, Command};
use futures::future::BoxFuture;
use std::{error::Error, process};

// Load the stress tests modules from the stress_tests directory
#[path = "stress_tests/basic_publish_read_test.rs"]
mod basic_publish_read_test;
#[path = "stress_tests/continuous_send_receive_stress.rs"]
mod continuous_send_receive_stress;

use basic_publish_read_test::basic_publish_read_spec;
use continuous_send_receive_stress::continuous_send_receive_spec;

type StressTestFuture = BoxFuture<'static, Result<(), Box<dyn Error + Send + Sync>>>;

#[derive(Clone)]
struct StressTestSpec {
    name: &'static str,
    description: &'static str,
    configure: fn(Command) -> Command,
    run: fn(ArgMatches) -> StressTestFuture,
}

#[tokio::main]
async fn main() {
    // Initialize tracing for test output
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let matches = build_cli().get_matches();

    if let Some((subcommand, sub_matches)) = matches.subcommand() {
        if let Some(spec) = registry().into_iter().find(|s| s.name == subcommand) {
            if let Err(e) = (spec.run)(sub_matches.clone()).await {
                eprintln!("{} FAILED: {}", spec.name, e);
                process::exit(1);
            }
            return;
        }

        eprintln!("Unknown test: {}", subcommand);
        process::exit(1);
    }

    // No subcommand: run all with default args
    println!("Running all stress tests with default settings...");
    for spec in registry() {
        let default_matches = (spec.configure)(Command::new(spec.name))
            .no_binary_name(true)
            .get_matches_from(Vec::<&str>::new());

        if let Err(e) = (spec.run)(default_matches).await {
            eprintln!("{} FAILED: {}", spec.name, e);
            process::exit(1);
        }
    }
}

fn registry() -> Vec<StressTestSpec> {
    vec![basic_publish_read_spec(), continuous_send_receive_spec()]
}

fn build_cli() -> Command {
    let mut cmd = Command::new("stress_tests")
        .about("Azure Event Hubs Stress Tests")
        .long_about(
            "This binary runs stress tests for Azure Event Hubs with configurable parameters.",
        );

    for spec in registry() {
        let sub = (spec.configure)(Command::new(spec.name).about(spec.description));
        cmd = cmd.subcommand(sub);
    }

    cmd
}
