use azure_storage::storage_shared_key_credential::StorageSharedKeyCredential;
use azure_storage_datalake::prelude::*;
use chrono::Utc;
use std::error::Error;

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
        .properties(fs_properties.clone())
        .into_future()
        .await?;
    println!("create file system response == {:?}\n", create_fs_response);

    let directory_name = "some/directory";
    let directory_client = file_system_client.get_directory_client(directory_name);
    println!("creating directory '{}'...", &directory_name);
    let create_directory_response = directory_client
        .create()
        .properties(fs_properties.clone())
        .into_future()
        .await?;
    println!(
        "create directory response == {:?}\n",
        create_directory_response
    );

    println!("creating directory '{}' if not exists...", directory_name);
    let create_directory_if_not_exists_result =
        directory_client.create_if_not_exists().into_future().await;
    println!(
        "create directory result (should fail) == {:?}\n",
        create_directory_if_not_exists_result
    );

    let new_directory_name = "some/directory2";
    println!(
        "renaming directory '{}' to '{}' ...",
        &directory_name, &new_directory_name
    );
    let rename_directory_response = directory_client
        .rename(new_directory_name)
        .properties(fs_properties.clone())
        .into_future()
        .await?;
    println!(
        "rename directory response == {:?}\n",
        rename_directory_response
    );

    println!("deleting file system...");
    let delete_fs_response = file_system_client.delete().into_future().await?;
    println!("delete file system response == {:?}\n", delete_fs_response);

    Ok(())
}

async fn create_data_lake_client() -> Result<DataLakeClient, Box<dyn Error + Send + Sync>> {
    let account_name = std::env::var("ADLSGEN2_STORAGE_ACCOUNT")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCOUNT first!");
    let account_key = std::env::var("ADLSGEN2_STORAGE_MASTER_KEY")
        .expect("Set env variable ADLSGEN2_STORAGE_MASTER_KEY first!");

    Ok(DataLakeClient::new(
        StorageSharedKeyCredential::new(account_name, account_key),
        None,
    ))
}
