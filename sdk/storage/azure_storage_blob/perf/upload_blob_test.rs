// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::{Arc, OnceLock};

use azure_core::Bytes;
use azure_core_test::{
    perf::{
        CreatePerfTestReturn, PerfRunner, PerfTest, PerfTestMetadata, PerfTestOption,
        TestOptionType,
    },
    TestContext,
};
use azure_storage_blob::BlobContainerClient;
use futures::{FutureExt, TryStreamExt};

pub struct UploadBlobTest {
    count: u32,
    size: usize,
    upload_buffer: OnceLock<Bytes>,
    endpoint: Option<String>,
    client: OnceLock<BlobContainerClient>,
}

impl UploadBlobTest {
    fn create_upload_blob_test(runner: PerfRunner) -> CreatePerfTestReturn {
        async move {
            Ok(Box::new(UploadBlobTest {
                count: runner
                    .try_get_test_arg("count")?
                    .expect("'count' argument is required."),
                endpoint: runner.try_get_test_arg("endpoint")?,
                client: OnceLock::new(),
                size: runner
                    .try_get_test_arg("size")?
                    .expect("'size' parameter is required."),
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
                PerfTestOption {
                    name: "count",
                    display_message: "The number of blobs to upload",
                    mandatory: false,
                    short_activator: Some('c'),
                    long_activator: "count",
                    expected_args_len: 1,
                    option_type: TestOptionType::Uint32,
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
                PerfTestOption {
                    name: "size",
                    display_message: "The size of each blob in bytes",
                    mandatory: true,
                    short_activator: Some('s'),
                    long_activator: "size",
                    expected_args_len: 1,
                    option_type: TestOptionType::Usize,
                    ..Default::default()
                },
            ],
            create_test: Self::create_upload_blob_test,
        }
    }
}

#[async_trait::async_trait]
impl PerfTest for UploadBlobTest {
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
        self.client.get_or_init(|| {
            BlobContainerClient::new(&endpoint, &container_name, Some(credential), None)
                .unwrap_or_else(|_| panic!("Failed to create BlobContainerClient"))
        });
        let data = vec![0u8; self.size];
        self.upload_buffer
            .get_or_init(|| Bytes::copy_from_slice(&data));

        // Retrieve the blob container client we just set (it's safe to unwrap here because we *just* set it above).
        let container_client = self.client.get().unwrap();
        let _result = container_client.create(None).await?;

        // Create the blobs for the test.
        for i in 0..self.count {
            let blob_name = format!("blob-{}", i);
            let blob_client = container_client.blob_client(&blob_name);
            let body = vec![0u8; 10]; // Tiny blob to focus on upload overhead rather than payload size.
            let body_bytes = Bytes::from(body);

            let _result = blob_client.upload(body_bytes.into(), true, 5, None).await?;
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

            let data_bytes = self.upload_buffer.get().unwrap().clone();
            blob_client
                .upload(data_bytes.into(), true, self.size as u64, None)
                .await?;
        }

        Ok(())
    }

    async fn cleanup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        // Cleanup code after running the test
        Ok(())
    }
}
