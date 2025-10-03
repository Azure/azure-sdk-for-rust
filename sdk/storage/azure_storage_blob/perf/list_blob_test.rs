// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{Bytes, Result};
use azure_core_test::{
    perf::{CreatePerfTestReturn, PerfRunner, PerfTest, PerfTestMetadata, PerfTestOption},
    TestContext,
};
use azure_identity::DeveloperToolsCredential;
use azure_storage_blob::BlobContainerClient;
use futures::TryStreamExt;

pub struct ListBlobTest {
    count: u32,
    client: BlobContainerClient,
}

impl ListBlobTest {
    fn create_list_blob_test(runner: &PerfRunner) -> CreatePerfTestReturn {
        async fn create_test(runner: PerfRunner) -> Result<Box<dyn PerfTest>> {
            let count: Option<&String> = runner.try_get_test_arg("count")?;

            println!("ListBlobTest with count: {:?}", count);
            let count = count.expect("count argument is mandatory").parse::<u32>()?;
            println!("Parsed count: {}", count);

            let endpoint: Option<&String> = runner.try_get_test_arg("endpoint")?;
            let endpoint = endpoint.expect("endpoint argument is mandatory").clone();
            println!("Using endpoint: {}", endpoint);

            let container_name = format!("perf-container-{}", uuid::Uuid::new_v4());
            let credential = DeveloperToolsCredential::new(None)?;
            let client = BlobContainerClient::new(&endpoint, container_name, credential, None)?;

            Ok(Box::new(ListBlobTest { count, client }) as Box<dyn PerfTest>)
        }
        // Here you would create and return an instance of your performance test.
        // For example:
        Box::pin(create_test(runner.clone()))
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
                    short_activator: 'c',
                    long_activator: "count",
                    expected_args_len: 1,
                    ..Default::default()
                },
                PerfTestOption {
                    name: "endpoint",
                    display_message: "The endpoint of the blob storage",
                    mandatory: true,
                    short_activator: 'e',
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
    async fn setup(&self, _context: &TestContext) -> azure_core::Result<()> {
        // Setup code before running the test

        let _result = self.client.create_container(None).await?;

        for i in 0..self.count {
            let blob_name = format!("blob-{}", i);
            let blob_client = self.client.blob_client(blob_name);

            let body = vec![0u8; 1024 * 1024]; // 1 MB blob
            let body_bytes = Bytes::from(body);

            let _result = blob_client.upload(body_bytes.into(), true, 5, None).await?;
        }

        Ok(())
    }

    async fn run(&self) -> azure_core::Result<()> {
        // The actual performance test code

        let mut iterator = self.client.list_blobs(None)?;
        while let Some(blob_segment) = iterator.try_next().await? {
            let _body = blob_segment.into_body()?;
        }

        Ok(())
    }

    async fn cleanup(&self, _context: &TestContext) -> azure_core::Result<()> {
        // Cleanup code after running the test
        Ok(())
    }
}
