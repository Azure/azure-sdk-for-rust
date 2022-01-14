#![cfg(all(test, feature = "test_e2e"))]
// #![cfg(feature = "mock_transport_framework")]

use azure_core::prelude::*;
use azure_storage_datalake::prelude::*;
use chrono::Utc;
use futures::stream::StreamExt;
use std::error::Error;
use std::num::NonZeroU32;

mod setup;

#[tokio::test]
async fn test_data_lake_file_system_functions() -> Result<(), Box<dyn Error + Send + Sync>> {
    let data_lake_client = setup::create_data_lake_client().await.unwrap();

    let file_system_name = format!(
        "azurerustsdk-datalake-e2etest-fs-{}",
        Utc::now().timestamp()
    );
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    let mut fs_properties = Properties::new();
    fs_properties.insert("AddedVia", "Azure SDK for Rust");

    let create_fs_response = file_system_client
        .create()
        .properties(fs_properties.clone())
        .into_future()
        .await?;
    assert!(
        create_fs_response.namespace_enabled,
        "namespace should be enabled"
    );

    let mut stream = data_lake_client
        .list_file_systems()
        .max_results(NonZeroU32::new(3).unwrap())
        .into_stream();
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

    let get_fs_props_response = file_system_client.get_properties().into_future().await?;
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

    fs_properties.insert("ModifiedBy", "Iota");
    file_system_client
        .set_properties(fs_properties)
        .into_future()
        .await?;

    let get_fs_props_response = file_system_client.get_properties().into_future().await?;
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

    file_system_client.delete().into_future().await?;

    Ok(())
}

#[tokio::test]
async fn test_data_lake_file_create_delete_functions() -> Result<(), Box<dyn Error + Send + Sync>> {
    let data_lake_client = setup::create_data_lake_client().await.unwrap();

    let file_system_name = format!(
        "azurerustsdk-datalake-e2etest-file-create-{}",
        Utc::now().timestamp()
    );
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    let create_fs_response = file_system_client.create().into_future().await?;
    assert!(
        create_fs_response.namespace_enabled,
        "namespace should be enabled"
    );

    // TODO: CoreError(PolicyError(RelativeUrlWithoutBase))

    let file_path = "some/path/e2etest-file.txt";
    let file_client = file_system_client.get_file_client(file_path);

    file_client.create().into_future().await?;

    let create_file_if_not_exists_result = file_client.create_if_not_exists().into_future().await;
    assert!(create_file_if_not_exists_result.is_err());

    file_client.create().into_future().await?;

    file_client.delete().into_future().await?;

    file_system_client.delete().into_future().await?;

    Ok(())
}

#[tokio::test]
async fn test_data_lake_file_upload_functions() -> Result<(), Box<dyn Error + Send + Sync>> {
    let data_lake_client = setup::create_data_lake_client().await.unwrap();

    let file_system_name = format!(
        "azurerustsdk-datalake-e2etest-file-upload-{}",
        Utc::now().timestamp()
    );
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    let create_fs_response = file_system_client.create().into_future().await?;
    assert!(
        create_fs_response.namespace_enabled,
        "namespace should be enabled"
    );

    let file_path = "some/path/e2etest-file.txt";
    let file_client = file_system_client.get_file_client(file_path);

    file_client.create().into_future().await?;

    let bytes = bytes::Bytes::from("some data");
    let file_length = bytes.len() as i64;
    file_system_client
        .append_to_file(
            Context::default(),
            file_path,
            bytes,
            0,
            FileAppendOptions::default(),
        )
        .await?;

    file_system_client
        .flush_file(
            Context::default(),
            file_path,
            file_length,
            true,
            FileFlushOptions::default(),
        )
        .await?;

    file_system_client.delete().into_future().await?;

    Ok(())
}

#[tokio::test]
async fn test_data_lake_file_rename_functions() -> Result<(), Box<dyn Error + Send + Sync>> {
    let data_lake_client = setup::create_data_lake_client().await.unwrap();

    let file_system_name = format!(
        "azurerustsdk-datalake-e2etest-file-rename-{}",
        Utc::now().timestamp()
    );
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    let create_fs_response = file_system_client.create().into_future().await?;
    assert!(
        create_fs_response.namespace_enabled,
        "namespace should be enabled"
    );

    let file_path1 = "some/path/e2etest-file1.txt";
    let file_client1 = file_system_client.get_file_client(file_path1);
    let file_path2 = "some/path/e2etest-file2.txt";
    let file_client2 = file_system_client.get_file_client(file_path2);

    file_client1.create().into_future().await?;

    file_client2.create().into_future().await?;

    let rename_file_if_not_exists_result = file_client1
        .rename_if_not_exists(file_path2)
        .into_future()
        .await;
    assert!(rename_file_if_not_exists_result.is_err());

    file_system_client
        .rename_file(
            Context::default(),
            file_path1,
            file_path2,
            FileRenameOptions::default(),
        )
        .await?;

    file_system_client.delete().into_future().await?;

    Ok(())
}
