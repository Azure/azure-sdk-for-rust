// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Getting started: configure a client and run point operations.
//!
//! This example walks through the basics of using `azure_data_cosmos`:
//!
//! 1. Build a `CosmosClient`, opting into the most useful client-level
//!    configuration (routing strategy, user-agent suffix).
//! 2. Get a `ContainerClient` for an existing container.
//! 3. Run the four point operations: create, read, replace, delete.
//!
//! Optimistic concurrency / ETag handling is covered separately in the
//! `cosmos_preconditions` example.
//!
//! ## Required setup (run once before this example)
//!
//! ```text
//! az cosmosdb sql database create  -a <account> -g <rg> -n samples
//! az cosmosdb sql container create -a <account> -g <rg> -d samples \
//!     -n orders --partition-key-path /customer --throughput 400
//! ```
//!
//! ## Running
//!
//! Microsoft Entra ID is the recommended way to authenticate. Shared-account-key
//! auth is supported but is intentionally opt-in via the `key_auth` Cargo
//! feature so that Entra ID is the default path for new applications.
//!
//! ```text
//! # Recommended: Microsoft Entra ID via the developer credential chain.
//! cargo run --example cosmos_getting_started -- \
//!     https://<account>.documents.azure.com:443/ --region "East US" --use-entra
//!
//! # Opt-in: shared-account-key auth (requires the `key_auth` feature).
//! cargo run --example cosmos_getting_started --features key_auth -- \
//!     https://<account>.documents.azure.com:443/ --region "East US" --key <KEY>
//! ```

use azure_data_cosmos::options::UserAgentSuffix;
use azure_data_cosmos::{AccountEndpoint, AccountReference, CosmosClient, RoutingStrategy};
use azure_identity::DeveloperToolsCredential;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Parser)]
struct Args {
    /// The Cosmos DB endpoint to connect to (e.g. https://acct.documents.azure.com:443/).
    endpoint: String,

    /// The Azure region where the application is running (e.g. "East US").
    #[arg(long)]
    region: String,

    /// Authenticate with Microsoft Entra ID (via DeveloperToolsCredential).
    #[arg(long)]
    use_entra: bool,

    /// Authenticate with a shared account key. Opt-in only — requires the
    /// `key_auth` Cargo feature. Prefer `--use-entra` when possible.
    #[arg(long, conflicts_with = "use_entra")]
    key: Option<String>,

    /// The database to use. Defaults to `samples`.
    #[arg(long, default_value = "samples")]
    database: String,

    /// The container to use. Defaults to `orders`.
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

    // ----- Build the client. ------------------------------------------------
    // The client is the entry point to every Cosmos DB resource. It owns
    // the routing map and the retry/backoff policy, so a typical app should
    // construct one and clone it everywhere it's needed (CosmosClient is
    // cheap to clone).
    let client = create_client(&args).await?;

    // A `ContainerClient` is cheap to obtain and caches the container's
    // partition-key definition on first use, so call this once per
    // container per app.
    let items = client
        .database_client(&args.database)
        .container_client(&args.container)
        .await?;

    // ----- 1. Create a brand-new item. --------------------------------------
    // The PartitionKey value MUST match the value that the partition-key
    // path (`/customer` in our setup) points to inside the item body.
    let new_order = Order {
        id: "o-100".into(),
        customer: "contoso".into(),
        total: 42.5,
    };
    let create_response = items
        .create_item(new_order.customer.clone(), &new_order.id, &new_order, None)
        .await?;
    println!(
        "created  status={:?} RU={:?}",
        create_response.status(),
        create_response.headers().request_charge(),
    );

    // ----- 2. Read it back. -------------------------------------------------
    // Read responses always include the body, so `into_model::<T>()` is
    // the natural way to deserialize the resource into an application
    // type.
    let read_response = items.read_item("contoso", "o-100", None).await?;
    let order: Order = read_response.into_model()?;
    println!("read     {order:?}");

    // ----- 3. Replace the item. ---------------------------------------------
    // `replace_item` is a last-write-wins update by default. To make the
    // replace conditional on the item's current ETag (optimistic
    // concurrency), see the `cosmos_preconditions` example.
    let updated = Order {
        total: 99.99,
        ..order
    };
    let replace_response = items
        .replace_item(updated.customer.clone(), &updated.id, &updated, None)
        .await?;
    println!(
        "replaced status={:?} RU={:?}",
        replace_response.status(),
        replace_response.headers().request_charge(),
    );

    // ----- 4. Tidy up. ------------------------------------------------------
    items.delete_item("contoso", "o-100", None).await?;
    println!("deleted  o-100");

    Ok(())
}

/// Builds a `CosmosClient` from the parsed command-line arguments.
///
/// Either `--use-entra` (Microsoft Entra ID via `DeveloperToolsCredential`,
/// recommended) or `--key <KEY>` (shared-account-key auth, opt-in via the
/// `key_auth` Cargo feature) must be set.
async fn create_client(args: &Args) -> Result<CosmosClient, Box<dyn Error>> {
    // The endpoint string is parsed into a strongly-typed `AccountEndpoint`
    // up-front so any malformed URL fails before we open a connection.
    let endpoint: AccountEndpoint = args.endpoint.parse()?;

    // `RoutingStrategy::ProximityTo(<region>)` tells the client to prefer
    // replicas closest to the given region. For multi-region apps that
    // want to override the natural ordering, use
    // `RoutingStrategy::PreferredRegions(vec![...])` instead.
    let strategy = RoutingStrategy::ProximityTo(args.region.clone().into());

    // A user-agent suffix shows up in service-side request logs and is the
    // easiest way to mark traffic from a specific app or build. Limited to
    // 25 HTTP-header-safe characters.
    let user_agent_suffix = UserAgentSuffix::new("samples-app");

    if let Some(key) = args.key.as_ref() {
        // Build with a shared account key. The `key_auth` Cargo feature is
        // off by default — not because key auth is expensive to compile, but
        // because we strongly encourage Entra ID for new applications and
        // want opting into key-based auth to be a deliberate choice.
        #[cfg(feature = "key_auth")]
        {
            let account = AccountReference::with_authentication_key(
                endpoint,
                azure_core::credentials::Secret::from(key.clone()),
            );
            return Ok(CosmosClient::builder()
                .with_user_agent_suffix(user_agent_suffix)
                .build(account, strategy)
                .await?);
        }
        #[cfg(not(feature = "key_auth"))]
        {
            let _ = (key, endpoint, strategy, user_agent_suffix);
            return Err("--key auth is opt-in; rebuild with `--features key_auth` to enable it (or use `--use-entra` for Microsoft Entra ID, which is recommended)".into());
        }
    }

    if !args.use_entra {
        return Err("specify either --use-entra or --key <KEY>".into());
    }

    // `DeveloperToolsCredential` chains `az login`, VS, VS Code, etc. — the
    // same chain customers already use for the rest of the Azure SDK.
    let credential = DeveloperToolsCredential::new(None)?;
    let account = AccountReference::with_credential(endpoint, credential);
    Ok(CosmosClient::builder()
        .with_user_agent_suffix(user_agent_suffix)
        .build(account, strategy)
        .await?)
}
