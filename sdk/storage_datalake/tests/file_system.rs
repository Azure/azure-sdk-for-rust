use azure_storage_datalake::prelude::*;
use futures::stream::StreamExt;
use std::num::NonZeroU32;

mod setup;

#[tokio::test]
async fn file_system_create_delete() -> azure_core::Result<()> {
    let data_lake_client = setup::create_data_lake_client("datalake_file_system")
        .await
        .unwrap();

    let file_system_name = "azurerustsdk-datalake-file-system";
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
        .prefix("azurerustsdk")
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
    let added_via_option = get_fs_props_response.properties.get("AddedVia");
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
    let modified_by_option = get_fs_props_response.properties.get("ModifiedBy");
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
async fn file_system_list_paths() -> azure_core::Result<()> {
    let data_lake_client = setup::create_data_lake_client("datalake_file_system_list_paths")
        .await
        .unwrap();

    let file_system_name = "azurerustsdk-datalake-file-system-list-paths";
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    file_system_client.create().into_future().await?;

    let file_path = "some/path/file1.txt";
    let file_client = file_system_client.get_file_client(file_path);
    file_client.create().into_future().await?;

    let file_path = "some/path/file2.txt";
    let file_client = file_system_client.get_file_client(file_path);
    file_client.create().into_future().await?;

    let file_path = "some/other_path/file3.txt";
    let file_client = file_system_client.get_file_client(file_path);
    file_client.create().into_future().await?;

    // by default all paths are listed
    let paths = file_system_client
        .list_paths()
        .into_stream()
        .next()
        .await
        .unwrap()?;
    assert_eq!(paths.paths.len(), 6);

    // test only paths within directory
    let paths = file_system_client
        .list_paths()
        .directory("some/path")
        .into_stream()
        .next()
        .await
        .unwrap()?;
    assert_eq!(paths.paths.len(), 2);

    // test non recursive paths
    let paths = file_system_client
        .list_paths()
        .directory("some")
        .recursive(false)
        .into_stream()
        .next()
        .await
        .unwrap()?;
    assert_eq!(paths.paths.len(), 2);

    file_system_client.delete().into_future().await?;

    Ok(())
}
