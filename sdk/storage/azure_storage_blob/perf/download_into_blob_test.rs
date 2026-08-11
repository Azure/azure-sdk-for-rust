// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    hint::black_box,
    num::NonZero,
    sync::{Arc, OnceLock},
};

use azure_core::http::Url;
use azure_core_test::{
    perf::{CreatePerfTestReturn, PerfRunner, PerfTest, PerfTestMetadata},
    TestContext,
};
use azure_storage_blob::{models::BlobClientDownloadOptions, BlobContainerClient};
use bytes::Bytes;
use futures::{lock::Mutex, FutureExt};

use crate::{
    extensions::{OnceLockExt, RecordingExt},
    options,
};

const BLOB_NAME: &str = "perf-blob";

pub struct DownloadIntoBlobTest {
    size: usize,
    concurrency: Option<NonZero<usize>>,
    partition_size: Option<NonZero<usize>>,
    endpoint: Option<Url>,
    client: OnceLock<BlobContainerClient>,
    buffer: Mutex<Vec<u8>>,
}

impl DownloadIntoBlobTest {
    fn create_test(runner: PerfRunner) -> CreatePerfTestReturn {
        async move {
            let size = runner
                .try_get_test_arg("size")?
                .expect("size argument is mandatory");
            let endpoint = runner
                .try_get_test_arg::<String>("endpoint")?
                .map(|endpoint| Url::parse(&endpoint))
                .transpose()?;

            Ok(Box::new(DownloadIntoBlobTest {
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
            description: "Download a blob from a container directly into a memory buffer.",
            options: vec![
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
