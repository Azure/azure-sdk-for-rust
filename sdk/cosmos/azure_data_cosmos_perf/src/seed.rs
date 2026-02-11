// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Container seeding with test data.

use azure_data_cosmos::clients::ContainerClient;

use crate::operations::PerfItem;

/// Seeds the container with `count` items using predictable IDs and partition keys.
///
/// Items are upserted so this is safe to call repeatedly. The item IDs follow
/// the pattern `perf-item-{i}` with partition keys `pk-{i}`.
pub async fn seed_container(
    container: &ContainerClient,
    count: usize,
    concurrency: usize,
) -> azure_core::Result<()> {
    println!("Seeding {count} items (concurrency: {concurrency})...");

    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(concurrency));
    let errors = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let completed = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));

    let mut handles = Vec::with_capacity(count);

    for i in 0..count {
        let container = container.clone();
        let sem = semaphore.clone();
        let errors = errors.clone();
        let completed = completed.clone();

        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();

            let item = PerfItem {
                id: format!("perf-item-{i}"),
                partition_key: format!("pk-{i}"),
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
    Ok(())
}
