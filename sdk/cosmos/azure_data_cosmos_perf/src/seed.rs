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

    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(concurrency));
    let failed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let completed = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));

    let mut items = Vec::with_capacity(count);
    for _ in 0..count {
        let id = Uuid::new_v4().to_string();
        let partition_key = Uuid::new_v4().to_string();
        items.push(SeededItem { id, partition_key });
    }

    let mut handles: Vec<tokio::task::JoinHandle<Option<azure_core::Error>>> =
        Vec::with_capacity(count);

    for (i, seeded) in items.iter().enumerate() {
        let container = container.clone();
        let sem = semaphore.clone();
        let failed = failed.clone();
        let completed = completed.clone();
        let id = seeded.id.clone();
        let pk = seeded.partition_key.clone();

        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();

            if failed.load(std::sync::atomic::Ordering::Relaxed) {
                return None;
            }

            let item = PerfItem {
                id,
                partition_key: pk.clone(),
                value: i as u64,
                payload: "perf-test-seed-payload".to_string(),
            };

            if let Err(e) = container
                .upsert_item(&item.partition_key, &item, None)
                .await
            {
                failed.store(true, std::sync::atomic::Ordering::Relaxed);
                eprintln!("Seed error for item {i}: {e}");
                return Some(e);
            }

            let done = completed.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
            if done.is_multiple_of(200) || done as usize == count {
                println!("  Seeded {done}/{count} items");
            }
            None
        }));
    }

    let mut first_error: Option<azure_core::Error> = None;
    for handle in handles {
        if let Some(e) = handle.await.unwrap() {
            if first_error.is_none() {
                first_error = Some(e);
            }
        }
    }

    if let Some(e) = first_error {
        return Err(e);
    }

    println!("Seeding complete.");
    Ok(items)
}
