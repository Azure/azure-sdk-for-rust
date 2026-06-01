// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::{Arc, OnceLock};

use azure_core::{http::Url, Bytes};
use azure_core_test::{
    perf::{CreatePerfTestReturn, PerfRunner, PerfTest, PerfTestMetadata},
    TestContext,
};

use super::options;
use azure_storage_blob::BlobContainerClient;
use azure_storage_blob_test::get_test_credential;
use futures::FutureExt;

pub struct UploadBlobTest {
    size: usize,
    upload_buffer: OnceLock<Bytes>,
    endpoint: Option<String>,
    client: OnceLock<BlobContainerClient>,
}

impl UploadBlobTest {
    fn create_upload_blob_test(runner: PerfRunner) -> CreatePerfTestReturn {
        async move {
            Ok(Box::new(UploadBlobTest {
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
            options: vec![options::size(), options::endpoint()],
            create_test: Self::create_upload_blob_test,
        }
    }
}

#[async_trait::async_trait]
impl PerfTest for UploadBlobTest {
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
        blob_client.upload(data_bytes.into(), None).await?;

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
