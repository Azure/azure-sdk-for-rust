#![cfg(all(test, feature = "test_e2e"))]
// #![cfg(feature = "mock_transport_framework")]

use azure_core::prelude::*;
use azure_identity::token_credentials::DefaultAzureCredential;
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
    let bearer_token = DefaultAzureCredential::default()
        .get_token(resource_id)
        .await?;

    let data_lake_client = storage_account_client
        .as_storage_client()
        // This test won't work during replay in CI until all operations are converted to pipeline architecture
        // .as_data_lake_client_with_transaction(account, bearer_token.token.secret().to_owned(), "test_data_lake_file_system_functions")?;
        .as_data_lake_client(account, bearer_token.token.secret().to_owned())?;

    let file_system_client = data_lake_client.as_file_system_client(&file_system_name)?;

    let mut fs_properties = Properties::new();
    fs_properties.insert("AddedVia", "Azure SDK for Rust");

    let create_fs_response = file_system_client
        .create()
        .properties(&fs_properties)
        .execute()
        .await?;
    assert!(
        create_fs_response.namespace_enabled,
        "namespace should be enabled"
    );

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
                break;
            }
        }
    }
    assert!(found, "did not find created file system");

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

    let file_name = "e2etest-file.txt";

    file_system_client
        .create_file(Context::default(), file_name, FileCreateOptions::default())
        .await?;

    file_system_client
        .create_file(Context::default(), file_name, FileCreateOptions::default())
        .await?;

    let do_not_overwrite =
        FileCreateOptions::new().if_match_condition(IfMatchCondition::NotMatch("*"));
    let create_path_result = file_system_client
        .create_file(Context::default(), file_name, do_not_overwrite) // Add method create_file_if_not_exists
        .await;
    assert!(create_path_result.is_err());

    fs_properties.insert("ModifiedBy", "Iota");
    file_system_client
        .set_properties(Some(&fs_properties))
        .execute()
        .await?;

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

    file_system_client.delete().execute().await?;

    Ok(())
}
