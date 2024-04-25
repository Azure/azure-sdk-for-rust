use azure_core::auth::TokenCredential;
use azure_identity::AzureauthCliCredential;
use clap::Parser;
use std::error::Error;

#[derive(Debug, Parser)]
struct Args {
    tenant_id: String,
    client_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let Args {
        tenant_id,
        client_id,
    } = Args::parse();

    let creds = AzureauthCliCredential::new(tenant_id, client_id);
    let res = creds
        // Get an access token for Azure Devops
        .get_token(&["499b84ac-1321-427f-aa17-267ca6975798/.default"])
        .await?;

    println!("azureauth cli response == {res:?}");

    Ok(())
}
