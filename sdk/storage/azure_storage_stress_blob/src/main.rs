// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod clients;
mod roundtrip_test;

use std::process::exit;

use azure_storage_stress::{
    args::StressRunnerOptions, Result, StressRunner, StressTest, StressTestFactory,
};
use clap::Subcommand;
use log::{error, info};
use serde::Serialize;

use crate::roundtrip_test::RoundtripBlobsTestArgs;

#[tokio::main]
async fn main() {
    println!("Azure Storage Blobs Stress Test");
    init_logger();

    let runner = StressRunner::<StressTests>::new(env!("CARGO_MANIFEST_DIR"), file!());

    info!("Runner options: {}", runner.options());

    if let Err(e) = runner.run().await {
        error!("{}", e);
        exit(1);
    }
}

#[derive(Debug, Serialize, Subcommand)]
enum StressTests {
    /// Continuously upload then download blobs.
    Roundtrip(RoundtripBlobsTestArgs),
}

impl StressTestFactory for StressTests {
    fn build_test(options: &StressRunnerOptions<Self>) -> Result<Box<dyn StressTest>> {
        match &options.command {
            StressTests::Roundtrip(roundtrip_args) => {
                roundtrip_args.as_test(&options.fault_options()?)
            }
        }
    }
}

impl std::fmt::Display for StressTests {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Roundtrip(args) => args.fmt(f),
        }
    }
}

fn init_logger() {
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stdout)
        .init();
}
