#![cfg(all(test, feature = "test_e2e"))]
use azure_storage::adls_gen2::prelude::*;
use azure_storage::core::prelude::*;

#[tokio::test]
async fn create_and_delete() {
    let filesystem_name1 = "azuresdkrustetoetsfs1";
    let filesystem_name2 = "azuresdkrustetoetsfs2";

    let properties1 = "n1=eWVz,n2=bm8=";
    let properties2 = "n1=bm8=,n2=eWVz";

    let client = initialize();
    client
        .create_filesystem()
        .with_filesystem(filesystem_name1)
        .finalize()
        .await
        .unwrap();
    client
        .create_filesystem()
        .with_filesystem(filesystem_name2)
        .with_properties(properties2)
        .finalize()
        .await
        .unwrap();

    client
        .set_filesystem_properties()
        .with_filesystem(filesystem_name1)
        .with_properties(properties1)
        .finalize()
        .await
        .unwrap();

    // TODO: Reenable test after the with_properties
    // setters have been reenabled.
    //let properties1_response = client
    //    .get_filesystem_properties()
    //    .with_filesystem(filesystem_name1)
    //    .finalize()
    //    .await
    //    .unwrap();
    //let properties2_response = client
    //    .get_filesystem_properties()
    //    .with_filesystem(filesystem_name2)
    //    .finalize()
    //    .await
    //    .unwrap();

    //assert_eq!(properties1, properties1_response.properties);
    //assert_eq!(properties2, properties2_response.properties);

    let file_systems = client.list_filesystems().finalize().await.unwrap();

    let found1 = if file_systems
        .incomplete_vector
        .iter()
        .any(|name| name.name == filesystem_name1)
    {
        client
            .delete_filesystem()
            .with_filesystem(filesystem_name1)
            .finalize()
            .await
            .unwrap();

        true
    } else {
        false
    };

    let found2 = if file_systems
        .incomplete_vector
        .iter()
        .any(|name| name.name == filesystem_name2)
    {
        client
            .delete_filesystem()
            .with_filesystem(filesystem_name2)
            .finalize()
            .await
            .unwrap();

        true
    } else {
        false
    };

    assert!(found1 && found2);
}

fn initialize() -> Box<dyn Client> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    Box::new(client::with_access_key(&account, &master_key))
}
