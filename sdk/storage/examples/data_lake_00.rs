use azure_core::prelude::IfMatchCondition;
use azure_core::prelude::*;
use azure_identity::token_credentials::DefaultCredential;
use azure_identity::token_credentials::TokenCredential;
use azure_storage::core::prelude::*;
use azure_storage::data_lake::prelude::*;
use chrono::Utc;
use futures::stream::StreamExt;
use std::error::Error;
use std::num::NonZeroU32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let account = std::env::var("ADLSGEN2_STORAGE_ACCOUNT")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCOUNT first!");
    let master_key = std::env::var("ADLSGEN2_STORAGE_MASTER_KEY")
        .expect("Set env variable ADLSGEN2_STORAGE_MASTER_KEY first!");

    let now = Utc::now();
    let file_system_name = format!("azurerustsdk-datalake-example-{}", now.timestamp());

    let http_client = new_http_client();

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);

    let resource_id = "https://storage.azure.com/";
    println!("getting bearer token for '{}'...", resource_id);
    let bearer_token = DefaultCredential::default().get_token(resource_id).await?;
    println!("token expires on {}", bearer_token.expires_on);
    println!();

    let storage_client = storage_account_client.as_storage_client();
    let data_lake = DataLakeClient::new(
        storage_client,
        account,
        bearer_token.token.secret().to_owned(),
        None,
    );

    let file_system = data_lake
        .clone()
        .into_file_system_client(file_system_name.to_string());

    let mut fs_properties = Properties::new();
    fs_properties.insert("AddedVia", "Azure SDK for Rust");

    println!("creating file system '{}'...", &file_system_name);
    let create_fs_response = file_system
        .create()
        .properties(&fs_properties)
        .execute()
        .await?;
    println!("create file system response == {:?}", create_fs_response);
    println!();

    println!("listing file systems...");
    let mut stream = Box::pin(
        data_lake
            .list()
            .max_results(NonZeroU32::new(3).unwrap())
            .stream(),
    );
    while let Some(list_fs_response) = stream.next().await {
        println!("list file system response == {:?}", list_fs_response);
        println!();
    }

    println!("getting file system properties...");
    let get_fs_props_response = file_system.get_properties().execute().await?;
    println!(
        "get file system properties response == {:?}",
        get_fs_props_response
    );
    println!();

    let file_name = "example-file.txt";

    println!("creating path '{}'...", file_name);
    let create_path_response = file_system
        .create_path(Context::default(), file_name, CreatePathOptions::default())
        .await?;
    println!("create path response == {:?}", create_path_response);
    println!();

    println!("creating path '{}' (overwrite)...", file_name);
    let create_path_response = file_system
        .create_path(Context::default(), file_name, CreatePathOptions::default())
        .await?;
    println!("create path response == {:?}", create_path_response);
    println!();

    println!("creating path '{}' (do not overwrite)...", file_name);
    let do_not_overwrite =
        CreatePathOptions::new().if_match_condition(IfMatchCondition::NotMatch("*"));
    let create_path_result = file_system
        .create_path(Context::default(), file_name, do_not_overwrite)
        .await;
    println!(
        "create path result (should fail) == {:?}",
        create_path_result
    );
    println!();

    println!("setting file system properties...");
    fs_properties.insert("ModifiedBy", "Iota");
    let set_fs_props_response = file_system
        .set_properties(Some(&fs_properties))
        .execute()
        .await?;
    println!(
        "set file system properties response == {:?}",
        set_fs_props_response
    );
    println!();

    println!("getting file system properties...");
    let get_fs_props_response = file_system.get_properties().execute().await?;
    println!(
        "get file system properties response == {:?}",
        get_fs_props_response
    );
    println!();

    println!("deleting file system...");
    let delete_fs_response = file_system.delete().execute().await?;
    println!("delete file system response == {:?}", delete_fs_response);
    println!();

    println!("data lake example done.");

    Ok(())
}
