// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Distributing query work across `FeedRange`s.
//!
//! Splits a cross-partition query into one sub-query per physical partition
//! using `FeedRange` + `FeedScope::range`, then fans the work out across
//! `tokio` tasks. Each sub-query has its own continuation token so a worker
//! pool can checkpoint and resume independently.
//!
//! Also demonstrates `feed_range_from_partition_key`, which lives on
//! `ContainerClient` (not on `FeedRange` itself) because resolving a
//! partition key into ranges requires the container's *partition key
//! definition*, which the container client caches. Once a partition key
//! has been resolved into a range, `FeedRange::is_subset_of` answers the
//! question "which physical partition will Cosmos route this key to?".
//!
//! ## Note on physical partitions
//!
//! Newly-created containers typically start with a single physical
//! partition, in which case `read_feed_ranges` returns one `FeedRange` and
//! the per-range fan-out collapses to a single task. To force the service
//! to provision multiple physical partitions, request more than 10,000
//! RU/s of reserved throughput on the container or database. On a live
//! Cosmos account that incurs the corresponding extra cost; on the local
//! [Cosmos DB Emulator] you can emulate it for free.
//!
//! [Cosmos DB Emulator]: https://learn.microsoft.com/azure/cosmos-db/emulator
//!
//! ## Required setup
//!
//! Same as the other examples: a `samples` database with an `orders`
//! container partitioned on `/customer`.
//!
//! ## Running
//!
//! ```text
//! cargo run --example cosmos_feed_ranges -- \
//!     https://<account>.documents.azure.com:443/ --region "East US" --use-entra
//! ```

use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::feed::FeedRange;
use azure_data_cosmos::{
    AccountEndpoint, AccountReference, CosmosClient, FeedScope, Query, RoutingStrategy,
};
use azure_identity::DeveloperToolsCredential;
use clap::Parser;
use futures::TryStreamExt;
use serde::Deserialize;
use std::error::Error;

#[derive(Parser)]
struct Args {
    endpoint: String,
    #[arg(long)]
    region: String,
    #[arg(long)]
    use_entra: bool,
    #[arg(long, conflicts_with = "use_entra")]
    key: Option<String>,
    #[arg(long, default_value = "samples")]
    database: String,
    #[arg(long, default_value = "orders")]
    container: String,
}

#[derive(Debug, Deserialize)]
struct Order {
    #[allow(dead_code)]
    id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let client = create_client(&args).await?;
    let items = client
        .database_client(&args.database)
        .container_client(&args.container)
        .await?;

    // ----- 1. Enumerate the container's physical partitions. ----------------
    // Each `FeedRange` covers a contiguous slice of the container's
    // effective-partition-key (EPK) space. `min_inclusive` and
    // `max_exclusive` return the hex-encoded EPK bounds — useful for
    // logging, diagnostics, and persisting per-range checkpoints — but
    // applications should still treat the values as opaque labels and
    // never hand-construct a `FeedRange` from them.
    let ranges: Vec<FeedRange> = items.read_feed_ranges(None).await?;
    println!(
        "container is split across {} physical partition(s)",
        ranges.len()
    );
    for (i, range) in ranges.iter().enumerate() {
        println!(
            "  range #{i}: [{min}, {max})",
            min = range.min_inclusive(),
            max = range.max_exclusive(),
        );
    }

    // ----- 2. One task per range, all running in parallel. ------------------
    // Pulling the per-range work into its own `async fn` lets `tokio::spawn`
    // infer the future type, so we don't need an `Ok::<_, _>(...)` turbofish
    // inside the closure.
    let query = Query::from("SELECT c.id FROM c");
    let mut tasks = Vec::with_capacity(ranges.len());
    for range in &ranges {
        tasks.push(tokio::spawn(count_range(
            items.clone(),
            query.clone(),
            range.clone(),
        )));
    }

    let mut total = 0u64;
    for task in tasks {
        // First `?`: was the task cancelled / did it panic?
        // Second `?`: did the per-range work return a CosmosError?
        total += task.await??;
    }
    println!("scanned {total} item(s) across all partitions");

    // ----- 3. Resolve a known partition key down to a `FeedRange`. ----------
    // Useful when the application already knows the partition key it wants
    // to operate on but still wants the routing-aware `FeedRange`
    // representation (for example, to feed it into the same worker pool as
    // above). Single-component keys always return exactly one range.
    let resolved = items.feed_range_from_partition_key("contoso", None).await?;
    println!(
        "partition key 'contoso' resolves to {} range(s)",
        resolved.len()
    );
    debug_assert_eq!(resolved.len(), 1);
    let key_range = &resolved[0];

    // ----- 4. Find the physical partition that owns that key. --------------
    // `FeedRange::is_subset_of` lets callers ask "is range A entirely
    // contained within range B?". Asking it for each physical range from
    // step 1 identifies which physical partition Cosmos will route the
    // partition key to, which is useful for diagnostics, hot-partition
    // analysis, and pinning per-key work to a specific worker.
    let owning = ranges
        .iter()
        .position(|range| key_range.is_subset_of(range))
        .ok_or("partition key 'contoso' did not resolve to any physical range")?;
    println!(
        "partition key 'contoso' lives in physical range #{owning} of {}",
        ranges.len()
    );

    Ok(())
}

/// Worker for one `FeedRange`. Returns the number of items scanned.
///
/// Lives in its own `async fn` so `tokio::spawn(count_range(...))` infers the
/// future's `Output` type without a closure-level type annotation.
async fn count_range(
    items: ContainerClient,
    query: Query,
    range: FeedRange,
) -> azure_data_cosmos::Result<u64> {
    let mut stream = items
        .query_items::<Order>(query, FeedScope::range(range), None)
        .await?;
    let mut count = 0u64;
    while stream.try_next().await?.is_some() {
        count += 1;
    }
    Ok(count)
}

async fn create_client(args: &Args) -> Result<CosmosClient, Box<dyn Error>> {
    let endpoint: AccountEndpoint = args.endpoint.parse()?;
    let strategy = RoutingStrategy::ProximityTo(args.region.clone().into());

    if let Some(key) = args.key.as_ref() {
        #[cfg(feature = "key_auth")]
        {
            let account = AccountReference::with_authentication_key(
                endpoint,
                azure_core::credentials::Secret::from(key.clone()),
            );
            return Ok(CosmosClient::builder().build(account, strategy).await?);
        }
        #[cfg(not(feature = "key_auth"))]
        {
            let _ = (key, endpoint, strategy);
            return Err("--key auth is opt-in; rebuild with `--features key_auth` to enable it (or use `--use-entra` for Microsoft Entra ID, which is recommended)".into());
        }
    }

    if !args.use_entra {
        return Err("specify either --use-entra or --key <KEY>".into());
    }

    let credential = DeveloperToolsCredential::new(None)?;
    let account = AccountReference::with_credential(endpoint, credential);
    Ok(CosmosClient::builder().build(account, strategy).await?)
}
