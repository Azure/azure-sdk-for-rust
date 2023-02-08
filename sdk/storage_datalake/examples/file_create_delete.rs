use azure_storage::prelude::StorageCredentials;
use azure_storage_datalake::prelude::*;
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let data_lake_client = create_data_lake_client();

    let file_system_name = format!(
        "azurerustsdk-datalake-example01-{}",
        OffsetDateTime::now_utc().unix_timestamp()
    );
    let file_system_client = data_lake_client.file_system_client(file_system_name.to_string());

    println!("creating file system '{}'...", &file_system_name);
    let create_fs_response = file_system_client.create().await?;
    println!("create file system response == {create_fs_response:?}\n");

    let file_path = "some/path/example-file.txt";
    let file_client = file_system_client.get_file_client(file_path);
    let mut file_properties = Properties::new();
    file_properties.insert("AddedVia", "Azure SDK for Rust");

    println!("creating file '{file_path}'...");
    let create_file_response = file_client
        .create()
        .properties(file_properties.clone())
        .await?;
    println!("create file response == {create_file_response:?}\n");

    println!("getting properties for file '{file_path}'...");
    let get_properties_response = file_client.get_properties().await?;
    println!("get file properties response == {get_properties_response:?}\n");

    println!("setting properties for file '{file_path}'...");
    file_properties.insert("ModifiedBy", "Iota");
    let set_properties_response = file_client.set_properties(file_properties).await?;
    println!("set file properties response == {set_properties_response:?}\n");

    println!("creating file '{file_path}' if not exists...");
    let create_file_if_not_exists_result = file_client.create_if_not_exists().await;
    println!("create file result (should fail) == {create_file_if_not_exists_result:?}\n");

    println!("creating file '{file_path}' (overwrite)...");
    let create_file_response = file_client.create().await?;
    println!("create file response == {create_file_response:?}\n");

    println!("deleting file '{file_path}'...");
    let delete_file_response = file_client.delete().await?;
    println!("delete_file file response == {delete_file_response:?}\n");

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

    let storage_credentials = StorageCredentials::Key(account_name.clone(), account_key);
    DataLakeClient::new(account_name, storage_credentials)
}
