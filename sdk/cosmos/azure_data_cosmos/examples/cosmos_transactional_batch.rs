// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Atomic multi-item operations with `TransactionalBatch`.
//!
//! All operations in a transactional batch share a single logical partition
//! key and either *all* commit or *none* do. This is the right tool for
//! invariants that span multiple items but live within a single partition
//! (e.g. order header + order lines).
//!
//! ## Required setup
//!
//! `samples` database, `orders` container partitioned on `/customer`.
//!
//! ## Running
//!
//! ```text
//! cargo run --example cosmos_transactional_batch -- \
//!     https://<account>.documents.azure.com:443/ --region "East US" --use-entra
//! ```

use azure_data_cosmos::options::{BatchOptions, ContentResponseOnWrite, OperationOptionsBuilder};
use azure_data_cosmos::{
    AccountEndpoint, AccountReference, CosmosClient, RoutingStrategy, TransactionalBatch,
};
use azure_identity::DeveloperToolsCredential;
use clap::Parser;
use serde::{Deserialize, Serialize};
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

    // ----- Build a batch with mixed sub-operations. -------------------------
    // Every sub-operation MUST be in the same logical partition, set once
    // when the batch is constructed.
    let batch = TransactionalBatch::new("contoso")
        // Sub-ops that serialize a body return `Result<Self>` so that
        // serialization errors surface immediately.
        .create_item(Order {
            id: "o-300".into(),
            customer: "contoso".into(),
            total: 10.0,
        })?
        .upsert_item(
            Order {
                id: "o-301".into(),
                customer: "contoso".into(),
                total: 20.0,
            },
            None,
        )?
        // `read_item` and `delete_item` don't serialize anything, so they
        // return `Self` directly.
        .read_item("o-300", None)
        .delete_item("o-301", None);

    // ----- Execute and inspect per-op results. ------------------------------
    // We need to enable `ContentResponseOnWrite` to get bodies back from any operation in a batch, even the reads.
    let options = OperationOptionsBuilder::new()
        .with_content_response_on_write(ContentResponseOnWrite::Enabled)
        .build();
    let response = items
        .execute_transactional_batch(
            batch,
            Some(BatchOptions::default().with_operation_options(options)),
        )
        .await?;
    let results = response.into_model()?;

    println!(
        "batch top-level status: {} sub-op(s)",
        results.results().len()
    );
    for (i, op) in results.results().iter().enumerate() {
        println!(
            "  [{i}] status={} RU={:?} success={} body_len={}",
            op.status_code(),
            op.request_charge(),
            op.is_success(),
            op.resource_body().map(|v| v.to_string().len()).unwrap_or(0),
        );
    }

    // ----- Decode a sub-op body as a typed model. ---------------------------
    // The `read_item` we issued at index 2 should round-trip our `Order`.
    if let Some(read_op) = results.results().get(2) {
        if read_op.is_success() {
            let order: Order = read_op.into_model()?.expect("read returned a body");
            println!("  read result: {order:?}");
        }
    }

    // ----- Partial-failure detection. ---------------------------------------
    // When one sub-op fails, every *other* sub-op reports HTTP 424
    // ("Failed Dependency") to make it easy to find the actual culprit.
    if let Some(culprit) = results
        .results()
        .iter()
        .find(|op| !op.is_success() && op.status_code() != 424)
    {
        eprintln!(
            "batch failed: first non-dependency error was {} (sub_status={:?})",
            culprit.status_code(),
            culprit.substatus_code(),
        );
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
