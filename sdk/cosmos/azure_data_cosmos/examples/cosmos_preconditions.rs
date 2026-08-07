// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Concurrency-safe updates with ETags and `Precondition::if_match`.
//!
//! Whenever you want to mutate an existing item without losing concurrent
//! writes from other clients, you need a read-modify-write loop with an
//! `If-Match` precondition:
//!
//! 1. Read the current item — Cosmos returns its ETag.
//! 2. Compute the new value from what you read.
//! 3. Replace the item with `Precondition::if_match(<that ETag>)`.
//! 4. If the service returns 412 Precondition Failed, somebody else got
//!    there first; loop back to step 1 and try again.
//!
//! The 412 surfaces as `CosmosError::status().is_precondition_failed() == true`.
//! Without this loop, a plain "read, change, replace" can silently clobber
//! another writer's update — the classic lost-update problem.
//!
//! To make the example exercise the retry path deterministically, the
//! helper below also simulates a concurrent writer slipping in between
//! the loop's first read and first write. See the comment in
//! [`update_with_retry`] for details.
//!
//! ## Required setup
//!
//! Same as `cosmos_getting_started`: a `samples` database with an `orders`
//! container partitioned on `/customer`.
//!
//! ## Running
//!
//! ```text
//! cargo run --example cosmos_preconditions -- \
//!     https://<account>.documents.azure.com:443/ --region "East US" --use-entra
//! ```

use azure_core::http::Etag;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::options::{ItemWriteOptions, Precondition};
use azure_data_cosmos::{AccountEndpoint, AccountReference, CosmosClient, RoutingStrategy};
use azure_identity::DeveloperToolsCredential;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::error::Error;

const ITEM_ID: &str = "o-pre-1";
const PARTITION: &str = "contoso";
const MAX_RETRIES: usize = 5;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Order {
    id: String,
    customer: String,
    total: f64,
    visits: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let client = create_client(&args).await?;
    let items = client
        .database_client(&args.database)
        .container_client(&args.container)
        .await?;

    // Make sure the item exists at a known starting state.
    let seed = Order {
        id: ITEM_ID.into(),
        customer: PARTITION.into(),
        total: 100.0,
        visits: 0,
    };
    items.upsert_item(PARTITION, &seed.id, &seed, None).await?;

    // Apply a concurrency-safe mutation: bump the running total by 1.0.
    // If another writer races us between our read and our write, the loop
    // notices via the 412 response and tries again on a fresh read.
    let final_total = update_with_retry(&items, |order| order.total + 1.0).await?;
    println!("update_with_retry: final total = {final_total}");

    Ok(())
}

/// Read-modify-write helper: reads the item, applies `mutate` to compute a
/// new `total`, and writes it back guarded by `If-Match` against the ETag
/// from the read. On a 412 Precondition Failed (mid-air collision with
/// another writer), it re-reads and tries again, up to `MAX_RETRIES` times.
///
/// Any non-412 error is surfaced to the caller.
async fn update_with_retry(
    items: &ContainerClient,
    mutate: impl Fn(&Order) -> f64,
) -> Result<f64, Box<dyn Error>> {
    for attempt in 1..=MAX_RETRIES {
        // Step 1: fresh read on every attempt. We need the matching ETag
        // for the version of the item we're about to base our write on.
        let read = items.read_item(PARTITION, ITEM_ID, None).await?;
        let etag: Etag = read
            .headers()
            .etag()
            .expect("read response must include an ETag")
            .clone();

        // Step 2: compute the new value from what we just read.
        let mut order: Order = read.into_model()?;
        order.total = mutate(&order);
        order.visits += 1;

        // ---- DEMO ONLY -------------------------------------------------
        // Simulate a concurrent writer landing between *our* read above
        // and *our* write below, but only on the first attempt.
        // In real code, the racing writer is some other process.
        if attempt == 1 {
            let competing = Order {
                id: ITEM_ID.into(),
                customer: PARTITION.into(),
                total: 999.0,
                visits: 99,
            };
            items
                .replace_item(PARTITION, ITEM_ID, &competing, None)
                .await?;
        }
        // ----------------------------------------------------------------

        // Step 3: replace with `If-Match`. The service compares our ETag
        // against the current server-side ETag; if they don't match, it
        // rejects the write with 412 instead of silently overwriting the
        // other writer's update.
        let result = items
            .replace_item(
                PARTITION,
                ITEM_ID,
                &order,
                Some(ItemWriteOptions::default().with_precondition(Precondition::if_match(etag))),
            )
            .await;

        match result {
            Ok(_) => return Ok(order.total),
            // Step 4: 412 means somebody else's write landed between our
            // read and our write. Loop and try again from a fresh read.
            Err(err) if err.status().is_precondition_failed() => {
                eprintln!("  attempt {attempt}: 412, retrying");
                continue;
            }
            // Any other error (404, network, etc.) is the caller's problem.
            Err(err) => return Err(err.into()),
        }
    }
    Err("exceeded retry budget while updating with optimistic concurrency".into())
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
