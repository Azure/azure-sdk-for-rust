// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    hint::black_box,
    num::NonZero,
    sync::{Arc, OnceLock},
};

use azure_core_test::{
    perf::{CreatePerfTestReturn, PerfRunner, PerfTest},
    TestContext,
};
use azure_storage_blob::{models::BlobClientDownloadOptions, BlobContainerClient};
use bytes::Bytes;
use futures::{lock::Mutex, FutureExt, TryStreamExt};

use crate::extensions::{OnceLockExt, RecordingExt};

const DEFAULT_NUM_BLOBS: usize = 5;

pub struct DownloadIntoBlobTest {
    count: usize,
    size: usize,
    concurrency: Option<NonZero<usize>>,
    partition_size: Option<NonZero<usize>>,
    endpoint: Option<String>,
    client: OnceLock<BlobContainerClient>,
    buffer: Mutex<Vec<u8>>,
}

impl DownloadIntoBlobTest {
    fn create_test(runner: PerfRunner) -> CreatePerfTestReturn {
        async move {
            let endpoint = runner.try_get_test_arg("endpoint")?;
            let size = runner
                .try_get_test_arg("size")?
                .expect("size argument is mandatory");

            Ok(Box::new(Self {
                count: runner
                    .try_get_test_arg("count")?
                    .unwrap_or(DEFAULT_NUM_BLOBS),
                size,
                concurrency: runner
                    .try_get_test_arg::<usize>("concurrency")?
                    .and_then(NonZero::new),
                partition_size: runner
                    .try_get_test_arg::<usize>("partition-size")?
                    .and_then(NonZero::new),
                endpoint,
                client: OnceLock::new(),
                buffer: Mutex::new(vec![0; size]),
            }) as Box<dyn PerfTest>)
        }
        .boxed()
    }

    pub fn test_metadata() -> PerfTestMetadata {
        PerfTestMetadata {
            name: "download_into_blob",
            description: "Download blobs from a container directly into memory buffers.",
            options: vec![
                options::count(),
                options::collect(),
                options::size(),
                options::concurrency(),
                options::partition_size(),
                options::endpoint(),
            ],
            create_test: Self::create_test,
        }
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
