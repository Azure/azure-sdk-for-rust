#![cfg(all(test, feature = "test_e2e"))]
use azure_core::prelude::*;
use azure_storage::core::prelude::*;
use azure_storage::data_lake::prelude::*;
use chrono::Utc;
use futures::stream::StreamExt;
use std::error::Error;
use std::num::NonZeroU32;
use azure_identity::token_credentials::DefaultCredential;
use azure_identity::token_credentials::TokenCredential;

#[tokio::test]
async fn test_data_lake_file_system_functions() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account = std::env::var("ADSL_STORAGE_ACCOUNT")
        .expect("Set env variable ADSL_STORAGE_ACCOUNT first!");
    let master_key = std::env::var("ADSL_STORAGE_MASTER_KEY")
        .expect("Set env variable ADSL_STORAGE_MASTER_KEY first!");

    let now = Utc::now();
    let file_system_name = format!("azurerustsdk-e2etest-datalake-{}", now.timestamp());

    let http_client = new_http_client();

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);

    println!("getting bearer token...");
    let bearer_token = DefaultCredential::default()
        .get_token("https://storage.azure.com/")
        .await?;

    let data_lake = storage_account_client
        .as_storage_client()
        .as_data_lake_client(account, bearer_token.token.secret().to_owned())?;

    let file_system = data_lake.as_file_system_client(&file_system_name)?;

    let mut properties = Properties::new();
    properties.insert("AddedVia", "Azure SDK for Rust");
    properties.insert("CreatedAt", chrono::Utc::now().to_string());
    println!("creating file system '{}'...", &file_system_name);
    let create_file_system_response = file_system
        .create()
        .properties(&properties)
        .execute()
        .await?;
    println!(
        "create file system response == {:?}",
        create_file_system_response
    );
    println!();

    println!("creating path...");
    let create_path_response = file_system
        .create_path(Context::new(), "file.txt", CreatePathOptions::new())
        .await;

    let create_path_worked = match create_path_response {
        Result::Ok(response) => {
            println!("create path response == {:?}", response);
            true
        },
        Result::Err(err) => {
            println!("create path response error == {:?}", err);
            false
        },
    };
    println!();

    println!("listing file system...");
    let mut stream = Box::pin(
        data_lake
            .list()
            .max_results(NonZeroU32::new(3).unwrap())
            .stream(),
    );

    while let Some(response) = stream.next().await {
        println!("list stream response == {:?}", response);
        println!();
    }

    println!("setting properties...");
    properties.insert("ModifiedBy", "Iota");
    let response = file_system
        .set_properties(Some(&properties))
        .execute()
        .await?;
    println!("set properties response == {:?}", response);
    println!();

    println!("getting properties...");
    let response = file_system.get_properties().execute().await?;
    println!("get properties response == {:?}", response);
    println!();

    println!("deleting file system...");
    let response = file_system.delete().execute().await?;
    println!("file system delete response == {:?}", response);
    println!();
    println!("data lake test done.");

    assert!(create_path_worked);

    Ok(())
}
