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
use futures::{lock::Mutex, FutureExt, TryStreamExt};

use crate::{
    clap_parsers::non_zero_usize,
    extensions::{OnceLockExt, RecordingExt},
};

#[derive(Args, Clone, Debug)]
pub struct DownloadIntoBlobTestOptions {
    // The size of each blob in bytes.
    #[arg(long)]
    size: usize,

    // The number of blobs to download.
    #[arg(long, default_value_t = 5)]
    count: usize,

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
    count: usize,
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
                count: args.count,
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

        for i in 0..self.count {
            let blob_name = format!("blob-{}", i);
            let blob_client = container_client.blob_client(&blob_name);
            let body = vec![0u8; self.size];
            let body_bytes = Bytes::from(body);
            let _ = blob_client.upload(body_bytes.into(), None).await?;
        }

        Ok(())
    }

    async fn run(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        let mut blob_list = self.client.get().unwrap().list_blobs(None)?;
        while let Some(blob_item) = blob_list.try_next().await? {
            self.client
                .get()
                .unwrap()
                .blob_client(blob_item.name.unwrap().as_ref())
                .download_into(&mut self.buffer.lock().await, Some(self.download_options()))
                .await?;
            black_box(&self.buffer.lock().await);
        }
        Ok(())
    }

    async fn cleanup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        self.client.get().unwrap().delete(None).await?;
        Ok(())
    }
}
