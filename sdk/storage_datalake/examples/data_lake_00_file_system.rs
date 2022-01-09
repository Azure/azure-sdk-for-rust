use azure_identity::token_credentials::DefaultAzureCredential;
use azure_identity::token_credentials::TokenCredential;
use azure_storage::core::prelude::*;
use azure_storage_datalake::prelude::*;
use chrono::Utc;
use futures::stream::StreamExt;
use std::error::Error;
use std::num::NonZeroU32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let data_lake_client = create_data_lake_client().await.unwrap();

    let file_system_name = format!("azurerustsdk-datalake-example00-{}", Utc::now().timestamp());
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    let mut fs_properties = Properties::new();
    fs_properties.insert("AddedVia", "Azure SDK for Rust");

    println!("creating file system '{}'...", &file_system_name);
    let create_fs_response = file_system_client
        .create()
        .properties(&fs_properties)
        .execute()
        .await?;
    println!("create file system response == {:?}\n", create_fs_response);

    println!("listing file systems...");
    let mut stream = Box::pin(
        data_lake_client
            .list()
            .max_results(NonZeroU32::new(3).unwrap())
            .stream(),
    );
    while let Some(list_fs_response) = stream.next().await {
        println!("list file system response == {:?}\n", list_fs_response);
    }

    println!("getting file system properties...");
    let get_fs_props_response = file_system_client.get_properties().execute().await?;
    println!(
        "get file system properties response == {:?}\n",
        get_fs_props_response
    );

    println!("setting file system properties...");
    fs_properties.insert("ModifiedBy", "Iota");
    let set_fs_props_response = file_system_client
        .set_properties(Some(&fs_properties))
        .execute()
        .await?;
    println!(
        "set file system properties response == {:?}\n",
        set_fs_props_response
    );

    println!("getting file system properties...");
    let get_fs_props_response = file_system_client.get_properties().execute().await?;
    println!(
        "get file system properties response == {:?}\n",
        get_fs_props_response
    );

    println!("deleting file system...");
    let delete_fs_response = file_system_client.delete().execute().await?;
    println!("delete file system response == {:?}\n", delete_fs_response);

    Ok(())
}

async fn create_data_lake_client() -> Result<DataLakeClient, Box<dyn Error + Send + Sync>> {
    let account = std::env::var("ADLSGEN2_STORAGE_ACCOUNT")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCOUNT first!");
    let master_key = std::env::var("ADLSGEN2_STORAGE_MASTER_KEY")
        .expect("Set env variable ADLSGEN2_STORAGE_MASTER_KEY first!");

    let options = StorageAccountOptions::default();

    let storage_account_client =
        StorageAccountClient::new_access_key(&account, &master_key, options);

    let resource_id = "https://storage.azure.com/";
    println!("getting bearer token for '{}'...", resource_id);
    let bearer_token = DefaultAzureCredential::default()
        .get_token(resource_id)
        .await?;
    println!("token expires on {}\n", bearer_token.expires_on);

    let storage_client = storage_account_client.as_storage_client();

    Ok(DataLakeClient::new(
        storage_client,
        account,
        bearer_token.token.secret().to_owned(),
        None,
    ))
}
