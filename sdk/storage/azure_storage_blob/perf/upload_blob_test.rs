// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    num::NonZero,
    sync::{Arc, OnceLock},
};

use azure_core::{http::Url, Bytes};
use azure_core_test::{
    perf::{CreatePerfTestReturn, PerfRunner, PerfTest, PerfTestMetadata},
    TestContext,
};
use azure_storage_blob::{models::BlobClientUploadOptions, BlobContainerClient};
use futures::FutureExt;

use crate::{
    extensions::{OnceLockExt, RecordingExt},
    options,
};

pub struct UploadBlobTest {
    size: usize,
    concurrency: Option<NonZero<usize>>,
    partition_size: Option<NonZero<u64>>,
    upload_buffer: OnceLock<Bytes>,
    endpoint: Option<Url>,
    client: OnceLock<BlobContainerClient>,
}

impl UploadBlobTest {
    fn create_test(runner: PerfRunner) -> CreatePerfTestReturn {
        async move {
            let endpoint = runner
                .try_get_test_arg::<String>("endpoint")?
                .map(|endpoint| Url::parse(&endpoint))
                .transpose()?;

            Ok(Box::new(UploadBlobTest {
                size: runner
                    .try_get_test_arg("size")?
                    .expect("size argument is mandatory"),
                concurrency: runner
                    .try_get_test_arg::<usize>("concurrency")?
                    .and_then(NonZero::new),
                partition_size: runner
                    .try_get_test_arg::<usize>("partition-size")?
                    .and_then(|value| NonZero::new(value as u64)),
                endpoint,
                client: OnceLock::new(),
                upload_buffer: OnceLock::new(),
            }) as Box<dyn PerfTest>)
        }
        .boxed()
    }

    pub fn test_metadata() -> PerfTestMetadata {
        PerfTestMetadata {
            name: "upload_blob",
            description: "Upload blobs to a container",
            options: vec![
                options::size(),
                options::concurrency(),
                options::partition_size(),
                options::endpoint(),
            ],
            create_test: Self::create_test,
        }
    }
}

#[async_trait::async_trait]
impl PerfTest for UploadBlobTest {
    async fn setup(&self, context: Arc<TestContext>) -> azure_core::Result<()> {
        let container_client = self.client.try_get_or_init(|| {
            context
                .recording()
                .get_container_client(self.endpoint.clone())
        })?;
        container_client.create(None).await?;

        let data = vec![0u8; self.size];
        self.upload_buffer
            .get_or_init(|| Bytes::copy_from_slice(&data));

        Ok(())
    }

    async fn run(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        let blob_client = self.client.get().unwrap().blob_client("perf-blob");
        let data_bytes = self.upload_buffer.get().unwrap().clone();
        let options = BlobClientUploadOptions {
            parallel: self.concurrency,
            partition_size: self.partition_size,
            ..Default::default()
        };
        blob_client.upload(data_bytes.into(), Some(options)).await?;

        Ok(())
    }

    async fn cleanup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        // Each instance has its own container, so just delete it.
        if let Some(container_client) = self.client.get() {
            container_client.delete(None).await?;
        }
        Ok(())
    }
}
