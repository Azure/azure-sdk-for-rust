// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub(crate) mod clients;
mod download_blobs_test;

use std::process::exit;

use azure_storage_blob_test::stress::{Result, StressRunner, StressTest, StressTestFactory};
use clap::Subcommand;

use crate::download_blobs_test::DownloadBlobsTestArgs;

#[tokio::main]
async fn main() {
    let runner = StressRunner::<StressTests>::new(env!("CARGO_MANIFEST_DIR"), file!());

    if let Err(e) = runner.run().await {
        eprintln!("{}", e);
        exit(1);
    }
}

#[derive(Debug, Subcommand)]
enum StressTests {
    /// Continuously download from a set of blobs.
    Download(DownloadBlobsTestArgs),
}

impl StressTestFactory for StressTests {
    fn build_test(&self) -> Result<Box<dyn StressTest>> {
        match self {
            StressTests::Download(args) => args.as_test(),
        }
    }
}

impl std::fmt::Display for StressTests {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Download(args) => args.fmt(f),
        }
    }
}
