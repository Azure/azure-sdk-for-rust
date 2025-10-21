// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::{Arc, OnceLock};

use azure_core::{error::ErrorKind, Bytes};
use azure_core_test::{
    perf::{CreatePerfTestReturn, PerfRunner, PerfTest, PerfTestMetadata, PerfTestOption},
    TestContext,
};
use azure_storage_blob::BlobContainerClient;
use futures::{FutureExt, TryStreamExt};

pub struct ListBlobTest {
    count: u32,
    endpoint: String,
    client: OnceLock<BlobContainerClient>,
}

impl ListBlobTest {
    fn create_list_blob_test(runner: PerfRunner) -> CreatePerfTestReturn {
        async move {
            let count: Option<&String> = runner.try_get_test_arg("count")?;

            println!("ListBlobTest with count: {:?}", count);
            let count = count.expect("count argument is mandatory").parse::<u32>()?;
            println!("Parsed count: {}", count);

            let endpoint: Option<&String> = runner.try_get_test_arg("endpoint")?;
            let endpoint = match endpoint {
                Some(e) => e.clone(),
                None => format!(
                    "https://{}.blob.core.windows.net",
                    std::env::var("AZURE_STORAGE_ACCOUNT_NAME")
                        .expect("AZURE_STORAGE_ACCOUNT_NAME is not set")
                ),
            };
            println!("Using endpoint: {}", endpoint);

            Ok(Box::new(ListBlobTest {
                count,
                endpoint,
                client: OnceLock::new(),
            }) as Box<dyn PerfTest>)
        }
        .boxed()
    }

    pub fn test_metadata() -> PerfTestMetadata {
        PerfTestMetadata {
            name: "list_blob",
            description: "List blobs in a container",
            options: vec![
                PerfTestOption {
                    name: "count",
                    display_message: "The number of blobs to list",
                    mandatory: true,
                    short_activator: Some('c'),
                    long_activator: "count",
                    expected_args_len: 1,
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
            create_test: Self::create_list_blob_test,
        }
    }
}

#[async_trait::async_trait]
impl PerfTest for ListBlobTest {
    async fn setup(&self, context: Arc<TestContext>) -> azure_core::Result<()> {
        // Setup code before running the test

        let recording = context.recording();
        let credential = recording.credential();
        let container_name = format!("perf-container-{}", uuid::Uuid::new_v4());
        let client = BlobContainerClient::new(&self.endpoint, &container_name, credential, None)?;
        self.client.set(client).map_err(|_| {
            azure_core::Error::with_message(ErrorKind::Other, "Failed to set client")
        })?;

        // Retrieve the blob container client we just set (it's safe to unwrap here because we *just* set it above).
        let container_client = self.client.get().unwrap();
        let _result = container_client.create_container(None).await?;

        // Create the blobs for the test.
        for i in 0..self.count {
            let blob_name = format!("blob-{}", i);
            let blob_client = container_client.blob_client(&blob_name);
            let body = vec![0u8; 1024 * 1024]; // 1 MB blob
            let body_bytes = Bytes::from(body);

            let _result = blob_client.upload(body_bytes.into(), true, 5, None).await?;
        }

        Ok(())
    }

    async fn run(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        // The actual performance test code

        let mut iterator = self.client.get().unwrap().list_blobs(None)?;
        while let Some(blob_segment) = iterator.try_next().await? {
            let _body = blob_segment.into_body()?;
        }

        Ok(())
    }

    async fn cleanup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        // Cleanup code after running the test
        Ok(())
    }
}
