// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_data_cosmos::CosmosClient;
use azure_identity::DeveloperToolsCredential;
use clap::{Args, CommandFactory, Parser, Subcommand};
use std::error::Error;

mod batch;
mod create;
mod delete;
mod metadata;
mod patch;
mod query;
mod read;
mod replace;
mod upsert;
mod utils;

/// A set of basic examples for interacting with Cosmos.
///
/// NOTE: This is not intended to be a general-purpose CLI for managing items in Cosmos DB.
/// It exists for illustrative purposes and to simplify ad-hoc end-to-end testing.
#[derive(Clone, Parser)]
struct ProgramArgs {
    #[command(flatten)]
    shared_args: SharedArgs,

    #[command(subcommand)]
    subcommand: Option<Subcommands>,
}

#[derive(Clone, Args)]
struct SharedArgs {
    /// The Cosmos DB endpoint to connect to.
    endpoint: String,

    /// An authentication key to use when connecting to the Cosmos DB account. If omitted, the connection will use Entra ID.
    #[arg(long)]
    key: Option<String>,
}

#[derive(Clone, Subcommand)]
enum Subcommands {
    Batch(batch::BatchCommand),
    Create(create::CreateCommand),
    Delete(delete::DeleteCommand),
    Metadata(metadata::MetadataCommand),
    Query(query::QueryCommand),
    Read(read::ReadCommand),
    Replace(replace::ReplaceCommand),
    Upsert(upsert::UpsertCommand),
    Patch(patch::PatchCommand),
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = ProgramArgs::parse();

    let Some(cmd) = args.subcommand else {
        ProgramArgs::command().print_long_help()?;
        return Ok(());
    };

    let client = create_client(&args.shared_args)?;

    match cmd {
        Subcommands::Batch(cmd) => cmd.run(&client).await,
        Subcommands::Create(cmd) => cmd.run(client).await,
        Subcommands::Delete(cmd) => cmd.run(client).await,
        Subcommands::Metadata(cmd) => cmd.run(client).await,
        Subcommands::Query(cmd) => cmd.run(client).await,
        Subcommands::Read(cmd) => cmd.run(client).await,
        Subcommands::Replace(cmd) => cmd.run(client).await,
        Subcommands::Upsert(cmd) => cmd.run(client).await,
        Subcommands::Patch(cmd) => cmd.run(client).await,
    }
}

fn create_client(args: &SharedArgs) -> Result<CosmosClient, Box<dyn Error>> {
    if let Some(key) = args.key.as_ref() {
        #[cfg(feature = "key_auth")]
        {
            Ok(CosmosClient::with_key(
                &args.endpoint,
                key.clone().into(),
                None,
            )?)
        }
        #[cfg(not(feature = "key_auth"))]
        {
            let _ = key; // Mark 'key' as used to make the compiler happy.
            Err("cannot authenticate with a key unless the 'key_auth' feature is enabled".into())
        }
    } else {
        let cred = DeveloperToolsCredential::new(None).unwrap();
        Ok(CosmosClient::new(&args.endpoint, cred, None)?)
    }
}
