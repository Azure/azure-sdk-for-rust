// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod mock;

use azure_core_test::perf::PerfRunner;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let runner = PerfRunner::new(
        env!("CARGO_MANIFEST_DIR"),
        file!(),
        vec![
            mock::json::MockJsonTest::test_metadata(),
            #[cfg(feature = "xml")]
            mock::xml::MockXmlTest::test_metadata(),
        ],
    )?;

    runner.run().await?;

    Ok(())
}
