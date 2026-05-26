// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    hint::black_box,
    sync::{Arc, OnceLock},
};

use azure_core_test::{
    perf::{
        CreatePerfTestReturn, PerfRunner, PerfTest, PerfTestMetadata, PerfTestOption,
        PerfTestOptionKind,
    },
    TestContext,
};
use azure_storage_blob::BlobContainerClient;
use bytes::Bytes;
use futures::{FutureExt, TryStreamExt};

use crate::extensions::{OnceLockExt, RecordingExt};

const DEFAULT_NUM_BLOBS: usize = 5;

pub struct DownloadIntoBlobTest {
    num_blobs: usize,
    blob_len: usize,
    endpoint: Option<String>,
    client: OnceLock<BlobContainerClient>,
}

impl DownloadIntoBlobTest {
    fn create_test(runner: PerfRunner) -> CreatePerfTestReturn {
        async move {
            let endpoint: Option<String> = runner.try_get_test_arg("endpoint")?;

            Ok(Box::new(Self {
                num_blobs: runner
                    .try_get_test_arg("num_blobs")?
                    .unwrap_or(DEFAULT_NUM_BLOBS),
                blob_len: runner
                    .try_get_test_arg("blob_len")?
                    .expect("size argument is mandatory"),
                endpoint,
                client: OnceLock::new(),
            }) as Box<dyn PerfTest>)
        }
        .boxed()
    }

    pub fn test_metadata() -> PerfTestMetadata {
        PerfTestMetadata {
            name: "download_into_blob",
            description: "Download blobs from a container directly into memory buffers.",
            options: vec![
                PerfTestOption {
                    name: "num_blobs",
                    display_message: "The number of blobs to download",
                    mandatory: false,
                    short_activator: Some('c'),
                    long_activator: "count",
                    expected_args_len: 1,
                    option_type: PerfTestOptionKind::Usize,
                    ..Default::default()
                },
                PerfTestOption {
                    name: "blob_len",
                    display_message: "The length of each blob in bytes",
                    mandatory: true,
                    short_activator: Some('s'),
                    long_activator: "size",
                    expected_args_len: 1,
                    option_type: PerfTestOptionKind::Usize,
                    ..Default::default()
                },
                PerfTestOption {
                    name: "endpoint",
                    display_message: "The endpoint of the blob storage",
                    mandatory: false,
                    short_activator: Some('e'),
                    long_activator: "endpoint",
                    expected_args_len: 1,
                    ..Default::default()
                },
            ],
            create_test: Self::create_test,
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

        for i in 0..self.num_blobs {
            let blob_name = format!("blob-{}", i);
            let blob_client = container_client.blob_client(&blob_name);
            let body = vec![0u8; self.blob_len];
            let body_bytes = Bytes::from(body);
            let _ = blob_client.upload(body_bytes.into(), None).await?;
        }

        Ok(())
    }

    async fn run(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        // TODO large buffer allocation and drop is now getting measured
        let mut buffer = vec![0u8; self.blob_len];

        let mut blob_list = self.client.get().unwrap().list_blobs(None)?;
        while let Some(blob_item) = blob_list.try_next().await? {
            self.client
                .get()
                .unwrap()
                .blob_client(blob_item.name.unwrap().as_ref())
                .download_into(&mut buffer, None)
                .await?;
            black_box(&buffer);
        }
        todo!()
    }

    async fn cleanup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        self.client.get().unwrap().delete(None).await?;
        Ok(())
    }
}
