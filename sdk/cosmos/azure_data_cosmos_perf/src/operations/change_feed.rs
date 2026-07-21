// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Bounded full-container change feed operation.

use std::time::Duration;

use async_trait::async_trait;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::feed::FeedScope;
use azure_data_cosmos::options::ChangeFeedStartFrom;
use futures::StreamExt;

use super::{extract_backend_duration, Operation, OperationResult, PerfItem};

/// Reads a bounded number of pages from the full-container change feed.
pub struct ChangeFeedOperation {
    max_pages: usize,
}

impl ChangeFeedOperation {
    pub fn new(max_pages: usize) -> Self {
        Self { max_pages }
    }
}

#[async_trait]
impl Operation for ChangeFeedOperation {
    fn name(&self) -> &'static str {
        "ChangeFeed"
    }

    async fn execute(
        &self,
        container: &ContainerClient,
        capture_diagnostics: bool,
    ) -> azure_data_cosmos::Result<OperationResult> {
        let mut stream = Box::pin(
            container
                .query_change_feed::<PerfItem>(
                    FeedScope::full_container(),
                    ChangeFeedStartFrom::Beginning,
                    None,
                )
                .await?
                .take(self.max_pages),
        );

        let mut backend_total: Option<Duration> = None;
        let mut diagnostics = Vec::new();
        while let Some(result) = stream.next().await {
            let page = result?;
            if let Some(duration) = extract_backend_duration(page.headers()) {
                backend_total = Some(backend_total.unwrap_or_default() + duration);
            }
            if capture_diagnostics {
                diagnostics.push(page.diagnostics());
            }
        }

        Ok(OperationResult::paged(backend_total, diagnostics))
    }
}
