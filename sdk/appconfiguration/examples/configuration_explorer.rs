use appconfiguration::prelude::*;
use azure_identity::DefaultAzureCredentialBuilder;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("AZCONFIG_NAME", "azure-rust-sdk");
    // env::set_var("FEATURE_FETCH_ALL_OFF", "off");

    let creds = Arc::new(
        DefaultAzureCredentialBuilder::new()
            .exclude_managed_identity_credential()
            .build(),
    );

    let configuration_manager = ConfigurationExplorer::new(creds);

    let mut value = configuration_manager.get_value(String::from("t"));
    println!("Value {value:?}");
    println!("****");

    value = configuration_manager.get_value(String::from("first_key"));
    println!("Value {value:?}");
    println!("****");

    let mut values = configuration_manager.get_values(String::from("first_key"));
    println!("Values {values:?}");
    println!("****");

    values = configuration_manager.get_values(String::from("first_label"));
    println!("Values {values:?}");
    println!("****");

    Ok(())
}
