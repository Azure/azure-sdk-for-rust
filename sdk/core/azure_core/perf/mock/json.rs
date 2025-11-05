// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::List;
use azure_core::{
    http::{Context, JsonFormat, Method, Pipeline, RawResponse, Request, Response},
    json,
};
use azure_core_test::{
    perf::{CreatePerfTestReturn, PerfRunner, PerfTest, PerfTestMetadata, PerfTestOption},
    TestContext,
};
use futures::FutureExt as _;
use std::{hint::black_box, sync::Arc};

pub struct MockJsonTest {
    pipeline: Pipeline,
}

impl MockJsonTest {
    fn create_items(runner: PerfRunner) -> CreatePerfTestReturn {
        async move {
            let count = runner
                .try_get_test_arg("count")?
                .cloned()
                .unwrap_or(super::DEFAULT_COUNT);
            let pipeline = super::create_pipeline(count, json::to_json)?;
            Ok(Box::new(MockJsonTest { pipeline }) as Box<dyn PerfTest>)
        }
        .boxed()
    }

    pub fn test_metadata() -> PerfTestMetadata {
        PerfTestMetadata {
            name: "mock_json",
            description: "Mock transport that returns JSON",
            options: vec![PerfTestOption {
                name: "count",
                display_message: "Number of items per page",
                mandatory: false,
                short_activator: None,
                long_activator: "count",
                expected_args_len: 1,
                ..Default::default()
            }],
            create_test: Self::create_items,
        }
    }
}

#[async_trait::async_trait]
impl PerfTest for MockJsonTest {
    async fn setup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        Ok(())
    }

    async fn run(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        let ctx = Context::new();
        let mut request = Request::new(
            "https://contoso.com/containers/t0123456789abcdef?api-version=2025-10-15".parse()?,
            Method::Get,
        );
        let response = self.pipeline.send(&ctx, &mut request, None).await?;
        // Make sure we deserialize the response.
        let (status, headers, body) = response.deconstruct();
        let response: Response<List, JsonFormat> =
            RawResponse::from_bytes(status, headers, body).into();
        let list: List = tokio::spawn(async move {
            tokio::task::yield_now().await;
            response.into_model()
        })
        .await
        .unwrap()?;
        assert_eq!(black_box(list.name), Some("t0123456789abcdef".into()));

        Ok(())
    }

    async fn cleanup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        Ok(())
    }
}
