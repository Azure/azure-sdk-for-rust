// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    hint::black_box,
    num::NonZero,
    sync::{Arc, OnceLock},
};

use azure_core::http::Url;
use azure_core_test::{
    perf::{CreatePerfTestReturn, PerfTest},
    TestContext,
};
use azure_storage_blob::{models::BlobClientDownloadOptions, BlobContainerClient};
use bytes::Bytes;
use clap::Args;
use futures::{lock::Mutex, FutureExt};

use crate::{
    clap_parsers::non_zero_usize,
    extensions::{OnceLockExt, RecordingExt},
};

const BLOB_NAME: &str = "perf-blob";

#[derive(Args, Clone, Debug)]
pub struct DownloadIntoBlobTestOptions {
    // The size of each blob in bytes.
    #[arg(long)]
    pub size: usize,

    // Number of concurrent network transfers.
    #[arg(long, value_parser = non_zero_usize)]
    concurrency: Option<NonZero<usize>>,

    // Size in bytes to partition data into for each transfer.
    #[arg(long, value_parser = non_zero_usize)]
    partition_size: Option<NonZero<usize>>,

    #[arg(long)]
    endpoint: Option<Url>,
}

pub struct DownloadIntoBlobTest {
    size: usize,
    concurrency: Option<NonZero<usize>>,
    partition_size: Option<NonZero<usize>>,
    endpoint: Option<Url>,
    client: OnceLock<BlobContainerClient>,
    buffer: Mutex<Vec<u8>>,
}

impl DownloadIntoBlobTest {
    pub fn new(args: DownloadIntoBlobTestOptions) -> CreatePerfTestReturn {
        async move {
            Ok(Box::new(DownloadIntoBlobTest {
                size: args.size,
                concurrency: args.concurrency,
                partition_size: args.partition_size,
                endpoint: args.endpoint,
                client: OnceLock::new(),
                buffer: Mutex::new(vec![0; args.size]),
            }) as Box<dyn PerfTest>)
        }
        .boxed()
    }

    fn download_options(&self) -> BlobClientDownloadOptions<'_> {
        BlobClientDownloadOptions {
            parallel: self.concurrency,
            partition_size: self.partition_size,
            ..Default::default()
        }
    }
}

#[async_trait::async_trait]
impl PerfTest for DownloadIntoBlobTest {
    async fn setup(&self, context: Arc<TestContext>) -> azure_core::Result<()> {
        let container_client = self.client.try_get_or_init(|| {
            context
                .recording()
                .get_container_client(self.endpoint.clone())
        })?;
        container_client.create(None).await?;

        // Create the blob for the test.
        let blob_client = container_client.blob_client(BLOB_NAME);
        let body = vec![0u8; self.size]; // Blob size specified by the test option
        let body_bytes = Bytes::from(body);

        blob_client.upload(body_bytes.into(), None).await?;

        Ok(())
    }

    async fn run(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        let blob_client = self.client.get().unwrap().blob_client(BLOB_NAME);
        let mut buf = self.buffer.lock().await;
        blob_client
            .download_into(&mut buf, Some(self.download_options()))
            .await?;
        black_box(buf);
        Ok(())
    }

    async fn cleanup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        self.client.get().unwrap().delete(None).await?;
        Ok(())
    }
}
