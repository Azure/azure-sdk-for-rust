// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::Arc;

use azure_data_cosmos::{
    clients::ContainerClient,
    feed::FeedScope,
    models::{PatchInstructions, PatchOperation},
    options::ChangeFeedStartFrom,
    Query, TransactionalBatch,
};
use futures::StreamExt;
use serde_json::json;

use super::contract::{Operation, Profile};
use crate::operations::{FeedRangeCache, PerfItem};

pub(super) struct Worker {
    pub operation: Operation,
    pub container: ContainerClient,
    pub item: PerfItem,
    pub ranges: FeedRangeCache,
    pub max_pages: usize,
    cursor: usize,
}

impl Worker {
    pub(super) fn new(
        operation: Operation,
        index: u32,
        container: ContainerClient,
        ranges: FeedRangeCache,
        profile: &Profile,
    ) -> Self {
        let id = format!("{}-{index}", operation.name());
        Self {
            operation,
            container,
            item: PerfItem {
                id: id.clone(),
                partition_key: id,
                value: u64::from(index),
                payload: "x".repeat(profile.payload_bytes as usize),
            },
            ranges,
            max_pages: profile.max_pages as usize,
            cursor: 0,
        }
    }

    pub(super) async fn prepare(&self) -> azure_data_cosmos::Result<()> {
        if matches!(self.operation, Operation::DeleteItem) {
            self.container
                .create_item(&self.item.partition_key, &self.item.id, &self.item, None)
                .await?;
        }
        Ok(())
    }

    pub(super) async fn cleanup(&self) -> azure_data_cosmos::Result<()> {
        if matches!(self.operation, Operation::CreateItem) {
            self.container
                .delete_item(&self.item.partition_key, &self.item.id, None)
                .await?;
        }
        Ok(())
    }

    pub(super) async fn execute(&mut self) -> azure_data_cosmos::Result<()> {
        let pk = &self.item.partition_key;
        let id = &self.item.id;
        match self.operation {
            Operation::ReadItem => {
                let response = self.container.read_item(pk, id, None).await?;
                let item: PerfItem = response.into_model()?;
                if item.id != *id {
                    return Err(invalid("point read returned the wrong item"));
                }
            }
            Operation::CreateItem => {
                self.container.create_item(pk, id, &self.item, None).await?;
            }
            Operation::UpsertItem => {
                self.container.upsert_item(pk, id, &self.item, None).await?;
            }
            Operation::ReplaceItem => {
                self.container
                    .replace_item(pk, id, &self.item, None)
                    .await?;
            }
            Operation::PatchItem => {
                let patch = PatchInstructions::from(vec![PatchOperation::set("/value", json!(1))]);
                self.container.patch_item(pk, id, patch, None).await?;
            }
            Operation::DeleteItem => {
                self.container.delete_item(pk, id, None).await?;
            }
            Operation::TransactionalBatch => {
                let batch = TransactionalBatch::new(pk)
                    .create_item(&self.item)?
                    .upsert_item(&self.item, None)?
                    .replace_item(id.clone(), &self.item, None)?
                    .read_item(id.clone(), None)
                    .delete_item(id.clone(), None);
                let response = self
                    .container
                    .execute_transactional_batch(batch, None)
                    .await?;
                let body = response.into_model()?;
                if body.results().len() != 5 || body.results().iter().any(|r| !r.is_success()) {
                    return Err(invalid(
                        "transactional batch did not complete all five operations",
                    ));
                }
            }
            Operation::ChangeFeed => {
                let scope = self.next_range()?;
                let feed = Box::pin(self.container.query_change_feed::<PerfItem>(
                    scope,
                    ChangeFeedStartFrom::Beginning,
                    None,
                ))
                .await?;
                let mut pages = Box::pin(feed.take(self.max_pages));
                let mut items = 0;
                while let Some(page) = pages.next().await {
                    items += page?.items().len();
                }
                if items == 0 {
                    return Err(invalid("change feed returned no seeded items"));
                }
            }
            operation => {
                let (query, scope) = match operation {
                    Operation::QueryItems => (
                        Query::from("SELECT * FROM c WHERE c.partition_key = @pk")
                            .with_parameter("@pk", pk)?,
                        FeedScope::partition(pk),
                    ),
                    Operation::FeedRangeQuery => {
                        (Query::from("SELECT * FROM c"), self.next_range()?)
                    }
                    Operation::CrossPartitionQuery => {
                        (Query::from("SELECT * FROM c"), FeedScope::full_container())
                    }
                    Operation::OrderByQuery => (
                        Query::from("SELECT * FROM c ORDER BY c.value"),
                        FeedScope::full_container(),
                    ),
                    Operation::AggregateQuery => (
                        Query::from("SELECT VALUE COUNT(1) FROM c"),
                        FeedScope::partition(pk),
                    ),
                    Operation::DistinctQuery => (
                        Query::from("SELECT DISTINCT TOP 100 VALUE c.value FROM c"),
                        FeedScope::full_container(),
                    ),
                    _ => unreachable!("non-query operations handled above"),
                };
                let mut pages = Box::pin(
                    self.container
                        .query_items::<serde_json::Value>(query, scope, None),
                )
                .await?
                .into_pages()
                .take(self.max_pages);
                let mut items = 0;
                while let Some(page) = pages.next().await {
                    let page = page?;
                    if operation == Operation::AggregateQuery
                        && page
                            .items()
                            .iter()
                            .any(|v| v.as_u64().is_none_or(|n| n == 0))
                    {
                        return Err(invalid("aggregate query returned an invalid count"));
                    }
                    items += page.items().len();
                }
                if items == 0 {
                    return Err(invalid("query returned no seeded items"));
                }
            }
        }
        Ok(())
    }

    fn next_range(&mut self) -> azure_data_cosmos::Result<FeedScope> {
        let ranges = Arc::clone(&self.ranges.read().expect("feed-range lock poisoned"));
        if ranges.is_empty() {
            return Err(invalid("no feed ranges"));
        }
        let range = ranges[self.cursor % ranges.len()].clone();
        self.cursor = self.cursor.wrapping_add(1);
        Ok(FeedScope::range(range))
    }
}

fn invalid(message: &'static str) -> azure_data_cosmos::CosmosError {
    azure_data_cosmos::CosmosError::builder()
        .with_message(message)
        .build()
}
