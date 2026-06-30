// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Background refresher for the shared feed-range cache.
//!
//! Spawned once per run from [`runner::run`](crate::runner::run). On each
//! tick it calls `ContainerClient::read_feed_ranges`, records the latency
//! under the synthetic stats line `ReadFeedRanges`, and swaps the new
//! snapshot into the shared cache used by
//! [`FeedRangeQueryOperation`](super::feed_range_query::FeedRangeQueryOperation).
//!
//! Errors are recorded as `record_error("ReadFeedRanges")` and the
//! previous cache is preserved — a transient lookup failure must not
//! clobber a known-good routing map.

use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::feed::FeedRange;

use crate::operations::feed_range_query::FeedRangeCache;
use crate::stats::Stats;

/// Stats line name for refresher latency / errors.
pub const READ_FEED_RANGES_STAT: &str = "ReadFeedRanges";

pub struct FeedRangeRefresher {
    container: ContainerClient,
    cache: FeedRangeCache,
    interval: Duration,
}

impl FeedRangeRefresher {
    pub fn new(container: ContainerClient, cache: FeedRangeCache, interval: Duration) -> Self {
        Self {
            container,
            cache,
            interval,
        }
    }

    /// Runs the refresh loop until `cancelled` flips to `true`.
    pub async fn run(self, stats: Arc<Stats>, cancelled: Arc<AtomicBool>) {
        let Self {
            container,
            cache,
            interval,
        } = self;
        let container = Arc::new(container);

        // Poll cancellation on a tighter beat than `interval` so a Ctrl+C
        // during a long sleep stops the task promptly.
        let poll_step = Duration::from_millis(250).min(interval);

        loop {
            if cancelled.load(Ordering::Relaxed) {
                break;
            }

            let mut waited = Duration::ZERO;
            while waited < interval {
                if cancelled.load(Ordering::Relaxed) {
                    return;
                }
                let step = poll_step.min(interval - waited);
                tokio::time::sleep(step).await;
                waited += step;
            }

            let container = Arc::clone(&container);
            refresh_once(&stats, &cache, || async move {
                container.read_feed_ranges(None).await
            })
            .await;
        }
    }
}

/// Performs one refresh cycle: invokes `fetch`, records latency or
/// error, swaps the cache on success. Extracted so the cache-on-error
/// invariant is unit-testable without an emulator.
async fn refresh_once<F, Fut>(stats: &Stats, cache: &FeedRangeCache, fetch: F)
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = azure_data_cosmos::Result<Vec<FeedRange>>>,
{
    let start = Instant::now();
    match fetch().await {
        Ok(ranges) if ranges.is_empty() => {
            // Empty list from a healthy account is itself a fault; keep
            // the previous cache and log so the operator notices.
            stats.record_error(READ_FEED_RANGES_STAT);
            eprintln!("warning: read_feed_ranges returned empty list; keeping previous cache");
        }
        Ok(ranges) => {
            stats.record_latency(READ_FEED_RANGES_STAT, start.elapsed(), None);
            let mut guard = cache.write().expect("feed-range cache lock poisoned");
            *guard = Arc::new(ranges);
        }
        Err(e) => {
            stats.record_error(READ_FEED_RANGES_STAT);
            eprintln!("warning: read_feed_ranges failed: {e}; keeping previous cache");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::RwLock;

    use azure_data_cosmos::CosmosStatus;
    use azure_data_cosmos_driver::error::CosmosError as DriverCosmosError;

    fn seed_cache(n: usize) -> FeedRangeCache {
        let ranges: Vec<FeedRange> = (0..n).map(|_| FeedRange::full()).collect();
        Arc::new(RwLock::new(Arc::new(ranges)))
    }

    fn cache_len(c: &FeedRangeCache) -> usize {
        c.read().unwrap().len()
    }

    #[tokio::test]
    async fn success_replaces_cache_and_records_latency() {
        let stats = Stats::new(&[READ_FEED_RANGES_STAT]);
        let cache = seed_cache(2);
        refresh_once(&stats, &cache, || async {
            Ok(vec![
                FeedRange::full(),
                FeedRange::full(),
                FeedRange::full(),
            ])
        })
        .await;
        assert_eq!(cache_len(&cache), 3);
    }

    #[tokio::test]
    async fn error_keeps_previous_cache() {
        let stats = Stats::new(&[READ_FEED_RANGES_STAT]);
        let cache = seed_cache(2);
        refresh_once(&stats, &cache, || async {
            Err(DriverCosmosError::builder()
                .with_status(CosmosStatus::TRANSPORT_GENERATED_503)
                .with_message("simulated transport failure")
                .build()
                .into())
        })
        .await;
        assert_eq!(cache_len(&cache), 2);
    }

    #[tokio::test]
    async fn empty_result_keeps_previous_cache() {
        let stats = Stats::new(&[READ_FEED_RANGES_STAT]);
        let cache = seed_cache(2);
        refresh_once(&stats, &cache, || async { Ok(Vec::new()) }).await;
        assert_eq!(cache_len(&cache), 2);
    }
}
