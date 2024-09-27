// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_data_cosmos::CosmosClient;
use azure_identity::DefaultAzureCredential;
use clap::{Args, CommandFactory, Parser, Subcommand};
use std::{error::Error, sync::Arc};

mod create;
mod metadata;
mod query;

/// An example to show querying a Cosmos DB container.
#[derive(Clone, Parser)]
struct ProgramArgs {
    #[clap(flatten)]
    shared_args: SharedArgs,

    #[command(subcommand)]
    subcommand: Option<Subcommands>,
}

#[derive(Clone, Args)]
struct SharedArgs {
    /// The Cosmos DB endpoint to connect to.
    endpoint: String,

    /// An authentication key to use when connecting to the Cosmos DB account. If omitted, the connection will use Entra ID.
    #[clap(long)]
    #[cfg(feature = "key_auth")]
    key: Option<String>,
}

#[derive(Clone, Subcommand)]
enum Subcommands {
    Query(query::QueryCommand),
    Metadata(metadata::MetadataCommand),
    Create(create::CreateCommand),
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = ProgramArgs::parse();

    let Some(cmd) = args.subcommand else {
        ProgramArgs::command().print_long_help()?;
        return Ok(());
    };

    let client = create_client(&args.shared_args);

    match cmd {
        Subcommands::Query(cmd) => cmd.run(client).await,
        Subcommands::Metadata(cmd) => cmd.run(client).await,
        Subcommands::Create(cmd) => cmd.run(client).await,
    }
}

#[cfg(feature = "key_auth")]
fn create_client(args: &SharedArgs) -> CosmosClient {
    if let Some(key) = args.key.as_ref() {
        CosmosClient::with_key(&args.endpoint, key.clone(), None).unwrap()
    } else {
        let cred = DefaultAzureCredential::new().map(Arc::new).unwrap();
        CosmosClient::new(&args.endpoint, cred, None).unwrap()
    }
}

#[cfg(not(feature = "key_auth"))]
fn create_client(args: &SharedArgs) -> CosmosClient {
    let cred = DefaultAzureCredential::new().map(Arc::new).unwrap();
    CosmosClient::new(&args.endpoint, cred, None).unwrap()
}
