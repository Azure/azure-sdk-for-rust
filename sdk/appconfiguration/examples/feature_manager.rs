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

    let features = FeatureManager::new(creds);
    println!("Features {features:?}");

    println!("*****");
    let mut feature_name = String::from("targeting");
    let user = String::from("test");
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

    println!("*****");
    feature_name = String::from("percentage");
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

    println!("*****");
    feature_name = String::from("on_off");
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

    println!("*****");
    feature_name = String::from("time_window");
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

    println!("*****");
    feature_name = String::from("time_window_never");
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

    println!("*****");
    std::thread::sleep(std::time::Duration::from_secs(20));

    feature_name = String::from("time_window_on");
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
