// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod download_blob_test;
/// list_blob performance test.
mod list_blob_test;
mod upload_blob_test;

use azure_core_test::perf::PerfRunner;
use download_blob_test::DownloadBlobTest;
use list_blob_test::ListBlobTest;
use upload_blob_test::UploadBlobTest;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let runner = PerfRunner::new(
        env!("CARGO_MANIFEST_DIR"),
        file!(),
        vec![
            ListBlobTest::test_metadata(),
            UploadBlobTest::test_metadata(),
            DownloadBlobTest::test_metadata(),
        ],
    )?;

    runner.run().await?;

    Ok(())
}
