// cargo run --example owned_crates
// Validates that the GitHub team has ownership of all crates.
// If not, it prints the command to add the crate.

use autorust_codegen::crates::list_crate_names;

/// https://github.com/orgs/Azure/teams/azure-sdk-publish-rust
/// https://crates.io/teams/github:azure:azure-sdk-publish-rust
const TEAM: &str = "github:azure:azure-sdk-publish-rust";

use crates_io_api::SyncClient;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let client = &SyncClient::new("azure-sdk-for-rust", std::time::Duration::from_millis(1000))?;
    for crate_name in &list_crate_names()? {
        if !is_owner(client, crate_name)? {
            println!("cargo owner --add {TEAM} -- {crate_name}")
        }
    }
    Ok(())
}

// This looks up each crate individually.
// It would be more efficient to get a list of crates for a user if possible.
// https://github.com/theduke/crates-io-api/issues/52
fn is_owner(client: &SyncClient, crate_name: &str) -> Result<bool> {
    let cr = client.full_crate(crate_name, false)?;
    Ok(cr.owners.iter().any(|owner| owner.login == TEAM))
}
