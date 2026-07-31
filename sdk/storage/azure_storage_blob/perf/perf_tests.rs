// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod clap_parsers;
mod download_blob_test;
mod download_into_blob_test;
mod extensions;
/// list_blob performance test.
mod list_blob_test;
mod upload_blob_test;

use azure_core::error::ResultExt;
use azure_core_test::perf::{
    CreatePerfTestReturn, PeekSubcommandInfo, PerfRunner, PerfTestFactory,
};
use clap::Subcommand;
use download_blob_test::DownloadBlobTestOptions;
use list_blob_test::ListBlobTestOptions;
use upload_blob_test::UploadBlobTestOptions;

use crate::download_into_blob_test::DownloadIntoBlobTestOptions;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    match PerfRunner::<BlobTest>::new(env!("CARGO_MANIFEST_DIR"), file!()) {
        Ok(runner) => runner.run().await,
        Err(e) => e.print().with_context(
            azure_core_test::ErrorKind::Other,
            "Failed to print parser error",
        ),
    }
}

#[derive(Subcommand, Clone, Debug)]
enum BlobTest {
    Upload(UploadBlobTestOptions),
    Download(DownloadBlobTestOptions),
    DownloadInto(DownloadIntoBlobTestOptions),
    List(ListBlobTestOptions),
}

impl PerfTestFactory for BlobTest {
    fn name(&self) -> &'static str {
        match self {
            BlobTest::Upload(_) => "UploadBlob",
            BlobTest::Download(_) => "DownloadBlob",
            BlobTest::DownloadInto(_) => "DownloadIntoBlob",
            BlobTest::List(_) => "ListBlob",
        }
    }
    fn peek(&self) -> PeekSubcommandInfo {
        match self {
            BlobTest::Upload(options) => PeekSubcommandInfo {
                size: Some(options.size as i64),
            },
            BlobTest::Download(options) => PeekSubcommandInfo {
                size: Some(options.size as i64),
            },
            BlobTest::DownloadInto(options) => PeekSubcommandInfo {
                size: Some(options.size as i64),
            },
            BlobTest::List(_) => PeekSubcommandInfo { size: None },
        }
    }
    fn create_test(&self) -> CreatePerfTestReturn {
        match self {
            BlobTest::Upload(options) => upload_blob_test::UploadBlobTest::new(options.clone()),
            BlobTest::Download(options) => {
                download_blob_test::DownloadBlobTest::new(options.clone())
            }
            BlobTest::DownloadInto(options) => {
                download_into_blob_test::DownloadIntoBlobTest::new(options.clone())
            }
            BlobTest::List(options) => list_blob_test::ListBlobTest::new(options.clone()),
        }
    }
}
