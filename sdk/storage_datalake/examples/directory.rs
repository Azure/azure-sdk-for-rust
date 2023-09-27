use azure_storage::prelude::StorageCredentials;
use azure_storage_datalake::prelude::*;
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let data_lake_client = create_data_lake_client();

    let file_system_name = format!(
        "azurerustsdk-datalake-example00-{}",
        OffsetDateTime::now_utc().unix_timestamp()
    );
    let file_system_client = data_lake_client.file_system_client(file_system_name.to_string());

    let mut fs_properties = Properties::new();
    fs_properties.insert("AddedVia", "Azure SDK for Rust");

    println!("creating file system '{}'...", &file_system_name);
    let create_fs_response = file_system_client
        .create()
        .properties(fs_properties.clone())
        .await?;
    println!("create file system response == {create_fs_response:?}\n");

    let directory_name = "some/directory";
    let directory_client = file_system_client.get_directory_client(directory_name);
    println!("creating directory '{}'...", &directory_name);
    let create_directory_response = directory_client
        .create()
        .properties(fs_properties.clone())
        .await?;
    println!("create directory response == {create_directory_response:?}\n");

    println!("creating directory '{directory_name}' if not exists...");
    let create_directory_if_not_exists_result = directory_client.create_if_not_exists().await;
    println!(
        "create directory result (should fail) == {create_directory_if_not_exists_result:?}\n"
    );

    let new_directory_name = "some/directory2";
    println!(
        "renaming directory '{}' to '{}' ...",
        &directory_name, &new_directory_name
    );
    directory_client
        .rename(new_directory_name)
        .properties(fs_properties)
        .await?;

    println!("deleting file system...");
    let delete_fs_response = file_system_client.delete().await?;
    println!("delete file system response == {delete_fs_response:?}\n");

    Ok(())
}

fn create_data_lake_client() -> DataLakeClient {
    let account_name = std::env::var("ADLSGEN2_STORAGE_ACCOUNT")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCOUNT first!");
    let account_key = std::env::var("ADLSGEN2_STORAGE_ACCESS_KEY")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCESS_KEY first!");

    let storage_credentials = StorageCredentials::access_key(account_name.clone(), account_key);
    DataLakeClient::new(account_name, storage_credentials)
}
