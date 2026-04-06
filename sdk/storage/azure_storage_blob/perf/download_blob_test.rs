// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    hint::black_box,
    sync::{Arc, OnceLock},
};

use azure_core::{error::ErrorKind, Bytes};
use azure_core_test::{
    perf::{
        CreatePerfTestReturn, PerfRunner, PerfTest, PerfTestMetadata, PerfTestOption,
        PerfTestOptionKind,
    },
    TestContext,
};
use azure_storage_blob::{BlobClient, BlobContainerClient};
use bytes::BytesMut;
use futures::{FutureExt, StreamExt, TryStreamExt};

enum CollectOptions {
    Stream,
    Core,
    VecBytes,
    Simple,
    Into,
}

pub struct DownloadBlobTest {
    count: u32,
    size: usize,
    collect: CollectOptions,
    endpoint: Option<String>,
    client: OnceLock<BlobContainerClient>,
}

impl DownloadBlobTest {
    fn create_download_blob_test(runner: PerfRunner) -> CreatePerfTestReturn {
        async move {
            let endpoint: Option<String> = runner.try_get_test_arg("endpoint")?;

            let collect = runner.try_get_test_arg("collect")?.unwrap_or(String::new());

            let collect_options = match collect.as_str() {
                "stream" => CollectOptions::Stream,
                "core" => CollectOptions::Core,
                "vec_bytes" => CollectOptions::VecBytes,
                "simple" => CollectOptions::Simple,
                "into" => CollectOptions::Into,
                "" => CollectOptions::Stream, // Default to streaming if no option is provided
                _ => {
                    return Err(azure_core::Error::with_message(
                        ErrorKind::Other,
                        format!("Invalid collect option '{}'", collect),
                    ))
                }
            };

            Ok(Box::new(DownloadBlobTest {
                count: runner.try_get_test_arg("count")?.unwrap_or(5),
                size: runner
                    .try_get_test_arg("size")?
                    .expect("size argument is mandatory"),
                collect: collect_options,
                endpoint,
                client: OnceLock::new(),
            }) as Box<dyn PerfTest>)
        }
        .boxed()
    }

    pub fn test_metadata() -> PerfTestMetadata {
        PerfTestMetadata {
            name: "download_blob",
            description: "Download blobs from a container",
            options: vec![
                PerfTestOption {
                    name: "count",
                    display_message: "The number of blobs to download",
                    mandatory: false,
                    short_activator: Some('c'),
                    long_activator: "count",
                    expected_args_len: 1,
                    option_type: PerfTestOptionKind::Uint32,
                    ..Default::default()
                },
                PerfTestOption {
                    name: "collect",
                    display_message: "Collect the blob contents instead of streaming them",
                    mandatory: false,
                    short_activator: Some('l'),
                    long_activator: "collect",
                    expected_args_len: 1,
                    option_type: PerfTestOptionKind::String,
                    ..Default::default()
                },
                PerfTestOption {
                    name: "size",
                    display_message: "The size of each blob in bytes",
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
            create_test: Self::create_download_blob_test,
        }
    }

    /// This method represents the most basic way to download a blob, where we simply stream the contents and do nothing with them. This is useful for testing the performance of streaming downloads without any additional overhead.
    async fn collect_stream(&self, blob_client: BlobClient) -> azure_core::Result<()> {
        let response = blob_client.download(None).await?;

        let mut body = response.body;

        while let Some(result) = body.next().await {
            // We don't actually care about the contents of the blob for this test, we just want to download it.
            black_box(result?);
        }
        Ok(())
    }

    /// This method collects the entire blob into memory in 512K chunks using the `collect_into` method on the body
    async fn collect_into(&self, blob_client: BlobClient) -> azure_core::Result<()> {
        let response = blob_client.download(None).await?;

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
        let response = blob_client.download(None).await?;

        let body = response.body.collect().await?;
        Ok(black_box(body))
    }

    /// This method collects the entire blob into memory using a simple loop and extending a `Vec<u8>`.
    /// This is the original blob collect method.
    async fn collect_blob_simple(&self, blob_client: BlobClient) -> azure_core::Result<Bytes> {
        let response = blob_client.download(None).await?;

        let mut body = response.body;

        let mut final_result: Vec<u8> = Vec::new();
        while let Some(res) = body.next().await {
            final_result.extend(&res?);
        }

        Ok(black_box(Into::<Bytes>::into(final_result)))
    }

    /// This method collects the entire blob into memory using a `BytesMut` buffer.
    async fn collect_blob_vec_bytes(&self, blob_client: BlobClient) -> azure_core::Result<Bytes> {
        let response = blob_client.download(None).await?;

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
        // Setup code before running the test

        let recording = context.recording();
        let credential = recording.credential();
        let container_name = format!("perf-container-{}", azure_core::Uuid::new_v4());
        let endpoint = match &self.endpoint {
            Some(e) => e.clone(),
            None => format!(
                "https://{}.blob.core.windows.net",
                recording.var("AZURE_STORAGE_ACCOUNT_NAME", None)
            ),
        };
        println!("Using endpoint: {}", endpoint);
        let client = BlobContainerClient::new(&endpoint, &container_name, Some(credential), None)?;
        self.client.get_or_init(|| client);

        // Retrieve the blob container client we just set (it's safe to unwrap here because we *just* set it above).
        let container_client = self.client.get().unwrap();
        let _result = container_client.create(None).await?;

        // Create the blobs for the test.
        for i in 0..self.count {
            let blob_name = format!("blob-{}", i);
            let blob_client = container_client.blob_client(&blob_name);
            let body = vec![0u8; self.size]; // Blob size specified by the test option
            let body_bytes = Bytes::from(body);

            let _result = blob_client.upload(body_bytes.into(), None).await?;
        }

        Ok(())
    }

    async fn run(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        // The actual performance test code

        let mut iterator = self.client.get().unwrap().list_blobs(None)?;
        while let Some(blob) = iterator.try_next().await? {
            let blob_client = self
                .client
                .get()
                .unwrap()
                .blob_client(blob.name.unwrap().as_ref());
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
        }

        Ok(())
    }

    async fn cleanup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        // Cleanup code after running the test
        let mut iterator = self.client.get().unwrap().list_blobs(None)?;
        while let Some(blob) = iterator.try_next().await? {
            let blob_client = self
                .client
                .get()
                .unwrap()
                .blob_client(blob.name.as_ref().unwrap());
            let _result = blob_client.delete(None).await?;
        }

        Ok(())
    }
}
