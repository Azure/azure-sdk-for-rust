#![cfg(all(test, feature = "test_e2e"))]
use azure_core::prelude::*;
use azure_identity::token_credentials::DefaultCredential;
use azure_identity::token_credentials::TokenCredential;
use azure_storage::core::prelude::*;
use azure_storage::data_lake::prelude::*;
use chrono::Utc;
use futures::stream::StreamExt;
use std::error::Error;
use std::num::NonZeroU32;

#[tokio::test]
async fn test_data_lake_file_system_functions() -> Result<(), Box<dyn Error + Send + Sync>> {
    let account = std::env::var("ADLSGEN2_STORAGE_ACCOUNT")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCOUNT first!");
    let master_key = std::env::var("ADLSGEN2_STORAGE_MASTER_KEY")
        .expect("Set env variable ADLSGEN2_STORAGE_MASTER_KEY first!");

    let now = Utc::now();
    let file_system_name = format!("azurerustsdk-datalake-e2etest-{}", now.timestamp());

    let http_client = new_http_client();

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);

    let resource_id = "https://storage.azure.com/";
    println!("getting bearer token for '{}'...", resource_id);
    let bearer_token = DefaultCredential::default().get_token(resource_id).await?;
    println!("token expires on {}", bearer_token.expires_on);
    println!();

    let data_lake_client = storage_account_client
        .as_storage_client()
        .as_data_lake_client(account, bearer_token.token.secret().to_owned())?;

    let file_system_client = data_lake_client.as_file_system_client(&file_system_name)?;

    let mut fs_properties = Properties::new();
    fs_properties.insert("AddedVia", "Azure SDK for Rust");

    println!("creating file system '{}'...", &file_system_name);
    let create_fs_response = file_system_client
        .create()
        .properties(&fs_properties)
        .execute()
        .await?;
    println!("create file system response == {:?}", create_fs_response);
    assert!(
        create_fs_response.namespace_enabled,
        "namespace should be enabled"
    );
    println!("namespace is enabled");
    println!();

    println!("listing file systems...");
    let mut stream = Box::pin(
        data_lake_client
            .list()
            .max_results(NonZeroU32::new(3).unwrap())
            .stream(),
    );
    let mut found = false;
    while let Some(list_fs_response) = stream.next().await {
        for fs in list_fs_response.unwrap().file_systems {
            if fs.name == file_system_name {
                found = true;
            }
        }
    }
    assert!(found, "did not find created file system");
    println!("found created file system");
    println!();

    println!("getting file system properties...");
    let get_fs_props_response = file_system_client.get_properties().execute().await?;
    let properties_hashmap = get_fs_props_response.properties.hash_map();
    let added_via_option = properties_hashmap.get("AddedVia");
    assert!(
        added_via_option.is_some(),
        "did not find expected property: AddedVia"
    );
    assert_eq!(
        added_via_option.unwrap().to_string(),
        "Azure SDK for Rust",
        "did not find expected property value for: AddedVia"
    );
    println!("found expected file system property: AddedVia");
    println!();

    let file_name = "e2etest-file.txt";

    println!("creating path '{}'...", file_name);
    let create_path_response = file_system_client
        .create_path(Context::default(), file_name, CreatePathOptions::default())
        .await?;
    println!("create path response == {:?}", create_path_response);
    println!();

    println!("creating path '{}' (overwrite)...", file_name);
    let create_path_response = file_system_client
        .create_path(Context::default(), file_name, CreatePathOptions::default())
        .await?;
    println!("create path response == {:?}", create_path_response);
    println!();

    println!("creating path '{}' (do not overwrite)...", file_name);
    let do_not_overwrite =
        CreatePathOptions::new().if_match_condition(IfMatchCondition::NotMatch("*"));
    let create_path_result = file_system_client
        .create_path(Context::default(), file_name, do_not_overwrite)
        .await;
    assert!(create_path_result.is_err());
    println!(
        "create path result (should fail) == {:?}",
        create_path_result
    );
    println!();

    println!("setting file system properties...");
    fs_properties.insert("ModifiedBy", "Iota");
    let set_fs_props_response = file_system_client
        .set_properties(Some(&fs_properties))
        .execute()
        .await?;
    println!(
        "set file system properties response == {:?}",
        set_fs_props_response
    );
    println!();

    println!("getting file system properties...");
    let get_fs_props_response = file_system_client.get_properties().execute().await?;
    let properties_hashmap = get_fs_props_response.properties.hash_map();
    let modified_by_option = properties_hashmap.get("ModifiedBy");
    assert!(
        modified_by_option.is_some(),
        "did not find expected property: ModifiedBy"
    );
    assert_eq!(
        modified_by_option.unwrap().to_string(),
        "Iota",
        "did not find expected property value for: ModifiedBy"
    );
    println!("found expected file system property: ModifiedBy");
    println!();

    println!("deleting file system...");
    let delete_fs_response = file_system_client.delete().execute().await?;
    println!("delete file system response == {:?}", delete_fs_response);
    println!();

    Ok(())
}
