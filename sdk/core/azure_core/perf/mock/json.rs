// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::List;
use azure_core::{
    http::{Context, JsonFormat, Method, Pipeline, RawResponse, Request, Response},
    json,
};
use azure_core_test::{
    perf::{CreatePerfTestReturn, PerfTest},
    TestContext,
};
use clap::Args;
use futures::FutureExt as _;
use std::{hint::black_box, sync::Arc};

pub struct MockJsonTest {
    pipeline: Pipeline,
}

#[derive(Args, Debug, Clone)]
pub struct MockJsonTestArgs {
    // Number of items per page.
    #[arg(long, default_value_t = super::DEFAULT_COUNT)]
    pub count: usize,
}

pub fn create_test(args: &MockJsonTestArgs) -> CreatePerfTestReturn {
    let count = args.count;
    async move {
        let pipeline = super::create_pipeline(count, json::to_json)?;
        Ok(Box::new(MockJsonTest { pipeline }) as Box<dyn PerfTest>)
    }
    .boxed()
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
