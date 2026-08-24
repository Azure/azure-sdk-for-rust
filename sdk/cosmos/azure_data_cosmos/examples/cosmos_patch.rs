// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Partial updates with PATCH.
//!
//! Demonstrates `ContainerClient::patch_item` and the `PatchInstructions`
//! builder, exercising every variant of `PatchOperation` (`set`, `add`,
//! `replace`, `remove`, `increment`, `move_value`).
//!
//! ## How PATCH actually works
//!
//! A patch runs one of two ways, chosen by `PatchItemOptions::with_strategy`:
//!
//! * **Server-side** — the operation list is sent to the service in a single
//!   request. One round trip, and on a multi-write-region account the service
//!   resolves concurrent patches at the *path* level, so two writers touching
//!   different properties of the same item both survive.
//! * **Client-side** — the SDK reads the item (capturing its ETag), applies
//!   the operations locally, and issues a conditional Replace gated on that
//!   ETag, retrying on 412 up to `with_max_attempts` times.
//!
//! The default, `PatchStrategy::Auto`, runs server-side whenever that is safe:
//! the operation list must survive being resent unchanged (so no `increment`,
//! `remove`, `move_value`, or array append) and must fit the service's
//! 10-operation limit. Otherwise it falls back to the client-side loop.
//!
//! The patch below deliberately includes `increment` and `move_value`, so
//! `Auto` resolves it to the client-side loop. See `ContainerClient::patch_item`
//! rustdoc for the idempotency caveats of each path.
//!
//! ## Required setup
//!
//! `samples` database, `orders` container partitioned on `/customer`.
//!
//! ## Running
//!
//! ```text
//! cargo run --example cosmos_patch -- \
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
        .container_client(&args.container)
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
    // This list contains `increment` and `move_value`, so `Auto` runs it
    // client-side; `with_max_attempts` bounds that loop. A list of only
    // `set`/`replace` ops would instead go to the service in one round trip,
    // where `max_attempts` has no effect.
    let response = items
        .patch_item(
            "contoso",
            "o-200",
            patch,
            Some(
                PatchItemOptions::default()
                    .with_max_attempts(NonZeroU8::new(5).expect("5 is non-zero")),
            ),
        )
        .await?;
    println!(
        "patched  status={:?} RU={:?}",
        response.status(),
        response.headers().request_charge(),
    );

    // The response is the standard `ItemResponse` shape, so we can read the
    // patched item back via `into_model::<T>()` when content-on-write is on.
    let patched: serde_json::Value = response.into_model()?;
    println!("after    {patched:#}");

    // ----- A patch that runs server-side. -----------------------------------
    // Only `set` and `replace`, so the list survives being resent unchanged and
    // `Auto` sends it to the service in a single round trip. Pinning
    // `ServerSide` here makes that explicit and fails loudly if the list ever
    // grows past the service's 10-operation limit, instead of silently
    // reverting to two round trips.
    let server_side = PatchInstructions::new()
        .with_operation(PatchOperation::set("/status", json!("shipped")))
        .with_operation(PatchOperation::replace("/priority", json!("high")));

    let response = items
        .patch_item(
            "contoso",
            "o-200",
            server_side,
            Some(PatchItemOptions::default().with_strategy(PatchStrategy::ServerSide)),
        )
        .await?;
    println!(
        "server   status={:?} RU={:?}",
        response.status(),
        response.headers().request_charge(),
    );

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
