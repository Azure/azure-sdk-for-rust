// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// list_blob performance test.
mod list_blob_test;

use azure_core_test::perf::PerfRunner;
use list_blob_test::ListBlobTest;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let runner = PerfRunner::new(
        env!("CARGO_MANIFEST_DIR"),
        "foo",
        vec![ListBlobTest::test_metadata()],
    )?;

    runner.run().await?;

    Ok(())
}
