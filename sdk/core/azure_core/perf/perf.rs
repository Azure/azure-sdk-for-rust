// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod mock;

use std::fmt::Display;

use azure_core_test::perf::{PerfRunner, PerfTestFactory};
use clap::Subcommand;

use crate::mock::json::{self, MockJsonTestArgs};
#[cfg(feature = "xml")]
use crate::mock::xml::{self, MockXmlTestArgs};

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let runner = PerfRunner::<Tests>::new(env!("CARGO_MANIFEST_DIR"), file!())?;
    runner.run().await?;
    Ok(())
}

#[derive(Subcommand, Debug, Clone)]
enum Tests {
    Json(MockJsonTestArgs),
    #[cfg(feature = "xml")]
    Xml(MockXmlTestArgs),
}

impl PerfTestFactory for Tests {
    fn name(&self) -> &'static str {
        match self {
            Tests::Json(_) => "mock_json",
            #[cfg(feature = "xml")]
            Tests::Xml(_) => "mock_xml",
        }
    }

    fn create_test(&self) -> azure_core_test::perf::CreatePerfTestReturn {
        match &self {
            Tests::Json(args) => json::create_test(args),
            #[cfg(feature = "xml")]
            Tests::Xml(args) => xml::create_test(args),
        }
    }
}
