use azure_storage::storage_shared_key_credential::StorageSharedKeyCredential;
use azure_storage_datalake::prelude::*;
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let data_lake_client = create_data_lake_client().await.unwrap();

    let file_system_name = format!(
        "azurerustsdk-datalake-example01-{}",
        OffsetDateTime::now_utc().unix_timestamp()
    );
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    println!("creating file system '{}'...", &file_system_name);
    let create_fs_response = file_system_client.create().into_future().await?;
    println!("create file system response == {:?}\n", create_fs_response);

    let file_path = "some/path/example-file.txt";
    let file_client = file_system_client.get_file_client(file_path);
    let mut file_properties = Properties::new();
    file_properties.insert("AddedVia", "Azure SDK for Rust");

    println!("creating file '{}'...", file_path);
    let create_file_response = file_client
        .create()
        .properties(file_properties.clone())
        .into_future()
        .await?;
    println!("create file response == {:?}\n", create_file_response);

    println!("getting properties for file '{}'...", file_path);
    let get_properties_response = file_client.get_properties().into_future().await?;
    println!(
        "get file properties response == {:?}\n",
        get_properties_response
    );

    println!("setting properties for file '{}'...", file_path);
    file_properties.insert("ModifiedBy", "Iota");
    let set_properties_response = file_client
        .set_properties(file_properties)
        .into_future()
        .await?;
    println!(
        "set file properties response == {:?}\n",
        set_properties_response
    );

    println!("creating file '{}' if not exists...", file_path);
    let create_file_if_not_exists_result = file_client.create_if_not_exists().into_future().await;
    println!(
        "create file result (should fail) == {:?}\n",
        create_file_if_not_exists_result
    );

    println!("creating file '{}' (overwrite)...", file_path);
    let create_file_response = file_client.create().into_future().await?;
    println!("create file response == {:?}\n", create_file_response);

    println!("deleting file '{}'...", file_path);
    let delete_file_response = file_client.delete().into_future().await?;
    println!("delete_file file response == {:?}\n", delete_file_response);

    println!("deleting file system...");
    let delete_fs_response = file_system_client.delete().into_future().await?;
    println!("delete file system response == {:?}\n", delete_fs_response);

    Ok(())
}

async fn create_data_lake_client() -> azure_core::Result<DataLakeClient> {
    let account_name = std::env::var("ADLSGEN2_STORAGE_ACCOUNT")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCOUNT first!");
    let account_key = std::env::var("ADLSGEN2_STORAGE_ACCESS_KEY")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCESS_KEY first!");

    Ok(DataLakeClient::new(
        StorageSharedKeyCredential::new(account_name, account_key),
        None,
    ))
}
