// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Queries: parameters, scopes, paging, and resumption.
//!
//! Demonstrates the four shapes of `ContainerClient::query_items`:
//!
//! 1. A query targeted at a single logical partition with parameters.
//! 2. A cross-partition query, iterated page by page so we can read the
//!    request charge and per-page query metrics.
//! 3. Pausing a cross-partition query mid-flight by capturing a
//!    `ContinuationToken` and resuming with a fresh iterator.
//!
//! ## Required setup
//!
//! Same as `cosmos_getting_started`: a `samples` database with an `orders` container
//! partitioned on `/customer`. Insert a few items first (e.g. by running
//! the `cosmos_getting_started` example with different `id`s) so the queries
//! return non-empty results.
//!
//! ## Running
//!
//! ```text
//! cargo run --example cosmos_queries -- \
//!     https://<account>.documents.azure.com:443/ --region "East US" --use-entra
//! ```

use azure_data_cosmos::feed::ContinuationToken;
use azure_data_cosmos::options::{MaxItemCountHint, QueryOptions};
use azure_data_cosmos::{
    AccountEndpoint, AccountReference, CosmosClient, FeedScope, Query, RoutingStrategy,
};
use azure_identity::DeveloperToolsCredential;
use clap::Parser;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::num::NonZeroU32;

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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let client = create_client(&args).await?;
    let items = client
        .database_client(&args.database)
        .container_client(&args.container)
        .await?;

    // Make sure there is at least one item to query so the example produces
    // visible output. Failures here are ignored — the item may already exist.
    let _ = items
        .upsert_item(
            "contoso",
            "o-100",
            &Order {
                id: "o-100".into(),
                customer: "contoso".into(),
                total: 42.5,
            },
            None,
        )
        .await;

    // ----- 1. Single-partition query with parameters. -------------------------
    // `Query::with_parameter` is eager: any `serde::Serialize` value that
    // fails to convert to JSON returns an error here, *before* the query is
    // ever sent to the service.
    let query = Query::from("SELECT * FROM c WHERE c.total > @min").with_parameter("@min", 0.0)?;

    // `FeedScope::partition` constrains the query to a single logical
    // partition — the cheapest and most predictable shape. Cross-partition
    // queries use `FeedScope::full_container()` instead (see below).
    let mut stream = items
        .query_items::<Order>(query, FeedScope::partition("contoso"), None)
        .await?;

    println!("--- single-partition query ---");
    while let Some(row) = stream.try_next().await? {
        println!("  {row:?}");
    }

    // ----- 2. Cross-partition query, page by page. --------------------------
    // `into_pages()` switches an item iterator into a page iterator so we
    // can inspect the per-page response metadata (RU charge, query metrics,
    // etc.) instead of just the items.
    let mut pages = items
        .query_items::<Order>(
            Query::from("SELECT * FROM c WHERE c.total >= 0"),
            FeedScope::full_container(),
            Some(
                QueryOptions::default()
                    .with_populate_query_metrics(true)
                    .with_populate_index_metrics(true),
            ),
        )
        .await?
        .into_pages();

    println!("--- cross-partition pages ---");
    while let Some(page) = pages.try_next().await? {
        println!(
            "  page: {} item(s)  RU={:?}  query_metrics={:?}",
            page.items().len(),
            page.headers().request_charge(),
            page.query_metrics(),
        );
    }

    // ----- 3. Pause and resume with a continuation token. -------------------
    // `MaxItemCountHint::Limit(NonZeroU32)` replaces the legacy `-1` wire
    // sentinel — the Rust SDK never asks callers to pass magic numbers.
    let limit = NonZeroU32::new(1).expect("1 is non-zero");
    let mut pages = items
        .query_items::<Order>(
            Query::from("SELECT * FROM c"),
            FeedScope::full_container(),
            Some(QueryOptions::default().with_max_item_count(MaxItemCountHint::Limit(limit))),
        )
        .await?
        .into_pages();

    let mut token: Option<ContinuationToken> = None;
    if pages.try_next().await?.is_some() {
        // Snapshot the *next* position. The iterator may continue to be
        // used afterwards, but the token resumes from here.
        token = Some(pages.to_continuation_token()?);
    }

    if let Some(token) = token {
        println!("--- resuming from continuation token ---");
        // Drop `pages` (e.g. across a process boundary), then re-issue the
        // *same* query with `with_continuation_token` to pick up from the
        // captured position.
        let mut resumed = items
            .query_items::<Order>(
                Query::from("SELECT * FROM c"),
                FeedScope::full_container(),
                Some(QueryOptions::default().with_continuation_token(token)),
            )
            .await?;
        while let Some(row) = resumed.try_next().await? {
            println!("  {row:?}");
        }
    }

    Ok(())
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
