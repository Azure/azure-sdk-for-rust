// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Container seeding with test data.

use std::sync::{Arc, Mutex};

use azure_data_cosmos::clients::ContainerClient;
use rand::RngExt;
use uuid::Uuid;

use crate::operations::PerfItem;

/// A reference to a seeded item's ID and partition key.
#[derive(Debug, Clone)]
pub struct SeededItem {
    pub id: String,
    pub partition_key: String,
}

/// Thread-safe, capacity-capped collection of seeded items.
///
/// Operations read randomly from the list, and the create operation adds new
/// items so they become available to subsequent reads and queries. When the
/// list reaches its capacity, new items replace random existing entries to
/// prevent unbounded memory growth.
#[derive(Debug)]
pub struct SharedItems {
    items: Mutex<Vec<SeededItem>>,
    capacity: usize,
}

impl SharedItems {
    /// Creates a new shared item list from the initial seeded items.
    ///
    /// Capacity is set to `2 * items.len()` so creates can grow the pool
    /// without unbounded allocation.
    pub fn new(items: Vec<SeededItem>) -> Arc<Self> {
        let capacity = items.len() * 2;
        Arc::new(Self {
            items: Mutex::new(items),
            capacity,
        })
    }

    /// Returns a random item from the list.
    pub fn random(&self) -> SeededItem {
        let items = self.items.lock().unwrap();
        let idx = rand::rng().random_range(0..items.len());
        items[idx].clone()
    }

    /// Adds a new item, replacing a random existing entry if at capacity.
    pub fn push(&self, item: SeededItem) {
        let mut items = self.items.lock().unwrap();
        if items.len() < self.capacity {
            items.push(item);
        } else {
            let idx = rand::rng().random_range(0..items.len());
            items[idx] = item;
        }
    }
}

/// Seeds the container with `count` items using UUID-based IDs and partition keys.
///
/// Returns a list of seeded item references so operations can target them.
/// Fails immediately on the first upsert error, cancelling remaining work.
pub async fn seed_container(
    container: &ContainerClient,
    count: usize,
    concurrency: usize,
) -> azure_core::Result<Vec<SeededItem>> {
    println!("Seeding {count} items (concurrency: {concurrency})...");

    let mut items = Vec::with_capacity(count);
    for _ in 0..count {
        let id = Uuid::new_v4().to_string();
        let partition_key = Uuid::new_v4().to_string();
        items.push(SeededItem { id, partition_key });
    }

    let completed = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let mut workers = tokio::task::JoinSet::new();
    let mut next = 0;

    while next < count || !workers.is_empty() {
        // Fill workers up to concurrency limit
        while next < count && workers.len() < concurrency {
            let container = container.clone();
            let completed = completed.clone();
            let id = items[next].id.clone();
            let pk = items[next].partition_key.clone();
            let idx = next;
            let total = count;

            workers.spawn(async move {
                let item = PerfItem {
                    id,
                    partition_key: pk.clone(),
                    value: idx as u64,
                    payload: "perf-test-seed-payload".to_string(),
                };

                let result = container
                    .upsert_item(&item.partition_key, &item, None)
                    .await;

                if result.is_ok() {
                    let done = completed.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                    if done.is_multiple_of(200) || done as usize == total {
                        println!("  Seeded {done}/{total} items");
                    }
                }

                (idx, result.err())
            });
            next += 1;
        }

        // Wait for one task to complete; successful completions are consumed
        // from the JoinSet implicitly, errors abort all remaining work.
        match workers.join_next().await {
            Some(Ok((idx, Some(e)))) => {
                eprintln!("Seed error for item {idx}: {e}");
                workers.abort_all();
                return Err(e);
            }
            Some(Ok((_, None))) => {} // Task succeeded, continue
            Some(Err(e)) => {
                workers.abort_all();
                return Err(azure_core::Error::new(
                    azure_core::error::ErrorKind::Other,
                    e,
                ));
            }
            None => {} // No more tasks
        }
    }

    println!("Seeding complete.");
    Ok(items)
}
