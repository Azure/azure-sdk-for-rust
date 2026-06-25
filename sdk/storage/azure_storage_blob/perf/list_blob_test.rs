// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::{Arc, OnceLock};

use azure_core::{error::ErrorKind, http::Url, Bytes};
use azure_core_test::{
    perf::{CreatePerfTestReturn, PerfRunner, PerfTest, PerfTestMetadata},
    TestContext,
};

use super::options;
use azure_storage_blob::BlobContainerClient;
use azure_storage_blob_test::get_test_credential;
use futures::{FutureExt, StreamExt, TryStreamExt};

pub struct ListBlobTest {
    count: u32,
    endpoint: Option<String>,
    client: OnceLock<BlobContainerClient>,
}

impl ListBlobTest {
    fn create_list_blob_test(runner: PerfRunner) -> CreatePerfTestReturn {
        async move {
            let endpoint: Option<String> = runner.try_get_test_arg("endpoint")?;

            Ok(Box::new(ListBlobTest {
                count: runner
                    .try_get_test_arg("count")?
                    .expect("count argument is mandatory"),
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
            options: vec![options::count(), options::endpoint()],
            create_test: Self::create_list_blob_test,
        }
    }
}

#[async_trait::async_trait]
impl PerfTest for ListBlobTest {
    async fn setup(&self, context: Arc<TestContext>) -> azure_core::Result<()> {
        // Setup code before running the test

        let recording = context.recording();
        let credential = get_test_credential(recording);
        let container_name = format!("perf-container-{}", azure_core::Uuid::new_v4());
        let endpoint = match &self.endpoint {
            Some(e) => e.clone(),
            None => format!(
                "https://{}.blob.core.windows.net",
                recording.var("AZURE_STORAGE_ACCOUNT_NAME", None)
            ),
        };
        println!("Using endpoint: {}", endpoint);
        let mut container_url = Url::parse(&endpoint)?;
        container_url
            .path_segments_mut()
            .expect("endpoint must be a valid base URL")
            .push(&container_name);
        let client = BlobContainerClient::new(container_url, Some(credential), None)?;
        self.client.set(client).map_err(|_| {
            azure_core::Error::with_message(ErrorKind::Other, "Failed to set client")
        })?;

        // Retrieve the blob container client we just set (it's safe to unwrap here because we *just* set it above).
        let container_client = self.client.get().unwrap();
        let _result = container_client.create(None).await?;

        let body_bytes = Bytes::from(vec![0u8; 10 * 1024]); // 10 KiB blob

        // Create the blobs for the test, running up to 16 uploads concurrently.
        futures::stream::iter(0..self.count)
            .map(|i| {
                let blob_client = container_client.blob_client(&format!("blob-{}", i));
                let body_bytes = body_bytes.clone();
                async move { blob_client.upload(body_bytes.into(), None).await }
            })
            .buffer_unordered(16)
            .try_for_each(|_| async { Ok(()) })
            .await?;

        Ok(())
    }

    async fn run(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        // The actual performance test code

        let mut pager = self.client.get().unwrap().list_blobs(None)?;
        while let Some(blob) = pager.try_next().await? {
            std::hint::black_box(&blob);
        }

        Ok(())
    }

    async fn cleanup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        // Cleanup code after running the test
        self.client.get().unwrap().delete(None).await?;
        Ok(())
    }
}
