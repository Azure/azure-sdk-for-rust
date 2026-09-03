// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Partial updates with PATCH.
//!
//! **Preview.** Requires the `preview_patch` feature.
//!
//! Demonstrates `ContainerClient::patch_item` and the `PatchInstructions`
//! builder, exercising every variant of `PatchOperation` (`set`, `add`,
//! `replace`, `remove`, `increment`, `move_value`).
//!
//! ## How PATCH actually works
//!
//! `PatchStrategy::Auto` sends retry-safe lists containing at most 10
//! instructions directly to Cosmos DB. Unsafe or longer lists use the tracked
//! client-side read-modify-write loop:
//!
//! 1. The SDK reads the current item (capturing its ETag).
//! 2. The SDK applies the patch operations locally.
//! 3. The SDK issues a conditional Replace gated on the ETag from step 1.
//! 4. On a 412 Precondition Failed (mid-air collision), the SDK starts another
//!    attempt from step 1 until the total `PatchItemOptions::with_max_attempts`
//!    budget, including the initial attempt, is exhausted.
//!
//! Explicit `ServerSide` never falls back and Cosmos DB rejects more than 10
//! instructions. See `ContainerClient::patch_item` for retry semantics.
//!
//! ## Required setup
//!
//! `samples` database, `orders` container partitioned on `/customer`.
//!
//! ## Running
//!
//! ```text
//! cargo run --features preview_patch --example cosmos_patch -- \
//!     https://<account>.documents.azure.com:443/ --region "East US" --use-entra
//! ```

use azure_data_cosmos::models::{CosmosNumber, PatchInstructions, PatchOperation};
use azure_data_cosmos::options::{PatchItemOptions, PatchStrategy};
use azure_data_cosmos::{AccountEndpoint, AccountReference, CosmosClient, RoutingStrategy};
use azure_identity::DeveloperToolsCredential;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::num::NonZeroU8;

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
    visits: i64,
    tags: Vec<String>,
    legacy_total: Option<f64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let client = create_client(&args).await?;
    let items = client
        .database_client(&args.database)
        .container_client(&args.container, None)
        .await?;

    // Seed an item with a shape rich enough to exercise every PatchOperation.
    let seed = Order {
        id: "o-200".into(),
        customer: "contoso".into(),
        total: 100.0,
        visits: 0,
        tags: vec!["preferred".into()],
        legacy_total: Some(75.0),
    };
    items
        .upsert_item(seed.customer.clone(), &seed.id, &seed, None)
        .await?;

    // ----- Build a PatchInstructions with one of every operation. -----------
    // The builder is deliberately small: there is no separate "patch builder"
    // helper — `with_operation` is the only entry point.
    let patch = PatchInstructions::new()
        // `set` writes a value, creating the leaf if missing (object-only).
        .with_operation(PatchOperation::set("/customer_tier", json!("gold")))
        // `add` appends to arrays or sets a property if the parent exists.
        .with_operation(PatchOperation::add("/tags/-", json!("vip")))
        // `replace` requires the leaf to already exist.
        .with_operation(PatchOperation::replace("/total", json!(125.0)))
        // `remove` deletes a field; cannot target the root.
        .with_operation(PatchOperation::remove("/legacy_total"))
        // `increment` adjusts a JSON number atomically. `CosmosNumber`
        // accepts any standard integer or float type.
        .with_operation(PatchOperation::increment(
            "/visits",
            CosmosNumber::from(1i64),
        ))
        // `move_value` moves a JSON value between paths in a single op.
        .with_operation(PatchOperation::move_value("/tags/0", "/headline_tag"));

    // ----- Issue the patch. -------------------------------------------------
    // This list includes non-idempotent operations, so Auto would choose RMW.
    // Select ClientSide explicitly because `with_max_attempts` applies only to
    // that path. The default is 5; this example sets it explicitly.
    // Very contended items may benefit from a higher cap, but consider
    // re-shaping the workload first.
    let response = items
        .patch_item(
            "contoso",
            "o-200",
            patch,
            Some(
                PatchItemOptions::default()
                    .with_strategy(PatchStrategy::ClientSide)
                    .with_max_attempts(NonZeroU8::new(5).expect("5 is non-zero")),
            ),
        )
        .await?;
    println!(
        "patched  status={:?} RU={:?}",
        response.status(),
        response.headers().request_charge(),
    );

    // PATCH returns the post-image by default. Setting
    // `content_response_on_write` to Disabled suppresses it for either path.
    let patched: serde_json::Value = response.into_model()?;
    println!("after    {patched:#}");

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
