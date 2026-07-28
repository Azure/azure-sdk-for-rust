// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    hint::black_box,
    num::NonZero,
    sync::{Arc, OnceLock},
};

use azure_core::{http::Url, Bytes};
use azure_core_test::{
    perf::{CreatePerfTestReturn, PerfTest},
    TestContext,
};
use azure_storage_blob::{models::BlobClientDownloadOptions, BlobClient, BlobContainerClient};
use bytes::BytesMut;
use clap::{Args, ValueEnum};
use futures::{FutureExt, StreamExt};

use crate::{
    clap_parsers::non_zero_usize,
    extensions::{OnceLockExt, RecordingExt},
};

const BLOB_NAME: &str = "perf-blob";

#[derive(Args, Clone, Debug)]
pub struct DownloadBlobTestOptions {
    // The size of each blob in bytes.
    #[arg(long)]
    pub size: usize,

    #[arg(long, default_value_t = CollectOptions::Stream, value_enum)]
    collect: CollectOptions,

    // Number of concurrent network transfers.
    #[arg(long, value_parser = non_zero_usize)]
    concurrency: Option<NonZero<usize>>,

    // Size in bytes to partition data into for each transfer.
    #[arg(long, value_parser = non_zero_usize)]
    partition_size: Option<NonZero<usize>>,

    #[arg(long)]
    endpoint: Option<Url>,
}

#[derive(ValueEnum, Clone, Debug)]
enum CollectOptions {
    Stream,
    Core,
    VecBytes,
    Simple,
    Into,
}

pub struct DownloadBlobTest {
    size: usize,
    collect: CollectOptions,
    concurrency: Option<NonZero<usize>>,
    partition_size: Option<NonZero<usize>>,
    endpoint: Option<Url>,
    client: OnceLock<BlobContainerClient>,
}

impl DownloadBlobTest {
    pub fn new(args: DownloadBlobTestOptions) -> CreatePerfTestReturn {
        async move {
            Ok(Box::new(DownloadBlobTest {
                size: args.size,
                collect: args.collect,
                concurrency: args.concurrency,
                partition_size: args.partition_size,
                endpoint: args.endpoint,
                client: OnceLock::new(),
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

    /// This method represents the most basic way to download a blob, where we simply stream the contents and do nothing with them. This is useful for testing the performance of streaming downloads without any additional overhead.
    async fn collect_stream(&self, blob_client: BlobClient) -> azure_core::Result<()> {
        let response = blob_client.download(Some(self.download_options())).await?;

        let mut body = response.body;

        while let Some(result) = body.next().await {
            // We don't actually care about the contents of the blob for this test, we just want to download it.
            black_box(result?);
        }
        Ok(())
    }

    /// This method collects the entire blob into memory in 512K chunks using the `collect_into` method on the body
    async fn collect_into(&self, blob_client: BlobClient) -> azure_core::Result<()> {
        let response = blob_client.download(Some(self.download_options())).await?;

        let mut buffer = vec![0u8; self.size];
        response.body.collect_into(&mut buffer).await?;
        black_box(buffer);
        Ok(())
    }

    /// This method collects the entire blob into memory using the `collect` method on the stream.
    ///
    /// This is useful for testing the performance of collecting the entire blob into memory, which
    /// may be a common scenario for smaller blobs.
    async fn collect_blob(&self, blob_client: BlobClient) -> azure_core::Result<Bytes> {
        let response = blob_client.download(Some(self.download_options())).await?;

        let body = response.body.collect().await?;
        Ok(black_box(body))
    }

    /// This method collects the entire blob into memory using a simple loop and extending a `Vec<u8>`.
    /// This is the original blob collect method.
    async fn collect_blob_simple(&self, blob_client: BlobClient) -> azure_core::Result<Bytes> {
        let response = blob_client.download(Some(self.download_options())).await?;

        let mut body = response.body;

        let mut final_result: Vec<u8> = Vec::new();
        while let Some(res) = body.next().await {
            final_result.extend(&res?);
        }

        Ok(black_box(Into::<Bytes>::into(final_result)))
    }

    /// This method collects the entire blob into memory using a `BytesMut` buffer.
    async fn collect_blob_vec_bytes(&self, blob_client: BlobClient) -> azure_core::Result<Bytes> {
        let response = blob_client.download(Some(self.download_options())).await?;

        let mut body = response.body;
        let mut bytes = Vec::<Bytes>::new();
        let mut total_length = 0usize;

        while let Some(res) = body.next().await {
            let res = res?;
            total_length += res.len();
            bytes.push(res);
        }

        let mut final_result = BytesMut::with_capacity(total_length);
        for b in bytes {
            final_result.extend(b);
        }

        Ok(black_box(final_result.freeze()))
    }
}

#[async_trait::async_trait]
impl PerfTest for DownloadBlobTest {
    async fn setup(&self, context: Arc<TestContext>) -> azure_core::Result<()> {
        let container_client = self.client.try_get_or_init(|| {
            context
                .recording()
                .get_container_client(self.endpoint.clone())
        })?;
        let _result = container_client.create(None).await?;

        // Create the blob for the test.
        let blob_client = container_client.blob_client(BLOB_NAME);
        let body = vec![0u8; self.size]; // Blob size specified by the test option
        let body_bytes = Bytes::from(body);

        let _result = blob_client.upload(body_bytes.into(), None).await?;

        Ok(())
    }

    async fn run(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        // The actual performance test code

        let blob_client = self.client.get().unwrap().blob_client(BLOB_NAME);
        match self.collect {
            CollectOptions::Stream => {
                self.collect_stream(blob_client).await?;
            }
            CollectOptions::Core => {
                self.collect_blob(blob_client).await?;
            }
            CollectOptions::Simple => {
                self.collect_blob_simple(blob_client).await?;
            }
            CollectOptions::VecBytes => {
                self.collect_blob_vec_bytes(blob_client).await?;
            }
            CollectOptions::Into => {
                self.collect_into(blob_client).await?;
            }
        }

        Ok(())
    }

    async fn cleanup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        if let Some(container_client) = self.client.get() {
            container_client.delete(None).await?;
        }
        Ok(())
    }
}
