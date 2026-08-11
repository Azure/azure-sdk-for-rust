// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::{Arc, OnceLock};

use azure_core::{http::Url, Bytes};
use azure_core_test::{
    perf::{CreatePerfTestReturn, PerfRunner, PerfTest, PerfTestMetadata},
    TestContext,
};
use azure_storage_blob::BlobContainerClient;
use futures::{FutureExt, StreamExt, TryStreamExt};

use crate::{
    extensions::{OnceLockExt, RecordingExt},
    options,
};

pub struct ListBlobTest {
    count: usize,
    endpoint: Option<Url>,
    client: OnceLock<BlobContainerClient>,
}

impl ListBlobTest {
    fn create_test(runner: PerfRunner) -> CreatePerfTestReturn {
        async move {
            let endpoint = runner
                .try_get_test_arg::<String>("endpoint")?
                .map(|endpoint| Url::parse(&endpoint))
                .transpose()?;

            Ok(Box::new(ListBlobTest {
                count: runner
                    .try_get_test_arg::<u32>("count")?
                    .expect("count argument is mandatory") as usize,
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
            create_test: Self::create_test,
        }
    }
}

#[async_trait::async_trait]
impl PerfTest for ListBlobTest {
    async fn setup(&self, context: Arc<TestContext>) -> azure_core::Result<()> {
        let container_client = self.client.try_get_or_init(|| {
            context
                .recording()
                .get_container_client(self.endpoint.clone())
        })?;
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
        if let Some(container_client) = self.client.get() {
            container_client.delete(None).await?;
        }
        Ok(())
    }
}
