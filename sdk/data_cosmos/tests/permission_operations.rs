use azure_data_cosmos::prelude::*;
use futures::StreamExt;

mod setup_mock;

#[tokio::test]
async fn permission_operations() {
    const DATABASE_NAME: &str = "cosmos-test-db-users";
    const COLLECTION_NAME: &str = "cosmos-test-db-users";
    const USER_NAME1: &str = "someone@cool.net";
    const USER_NAME2: &str = "pollastro@cool.net";
    const PERMISSION1: &str = "godmode";
    const PERMISSION2: &str = "spyme";

    let client = setup_mock::initialize("permission_operations").unwrap();

    // create a temp database
    let _ = client
        .create_database(DATABASE_NAME)
        .into_future()
        .await
        .unwrap();

    let database = client.database_client(DATABASE_NAME);

    // create two users
    let user1 = database.user_client(USER_NAME1);
    let _create_user_response = user1.create_user().into_future().await.unwrap();
    let user2 = database.user_client(USER_NAME2);
    let _create_user_response = user2.create_user().into_future().await.unwrap();

    // create a temp collection
    let create_collection_response = database
        .create_collection(COLLECTION_NAME, "/id")
        .into_future()
        .await
        .unwrap();

    // create two permissions
    let permission_user1 = user1.permission_client(PERMISSION1);
    let permission_user2 = user2.permission_client(PERMISSION2);

    let _ = permission_user1
        .create_permission(create_collection_response.collection.all_permission())
        .expiry_seconds(18000u64) // 5 hours, max!
        .into_future()
        .await
        .unwrap();

    let _ = permission_user2
        .create_permission(create_collection_response.collection.read_permission())
        .expiry_seconds(18000u64) // 5 hours, max!
        .into_future()
        .await
        .unwrap();

    let list_permissions_response = user1
        .list_permissions()
        .into_stream()
        .next()
        .await
        .unwrap()
        .unwrap();
    assert_eq!(list_permissions_response.permissions.len(), 1);

    let list_permissions_response = user2
        .list_permissions()
        .into_stream()
        .next()
        .await
        .unwrap()
        .unwrap();
    assert_eq!(list_permissions_response.permissions.len(), 1);

    // delete the database
    database.delete_database().into_future().await.unwrap();
}
