// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{List, ListItem, ListItemProperties, ListItemsContainer};
use azure_core::{
    base64,
    http::{
        headers::Headers, BufResponse, ClientOptions, Context, JsonFormat, Method, Pipeline,
        RawResponse, Request, Response, StatusCode, Transport,
    },
    json,
    time::OffsetDateTime,
};
use azure_core_test::{
    http::MockHttpClient,
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
            let mut list = List {
                name: Some("t0123456789abcdef".into()),
                ..Default::default()
            };
            let mut items = Vec::with_capacity(count);
            let now = OffsetDateTime::now_utc();
            for (i, item) in items.iter_mut().enumerate() {
                let name = format!("testItem{i}");
                let hash = base64::encode(&name).as_bytes().to_vec();
                *item = ListItem {
                    name: Some(name),
                    properties: Some(ListItemProperties {
                        etag: Some(i.to_string().into()),
                        creation_time: Some(now),
                        last_modified: Some(now),
                        content_md5: Some(hash),
                    }),
                };
            }
            list.container = Some(ListItemsContainer { items: Some(items) });

            let body = json::to_json(&list)?;
            let client = Arc::new(MockHttpClient::new(move |_| {
                let body = body.clone();
                async move {
                    // Yield simulates an expected network call but kills performance by ~45%.
                    tokio::task::yield_now().await;
                    Ok(BufResponse::from_bytes(
                        StatusCode::Ok,
                        Headers::new(),
                        body,
                    ))
                }
                .boxed()
            }));
            let options = ClientOptions {
                transport: Some(Transport::new(client)),
                ..Default::default()
            };
            let pipeline = Pipeline::new(
                Some("perf"),
                Some("0.1.0"),
                options,
                Vec::new(),
                Vec::new(),
                None,
            );

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
        let list: List = response.into_body()?;
        black_box(&list);

        Ok(())
    }

    async fn cleanup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        Ok(())
    }
}
