// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Container seeding with test data.

use azure_data_cosmos::clients::ContainerClient;
use uuid::Uuid;

use crate::operations::PerfItem;

/// A reference to a seeded item's ID and partition key.
#[derive(Debug, Clone)]
pub struct SeededItem {
    pub id: String,
    pub partition_key: String,
}

/// Seeds the container with `count` items using UUID-based IDs and partition keys.
///
/// Returns a list of seeded item references so operations can target them.
pub async fn seed_container(
    container: &ContainerClient,
    count: usize,
    concurrency: usize,
) -> azure_core::Result<Vec<SeededItem>> {
    println!("Seeding {count} items (concurrency: {concurrency})...");

    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(concurrency));
    let errors = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let completed = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));

    let mut items = Vec::with_capacity(count);
    for _ in 0..count {
        let id = Uuid::new_v4().to_string();
        let partition_key = Uuid::new_v4().to_string();
        items.push(SeededItem { id, partition_key });
    }

    let mut handles = Vec::with_capacity(count);

    for (i, seeded) in items.iter().enumerate() {
        let container = container.clone();
        let sem = semaphore.clone();
        let errors = errors.clone();
        let completed = completed.clone();
        let id = seeded.id.clone();
        let pk = seeded.partition_key.clone();

        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();

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
                errors.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                eprintln!("Seed error for item {i}: {e}");
            }

            let done = completed.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
            if done % 200 == 0 || done as usize == count {
                println!("  Seeded {done}/{count} items");
            }
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let total_errors = errors.load(std::sync::atomic::Ordering::Relaxed);
    if total_errors > 0 {
        eprintln!("Warning: {total_errors} seed errors occurred");
    }

    println!("Seeding complete.");
    Ok(items)
}
