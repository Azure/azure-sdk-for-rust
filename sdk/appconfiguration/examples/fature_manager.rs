use appconfiguration::prelude::*;
use azure_identity::DefaultAzureCredentialBuilder;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let creds = Arc::new(
        DefaultAzureCredentialBuilder::new()
            .exclude_managed_identity_credential()
            .build(),
    );

    let features = FeatureHolder::new(creds);
    println!("Features {features:?}");

    if features.is_enabled(String::from("test_feature")) {
        println!("test_feature enabled for user - test!");
    }

    if !features.is_enabled(String::from("test_feature_1")) {
        println!("test_feature_1 not enabled for user - test!");
    }

    if !features.is_enabled(String::from("test_feature_33")) {
        println!("test_feature_3 not enabled for user - test!");
    }

    Ok(())
}
