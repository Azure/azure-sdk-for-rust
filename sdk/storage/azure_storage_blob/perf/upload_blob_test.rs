// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    num::NonZero,
    sync::{Arc, OnceLock},
};

use azure_core::{http::Url, Bytes};
use azure_core_test::{
    perf::{CreatePerfTestReturn, PerfTest},
    TestContext,
};
use azure_storage_blob::{models::BlobClientUploadOptions, BlobContainerClient};
use azure_storage_blob_test::get_test_credential;
use clap::Args;
use futures::FutureExt;

use crate::clap_parsers::non_zero_usize;

#[derive(Args, Clone, Debug)]
pub struct UploadBlobTestOptions {
    // The size of each blob in bytes.
    #[arg(long)]
    size: usize,

    // Number of concurrent network transfers.
    #[arg(long, value_parser = non_zero_usize)]
    concurrency: Option<NonZero<usize>>,

    // Size in bytes to partition data into for each transfer.
    #[arg(long, value_parser = non_zero_usize)]
    partition_size: Option<NonZero<u64>>,

    #[arg(long)]
    endpoint: Option<Url>,
}

pub struct UploadBlobTest {
    size: usize,
    concurrency: Option<NonZero<usize>>,
    partition_size: Option<NonZero<u64>>,
    upload_buffer: OnceLock<Bytes>,
    endpoint: Option<Url>,
    client: OnceLock<BlobContainerClient>,
}

impl UploadBlobTest {
    pub fn new(args: UploadBlobTestOptions) -> CreatePerfTestReturn {
        async move {
            Ok(Box::new(UploadBlobTest {
                size: args.size,
                concurrency: args.concurrency,
                partition_size: args.partition_size,
                endpoint: args.endpoint,
                client: OnceLock::new(),
                upload_buffer: OnceLock::new(),
            }) as Box<dyn PerfTest>)
        }
        .boxed()
    }
}

#[async_trait::async_trait]
impl PerfTest for UploadBlobTest {
    async fn setup(&self, context: Arc<TestContext>) -> azure_core::Result<()> {
        // Setup code before running the test

        let recording = context.recording();
        let credential = get_test_credential(recording);
        let container_name = format!("perf-container-{}", azure_core::Uuid::new_v4());
        let mut container_url = match &self.endpoint {
            Some(e) => e.clone(),
            None => Url::parse(&format!(
                "https://{}.blob.core.windows.net",
                recording.var("AZURE_STORAGE_ACCOUNT_NAME", None)
            ))?,
        };
        container_url
            .path_segments_mut()
            .expect("endpoint must be a valid base URL")
            .push(&container_name);
        let client = BlobContainerClient::new(container_url, Some(credential), None)?;
        self.client.get_or_init(|| client);
        let data = vec![0u8; self.size];
        self.upload_buffer
            .get_or_init(|| Bytes::copy_from_slice(&data));

        let container_client = self.client.get().unwrap();
        container_client.create(None).await?;

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
