use appconfiguration::prelude::*;
use azure_identity::DefaultAzureCredentialBuilder;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("AZCONFIG_NAME", "azure-rust-sdk");
    env::set_var(
        "FEATURE_ON_OFF",
        "sdk/appconfiguration/examples/on_off.json",
    );
    let name = std::env::var("AZCONFIG_NAME").expect("Missing AZCONFIG_NAME environment variable.");
    let endpoint = format!("https://{name}.azconfig.io");

    let creds = Arc::new(
        DefaultAzureCredentialBuilder::new()
            .exclude_managed_identity_credential()
            .build(),
    );

    let features = FeatureExplorer::new(&endpoint, creds, None);
    println!("Features {features:?}");

    println!("***dev_on_off***");
    let user = String::from("test");
    let feature_name = String::from("on_off");

    println!(
        "Feature - {:?} is {:?} for User - {:?}",
        feature_name,
        if features.is_enabled(feature_name.clone()) {
            "enabled"
        } else {
            "not enabled"
        },
        user,
    );

    Ok(())
}
